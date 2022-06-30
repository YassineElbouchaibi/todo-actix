mod model;

// External dependencies
use actix_web::{http, patch, web, HttpResponse, Responder};
use sea_orm::{ActiveModelTrait, Set};
use utoipa::IntoParams;

// Internal dependencies
use entity::todo::ActiveModel as TodoActiveModel;

// Application level dependencies
use crate::server::_models::app_state::AppState;
use crate::server::_models::error_response::ErrorResponse;
use crate::server::_services::v1::todo::model::Todo;

// Module level dependencies
pub use model::{TodoUpdateParams, TodoUpdatePayload, TodoUpdateResponse};

#[tracing::instrument]
#[utoipa::path(
    context_path = "/v1/todo",
    tag = "Todo",
    request_body = TodoUpdatePayload,
    responses(
        (status = 200, description = "The Updated Todo", body = TodoUpdateResponse),
        (status = 400, description = "Bad Request", body = ErrorResponse),
        (status = 404, description = "Not Found", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse),
    ),
)]
#[patch("/{id}")]
async fn update(
    params: web::Path<TodoUpdateParams>,
    payload: web::Json<TodoUpdatePayload>,
    data: web::Data<AppState>,
) -> impl Responder {
    let db_connection = &data.db_connection;

    let id = params.id;

    let now = chrono::Utc::now();

    let result = TodoActiveModel {
        id: Set(id),
        title: Set(payload.title.to_owned()),
        completed_at: if payload.completed {
            Set(Some(now))
        } else {
            Set(None)
        },
        updated_at: Set(now),
        ..Default::default()
    }
    .update(db_connection)
    .await;

    match result {
        Ok(todo) => {
            tracing::info!("Updated Todo {:#?}", todo);
            return HttpResponse::Ok().json(TodoUpdateResponse {
                todo: Todo::from_entity(&todo),
            });
        }
        Err(error) => {
            tracing::error!("Failed to update Todo: {:?}", error);
            return HttpResponse::InternalServerError().json(ErrorResponse {
                code: http::StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                message: "Failed to update Todo".to_string(),
            });
        }
    }
}
