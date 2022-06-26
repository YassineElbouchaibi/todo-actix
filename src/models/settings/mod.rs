mod consul;
mod database;
mod server;
mod tracing;

// External dependencies
use ::tracing::info;
use config::{Config, ConfigError, Environment};
use serde::Deserialize;

// Module level dependencies
use self::tracing::{TracingSettings, TracingSettingsDefaults};
use consul::{ConsulSettings, ConsulSettingsDefaults};
use database::{DatabaseSettings, DatabaseSettingsDefaults};
use server::{ServerSettings, ServerSettingsDefaults};

#[derive(Deserialize, Debug)]
pub struct Settings {
    pub server: ServerSettings,
    pub database: DatabaseSettings,
    pub tracing: TracingSettings,
    pub consul: ConsulSettings,
}

impl Settings {
    pub fn from_environment() -> Result<Self, ConfigError> {
        info!("Loading environment variables...");
        dotenv::dotenv().ok();

        info!("Loading settings...");
        let settings = Config::builder()
            // Set default values for consul
            .set_consul_defaults()?
            // Set default values for database
            .set_database_defaults()?
            // Set default values for server
            .set_server_defaults()?
            // Set default values for tracing
            .set_tracing_defaults()?
            // Only parse the environment variables prefixed with "RUST_APP_" (Case insensitive)
            .add_source(Environment::with_prefix("rust_app").try_parsing(true))
            .build()?
            .try_deserialize();

        info!("Settings loaded: {:?}", settings);

        return settings;
    }
}
