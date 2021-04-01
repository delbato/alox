/// Alox server config module
pub mod alox;

/// Extension config module
pub mod ext;

use serde::{
    Serialize,
    Deserialize
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Config {
    #[serde(rename = "ext")]
    ExtDir(ext::ExtDirConfig)
}