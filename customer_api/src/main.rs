use actix_web::middleware::Logger;
use actix_web::{App, HttpServer, web};
use customer_api::handlers::customer::CustomerHandler;
use customer_api::messaging::connection::{create_channel, declare_exchange};
use customer_api::messaging::publisher::Publisher;
use customer_api::routes::customer::{add_address, add_contact, create_customer};
use customer_api::services::certification::CertificateService;
use log::info;

use actix::Actor;
use lapin::ExchangeKind::Direct;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    info!("Starting Customer API...");

    let channel = create_channel().await.expect("Failed to create channel");
    declare_exchange(&channel, "customer_exchange", Direct).await;

    let publisher = Publisher {
        channel,
        exchange: "customer_exchange".into(),
    }
    .start();
    let customer_handler = CustomerHandler::new().await;
    let cert_handler = CertificateService::new("RSAKeyStore/public_key.pem")
        .expect("Failed to create CertificateService");
    let customer_data = web::Data::new(customer_handler);
    let cert_service = web::Data::new(cert_handler);

    let publisher_service = web::Data::new(publisher);
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(customer_data.clone())
            .app_data(cert_service.clone())
            .app_data(publisher_service.clone())
            .service(create_customer)
            .service(add_address)
            .service(add_contact)
    })
    .bind("127.0.0.1:8090")?
    .run()
    .await

    // let user_handler = UserHandler::new().await;
    // let cert_handler =
    //     CertificateService::new("RSAKeyStore/private_key.pem", "RSAKeyStore/public_key.pem")
    //         .expect("Failed to create CertificateService");

    // let handler_data = web::Data::new(auth_handler);
    // let user_data = web::Data::new(user_handler);
    // let cert_service = web::Data::new(cert_handler);
    // println!("Starting Database...");

    // let new_request = NewCustomerRequest {
    //     name: "Space design house".to_string(),
    //     primary_phone: "1234567890".to_string(),
    //     iso_country_code: "US".to_string(),
    // };

    // let new_address_request = NewAddressRequest {
    //     street: "123 Main St".to_string(),
    //     city: "Anytown".to_string(),
    //     state: "CA".to_string(),
    //     zip: "12345".to_string(),
    //     country: "US".to_string(),
    //     iso_code: "US".to_string(),
    // };

    // let address = Address::create_new(new_address_request);

    // let customer_id = "686153021f8d0d5b45e82113";

    // let customer_handler = CustomerHandler::new().await;

    // let result = customer_handler.get_customer(customer_id).await;

    // match result {
    //     Ok(_customer) => {
    //         println!("Retrieved customer successfully. {:?}", _customer);

    //         let result = customer_handler.add_address(customer_id, address).await;

    //         match result {
    //             Ok(_customer) => println!("Created customer successfully. {:?}", _customer),
    //             Err(e) => println!("Failed to create customer: {:?}", e),
    //         }
    //     }
    //     Err(e) => println!("Failed to retrieve customer: {:?}", e),
    // }

    // // let result = customer_handler.create_customer(new_request).await;

    // // match result {
    // //     Ok(_customer) => println!("Created customer successfully. {:?}", _customer),
    // //     Err(e) => println!("Failed to create customer: {:?}", e),
    // // }
}
