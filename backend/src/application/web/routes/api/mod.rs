use std::sync::Arc;

use axum::{middleware, Router};

use crate::application::web::routes::api::admin::authorize_admin;
use crate::application::web::routes::api::auth::auth_router;
use crate::application::web::routes::api::data::data_router;
use crate::context::MycologContext;

use self::email::email_router;

mod admin;
mod auth;
mod data;
mod email;

pub fn api_router(context: &Arc<MycologContext>) -> Router<Arc<MycologContext>> {
    Router::new()
        .nest("/email", email_router(context))
        .nest("/auth", auth_router(context))
        .nest("/data", data_router(context))
        .route_layer(middleware::from_fn_with_state(
            Arc::clone(context),
            authorize_admin,
        ))
}
