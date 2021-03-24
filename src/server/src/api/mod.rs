pub mod user;

pub mod bucket;

pub mod response;

pub mod error;

use actix_web::{
    Scope,
    web
};

/// Gets the actix api scope for the API endpoints
pub fn get_api_scope(prefix: &str) -> Scope {
    web::scope(prefix)
        .service(web::scope("/users")
            .service(user::get_action)
            .service(user::login_action)
            .service(user::edit_action)
            .service(user::register_action)
            .service(user::get_permissions_action)
        )
}