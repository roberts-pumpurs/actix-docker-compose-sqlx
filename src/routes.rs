
use crate::handlers::{
    health::health_check, users, subtask, todo
};
use actix_web::web;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg
        // Healthcheck
        .route("/health", web::get().to(health_check))
        // /api/v1 routes
        .service(
            web::scope("/api/v1")
                .service( web::scope("/user").configure(users::routes))
                .service( web::scope("/subtask").configure(subtask::routes))
                .service( web::scope("/todo").configure(todo::routes))
        );
}
