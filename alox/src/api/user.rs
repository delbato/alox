use crate::{
    api::{
        response::ApiResponse,
        error::{
            ApiResult,
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
pub async fn login_action(login_body: Json<LoginBody>, user_repo: UserRepo, jwt: Data<JwtManager>) -> ApiResult<Value> {
    let user = user_repo.find_by_username(&login_body.username).await
        .map_err(|_| ApiError::new_msg(
            404,
            "User not found"
        ))?;
    
    if !user.verify_password(&login_body.password) {
        return Err(ApiError::new_msg(
            401,
            "Unauthorized"
        ));
    }

    let jwt_claims = JwtClaims::from(user);
    let token = jwt.generate_token(jwt_claims);
    
    Ok(ApiResponse::new(true, json!({
        "message": "Logged in successfully",
        "token": token
    })))
}

#[derive(Deserialize)]
struct RegisterBody {
    pub username: String,
    pub password: String,
    pub email: String
}

#[post("/users")]
pub async fn register_action(register_body: Json<RegisterBody>, user_repo: UserRepo) -> ApiResult<String> {
    let user_exists = user_repo.find_by_username(&register_body.username).await
        .is_ok();
    if user_exists {
        return Err(ApiError::new_msg(
            400,
            "User already exists"
        ));
    }

    let password_salt = generate_salt(16);
    let password_salted = format!("{}{}", register_body.password, password_salt);
    let password_hashed = generate_hash(&password_salted);

    let mut user = User {
        id: None,
        key: None,
        password: password_hashed,
        password_salt,
        email: register_body.email.clone(),
        username: register_body.username.clone(),
        is_admin: false
    };

    user_repo.insert(user).await
        .map_err(|_| ApiError::new_msg(
            500,
            "Couldnt register user"
        ))?;

    Ok(ApiResponse::new(
        true,
        String::from("User successfully registered!")
    ))
}

#[put("/users/{user_id}")]
pub async fn edit_action(user_id: Path<u32>) -> ApiResult<String> {
    Err(ApiError::new_msg(
        501,
        "Not implemented"
    ))
}