use std::sync::Arc;

use axum::Router;
use tracing::instrument;

use crate::context::MycologContext;

use self::email::email_router;

mod email;

#[instrument(level = Level::DEBUG)]
pub fn api_router() -> Router<Arc<MycologContext>> {
    Router::new().nest("/email", email_router())
}
