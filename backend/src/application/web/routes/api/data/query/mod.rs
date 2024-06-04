use std::sync::Arc;

use axum::http::StatusCode;
use axum::routing::post;
use axum::{Json, Router};
use surrealdb_core::sql::parse;

use crate::application::database::system::{DatabaseScopeAccess, Response};
use crate::application::web::error::{ResponseErrorExt, ResponseResult};
use crate::application::web::routes::api::data::query::data::QueryRequest;
use crate::context::MycologContext;

pub mod data;

pub fn query_router(context: &Arc<MycologContext>) -> Router<Arc<MycologContext>> {
    Router::new().route("/", post(handle_query))
}

pub async fn handle_query(
    db: DatabaseScopeAccess,
    Json(request): Json<QueryRequest>,
) -> ResponseResult<Json<Vec<Response>>> {
    let statements =
        parse(&request.statements).map_err(|err| err.with_code(StatusCode::BAD_REQUEST))?;

    let mut query = db.query(statements);
    if let Some(variables) = request.variables {
        for (variable, value) in variables {
            query = query.bind(&variable, value);
        }
    }
    let result = query.await?;
    Ok(Json(result.collect()))
}
