use crate::{
    model::{
        cms::Cms
    }
};

use serde::{
    Serialize,
    Deserialize
};

use serde_json::{
    Value
};

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Page {
    #[serde(rename = "full")]
    Full(PageFull),
    #[serde(rename = "flat")]
    Flat(PageFlat)
}

#[derive(Serialize, Deserialize)]
pub struct PageFlat {
    #[serde(rename = "_key")]
    pub key: Option<String>,
    #[serde(rename = "_key_cms")]
    pub key_cms: Option<String>,
    pub url: String,
    pub template: String,
    pub schema: Option<Value>,
    pub data: Option<Value>
}

#[derive(Serialize, Deserialize)]
pub struct PageFull {
    #[serde(rename = "_key")]
    pub key: Option<String>,
    pub cms: Cms,
    pub url: String,
    pub template: String,
    pub schema: Option<Value>,
    pub data: Option<Value>
}