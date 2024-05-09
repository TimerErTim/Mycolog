use std::sync::Arc;

use axum::Router;

use crate::application::web::routes::api::data::image::image_router;
use crate::application::web::routes::api::data::query::query_router;
use crate::context::MycologContext;

mod access;
mod image;
mod query;

pub fn data_router(context: &Arc<MycologContext>) -> Router<Arc<MycologContext>> {
    Router::new()
        .nest("/query", query_router(context))
        .nest("/image", image_router(context))
}
