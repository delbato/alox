use std::{
    path::PathBuf
};

use serde::{
    Serialize,
    Deserialize
};

#[derive(Serialize, Deserialize)]
pub struct Site {
    #[serde(rename = "_key")]
    pub key: Option<String>,
    #[serde(rename = "_key_app")]
    pub key_app: String,
    pub enabled: bool,
    pub port: Option<String>,
    pub name: String,
    pub description: Option<String>,
    pub root_dir: Option<PathBuf>,
}
