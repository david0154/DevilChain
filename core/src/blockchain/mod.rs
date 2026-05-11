use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub block_height: u64,
    pub timestamp: u64,
    pub previous_hash: String,
    pub validator: String,
    pub transactions: Vec<Transaction>,
    pub merkle_root: String,
    pub nonce: u64,
    pub ai_score: f64,
    pub dao_signature: String,
    pub block_hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub tx_hash: String,
    pub from: String,
    pub to: String,
    pub amount: f64,
    pub gas_fee: f64,
    pub timestamp: u64,
    pub signature: String,
}

pub struct Blockchain {
    pub chain: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Self {
        let genesis = Block::genesis();
        Blockchain { chain: vec![genesis] }
    }

    pub fn latest_block(&self) -> &Block {
        self.chain.last().unwrap()
    }

    pub fn add_block(&mut self, mut block: Block) -> bool {
        let prev = self.latest_block().clone();
        if block.previous_hash != prev.block_hash {
            return false;
        }
        block.block_hash = block.compute_hash();
        self.chain.push(block);
        true
    }

    pub fn is_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let curr = &self.chain[i];
            let prev = &self.chain[i - 1];
            if curr.previous_hash != prev.block_hash {
                return false;
            }
        }
        true
    }
}

impl Block {
    pub fn genesis() -> Self {
        let mut b = Block {
            block_height: 0,
            timestamp: 1777000000,
            previous_hash: "0000000000000000".to_string(),
            validator: "genesis".to_string(),
            transactions: vec![],
            merkle_root: "0x0".to_string(),
            nonce: 0,
            ai_score: 1.0,
            dao_signature: "genesis_dao".to_string(),
            block_hash: String::new(),
        };
        b.block_hash = b.compute_hash();
        b
    }

    pub fn compute_hash(&self) -> String {
        let data = format!(
            "{}{}{}{}{}",
            self.block_height, self.timestamp,
            self.previous_hash, self.nonce, self.merkle_root
        );
        let hash = Sha256::digest(data.as_bytes());
        format!("0x{:x}", hash)
    }

    pub fn compute_merkle_root(txs: &[Transaction]) -> String {
        if txs.is_empty() {
            return "0x0".to_string();
        }
        let hashes: Vec<String> = txs.iter().map(|tx| tx.tx_hash.clone()).collect();
        let combined = hashes.join("");
        let hash = Sha256::digest(combined.as_bytes());
        format!("0x{:x}", hash)
    }
}

impl Transaction {
    pub fn compute_hash(&self) -> String {
        let data = format!("{}{}{}{}", self.from, self.to, self.amount, self.timestamp);
        let hash = Sha256::digest(data.as_bytes());
        format!("0x{:x}", hash)
    }
}

pub fn now_timestamp() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
}
