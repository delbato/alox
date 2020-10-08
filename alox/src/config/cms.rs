use std::{
    path::PathBuf
};

use serde::{
    Serialize,
    Deserialize
};

#[derive(Debug, Serialize, Deserialize)]
pub struct CmsConfig {
    pub enabled: bool,
    pub root_dir: Option<PathBuf>,
    pub base_url: String
}