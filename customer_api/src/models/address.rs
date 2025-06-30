use mongodb::bson::DateTime;
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Address {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub street: String,
    pub city: String,
    pub state: String,
    pub zip: String,
    pub country: String,
    pub iso_code: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

pub struct NewAddressRequest {
    pub street: String,
    pub city: String,
    pub state: String,
    pub zip: String,
    pub country: String,
    pub iso_code: String,
}

impl Address {
    pub fn create_new(request: NewAddressRequest) -> Self {
        let now = DateTime::now();
        Address {
            id: Some(ObjectId::new()),
            street: request.street,
            city: request.city,
            state: request.state,
            zip: request.zip,
            country: request.country,
            iso_code: request.iso_code,
            created_at: now,
            updated_at: now,
        }
    }
}
