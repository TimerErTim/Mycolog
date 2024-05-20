use std::sync::Arc;

use axum::body::Body;
use axum::http::{header, HeaderMap};
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::Router;
use tokio_util::io::ReaderStream;

use crate::application::database::DatabaseRootAccess;
use crate::application::web::error::ResponseResult;
use crate::context::MycologContext;

pub fn backup_router(context: &Arc<MycologContext>) -> Router<Arc<MycologContext>> {
    Router::new().route("/", get(handle_backup))
}

async fn handle_backup(db: DatabaseRootAccess) -> ResponseResult<Response> {
    let compressed_reader = db.backup().await?;

    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, "application/brotli".parse()?);
    Ok((
        headers,
        Body::from_stream(ReaderStream::new(compressed_reader)),
    )
        .into_response())
}
