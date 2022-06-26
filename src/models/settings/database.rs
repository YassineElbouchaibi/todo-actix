// External dependencies
use config::{builder::DefaultState, ConfigBuilder, ConfigError};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct DatabaseSettings {
    pub protocol: String,
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub database: String,
}

pub trait DatabaseSettingsDefaults {
    fn set_database_defaults(self) -> Result<Self, ConfigError>
    where
        Self: std::marker::Sized;
}

impl DatabaseSettingsDefaults for ConfigBuilder<DefaultState> {
    fn set_database_defaults(self) -> Result<Self, ConfigError> {
        self.set_default("database.protocol", "postgres")?
            .set_default("database.host", "localhost")?
            .set_default("database.port", 5432)?
            .set_default("database.user", "postgres")?
            .set_default("database.password", "postgres")?
            .set_default("database.database", "postgres")
    }
}
