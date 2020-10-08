use crate::{
    model::{
        cms::Cms,
        proxy::Proxy
    }
};

use std::{
    path::PathBuf,
    marker::PhantomData
};

use serde::{
    Deserialize,
    de::DeserializeOwned,
    Serialize
};

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum App {
    #[serde(rename = "full")]
    Full(AppFull),
    #[serde(rename = "flat")]
    Flat(AppFlat)
}

impl App {
    fn as_full(self) -> AppFull {
        if let App::Full(app_full) = self {
            app_full
        } else {
            panic!("Not a full app!");
        }
    }

    fn as_flat(self) -> AppFlat {
        if let App::Flat(app_flat) = self {
            app_flat
        } else {
            panic!("Not a flat app!");
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct AppFlat {
    #[serde(rename = "_key")]
    pub key: Option<String>,
    pub enabled: bool,
    pub ident: String,
    pub name: String,
    pub description: Option<String>,
    pub http_enabled: bool,
    pub https_enabled: bool,
    pub https_redir: bool,
    pub root_dir: Option<PathBuf>
}

#[derive(Serialize, Deserialize)]
pub struct AppFull {
    #[serde(rename = "_key")]
    pub key: Option<String>,
    pub enabled: bool,
    pub ident: String,
    pub name: String,
    pub description: Option<String>,
    pub http_enabled: bool,
    pub https_enabled: bool,
    pub https_redir: bool,
    pub root_dir: Option<PathBuf>,
    pub proxies: Vec<Proxy>,
    pub cms: Option<Cms>
}