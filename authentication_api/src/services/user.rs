use crate::database::mongo_db::MongoDb;
use crate::models;
use anyhow::Result;
use models::user::User;
use mongodb::bson::{Document, doc, oid::ObjectId};
use mongodb::{Collection, Database};

pub struct UserService {
    collection: Collection<User>,
}

/// Initializes the UserService
/// returns a UserService instance, creates a new mongodb collection instance
impl UserService {
    pub async fn new() -> Self {
        let mongo_db = MongoDb::init().await.expect("Mongodb Connect Failed");

        let database: Database = mongo_db.database;
        let collection: Collection<User> = database.collection::<User>("users");
        UserService { collection }
    }

    /// Creates a new user in the database
    pub async fn create_user(&self, user: User) -> Result<Option<User>> {
        match self.find_by_email(&user.email).await {
            Ok(Some(_)) => Err(anyhow::anyhow!(
                "User with email {} already exists",
                user.email
            )),
            Ok(None) => {
                let insert_result = self.collection.insert_one(&user).await?;
                if let Some(inserted_id) = insert_result.inserted_id.as_object_id() {
                    let filter = doc! { "_id": inserted_id };
                    let created_user = self.collection.find_one(filter).await?;
                    Ok(created_user)
                } else {
                    Err(anyhow::anyhow!(
                        "Failed to retrieve inserted_id as ObjectId"
                    ))
                }
            }
            Err(e) => Err(e),
        }
    }

    pub async fn find_by_email(&self, email: &str) -> Result<Option<User>> {
        let filter = doc! {"email": email};
        let user = self.collection.find_one(filter).await?;
        Ok(user)
    }

    pub async fn find_by_id(&self, id: &str) -> Result<Option<User>> {
        let object_id = ObjectId::parse_str(id)
            .map_err(|e| anyhow::anyhow!("Invalid ObjectId format: {}", e))?;
        let filter = doc! {"_id": object_id};
        let user = self.collection.find_one(filter).await?;

        Ok(user)
    }

    pub async fn update_user(
        &self,
        id: &str,
        update_doc: Document,
    ) -> Result<Option<User>, String> {
        let filter = doc! { "_id": ObjectId::parse_str(id).map_err(|e| e.to_string())? };
        let update = doc! { "$set": update_doc };

        let update_result = self
            .collection
            .update_one(filter.clone(), update)
            .await
            .map_err(|e| e.to_string())?;
        match update_result.modified_count {
            1 => self
                .collection
                .find_one(filter)
                .await
                .map_err(|e| e.to_string()),
            _ => Ok(None),
        }
    }
}
