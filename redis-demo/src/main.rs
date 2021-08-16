use redis::{Client, aio::Connection};
use redis::AsyncCommands;


async fn get_connect(path: &str) -> Result<Connection, Box<dyn std::error::Error>> {
    let client = Client::open(path)?;
    let conn = client.get_tokio_connection().await?;
    Ok(conn)
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut conn = get_connect("redis://127.0.0.1:6379").await?;
    // cmd ping
    let pong: String = redis::cmd("PING").query_async(&mut conn).await?;
    println!("{}", pong);

    // set get del
    conn.set("hello", "world").await?;
    let res: String = match conn.get("hello").await {
        Ok(res) => res,
        Err(_) => {
            println!("key is nil");
            "".to_string()
        },
    };
    if res != "".to_string() {
        println!("{}", res);
        conn.del("hello").await?;
    }

    Ok(())
}
