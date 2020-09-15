use serde::{
    Serialize
};
use serde_json::{
    to_string
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
pub struct ApiResponse<T: Serialize> {
    pub success: bool,
    pub payload: T
}


impl<T: Serialize> ApiResponse<T> {
    pub fn new(success: bool, payload: T) -> Self {
        Self {
            success,
            payload
        }
    }
}

impl<T: Serialize> Responder for ApiResponse<T> {
    type Error = Error;
    type Future = HttpResponse;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        HttpResponse::build(StatusCode::OK)
            .content_type("application/json")
            .body(to_string(&self).unwrap())
    }
}