use serde::{
    Serialize
};
use serde_json::{
    to_string,
    to_value,
    Value
};
use std::{
    future::Future
};
use actix_web::{
    Responder,
    Error,
    HttpResponse,
    HttpRequest,
    http::StatusCode
};

/// General API response object
///
/// Intended for use with JSON Responses
#[derive(Debug, Serialize)]
pub struct ApiResponse {
    pub success: bool,
    pub payload: Value
}


impl ApiResponse {
    pub fn new<T: Serialize>(success: bool, payload: T) -> Self {
        Self {
            success,
            payload: to_value(&payload).unwrap()
        }
    }
}

impl Responder for ApiResponse {
    type Error = Error;
    type Future = HttpResponse;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        HttpResponse::build(StatusCode::OK)
            .content_type("application/json")
            .body(to_string(&self).unwrap())
    }
}