use std::sync::Arc;

use axum::Router;
use tracing::instrument;

use crate::context::MycologContext;

use self::webhook::email_webhook_router;

mod webhook;

#[instrument(level = Level::DEBUG)]
pub fn email_router() -> Router<Arc<MycologContext>> {
    Router::new().nest("/webhook", email_webhook_router())
}
