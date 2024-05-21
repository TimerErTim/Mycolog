use std::io::BufWriter;
use std::time::Duration;

use async_compression::tokio::write::BrotliEncoder;
use tokio::io::BufReader;
use tokio::time::{interval, interval_at, Instant};
use tokio_util::sync::CancellationToken;
use tracing::log::trace;
use tracing::{debug, error, info, info_span, instrument};

use crate::application::database::DatabaseRootAccess;
use crate::application::ScheduleQueries;

pub async fn logging_service(shutdown_token: CancellationToken) -> anyhow::Result<()> {
    let mut daily_timer = interval_at(
        Instant::now() + Duration::from_mins(1),
        Duration::from_days(1),
    );

    info!("started logging service");
    while !shutdown_token.is_cancelled() {
        tokio::select!(
            _ = shutdown_token.cancelled() => break,
            _ = daily_timer.tick() => compress_log_files().await.inspect_err(|err| error!(?err, "compressing log files failed")),
        );
    }

    Ok(())
}

#[instrument]
async fn compress_log_files() -> anyhow::Result<()> {
    let mut compressable_files = Vec::new();

    let mut dir = tokio::fs::read_dir("logs/").await?;
    while let Some(entry) = dir.next_entry().await? {
        let path = entry.path();
        let Some(extension) = path
            .extension()
            .and_then(|string| string.to_str())
            .map(|str| str.to_string())
        else {
            continue;
        };
        let Ok(modified_time) = tokio::fs::metadata(&path)
            .await
            .and_then(|data| data.modified())
        else {
            continue;
        };

        if extension.trim() == "log" {
            compressable_files.push((path, modified_time));
        }
    }

    compressable_files.sort_by_key(|(_, time)| time.clone());
    compressable_files.pop();

    if compressable_files.len() <= 0 {
        info!("no compressable log files found");
        return Ok(());
    }

    info!("found {} compressable log files", compressable_files.len());
    for (path, _) in compressable_files.iter().skip(1) {
        let mut compressed_path = path.clone();
        compressed_path.set_extension("log.br");

        debug!(file = %path.display(), compressed_file = %compressed_path.display(), "compressing file");
        let src_file = tokio::fs::File::open(&path).await?;
        let trgt_file = tokio::fs::File::create(compressed_path).await?;
        tokio::io::copy_buf(
            &mut BufReader::new(src_file),
            &mut BrotliEncoder::new(trgt_file),
        )
        .await?;

        trace!("deleting log file");
        tokio::fs::remove_file(path).await?;
    }

    Ok(())
}
