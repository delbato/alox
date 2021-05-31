use crate::{
    db::{
        ArangoConnectionManager,
        ArangoConnection,
        ArangoPool
    },
    model::{
        content::{
            Content,
            ContentFlat,
            ContentChildren,
            ContentChildrenFlat
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


pub struct ContentRepo {
    database: Database<ReqwestClient>
}

impl ContentRepo {
    pub async fn new(pool: Data<ArangoPool>) -> Self {
        let connection = pool.get().await.unwrap();
        let database = connection.db("alox").await.expect("Database \"alox\" does not exist!");
        Self {
            database
        }
    }

    #[async_recursion]
    pub async fn load_content_tree(&self, content_key: &str) -> Result<Content, ()> {
        let mut content_flat = self.load_content_flat(content_key).await?;
        let mut children_flat = None;
        mem::swap(&mut children_flat, &mut content_flat.children);
        let mut content = Content::from(content_flat);
        if let Some(children_flat) = children_flat {
            let content_children =  match children_flat {
                ContentChildrenFlat::Array {
                    size,
                    children
                } => {
                    let mut children_ret = vec![];

                    for i in 0..size as usize {
                        let child_key = children.get(i)
                            .ok_or_else(|| ())?;
                        let child_content = self.load_content_tree(child_key).await?;
                        children_ret.push(child_content);
                    }
                    ContentChildren::Array {
                        size,
                        children: children_ret
                    }
                },
                ContentChildrenFlat::List { children } => {
                    let mut children_ret = vec![];
                    for child_key in children {
                        let child_content = self.load_content_tree(&child_key).await?;
                        children_ret.push(child_content);
                    }

                    ContentChildren::List { children: children_ret }
                }
            };

            content.children = Some(content_children);
        }

        Ok(content)
    }

    pub async fn load_content_flat(&self, content_key: &str) -> Result<ContentFlat, ()> {
        let query = "
            LET content = DOCUMENT(CONCAT(\"content/\", @content_key))
            RETURN content
        ";
        let mut result_vec = self.database.aql_bind_vars(query, hashmap!{ "content_key" => json!(content_key) })
            .await
            .map_err(|_| ())?;
        result_vec.pop()
            .ok_or_else(|| ())
    }
}

impl FromRequest for ContentRepo {
    type Error = ();
    type Config = ();
    type Future = impl Future<Output = Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let pool = req.app_data::<Data<ArangoPool>>().cloned().unwrap();
        async move {
            Ok(
                ContentRepo::new(pool).await
            )
        }
    }
}
