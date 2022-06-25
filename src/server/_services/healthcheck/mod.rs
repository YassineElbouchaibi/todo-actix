mod model;

// External dependencies
use actix_web::{get, web, HttpResponse, Responder};
use sea_orm::{ConnectionTrait, Statement};

// Application level dependencies
use crate::server::_models::app_state::AppState;

// Module level dependencies
pub use model::{HealthCheckResponse, HealthCheckStatus};

#[utoipa::path(
    tag = "Maintenance",
    responses(
        (status = 200, description = "Service is available", body = HealthCheckResponse),
        (status = 500, description = "Service is unavailable", body = HealthCheckResponse),
    )
)]
#[get("/healthcheck")]
async fn healthcheck(data: web::Data<AppState>) -> impl Responder {
    let db_connection = &data.db_connection;

    let outcome = db_connection
        .execute(Statement::from_string(
            db_connection.get_database_backend(),
            "SELECT 1".to_owned(),
        ))
        .await;

    if outcome.is_err() {
        return HttpResponse::InternalServerError().json(HealthCheckResponse {
            status: HealthCheckStatus::Unavailable,
        });
    }

    return HttpResponse::Ok().json(HealthCheckResponse {
        status: HealthCheckStatus::Available,
    });
}
