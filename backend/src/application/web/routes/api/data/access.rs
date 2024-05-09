use std::sync::Arc;

use anyhow::anyhow;
use async_trait::async_trait;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::http::StatusCode;

use crate::application::database::system::{AuthToken, DatabaseScopeAccess};
use crate::application::web::error::{ResponseError, ResponseErrorExt};
use crate::application::web::routes::api::admin::AdminStatus;
use crate::context::MycologContext;

#[async_trait]
impl FromRequestParts<Arc<MycologContext>> for DatabaseScopeAccess {
    type Rejection = ResponseError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &Arc<MycologContext>,
    ) -> Result<Self, Self::Rejection> {
        let admin_status = parts
            .extensions
            .get::<AdminStatus>()
            .cloned()
            .unwrap_or(AdminStatus::Unauthorized);
        if let AdminStatus::Authorized = admin_status {
            return Ok(state.db.auth_root().into_scoped());
        }

        let auth = AuthToken::from_request_parts(parts, state).await?;
        state.db.auth_token(auth).await.map_err(|err| {
            anyhow!("unable to authorize token: {err:?}").with_code(StatusCode::UNAUTHORIZED)
        })
    }
}
