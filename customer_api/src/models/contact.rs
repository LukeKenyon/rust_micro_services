use mongodb::bson::DateTime;
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Contact {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub title: String,
    pub name: String,
    pub email: String,
    pub phone: String,
    pub mobile: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewContactRequest {
    pub title: String,
    pub name: String,
    pub email: String,
    pub phone: String,
    pub mobile: String,
}

impl Contact {
    pub fn create_new(request: NewContactRequest) -> Self {
        let now = DateTime::now();
        Contact {
            id: Some(ObjectId::new()),
            title: request.title,
            name: request.name,
            email: request.email,
            phone: request.phone,
            mobile: request.mobile,
            created_at: now,
            updated_at: now,
        }
    }
}
