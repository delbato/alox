use crate::{
    api::{
        response::ApiResponse
    },
    db::{
        error::Error as DbError
    }
};

use std::{
    result::Result as StdResult,
    error::Error as StdError,
    fmt::{
        Debug,
        Display,
        Result as FmtResult,
        Formatter
    }
};

use serde::Serialize;
use serde_json::to_string;
use actix_web::{
    ResponseError,
    Responder,
    HttpResponse,
    http::{
        StatusCode
    }
};
use arangors::{
    ClientError
};

pub type ApiResult<T> = Result<ApiResponse<T>, ApiError>;

#[derive(Debug)]
pub struct ApiError {
    pub status_code: u16,
    pub error_type: ApiErrorType
}

impl ApiError {
    pub fn auto(error_type: ApiErrorType) -> Self { 
        let status_code = match error_type {
            ApiErrorType::NotAuthorized => 401,
            ApiErrorType::NotFound => 404,
            ApiErrorType::InvalidRequest => 400,
            _ => 500
        };
        Self {
            error_type,
            status_code
        }
    }

    pub fn new(status_code: u16, error_type: ApiErrorType) -> Self {
        Self {
            status_code,
            error_type
        }
    }

    pub fn new_msg<M: Into<String>>(status_code: u16, message: M) -> Self {
        Self {
            status_code,
            error_type: ApiErrorType::Message(message.into())
        }
    }
}

#[derive(Debug)]
pub enum ApiErrorType {
    Unknown,
    Custom(Box<dyn StdError>),
    NotFound,
    NotAuthorized,
    InvalidRequest,
    Message(String)
}

impl Display for ApiError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{:#?}", self)
    }
}

impl StdError for ApiError {}

impl From<Box<dyn StdError>> for ApiError {
    fn from(err: Box<dyn StdError>) -> Self {
        Self::new(500, ApiErrorType::Custom(err))
    }
}

impl From<DbError> for ApiError {
    fn from(_err: DbError) -> Self {
        Self::new(500, ApiErrorType::Unknown)
    }
}

impl From<ClientError> for ApiError {
    fn from(_err: ClientError) -> Self {
        Self::new(500, ApiErrorType::Unknown)
    }
}

impl ResponseError for ApiError {
    fn status_code(&self) -> StatusCode {
        StatusCode::from_u16(self.status_code).expect("Unknown status code!")
    }

    fn error_response(&self) -> HttpResponse {
        let api_response = ApiResponse::new(
            false,
            format!("ERROR! {:#?}", self)
        );
        let body = to_string(&api_response).unwrap();
        HttpResponse::build(self.status_code())
            .content_type("application/json")
            .body(body)
    }
}