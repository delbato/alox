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
            User,
            UserWithoutPassword
        }
    },
    actix_web::{
        HttpServer,
        App,
        web,
        middleware::Logger
    },
    actix_rt,
    mdw
};

type Result<T> = StdResult<T, Box<dyn Error>>;

#[actix_rt::test]
async fn test_user_auth() -> Result<()> {
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
            .wrap(mdw::jwt::Jwt::with_exclude(&[
                "/alox-api/users/login"
            ]))
            .service(web::scope("/alox-api")
                .service(user::login_action)
                .service(user::register_action)
            )
    })
        .bind("0.0.0.0:1337")?
        .run()
        .await?;
    Ok(())
}