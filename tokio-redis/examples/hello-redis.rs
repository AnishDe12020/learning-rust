use mini_redis::{client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let mut client = client::connect("127.0.0.1:6379").await?;

    client.set("gm", "wagmi".into()).await?;

    let result = client.get("gm").await?;

    println!("Got from server for gm: {:?}", result);

    Ok(())
}
