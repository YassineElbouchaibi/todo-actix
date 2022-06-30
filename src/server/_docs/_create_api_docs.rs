// External dependencies
use utoipa::{openapi, Modify};

// Application level dependencies
use crate::server::_models::error_response::ErrorResponse;
use crate::server::_services::healthcheck::{HealthCheckResponse, HealthCheckStatus};
use crate::server::_services::v1::todo::{
    // Create
    create::TodoCreatePayload,
    create::TodoCreateResponse,
    // Delete
    delete::TodoDeleteResponse,
    // Get
    get::TodoGetResponse,
    // List
    list::TodoListResponse,
    // Model
    model::Todo,
    // Update
    update::TodoUpdatePayload,
    update::TodoUpdateResponse,
};

pub fn create_api_docs() -> utoipa::openapi::OpenApi {
    #[derive(utoipa::OpenApi)]
    #[openapi(
        handlers(
            crate::server::_services::healthcheck::healthcheck,
            crate::server::_services::v1::todo::list::list,
            crate::server::_services::v1::todo::create::create,
            crate::server::_services::v1::todo::get::get,
            crate::server::_services::v1::todo::update::update,
            crate::server::_services::v1::todo::delete::delete,
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
            //// Update
            TodoUpdatePayload,
            TodoUpdateResponse,
            //// Delete
            TodoDeleteResponse,

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
