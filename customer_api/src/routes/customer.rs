use crate::messaging::publisher::{Publish, Publisher};
use crate::models::address::{Address, NewAddressRequest};
use crate::models::contact::{Contact, NewContactRequest};
use crate::models::customer::NewCustomerRequest;
use crate::{handlers::customer::CustomerHandler, services::certification::CertificateService};
use actix::Addr;
use actix_web::{Error, HttpResponse, Responder, get, post, web};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use axum::middleware::IntoMapRequestResult;

use log::info;

#[post("/customer/create")]
async fn create_customer(
    customer_handler: web::Data<CustomerHandler>,
    cert_handler: web::Data<CertificateService>,
    publisher: web::Data<Addr<Publisher>>,
    new_customer: web::Json<NewCustomerRequest>,
    auth: BearerAuth,
) -> Result<HttpResponse, Error> {
    let token = auth.token();

    match cert_handler.has_scope(token, "customer:manager") {
        Ok(()) => match customer_handler
            .create_customer(new_customer.into_inner())
            .await
        {
            Ok(handler_response) => {
                let msg = Publish {
                    routing_key: "customer.created".to_string(),
                    payload: handler_response.clone(),
                };
                if let Err(_) = publisher.send(msg).await {
                    return Ok(HttpResponse::InternalServerError().finish());
                }
                Ok(HttpResponse::Ok().json(handler_response))
            }
            Err(err_msg) => Ok(HttpResponse::InternalServerError().body(err_msg)),
        },
        Err(resp) => Ok(resp),
    }
}

#[post("/address/add/{id}")]
async fn add_address(
    customer_handler: web::Data<CustomerHandler>,
    cert_handler: web::Data<CertificateService>,
    address_request: web::Json<NewAddressRequest>,
    publisher: web::Data<Addr<Publisher>>,
    id: web::Path<String>,
    auth: BearerAuth,
) -> Result<HttpResponse, Error> {
    let id_ref = id.as_str();
    let token = auth.token();
    let address = Address::create_new(address_request.into_inner());
    match cert_handler.has_scope(token, "customer:manager") {
        Ok(()) => match customer_handler.add_address(id_ref, address).await {
            Ok(handler_response) => {
                let msg = Publish {
                    routing_key: "customer.address.added".to_string(),
                    payload: handler_response.clone(),
                };
                if let Err(_) = publisher.send(msg).await {
                    return Ok(HttpResponse::InternalServerError().finish());
                }
                Ok(HttpResponse::Ok().json(handler_response))
            }
            Err(err_msg) => Ok(HttpResponse::InternalServerError().body(err_msg)),
        },
        Err(resp) => Ok(resp),
    }
}

#[post("/contact/add/{id}")]
async fn add_contact(
    customer_handler: web::Data<CustomerHandler>,
    cert_handler: web::Data<CertificateService>,
    contact_request: web::Json<NewContactRequest>,
    publisher: web::Data<Addr<Publisher>>,
    id: web::Path<String>,
    auth: BearerAuth,
) -> Result<HttpResponse, Error> {
    let id_ref = id.as_str();
    let token = auth.token();

    let contact = Contact::create_new(contact_request.into_inner());

    match cert_handler.has_scope(token, "customer:manager") {
        Ok(()) => match customer_handler.add_contact(id_ref, contact).await {
            Ok(handler_response) => {
                let msg = Publish {
                    routing_key: "customer.contact.added".to_string(),
                    payload: handler_response.clone(),
                };
                if let Err(_) = publisher.send(msg).await {
                    return Ok(HttpResponse::InternalServerError().finish());
                }
                Ok(HttpResponse::Ok().json(handler_response))
            }
            Err(err_msg) => Ok(HttpResponse::InternalServerError().body(err_msg)),
        },
        Err(resp) => Ok(resp),
    }
}
