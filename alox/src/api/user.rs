use crate::{
    api::{
        response::ApiResponse,
        error::{
            ApiResult,
            ApiResultExt,
            ApiError,
            ApiErrorType,
        }
    },
    model::{
        user::User
    },
    repo::{
        user::UserRepo
    },
    util::{
        jwt::{
            JwtManager,
            JwtClaims
        },
        generate_hash,
        generate_salt
    }
};

use std::{
    collections::HashMap,
    sync::Arc
};

use serde::{
    Deserialize
};
use serde_json::{
    Value,
    json
};
use actix_web::{
    get,
    put,
    post,
    delete,
    web::{
        HttpRequest,
        HttpResponse,
        Json,
        Data,
        Path
    },
    http::{
        StatusCode
    }
};

#[derive(Deserialize)]
struct LoginBody {
    pub username: String,
    pub password: String
}

#[post("/users/login")]
pub async fn login_action(login_body: Json<LoginBody>, user_repo: UserRepo, jwt: Data<JwtManager>) -> ApiResult {
    let user = user_repo.find_by_username(&login_body.username).await
        .map_err(|_| ApiResult::error(404, "User not found!").unwrap_err())?;
    
    if !user.verify_password(&login_body.password) {
        return ApiResult::error(401, "Incorrect password or username");
    }

    let jwt_claims = JwtClaims::from(user);
    let token = jwt.generate_token(jwt_claims);
    
    ApiResult::success(json!({
        "message": "Login successful!",
        "token": token
    }))
}

#[derive(Deserialize)]
struct RegisterBody {
    pub username: String,
    pub password: String,
    pub email: String
}

#[post("/users")]
pub async fn register_action(register_body: Json<RegisterBody>, user_repo: UserRepo) -> ApiResult {
    let user_exists = user_repo.find_by_username(&register_body.username).await
        .is_ok();
    if user_exists {
        return ApiResult::error(400, "Username taken");
    }

    let password_salt = generate_salt(32);
    let password_salted = format!("{}{}", register_body.password, password_salt);
    let password_hashed = generate_hash(&password_salted);

    let user = User {
        id: None,
        key: None,
        password: password_hashed,
        password_salt,
        email: register_body.email.clone(),
        username: register_body.username.clone(),
        is_admin: false
    };

    user_repo.insert(user).await
        .map_err(|_| ApiResult::error(500, "Couldnt insert user into DB").unwrap_err())?;

    ApiResult::success("User successfuly registered")
}

#[put("/users/{user_id}")]
pub async fn edit_action(user_id: Path<String>) -> ApiResult {
    Err(ApiError::new_msg(
        501,
        "Not implemented"
    ))
}

#[get("/users/{user_id}")]
pub async fn get_action(user_id: Path<String>, jwt_claims: JwtClaims, user_repo: UserRepo) -> ApiResult {
    let user_key = jwt_claims.user.key.unwrap();
    if user_key != *user_id && !jwt_claims.user.is_admin {
        return ApiResult::error(401, "Unauthorized");
    }
    let user = user_repo.find(&*user_id).await
        .map_err(|_| ApiResult::error(404, "User not found").unwrap_err())?;
    ApiResult::success(user)
}