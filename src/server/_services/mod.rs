mod default_service;
pub mod healthcheck;
pub mod v1;

pub fn init_services(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.default_service(actix_web::web::to(default_service::default_service));
    cfg.service(healthcheck::healthcheck);
    cfg.service(v1::init_v1());
}
