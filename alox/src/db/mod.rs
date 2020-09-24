pub mod error;

use error::Error;

use arangors::Connection;
use bb8::{
    Pool,
    ManageConnection,
    PooledConnection
};

pub type ArangoPool = Pool<ArangoConnectionManager>;

pub type ArangoConnection<'c> = PooledConnection<'c, ArangoConnectionManager>;

pub struct ArangoConnectionManager {
    username: String,
    password: String,
    url: String
}

impl ArangoConnectionManager {
    pub fn new<S: Into<String>>(
        url: S,
        username: S,
        password: S
    ) -> Self {
        Self {
            url: url.into(),
            username: username.into(),
            password: password.into()
        }
    }
}

#[async_trait::async_trait]
impl ManageConnection for ArangoConnectionManager {
    type Connection = Connection;
    type Error = Error;

    async fn connect(&self) -> Result<Self::Connection, Self::Error> {
        Connection::establish_basic_auth(&self.url, &self.username, &self.password)
            .await
            .map_err(|_| Error::CouldntConnect)
    }

    async fn is_valid(&self, conn: Self::Connection) -> Result<Self::Connection, Self::Error> {
        Ok(conn)
    }

    fn has_broken(&self, _conn: &mut Self::Connection) -> bool {
        false
    }
}

pub async fn get_connection_pool<S: Into<String>>(
    url: S,
    username: S,
    password: S,
    size: u32
) -> Result<ArangoPool, Error> {
    let manager = ArangoConnectionManager::new(url, username, password);
    Pool::builder()
        .max_size(size)
        .build(manager)
        .await
}