use crate::{
    db::{
        ArangoConnectionManager,
        ArangoConnection,
        ArangoPool
    },
    model::{
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


pub struct PageRepo {
    database: Database<ReqwestClient>
}

impl PageRepo {
    pub async fn new(pool: Data<ArangoPool>) -> Self {
        let connection = pool.get().await.unwrap();
        let database = connection.db("alox").await.expect("Database \"alox\" does not exist!");
        Self {
            database
        }
    }
}

impl FromRequest for PageRepo {
    type Error = ();
    type Config = ();
    type Future = impl Future<Output = Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let pool = req.app_data::<Data<ArangoPool>>().cloned().unwrap();
        async move {
            Ok(
                PageRepo::new(pool).await
            )
        }
    }
}
