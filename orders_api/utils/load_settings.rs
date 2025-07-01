use config::{Config, ConfigError, File};
use serde::Deserialize;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct DatabaseSettings {
    pub local_uri: String,
    pub remote_uri: String,
    pub database_name: String,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
}

impl Settings {
    pub fn load() -> Result<Self, ConfigError> {
        let settings = Config::builder()
            .add_source(File::with_name("settings"))
            .build()?;

        settings.try_deserialize()
    }
}
