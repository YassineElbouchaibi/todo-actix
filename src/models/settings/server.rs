// External dependencies
use config::{builder::DefaultState, ConfigBuilder, ConfigError};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ServerSettings {
    pub host: String,
    pub port: u16,
}

pub trait ServerSettingsDefaults {
    fn set_server_defaults(self) -> Result<Self, ConfigError>
    where
        Self: std::marker::Sized;
}

impl ServerSettingsDefaults for ConfigBuilder<DefaultState> {
    fn set_server_defaults(self) -> Result<Self, ConfigError> {
        self.set_default("server.host", "127.0.0.1")?
            .set_default("server.port", 8080)
    }
}
