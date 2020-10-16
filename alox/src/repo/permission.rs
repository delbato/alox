use crate::{
    db::{
        ArangoConnectionManager,
        ArangoConnection,
        ArangoPool
    },
    model::{
        permission::Permission
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

    pub async fn insert(&self, permission: Permission) -> Result<Permission, ()> {
        let mut result_vec: Vec<Permission> = self.database.aql_bind_vars("
            INSERT @permission INTO permissions
            RETURN NEW
        ", hashmap!{
            "permission" => json!(permission)
        }).await
            .map_err(|_| ())?;
        if result_vec.len() != 1 {
            return Err(());
        }
        Ok(
            result_vec.remove(0)
        )
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