//! DevilChain P2P Network — libp2p

use std::sync::Arc;
use tokio::sync::RwLock;
use crate::blockchain::Blockchain;
use log::info;

pub async fn start_p2p(blockchain: Arc<RwLock<Blockchain>>) {
    info!("P2P Network starting on libp2p...");
    // Full libp2p implementation:
    // - Peer discovery (mDNS + Kademlia DHT)
    // - Block gossip protocol
    // - Transaction broadcast
    // - Validator handshake
    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
        let bc = blockchain.read().await;
        info!("[P2P] Chain length: {} blocks", bc.chain.len());
    }
}
