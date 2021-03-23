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
    collections::{
        HashMap,
        HashSet
    },
    task::{Context, Poll},
    iter::FromIterator,
    borrow::Borrow,
    cmp::Eq,
    hash::Hash
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
    excluded_paths: HashMap<String, HashSet<String>>,
    require_admin: bool
}

impl Jwt {
    pub fn with_exclude<S>(mut self, paths: HashMap<S, Vec<S>>) -> Self
    where S: Into<String> {
        for (path, path_methods) in paths.into_iter() {
            let path = path.into();
            let path_methods: HashSet<String> = path_methods.into_iter().map(|item| item.into()).collect();

            if let Some(methods) = self.excluded_paths.get_mut(&path) {
                methods.extend(path_methods.into_iter().map(|item| item.into()));
            } else {
                let path_methods_iter = path_methods.into_iter().map(|item| item.into());
                let mut path_methods = HashSet::new();
                for methods in path_methods_iter {
                    path_methods.insert(methods);
                }
                self.excluded_paths.insert(path.into(), path_methods);
            }
        }
        self
    }

    pub fn with_require_admin(mut self, require_admin: bool) -> Self {
        self.require_admin = require_admin;
        self
    }
}

impl Default for Jwt {
    fn default() -> Self {
        Self {
            excluded_paths: HashMap::new(),
            require_admin: false
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
        let require_admin = self.require_admin;
        async move {
            Ok(
                JwtMiddleware { 
                    service, 
                    excluded_paths, 
                    require_admin
                }
            )
        }
    }
}

pub struct JwtMiddleware<S> {
    service: S,
    require_admin: bool,
    excluded_paths: HashMap<String, HashSet<String>>
}

impl<S> JwtMiddleware<S> {
    fn check_exclusion(&self, req: &ServiceRequest) -> bool {
        let method = req.method().as_str();
        error!("Method: {}", method);
        let path = req.path();
        for (exc_path, methods) in self.excluded_paths.iter() {
            if !path.contains(exc_path) {
                continue;
            }
            if methods.contains(method) {
                return true
            } else {
                return false
            }
        }
        false
    }

    fn run(&self, req: &mut ServiceRequest) -> ApiResult {
        if self.check_exclusion(req) {
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
        let jwt_claims = jwt_manager.validate_token(jwt_token)
            .ok_or(ApiResult::error(401, "Invalid JWT").unwrap_err())?;
        if self.require_admin && !jwt_claims.user.is_admin {
            return ApiResult::error(401, "Not an administrator");
        }
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