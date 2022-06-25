// External dependencies
use utoipa_swagger_ui::SwaggerUi;

// Module level dependencies
use super::_create_api_docs::create_api_docs;

pub fn create_swagger_ui() -> SwaggerUi {
    SwaggerUi::new("/docs/{_:.*}").url("/docs/openapi.json", create_api_docs())
}
