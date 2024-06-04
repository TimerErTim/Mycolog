use std::collections::BTreeMap;

use crate::application::web::routes::api::data::query::data::QueryRequest;
use serde::Deserialize;
use serde_json::Value;

#[derive(Clone, Deserialize)]
pub struct MultiRequest {
    pub requests: BTreeMap<String, QueryRequest>,
}
