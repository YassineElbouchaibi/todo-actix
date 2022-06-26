// External dependencies
use config::{builder::DefaultState, ConfigBuilder, ConfigError};
use serde::Deserialize;

// Application level dependencies
use crate::utils::consul::ConfigureConsulParameters;

#[derive(Deserialize, Debug)]
pub struct ConsulSettings {
    pub name: String,
    pub host: String,
    pub port: u16,
    health_check_host: String,
    health_check_port: u16,
    health_check_path: String,
    health_check_interval: String,
}

impl ConfigureConsulParameters for ConsulSettings {
    fn get_name(&self) -> String {
        self.name.clone()
    }
    fn get_host(&self) -> String {
        self.host.clone()
    }
    fn get_port(&self) -> u16 {
        self.port
    }
    fn get_health_check_host(&self) -> String {
        self.health_check_host.clone()
    }
    fn get_health_check_port(&self) -> u16 {
        self.health_check_port
    }
    fn get_health_check_path(&self) -> String {
        self.health_check_path.clone()
    }
    fn get_health_check_interval(&self) -> String {
        self.health_check_interval.clone()
    }
}

pub trait ConsulSettingsDefaults {
    fn set_consul_defaults(self) -> Result<Self, ConfigError>
    where
        Self: std::marker::Sized;
}

impl ConsulSettingsDefaults for ConfigBuilder<DefaultState> {
    fn set_consul_defaults(self) -> Result<Self, ConfigError> {
        self.set_default("consul.name", "unknown-rust-app")?
            .set_default("consul.host", "127.0.0.1")?
            .set_default("consul.port", 8080)?
            .set_default("consul.health_check_host", "127.0.0.1")?
            .set_default("consul.health_check_port", 8080)?
            .set_default("consul.health_check_path", "/healthcheck")?
            .set_default("consul.health_check_interval", "10s")
    }
}
