// External dependencies
use opentelemetry::{
    global, runtime::TokioCurrentThread, sdk::propagation::TraceContextPropagator,
};
use serde::Deserialize;
use tracing_subscriber::{
    fmt::{self, format::FmtSpan},
    prelude::__tracing_subscriber_SubscriberExt,
    util::SubscriberInitExt,
    EnvFilter,
};

#[derive(Debug, Deserialize, Clone, Copy)]
pub enum TracingStructure {
    Full,
    Compact,
    Pretty,
    Json,
    Bunyan,
}

pub trait ConfigureTracingParameters {
    fn get_app_name(&self) -> String;
    fn get_with_telemetry(&self) -> bool;
    fn get_structure(&self) -> TracingStructure;
    fn get_with_file(&self) -> bool;
    fn get_with_level(&self) -> bool;
    fn get_with_line_number(&self) -> bool;
    fn get_with_target(&self) -> bool;
    fn get_with_thread_ids(&self) -> bool;
    fn get_with_thread_names(&self) -> bool;
    fn get_with_ansi(&self) -> bool;
    // Json specific configuration
    fn get_json_flatten_event(&self) -> bool;
    fn get_json_with_current_span(&self) -> bool;
    fn get_json_with_span_list(&self) -> bool;
}

pub fn configure_tracing<T: ConfigureTracingParameters>(
    params: &T,
) -> Result<(), Box<dyn std::error::Error>> {
    // Create a filter layer
    let filter_layer = EnvFilter::try_from_default_env().or_else(|_| EnvFilter::try_new("info"))?;

    // Create a telemetry layer if we are using telemetry
    let telemetry_layer = if params.get_with_telemetry() {
        global::set_text_map_propagator(TraceContextPropagator::new());
        let tracer = opentelemetry_jaeger::new_pipeline()
            .with_service_name(params.get_app_name())
            .install_batch(TokioCurrentThread)
            .expect("Failed to install OpenTelemetry tracer.");

        // Create a `tracing` layer using the Jaeger tracer
        Some(tracing_opentelemetry::layer().with_tracer(tracer))
    } else {
        None
    };

    // Create layer registry
    let registry = tracing_subscriber::registry()
        .with(filter_layer)
        .with(telemetry_layer);

    Ok(match params.get_structure() {
        TracingStructure::Full => {
            registry
                .with(
                    fmt::layer()
                        .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE)
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
                        .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE)
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
                        .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE)
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
                        .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE)
                        .with_file(params.get_with_file())
                        .with_level(params.get_with_level())
                        .with_line_number(params.get_with_line_number())
                        .with_target(params.get_with_target())
                        .with_thread_ids(params.get_with_thread_ids())
                        .with_thread_names(params.get_with_thread_names())
                        .with_ansi(params.get_with_ansi())
                        // Json specific configuration
                        .flatten_event(params.get_json_flatten_event())
                        .with_current_span(params.get_json_with_current_span())
                        .with_span_list(params.get_json_with_span_list()),
                )
                .init();
        }
        TracingStructure::Bunyan => {
            registry
                .with(tracing_bunyan_formatter::JsonStorageLayer)
                .with(tracing_bunyan_formatter::BunyanFormattingLayer::new(
                    params.get_app_name(),
                    std::io::stdout,
                ))
                .init();
        }
    })
}
