use std::collections::BTreeMap;

use crate::application::web::routes::api::data::query::data::QueryRequest;
use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct MultiRequest(pub BTreeMap<String, QueryRequest>);
