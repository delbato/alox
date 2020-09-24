extern crate tokio;
extern crate alox;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

use alox::{
    db::{
        ArangoConnection
    },
    model::{
        user::{
            User,
            UserWithoutPassword
        }
    },
    serde_json::to_string
};