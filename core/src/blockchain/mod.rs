//! DevilChain Blockchain Core
//! Fixed: u128 for amounts, real Merkle tree, sig verification, hash validation
//!
//! Developed by Nexuzy Lab (nexuzy.tech) | Powered by Devil One (devilone.in)

use sha2::{Sha256, Digest};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use crate::tokenomics::{
    split_fee, DEV_WALLET, BURN_ADDRESS, LIQUIDITY_LOCK_VAULT,
    SupplyTracker, block_reward_at,
};

// ── Types (NO f64 for money) ──────────────────────────────────────────────────
/// All amounts in micro-DVC (u128). 1 DVC = 1_000_000
pub type Amount = u128;

// ── Transaction ───────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub tx_hash:   String,
    pub from:      String,
    pub to:        String,
    pub amount:    Amount,    // micro-DVC
    pub gas_fee:   Amount,    // micro-DVC
    pub nonce:     u64,       // replay protection
    pub data:      Option<String>,
    pub signature: String,    // hex(Ed25519 sig)
    pub public_key: String,   // hex(Ed25519 pubkey) for verification
    pub timestamp: u64,
}

impl Transaction {
    pub fn compute_hash(&self) -> String {
        let payload = format!(
            "{}:{}:{}:{}:{}:{}",
            self.from, self.to, self.amount, self.gas_fee,
            self.nonce, self.timestamp
        );
        let hash = Sha256::digest(payload.as_bytes());
        hex::encode(hash)
    }

    /// Verify Ed25519 signature using embedded public_key
    pub fn verify_signature(&self) -> bool {
        use ed25519_dalek::{VerifyingKey, Signature, Verifier};
        let pk_bytes = match hex::decode(&self.public_key) {
            Ok(b) if b.len() == 32 => b,
            _ => return false,
        };
        let sig_bytes = match hex::decode(&self.signature) {
            Ok(b) if b.len() == 64 => b,
            _ => return false,
        };
        let Ok(vk)  = VerifyingKey::from_bytes(pk_bytes.as_slice().try_into().unwrap_or(&[0u8;32])) else { return false; };
        let Ok(sig) = Signature::from_bytes(sig_bytes.as_slice().try_into().unwrap_or(&[0u8;64])) else { return false; };
        let msg = self.compute_hash();
        vk.verify(msg.as_bytes(), &sig).is_ok()
    }

    /// Derive expected sender address from embedded pubkey
    pub fn sender_address(&self) -> String {
        let pk_bytes = hex::decode(&self.public_key).unwrap_or_default();
        let hash = Sha256::digest(&pk_bytes);
        format!("db1x{}", &hex::encode(hash)[..32])
    }

    /// Full validity: sig + address match + positive amounts
    pub fn is_valid(&self) -> bool {
        if self.amount == 0 { return false; }
        if self.gas_fee < crate::tokenomics::MIN_GAS_FEE { return false; }
        if self.from == BURN_ADDRESS { return false; }
        if !self.verify_signature() { return false; }
        if self.sender_address() != self.from { return false; }
        true
    }
}

// ── Block ─────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub height:        u64,
    pub timestamp:     u64,
    pub previous_hash: String,
    pub block_hash:    String,
    pub merkle_root:   String,
    pub transactions:  Vec<Transaction>,
    pub validator:     String,
    pub nonce:         u64,
    pub difficulty:    u32,
    pub block_reward:  Amount,
    pub total_fees:    Amount,
    pub ai_score:      u32,    // 0–100, set by real AI scanner
    pub dao_signature: String, // real DAO threshold sig
}

impl Block {
    pub fn compute_hash(&self) -> String {
        let payload = format!(
            "{}:{}:{}:{}:{}:{}",
            self.height, self.timestamp, self.previous_hash,
            self.merkle_root, self.nonce, self.validator
        );
        let hash = Sha256::digest(payload.as_bytes());
        hex::encode(hash)
    }

    /// Real binary Merkle tree
    pub fn compute_merkle_root(txs: &[Transaction]) -> String {
        if txs.is_empty() {
            return hex::encode(Sha256::digest(b"empty"));
        }
        let mut layer: Vec<String> = txs.iter()
            .map(|tx| tx.compute_hash())
            .collect();
        while layer.len() > 1 {
            if layer.len() % 2 != 0 {
                layer.push(layer.last().unwrap().clone()); // duplicate last
            }
            layer = layer.chunks(2)
                .map(|pair| {
                    let combined = format!("{}{}", pair[0], pair[1]);
                    hex::encode(Sha256::digest(combined.as_bytes()))
                })
                .collect();
        }
        layer.into_iter().next().unwrap_or_default()
    }

    /// Verify block_hash == recomputed hash
    pub fn verify_hash(&self) -> bool {
        self.block_hash == self.compute_hash()
    }

    /// Hash meets difficulty (leading zero bits)
    pub fn meets_difficulty(&self) -> bool {
        let leading_zeros = self.difficulty / 4; // hex chars
        self.block_hash.starts_with(&"0".repeat(leading_zeros as usize))
    }
}

// ── Balance Ledger ────────────────────────────────────────────────────────────

#[derive(Debug, Default)]
pub struct Ledger {
    balances: HashMap<String, Amount>,
    nonces:   HashMap<String, u64>,
}

impl Ledger {
    pub fn balance(&self, addr: &str) -> Amount {
        *self.balances.get(addr).unwrap_or(&0)
    }

    pub fn nonce(&self, addr: &str) -> u64 {
        *self.nonces.get(addr).unwrap_or(&0)
    }

    pub fn credit(&mut self, addr: &str, amount: Amount) {
        *self.balances.entry(addr.to_string()).or_insert(0) += amount;
    }

    pub fn debit(&mut self, addr: &str, amount: Amount) -> bool {
        let bal = self.balances.entry(addr.to_string()).or_insert(0);
        if *bal < amount { return false; }
        *bal -= amount;
        true
    }

    pub fn increment_nonce(&mut self, addr: &str) {
        *self.nonces.entry(addr.to_string()).or_insert(0) += 1;
    }
}

// ── Blockchain ────────────────────────────────────────────────────────────────

pub struct Blockchain {
    pub chain:   Vec<Block>,
    pub ledger:  Ledger,
    pub supply:  SupplyTracker,
}

impl Default for Blockchain {
    fn default() -> Self {
        let mut bc = Self {
            chain:  Vec::new(),
            ledger: Ledger::default(),
            supply: SupplyTracker::default(),
        };
        bc.genesis();
        bc
    }
}

impl Blockchain {
    fn genesis(&mut self) {
        let genesis = Block {
            height:        0,
            timestamp:     1_700_000_000,
            previous_hash: "0".repeat(64),
            block_hash:    String::new(),
            merkle_root:   Block::compute_merkle_root(&[]),
            transactions:  vec![],
            validator:     "genesis".to_string(),
            nonce:         0,
            difficulty:    4,
            block_reward:  0,
            total_fees:    0,
            ai_score:      100,
            dao_signature: "genesis_dao".to_string(),
        };
        let mut g = genesis;
        g.block_hash = g.compute_hash();
        // Pre-fund genesis allocations
        self.ledger.credit(DEV_WALLET, 100_000_000 * crate::tokenomics::DECIMALS as u128);
        self.ledger.credit("db1xecosystem", 200_000_000 * crate::tokenomics::DECIMALS as u128);
        self.ledger.credit("db1xdao_treasury", 150_000_000 * crate::tokenomics::DECIMALS as u128);
        self.ledger.credit(MINING_POOL_WALLET, 100_000_000 * crate::tokenomics::DECIMALS as u128);
        self.ledger.credit("db1xinvestors", 50_000_000 * crate::tokenomics::DECIMALS as u128);
        self.ledger.credit("db1xcommunity", 50_000_000 * crate::tokenomics::DECIMALS as u128);
        self.supply.total_minted = 650_000_000 * crate::tokenomics::DECIMALS as u128;
        self.chain.push(g);
    }

    pub fn latest_block(&self) -> Option<&Block> { self.chain.last() }
    pub fn height(&self) -> u64 { self.chain.len() as u64 }

    /// Full chain validation: hash integrity + tx signatures + merkle roots
    pub fn is_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let prev = &self.chain[i - 1];
            let curr = &self.chain[i];
            // 1. block_hash must match recomputed hash
            if !curr.verify_hash() { return false; }
            // 2. previous_hash linkage
            if curr.previous_hash != prev.block_hash { return false; }
            // 3. merkle root matches transactions
            if curr.merkle_root != Block::compute_merkle_root(&curr.transactions) {
                return false;
            }
            // 4. every transaction signature is valid
            for tx in &curr.transactions {
                if !tx.is_valid() { return false; }
            }
        }
        true
    }

    /// Add a validated block and apply fee distribution
    pub fn add_block(&mut self, mut block: Block) -> Result<(), &'static str> {
        // Verify hash
        if !block.verify_hash() { return Err("Invalid block hash"); }
        // Verify previous hash
        let prev_hash = self.chain.last()
            .map(|b| b.block_hash.clone())
            .unwrap_or_else(|| "0".repeat(64));
        if block.previous_hash != prev_hash { return Err("Previous hash mismatch"); }
        // Verify merkle root
        if block.merkle_root != Block::compute_merkle_root(&block.transactions) {
            return Err("Merkle root mismatch");
        }

        let mut total_fees: Amount = 0;

        // Apply transactions
        for tx in &block.transactions {
            if !tx.is_valid() { return Err("Invalid transaction signature"); }
            let total_cost = tx.amount + tx.gas_fee;
            // Replay protection: nonce must be exactly next
            let expected_nonce = self.ledger.nonce(&tx.from);
            if tx.nonce != expected_nonce { return Err("Invalid nonce"); }
            if !self.ledger.debit(&tx.from, total_cost) { return Err("Insufficient balance"); }
            self.ledger.credit(&tx.to, tx.amount);
            self.ledger.increment_nonce(&tx.from);
            total_fees += tx.gas_fee;
        }

        block.total_fees = total_fees;

        // Fee distribution: 60% miner, 20% dev, 10% burn, 10% liquidity
        let split = split_fee(total_fees);
        self.ledger.credit(&block.validator, split.miner);
        self.ledger.credit(DEV_WALLET, split.dev);
        // burn: credit to burn address (unspendable)
        self.ledger.credit(BURN_ADDRESS, split.burn);
        self.supply.burn(split.burn);
        self.ledger.credit(LIQUIDITY_LOCK_VAULT, split.liquidity);

        // Block reward (from emission)
        let reward = block_reward_at(block.height);
        if self.supply.can_mint(reward) {
            let _ = self.supply.mint(reward);
            self.ledger.credit(&block.validator, reward);
            block.block_reward = reward;
        }

        self.chain.push(block);
        Ok(())
    }
}

use crate::tokenomics::MINING_POOL_WALLET;
