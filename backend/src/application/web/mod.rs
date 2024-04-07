use std::sync::Arc;

use tracing::{error, info};

use crate::application::web::service::web_server_service;
use crate::context::MycologContext;
use crate::shutdown::exit::init_exit;

mod error;
mod routes;
mod service;

pub async fn web_server_task(context: Arc<MycologContext>) {
    let shutdown_token = context.task_cancel_token.clone();

    if let Err(err) = web_server_service(context, shutdown_token).await {
        error!(?err, "web server service crashed, shutting down...");
        init_exit(42);
    }
    info!("stopped web server service");
}
