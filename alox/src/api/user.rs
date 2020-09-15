use crate::{
    api::{
        response::ApiResponse
    },
    model::{
        user::User
    },
    db::{
        ArangoConnection
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
        Data
    },
    http::{
        StatusCode
    }
};

#[post("/users/login")]
pub async fn login_action(_req: HttpRequest, user_login: Json<Value>, arango: Data<ArangoConnection>, jwt: Data<JwtManager>) -> ApiResponse<String> {
    let username = user_login.get("username").unwrap()
        .as_str()
        .unwrap();
    let password = user_login.get("password").unwrap()
        .as_str()
        .unwrap();
    let alox_db = arango.get_db("alox").await.unwrap();
    let aql = format!("
        FOR u IN users
            FILTER u.username == \"{}\"
            RETURN u
    ", username);
    let user: User = alox_db.aql_str(&aql).await
        .unwrap()
        .remove(0);
    let password_salted = format!("{}{}", password, user.password_salt);
    let password_hashed = generate_hash(&password_salted);
    
    return if user.password != password_hashed {
        ApiResponse::new(false, String::from("Invalid credentials"))
    } else {
        let jwt_claims = JwtClaims::from(user);
        let jwt_token = jwt.generate_token(jwt_claims);
        ApiResponse::new(true, jwt_token)
    };
}

#[post("/users")]
pub async fn register_action(user_registration: Json<Value>, arango: Data<ArangoConnection>) -> ApiResponse<String> {
    let username = user_registration.get("username").unwrap()
        .as_str()
        .unwrap()
        .to_string();
    let password_raw = user_registration.get("password").unwrap()
        .as_str()
        .unwrap()
        .to_string();
    let email = user_registration.get("email").unwrap()
        .as_str()
        .unwrap()
        .to_string();
    let password_salt = generate_salt(16);
    let password_salted = format!("{}{}", password_raw, password_salt);
    let password = generate_hash(&password_salted);

    let user_value = json!({
        "username": username,
        "password": password,
        "password_salt": password_salt,
        "email": email,
        "is_admin": false
    });

    let mut bind_vars = HashMap::new();
    bind_vars.insert("user", user_value);

    let alox_db = arango.get_db("alox").await.unwrap();

    let insert_res: Vec<Value> = alox_db.aql_bind_vars("INSERT @user INTO users LET result = NEW RETURN result", bind_vars)
        .await
        .unwrap();
        
    return if insert_res.len() != 1 {
        ApiResponse::new(false, format!("Failed to register this user"))
    } else {
        ApiResponse::new(true, format!("User successfully registered"))
    };
}

pub async fn list_action(_req: HttpRequest) -> HttpResponse {
    HttpResponse::build(StatusCode::OK)
        .content_type("application/json")
        .body("
            {
                \"success\":true
            }
        ")
}