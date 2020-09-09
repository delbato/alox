use crate::{
    model::{
        user::{
            User,
            UserWithoutPassword
        }
    }
};

use std::{
};

use jwt::{

};

use chrono::{
    DateTime,
    Utc,
    Duration
};

pub struct JwtClaims {
    pub aud: String,
    pub sub: String,
    pub exp: usize,
    pub user: UserWithoutPassword
}

impl From<User> for JwtClaims {
    fn from(user: User) -> Self {
        let exp: DateTime<Utc> = Utc::now() + Duration::days(30);
        Self {
            sub: user.username.clone(),
            aud: String::from("alox"),
            exp: exp.timestamp() as usize,
            user: user.into()
        }
    }
}

pub struct UserClaims {
    pub username: String,
}

pub struct JwtManager {
    secret: String,
}

impl JwtManager {
    pub fn new<S: Into<String>>(secret: S) -> Self {
        Self {
            secret: secret.into()
        }
    }
}