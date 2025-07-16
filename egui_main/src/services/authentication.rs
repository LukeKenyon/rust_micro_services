use crate::LoginResponse;
use reqwest::Client;
use serde::Serialize;
use tokio::sync::mpsc;

#[derive(Serialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

pub struct AuthService {
    client: Client,
    current_token: Option<String>,
    api_base_url: String,
}

impl AuthService {
    pub fn new(api_base_url: String) -> Self {
        Self {
            client: Client::new(),
            current_token: None,
            api_base_url,
        }
    }

    pub fn set_token(&mut self, token: String) {
        self.current_token = Some(token);
    }

    pub fn get_token(&self) -> Option<&String> {
        self.current_token.as_ref()
    }

    pub fn is_token_expired(&self, _token: &str) -> bool {
        // TODO: Implement proper JWT expiration check
        // For now, assume token is always valid
        false
    }

    pub fn login_async(
        &self,
        email: String,
        password: String,
        sender: mpsc::UnboundedSender<Result<LoginResponse, String>>,
    ) {
        let client = self.client.clone();
        let api_url = format!("{}/auth/login", self.api_base_url);

        tokio::spawn(async move {
            let login_request = LoginRequest { email, password };

            let result = client.post(&api_url).json(&login_request).send().await;

            let response = match result {
                Ok(response) => {
                    if response.status().is_success() {
                        match response.json::<LoginResponse>().await {
                            Ok(login_response) => Ok(login_response),
                            Err(e) => Err(format!("Failed to parse response: {}", e)),
                        }
                    } else {
                        Err(format!("Login failed: {}", response.status()))
                    }
                }
                Err(e) => Err(format!("Network error: {}", e)),
            };

            let _ = sender.send(response);
        });
    }
}

impl Default for AuthService {
    fn default() -> Self {
        Self::new("http://localhost:8080".to_string())
    }
}
