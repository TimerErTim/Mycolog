use std::collections::VecDeque;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::Duration;

use anyhow::{anyhow, bail};
use chrono::Local;
use flate2::write::GzEncoder;
use flate2::Compression;
use futures_lite::StreamExt;
use tokio::time::{interval, interval_at, sleep, Instant, MissedTickBehavior};
use tokio_util::sync::CancellationToken;
use tracing::{debug, error, info, instrument, warn};

use crate::application::database::DatabaseRootAccess;
use crate::application::BackupLimit;
use crate::config::MycologConfig;
use crate::utils::asynchronous::run_catch;

pub async fn backup_service(
    config: &MycologConfig,
    db: DatabaseRootAccess,
    shutdown_token: CancellationToken,
) -> anyhow::Result<()> {
    info!("started backup service");

    // Initial delay
    let delay_duration = Duration::from_secs(config.backup_delay_hours * 3600);

    // Frequency
    let mut interval = interval_at(
        Instant::now() + delay_duration,
        Duration::from_secs(config.backup_interval_hours * 3600),
    );
    interval.set_missed_tick_behavior(MissedTickBehavior::Skip);

    while !shutdown_token.is_cancelled() {
        tokio::select! {
            _ = shutdown_token.cancelled() => break,
            _ = interval.tick() => {
                info!("backing up database...");
                let backup_result = backup_database(&db, &config.backup_limit).await;
                if let Err(err) = backup_result {
                    error!("database backup with error: {err}");
                }
            }
        }
    }
}

#[instrument(skip_all)]
async fn backup_database(
    surreal: &DatabaseRootAccess,
    backup_limit: &BackupLimit,
) -> anyhow::Result<()> {
    let file_path = calc_backup_file_name();

    let target_file = File::create(file_path.clone())?;
    let _ = write_backup(surreal, target_file).await?;
    info!(
        "database backup written to: {:?}",
        file_path.file_name().ok_or(anyhow!("no filename"))?
    );

    match constraint_backups(backup_limit).await {
        Ok(backups_deleted) => {
            if backups_deleted > 0 {
                info!(
                    amount = backups_deleted,
                    "previous database backups were deleted according to limit"
                );
            }
        }
        Err(err) => {
            error!(err = err.to_string(), "enforcing backup limits failed");
        }
    }

    Ok(())
}

async fn write_backup<W: Write + Send + 'static>(
    db: &DatabaseRootAccess,
    writer: W,
) -> anyhow::Result<W> {
    let (sender, mut receiver) = async_channel::bounded::<Vec<u8>>(10_000);
    let write_task = run_catch(async move {
        let mut compression_encoder = GzEncoder::new(writer, Compression::best());

        while let Some(bytes) = receiver.next().await {
            compression_encoder.write_all(&bytes)?;
        }

        compression_encoder.try_finish()?;
        Ok(compression_encoder.finish()?)
    });

    db.export(sender).await?;
    let task_output = write_task.await?;
    Ok(task_output)
}

fn calc_backup_file_name() -> PathBuf {
    let now = Local::now();
    let time = now.format("%Y%m%d%H%M%S").to_string();
    let file_name = format!("backup_{time}");

    let mut file_path = Path::new("backups").join(file_name.clone());
    file_path.set_extension("gz");
    file_path
}

async fn get_backup_paths() -> anyhow::Result<Vec<PathBuf>> {
    let mut valid_filenames = Path::new("backups/")
        .read_dir()?
        .filter_map(|entry| match entry {
            Ok(entry) => {
                let file_path = entry.path();
                let file_name = file_path.file_name()?;
                if !file_name.to_str()?.starts_with("backup_") {
                    return None;
                }
                Some(file_path)
            }
            Err(err) => {
                warn!(err = err.to_string(), "backup dir entry error");
                None
            }
        })
        .collect::<Vec<_>>();
    valid_filenames.sort();
    Ok(valid_filenames)
}

/// Ensures the backup limits are respected
async fn constraint_backups(limit: &BackupLimit) -> anyhow::Result<u32> {
    let mut count = 0;
    let mut paths = VecDeque::from(get_backup_paths().await?);
    while limit.check_exceeded(paths.make_contiguous()) {
        if paths.len() > 1 {
            warn!("latest backup should be deleted according to limit, ignoring...");
            break;
        }

        delete_oldest_backup(&mut paths).await?;
        count += 1;
    }

    Ok(count)
}

async fn delete_oldest_backup(paths: &mut VecDeque<PathBuf>) -> anyhow::Result<()> {
    let backup_path = paths.pop_front();
    if let Some(path) = backup_path {
        tokio::fs::remove_file(path).await?;
    } else {
        bail!("no backup remaining to delete");
    }

    Ok(())
}
