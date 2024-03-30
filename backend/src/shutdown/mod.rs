use std::sync::{Arc, Mutex};

use lazy_static::lazy_static;
use tokio::sync::mpsc::Receiver;
use tokio::sync::mpsc::Sender;
use tracing::log::info;
use tracing::{error, instrument, warn};

use crate::context::MycologContext;

pub mod exit;

#[instrument(skip_all)]
pub async fn shutdown(state: Arc<MycologContext>) -> i32 {
    let shutdown_result = try_shutdown(state).await;

    match shutdown_result {
        Ok(_) => 0,
        Err(err) => {
            error!("Expierenced error during graceful shutdown: {err}");
            1
        }
    }
}

pub async fn try_shutdown(context: Arc<MycologContext>) -> anyhow::Result<()> {
    context.task_cancel_token.cancel();
    info!("waiting for background tasks to quit...");
    context.tasks.close();
    context.tasks.wait().await;
    info!("quitted all background tasks");

    match Arc::into_inner(context) {
        None => {
            warn!("some threads still made use of global context");
        }
        Some(context) => {
            drop(context);
        }
    };

    Ok(())
}
