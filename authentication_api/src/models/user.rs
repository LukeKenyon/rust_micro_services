use mongodb::bson::DateTime;
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub email: String,
    pub password: String,
    pub scopes: Vec<String>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Serialize, Deserialize)]
/// Implementation of NewUserRequest struct, used for user registration
pub struct NewUserRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
/// Implementation of UserResponse struct, used for user response
pub struct UserResponse {
    pub id: Option<ObjectId>,
    pub email: String,
    pub scopes: Vec<String>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

/// Implementation of User for creating  new user from NewUserRequest
impl User {
    pub fn create_new(new_user: NewUserRequest) -> Self {
        let now = DateTime::now();
        User {
            id: Some(ObjectId::new()),
            email: new_user.email,
            password: new_user.password,
            scopes: vec![],
            created_at: now,
            updated_at: now,
        }
    }

    /// Implementation of UserResponse for converting User to UserResponse
    pub fn to_user_response(&self) -> UserResponse {
        UserResponse {
            id: self.id.clone(),
            email: self.email.clone(),
            scopes: self.scopes.clone(),
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}
