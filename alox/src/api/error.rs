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
use serde_json::{
    to_string,
    Value,
    json,
    to_value
};
use actix_web::{
    ResponseError,
    Responder,
    HttpResponse,
    http::{
        StatusCode
    },
    Error as ActixError
};
use arangors::{
    ClientError
};

pub type ApiResult = Result<ApiResponse, ApiError>;

pub trait ApiResultExt<T: Serialize> {
    fn error(status_code: u16, payload: T) -> Self;
    fn success(payload: T) -> Self;
}

impl<T: Serialize> ApiResultExt<T> for ApiResult {
    fn error(status_code: u16, payload: T) -> Self {
        Err(
            ApiError::new_payload(status_code, payload)
        )
    }

    fn success(payload: T) -> Self {
        Ok(
            ApiResponse::new(true, payload)
        )
    }
}

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

    pub fn new_payload<T: Serialize>(status_code: u16, payload: T) -> Self {
        Self {
            status_code,
            error_type: ApiErrorType::Payload(to_value(&payload).unwrap())
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
    Message(String),
    Payload(Value)
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
        match &self.error_type {
            ApiErrorType::Payload(json_value) => {
                HttpResponse::build(self.status_code())
                    .content_type("application/json")
                    .body(json!({
                        "success": false,
                        "payload": json_value
                    }))
            },
            _ => {
                let body = to_string(&api_response).unwrap();
                HttpResponse::build(self.status_code())
                    .content_type("application/json")
                    .body(body)
            }
        }
    }
}