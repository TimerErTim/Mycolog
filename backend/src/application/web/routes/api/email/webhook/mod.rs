use std::sync::Arc;

use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::post;
use axum::{Json, Router};
use serde_json::Value;
use tracing::Level;
use tracing::{info, instrument};

use crate::application::web::routes::api::email::webhook::extractors::Signed;
use crate::context::MycologContext;

mod channel;
mod events;
mod extractors;

#[instrument(level = Level::DEBUG)]
pub fn email_webhook_router() -> Router<Arc<MycologContext>> {
    Router::new().route("/", post(handle_webhook_request))
}

async fn handle_webhook_request(Signed(Json(value)): Signed<Json<Value>>) -> StatusCode {
    info!(?value, "received signed json value");
    StatusCode::OK
}
