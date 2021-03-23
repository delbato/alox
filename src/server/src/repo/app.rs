use crate::{
    db::{
        ArangoConnectionManager,
        ArangoConnection,
        ArangoPool
    },
    model::{
        app::{
            App
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

pub struct AppRepo {
    database: Database<ReqwestClient>
}

impl AppRepo {
    pub async fn new(pool: Data<ArangoPool>) -> Self {
        let conn = pool.get().await.unwrap();
        Self {
            database: conn.db("alox").await.unwrap()
        }
    }

    pub async fn insert(&self, app: App) -> Result<App, ()> {
        Err(())
    }

    pub async fn find(&self, app_key: &str, full: bool) -> Result<App, ()> {
        Err(())
    }
}

impl FromRequest for AppRepo {
    type Error = ();
    type Config = ();
    type Future = impl Future<Output = Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let pool = req.app_data::<Data<ArangoPool>>().cloned().unwrap();
        async move {
            Ok(
                AppRepo::new(pool).await
            )
        }
    }
}
