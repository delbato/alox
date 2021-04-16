use std::{
    path::PathBuf,
    collections::HashMap
};

use serde::{
    Serialize,
    Deserialize
};

fn default_children() -> BlockChildType { BlockChildType::None }
fn default_description() -> Option<String> { None }

#[derive(Serialize, Deserialize, Debug)]
pub struct Block {
    pub ident: String,
    pub name: String,
    #[serde(default = "default_description")]
    pub description: Option<String>,
    pub template: PathBuf,
    pub preview_template_str: Option<String>,
    pub params: HashMap<String, BlockParamType>,
    #[serde(default = "default_children")]
    pub children: BlockChildType
}

#[derive(Debug, Serialize, Deserialize)]
pub enum BlockChildType {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "grid")]
    Grid {
        width: u8,
        height: u8
    },
    #[serde(rename = "autogrid")]
    AutoGrid {
        by_row: bool,
        size: u8
    },
    #[serde(rename = "array")]
    Array {
        size: u8
    },
    #[serde(rename = "list")]
    List
}

#[derive(Serialize, Deserialize, Debug)]
pub enum BlockParamType {
    #[serde(rename = "number")]
    Number,
    #[serde(rename = "range")]
    Range(f32, f32),
    #[serde(rename = "text")]
    Text,
    #[serde(rename = "bigtext")]
    BigText,
    #[serde(rename = "bool")]
    Bool,
    #[serde(rename = "asset")]
    Asset,
    #[serde(rename = "html")]
    Html,
    #[serde(rename = "enum")]
    Enum(Vec<String>)
}
