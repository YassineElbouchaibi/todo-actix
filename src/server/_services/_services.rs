use super::healthcheck::healthcheck;

pub fn init(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(healthcheck);
}
