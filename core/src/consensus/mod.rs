//! DevilChain Consensus — DHP, VRF validator selection
//! ✅ mine_block() uses tokio::task::spawn_blocking — never blocks async runtime
//! Developed by Nexuzy Lab (nexuzy.tech) | Powered by Devil One (devilone.in)

use sha2::{Sha256, Digest};
use std::collections::HashMap;
use crate::blockchain::{Block, Amount};
use crate::tokenomics::MINING_POOL_WALLET;

// ── Validator ─────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct Validator {
    pub address:         String,
    pub stake:           u128,
    pub reputation:      u32,
    pub blocks_produced: u64,
    pub active:          bool,
}

impl Validator {
    /// Pure integer voting power — no f64, no NaN
    pub fn voting_power(&self) -> u128 {
        self.stake.saturating_mul(self.reputation as u128)
    }
}

// ── ValidatorRegistry ─────────────────────────────────────────────────────────

pub struct ValidatorRegistry {
    validators: HashMap<String, Validator>,
}

impl Default for ValidatorRegistry {
    fn default() -> Self {
        let mut r = Self { validators: HashMap::new() };
        let _ = r.register(Validator {
            address:         MINING_POOL_WALLET.to_string(),
            stake:           500 * 1_000_000,
            reputation:      80,
            blocks_produced: 0,
            active:          true,
        });
        r
    }
}

impl ValidatorRegistry {
    pub const MIN_STAKE: u128 = 100 * 1_000_000;

    pub fn register(&mut self, v: Validator) -> Result<(), &'static str> {
        if v.stake < Self::MIN_STAKE { return Err("Insufficient stake"); }
        self.validators.insert(v.address.clone(), v);
        Ok(())
    }

    pub fn is_eligible(&self, addr: &str) -> bool {
        self.validators.get(addr)
            .map(|v| v.active && v.stake >= Self::MIN_STAKE)
            .unwrap_or(false)
    }

    /// VRF-style weighted random using prev block hash as entropy
    pub fn select_validator(&self, entropy: &str) -> Option<String> {
        let eligible: Vec<&Validator> = self.validators.values()
            .filter(|v| v.active && v.stake >= Self::MIN_STAKE).collect();
        if eligible.is_empty() { return None; }
        let total: u128 = eligible.iter()
            .map(|v| v.voting_power())
            .fold(0u128, |a, b| a.saturating_add(b));
        if total == 0 { return None; }
        let seed  = Sha256::digest(entropy.as_bytes());
        let pick_seed = u128::from_be_bytes(seed[..16].try_into().unwrap_or([0u8;16]));
        let mut pick  = pick_seed % total;
        for v in &eligible {
            let vp = v.voting_power();
            if pick < vp { return Some(v.address.clone()); }
            pick -= vp;
        }
        eligible.last().map(|v| v.address.clone())
    }

    pub fn reward_validator(&mut self, addr: &str) {
        if let Some(v) = self.validators.get_mut(addr) {
            v.blocks_produced += 1;
            v.reputation = (v.reputation + 1).min(100);
        }
    }

    pub fn get_all(&self) -> Vec<&Validator> { self.validators.values().collect() }
    pub fn get(&self, addr: &str) -> Option<&Validator> { self.validators.get(addr) }
}

// ── DHPConsensus ──────────────────────────────────────────────────────────────

pub struct DHPConsensus {
    pub difficulty:        u32,
    pub target_block_time: u64,
    pub validators:        ValidatorRegistry,
}

impl Default for DHPConsensus {
    fn default() -> Self {
        Self {
            difficulty:        4,
            target_block_time: 3,
            validators:        ValidatorRegistry::default(),
        }
    }
}

impl DHPConsensus {
    pub fn validate_block(&self, block: &Block, prev_hash: &str) -> bool {
        block.verify_hash()
            && block.previous_hash == prev_hash
            && block.meets_difficulty()
            && block.merkle_root == Block::compute_merkle_root(&block.transactions)
            && block.ai_score >= 50
            && !block.dao_signature.is_empty()
    }

    /// ✅ spawn_blocking: PoW loop runs on a dedicated OS thread,
    ///    never starves the Tokio async runtime.
    pub async fn mine_block(&self, mut block: Block) -> Block {
        let diff = self.difficulty;
        tokio::task::spawn_blocking(move || {
            let prefix = "0".repeat((diff / 4) as usize);
            loop {
                block.block_hash = block.compute_hash();
                if block.block_hash.starts_with(&prefix) { return block; }
                block.nonce = block.nonce.wrapping_add(1);
            }
        })
        .await
        .expect("Mining thread panicked")
    }

    pub fn adjust_difficulty(&mut self, avg_block_time: u64) {
        let t = self.target_block_time;
        if avg_block_time < t.saturating_sub(1) {
            self.difficulty = (self.difficulty + 1).min(32);
        } else if avg_block_time > t + 1 {
            self.difficulty = self.difficulty.saturating_sub(1).max(2);
        }
    }
}
