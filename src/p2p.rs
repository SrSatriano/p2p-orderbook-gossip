use crate::orderbook::Book;
use anyhow::Result;

pub async fn start_network(_listen: &str, _book: &mut Book) -> Result<()> {
    // TODO: libp2p Swarm + GossipSub topic "orders/v1"
    println!("[p2p] gossip scaffold — integrate libp2p");
    tokio::signal::ctrl_c().await?;
    Ok(())
}
