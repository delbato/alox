extern crate tokio;
extern crate alox;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

use alox::{
    db::{
        ArangoConnection
    },
    model::{
        user::{
            User,
            UserWithoutPassword
        }
    },
    serde_json::to_string
};

#[tokio::test]
async fn test_arango_connection() -> Result<()> {
    let _ = ArangoConnection::new("http://localhost:8529", "alox", "alox").await?;
    Ok(())
}

#[tokio::test]
async fn test_arango_get_single_user() -> Result<()> {
    let conn = ArangoConnection::new("http://localhost:8529", "alox", "alox").await?;
    let db = conn.get_db("alox").await?;
    let mut user: Vec<User> = db.aql_str("FOR u in users LIMIT 1 RETURN u").await?;
    println!("{:#?}", user.get(0));
    println!("{:#?}", to_string(&UserWithoutPassword::from(user.remove(0)))?);
    Ok(())
}