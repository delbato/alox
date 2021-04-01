use crate::{
    db::{
        ArangoConnectionManager,
        ArangoConnection,
        ArangoPool
    },
    model::{
        user::{
            UserFlat,
            User
        }
    }
};

use std::{
    pin::Pin,
    ops::Deref,
    sync::Arc
};

use log::error;
use serde_json::{
    Value,
    json,
    to_value
};
use maplit::hashmap;
use futures::{
    future::{
        Future,
    },
    task::Poll
};
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
    pub async fn new(pool: Data<ArangoPool>) -> Self {
        let connection = pool.get().await.unwrap();
        let database = connection.db("alox").await.expect("Database \"alox\" does not exist!");
        Self {
            database
        }
    }

    pub async fn find(&self, user_key: &str, full: bool) -> Result<User, ()> {
        let query = if full {
            "
            LET user = DOCUMENT(CONCAT(\"users/\", @key))
            user.type = \"full\"
            LET permissions = (
                FOR perm IN permissions
                    FILTER perm._key_user = @key
                    RETURN perm
            )
            return MERGE(user, { permissions: permissions })
            "
        } else {
            "
            LET user = DOCUMENT(CONCAT(\"users/\", @key))
            user.type = \"flat\"
            RETURN user
            "
        };
        let mut result_vec: Vec<User> = self.database.aql_bind_vars(query, hashmap!{
            "key" => user_key.into()
        }).await
            .map_err(|err| {
                error!("AQL Error: {:#?}", err);
                ()
            })?;
        if result_vec.len() != 1 {
            Err(())
        } else {
            result_vec.pop()
                .ok_or(())
        }
    }

    pub async fn find_by_username(&self, username: &str) -> Result<UserFlat, ()> {
        println!("Attempting to find by username...");
        let mut result_vec: Vec<UserFlat> = self.database.aql_bind_vars("
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

    pub async fn insert(&self, user: UserFlat) -> Result<UserFlat, ()> {
        let json = json!(&user);
        println!("{}", json);
        let mut result_vec: Vec<UserFlat> = self.database.aql_bind_vars("
            INSERT @user INTO users
            RETURN NEW
        ", hashmap!{
            "user" => json
        }).await
            .map_err(|e| { eprintln!("{}", e); () })?;
        if result_vec.len() != 1 {
            return Err(());
        }
        result_vec.pop()
            .ok_or(())
    }

    pub async fn update(&self, user: UserFlat) -> Result<UserFlat, ()> {
        let mut result_vec: Vec<UserFlat> = self.database.aql_bind_vars("
            LET doc = DOCUMENT(CONCAT(\"users/\", @user._key))
            UPDATE doc WITH @user IN users
            RETURN NEW
        ", hashmap! {
            "user" => json!(user)
        }).await
            .map_err(|err| {
                error!("AQL Error: {:#?}", err);
                ()
            })?;
        
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
        let pool = req.app_data::<Data<ArangoPool>>().cloned().unwrap();
        async move {
            Ok(
                UserRepo::new(pool).await
            )
        }
    }
}
