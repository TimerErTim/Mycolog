use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct SignupCredentials {
    pub email: String,
    pub password: String,
}
