use std::sync::Arc;
use std::time::Duration;

use anyhow::anyhow;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::routing::post;
use axum::{Json, Router};
use axum_extra::extract::cookie::{Cookie, SameSite};
use axum_extra::extract::CookieJar;
use email_address_parser::EmailAddress;
use tracing::{debug, info, instrument, trace, Level};

use crate::application::web::error::{ResponseErrorExt, ResponseResult};
use crate::application::web::routes::api::auth::cookie::build_auth_cookie;
use crate::application::web::routes::api::auth::signin::data::{SigninCredentials, SigninOptions};
use crate::context::MycologContext;

mod data;

pub fn signin_router(state: &Arc<MycologContext>) -> Router<Arc<MycologContext>> {
    Router::new().route("/", post(handle_signin))
}

#[instrument(level = Level::DEBUG, skip_all, fields(? options.remember, email = ? credentials.email))]
async fn handle_signin(
    State(context): State<Arc<MycologContext>>,
    Query(options): Query<SigninOptions>,
    jar: CookieJar,
    Json(credentials): Json<SigninCredentials>,
) -> ResponseResult<CookieJar> {
    if !EmailAddress::is_valid(&credentials.email, None) {
        return Err(
            anyhow!("given email is no valid email addresse").with_code(StatusCode::BAD_REQUEST)
        );
    }

    debug!("received signin request");
    let token = context
        .db
        .signin("user", credentials.clone())
        .await
        .map_err(|err| err.with_code(StatusCode::UNAUTHORIZED))?;
    info!(email = ?credentials.email, "approved signin request");

    let cookie = build_auth_cookie(token, options.remember.unwrap_or(false));
    Ok(jar.add(cookie))
}
