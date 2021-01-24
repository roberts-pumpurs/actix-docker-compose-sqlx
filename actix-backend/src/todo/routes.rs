use actix_web::web;

use super::handlers::users::init;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/user").configure(init));
}
