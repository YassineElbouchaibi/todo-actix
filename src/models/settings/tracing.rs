// External dependencies
use config::{builder::DefaultState, ConfigBuilder, ConfigError};
use serde::Deserialize;

// Application level dependencies
use crate::utils::tracing::{ConfigureTracingParameters, TracingStructure};

#[derive(Deserialize, Debug, Clone, Copy)]
pub struct TracingSettings {
    pub structure: TracingStructure,
    pub with_file: bool,
    pub with_level: bool,
    pub with_line_number: bool,
    pub with_target: bool,
    pub with_thread_ids: bool,
    pub with_thread_names: bool,
    pub with_ansi: bool,
    // Json specific configuration
    pub json_flatten_event: bool,
    pub json_with_current_span: bool,
    pub json_with_span_list: bool,
}

impl ConfigureTracingParameters for TracingSettings {
    fn get_structure(&self) -> TracingStructure {
        self.structure
    }
    fn get_with_file(&self) -> bool {
        self.with_file
    }
    fn get_with_level(&self) -> bool {
        self.with_level
    }
    fn get_with_line_number(&self) -> bool {
        self.with_line_number
    }
    fn get_with_target(&self) -> bool {
        self.with_target
    }
    fn get_with_thread_ids(&self) -> bool {
        self.with_thread_ids
    }
    fn get_with_thread_names(&self) -> bool {
        self.with_thread_names
    }
    fn get_with_ansi(&self) -> bool {
        self.with_ansi
    }
    fn json_get_flatten_event(&self) -> bool {
        self.json_flatten_event
    }
    fn json_get_with_current_span(&self) -> bool {
        self.json_with_current_span
    }
    fn json_get_with_span_list(&self) -> bool {
        self.json_with_span_list
    }
}

pub trait TracingSettingsDefaults {
    fn set_tracing_defaults(self) -> Result<Self, ConfigError>
    where
        Self: std::marker::Sized;
}

impl TracingSettingsDefaults for ConfigBuilder<DefaultState> {
    fn set_tracing_defaults(self) -> Result<Self, ConfigError> {
        self.set_default("tracing.structure", "Full")?
            .set_default("tracing.with_file", true)?
            .set_default("tracing.with_level", true)?
            .set_default("tracing.with_line_number", true)?
            .set_default("tracing.with_target", true)?
            .set_default("tracing.with_thread_ids", true)?
            .set_default("tracing.with_thread_names", true)?
            .set_default("tracing.with_ansi", true)?
            // Json specific configuration
            .set_default("tracing.json_flatten_event", true)?
            .set_default("tracing.json_with_current_span", true)?
            .set_default("tracing.json_with_span_list", true)
    }
}
