pub mod error;

use error::{
    Error,
    Result
};

use noria::{
    ControllerHandle,
    ZookeeperAuthority,
    Table,
    View
};
pub struct NoriaConnection {
    handle: ControllerHandle<ZookeeperAuthority>
}

impl NoriaConnection {
    pub async fn new_zk(address: &str) -> Result<Self> {
        println!("Connecting...");
        let mut handle = ControllerHandle::from_zk(address).await
            .map_err(|_| Error::CouldntConnect)?;
        println!("Connected. {:#?}", handle.statistics().await);
        Ok(
            Self {
                handle
            }
        )
    }

    pub async fn install_sql(&mut self, sql: &str) -> Result<()> {
        self.handle.install_recipe(sql).await
            .map_err(|_| Error::Unknown)?;
        Ok(())
    }

    pub async fn extend_sql(&mut self, sql: &str) -> Result<()> {
        self.handle.extend_recipe(sql).await
            .map_err(|_| Error::Unknown)?;
        Ok(())
    }

    pub async fn table(&mut self, table_name: &str) -> Result<Table> {
        self.handle.table(table_name).await
            .map_err(|_| Error::Unknown)
    }

    pub async fn view(&mut self, view_name: &str) -> Result<View> {
        self.handle.view(view_name).await
            .map_err(|_| Error::Unknown)
    }
}