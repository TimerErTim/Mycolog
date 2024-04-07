use std::sync::Arc;

use axum::Router;
use tracing::instrument;

use crate::context::MycologContext;
use crate::utils::asynchronous::run_catch_blocking;

use self::api::api_router;

mod api;

#[instrument(level = Level::DEBUG)]
pub fn try_build_routes() -> anyhow::Result<Router<Arc<MycologContext>>> {
    run_catch_blocking(|| Ok(root_router()))
}

fn root_router() -> Router<Arc<MycologContext>> {
    Router::new().nest("/api", api_router())
}
