// External dependencies
use actix_web::{http, HttpResponse, Responder};

// Application level dependencies
use crate::server::_models::error_response::ErrorResponse;

#[tracing::instrument]
pub async fn default_service() -> impl Responder {
    HttpResponse::NotFound().json(ErrorResponse {
        code: http::StatusCode::NOT_FOUND.as_u16(),
        message: "Not Found".to_string(),
    })
}
