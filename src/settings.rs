use serde::Deserialize;
use config::{Config, ConfigError, Environment};

#[derive(Deserialize, Debug)]
pub struct ServerSettings {
    pub host: String,
    pub port: u16,
}

#[derive(Deserialize, Debug)]
pub struct Settings {
    pub server: ServerSettings,
}

impl Settings {
    pub fn from_environment() -> Result<Self, ConfigError> {
        Config::builder()
            .set_default("server.host", "127.0.0.1")?
            .set_default("server.port", 8080)?
            .add_source(
                Environment::with_prefix("rust_app")
                    .try_parsing(true)
            )
            .build()?
            .try_deserialize()
    }
}