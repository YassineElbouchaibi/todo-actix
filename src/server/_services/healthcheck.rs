use actix_web::{get, http, web, Responder, Result};
use sea_orm::{ConnectionTrait, Statement};
use serde::Serialize;

use crate::server::_models::app_state::AppState;

#[derive(Serialize)]
enum HealthCheckStatus {
    Available = 1,
    Unavailable = 2,
}

#[derive(Serialize)]
struct HealthCheckResponse {
    status: HealthCheckStatus,
}

#[get("/healthcheck")]
async fn healthcheck(data: web::Data<AppState>) -> Result<impl Responder> {
    let db_connection = &data.db_connection;

    let outcome = db_connection
        .execute(Statement::from_string(
            db_connection.get_database_backend(),
            "SELECT 1".to_owned(),
        ))
        .await;

    if outcome.is_err() {
        return Ok((
            web::Json(HealthCheckResponse {
                status: HealthCheckStatus::Unavailable,
            }),
            http::StatusCode::INTERNAL_SERVER_ERROR,
        ));
    }

    return Ok((
        web::Json(HealthCheckResponse {
            status: HealthCheckStatus::Available,
        }),
        http::StatusCode::OK,
    ));
}
