use std::sync::Arc;

use axum::Router;

use crate::application::web::routes::api::auth::logout::logout_router;
use crate::application::web::routes::api::auth::signin::signin_router;
use crate::application::web::routes::api::auth::signup::signup_router;
use crate::context::MycologContext;

mod cookie;
mod logout;
mod signin;
mod signup;
mod token;

pub fn auth_router(context: &Arc<MycologContext>) -> Router<Arc<MycologContext>> {
    Router::new()
        .nest("/signup", signup_router(context))
        .nest("/signin", signin_router(context))
        .nest("/logout", logout_router(context))
}
