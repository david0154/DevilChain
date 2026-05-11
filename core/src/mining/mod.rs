//! DevilMine Engine — DVLHash-AI Algorithm
//! CPU-optimized, Anti-ASIC, Dynamic difficulty

use crate::blockchain::{Block, Transaction, Blockchain, now_timestamp};
use crate::consensus::DHPConsensus;
use std::sync::Arc;
use tokio::sync::RwLock;
use log::info;

pub async fn start_mining(
    blockchain: Arc<RwLock<Blockchain>>,
    mempool: Arc<RwLock<crate::mempool::Mempool>>,
) {
    let consensus = DHPConsensus::new();
    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;

        let mut mp = mempool.write().await;
        let txs = mp.get_transactions(100);
        drop(mp);

        let bc = blockchain.read().await;
        let prev = bc.latest_block().clone();
        drop(bc);

        let merkle = Block::compute_merkle_root(&txs);
        let new_block = Block {
            block_height: prev.block_height + 1,
            timestamp: now_timestamp(),
            previous_hash: prev.block_hash.clone(),
            validator: "node_self".to_string(),
            transactions: txs,
            merkle_root: merkle,
            nonce: 0,
            ai_score: 0.98,
            dao_signature: "auto_dao".to_string(),
            block_hash: String::new(),
        };

        let mined = consensus.mine_block(new_block);
        info!("⛏️ Mined Block #{} | Hash: {}", mined.block_height, mined.block_hash);

        let mut bc = blockchain.write().await;
        bc.add_block(mined);
    }
