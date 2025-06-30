use crate::models::authentication::{Login, LoginResponse};
use crate::services::certification::CertificateService;
use crate::services::user::UserService;
use crate::utils::password_utils::verify_password;

pub struct AuthenticationHandler {
    user_service: UserService,
    certificate_service: CertificateService,
}

/// Initializes a new instance of the AuthenticationHandler.
impl AuthenticationHandler {
    pub async fn new() -> Self {
        AuthenticationHandler {
            user_service: UserService::new().await,
            certificate_service: CertificateService::new(
                "RSAKeyStore/private_key.pem",
                "RSAKeyStore/public_key.pem",
            )
            .expect("Failed to create CertificateService"),
        }
    }

    /// Logs in a user with the provided credentials.
    pub async fn login_user(&self, login: Login) -> Result<LoginResponse, String> {
        match self.user_service.find_by_email(login.email.as_str()).await {
            Ok(Some(user)) => {
                let user_hash = user.password.clone();

                match verify_password(login.password.as_str(), &user_hash) {
                    Ok(is_valid) => {
                        if is_valid {
                            let jwt_token = self.certificate_service.create_token(
                                &user.id.unwrap().to_string(),
                                Some(user.email),
                                user.scopes,
                            );
                            Ok(LoginResponse {
                                token: jwt_token.unwrap(),
                                refresh_token: "refresh_token".to_string(),
                                message: "Successfully logged in".to_string(),
                            })
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

    /// Verifies a user's token.
    pub async fn verify_token(&self, token: &str) -> Result<bool, String> {
        match self.certificate_service.verify_token(token) {
            Ok(result) => Ok(true),
            Err(e) => Err(format!("Token verification error: {}", e)),
        }
    }
}
