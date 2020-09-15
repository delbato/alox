use serde::{
    Serialize,
    Deserialize
};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "_id")]
    pub id: Option<String>,
    #[serde(rename = "_key")]
    pub key: Option<String>,
    pub username: String,
    pub email: String,
    pub password: String,
    pub password_salt: String,
    pub is_admin: bool
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserWithoutPassword {
    #[serde(rename = "_id")]
    pub id: Option<String>,
    #[serde(rename = "_key")]
    pub key: Option<String>,
    pub username: String,
    pub email: String,
    pub is_admin: bool
}

impl User {
    pub fn new() -> Self {
        Self {
            id: None,
            key: None,
            username: String::new(),
            email: String::new(),
            password: String::new(),
            password_salt: String::new(),
            is_admin: false,
        }
    }
}

impl From<User> for UserWithoutPassword {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            key: user.key,
            username: user.username,
            email: user.email,
            is_admin: user.is_admin
        }
    }
}