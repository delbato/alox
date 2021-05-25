use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Extension {
    pub ext_dir: PathBuf,
    pub ident: String,
    pub name: String,
    pub description: Option<String>
}
