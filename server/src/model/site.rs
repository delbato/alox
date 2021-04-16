use std::{
    path::PathBuf
};

use serde::{
    Serialize,
    Deserialize
};

#[derive(Serialize, Deserialize)]
pub struct SiteFlat {
    #[serde(rename = "_key")]
    pub key: Option<String>,
    pub enabled: bool,
    pub base_url: String,
    pub host: String,
    pub name: String,
    pub description: Option<String>,
    pub root_page: Option<String>
}

#[derive(Serialize, Deserialize)]
pub struct Site {
    #[serde(rename = "_key")]
    pub key: Option<String>,
    pub enabled: bool,
    pub base_url: String,
    pub host: String,
    pub name: String,
    pub description: Option<String>,
    pub root_page: Option<String>
}