use crate::{
    db::{
        ArangoConnectionManager,
        ArangoConnection,
        ArangoPool
    },
    model::{
        site::{
            SiteFlat
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


pub struct SiteRepo {
    database: Database<ReqwestClient>
}

impl SiteRepo {
    pub async fn new(pool: Data<ArangoPool>) -> Self {
        let connection = pool.get().await.unwrap();
        let database = connection.db("alox").await.expect("Database \"alox\" does not exist!");
        Self {
            database
        }
    }

    pub async fn insert(&self, site: SiteFlat) -> Result<(), ()> {
        
        Err(())
    }
}

impl FromRequest for SiteRepo {
    type Error = ();
    type Config = ();
    type Future = impl Future<Output = Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let pool = req.app_data::<Data<ArangoPool>>().cloned().unwrap();
        async move {
            Ok(
                SiteRepo::new(pool).await
            )
        }
    }
}
