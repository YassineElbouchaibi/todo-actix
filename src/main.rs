mod models;
mod server;
mod utils;

// External dependencies
use log::info;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load settings from environment variables
    let settings = models::settings::Settings::from_environment().unwrap();

    // Configure tracing
    // TODO: Load only tracing settings before configuring tracing and load the rest after
    utils::tracing::configure_tracing(settings.tracing).unwrap();

    // Log settings
    info!("{:?}", settings);

    // Create server
    let server = server::create_server(&settings).await?;

    // Wait for server
    let outcome = server.await;
    info!("Terminating server...");
    return outcome;
}
