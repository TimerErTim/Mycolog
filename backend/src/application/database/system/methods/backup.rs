use std::collections::VecDeque;
use std::io::Write;

use async_compression::tokio::bufread::BrotliEncoder;
use async_compression::Level;
use futures_lite::io::BlockOn;
use futures_lite::{Stream, StreamExt};
use tokio::io::{AsyncBufRead, BufReader};
use tokio_util::io::ReaderStream;
use tracing::{error, info};

use crate::application::database::DatabaseRootAccess;
use crate::utils::asynchronous::run_catch;

impl DatabaseRootAccess {
    /// [Bro](GzEncoder) export of this database.
    pub async fn backup(&self) -> anyhow::Result<impl AsyncBufRead> {
        info!("writing database backup");

        let export_reader = self.export().await?;
        let brotli_encoder = BrotliEncoder::with_quality(export_reader, Level::Best);

        Ok(BufReader::new(brotli_encoder))
    }
}
