use serde::{
    Serialize,
    Deserialize
};

use serde_json::{
    Value
};

fn default_content() -> Vec<String> { vec![] }

#[derive(Debug, Serialize, Deserialize)]
pub struct PageFlat {
    #[serde(rename = "_key")]
    pub key: Option<String>,
    #[serde(rename = "_key_site")]
    pub key_site: Option<String>,
    #[serde(rename = "_key_parent_page")]
    pub key_parent_page: Option<String>,
    #[serde(default = "default_content")]
    pub content: Vec<String>,
    pub url: String,
    pub title: String
}

#[derive(Debug, Serialize)]
pub struct Page {
    #[serde(rename = "_key")]
    pub key: Option<String>,
    #[serde(rename = "_key_site")]
    pub key_site: Option<String>,
    pub url: String,
    pub title: String,
    pub children: Vec<Page>
}