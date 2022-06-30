mod constants;
mod model;

// External dependencies
use actix_web::{get, http, web, HttpResponse, Responder};
use sea_orm::{EntityTrait, PaginatorTrait, QueryOrder};
use utoipa::IntoParams;

// Internal dependencies
use entity::todo::{Column as TodoColumn, Entity as TodoEntity};

// Application level dependencies
use crate::server::_models::app_state::AppState;
use crate::server::_models::error_response::ErrorResponse;
use crate::server::_services::v1::todo::model::Todo;

// Module level dependencies
use constants::DEFAULT_TODOS_PER_PAGE;
pub use model::{TodoListParams, TodoListResponse};

#[tracing::instrument]
#[utoipa::path(
    context_path = "/v1/todo",
    tag = "Todo",
    responses(
        (status = 200, description = "List of Todos", body = TodoListResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse),
    ),
)]
#[get("/list")]
async fn list(params: web::Query<TodoListParams>, data: web::Data<AppState>) -> impl Responder {
    let db_connection = &data.db_connection;

    // Extract params and set defaults if not set
    let page = params.page.unwrap_or(1);
    let todos_per_page = params.todos_per_page.unwrap_or(DEFAULT_TODOS_PER_PAGE);

    // Create Pagination object
    let paginator = TodoEntity::find()
        .order_by_asc(TodoColumn::Id)
        .paginate(db_connection, todos_per_page);
    let num_pages = match paginator.num_pages().await {
        Ok(num_pages) => num_pages,
        Err(error) => {
            tracing::error!("Failed to get number of pages: {:?}", error);
            return HttpResponse::InternalServerError().json(ErrorResponse {
                code: http::StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                message: "Failed to get number of pages".to_string(),
            });
        }
    };

    match paginator.fetch_page(page - 1).await {
        Ok(todos) => {
            tracing::info!("Found Todos {:?}", todos);
            return HttpResponse::Ok().json(TodoListResponse {
                todos: todos.iter().map(Todo::from_entity).collect(),
                page,
                todos_per_page,
                num_pages,
            });
        }
        Err(error) => {
            tracing::error!("Failed to get Todos: {:?}", error);
            return HttpResponse::InternalServerError().json(ErrorResponse {
                code: http::StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                message: "Failed to get Todos".to_string(),
            });
        }
    }
}
