use std::sync::Arc;

use anyhow::anyhow;
use axum::extract::State;
use axum::routing::post;
use axum::{Json, Router};
use axum_extra::extract::CookieJar;
use email_address_parser::EmailAddress;
use reqwest::StatusCode;
use serde_json::Value;
use tracing::{debug, error, info, instrument, Level};

use crate::application::email::Recipient;
use crate::application::web::error::{ResponseErrorExt, ResponseResult};
use crate::application::web::routes::api::auth::cookie::build_auth_cookie;
use crate::application::web::routes::api::auth::signup::data::SignupCredentials;
use crate::context::MycologContext;

mod data;

pub fn signup_router() -> Router<Arc<MycologContext>> {
    Router::new().route("/", post(handle_signup))
}

#[instrument(level = Level::DEBUG, skip_all, fields(email = ? credentials.email))]
async fn handle_signup(
    State(context): State<Arc<MycologContext>>,
    jar: CookieJar,
    Json(credentials): Json<SignupCredentials>,
) -> ResponseResult<CookieJar> {
    let email = credentials.email.clone();
    if !EmailAddress::is_valid(&email, None) {
        return Err(
            anyhow!("given email is no valid email addresse").status(StatusCode::BAD_REQUEST)
        );
    }

    debug!("received signup request");
    let token = context
        .db
        .signup("user", credentials)
        .await
        .map_err(|err| err.status(StatusCode::UNAUTHORIZED))?;
    /*tokio::spawn(async move {
        if let Err(err) = context.email.sumbit_email(
            "verify",
            "Verify your Mycolog Account",
            vec![Recipient::new(&email).bind("email_addresse", &email)]).await {
            error!(?err, recipient = %email, "unable to submit verification email");
        }
    });*/
    info!("approved signup request", email = ?credentials.email);

    let cookie = build_auth_cookie(token, false);
    Ok(jar.add(cookie))
}
