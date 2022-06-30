mod model;

// External dependencies
use actix_web::{get, http, web, HttpResponse, Responder};
use sea_orm::EntityTrait;
use utoipa::IntoParams;

// Internal dependencies
use entity::todo::Entity as TodoEntity;

// Application level dependencies
use crate::server::_models::app_state::AppState;
use crate::server::_models::error_response::ErrorResponse;
use crate::server::_services::v1::todo::model::Todo;

// Module level dependencies
pub use model::{TodoGetParams, TodoGetResponse};

#[tracing::instrument]
#[utoipa::path(
    context_path = "/v1/todo",
    tag = "Todo",
    responses(
        (status = 200, description = "A Todo", body = TodoGetResponse),
        (status = 400, description = "Bad Request", body = ErrorResponse),
        (status = 404, description = "Not Found", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse),
    ),
)]
#[get("/{id}")]
async fn get(params: web::Path<TodoGetParams>, data: web::Data<AppState>) -> impl Responder {
    let db_connection = &data.db_connection;

    let id = params.id;

    match TodoEntity::find_by_id(id).one(db_connection).await {
        Ok(todo) => match todo {
            Some(todo) => {
                tracing::info!("Found Todo {:#?}", todo);
                return HttpResponse::Ok().json(TodoGetResponse {
                    todo: Todo::from_entity(&todo),
                });
            }
            None => {
                tracing::error!("Todo not found: {:?}", id);
                return HttpResponse::NotFound().json(ErrorResponse {
                    code: http::StatusCode::NOT_FOUND.as_u16(),
                    message: "Todo not found".to_string(),
                });
            }
        },
        Err(error) => {
            tracing::error!("Failed to get Todo: {:?}", error);
            return HttpResponse::InternalServerError().json(ErrorResponse {
                code: http::StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                message: "Failed to get Todo".to_string(),
            });
        }
    }
}
