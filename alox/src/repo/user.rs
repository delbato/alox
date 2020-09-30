use crate::{
    db::{
        ArangoConnectionManager,
        ArangoConnection,
        ArangoPool
    },
    model::{
        user::User
    }
};

use std::{
    pin::Pin,
    ops::Deref
};

use serde_json::{
    Value,
    json,
    to_value
};
use maplit::hashmap;
use futures::Future;
use actix_web::{
    FromRequest,
    HttpRequest,
    dev::Payload,
    web::Data
};
use arangors::{
    Connection,
    Database,
    client::reqwest::ReqwestClient
};


pub struct UserRepo {
    database: Database<ReqwestClient>
}

impl UserRepo {
    pub async fn new(connection: &Connection) -> Self {
        let database = connection.db("alox").await.expect("Database \"alox\" does not exist!");
        Self {
            database
        }
    }

    pub async fn find(&self, user_key: &str) -> Result<User, ()> {
        let mut result_vec: Vec<User> = self.database.aql_bind_vars("
            RETURN DOCUMENT(\"users/@key\")
        ", hashmap!{
            "key" => user_key.into()
        }).await
            .map_err(|_| ())?;
        if result_vec.len() != 1 {
            Err(())
        } else {
            result_vec.pop()
                .ok_or(())
        }
    }

    pub async fn find_by_username(&self, username: &str) -> Result<User, ()> {
        println!("Attempting to find by username...");
        let mut result_vec: Vec<User> = self.database.aql_bind_vars("
            FOR u IN users
                FILTER u.username == @username
                RETURN u
        ", hashmap!{
            "username" => username.into()
        }).await
            .map_err(|err| {
                eprintln!("{:#?}", err);
                ()
            })?;
            
        println!("Users with this username: {:#?}", result_vec);

        if result_vec.len() != 1 {
            return Err(());
        }

        result_vec.pop()
            .ok_or(())
    }

    pub async fn insert(&self, user: User) -> Result<User, ()> {
        let mut result_vec: Vec<User> = self.database.aql_bind_vars("
            INSERT {
                username: @username,
                email: @email,
                password: @password,
                password_salt: @password_salt,
                is_admin: @is_admin
            } INTO users
            RETURN NEW
        ", hashmap!{
            "username" => user.username.into(),
            "password" => user.password.into(),
            "password_salt" => user.password_salt.into(),
            "email" => user.email.into(),
            "is_admin" => user.is_admin.into()
        }).await
            .map_err(|_| ())?;
        if result_vec.len() != 1 {
            return Err(());
        }
        result_vec.pop()
            .ok_or(())
    }
}

impl FromRequest for UserRepo {
    type Error = ();
    type Config = ();
    type Future = impl Future<Output = Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let pool = req.app_data::<Data<ArangoPool>>().expect("No database pool!")
            .clone();
        async move {
            let conn = pool.get().await.map_err(|_| ())?;
            let user_repo = UserRepo::new(&conn).await;
            Ok(user_repo)
        }
    }
}
