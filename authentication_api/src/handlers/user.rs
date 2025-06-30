use crate::models::user::{NewUserRequest, User, UserResponse};
use crate::services::user::UserService;
use crate::utils::password_utils::verify_password;
use mongodb::bson::{DateTime, Document};

pub struct UserHandler {
    user_service: UserService,
}

/// Implementation of UserHandler
impl UserHandler {
    pub async fn new() -> Self {
        Self {
            user_service: UserService::new().await,
        }
    }
    /// Create a new user
    pub async fn create_user(&self, user_request: NewUserRequest) -> Result<UserResponse, String> {
        let new_user = User::create_new(user_request);
        match self.user_service.create_user(new_user).await {
            Ok(Some(user)) => Ok(user.to_user_response()),
            Ok(None) => Err("User Not Created".to_string()),
            Err(e) => Err(format!("DatabaseError: {}", e)),
        }
    }

    //// Get a user by id
    pub async fn find_user_by_id(&self, id: &str) -> Result<UserResponse, String> {
        match self.user_service.find_by_id(id).await {
            Ok(Some(user)) => Ok(user.to_user_response()),
            Ok(None) => Err("User not found".to_string()),
            Err(e) => Err(format!("DatabaseError: {}", e)),
        }
    }

    ////Get user by email
    pub async fn find_user_by_email(&self, email: &str) -> Result<UserResponse, String> {
        match self.user_service.find_by_email(email).await {
            Ok(Some(user)) => Ok(user.to_user_response()),
            Ok(None) => Err("User not found".to_string()),
            Err(e) => Err(format!("DatabaseError: {}", e)),
        }
    }

    //// Updates the user scopes
    pub async fn update_user_scopes(
        &self,
        id: &str,
        scopes: Vec<String>,
    ) -> Result<UserResponse, String> {
        let mut update_doc = Document::new();
        update_doc.insert("scopes", mongodb::bson::to_bson(&scopes).unwrap());
        update_doc.insert("updated_at", DateTime::now());
        match self.user_service.update_user(id, update_doc).await {
            Ok(Some(user)) => Ok(user.to_user_response()),
            Ok(None) => Err("User not found".to_string()),
            Err(e) => Err(format!("DatabaseError: {}", e)),
        }
    }

    /// Updates the user password
    pub async fn update_user_password(
        &self,
        id: &str,
        password: &str,
    ) -> Result<UserResponse, String> {
        let mut update_doc = Document::new();
        update_doc.insert("password", mongodb::bson::to_bson(password).unwrap());
        update_doc.insert("updated_at", DateTime::now());
        match self.user_service.update_user(id, update_doc).await {
            Ok(Some(user)) => Ok(user.to_user_response()),
            Ok(None) => Err("User not found".to_string()),
            Err(e) => Err(format!("DatabaseError: {}", e)),
        }
    }

    /// Logs in a user by email and password
    pub async fn login_user(&self, email: &str, password: &str) -> Result<UserResponse, String> {
        match self.user_service.find_by_email(email).await {
            Ok(Some(user)) => {
                let user_hash = user.password.clone();

                match verify_password(password, &user_hash) {
                    Ok(is_valid) => {
                        if is_valid {
                            Ok(user.to_user_response())
                        } else {
                            Err("Invalid credentials".to_string())
                        }
                    }
                    Err(e) => Err(format!("Password verification error: {}", e)),
                }
            }
            Ok(None) => Err("User not found".to_string()),
            Err(e) => Err(format!("DatabaseError: {}", e)),
        }
    }
}
