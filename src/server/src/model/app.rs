use crate::{
    model::{
        site::Site
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
pub struct App {
    #[serde(rename = "_key")]
    pub key: Option<String>,
    pub enabled: bool,
    pub ident: String,
    pub name: String,
    pub description: Option<String>,
    pub root_dir: PathBuf
}