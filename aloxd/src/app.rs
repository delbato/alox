use alox::{
    actix_web::{
        HttpServer,
        App,
        web::{
            self
        }
    },
    api
};

pub async fn start_alox() -> std::io::Result<()> {
    HttpServer::new(||{
        App::new().service(
            web::scope("/users")
                .default_service(web::get().to(api::user::list_action))
        )
    }).bind("0.0.0.0:10000")?
    .run()
    .await
}