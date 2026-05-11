//! Devil Hybrid Protocol (DHP) Consensus
//! Components: PoS + Micro PoW + DAO Validation + AI Optimization

use crate::blockchain::{Block, Transaction};

pub struct DHPConsensus {
    pub difficulty: u32,
    pub min_stake: f64,
}

impl DHPConsensus {
    pub fn new() -> Self {
        DHPConsensus {
            difficulty: 4,
            min_stake: 100.0,
        }
    }

    /// Micro PoW: check nonce satisfies difficulty
    pub fn verify_pow(&self, block: &Block) -> bool {
        let hash = &block.block_hash;
        let prefix = "0".repeat(self.difficulty as usize);
        hash.trim_start_matches("0x").starts_with(&prefix)
    }

    /// AI score must exceed threshold for block acceptance
    pub fn verify_ai_score(&self, block: &Block) -> bool {
        block.ai_score >= 0.75
    }

    /// DAO signature must be present
    pub fn verify_dao_sig(&self, block: &Block) -> bool {
        !block.dao_signature.is_empty() && block.dao_signature != "none"
    }

    /// Full DHP block validation
    pub fn validate_block(&self, block: &Block) -> bool {
        self.verify_pow(block)
            && self.verify_ai_score(block)
            && self.verify_dao_sig(block)
    }

    /// Mine: find nonce that satisfies micro PoW
    pub fn mine_block(&self, mut block: Block) -> Block {
        loop {
            let hash = block.compute_hash();
            let stripped = hash.trim_start_matches("0x");
            let prefix = "0".repeat(self.difficulty as usize);
            if stripped.starts_with(&prefix) {
                block.block_hash = hash;
                return block;
            }
            block.nonce += 1;
        }
    }
}
