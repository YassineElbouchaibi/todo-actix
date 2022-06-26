mod model;

// External dependencies
use actix_web::{http, post, web, HttpResponse, Responder};
use sea_orm::{ActiveModelTrait, Set};
use tracing::info;

// Internal dependencies
use entity::todo::ActiveModel as TodoActiveModel;

// Application level dependencies
use crate::server::_models::app_state::AppState;
use crate::server::_models::error_response::ErrorResponse;
use crate::server::_services::todo::model::Todo;

// Module level dependencies
pub use model::{TodoCreatePayload, TodoCreateResponse};

#[tracing::instrument]
#[utoipa::path(
    context_path = "/v1/todo",
    tag = "Todo",
    request_body = TodoCreatePayload,
    responses(
        (status = 201, description = "Todo created", body = TodoCreateResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse),
    ),
)]
#[post("/create")] // Using post instead of put because each request is a new entity
async fn create(
    payload: web::Json<TodoCreatePayload>,
    data: web::Data<AppState>,
) -> impl Responder {
    let db_connection = &data.db_connection;

    // Create Todo
    let result = TodoActiveModel {
        title: Set(payload.title.to_owned()),
        ..Default::default()
    }
    .insert(db_connection)
    .await;

    info!("Created Todo {:#?}", result);

    if result.is_err() {
        return HttpResponse::InternalServerError().json(ErrorResponse {
            code: http::StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            message: "Failed to create Todo".to_string(),
        });
    } else {
        let todo = result.unwrap();
        return HttpResponse::Created().json(TodoCreateResponse {
            todo: Todo::from_entity(&todo),
        });
    }
}
