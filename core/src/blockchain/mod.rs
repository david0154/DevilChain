//! DevilChain Blockchain Core — zero f64, real Ed25519, tx_index, tiered fees
//! Developed by Nexuzy Lab (nexuzy.tech) | Powered by Devil One (devilone.in)

use sha2::{Sha256, Digest};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use crate::tokenomics::{
    split_fee, DEV_WALLET, BURN_ADDRESS, LIQUIDITY_LOCK_VAULT, MARKETING_WALLET,
    SupplyTracker, block_reward_at, MIN_GAS_FEE, DECIMALS, TxType, compute_gas,
};

pub type Amount = u128;  // µDVC — 1 DVC = 1_000_000 µDVC. NO f64.

// ── Transaction ───────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub tx_hash:    String,
    pub from:       String,
    pub to:         String,
    pub amount:     Amount,      // µDVC — u128
    pub gas_fee:    Amount,      // µDVC — u128
    pub nonce:      u64,
    pub tx_type:    TxType,      // Standard / DEX / NFT / Contract
    pub data:       Option<String>,
    pub signature:  String,      // hex(Ed25519 64-byte sig)
    pub public_key: String,      // hex(Ed25519 32-byte verifying key)
    pub timestamp:  u64,
}

impl Transaction {
    pub fn compute_hash(&self) -> String {
        let payload = format!(
            "{}:{}:{}:{}:{}:{}",
            self.from, self.to, self.amount, self.gas_fee,
            self.nonce, self.timestamp
        );
        hex::encode(Sha256::digest(payload.as_bytes()))
    }

    pub fn verify_signature(&self) -> bool {
        use ed25519_dalek::{VerifyingKey, Signature, Verifier};
        let pk  = match hex::decode(&self.public_key)  { Ok(b) if b.len()==32 => b, _ => return false };
        let sig = match hex::decode(&self.signature)   { Ok(b) if b.len()==64 => b, _ => return false };
        let arr_pk:  [u8;32] = match pk.try_into()  { Ok(a) => a, _ => return false };
        let arr_sig: [u8;64] = match sig.try_into() { Ok(a) => a, _ => return false };
        let Ok(vk)  = VerifyingKey::from_bytes(&arr_pk)  else { return false };
        let Ok(sg)  = Signature::from_bytes(&arr_sig)    else { return false };
        vk.verify(self.compute_hash().as_bytes(), &sg).is_ok()
    }

    pub fn sender_address(&self) -> String {
        let pk = hex::decode(&self.public_key).unwrap_or_default();
        format!("db1x{}", &hex::encode(Sha256::digest(&pk))[..32])
    }

    /// Validate gas_fee >= expected for tx_type
    pub fn gas_fee_sufficient(&self) -> bool {
        let expected = compute_gas(self.amount, self.tx_type);
        self.gas_fee >= expected
    }

    pub fn is_valid(&self) -> bool {
        self.amount > 0
            && self.gas_fee >= MIN_GAS_FEE
            && self.gas_fee_sufficient()
            && self.from != BURN_ADDRESS
            && self.verify_signature()
            && self.sender_address() == self.from
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
    pub ai_score:      u32,
    pub dao_signature: String,
}

impl Block {
    pub fn compute_hash(&self) -> String {
        let payload = format!(
            "{}:{}:{}:{}:{}:{}",
            self.height, self.timestamp, self.previous_hash,
            self.merkle_root, self.nonce, self.validator
        );
        hex::encode(Sha256::digest(payload.as_bytes()))
    }

    pub fn compute_merkle_root(txs: &[Transaction]) -> String {
        if txs.is_empty() { return hex::encode(Sha256::digest(b"empty")); }
        let mut layer: Vec<String> = txs.iter().map(|tx| tx.compute_hash()).collect();
        while layer.len() > 1 {
            if layer.len() % 2 != 0 { layer.push(layer.last().unwrap().clone()); }
            layer = layer.chunks(2).map(|p| {
                hex::encode(Sha256::digest(format!("{}{}", p[0], p[1]).as_bytes()))
            }).collect();
        }
        layer.into_iter().next().unwrap_or_default()
    }

    pub fn verify_hash(&self)      -> bool { self.block_hash == self.compute_hash() }
    pub fn meets_difficulty(&self) -> bool {
        self.block_hash.starts_with(&"0".repeat((self.difficulty / 4) as usize))
    }
}

// ── Ledger ────────────────────────────────────────────────────────────────────

#[derive(Debug, Default)]
pub struct Ledger {
    balances: HashMap<String, Amount>,
    nonces:   HashMap<String, u64>,
}
impl Ledger {
    pub fn balance(&self, a: &str) -> Amount { *self.balances.get(a).unwrap_or(&0) }
    pub fn nonce(&self,   a: &str) -> u64    { *self.nonces.get(a).unwrap_or(&0)   }
    pub fn credit(&mut self, a: &str, v: Amount) {
        *self.balances.entry(a.to_string()).or_insert(0) += v;
    }
    pub fn debit(&mut self, a: &str, v: Amount) -> bool {
        let b = self.balances.entry(a.to_string()).or_insert(0);
        if *b < v { return false; } *b -= v; true
    }
    pub fn increment_nonce(&mut self, a: &str) {
        *self.nonces.entry(a.to_string()).or_insert(0) += 1;
    }
}

// ── Blockchain ────────────────────────────────────────────────────────────────

pub struct Blockchain {
    pub chain:    Vec<Block>,
    pub ledger:   Ledger,
    pub supply:   SupplyTracker,
    pub tx_index: HashMap<String, (u64, usize)>,  // O(1) tx lookup
}

impl Default for Blockchain {
    fn default() -> Self {
        let mut bc = Self {
            chain: Vec::new(), ledger: Ledger::default(),
            supply: SupplyTracker::default(), tx_index: HashMap::new(),
        };
        bc.genesis();
        bc
    }
}

impl Blockchain {
    fn genesis(&mut self) {
        let mut g = Block {
            height: 0, timestamp: 1_700_000_000,
            previous_hash: "0".repeat(64), block_hash: String::new(),
            merkle_root: Block::compute_merkle_root(&[]),
            transactions: vec![], validator: "genesis".into(),
            nonce: 0, difficulty: 4, block_reward: 0, total_fees: 0,
            ai_score: 100, dao_signature: "genesis_dao".into(),
        };
        g.block_hash = g.compute_hash();
        for (addr, dvc) in &[
            (DEV_WALLET,          100_000_000u128),
            ("db1xecosystem",     200_000_000),
            ("db1xdao_treasury",  150_000_000),
            (crate::tokenomics::MINING_POOL_WALLET, 100_000_000),
            ("db1xinvestors",      50_000_000),
            ("db1xcommunity",      50_000_000),
            (MARKETING_WALLET,    50_000_000),
        ] {
            self.ledger.credit(addr, dvc * DECIMALS as u128);
        }
        self.supply.total_minted = 700_000_000 * DECIMALS as u128;
        self.chain.push(g);
    }

    pub fn latest_block(&self) -> Option<&Block> { self.chain.last() }
    pub fn height(&self) -> u64 { self.chain.len() as u64 }

    pub fn get_transaction(&self, hash: &str) -> Option<&Transaction> {
        let (bh, ti) = self.tx_index.get(hash)?;
        self.chain.get(*bh as usize)?.transactions.get(*ti)
    }

    pub fn is_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let (prev, curr) = (&self.chain[i-1], &self.chain[i]);
            if !curr.verify_hash() { return false; }
            if curr.previous_hash != prev.block_hash { return false; }
            if curr.merkle_root != Block::compute_merkle_root(&curr.transactions) {
                return false;
            }
            for tx in &curr.transactions { if !tx.is_valid() { return false; } }
        }
        true
    }

    pub fn add_block(&mut self, mut block: Block) -> Result<(), &'static str> {
        if !block.verify_hash() { return Err("Invalid block hash"); }
        let prev_hash = self.chain.last().map(|b| b.block_hash.clone())
            .unwrap_or_else(|| "0".repeat(64));
        if block.previous_hash != prev_hash { return Err("Previous hash mismatch"); }
        if block.merkle_root != Block::compute_merkle_root(&block.transactions) {
            return Err("Merkle root mismatch");
        }
        if block.ai_score < 50            { return Err("AI score too low"); }
        if block.dao_signature.is_empty() { return Err("Missing DAO signature"); }

        let mut total_fees: Amount = 0;
        for (ti, tx) in block.transactions.iter().enumerate() {
            if !tx.is_valid() { return Err("Invalid TX"); }
            if tx.nonce != self.ledger.nonce(&tx.from) { return Err("Invalid nonce"); }
            let cost = tx.amount.checked_add(tx.gas_fee).ok_or("Overflow")?;
            if !self.ledger.debit(&tx.from, cost) { return Err("Insufficient balance"); }
            self.ledger.credit(&tx.to, tx.amount);
            self.ledger.increment_nonce(&tx.from);
            total_fees = total_fees.checked_add(tx.gas_fee).ok_or("Fee overflow")?;
            self.tx_index.insert(tx.tx_hash.clone(), (block.height, ti));
        }
        block.total_fees = total_fees;

        // 5-way fee split
        let split = split_fee(total_fees);
        self.ledger.credit(&block.validator, split.miner);
        self.ledger.credit(DEV_WALLET,            split.dev);
        self.ledger.credit(BURN_ADDRESS,           split.burn);
        self.supply.burn(split.burn);
        self.ledger.credit(LIQUIDITY_LOCK_VAULT,   split.liquidity);
        self.ledger.credit(MARKETING_WALLET,       split.marketing);

        // Block reward
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
