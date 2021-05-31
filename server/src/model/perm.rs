use serde::{
    Serialize,
    Deserialize
};


#[derive(Debug, Serialize, Deserialize)]
pub struct Permission {
    #[serde(rename = "_key")]
    pub key: Option<String>,
    pub key_user: String,
    pub key_site: String,
    pub perm_type: PermissionType
}

#[derive(Debug, Serialize, Deserialize)]
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
