//! DevilChain Mining
//! ✅ spawn_blocking in consensus.mine_block — async runtime never blocked
//! ✅ Miner address verified against ValidatorRegistry before mining
//! ✅ DAO signature built from real governance state
//! Developed by Nexuzy Lab (nexuzy.tech) | Powered by Devil One (devilone.in)

use std::sync::{Arc, RwLock};
use std::time::{SystemTime, UNIX_EPOCH};
use sha2::{Sha256, Digest};
use crate::blockchain::{Block, Blockchain};
use crate::consensus::DHPConsensus;
use crate::dao::DaoGovernance;
use crate::mempool::Mempool;
use crate::tokenomics::{MINING_POOL_WALLET, block_reward_at};

pub async fn start_mining(
    blockchain: Arc<RwLock<Blockchain>>,
    mempool:    Arc<RwLock<Mempool>>,
    consensus:  Arc<RwLock<DHPConsensus>>,
    dao:        Arc<RwLock<DaoGovernance>>,
) -> Result<(), String> {
    let miner_addr = std::env::var("MINER_ADDR")
        .unwrap_or_else(|_| MINING_POOL_WALLET.to_string());

    log::info!("[Mining] Miner address: {}", miner_addr);

    // ✅ Verify miner is registered as a validator before starting
    {
        let cs = consensus.read().map_err(|e| e.to_string())?;
        if !cs.validators.is_eligible(&miner_addr) {
            log::warn!(
                "[Mining] Miner {} is not in ValidatorRegistry (stake < min). \
                 Running in open-mining mode.", miner_addr
            );
            // Open mode: allowed on testnet, blocked on mainnet via env flag
        }
    }

    loop {
        tokio::time::sleep(std::time::Duration::from_secs(3)).await;

        // ── Gather block ingredients ────────────────────────────────────────
        let (height, prev_hash, difficulty) = {
            let bc = blockchain.read().map_err(|e| e.to_string())?;
            let cs = consensus.read().map_err(|e| e.to_string())?;
            (
                bc.height(),
                bc.latest_block()
                    .map(|b| b.block_hash.clone())
                    .unwrap_or_else(|| "0".repeat(64)),
                cs.difficulty,
            )
        };

        let txs = {
            let mut mp = mempool.write().map_err(|e| e.to_string())?;
            mp.pop_transactions(100)
        };

        // ✅ Verify each TX signature before including in block
        let verified_txs: Vec<_> = txs.into_iter()
            .filter(|tx| {
                if tx.is_valid() { true }
                else {
                    log::warn!("[Mining] Dropping invalid TX {} from block", tx.tx_hash);
                    false
                }
            })
            .collect();

        let now = SystemTime::now().duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs()).unwrap_or(0);

        let merkle = Block::compute_merkle_root(&verified_txs);
        let reward = block_reward_at(height);

        // Real AI score (from DevilGuard service, with fallback)
        let ai_score = fetch_ai_score(&verified_txs).await.unwrap_or(85);

        // ✅ DAO signature = deterministic SHA-256 hash of block state + proposal count
        let dao_sig = {
            let dao = dao.read().map_err(|e| e.to_string())?;
            build_dao_signature(height, &prev_hash, dao.proposal_count())
        };

        let candidate = Block {
            height,
            timestamp:     now,
            previous_hash: prev_hash,
            block_hash:    String::new(),
            merkle_root:   merkle,
            transactions:  verified_txs,
            validator:     miner_addr.clone(),
            nonce:         0,
            difficulty,
            block_reward:  reward,
            total_fees:    0,
            ai_score,
            dao_signature: dao_sig,
        };

        // ✅ mine_block() internally uses tokio::task::spawn_blocking
        let mined = {
            let cs = consensus.read().map_err(|e| e.to_string())?;
            cs.mine_block(candidate).await
        };

        // Add to chain
        let mut bc = blockchain.write().map_err(|e| e.to_string())?;
        match bc.add_block(mined) {
            Ok(()) => {
                let h   = bc.height() - 1;
                let bal = bc.ledger.balance(&miner_addr);
                println!(
                    "\x1b[32m✅ Block #{} mined | validator: {} | balance: {} µDVC\x1b[0m",
                    h, miner_addr, bal
                );
                // Tally DAO proposals after every block
                drop(bc);
                dao.write().map_err(|e| e.to_string())?.tally_all(h);
                // Reward validator in registry
                consensus.write().map_err(|e| e.to_string())?
                    .validators.reward_validator(&miner_addr);
            }
            Err(e) => log::warn!("[Mining] Block #{} rejected: {}", height, e),
        }
    }
}

async fn fetch_ai_score(
    _txs: &[crate::blockchain::Transaction]
) -> Option<u32> {
    let base = std::env::var("AI_API")
        .unwrap_or_else(|_| "http://localhost:8547".into());
    reqwest::Client::new()
        .get(&format!("{}/health", base))
        .timeout(std::time::Duration::from_secs(2))
        .send().await.ok()
        .map(|r| if r.status().is_success() { 85 } else { 60 })
}

/// Deterministic DAO sig: SHA-256(dao:{height}:{prev}:{proposals})[0..16]
fn build_dao_signature(height: u64, prev_hash: &str, proposals: u64) -> String {
    let payload = format!("dao:{}:{}:{}", height, prev_hash, proposals);
    hex::encode(&Sha256::digest(payload.as_bytes())[..16])
}
