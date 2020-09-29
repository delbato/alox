use crate::{
    api::{
        error::{
            ApiError,
            ApiResult,
            ApiResultExt
        }
    },
    util::{
        jwt::{
            JwtClaims,
            JwtManager
        }
    }
};

use std::{
    pin::Pin,
    collections::HashSet,
    task::{Context, Poll},
    iter::FromIterator
};

use actix_service::{Service, Transform};
use actix_web::{
    Error, HttpResponse,
    dev::{ServiceRequest, ServiceResponse},
    http::{Method, HeaderName, HeaderValue, StatusCode},
    web::Data,
};
use futures::{
    future::{ok, Ready},
    Future,
};
use serde_json::{
    to_string,
    json
};
use log::error;

pub struct Jwt {
    excluded_paths: HashSet<String>
}

impl Jwt {
    pub fn with_exclude(excluded_paths: &[&str]) -> Self {
        let excluded_paths = HashSet::from_iter(excluded_paths.iter().map(|s| String::from(*s)));
        Self {
            excluded_paths
        }
    }
}

impl Default for Jwt {
    fn default() -> Self {
        Self {
            excluded_paths: HashSet::new()
        }
    }
}

impl<S, B> Transform<S> for Jwt
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = JwtMiddleware<S>;
    type Future = impl Future<Output = Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        let excluded_paths = self.excluded_paths.clone();
        async move {
            Ok(
                JwtMiddleware { service, excluded_paths }
            )
        }
    }
}

pub struct JwtMiddleware<S> {
    service: S,
    excluded_paths: HashSet<String>
}

impl<B: 'static, S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>> JwtMiddleware<S> {
    fn run(&self, req: &mut ServiceRequest) -> ApiResult {
        if self.excluded_paths.contains(req.path()) {
            return ApiResult::success(());
        }
        let header_auth_opt = req.headers().get("Authorization");
        if header_auth_opt.is_none() {
            return ApiResult::error(401, "No Authorization header");
        }
        let header_auth = header_auth_opt.unwrap();
        let auth_raw = header_auth.to_str().unwrap();
        if !auth_raw.starts_with("Bearer ") {
            return ApiResult::error(401, "Malformed Authorization header");
        }
        let auth_split: Vec<&str> = auth_raw.split(" ").collect();
        if auth_split.len() != 2 {
            return ApiResult::error(401, "Malformed Authorization header");
        }
        let jwt_token = auth_split[1];
        let jwt_manager = req.app_data::<Data<JwtManager>>().cloned().unwrap();
        let _jwt_claims = jwt_manager.validate_token(jwt_token)
            .ok_or(ApiResult::error(401, "Invalid JWT").unwrap_err())?;
        ApiResult::success(())
    }
}

impl<S, B> Service for JwtMiddleware<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, mut req: ServiceRequest) -> Self::Future {
        let jwt_result = self.run(&mut req);
        if jwt_result.is_ok() {
            let fut = self.service.call(req);
            Box::pin(async move {
                let res = fut.await?;
                Ok(res)
            })
        } else {
            Box::pin(async move {
                Err(jwt_result.unwrap_err().into())
            })
        }
    }
}