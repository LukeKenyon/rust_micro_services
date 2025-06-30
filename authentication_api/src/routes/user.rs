use crate::models::user::NewUserRequest;
use crate::{handlers::user::UserHandler, services::certification::CertificateService};
use actix_web::{Error, HttpResponse, Responder, get, post, web};
use actix_web_httpauth::extractors::bearer::BearerAuth;

#[post("/user/register")]
/// Register a new user,stuff
async fn register_user(
    user_handler: web::Data<UserHandler>,
    new_user: web::Json<NewUserRequest>,
) -> impl Responder {
    match user_handler.create_user(new_user.into_inner()).await {
        Ok(user_response) => HttpResponse::Ok().json(user_response),
        Err(err_msg) => HttpResponse::InternalServerError().body(err_msg),
    }
}
#[get("/user/{id}")]
/// Get a user by ID, requires "user:read" scope
async fn get_user(
    user_handler: web::Data<UserHandler>,
    cert_handler: web::Data<CertificateService>,
    id: web::Path<String>,
    auth: BearerAuth,
) -> Result<HttpResponse, Error> {
    let token = auth.token();
    let id_ref = id.as_str();

    // Validate token scope and token
    match cert_handler.has_scope(token, "user:read") {
        Ok(()) => match user_handler.find_user_by_id(id_ref).await {
            Ok(user_response) => Ok(HttpResponse::Ok().json(user_response)),
            Err(err_msg) => Ok(HttpResponse::InternalServerError().body(err_msg)),
        },
        Err(resp) => Ok(resp),
    }
}
