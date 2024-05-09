use std::future::Future;
use std::sync::Arc;

use anyhow::{anyhow, Error};
use axum::body::{Body, Bytes};
use axum::extract::{DefaultBodyLimit, Path, State};
use axum::http::{header, HeaderMap, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use axum::{Json, Router};
use axum_extra::extract::multipart::MultipartError;
use axum_extra::extract::Multipart;
use surrealdb_core::sql;
use surrealdb_core::sql::parse;
use tracing::{debug, error, instrument, trace, warn, Level};

use crate::application::database::system::DatabaseScopeAccess;
use crate::application::images::StoreImageError;
use crate::application::web::error::{ResponseErrorExt, ResponseResult};
use crate::context::MycologContext;
use crate::utils::codec::{json_encoded_to_utf8, utf8_to_json_encoded};

const MB: usize = 2usize.pow(20);

pub fn image_router(context: &Arc<MycologContext>) -> Router<Arc<MycologContext>> {
    Router::new().route("/*id", get(handle_image_get)).route(
        "/",
        post(handle_image_post).layer(DefaultBodyLimit::max(10 * MB)),
    )
}

#[instrument(level = Level::DEBUG, skip_all)]
async fn handle_image_post(
    State(context): State<Arc<MycologContext>>,
    db: DatabaseScopeAccess,
    headers: HeaderMap,
    bytes: Bytes,
) -> ResponseResult<String> {
    let escaped_file_name = headers
        .get("content-name")
        .ok_or(anyhow!("header `content-name` missing").with_code(StatusCode::BAD_REQUEST))?
        .to_str()
        .map_err(|err| err.with_code(StatusCode::BAD_REQUEST))?;
    let file_name = json_encoded_to_utf8(escaped_file_name)?;

    let thing = context
        .images
        .store_image(&db, &file_name, bytes)
        .await
        .inspect_err(|err| error!(%err, "unable to store image"))
        .map_err(|err| match &err {
            StoreImageError::StorageExceeded { .. } => {
                err.with_code(StatusCode::INSUFFICIENT_STORAGE)
            }
            StoreImageError::InvalidFormat => err.with_code(StatusCode::UNSUPPORTED_MEDIA_TYPE),
            StoreImageError::Unauthorized => err.with_code(StatusCode::UNAUTHORIZED),
            _ => err.into(),
        })?;

    Ok(thing.to_raw())
}

#[instrument(level = Level::DEBUG, skip_all, fields(id = % id))]
async fn handle_image_get(
    State(context): State<Arc<MycologContext>>,
    db: DatabaseScopeAccess,
    Path(id): Path<String>,
) -> ResponseResult<Response> {
    trace!("retrieving image `{id}`");
    let data = context
        .images
        .get_image(&db, id.clone())
        .await
        .inspect_err(|err| debug!(?err, "image `{id}` not retrievable"))
        .map_err(|err| err.with_code(StatusCode::NOT_FOUND))?;
    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, data.info.file_type.parse()?);
    headers.insert(
        header::LAST_MODIFIED,
        data.info.time_created.to_rfc2822().parse()?,
    );
    headers.insert(
        "content-name",
        utf8_to_json_encoded(&data.info.file_name)?.parse()?,
    );
    Ok((headers, Body::from_stream(data.bytes)).into_response())
}
