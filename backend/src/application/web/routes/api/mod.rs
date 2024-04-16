use std::sync::Arc;

use axum::Router;

use crate::application::web::routes::api::auth::auth_router;
use crate::application::web::routes::api::db::db_router;
use crate::context::MycologContext;

use self::email::email_router;

mod auth;
mod db;
mod email;

pub fn api_router() -> Router<Arc<MycologContext>> {
    Router::new()
        .nest("/email", email_router())
        .nest("/auth", auth_router())
        .nest("/db", db_router())
}
