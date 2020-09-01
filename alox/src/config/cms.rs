use std::{
    path::PathBuf
};

use serde::{
    Serialize,
    Deserialize
};

#[derive(Debug, Serialize, Deserialize)]
pub struct CmsConfig {
    pub host: String,
    pub port: u16,
    pub root_dir: PathBuf
}