use std::{
    path::PathBuf
};

use serde::{
    Serialize,
    Deserialize
};

#[derive(Serialize, Deserialize)]
pub struct Cms {
    #[serde(rename = "_key")]
    pub key: Option<String>,
    #[serde(rename = "_key_app")]
    pub key_app: String,
    pub enabled: bool,
    pub root_dir: Option<PathBuf>,
}
