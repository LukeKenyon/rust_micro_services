use crate::models::address::Address;
use crate::models::contact::Contact;
use mongodb::bson::oid::ObjectId;
use mongodb::bson::{DateTime, Document};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Customer {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub primary_phone: String,
    pub name: String,
    pub iso_country_code: String,
    pub addresses: Vec<Address>,
    pub contacts: Vec<Contact>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewCustomerRequest {
    pub name: String,
    pub primary_phone: String,
    pub iso_country_code: String,
}

impl Customer {
    pub fn create_new(request: NewCustomerRequest) -> Self {
        let now = DateTime::now();
        Customer {
            id: None,
            primary_phone: request.primary_phone,
            name: request.name,
            iso_country_code: request.iso_country_code,
            addresses: Vec::new(),
            contacts: Vec::new(),
            created_at: now,
            updated_at: now,
        }
    }

    pub fn add_address(&mut self, address: Address) {
        self.addresses.push(address);
    }

    pub fn add_contact(&mut self, contact: Contact) {
        self.contacts.push(contact);
    }
}
