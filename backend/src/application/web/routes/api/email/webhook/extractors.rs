use std::sync::Arc;

use anyhow::anyhow;
use async_trait::async_trait;
use axum::body::{Body, Bytes};
use axum::extract::{FromRequest, Request};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{Json, RequestExt};
use hmac::{Hmac, Mac};
use image::EncodableLayout;
use serde_json::Value;
use sha2::Sha256;

use crate::application::email::events::EmailWebhookEvent;
use crate::application::web::error::ResponseError;
use crate::application::web::error::ResponseErrorExt;
use crate::context::MycologContext;
use crate::secrets::MycologSecrets;

pub struct Signed<E>(pub E);

type HmacSha256 = Hmac<Sha256>;

impl AsRef<MycologSecrets> for Arc<MycologContext> {
    fn as_ref(&self) -> &MycologSecrets {
        &self.secrets
    }
}

// Reusability: Define abstract way of fetching signing key
#[async_trait]
impl<S: AsRef<MycologSecrets> + Send + Sync, E: FromRequest<S, Rejection: IntoResponse>>
    FromRequest<S> for Signed<E>
{
    type Rejection = ResponseError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let (req_parts, req_body) = req.into_parts();
        let signature_str = req_parts
            .headers
            .get("signature")
            .ok_or(anyhow!("no `signature` header field").with_code(StatusCode::BAD_REQUEST))?
            .to_str()
            .map_err(|err| {
                anyhow!("`signature` header field is no readable string: {:?}", err)
                    .with_code(StatusCode::BAD_REQUEST)
            })?;
        let expected_signature = hex::decode(signature_str).map_err(|err| {
            anyhow!("`signature` header field is no hex number: {:?}", err)
                .with_code(StatusCode::BAD_REQUEST)
        })?;
        let body_bytes =
            Bytes::from_request(Request::from_parts(req_parts.clone(), req_body), state)
                .await
                .map_err(ResponseError::from_response)?;

        let mut hmac =
            HmacSha256::new_from_slice(state.as_ref().keys.mailersend_webhook().as_bytes())
                .map_err(|err| {
                    anyhow!("server defined invalid signing key: {:?}", err)
                        .with_code(StatusCode::INTERNAL_SERVER_ERROR)
                })?;
        hmac.update(&body_bytes);
        hmac.verify_slice(&expected_signature).map_err(|err| {
            anyhow!("`signature` header does not match content: {:?}", err)
                .with_code(StatusCode::UNAUTHORIZED)
        })?;

        let req = Request::from_parts(req_parts, Body::from(body_bytes));
        let inner = E::from_request(req, state)
            .await
            .map_err(ResponseError::from_response)?;

        Ok(Self(inner))
    }
}

#[async_trait]
impl<S: Send + Sync> FromRequest<S> for EmailWebhookEvent {
    type Rejection = ResponseError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(value) = Json::<Value>::from_request(req, state)
            .await
            .map_err(ResponseError::from_response)?;
        let event = EmailWebhookEvent::try_from(value)
            .map_err(|err: anyhow::Error| err.with_code(StatusCode::UNPROCESSABLE_ENTITY))?;
        Ok(event)
    }
}
