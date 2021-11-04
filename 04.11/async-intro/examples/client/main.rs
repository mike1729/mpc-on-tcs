use mini_redis::{client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let mut client = client::connect("127.0.0.1:1729").await?;

    let result = client.get("hello").await?;

    println!("Got value from the server; result={:?}", result);

    Ok(())
}
