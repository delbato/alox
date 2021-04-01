extern crate alox;
extern crate clap;
extern crate string_error;

use alox::{
    actix_rt,
    arangors::{client::reqwest::ReqwestClient, collection::CollectionType, Database},
    bb8::{Pool, PooledConnection},
    config::alox::AloxConfig,
    db::ArangoConnectionManager,
    maplit::hashmap,
    model::user::{User, UserFlat},
    repo::user::UserRepo,
    serde_json::json,
    toml::from_str,
    util::{generate_hash, generate_salt},
};

use clap::Clap;
use std::{
    error::Error as StdError,
    fs::File,
    io::{BufRead, Read},
    path::PathBuf,
    result::Result as StdResult,
};
use string_error::new_err;

/// Run ops for CLI
#[derive(Clap)]
#[clap(
    name = "aloxctl",
    version = "0.1.0",
    author = "Daniel Wanner <delbato@pm.me>",
    about = "alox control utility"
)]
pub struct RunOps {
    /// Path to the main config file
    #[clap(short, long, default_value = "/etc/alox/alox.toml")]
    pub config_file: PathBuf,
    #[clap(subcommand)]
    pub subcmd: SubCommand,
}

#[derive(Clap)]
pub enum SubCommand {
    #[clap(about = "User management")]
    Users(UsersSubCommand),
    #[clap(about = "DB management")]
    Db(DbSubCommand),
}

#[derive(Clap)]
pub enum DbSubCommand {
    #[clap(about = "Re-create database")]
    Create {
        #[clap(index = 1, required = true)]
        database: String,
    },
    #[clap(about = "Drop database")]
    Drop {
        #[clap(index = 1, required = true)]
        database: String,
    },
    #[clap(name = "init", about = "Initialize database")]
    Initialize {
        #[clap(index = 1, required = true)]
        database: String,
    },
}

#[derive(Clap)]
pub enum UsersSubCommand {
    #[clap(about = "Create a new admin user")]
    Create(UsersCreateParams),
}

#[derive(Clap)]
pub struct UsersCreateParams {
    #[clap(index = 1, required = true)]
    pub username: String,
    #[clap(index = 2, required = true)]
    pub password: String,
    #[clap(index = 3, required = true)]
    pub email: String,
}

type Result<T> = StdResult<T, Box<dyn StdError>>;

#[actix_rt::main]
async fn main() -> Result<()> {
    let run_ops = RunOps::parse();
    let mut file = File::open(&run_ops.config_file)?;
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)?;
    let alox_config: AloxConfig = from_str(&file_contents)?;
    let arango_manager = ArangoConnectionManager::new(
        &alox_config.arango.url,
        &alox_config.arango.username,
        &alox_config.arango.password,
    );
    let arango_pool = Pool::builder().build(arango_manager).await?;
    let connection = arango_pool.get().await.unwrap();

    match run_ops.subcmd {
        SubCommand::Db(subcmd) => subcmd_db(subcmd, &connection).await?,
        SubCommand::Users(subcmd) => subcmd_users(subcmd, &connection).await?,
    };

    Ok(())
}

async fn subcmd_db<'c>(
    subcmd: DbSubCommand,
    connection: &PooledConnection<'c, ArangoConnectionManager>,
) -> Result<()> {
    match subcmd {
        DbSubCommand::Create { database } => {
            let db_list = connection.accessible_databases().await?;
            if !db_list.contains_key(&database) {
                println!("Database \"{}\" doesn't exist, creating it...", &database);
                connection.create_database(&database).await?;
                println!("Done. Database \"{}\" created.", &database)
            } else {
                return Err(new_err(&format!(
                    "Database \"{}\" already exists.",
                    &database
                )));
            }
        }
        DbSubCommand::Drop { database } => {
            let db_list = connection.accessible_databases().await?;
            if !db_list.contains_key(&database) {
                return Err(new_err(&format!(
                    "Database \"{}\" doesn't exist.",
                    &database
                )));
            } else {
                println!("Deleting database \"{}\"...", &database);
                connection.drop_database(&database).await?;
                println!("Done. Database \"{}\" deleted.", &database)
            }
        }
        DbSubCommand::Initialize { database } => {
            let db_list = connection.accessible_databases().await?;
            if !db_list.contains_key(&database) {
                return Err(new_err(&format!(
                    "Database \"{}\" doesn't exist.",
                    &database
                )));
            } else {
                println!("Initializing Database structure...");
                let database = connection.db(&database).await?;
                let db_list = vec!["users"];
                for db_name in db_list {
                    create_collection(&database, db_name).await?;
                }
                println!("Done.");
            }
        }
    };

    Ok(())
}

async fn create_collection(database: &Database<ReqwestClient>, name: &str) -> Result<()> {
    let db_list = database.accessible_collections().await?;
    if db_list.into_iter().any(|item| item.name == name) {
        return Err(new_err(&format!("Collection already exists.")));
    }
    database.create_collection(&name).await?;
    Ok(())
}

async fn subcmd_users<'c>(
    subcmd: UsersSubCommand,
    connection: &PooledConnection<'c, ArangoConnectionManager>,
) -> Result<()> {
    let database = connection.db("alox").await?;
    match subcmd {
        UsersSubCommand::Create(params) => {
            println!("Creating new user \"{}\"...", params.username);
            let password_salt = generate_salt(32);
            let password_salted = format!("{}{}", params.password, password_salt);
            let password_hashed = generate_hash(&password_salted);

            let user = UserFlat {
                key: None,
                password: password_hashed,
                password_salt,
                email: params.email,
                username: params.username,
                is_admin: true,
                permissions: vec![]
            };
            let json = json!(&user);
            println!("{}", json);
            let mut result_vec: Vec<UserFlat> = database
                .aql_bind_vars(
                    "
                INSERT @user INTO users
                RETURN NEW
            ",
                    hashmap! {
                        "user" => json
                    },
                )
                .await?;
            result_vec.pop().ok_or(());
            println!("Done.");
        }
    };
    Ok(())
}
