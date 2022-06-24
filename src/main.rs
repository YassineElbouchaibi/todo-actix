mod models;
mod server;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load settings from environment variables
    let settings = models::settings::Settings::from_environment().unwrap();

    // Create server
    let server = server::create_server(&settings).await?;

    // Wait for server
    let outcome = server.await;
    println!("Terminating server...");
    return outcome;
}
