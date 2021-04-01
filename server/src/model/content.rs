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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Content {
    #[serde(rename = "_key")]
    pub key: Option<String>,
    pub linked: bool,
    pub block_ident: String,
    pub args: HashMap<String, Value>
}