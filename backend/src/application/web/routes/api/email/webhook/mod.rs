use std::sync::Arc;

use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::post;
use axum::{Json, Router};
use serde_json::Value;
use tracing::{debug, Level};
use tracing::{info, instrument};

use crate::application::email::events::EmailWebhookEvent;
use crate::application::web::error::ResponseResult;
use crate::application::web::routes::api::email::webhook::extractors::Signed;
use crate::context::MycologContext;

mod events;
mod extractors;

pub fn email_webhook_router() -> Router<Arc<MycologContext>> {
    Router::new().route("/", post(handle_webhook_request))
}

async fn handle_webhook_request(
    State(context): State<Arc<MycologContext>>,
    Signed(event): Signed<EmailWebhookEvent>,
) -> ResponseResult<StatusCode> {
    debug!(?event, "received email webhook event");
    tokio::spawn(async move { context.email.process(event).await });
    Ok(StatusCode::OK)
}
