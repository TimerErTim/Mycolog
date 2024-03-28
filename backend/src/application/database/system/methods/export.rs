use std::io::{BufReader, BufWriter, Read, Write};

use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};
use tokio::sync::mpsc::{Sender, UnboundedSender};
use tracing::{error, info};

use crate::application::database::system::access::Auth;
use crate::application::database::DatabaseRootAccess;

impl DatabaseRootAccess {
    pub async fn export(&self, export: impl IntoExport) -> anyhow::Result<()> {
        info!("attempting database export");
        let result: anyhow::Result<()> = try {
            let sender = export.into_export().await?;
            let export = self
                .datastore
                .export(self.auth.as_session(), sender)
                .await?;
            export.await?;
        };
        result.inspect_err(|err| error!(?err, "database export failed due to error"))?;
        Ok(())
    }
}

pub trait IntoExport {
    async fn into_export(self) -> anyhow::Result<async_channel::Sender<Vec<u8>>>;
}

impl IntoExport for async_channel::Sender<Vec<u8>> {
    async fn into_export(self) -> anyhow::Result<async_channel::Sender<Vec<u8>>> {
        Ok(self)
    }
}

impl IntoExport for Sender<Vec<u8>> {
    async fn into_export(self) -> anyhow::Result<async_channel::Sender<Vec<u8>>> {
        let (send, receiver) = async_channel::bounded::<Vec<u8>>(self.max_capacity());
        tokio::spawn(async move {
            while let Ok(bytes) = receiver.recv().await {
                if let Err(err) = self.send(bytes).await {
                    error!(?err, "expierenced error during export sender propagation");
                    break;
                }
            }
        });
        Ok(send)
    }
}

impl IntoExport for UnboundedSender<Vec<u8>> {
    async fn into_export(self) -> anyhow::Result<async_channel::Sender<Vec<u8>>> {
        let (send, receiver) = async_channel::unbounded::<Vec<u8>>();
        tokio::spawn(async move {
            while let Ok(bytes) = receiver.recv().await {
                if let Err(err) = self.send(bytes) {
                    error!(?err, "expierenced error during export sender propagation");
                    break;
                }
            }
        });
        Ok(send)
    }
}

impl<W: Write + Send + 'static> IntoExport for BufWriter<W> {
    async fn into_export(mut self) -> anyhow::Result<async_channel::Sender<Vec<u8>>> {
        let (send, receiver) = async_channel::unbounded::<Vec<u8>>();
        tokio::spawn(async move {
            while let Ok(bytes) = receiver.recv().await {
                if let Err(err) = self.write_all(&bytes) {
                    error!(?err, "expierenced error during export writing");
                    break;
                }
            }
        });
        Ok(send)
    }
}

impl<W: AsyncWrite + Unpin + Send + 'static> IntoExport for tokio::io::BufWriter<W> {
    async fn into_export(mut self) -> anyhow::Result<async_channel::Sender<Vec<u8>>> {
        let (send, receiver) = async_channel::bounded::<Vec<u8>>(10_000);
        tokio::spawn(async move {
            while let Ok(bytes) = receiver.recv().await {
                if let Err(err) = self.write_all(&bytes).await {
                    error!(?err, "expierenced error during export writing");
                    break;
                }
            }
        });
        Ok(send)
    }
}
