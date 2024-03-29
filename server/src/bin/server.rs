extern crate alox;

use alox::{
    api,
    config::alox::AloxConfig,
    db::{ArangoConnectionManager, ArangoPool},
    util::jwt::JwtManager,
};

use std::{
    error::Error as StdError,
    fs::File,
    io::{BufRead, Read},
    path::PathBuf,
    result::Result as StdResult,
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc::channel,
        Arc,
    },
    thread::sleep,
    time::Duration,
};

use actix_web::{
    web::{self, Data},
    App, HttpServer,
};
use bb8::Pool;
use clap::Clap;
use ctrlc::set_handler;
use toml::from_str;

type Result<T> = StdResult<T, Box<dyn StdError>>;

/// CLI Run parameters
#[derive(Clap)]
#[clap(
    name = "aloxd",
    version = "0.1.0",
    author = "Daniel Wanner <delbato@pm.me>"
)]
pub struct RunOps {
    /// Path to the main config file
    #[clap(short, long, default_value = "/etc/alox/alox.toml")]
    pub config_file: PathBuf,
    /// HTTP Port override
    #[clap(long)]
    pub http_port: Option<u16>,
    /// HTTPS Port override
    #[clap(long)]
    pub https_port: Option<u16>,
}

#[actix_rt::main]
pub async fn main() -> Result<()> {
    let run_ops = RunOps::parse();
    let mut file = File::open(&run_ops.config_file)?;
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)?;
    let alox_config: AloxConfig = from_str(&file_contents)?;
    let http_port = alox_config.http_port;
    println!("Starting alox on port {}...", http_port);

    let arango_manager = ArangoConnectionManager::new(
        &alox_config.arango.url,
        &alox_config.arango.username,
        &alox_config.arango.password,
    );

    let (mut tx, rx) = channel::<bool>();

    let mut should_restart = false;
    let arango_pool = Pool::builder().build(arango_manager).await?;
    let jwt_manager = JwtManager::new(&alox_config.secret);


    let http_server = HttpServer::new(move || {
        let arango_pool = arango_pool.clone();
        let jwt_manager = jwt_manager.clone();
        App::new()
            .data(arango_pool)
            .data(jwt_manager)
            .service(api::get_api_scope(&alox_config.api_prefix))
    })
    .bind(&format!("0.0.0.0:{}", http_port))?
    .run()
    .await?;

    Ok(())
}
