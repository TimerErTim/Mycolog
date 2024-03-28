use std::io::{BufReader, Read};

use tokio::io::{AsyncRead, AsyncReadExt};
use tracing::{error, info};

use crate::application::database::system::access::Auth;
use crate::application::database::DatabaseRootAccess;

impl DatabaseRootAccess {
    pub async fn import(&self, import: impl IntoImport) -> anyhow::Result<()> {
        info!("attempting database import");
        let result: anyhow::Result<()> = try {
            let content = import.into_import().await?;
            self.datastore
                .import(&content, self.auth.as_session())
                .await?;
        };
        result.inspect_err(|err| error!(?err, "database import failed due to error"))?;
        Ok(())
    }
}

pub trait IntoImport {
    async fn into_import(self) -> anyhow::Result<String>;
}

impl<R: Read + Send + 'static> IntoImport for BufReader<R> {
    async fn into_import(self) -> anyhow::Result<String> {
        let mut reader = self;
        let mut string = String::new();
        reader.read_to_string(&mut string)?;
        Ok(string)
    }
}

impl<R: AsyncRead + Unpin + Send + 'static> IntoImport for tokio::io::BufReader<R> {
    async fn into_import(self) -> anyhow::Result<String> {
        let mut reader = self;
        let mut string = String::new();
        reader.read_to_string(&mut string).await?;
        Ok(string)
    }
}

impl IntoImport for String {
    async fn into_import(self) -> anyhow::Result<String> {
        Ok(self)
    }
}

impl IntoImport for &str {
    async fn into_import(self) -> anyhow::Result<String> {
        Ok(self.to_string())
    }
}

impl IntoImport for &String {
    async fn into_import(self) -> anyhow::Result<String> {
        Ok(self.clone())
    }
}
