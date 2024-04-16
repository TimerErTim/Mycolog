use std::sync::Arc;

use axum::Router;

use crate::context::MycologContext;

use self::webhook::email_webhook_router;

mod webhook;

pub fn email_router() -> Router<Arc<MycologContext>> {
    Router::new().nest("/webhook", email_webhook_router())
}
