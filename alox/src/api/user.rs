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
pub struct LoginBody {
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
pub struct RegisterBody {
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

#[derive(Deserialize)]
pub struct EditBody {
    username: Option<String>,
    password: Option<String>,
    password_old: Option<String>,
    email: Option<String>,
    is_admin: Option<bool>
}

#[put("/users/{user_key}")]
pub async fn edit_action(edit_body: Json<EditBody>, user_key: Path<String>, jwt_claims: JwtClaims, user_repo: UserRepo) -> ApiResult {
    if &*user_key != jwt_claims.user.key.as_ref().unwrap() && !jwt_claims.user.is_admin {
        return ApiResult::error(403, "Not authorized to do this");
    }

    let mut user = user_repo.find(&*user_key).await?;

    if let Some(username) = edit_body.username.as_ref().cloned() {
        user.username = username;
    }

    if edit_body.password.is_some() && edit_body.password_old.is_some() {
        let password = edit_body.password.as_ref()
            .cloned()
            .unwrap();
        let password_old = edit_body.password_old.as_ref()
            .cloned()
            .unwrap();
        let password_old_salted = format!("{}{}", password_old, user.password_salt);
        let password_old_hashed = generate_hash(&password_old_salted);
        if user.password != password_old_hashed {
            return ApiResult::error(400, "Passwords do not match");
        }
        let salt = generate_salt(16);
        let password_new_salted = format!("{}{}", salt, password);
        let password_new_hashed = generate_hash(&password_new_salted);
        user.password = password_new_hashed;
        user.password_salt = salt;
    }

    if let Some(email) = edit_body.email.as_ref().cloned() {
        user.email = email;
    }

    if edit_body.is_admin.is_some() && jwt_claims.user.is_admin {
        let is_admin = edit_body.is_admin.unwrap();
        user.is_admin = is_admin;
    } else if edit_body.is_admin.is_some() && !jwt_claims.user.is_admin {
        return ApiResult::error(403, "Not authorized to do this");
    }

    user_repo.update(user).await?;

    ApiResult::success("User successfully updated")
}

#[get("/users/{user_key}")]
pub async fn get_action(user_key: Path<String>, jwt_claims: JwtClaims, user_repo: UserRepo) -> ApiResult {
    println!("user_key: {}, jwt_user_key: {}", user_key, jwt_claims.user.key.as_ref().unwrap());
    if &*user_key != jwt_claims.user.key.as_ref().unwrap() && !jwt_claims.user.is_admin {
        return ApiResult::error(403, "Not authorized to do this");
    }
    let user = user_repo.find(&*user_key).await?;
    ApiResult::success(user)
}