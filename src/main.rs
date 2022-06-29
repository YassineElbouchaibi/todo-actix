mod models;
mod server;
mod utils;

// External dependencies
use tracing::info;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load settings from environment variables
    let settings = models::settings::Settings::from_environment().unwrap();

    // Configure tracing
    // TODO: Load only tracing settings before configuring tracing and load the rest after
    let _guard = utils::tracing::configure_tracing(&settings.tracing).unwrap();

    let consul_registrator_handle = utils::consul::register_with_consul(
        &settings.consul,
        settings.server.host.clone(),
        settings.server.port,
    )
    .await;

    // Log settings
    info!("{:?}", settings);

    // Create server
    let server = server::create_server(&settings).await?;

    // Wait for server
    server.await?;
    info!("Server terminated.");

    consul_registrator_handle.await?;
    info!("Consul registrator terminated.");

    Ok(())
}
