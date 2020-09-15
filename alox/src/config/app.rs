use crate::{
    config::{
        proxy::ProxyConfig,
        cms::CmsConfig
    }
};

use serde::{
    Serialize,
    Deserialize
};

#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
    pub identifier: String,
    #[serde(rename = "proxy")]
    pub proxies: Vec<ProxyConfig>,
    pub cms: Vec<CmsConfig>
}