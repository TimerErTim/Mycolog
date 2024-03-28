use anyhow::bail;
use surrealdb_core::err::Error;
use surrealdb_core::kvs::LockType::Optimistic;
use surrealdb_core::kvs::Transaction;
use surrealdb_core::kvs::TransactionType::Read;
use tracing::trace;
use tracing::{instrument, warn};

use crate::application::database::DatabaseRootAccess;

impl DatabaseRootAccess {
    #[instrument(skip_all)]
    pub async fn health(&self) -> anyhow::Result<()> {
        match self.datastore.transaction(Read, Optimistic).await {
            Ok(mut tx) => {
                // Cancel the transaction
                trace!("cancelling health transaction");
                if let Err(err) = tx.cancel().await {
                    warn!(?err, "health transaction cancellation failed");
                }
                // Health is ok
                Ok(())
            }
            Err(err) => bail!(err),
        }
    }
}
