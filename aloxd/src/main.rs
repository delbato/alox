extern crate alox;

pub mod app;

use alox::{
    actix_web::{
        self
    }
};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[actix_web::main]
async fn main() -> Result<()> {
    app::start_alox().await?;
    Ok(())
}
