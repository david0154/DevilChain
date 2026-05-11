//! DevilChain Tokenomics Engine
//! Supply cap, emission, halving, burn, liquidity lock, fee distribution
//! All amounts in micro-DVC (1 DVC = 1_000_000 u_DVC) — NO f64 for money
//!
//! Developed by Nexuzy Lab (nexuzy.tech) | Powered by Devil One (devilone.in)

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

// ── Constants ────────────────────────────────────────────────────────────────
/// 1 DVC = 1_000_000 micro-DVC (like satoshi)
pub const DECIMALS: u64        = 1_000_000;
pub const MAX_SUPPLY: u128     = 1_000_000_000 * DECIMALS as u128;  // 1 billion DVC
pub const BLOCK_REWARD: u128   = 50 * DECIMALS as u128;             // 50 DVC
pub const HALVING_INTERVAL: u64 = 2_100_000;                         // blocks
pub const MIN_GAS_FEE: u128    = 10_000;                            // 0.01 DVC

// ── Fee Distribution ─────────────────────────────────────────────────────────
/// Of every transaction fee:
///   60% → miner who mined the block
///   20% → developer wallet (Nexuzy Lab)
///   10% → burn (deflation)
///   10% → liquidity lock pool
pub const FEE_MINER_PCT:   u128 = 60;
pub const FEE_DEV_PCT:     u128 = 20;
pub const FEE_BURN_PCT:    u128 = 10;
pub const FEE_LIQUIDITY_PCT: u128 = 10;

/// Developer wallet — Nexuzy Lab / David @david0154
pub const DEV_WALLET: &str = "db1xdev_nexuzy_lab_david0154_00000000";
/// Mining pool wallet
pub const MINING_POOL_WALLET: &str = "db1xmining_pool_devilchain_000000000";
/// Burn address — coins sent here are unspendable
pub const BURN_ADDRESS: &str = "db1x000000000000000000000000000burn";
/// Liquidity lock vault — time-locked, DAO-controlled
pub const LIQUIDITY_LOCK_VAULT: &str = "db1xliquidity_lock_vault_dao_00000000";

// ── Emission Schedule ────────────────────────────────────────────────────────

/// Calculate block reward at a given block height (with halving)
pub fn block_reward_at(height: u64) -> u128 {
    let halvings = height / HALVING_INTERVAL;
    if halvings >= 64 { return 0; } // reward hits 0 after 64 halvings
    BLOCK_REWARD >> halvings
}

/// Split a transaction fee into (miner, dev, burn, liquidity) amounts
pub fn split_fee(fee: u128) -> FeeSplit {
    let miner     = fee * FEE_MINER_PCT     / 100;
    let dev       = fee * FEE_DEV_PCT       / 100;
    let burn      = fee * FEE_BURN_PCT      / 100;
    let liquidity = fee * FEE_LIQUIDITY_PCT / 100;
    FeeSplit { miner, dev, burn, liquidity }
}

#[derive(Debug, Clone)]
pub struct FeeSplit {
    pub miner:     u128,  // → miner address
    pub dev:       u128,  // → DEV_WALLET
    pub burn:      u128,  // → BURN_ADDRESS (destroyed)
    pub liquidity: u128,  // → LIQUIDITY_LOCK_VAULT
}

// ── Supply Tracker ───────────────────────────────────────────────────────────

#[derive(Debug)]
pub struct SupplyTracker {
    pub total_minted:  u128,
    pub total_burned:  u128,
    pub locked_supply: u128,  // in liquidity lock vault
}

impl Default for SupplyTracker {
    fn default() -> Self {
        Self { total_minted: 0, total_burned: 0, locked_supply: 0 }
    }
}

impl SupplyTracker {
    pub fn circulating(&self) -> u128 {
        self.total_minted
            .saturating_sub(self.total_burned)
            .saturating_sub(self.locked_supply)
    }

    pub fn can_mint(&self, amount: u128) -> bool {
        self.total_minted.saturating_add(amount) <= MAX_SUPPLY
    }

    pub fn mint(&mut self, amount: u128) -> Result<(), &'static str> {
        if !self.can_mint(amount) {
            return Err("Max supply reached");
        }
        self.total_minted += amount;
        Ok(())
    }

    pub fn burn(&mut self, amount: u128) {
        self.total_burned = self.total_burned.saturating_add(amount);
    }

    pub fn lock_liquidity(&mut self, amount: u128) {
        self.locked_supply = self.locked_supply.saturating_add(amount);
    }

    pub fn unlock_liquidity(&mut self, amount: u128) {
        self.locked_supply = self.locked_supply.saturating_sub(amount);
    }
}

// ── Liquidity Lock ───────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct LiquidityLock {
    pub id:          String,
    pub locker:      String,
    pub amount:      u128,
    pub unlock_block: u64,
    pub unlocked:    bool,
}

#[derive(Debug, Default)]
pub struct LiquidityPool {
    locks: Vec<LiquidityLock>,
    next_id: u64,
}

impl LiquidityPool {
    pub fn lock(&mut self, locker: String, amount: u128, current_block: u64,
                lock_blocks: u64) -> String {
        let id = format!("lock_{}", self.next_id);
        self.next_id += 1;
        self.locks.push(LiquidityLock {
            id: id.clone(), locker, amount,
            unlock_block: current_block + lock_blocks,
            unlocked: false,
        });
        id
    }

    pub fn try_unlock(&mut self, id: &str, current_block: u64)
        -> Result<u128, &'static str>
    {
        let lock = self.locks.iter_mut()
            .find(|l| l.id == id)
            .ok_or("Lock not found")?;
        if lock.unlocked { return Err("Already unlocked"); }
        if current_block < lock.unlock_block {
            return Err("Still locked");
        }
        lock.unlocked = true;
        Ok(lock.amount)
    }

    pub fn get_locks(&self) -> &[LiquidityLock] { &self.locks }
}

// ── NFT Engine ───────────────────────────────────────────────────────────────

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NftToken {
    pub token_id:   String,
    pub collection: String,
    pub owner:      String,
    pub creator:    String,
    pub metadata:   NftMetadata,
    pub minted_at:  u64,
    pub tx_hash:    String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NftMetadata {
    pub name:        String,
    pub description: String,
    pub image_cid:   String,  // DevilStorage CID
    pub attributes:  Vec<NftAttribute>,
    pub royalty_pct: u8,      // 0–10%
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NftAttribute {
    pub trait_type: String,
    pub value:      String,
}

#[derive(Debug, Default)]
pub struct NftRegistry {
    tokens: HashMap<String, NftToken>,
    collections: HashMap<String, Vec<String>>, // collection → token_ids
}

impl NftRegistry {
    pub fn mint(&mut self, token: NftToken) -> Result<(), &'static str> {
        if self.tokens.contains_key(&token.token_id) {
            return Err("Token ID already exists");
        }
        self.collections
            .entry(token.collection.clone())
            .or_default()
            .push(token.token_id.clone());
        self.tokens.insert(token.token_id.clone(), token);
        Ok(())
    }

    pub fn transfer(&mut self, token_id: &str, from: &str, to: &str)
        -> Result<(), &'static str>
    {
        let token = self.tokens.get_mut(token_id).ok_or("Token not found")?;
        if token.owner != from { return Err("Not owner"); }
        token.owner = to.to_string();
        Ok(())
    }

    pub fn get(&self, token_id: &str) -> Option<&NftToken> {
        self.tokens.get(token_id)
    }

    pub fn tokens_of(&self, owner: &str) -> Vec<&NftToken> {
        self.tokens.values().filter(|t| t.owner == owner).collect()
    }

    pub fn collection(&self, name: &str) -> Vec<&NftToken> {
        self.collections.get(name)
            .map(|ids| ids.iter()
                .filter_map(|id| self.tokens.get(id))
                .collect())
            .unwrap_or_default()
    }
}
