extern crate alox;
extern crate tokio;

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
    db::ArangoConnection,
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
        web
    },
    actix_rt
};

type Result<T> = StdResult<T, Box<dyn Error>>;

#[actix_rt::test]
async fn test_user_auth() -> Result<()> {
    let arango = ArangoConnection::new("http://localhost:8529", "alox", "alox").await?;
    let jwt_manager = JwtManager::new("12345");
    HttpServer::new(move || {
        let arango = arango.clone();
        let jwt_manager = jwt_manager.clone();
        App::new()
            .data(jwt_manager)
            .data(arango)
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