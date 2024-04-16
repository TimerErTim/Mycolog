use anyhow::anyhow;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::http::StatusCode;
use axum_extra::extract::CookieJar;

use crate::application::database::system::AuthToken;
use crate::application::web::error::{ResponseError, ResponseErrorExt};

impl<S> FromRequestParts<S> for AuthToken {
    type Rejection = ResponseError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let jar = CookieJar::from_request_parts(parts, state).await?;
        let Some(token) = jar.get("auth") else {
            return Err(anyhow!("no `auth` cookie in request").status(StatusCode::BAD_REQUEST));
        };
        token.value().into()
    }
}
