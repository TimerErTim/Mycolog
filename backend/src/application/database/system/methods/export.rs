use std::collections::VecDeque;
use std::io::{BufReader, BufWriter, Read, Write};
use std::{io, thread};

use futures_lite::{Stream, StreamExt};
use tokio::io::{AsyncBufRead, AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};
use tokio::sync::mpsc::{Sender, UnboundedSender};
use tokio_util::io::StreamReader;
use tracing::{error, info};

use crate::application::database::system::access::Auth;
use crate::application::database::DatabaseRootAccess;

impl DatabaseRootAccess {
    pub async fn export(&self) -> anyhow::Result<impl AsyncBufRead> {
        info!("attempting database export");
        let (send, recv) = async_channel::bounded(4_096);
        let export = self.datastore.export(self.auth.as_session(), send).await?;
        tokio::spawn(async move {
            export
                .await
                .inspect_err(|err| error!(?err, "database export failed due to error"))
        });

        Ok(StreamReader::new(recv.map(|bytes| -> io::Result<_> {
            Ok(VecDeque::from(bytes))
        })))
    }
}
