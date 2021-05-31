/// User API endpoints
pub mod user;

/// Bucket API endpoints
pub mod bucket;

/// Site API endpoints
pub mod site;

/// Content API endpoints
pub mod content;

/// Response API endpoints
pub mod response;

/// Page API endpoints
pub mod page;

/// Permission API endpoints
pub mod perm;

/// API error logic
pub mod error;

use crate::mdw::jwt::{Jwt, JwtMiddleware};

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
            web::scope("/perm")
                .wrap(Jwt::default())
                .service(perm::get_action)
                .service(perm::get_by_user_action)
                .service(perm::create_action)
                .service(perm::update_action)
                .service(perm::delete_action)
        )
        .service(
            web::scope("/sites")
                .wrap(Jwt::default())
                .service(site::get_action)
                .service(site::create_action)
                .service(site::update_action)
                .service(site::delete_action)
        )
        .service(
            web::scope("/content")
                .wrap(Jwt::default())
                .service(content::get_action)
                .service(content::create_action)
                .service(content::update_action)
                .service(content::delete_action),
        )
        .service(
            web::scope("/pages")
                .wrap(Jwt::default())
                .service(page::get_action)
                .service(page::create_action)
                .service(page::update_action)
                .service(page::delete_action)
        )
}
