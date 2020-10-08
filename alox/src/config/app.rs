use crate::{
    config::{
        proxy::ProxyConfig,
        cms::CmsConfig
    }
};

use std::{
    path::PathBuf
};

use serde::{
    Serialize,
    Deserialize
};

#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    pub ident: String,
    pub name: String,
    pub host: String,
    pub http_enabled: bool,
    pub https_enabled: bool,
    pub https_redir: bool,
    #[serde(rename = "proxy")]
    pub proxies: Vec<ProxyConfig>,
    pub cms: Option<CmsConfig>
}

fn default_enabled() -> bool {
    true
}