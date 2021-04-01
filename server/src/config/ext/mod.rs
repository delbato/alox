pub mod block;

use std::{
    path::PathBuf,
    collections::HashMap
};

use serde::{
    Serialize,
    Deserialize
};

fn default_false() -> bool { false }
fn default_true() -> bool { true }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtDirConfig {
    /// Path to the extension directory
    pub ext_dir: PathBuf
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtConfig {
    pub ident: String,
    pub name: String,
    pub description: Option<String>,
    pub styles: HashMap<String, String>,
}