// External dependencies
use utoipa::{openapi, Modify};

// Application level dependencies
use crate::server::_models::error_response::ErrorResponse;
use crate::server::_services::healthcheck::{HealthCheckResponse, HealthCheckStatus};
use crate::server::_services::todo::{
    // Model
    model::Todo,
    // List
    list::TodoListResponse,
    // Create
    create::TodoCreatePayload,
    create::TodoCreateResponse,
    // Get
    get::TodoGetResponse,
};

pub fn create_api_docs() -> utoipa::openapi::OpenApi {
    #[derive(utoipa::OpenApi)]
    #[openapi(
        handlers(
            crate::server::_services::healthcheck::healthcheck,
            crate::server::_services::todo::list::list,
            crate::server::_services::todo::create::create,
            crate::server::_services::todo::get::get,
        ),
        components(
            // General
            ErrorResponse,

            // Todo
            Todo,
            //// List
            TodoListResponse,
            //// Create
            TodoCreatePayload,
            TodoCreateResponse,
            //// Get
            TodoGetResponse,
            
            // Maintenance
            HealthCheckStatus,
            HealthCheckResponse,
        ),
        tags(
            (name = "Todo", description = "Todo management endpoints."),
            (name = "Maintenance", description = "Maintenance endpoints."),
        ),
        modifiers(
            &ApiDocInfo
        ),
    )]
    struct ApiDoc;

    struct ApiDocInfo;
    impl Modify for ApiDocInfo {
        fn modify(&self, openapi: &mut openapi::OpenApi) {
            openapi.info.description = Some(include_str!("./_api_description.md").to_string());
            openapi.info.license = Some(
                openapi::LicenseBuilder::new()
                    .name("MIT")
                    .url(Some("https://opensource.org/licenses/MIT"))
                    .build(),
            );
        }
    }

    <ApiDoc as utoipa::OpenApi>::openapi()
}
