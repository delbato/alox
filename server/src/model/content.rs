use crate::{
    config::{
        ext::{
            block::BlockParamType
        }
    }
};

use std::{
    collections::HashMap
};

use serde::{
    Serialize,
    Deserialize
};
use serde_json::{
    Value
};

#[derive(Debug, Serialize, Deserialize)]
pub struct ContentFlat {
    #[serde(rename = "_key")]
    pub key: Option<String>,
    pub linked: bool,
    pub block_ident: String,
    pub params: HashMap<String, Value>,
    pub children: Option<ContentChildrenFlat>
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ContentChildrenFlat {
    #[serde(rename = "array")]
    Array {
        size: u8,
        children: Vec<String>
    },
    #[serde(rename = "list")]
    List {
        children: Vec<String>
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Content {
    #[serde(rename = "_key")]
    pub key: Option<String>,
    pub linked: bool,
    pub block_ident: String,
    pub params: HashMap<String, Value>,
    pub children: Option<ContentChildren>
}

impl From<ContentFlat> for Content {
    fn from(content_flat: ContentFlat) -> Self {
        Self {
            key: content_flat.key,
            linked: content_flat.linked,
            block_ident: content_flat.block_ident,
            params: content_flat.params,
            children: None
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ContentChildren {
    #[serde(rename = "array")]
    Array {
        size: u8,
        children: Vec<Content>
    },
    #[serde(rename = "list")]
    List {
        children: Vec<Content>
    }
}