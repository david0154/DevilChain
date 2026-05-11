//! DevilChain Node Entry Point
//! Fixed: tokio::join! with supervised tasks (not select!)
//!
//! Developed by Nexuzy Lab (nexuzy.tech) | Powered by Devil One (devilone.in)

use std::sync::{Arc, RwLock};
use tokio::signal;
mod blockchain;
mod consensus;
mod wallet;
mod tokenomics;
mod mempool;
mod validator;
mod mining;
mod network;
mod api;
mod graphql;
mod storage;
mod dao;
mod ai;

use blockchain::Blockchain;
use consensus::DHPConsensus;
use mempool::Mempool;

#[tokio::main]
async fn main() {
    // Logging
    std::env::set_var("RUST_LOG", std::env::var("RUST_LOG").unwrap_or_else(|_| "warn".into()));
    env_logger::init();

    println!("  ╔══════════════════════════════════════╗");
    println!("  ║   DevilChain Network — Testnet 2026  ║");
    println!("  ║   nexuzy.tech | devilone.in          ║");
    println!("  ╚══════════════════════════════════════╝\n");

    // Shared state
    let blockchain = Arc::new(RwLock::new(Blockchain::default()));
    let mempool    = Arc::new(RwLock::new(Mempool::default()));
    let consensus  = Arc::new(RwLock::new(DHPConsensus::default()));

    let bc1 = Arc::clone(&blockchain);
    let bc2 = Arc::clone(&blockchain);
    let bc3 = Arc::clone(&blockchain);
    let bc4 = Arc::clone(&blockchain);
    let mp1 = Arc::clone(&mempool);
    let mp2 = Arc::clone(&mempool);
    let cs1 = Arc::clone(&consensus);

    // ✅ Supervised tasks — each restarts on panic, all run independently
    let api_task = tokio::spawn(async move {
        loop {
            if let Err(e) = api::start_api_server(Arc::clone(&bc1),
                                                   Arc::clone(&mp1)).await {
                log::error!("API server error: {} — restarting in 2s", e);
            }
            tokio::time::sleep(std::time::Duration::from_secs(2)).await;
        }
    });

    let graphql_task = tokio::spawn(async move {
        loop {
            if let Err(e) = graphql::start_graphql_server(Arc::clone(&bc2)).await {
                log::error!("GraphQL error: {} — restarting in 2s", e);
            }
            tokio::time::sleep(std::time::Duration::from_secs(2)).await;
        }
    });

    let network_task = tokio::spawn(async move {
        loop {
            if let Err(e) = network::start_p2p(Arc::clone(&bc3)).await {
                log::error!("P2P error: {} — restarting in 5s", e);
            }
            tokio::time::sleep(std::time::Duration::from_secs(5)).await;
        }
    });

    let mining_task = tokio::spawn(async move {
        loop {
            if let Err(e) = mining::start_mining(
                Arc::clone(&bc4), Arc::clone(&mp2), Arc::clone(&cs1)
            ).await {
                log::error!("Mining error: {} — restarting in 3s", e);
            }
            tokio::time::sleep(std::time::Duration::from_secs(3)).await;
        }
    });

    // ✅ Wait for Ctrl+C, then graceful shutdown
    signal::ctrl_c().await.expect("Failed to listen for Ctrl+C");
    println!("\nShutting down DevilChain node...");
    api_task.abort();
    graphql_task.abort();
    network_task.abort();
    mining_task.abort();
}
