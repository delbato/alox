use crate::{
    db::{
        ArangoConnectionManager,
        ArangoConnection,
        ArangoPool
    },
    model::{
        app::{
            App,
            AppFlat,
            AppFull
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

    pub async fn find(&self, app_key: &str, full: bool) -> Result<App, ()> {
        let query = if full {
            "
            LET app = DOCUMENT(CONCAT(\"apps/\", @app_key))
            LET proxies = (
                FOR proxy in proxies
                    FILTER proxy._key_app = @app_key
                    RETURN proxy
            )
            LET cms = (
                FOR cms in cms_configs
                    FILTER cms._key_app = @app_key
                    RETURN cms
            )
            app.type = \"full\"
            RETURN MERGE(a, { proxies: proxies, cms: cms })
            "
        } else {
            "
            LET app = DOCUMENT(CONCAT(\"apps/\", @app_key))
            app.type = \"flat\"
            RETURN app
            "
        };
        let mut result_vec: Vec<App> = self.database.aql_bind_vars(query, hashmap!{ "app_key" => json!(app_key) }).await
            .map_err(|_| ())?;
        if result_vec.len() != 1 {
            return Err(());
        }
        Ok(
            result_vec.remove(0)
        )
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
