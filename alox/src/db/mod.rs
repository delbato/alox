pub mod error;

use error::{
    Result,
    Error
};

use std::{
    sync::{
        Arc,
        RwLock
    }
};

use arangors::{
    Connection,
    Database,
    Collection,
    client::{
        reqwest::ReqwestClient
    }
};

#[derive(Clone)]
pub struct ArangoConnection {
    handle: Arc<RwLock<Connection>>
}

impl ArangoConnection {
    pub async fn new(url: &str, username: &str, password: &str) -> Result<Self> {
        let handle = Connection::establish_jwt(url, username, password)
            .await
            .map_err(|e| {
                println!("Error! {:#?}", e);
                Error::CouldntConnect
            })?;
        Ok(Self{
            handle: Arc::new(RwLock::new(handle))
        })
    }

    pub async fn get_db(&self, db_name: &str) -> Result<Database<ReqwestClient>> {
        let handle = self.handle.read()
            .map_err(|_| Error::Unknown)?;
        handle.db(db_name).await
            .map_err(|_| Error::CouldntGetDatabase)
    }
}