mod blockchain;
mod consensus;
mod mempool;
mod network;
mod wallet;
mod api;
mod ai;
mod dao;
mod mining;
mod validator;
mod storage;
mod graphql;

use std::sync::Arc;
use tokio::sync::RwLock;
use log::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();
    info!("🔥 DevilChain Network Node Starting...");
    info!("Native Coin: DevilCoin (DVC) | Symbol: DVL");
    info!("Consensus:   Devil Hybrid Protocol (DHP)");
    info!("REST API:    http://0.0.0.0:8545");
    info!("GraphQL:     http://0.0.0.0:8546/graphql");
    info!("P2P:         libp2p port 30303");

    // Open persistent RocksDB storage
    let db = Arc::new(storage::ChainDB::open("./data/devilchain")?);

    // Init blockchain (load from DB or genesis)
    let blockchain = Arc::new(RwLock::new(blockchain::Blockchain::new()));
    let mempool    = Arc::new(RwLock::new(mempool::Mempool::new()));
    let validators = Arc::new(RwLock::new(validator::ValidatorSet::new()));

    info!("✅ Blockchain initialized. Address prefix: db1x");

    // Spawn all services concurrently
    tokio::select! {
        _ = api::start_api_server(blockchain.clone(), validators.clone(), db.clone()) => {},
        _ = graphql::start_graphql_server(blockchain.clone()) => {},
        _ = network::start_p2p(blockchain.clone()) => {},
        _ = mining::start_mining(blockchain.clone(), mempool.clone()) => {},
    }

    Ok(())
}
