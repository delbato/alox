use serde::{
    Deserialize,
    Serialize
};

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum PermissionType {
    Read,
    Write,
    Admin,
    Owner
}


#[derive(Serialize, Deserialize)]
pub struct Permission {
    #[serde(rename = "_key")]
    pub key: Option<String>,
    pub key_app: String,
    pub permission_type: PermissionType
}