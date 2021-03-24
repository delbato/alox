use crate::{
    util::{
        generate_hash
    },
    model::{
        permission::Permission
    }
};

use serde::{
    Serialize,
    Deserialize
};

#[derive(Deserialize)]
#[serde(tag = "type")]
pub enum User {
    #[serde(rename = "flat")]
    Flat(UserFlat),
    #[serde(rename = "flat")]
    Full(UserFull),
}

impl User {
    pub fn as_flat(self) -> UserFlat {
        if let User::Flat(user_flat) = self {
            user_flat
        } else {
            panic!("Not matching!");
        }
    }

    pub fn as_full(self) -> UserFull {
        if let User::Full(user_full) = self {
            user_full
        } else {
            panic!("Not matching!");
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct UserFull {
    #[serde(rename = "_key")]
    pub key: Option<String>,
    pub username: String,
    pub email: String,
    pub password: String,
    pub password_salt: String,
    pub is_admin: bool,
    pub permissions: Vec<Permission>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserFlat {
    #[serde(rename = "_key", skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    pub username: String,
    pub email: String,
    pub password: String,
    pub password_salt: String,
    pub is_admin: bool
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserNoPw {
    #[serde(rename = "_key")]
    pub key: Option<String>,
    pub username: String,
    pub email: String,
    pub is_admin: bool
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserClaims {
    #[serde(rename = "_key")]
    pub key: Option<String>,
    pub username: String,
    pub email: String,
    pub is_admin: bool,
    pub permissions: Vec<Permission>
}

impl UserFlat {
    pub fn new() -> Self {
        Self {
            key: None,
            username: String::new(),
            email: String::new(),
            password: String::new(),
            password_salt: String::new(),
            is_admin: false,
        }
    }

    pub fn verify_password(&self, password: &str) -> bool {
        let password_salted = format!("{}{}", password, self.password_salt);
        let password_hashed = generate_hash(&password_salted);
        self.password == password_hashed
    }
}

impl From<UserFlat> for UserNoPw {
    fn from(user: UserFlat) -> Self {
        Self {
            key: user.key,
            username: user.username,
            email: user.email,
            is_admin: user.is_admin
        }
    }
}