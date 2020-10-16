extern crate alox;
extern crate tokio;
extern crate env_logger;

use std::{
    result::Result as StdResult,
    error::Error,
};

use alox::{
    util::{
        jwt::{
            JwtManager,
            JwtClaims
        }
    },
    db::get_connection_pool,
    api::{
        user
    },
    model::{
        user::{
            UserFlat,
            UserNoPw
        }
    },
    actix_web::{
        HttpServer,
        App,
        web,
        middleware::Logger
    },
    actix_rt,
    middleware::jwt::Jwt,
    maplit::{
        hashmap,
        hashset
    }
};

type Result<T> = StdResult<T, Box<dyn Error>>;

#[actix_rt::test]
async fn test_user_api() -> Result<()> {
    env_logger::init();
    let arango_pool = get_connection_pool("http://localhost:8529", "alox", "alox", 16).await?;
    let jwt_manager = JwtManager::new("12345");
    HttpServer::new(move || {
        let arango_pool = arango_pool.clone();
        let jwt_manager = jwt_manager.clone();
        App::new()
            .data(jwt_manager)
            .data(arango_pool)
            .wrap(Logger::default())
            .service(web::scope("/alox-api")
                .wrap(Jwt::default()
                    .with_exclude(hashmap!{
                        "/users/login" => vec![ "POST" ],
                        "/users" => vec![ "POST" ]
                    })
                    .with_require_admin(false)
                )
                .service(user::get_action)
                .service(user::edit_action)
                .service(user::login_action)
                .service(user::register_action)
            )
    })
        .bind("0.0.0.0:1337")?
        .run()
        .await?;
    Ok(())
}