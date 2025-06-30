use crate::database::mongo_db::MongoDb;
use crate::models::customer::Customer;
use anyhow::Error;
use mongodb::bson::{Document, doc, oid::ObjectId};
use mongodb::{Collection, Database};

pub struct CustomerService {
    collection: Collection<Customer>,
}

impl CustomerService {
    pub async fn new() -> Self {
        let mongo_db = MongoDb::init().await.expect("Mongodb Connect Failed");
        let database: Database = mongo_db.database;
        let collection: Collection<Customer> = database.collection("customers");

        CustomerService { collection }
    }

    pub async fn create_customer(&self, new_customer: Customer) -> Result<Customer, Error> {
        let result = self.collection.insert_one(new_customer).await?;
        if let Some(inserted_id) = result.inserted_id.as_object_id() {
            let filter = doc! { "_id": inserted_id };
            if let Some(response) = self.collection.find_one(filter).await? {
                return Ok(response);
            }
        }
        Err(Error::msg("Failed to create customer"))
    }

    pub async fn get_customer(&self, id: &str) -> Result<Customer, Error> {
        let object_id = ObjectId::parse_str(id)
            .map_err(|e| anyhow::anyhow!("Invalid ObjectId format: {}", e))?;
        let filter = doc! { "_id": object_id };
        if let Some(response) = self.collection.find_one(filter).await? {
            return Ok(response);
        }
        Err(Error::msg("Customer not found"))
    }

    pub async fn update_customer(
        &self,
        id: &str,
        update_doc: Document,
    ) -> Result<Customer, String> {
        let filter = doc! { "_id": ObjectId::parse_str(id).map_err(|e| e.to_string())? };
        let update = doc! { "$set": update_doc };

        let update_result = self
            .collection
            .update_one(filter.clone(), update)
            .await
            .map_err(|e| e.to_string())?;
        if update_result.modified_count == 1 {
            match self.collection.find_one(filter).await {
                Ok(Some(customer)) => Ok(customer),
                Ok(None) => Err("Customer not found after update".to_string()),
                Err(e) => Err(e.to_string()),
            }
        } else {
            Err("No customer updated".to_string())
        }
    }
}
