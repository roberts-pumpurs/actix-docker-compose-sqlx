use actix_web::{HttpResponse, Responder};



pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("I am healthy!")
}
