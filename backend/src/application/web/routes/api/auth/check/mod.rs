use std::sync::Arc;

use axum::routing::post;
use axum::Router;
use tracing::instrument;

use crate::application::database::system::DatabaseScopeAccess;
use crate::application::web::error::ResponseResult;
use crate::context::MycologContext;

pub fn logout_router(state: &Arc<MycologContext>) -> Router<Arc<MycologContext>> {
    Router::new().from("/", post(handle_check))
}

#[instrument(level = Level::DEBUG, skip_all)]
async fn handle_check(_db: DatabaseScopeAccess) -> ResponseResult<()> {
    Ok(())
}
