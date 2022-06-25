mod _create_api_docs;
mod _create_swagger_ui;

pub fn init_docs(cfg: &mut actix_web::web::ServiceConfig) {
    let swagger_ui = _create_swagger_ui::create_swagger_ui();

    cfg.service(swagger_ui);
}
