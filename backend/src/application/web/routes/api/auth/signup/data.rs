use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SignupCredentials {
    pub email: String,
    pub password: String,
}
