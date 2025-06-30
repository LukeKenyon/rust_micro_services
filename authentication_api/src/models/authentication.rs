use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
/// Implementation of Login struct, used for user login
pub struct Login {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
/// Implementation of LoginResponse struct, used for user login response
pub struct LoginResponse {
    pub token: String,
    pub message: String,
    pub refresh_token: String,
}
