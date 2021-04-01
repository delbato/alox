use std::{
    path::PathBuf
};

use serde::{
    Serialize,
    Deserialize
};

fn default_false() -> bool { false }
fn default_true() -> bool { true }

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Config {
    #[serde(rename = "ext")]
    ExtConfig(ExtConfig)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtConfig {
    /// The extension id
    pub id: String,
    /// Path to the extension directory
    pub ext_dir: PathBuf
}