use std::{
    path::PathBuf
};

use serde::{
    Serialize,
    Deserialize
};

fn default_false() -> bool { false }
fn default_true() -> bool { true }
fn default_http_port() -> u16 { 80 }
fn default_https_port() -> u16 { 443 }

/// Alox configuration struct
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AloxConfig {
    /// Config dir
    pub config_dir: PathBuf,
    /// Cache dir
    pub cache_dir: PathBuf,
    /// API Prefix
    pub api_prefix: String,
    /// JWT Secret
    pub secret: String,
    /// HTTP port
    #[serde(default = "default_http_port")]
    pub http_port: u16,
    /// HTTPS port
    #[serde(default = "default_https_port")]
    pub https_port: u16,
    /// HTTP enabled,
    #[serde(default = "default_true")]
    pub http_enabled: bool,
    /// HTTPS enabled,
    #[serde(default = "default_false")]
    pub https_enabled: bool,
    /// ArangoDB config
    pub arango: ArangoConfig
}

/// ArangoDB configuration struct
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ArangoConfig {
    /// URL of the ArangoDB server
    pub url: String,
    /// Username of the ArangoDB server
    pub username: String,
    /// Password of the ArangoDB server
    pub password: String,
    /// Database
    pub database: String
}