use crate::handlers::authentication::AuthenticationHandler;
use crate::models::authentication::Login;
use actix_web::{HttpResponse, Responder, post, web};

#[post("/auth/login")]
/// Logs in a user and returns a JWT token.
async fn login(
    login_data: web::Json<Login>,
    handler: web::Data<AuthenticationHandler>,
) -> impl Responder {
    match handler.login_user(login_data.into_inner()).await {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(e) => HttpResponse::Unauthorized().body(e),
    }
}

#[post("/auth/verify")]
/// Verifies a JWT token is valid, would be sent in the header.
async fn verify_token(
    token: web::Json<String>,
    handler: web::Data<AuthenticationHandler>,
) -> impl Responder {
    let token_str = token.as_str();
    match handler.verify_token(token_str).await {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(e) => HttpResponse::Unauthorized().body(e),
    }
}
