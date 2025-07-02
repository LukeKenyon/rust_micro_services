use crate::utils;
use mongodb::{Client, Database, error::Result, options::ClientOptions};
use utils::load_settings::Settings;

pub struct MongoDb {
    pub database: Database,
}

/// Initializes the MongoDB client and database connection
/// returns a MongoDb instance
impl MongoDb {
    pub async fn init() -> Result<Self> {
        let settings = Settings::load().unwrap();
        let client_options = ClientOptions::parse(&settings.database.remote_uri)
            .await
            .unwrap();
        let client = Client::with_options(client_options).unwrap();

        let database = client.database(&settings.database.database_name);
        Ok(MongoDb { database })
    }
}
