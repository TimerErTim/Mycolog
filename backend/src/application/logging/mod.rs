use std::sync::Arc;

use tracing::{error, info};

use crate::application::logging::service::logging_service;
use crate::context::MycologContext;

mod service;

pub async fn logging_task(context: Arc<MycologContext>) {
    let shutdown_token = context.task_cancel_token.clone();

    if let Err(err) = logging_service(shutdown_token).await {
        error!(?err, "logging service stopped working due to error")
    }
    info!("stopped logging service");
}
