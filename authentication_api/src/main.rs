use actix_web::middleware::Logger;
use actix_web::{App, HttpServer, web};
use authentication_api::handlers::authentication::AuthenticationHandler;
use authentication_api::handlers::user::UserHandler;
use authentication_api::routes::authentication::{login, verify_token};
use authentication_api::routes::user::{get_user, register_user};
use authentication_api::services::certification::CertificateService;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let auth_handler = AuthenticationHandler::new().await;
    let user_handler = UserHandler::new().await;
    let cert_handler =
        CertificateService::new("RSAKeyStore/private_key.pem", "RSAKeyStore/public_key.pem")
            .expect("Failed to create CertificateService");

    let handler_data = web::Data::new(auth_handler);
    let user_data = web::Data::new(user_handler);
    let cert_service = web::Data::new(cert_handler);

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(handler_data.clone())
            .app_data(user_data.clone())
            .app_data(cert_service.clone())
            .service(login)
            .service(verify_token)
            .service(register_user)
            .service(get_user)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

// use authentication_api::handlers::user::UserHandler;
// use authentication_api::services::certification::CertificateService;

// #[tokio::main]
// async fn main() {
//     println!("Starting Database...");

//     let user_handler = UserHandler::new().await;
//     let cert_service =
//         CertificateService::new("RSAKeyStore/private_key.pem", "RSAKeyStore/public_key.pem")
//             .expect("Failed to create CertificateService");

//     let user_pwd = "password101";
//     let user_email = "luke.kenyon@test.com";

//     let login_result = user_handler.login_user(user_email, user_pwd).await;

//     match &login_result {
//         Ok(_) => println!("Login succeeded"),
//         Err(e) => println!("Login failed: {:?}", e),
//     }

//     let login_data = login_result.unwrap();

//     let jwt_token = cert_service.create_token(
//         &login_data.id.unwrap().to_string(),
//         Some(login_data.email),
//         login_data.scopes,
//     );

//     let user_token = jwt_token.unwrap();
//     println!("jwt token data is {:?}", user_token);

//     println!(
//         "Has user:read: {:?}",
//         cert_service.has_scope(&user_token, "user:write")
//     );
// }
