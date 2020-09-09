pub mod error;

use error::{
    Result,
    Error
};

use arangors::{
    Connection,
    Database,
    Collection,
    client::{
        reqwest::ReqwestClient
    }
};

pub struct ArangoConnection {
    handle: Connection
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
            handle
        })
    }

    pub async fn get_db(&self, db_name: &str) -> Result<Database<ReqwestClient>> {
        self.handle.db(db_name).await
            .map_err(|_| Error::CouldntGetDatabase)
    }
}