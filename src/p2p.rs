use crate::orderbook::Book;
use anyhow::Result;

pub async fn start_network(_listen: &str, _book: &mut Book) -> Result<()> {
        println!("[p2p] gossip — integrate libp2p");
    tokio::signal::ctrl_c().await?;
    Ok(())
}
