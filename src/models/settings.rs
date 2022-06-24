// External dependencies
use config::{Config, ConfigError, Environment};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ServerSettings {
    pub host: String,
    pub port: u16,
}

#[derive(Deserialize, Debug)]
pub struct DatabaseSettings {
    pub protocol: String,
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub database: String,
}

#[derive(Deserialize, Debug)]
pub struct Settings {
    pub server: ServerSettings,
    pub database: DatabaseSettings,
}

impl Settings {
    pub fn from_environment() -> Result<Self, ConfigError> {
        println!("Loading environment variables...");
        dotenv::dotenv().ok();

        println!("Loading settings...");
        let settings = Config::builder()
            .set_default("server.host", "127.0.0.1")?
            .set_default("server.port", 8080)?
            .add_source(Environment::with_prefix("rust_app").try_parsing(true))
            .build()?
            .try_deserialize();

        println!("Settings loaded: {:#?}", settings);

        return settings;
    }
}
