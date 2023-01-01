use surrealdb_rs::net::WsClient;
use surrealdb_rs::param::Root;
use surrealdb_rs::protocol::Ws;
use surrealdb_rs::{Result, Surreal};
pub async fn root_client(namespace: &str, database: &str) -> Result<Surreal<WsClient>> {
    let client = Surreal::connect::<Ws>("localhost:8000").await?;
    let username = "root";
    let password = "root";
    client.signin(Root { username, password }).await?;
    client.use_ns(namespace).use_db(database).await?;
    Ok(client)
}
