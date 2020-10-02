use serde::{
    Deserialize,
    Serialize
};
use serde_json::{
    Value
};

#[derive(Serialize, Deserialize)]
pub struct Bucket {
    #[serde(rename = "_key")]
    pub key: Option<String>,
    pub ident: String,
    pub collection: String,
    pub schema: Value
}