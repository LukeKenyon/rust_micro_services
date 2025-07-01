use crate::models::address::Address;
use crate::models::contact::Contact;
use crate::models::customer::{Customer, NewCustomerRequest};
use crate::services::customer::CustomerService;
use mongodb::bson::{DateTime, Document};

pub struct CustomerHandler {
    customer_service: CustomerService,
}

impl CustomerHandler {
    pub async fn new() -> Self {
        CustomerHandler {
            customer_service: CustomerService::new().await,
        }
    }

    pub async fn create_customer(
        &self,
        customer_request: NewCustomerRequest,
    ) -> Result<Customer, String> {
        let new_customer = Customer::create_new(customer_request);
        match self.customer_service.create_customer(new_customer).await {
            Ok(customer) => Ok(customer),
            Err(err) => Err(err.to_string()),
        }
    }

    pub async fn get_customer(&self, id: &str) -> Result<Customer, String> {
        match self.customer_service.get_customer(id).await {
            Ok(customer) => Ok(customer),
            Err(err) => Err(err.to_string()),
        }
    }

    pub async fn add_address(&self, id: &str, address: Address) -> Result<Customer, String> {
        match self.customer_service.get_customer(id).await {
            Ok(mut customer) => {
                let mut addresses = customer.addresses;
                addresses.push(address);
                let mut update_doc = Document::new();
                update_doc.insert("addresses", mongodb::bson::to_bson(&addresses).unwrap());
                update_doc.insert("updated_at", DateTime::now());

                match self.customer_service.update_customer(id, update_doc).await {
                    Ok(customer) => Ok(customer),
                    Err(err) => Err(err.to_string()),
                }
            }
            Err(err) => Err(err.to_string()),
        }
    }

    pub async fn add_contact(&self, id: &str, contact: Contact) -> Result<Customer, String> {
        match self.customer_service.get_customer(id).await {
            Ok(mut customer) => {
                let mut contacts = customer.contacts;
                contacts.push(contact);
                let mut update_doc = Document::new();
                update_doc.insert("contacts", mongodb::bson::to_bson(&contacts).unwrap());
                update_doc.insert("updated_at", DateTime::now());

                match self.customer_service.update_customer(id, update_doc).await {
                    Ok(customer) => Ok(customer),
                    Err(err) => Err(err.to_string()),
                }
            }
            Err(err) => Err(err.to_string()),
        }
    }
}
