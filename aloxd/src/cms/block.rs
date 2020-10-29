use std::{
    path::PathBuf,
    collections::HashMap
};

use serde::{
    Serialize,
    Deserialize
};

#[derive(Serialize, Deserialize, Debug)]
pub struct Block {
    pub ident: String,
    pub template: PathBuf,
    pub preview_template_str: Option<String>,
    pub params: HashMap<String, BlockParamType>
}

#[derive(Serialize, Deserialize, Debug)]
pub enum BlockParamType {
    #[serde(rename = "number")]
    Number,
    #[serde(rename = "text")]
    Text,
    #[serde(rename = "bool")]
    Bool,
    #[serde(rename = "asset")]
    Asset,
    #[serde(rename = "richtext")]
    RichText,
    #[serde(rename = "html")]
    Html,
    #[serde(rename = "block")]
    Block,
    #[serde(rename = "list")]
    List(Box<BlockParamType>),
    #[serde(rename = "row")]
    Row(Vec<BlockParamType>),
    #[serde(rename = "enum")]
    Enum(Vec<String>)
}

