mod orderbook;
mod p2p;

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let listen = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "/ip4/0.0.0.0/tcp/9000".into());
    println!("P2P orderbook starting on {listen}");
    let mut book = orderbook::Book::new();
    p2p::start_network(&listen, &mut book).await?;
    Ok(())
}
