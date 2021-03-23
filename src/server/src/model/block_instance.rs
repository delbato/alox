use serde::{
    Serialize,
    Deserialize
};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct BlockInstance {
    #[serde(rename = "_key")]
    pub key: Option<String>,
    #[serde(rename = "_key_page")]
    pub key_page: String,
    pub sorting: usize,
    pub block_ident: String,
    pub params: Value
}
