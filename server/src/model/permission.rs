use serde::{
    Deserialize,
    Serialize
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PermissionType {
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "write")]
    Write,
    #[serde(rename = "admin")]
    Admin,
    #[serde(rename = "owner")]
    Owner
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Permission {
    #[serde(rename = "_key")]
    pub key: Option<String>,
    #[serde(rename = "_key_user")]
    pub key_user: String,
    #[serde(rename = "_key_app")]
    pub key_app: String,
    pub permission_type: PermissionType
}