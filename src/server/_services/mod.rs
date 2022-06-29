mod default_service;
pub mod healthcheck;
pub mod todo;

pub fn init_services(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.default_service(actix_web::web::to(default_service::default_service));
    cfg.service(healthcheck::healthcheck);
    cfg.service(init_v1());
}

fn init_v1() -> actix_web::Scope {
    actix_web::web::scope("/v1/todo")
        .service(todo::list::list)
        .service(todo::create::create)
        .service(todo::get::get)
}
