pub mod cms;

pub mod proxy;

pub mod app;

use std::{
    path::PathBuf
};

use serde::{
    Serialize,
    Deserialize
};

#[derive(Debug, Serialize, Deserialize)]
pub struct AloxConfig {
    pub arango: ArangoConfig,
    pub storage_dir: PathBuf,
    pub conf_dir: PathBuf
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ArangoConfig {
    pub url: String,
    pub username: String,
    pub password: String,
    pub db: String
}
