
use actix_web::{ResponseError, dev::Body, web::Bytes};
use sqlx::Error as SqlxError;

// https://docs.rs/anyhow
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Sqlx error: {0}")]
    Sqlx(#[from] SqlxError),
}

impl ResponseError for Error {
    /// Convert an Error to a HttpResponse
    fn error_response(&self) -> actix_web::HttpResponse {
        let resp = actix_web::HttpResponse::new(self.status_code());
        let body = format!("{}", &self);
        let body = body.as_bytes();
        let body = Body::Bytes(Bytes::copy_from_slice(body));
        let resp = resp.set_body(body);
        resp
    }
}
