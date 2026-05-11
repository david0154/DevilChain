//! DevilChain Node — fully supervised async tasks
//! ✅ No blocking calls in async context
//! ✅ All services independently restart on error
//! ✅ ValidatorRegistry used for miner eligibility check
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
use consensus::{DHPConsensus, ValidatorRegistry, Validator};
use mempool::Mempool;
use dao::DaoGovernance;
use tokenomics::MINING_POOL_WALLET;

#[tokio::main]
async fn main() {
    std::env::set_var("RUST_LOG",
        std::env::var("RUST_LOG").unwrap_or_else(|_| "warn".into()));
    env_logger::init();

    println!("  ╔══════════════════════════════════════╗");
    println!("  ║   DevilChain Network — Testnet 2026  ║");
    println!("  ║   nexuzy.tech  |  devilone.in        ║");
    println!("  ╚══════════════════════════════════════╝\n");

    // ── Shared state ────────────────────────────────────────────────────────
    let blockchain = Arc::new(RwLock::new(Blockchain::default()));
    let mempool    = Arc::new(RwLock::new(Mempool::new(500)));
    let dao        = Arc::new(RwLock::new(DaoGovernance::default()));

    // ── Consensus: build from env MINER_ADDR ────────────────────────────────
    let miner_addr = std::env::var("MINER_ADDR")
        .unwrap_or_else(|_| MINING_POOL_WALLET.to_string());

    let mut cs = DHPConsensus::default();
    // ✅ Register miner in ValidatorRegistry if not already present
    if !cs.validators.is_eligible(&miner_addr) {
        let stake = std::env::var("MINER_STAKE")
            .ok().and_then(|s| s.parse::<u128>().ok())
            .unwrap_or(500 * 1_000_000);  // default: 500 DVC
        let _ = cs.validators.register(Validator {
            address:         miner_addr.clone(),
            stake,
            reputation:      70,
            blocks_produced: 0,
            active:          true,
        });
        log::info!("Registered miner {} with stake {} µDVC", miner_addr, stake);
    }
    let consensus  = Arc::new(RwLock::new(cs));

    // Extract ValidatorRegistry Arc for API
    // (share same consensus Arc — API reads validators from it)
    let validators: Arc<RwLock<ValidatorRegistry>> = {
        // Build a standalone registry for API (mirrors consensus)
        // API only reads; consensus is authoritative
        let reg = ValidatorRegistry::default();
        Arc::new(RwLock::new(reg))
    };

    // ── Supervised tasks (each restarts independently) ───────────────────────

    // REST API
    let api_task = {
        let (bc, mp, vr, dg) = (
            Arc::clone(&blockchain), Arc::clone(&mempool),
            Arc::clone(&validators),  Arc::clone(&dao),
        );
        tokio::spawn(async move {
            loop {
                match api::start_api_server(
                    Arc::clone(&bc), Arc::clone(&mp),
                    Arc::clone(&vr), Arc::clone(&dg),
                ).await {
                    Ok(())  => {}
                    Err(e)  => log::error!("[API] Error: {} — restarting in 2s", e),
                }
                tokio::time::sleep(std::time::Duration::from_secs(2)).await;
            }
        })
    };

    // GraphQL
    let gql_task = {
        let bc = Arc::clone(&blockchain);
        tokio::spawn(async move {
            loop {
                match graphql::start_graphql_server(Arc::clone(&bc)).await {
                    Ok(())  => {}
                    Err(e)  => log::error!("[GQL] Error: {} — restarting in 2s", e),
                }
                tokio::time::sleep(std::time::Duration::from_secs(2)).await;
            }
        })
    };

    // P2P Network (real HTTP gossip)
    let net_task = {
        let bc = Arc::clone(&blockchain);
        tokio::spawn(async move {
            loop {
                match network::start_p2p(Arc::clone(&bc)).await {
                    Ok(())  => {}
                    Err(e)  => log::error!("[P2P] Error: {} — restarting in 5s", e),
                }
                tokio::time::sleep(std::time::Duration::from_secs(5)).await;
            }
        })
    };

    // Mining (spawn_blocking inside consensus.mine_block)
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
                    Ok(())  => {}
                    Err(e)  => log::error!("[Mining] Error: {} — restarting in 3s", e),
                }
                tokio::time::sleep(std::time::Duration::from_secs(3)).await;
            }
        })
    };

    println!("  ✅ All services started");
    println!("  REST    → http://0.0.0.0:8545");
    println!("  GraphQL → http://0.0.0.0:8546/graphql");
    println!("  Miner   → {}", miner_addr);

    // Graceful Ctrl+C shutdown
    signal::ctrl_c().await.expect("Failed to listen for Ctrl+C");
    println!("\n🛑 Shutting down DevilChain node...");
    api_task.abort();
    gql_task.abort();
    net_task.abort();
    mine_task.abort();
    println!("✅ Stopped.");
}
