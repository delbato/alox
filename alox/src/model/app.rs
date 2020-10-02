use serde::{
    Deserialize,
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
    pub root_dir: Option<String>
}