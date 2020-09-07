extern crate tokio;
extern crate alox;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

use alox::{
    db::NoriaConnection,
    model::{
        Model,
        user::User
    }
};

#[tokio::test]
async fn test_noria_simple() -> Result<()> {
    let mut connection = NoriaConnection::new_zk("127.0.0.1:2181/alox_test").await?;
    
    connection.install_sql("
        CREATE TABLE alox_users (
            id BIGINT,
            username TEXT,
            email TEXT,
            password TEXT,
            password_salt TEXT,
            is_admin INT,
            PRIMARY KEY(id)
        );
        QUERY alox_users_seq: \
            SELECT MAX(alox_users.id) \
            FROM alox_users;
    ").await?;
    Ok(())
}

#[tokio::test]
async fn test_noria_users() -> Result<()> {
    let mut connection = NoriaConnection::new_zk("127.0.0.1:2181/alox_test").await?;
    let mut users_table = connection.table("alox_users").await?;
    let mut users_id_seq = connection.view("alox_users_seq").await?;
    let mut user = User::new(0);
    user.username = String::from("d.wanner");
    user.email = String::from("d.wanner@pragmatic-apps.de");
    user.password = String::from("123456789");
    user.password_salt = String::from("6789");
    users_table.insert(user.into_row()).await.unwrap();
    let res = users_id_seq.lookup(&[], false).await.unwrap();
    println!("{:#?}", res);
    Ok(())
}

#[tokio::test]
async fn test_noria_delete() -> Result<()> {
    let mut connection = NoriaConnection::new_zk("127.0.0.1:2181/alox_test").await?;
    connection.install_sql("DROP TABLE users;").await?;
    Ok(())
}