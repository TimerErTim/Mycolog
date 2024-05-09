use std::collections::BTreeMap;

use serde::Deserialize;
use serde_json::Value;

#[derive(Clone, Deserialize)]
pub struct QueryRequest {
    pub statements: String,
    pub variables: Option<BTreeMap<String, Value>>,
}
