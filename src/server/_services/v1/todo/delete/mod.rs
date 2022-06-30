mod model;

// External dependencies
use actix_web::{delete, http, web, HttpResponse, Responder};
use sea_orm::EntityTrait;
use utoipa::IntoParams;

// Internal dependencies
use entity::todo::Entity as TodoEntity;

// Application level dependencies
use crate::server::_models::app_state::AppState;
use crate::server::_models::error_response::ErrorResponse;

// Module level dependencies
pub use model::{TodoDeleteParams, TodoDeleteResponse};

#[tracing::instrument]
#[utoipa::path(
    context_path = "/v1/todo",
    tag = "Todo",
    responses(
        (status = 200, description = "Success message", body = TodoDeleteResponse),
        (status = 400, description = "Bad Request", body = ErrorResponse),
        (status = 404, description = "Not Found", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse),
    ),
)]
#[delete("/{id}")]
async fn delete(params: web::Path<TodoDeleteParams>, data: web::Data<AppState>) -> impl Responder {
    let db_connection = &data.db_connection;

    let id = params.id;

    match TodoEntity::delete_by_id(id).exec(db_connection).await {
        Ok(delete_result) => {
            tracing::info!("Deleted Todo: {:#?}", delete_result);
            return HttpResponse::Ok().json(TodoDeleteResponse { id });
        }
        Err(error) => {
            tracing::error!("Deleted Todo failed: {:?}", error);
            return HttpResponse::InternalServerError().json(ErrorResponse {
                code: http::StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                message: "Failed to delete Todo".to_string(),
            });
        }
    }
}
