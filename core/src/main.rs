//! DevilChain Node — supervised tasks, correct API signature
//! Developed by Nexuzy Lab (nexuzy.tech) | Powered by Devil One (devilone.in)

use std::sync::{Arc, RwLock};
use tokio::signal;

mod blockchain;
mod consensus;
mod wallet;
mod tokenomics;
mod mempool;
mod mining;
mod network;
mod api;
mod graphql;
mod storage;
mod dao;
mod ai;

use blockchain::Blockchain;
use consensus::{DHPConsensus, ValidatorRegistry};
use mempool::Mempool;
use dao::DaoGovernance;

#[tokio::main]
async fn main() {
    std::env::set_var("RUST_LOG",
        std::env::var("RUST_LOG").unwrap_or_else(|_| "warn".into()));
    env_logger::init();

    println!("  ╔══════════════════════════════════════╗");
    println!("  ║   DevilChain Network — Testnet 2026  ║");
    println!("  ║   nexuzy.tech  |  devilone.in        ║");
    println!("  ╚══════════════════════════════════════╝\n");

    // Shared state — all Arc<RwLock<_>>
    let blockchain = Arc::new(RwLock::new(Blockchain::default()));
    let mempool    = Arc::new(RwLock::new(Mempool::new(500)));
    let consensus  = Arc::new(RwLock::new(DHPConsensus::default()));
    let validators = Arc::new(RwLock::new(ValidatorRegistry::default()));
    let dao        = Arc::new(RwLock::new(DaoGovernance::default()));

    // ✅ Supervised tasks — each restarts independently on error
    let api_task = {
        let (bc, mp, vr, dg) = (
            Arc::clone(&blockchain), Arc::clone(&mempool),
            Arc::clone(&validators), Arc::clone(&dao),
        );
        tokio::spawn(async move {
            loop {
                // ✅ Correct 4-arg signature
                match api::start_api_server(
                    Arc::clone(&bc), Arc::clone(&mp),
                    Arc::clone(&vr), Arc::clone(&dg),
                ).await {
                    Ok(()) => {}
                    Err(e) => log::error!("API error: {} — restarting in 2s", e),
                }
                tokio::time::sleep(std::time::Duration::from_secs(2)).await;
            }
        })
    };

    let gql_task = {
        let bc = Arc::clone(&blockchain);
        tokio::spawn(async move {
            loop {
                match graphql::start_graphql_server(Arc::clone(&bc)).await {
                    Ok(()) => {}
                    Err(e) => log::error!("GraphQL error: {} — restarting in 2s", e),
                }
                tokio::time::sleep(std::time::Duration::from_secs(2)).await;
            }
        })
    };

    let net_task = {
        let bc = Arc::clone(&blockchain);
        tokio::spawn(async move {
            loop {
                match network::start_p2p(Arc::clone(&bc)).await {
                    Ok(()) => {}
                    Err(e) => log::error!("P2P error: {} — restarting in 5s", e),
                }
                tokio::time::sleep(std::time::Duration::from_secs(5)).await;
            }
        })
    };

    let mine_task = {
        let (bc, mp, cs, dg) = (
            Arc::clone(&blockchain), Arc::clone(&mempool),
            Arc::clone(&consensus),  Arc::clone(&dao),
        );
        tokio::spawn(async move {
            loop {
                match mining::start_mining(
                    Arc::clone(&bc), Arc::clone(&mp),
                    Arc::clone(&cs), Arc::clone(&dg),
                ).await {
                    Ok(()) => {}
                    Err(e) => log::error!("Mining error: {} — restarting in 3s", e),
                }
                tokio::time::sleep(std::time::Duration::from_secs(3)).await;
            }
        })
    };

    println!("  ✅ All services started");
    println!("  REST  → http://0.0.0.0:8545");
    println!("  GQL   → http://0.0.0.0:8546/graphql");

    signal::ctrl_c().await.expect("Failed to listen for Ctrl+C");
    println!("\nShutting down...");
    api_task.abort(); gql_task.abort();
    net_task.abort(); mine_task.abort();
}
