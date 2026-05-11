mod blockchain;
mod consensus;
mod mempool;
mod network;
mod wallet;
mod api;
mod ai;
mod dao;
mod mining;

use std::sync::Arc;
use tokio::sync::RwLock;
use log::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();
    info!("🔥 DevilChain Network Node Starting...");
    info!("Native Coin: DevilCoin (DVC) | Symbol: DVL");
    info!("Consensus: Devil Hybrid Protocol (DHP)");

    let blockchain = Arc::new(RwLock::new(blockchain::Blockchain::new()));
    let mempool = Arc::new(RwLock::new(mempool::Mempool::new()));

    info!("Blockchain initialized. Address prefix: db1x");

    // Start network, API, mining loops
    let bc = Arc::clone(&blockchain);
    tokio::select! {
        _ = api::start_api_server(bc.clone()) => {},
        _ = network::start_p2p(bc.clone()) => {},
        _ = mining::start_mining(bc.clone(), mempool.clone()) => {},
    }

    Ok(())
}
