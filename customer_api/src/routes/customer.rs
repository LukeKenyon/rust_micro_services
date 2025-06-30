use crate::models::address::{Address, NewAddressRequest};
use crate::models::customer::NewCustomerRequest;
use crate::{handlers::customer::CustomerHandler, services::certification::CertificateService};
use actix_web::{Error, HttpResponse, Responder, get, post, web};
use actix_web_httpauth::extractors::bearer::BearerAuth;

#[post("/customer/create")]
async fn create_customer(
    customer_handler: web::Data<CustomerHandler>,
    new_customer: web::Json<NewCustomerRequest>,
) -> impl Responder {
    match customer_handler
        .create_customer(new_customer.into_inner())
        .await
    {
        Ok(customer_response) => HttpResponse::Ok().json(customer_response),
        Err(err_msg) => HttpResponse::InternalServerError().body(err_msg),
    }
}

#[post("/address/create/{id}")]
async fn create_address(
    customer_handler: web::Data<CustomerHandler>,
    address_request: web::Json<NewAddressRequest>,
    id: web::Path<String>,
) -> impl Responder {
    let id_ref = id.as_str();

    let address = Address::create_new(address_request.into_inner());
    match customer_handler.add_address(id_ref, address).await {
        Ok(address_response) => HttpResponse::Ok().json(address_response),
        Err(err_msg) => HttpResponse::InternalServerError().body(err_msg),
    }
}
