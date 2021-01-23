use serde::Deserialize;
use serde::Serialize;
use std::borrow::Cow;

use actix_web::{http::StatusCode, Error, HttpResponse};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ApiResult<T = ()> {
    pub code: u16,
    pub msg: Option<Cow<'static, str>>,
    pub data: Option<T>,
}

impl<T: Serialize> ApiResult<T> {
    pub fn new() -> Self {
        Self {
            code: 200,
            msg: None,
            data: None,
        }
    }
    pub fn code(mut self, code: u16) -> Self {
        self.code = code;
        self
    }
    pub fn with_msg<S: Into<Cow<'static, str>>>(mut self, msg: S) -> Self {
        self.msg = Some(msg.into());
        self
    }
    pub fn msg_as_str(&self) -> &str {
        self.msg.as_ref().map(|s| s.as_ref()).unwrap_or_default()
    }
    pub fn with_data(mut self, data: T) -> Self {
        self.data = Some(data);
        self
    }
    pub fn to_resp(&self) -> HttpResponse {
        let resp = match serde_json::to_string(self) {
            Ok(json) => HttpResponse::Ok()
                .content_type("application/json")
                .status(
                    StatusCode::from_u16(self.code).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR),
                )
                .body(json),
            Err(e) => Error::from(e).into(),
        };

        resp
    }
}
