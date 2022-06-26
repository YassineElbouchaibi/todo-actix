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
    // Sentry
    fn get_with_sentry(&self) -> bool;
    fn get_sentry_environment(&self) -> String;
    fn get_sentry_traces_sample_rate(&self) -> f32;
}

pub fn configure_tracing<T: ConfigureTracingParameters>(
    params: &T,
) -> Result<Option<sentry::ClientInitGuard>, Box<dyn std::error::Error>> {
    // Configure sentry
    let (_guard, sentry_layer) = if params.get_with_sentry() {
        std::env::set_var("RUST_BACKTRACE", "1");
        (
            Some(sentry::init(sentry::ClientOptions {
                release: sentry::release_name!(),
                // DSN is set through SENTRY_DSN environment variable
                environment: Some(params.get_sentry_environment().into()),
                traces_sample_rate: params.get_sentry_traces_sample_rate(),
                // Session mode
                session_mode: sentry::SessionMode::Request,
                auto_session_tracking: true,
                ..Default::default()
            })),
            Some(sentry_tracing::layer()),
        )
    } else {
        (None, None)
    };

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
        .with(sentry_layer)
        .with(telemetry_layer);

    match params.get_structure() {
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
    }

    Ok(_guard)
}
