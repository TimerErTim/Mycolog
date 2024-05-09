use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::utils::serde::empty_string_as_none;

#[derive(Clone, Serialize, Deserialize)]
pub struct SigninCredentials {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct SigninOptions {
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub remember: Option<bool>,
}
