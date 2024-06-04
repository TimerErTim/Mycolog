mod data;

use crate::application::database::system::{DatabaseScopeAccess, Response};
use crate::application::web::error::ResponseResult;
use crate::application::web::routes::api::data::multi::data::MultiRequest;
use crate::application::web::routes::api::data::query::handle_query;
use crate::context::MycologContext;
use axum::routing::post;
use axum::{Json, Router};
use std::collections::BTreeMap;
use std::sync::Arc;

pub fn multi_router(context: &Arc<MycologContext>) -> Router<Arc<MycologContext>> {
    Router::new().route("/", post(handle_multi))
}

async fn handle_multi(
    db: DatabaseScopeAccess,
    Json(payload): Json<MultiRequest>,
) -> ResponseResult<Json<BTreeMap<String, Vec<Response>>>> {
    let mut responses = BTreeMap::new();

    for (id, request) in payload.requests {
        let result = handle_query(db.clone(), Json(request)).await;
        if let Ok(Json(response)) = result {
            responses.insert(id, response);
        }
    }

    Ok(Json(responses))
}
