use std::sync::Arc;

use axum::handler::HandlerWithoutStateExt;
use axum::Router;
use tower_http::services::{ServeDir, ServeFile};
use tracing::instrument;

use crate::context::MycologContext;

use self::api::api_router;

mod api;

#[instrument(level = tracing::Level::DEBUG, skip_all)]
pub fn try_build_routes(
    context: &Arc<MycologContext>,
) -> anyhow::Result<Router<Arc<MycologContext>>> {
    Ok(root_router(context))
}

fn root_router(context: &Arc<MycologContext>) -> Router<Arc<MycologContext>> {
    Router::new()
        .nest("/api", api_router(context))
        .fallback_service(ServeDir::new("site").fallback(ServeFile::new("site/404.html")))
}
