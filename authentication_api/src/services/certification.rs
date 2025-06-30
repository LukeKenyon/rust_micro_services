use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};

use actix_web::{Error, HttpResponse, dev::ServiceRequest};
use actix_web_httpauth::extractors::AuthenticationError;
use actix_web_httpauth::extractors::bearer::{BearerAuth, Config};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub iat: usize,
    // Add other claims as needed
    pub email: Option<String>,
    pub scopes: Vec<String>,
}

pub struct CertificateService {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
}

/// Creates a new instance of the CertificateService
impl CertificateService {
    pub fn new(
        private_key_path: &str,
        public_key_path: &str,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let private_key = fs::read(private_key_path)?;
        let public_key = fs::read(public_key_path)?;

        let encoding_key = EncodingKey::from_rsa_pem(&private_key)?;
        let decoding_key = DecodingKey::from_rsa_pem(&public_key)?;

        Ok(CertificateService {
            encoding_key,
            decoding_key,
        })
    }

    /// Creates a new JWT token for a user
    pub fn create_token(
        &self,
        user_id: &str,
        email: Option<String>,
        scopes: Vec<String>,
    ) -> Result<String, jsonwebtoken::errors::Error> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as usize;

        let claims = Claims {
            sub: user_id.to_string(),
            exp: now + 3600, // Token expires in 1 hour
            iat: now,
            email,
            scopes,
        };

        let header = Header::new(Algorithm::RS256);
        encode(&header, &claims, &self.encoding_key)
    }

    /// Verifies a JWT token and returns the claims
    pub fn verify_token(&self, token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
        let validation = Validation::new(Algorithm::RS256);
        let token_data = decode::<Claims>(token, &self.decoding_key, &validation)?;
        Ok(token_data.claims)
    }

    /// Checks if a JWT token is valid and has a specific scope, extension
    /// to ensure the token is valid and has the required scope.
    pub fn has_scope(&self, token: &str, required_scope: &str) -> Result<(), HttpResponse> {
        match self.verify_token(token) {
            Ok(claims) => {
                if claims.scopes.contains(&required_scope.to_string()) {
                    Ok(())
                } else {
                    Err(HttpResponse::Unauthorized().body("Required access scope not found"))
                }
            }
            Err(_) => Err(HttpResponse::Unauthorized().body("Invalid token")),
        }
    }
}
