use std::fs::{remove_file, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;

use anyhow::bail;
use chrono::Local;
use flate2::write::GzEncoder;
use flate2::Compression;
use surrealdb::engine::any::Any;
use surrealdb::{Connection, Surreal};
use tokio::spawn;
use tokio::task::JoinHandle;
use tokio::time::{interval, sleep, MissedTickBehavior};
use tokio_util::sync::CancellationToken;
use tracing::{debug, error, info, info_span, trace, Instrument};

use crate::application::MycologContext;
use crate::config::GLOBAL_CONFIG;

pub fn backup_task(context: &MycologContext) -> JoinHandle<()> {
    let surreal = Arc::clone(&context.db);
    let shutdown_token = context.task_cancel_token.clone();
    spawn(async move { backup_service(surreal, shutdown_token).await })
}

async fn backup_service(surreal: Arc<Surreal<Any>>, shutdown_token: CancellationToken) {
    // Initial delay
    sleep(Duration::from_secs(
        GLOBAL_CONFIG.backup_delay_hours as u64 * 3600,
    ))
    .await;

    let mut interval = interval(Duration::from_secs(
        GLOBAL_CONFIG.backup_frequency_hours as u64 * 3600,
    ));
    interval.set_missed_tick_behavior(MissedTickBehavior::Skip);

    let backup_span = info_span!("backup");
    loop {
        tokio::select! {
            _ = shutdown_token.cancelled() => {
                break;
            }
            _ = interval.tick() => {
                debug!("Backing up database...");
                let backup_result = backup_database(Arc::clone(&surreal))
                    .instrument(backup_span.clone().or_current())
                    .await;
                if let Err(err) = backup_result {
                    error!("Database backup with error: {err}");
                }
            }
        }
    }
}

async fn backup_database(surreal: Arc<Surreal<Any>>) -> anyhow::Result<()> {
    let file_path = calc_backup_file_name();

    let target_file = File::create(file_path.clone())?;
    let file_name = write_backup(&*surreal, target_file).await?;
    info!(
        "Database backup written to: {}",
        file_path.file_name().unwrap().to_str().unwrap().to_string()
    );
    let file_names = get_backup_names().await?;
    constraint_backups(file_names)?;

    Ok(())
}

async fn write_backup<C: Connection, W: Write>(
    surreal: &Surreal<C>,
    writer: W,
) -> anyhow::Result<W> {
    let mut export_stream = surreal.export(()).await?;
    let mut compression_encoder = GzEncoder::new(writer, Compression::best());

    while let Some(result) = export_stream.next().await {
        match result {
            Ok(bytes) => {
                compression_encoder.write_all(&bytes)?;
            }
            Err(err) => {
                bail!(err);
            }
        }
    }

    compression_encoder.try_finish()?;
    let writer = compression_encoder.finish()?;
    Ok(writer)
}

fn calc_backup_file_name() -> PathBuf {
    let now = Local::now();
    let time = now.format("%Y%m%d%H%M%S").to_string();
    let file_name = format!("backup_{time}");

    let mut file_path = Path::new("backup").join(file_name.clone());
    file_path.set_extension(".gzip");
    file_path
}

async fn get_backup_names() -> anyhow::Result<Vec<String>> {
    let mut valid_filenames = Path::new("backup/")
        .read_dir()?
        .filter_map(|entry| match entry {
            Ok(entry) => {
                let filename = entry.file_name();
                if !filename.to_str().unwrap().starts_with("backup_") {
                    return None;
                }
                filename.into_string().ok()
            }
            Err(err) => {
                trace!(info = err.to_string(), "Backup dir entry error");
                None
            }
        })
        .collect::<Vec<_>>();
    valid_filenames.sort();
    Ok(valid_filenames)
}

fn constraint_backups(file_names: Vec<String>) -> anyhow::Result<()> {
    if file_names.len() > GLOBAL_CONFIG.backup_max_amount {
        if let Some(file_name) = file_names.first() {
            info!("Delete backup with name: {file_name}");
            remove_file(Path::new("backup").join(file_name).with_extension("gzip"))?;
        }
    }

    Ok(())
}
