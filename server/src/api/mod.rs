pub mod user;

pub mod bucket;

pub mod site;

pub mod content;

pub mod response;

pub mod error;

use crate::middleware::jwt::{Jwt, JwtMiddleware};

use actix_web::{web, Scope};
use maplit::{hashmap, hashset};

/// Gets the actix api scope for the API endpoints
pub fn get_api_scope(prefix: &str) -> Scope {
    web::scope(prefix)
        .service(
            web::scope("/users")
                .wrap(
                    Jwt::default()
                        .with_require_admin(true)
                        .with_exclude(hashmap! {
                            "/users/login" => vec![ "POST" ]
                        }),
                )
                .service(user::get_action)
                .service(user::login_action)
                .service(user::edit_action),
        )
        .service(
            web::scope("/content")
                .wrap(Jwt::default())
                .service(content::get_action)
                .service(content::create_action)
                .service(content::update_action)
                .service(content::delete_action),
        )
}
