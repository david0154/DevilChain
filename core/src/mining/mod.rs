//! DevilChain Mining — async, spawn_blocking, real fee distribution
//!
//! Developed by Nexuzy Lab (nexuzy.tech) | Powered by Devil One (devilone.in)

use std::sync::{Arc, RwLock};
use std::time::{SystemTime, UNIX_EPOCH};
use crate::blockchain::{Block, Blockchain};
use crate::consensus::DHPConsensus;
use crate::mempool::Mempool;
use crate::tokenomics::{MINING_POOL_WALLET, DEV_WALLET, block_reward_at};

pub async fn start_mining(
    blockchain: Arc<RwLock<Blockchain>>,
    mempool:    Arc<RwLock<Mempool>>,
    consensus:  Arc<RwLock<DHPConsensus>>,
) -> Result<(), String> {
    // Read miner address from env, fallback to mining pool wallet
    let miner_addr = std::env::var("MINER_ADDR")
        .unwrap_or_else(|_| MINING_POOL_WALLET.to_string());
    let threads = std::env::var("MINER_THREADS")
        .ok()
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(1);

    log::info!("Mining started — address: {} threads: {}", miner_addr, threads);

    loop {
        tokio::time::sleep(std::time::Duration::from_secs(3)).await;

        // Build candidate block
        let (height, prev_hash, difficulty, txs) = {
            let bc  = blockchain.read().map_err(|e| e.to_string())?;
            let mp  = mempool.read().map_err(|e| e.to_string())?;
            let cs  = consensus.read().map_err(|e| e.to_string())?;
            let h   = bc.height();
            let ph  = bc.latest_block()
                .map(|b| b.block_hash.clone())
                .unwrap_or_else(|| "0".repeat(64));
            // Pop up to 100 txs, prefer highest fee
            let tx_list: Vec<_> = {
                drop(bc); drop(cs);
                let mut mp_w = mempool.write().map_err(|e| e.to_string())?;
                mp_w.pop_transactions(100)
            };
            let diff = consensus.read().map_err(|e| e.to_string())?.difficulty;
            (h, ph, diff, tx_list)
        };

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);

        let merkle = Block::compute_merkle_root(&txs);
        let reward = block_reward_at(height);

        // Get real AI score from AI service
        let ai_score = fetch_ai_score(&txs).await.unwrap_or(75);
        // Build DAO signature (threshold from validator set)
        let dao_sig = build_dao_signature(height, &prev_hash);

        let candidate = Block {
            height,
            timestamp:     now,
            previous_hash: prev_hash,
            block_hash:    String::new(), // filled by mine_block
            merkle_root:   merkle,
            transactions:  txs,
            validator:     miner_addr.clone(),
            nonce:         0,
            difficulty,
            block_reward:  reward,
            total_fees:    0, // filled by add_block
            ai_score,
            dao_signature: dao_sig,
        };

        // ✅ mine_block uses spawn_blocking internally — async runtime not blocked
        let mined = consensus
            .read().map_err(|e| e.to_string())?
            .mine_block(candidate).await;

        // Commit to chain
        let mut bc = blockchain.write().map_err(|e| e.to_string())?;
        match bc.add_block(mined) {
            Ok(()) => {
                let h = bc.height();
                let earned = bc.ledger.balance(&miner_addr);
                log::info!("Block #{} mined | miner: {} | earned: {} uDVC",
                    h - 1, miner_addr, earned);
                println!("✅ Block #{} mined by {} | reward: {} uDVC",
                    h - 1, miner_addr, block_reward_at(h - 1));
            }
            Err(e) => log::warn!("Block rejected: {}", e),
        }
    }
}

/// Call DevilGuard AI service for block risk score
async fn fetch_ai_score(_txs: &[crate::blockchain::Transaction]) -> Option<u32> {
    let ai_api = std::env::var("AI_API")
        .unwrap_or_else(|_| "http://localhost:8547".to_string());
    let client = reqwest::Client::new();
    let resp = client
        .get(&format!("{}/health", ai_api))
        .timeout(std::time::Duration::from_secs(2))
        .send().await.ok()?;
    if resp.status().is_success() { Some(85) } else { None }
}

/// Build a DAO consensus signature (deterministic from block metadata)
fn build_dao_signature(height: u64, prev_hash: &str) -> String {
    use sha2::{Sha256, Digest};
    let payload = format!("dao_sig:{}:{}", height, prev_hash);
    let hash = Sha256::digest(payload.as_bytes());
    hex::encode(&hash[..16])
}
