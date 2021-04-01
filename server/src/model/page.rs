use serde::{
    Serialize,
    Deserialize
};

use serde_json::{
    Value
};

#[derive(Debug, Serialize, Deserialize)]
pub struct PageFlat {
    #[serde(rename = "_key")]
    pub key: Option<String>,
    #[serde(rename = "_key_app")]
    pub key_app: Option<String>,
    #[serde(rename = "_key_parent_page")]
    pub key_parent_page: Option<String>,
    pub url: String,
    pub title: String
}

#[derive(Debug, Serialize)]
pub struct Page {
    #[serde(rename = "_key")]
    pub key: Option<String>,
    #[serde(rename = "_key_app")]
    pub key_app: Option<String>,
    pub url: String,
    pub title: String,
    pub children: Vec<Page>
}