use crate::{
    db::{
        ArangoConnectionManager,
        ArangoConnection,
        ArangoPool
    },
    model::{
        perm::{
            PermissionType,
            Permission
        }
    }
};

use std::{
    pin::Pin,
    ops::Deref,
    sync::Arc,
    mem
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
use async_recursion::async_recursion;


pub struct PermissionRepo {
    database: Database<ReqwestClient>
}

impl PermissionRepo {
    pub async fn new(pool: Data<ArangoPool>) -> Self {
        let connection = pool.get().await.unwrap();
        let database = connection.db("alox").await.expect("Database \"alox\" does not exist!");
        Self {
            database
        }
    }

    pub async fn find(&self, perm_key: &str) -> Result<Permission, ()> {
        let query = "
            RETURN DOCUMENT(CONCAT(\"permissions/\", @key))
        ";

        let mut result_vec: Vec<Permission> = self.database.aql_bind_vars(query, hashmap!{
            "key" => perm_key.into()
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

    pub async fn find_by_site(&self, site_key: &str) -> Result<Permission, ()> {
        let query = "
            FOR perm IN permissions
                FILTER perm.key_site = @key_site
                RETURN perm
        ";

        let mut result_vec: Vec<Permission> = self.database.aql_bind_vars(query, hashmap!{
            "key_site" => site_key.into()
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


    pub async fn find_by_user(&self, user_key: &str) -> Result<Permission, ()> {
        let query = "
            FOR perm IN permissions
                FILTER perm.key_user = @key_user
                RETURN perm
        ";

        let mut result_vec: Vec<Permission> = self.database.aql_bind_vars(query, hashmap!{
            "key_user" => user_key.into(),
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

    pub async fn find_by_user_and_site(&self, user_key: &str, site_key: &str) -> Result<Permission, ()> {
        let query = "
            FOR perm IN permissions
                FILTER perm.key_user = @key_user
                FILTER perm.key_site = @key_site
                RETURN perm
        ";

        let mut result_vec: Vec<Permission> = self.database.aql_bind_vars(query, hashmap!{
            "key_user" => user_key.into(),
            "key_site" => site_key.into()
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
}

impl FromRequest for PermissionRepo {
    type Error = ();
    type Config = ();
    type Future = impl Future<Output = Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let pool = req.app_data::<Data<ArangoPool>>().cloned().unwrap();
        async move {
            Ok(
                PermissionRepo::new(pool).await
            )
        }
    }
}
