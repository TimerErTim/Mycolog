use std::sync::Arc;

use anyhow::anyhow;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::routing::post;
use axum::{Json, Router};
use axum_extra::extract::cookie::Cookie;
use axum_extra::extract::CookieJar;
use email_address_parser::EmailAddress;
use tracing::{debug, info, instrument, Level};

use crate::application::database::system::{AuthToken, DatabaseScopeAccess};
use crate::application::web::error::{ResponseErrorExt, ResponseResult};
use crate::application::web::routes::api::auth::cookie::build_auth_cookie;
use crate::context::MycologContext;

pub fn logout_router(state: &Arc<MycologContext>) -> Router<Arc<MycologContext>> {
    Router::new().route("/", post(handle_logout))
}

#[instrument(level = Level::DEBUG, skip_all)]
async fn handle_logout(
    State(context): State<Arc<MycologContext>>,
    db: DatabaseScopeAccess,
    jar: CookieJar,
) -> ResponseResult<CookieJar> {
    let email = db
        .query("SELECT VALUE email FROM ONLY $auth.id;")
        .await
        .and_then(|mut response| response.take::<Option<String>>(0))
        .map_err(|err| {
            err.context("cannot retrieve email from authorized account")
                .with_code(StatusCode::UNAUTHORIZED)
        })?;

    debug!(?email, "received logout request");

    Ok(jar.remove(Cookie::from("auth")))
}
