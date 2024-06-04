use std::sync::Arc;

use axum::http::HeaderValue;
use axum::{middleware, Router};
use tower_http::cors::{AllowCredentials, AllowHeaders, CorsLayer};

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
    let mut router = Router::new()
        .nest("/email", email_router(context))
        .nest("/auth", auth_router(context))
        .nest("/data", data_router(context))
        .route_layer(middleware::from_fn_with_state(
            Arc::clone(context),
            authorize_admin,
        ));

    if cfg!(feature = "dev-env") {
        // Enable cors support in dev environment for seperate frontend
        router = router.layer(
            CorsLayer::very_permissive()
                .allow_origin("http://localhost:5173".parse::<HeaderValue>().unwrap()),
        )
    }

    router
}
