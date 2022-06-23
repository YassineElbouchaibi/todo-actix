mod settings;
mod models;

use crate::models::{StatusResponse, Status};
use actix_web::{HttpServer, App, web, Responder};
use dotenv::dotenv;

use std::io::{Result};

use settings::Settings;

async fn status() -> impl Responder {
    web::Json(StatusResponse { status: Status::Running })
}

#[actix_web::main]
async fn main() -> Result<()> {
    println!("Loading environment variables...");
    dotenv().ok();

    println!("Loading settings...");
    let settings = Settings::from_environment().unwrap();
    println!("Settings loaded: {:#?}", settings);

    println!("Starting server...");
    let server = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(status))
    })
    .bind((settings.server.host.as_str(), settings.server.port))?
    .run();
    println!("Server started on http://{host}:{port}/", host=settings.server.host, port=settings.server.port);

    let outcome = server.await;
    println!("Terminating server...");
    return outcome
}
