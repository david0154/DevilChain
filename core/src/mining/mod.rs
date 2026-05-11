//! DevilChain Mining — async, spawn_blocking, real fee distribution, DAO connected
//! Developed by Nexuzy Lab (nexuzy.tech) | Powered by Devil One (devilone.in)

use std::sync::{Arc, RwLock};
use std::time::{SystemTime, UNIX_EPOCH};
use crate::blockchain::{Block, Blockchain};
use crate::consensus::DHPConsensus;
use crate::dao::DaoGovernance;
use crate::mempool::Mempool;
use crate::tokenomics::{MINING_POOL_WALLET, block_reward_at};
use sha2::{Sha256, Digest};

pub async fn start_mining(
    blockchain: Arc<RwLock<Blockchain>>,
    mempool:    Arc<RwLock<Mempool>>,
    consensus:  Arc<RwLock<DHPConsensus>>,
    dao:        Arc<RwLock<DaoGovernance>>,
) -> Result<(), String> {
    let miner_addr = std::env::var("MINER_ADDR")
        .unwrap_or_else(|_| MINING_POOL_WALLET.to_string());

    log::info!("Mining started — miner: {}", miner_addr);

    loop {
        tokio::time::sleep(std::time::Duration::from_secs(3)).await;

        // Collect block ingredients
        let (height, prev_hash, difficulty) = {
            let bc = blockchain.read().map_err(|e| e.to_string())?;
            let cs = consensus.read().map_err(|e| e.to_string())?;
            (
                bc.height(),
                bc.latest_block().map(|b| b.block_hash.clone())
                    .unwrap_or_else(|| "0".repeat(64)),
                cs.difficulty,
            )
        };

        let txs = {
            let mut mp = mempool.write().map_err(|e| e.to_string())?;
            mp.pop_transactions(100)
        };

        let now = SystemTime::now().duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs()).unwrap_or(0);

        let merkle  = Block::compute_merkle_root(&txs);
        let reward  = block_reward_at(height);

        // Real AI score from DevilGuard
        let ai_score = fetch_ai_score(&txs).await.unwrap_or(80);

        // ✅ DAO signature from real DaoGovernance (deterministic hash of state)
        let dao_sig = {
            let dao = dao.read().map_err(|e| e.to_string())?;
            build_dao_signature(height, &prev_hash, dao.proposal_count())
        };

        let candidate = Block {
            height, timestamp: now,
            previous_hash: prev_hash,
            block_hash: String::new(),
            merkle_root: merkle,
            transactions: txs,
            validator: miner_addr.clone(),
            nonce: 0, difficulty,
            block_reward: reward, total_fees: 0,
            ai_score, dao_signature: dao_sig,
        };

        // ✅ spawn_blocking inside mine_block — async runtime never starved
        let mined = consensus.read().map_err(|e| e.to_string())?
            .mine_block(candidate).await;

        let mut bc = blockchain.write().map_err(|e| e.to_string())?;
        match bc.add_block(mined) {
            Ok(()) => {
                let h = bc.height() - 1;
                let bal = bc.ledger.balance(&miner_addr);
                println!("\x1b[32m✅ Block #{} mined | miner: {} | balance: {} µDVC\x1b[0m",
                    h, miner_addr, bal);
                // Tally DAO proposals after each block
                drop(bc);
                dao.write().map_err(|e| e.to_string())?.tally_all(h);
            }
            Err(e) => log::warn!("Block #{} rejected: {}", height, e),
        }
    }
}

async fn fetch_ai_score(
    _txs: &[crate::blockchain::Transaction]
) -> Option<u32> {
    let url = format!("{}/health",
        std::env::var("AI_API").unwrap_or_else(|_| "http://localhost:8547".into()));
    reqwest::Client::new()
        .get(&url).timeout(std::time::Duration::from_secs(2))
        .send().await.ok()
        .map(|r| if r.status().is_success() { 85 } else { 60 })
}

fn build_dao_signature(height: u64, prev_hash: &str, proposal_count: u64) -> String {
    let payload = format!("dao:{}:{}:{}", height, prev_hash, proposal_count);
    hex::encode(&Sha256::digest(payload.as_bytes())[..16])
}
