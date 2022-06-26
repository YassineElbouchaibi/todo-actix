// External dependencies
use serde::Deserialize;
use tracing_subscriber::{
    fmt, prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt, EnvFilter,
};

#[derive(Debug, Deserialize, Clone, Copy)]
pub enum TracingStructure {
    Full,
    Compact,
    Pretty,
    Json,
}

pub trait ConfigureTracingParameters {
    fn get_structure(&self) -> TracingStructure;
    fn get_with_file(&self) -> bool;
    fn get_with_level(&self) -> bool;
    fn get_with_line_number(&self) -> bool;
    fn get_with_target(&self) -> bool;
    fn get_with_thread_ids(&self) -> bool;
    fn get_with_thread_names(&self) -> bool;
    fn get_with_ansi(&self) -> bool;
    // Json specific configuration
    fn json_get_flatten_event(&self) -> bool;
    fn json_get_with_current_span(&self) -> bool;
    fn json_get_with_span_list(&self) -> bool;
}

pub fn configure_tracing<T: ConfigureTracingParameters>(
    params: T,
) -> Result<(), Box<dyn std::error::Error>> {
    // Configure Tracing filters
    let filter_layer = EnvFilter::try_from_default_env().or_else(|_| EnvFilter::try_new("info"))?;
    let registry = tracing_subscriber::registry().with(filter_layer);

    Ok(match params.get_structure() {
        TracingStructure::Full => {
            registry
                .with(
                    fmt::layer()
                        .with_file(params.get_with_file())
                        .with_level(params.get_with_level())
                        .with_line_number(params.get_with_line_number())
                        .with_target(params.get_with_target())
                        .with_thread_ids(params.get_with_thread_ids())
                        .with_thread_names(params.get_with_thread_names())
                        .with_ansi(params.get_with_ansi()),
                )
                .init();
        }
        TracingStructure::Compact => {
            registry
                .with(
                    fmt::layer()
                        .compact()
                        .with_file(params.get_with_file())
                        .with_level(params.get_with_level())
                        .with_line_number(params.get_with_line_number())
                        .with_target(params.get_with_target())
                        .with_thread_ids(params.get_with_thread_ids())
                        .with_thread_names(params.get_with_thread_names())
                        .with_ansi(params.get_with_ansi()),
                )
                .init();
        }
        TracingStructure::Pretty => {
            registry
                .with(
                    fmt::layer()
                        .pretty()
                        .with_file(params.get_with_file())
                        .with_level(params.get_with_level())
                        .with_line_number(params.get_with_line_number())
                        .with_target(params.get_with_target())
                        .with_thread_ids(params.get_with_thread_ids())
                        .with_thread_names(params.get_with_thread_names())
                        .with_ansi(params.get_with_ansi()),
                )
                .init();
        }
        TracingStructure::Json => {
            registry
                .with(
                    fmt::layer()
                        .json()
                        .with_file(params.get_with_file())
                        .with_level(params.get_with_level())
                        .with_line_number(params.get_with_line_number())
                        .with_target(params.get_with_target())
                        .with_thread_ids(params.get_with_thread_ids())
                        .with_thread_names(params.get_with_thread_names())
                        .with_ansi(params.get_with_ansi())
                        // Json specific configuration
                        .flatten_event(params.json_get_flatten_event())
                        .with_current_span(params.json_get_with_current_span())
                        .with_span_list(params.json_get_with_span_list()),
                )
                .init();
        }
    })
}
