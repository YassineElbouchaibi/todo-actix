use actix_web::dev::Server;
use actix_web::middleware;

use crate::models::settings::Settings;

use super::_models::app_state::AppState;
use super::_services::init;

pub async fn create_server<'a>(settings: &Settings) -> std::io::Result<Server> {
    // Create application state
    let state = AppState::from_settings(&settings).await;

    println!("Starting server...");

    let server = actix_web::HttpServer::new(move || {
        actix_web::App::new()
            .app_data(actix_web::web::Data::new(state.clone()))
            .wrap(middleware::Logger::default())
            .configure(init)
    })
    .bind((settings.server.host.as_str(), settings.server.port))?
    .run();

    println!(
        "Server started on http://{host}:{port}/",
        host = &settings.server.host,
        port = settings.server.port
    );

    Ok(server)
}
