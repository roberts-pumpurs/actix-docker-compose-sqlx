
use actix_web::web;

use super::handlers::users::get_users;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg
        .route("/user", web::get().to(get_users));
}
