//! DevilChain Tokenomics Engine — v2
//! Supply cap, emission, halving, burn, liquidity lock, fee distribution
//! All amounts in micro-DVC (1 DVC = 1_000_000 µDVC) — ZERO f64 for money
//!
//! Fee Schedule:
//!   Standard Transfer  = 0.3%
//!   DEX Swap           = 1.5%
//!   NFT Marketplace    = 2.0%
//!   Smart Contract     = 5.0% (variable gas, minimum 5%)
//!
//! Fee Distribution (of collected fee):
//!   55% → Miner/Validator
//!   18% → Development Fund (Nexuzy Lab)
//!   10% → Burn (deflationary)
//!   10% → Liquidity Pool
//!    7% → Marketing Fund
//!
//! Developed by Nexuzy Lab (nexuzy.tech) | Powered by Devil One (devilone.in)

use std::collections::HashMap;

// ── Constants ─────────────────────────────────────────────────────────────────
pub const DECIMALS:          u64  = 1_000_000;
pub const MAX_SUPPLY:        u128 = 1_000_000_000 * DECIMALS as u128;  // 1B DVC
pub const BLOCK_REWARD:      u128 = 50 * DECIMALS as u128;             // 50 DVC
pub const HALVING_INTERVAL:  u64  = 2_100_000;                         // blocks

/// Minimum gas fee in µDVC — 0.01 DVC (used as floor for all tx types)
pub const MIN_GAS_FEE: u128 = 10_000;

// ── Gas Fee Rates (basis points, 1 bp = 0.01%) ────────────────────────────────
/// Standard transfer: 0.3% = 30 bp
pub const GAS_STANDARD_BP: u128 = 30;
/// DEX swap: 1.5% = 150 bp
pub const GAS_DEX_BP: u128 = 150;
/// NFT marketplace: 2.0% = 200 bp
pub const GAS_NFT_BP: u128 = 200;
/// Smart contract deployment: 5.0% = 500 bp (minimum, scales with code size)
pub const GAS_CONTRACT_BP: u128 = 500;

/// Compute gas fee for a given amount and tx type
pub fn compute_gas(amount: u128, tx_type: TxType) -> u128 {
    let bp = match tx_type {
        TxType::Transfer  => GAS_STANDARD_BP,
        TxType::DexSwap   => GAS_DEX_BP,
        TxType::NftSale   => GAS_NFT_BP,
        TxType::Contract  => GAS_CONTRACT_BP,
    };
    let fee = amount * bp / 10_000;
    fee.max(MIN_GAS_FEE)  // always at least 0.01 DVC
}

#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum TxType {
    Transfer,   // 0.3%
    DexSwap,    // 1.5%
    NftSale,    // 2.0%
    Contract,   // 5.0%+
}

impl Default for TxType {
    fn default() -> Self { TxType::Transfer }
}

// ── Fee Distribution ──────────────────────────────────────────────────────────
/// 55% → Miner, 18% → Dev, 10% → Burn, 10% → Liquidity, 7% → Marketing
pub const FEE_MINER_PCT:     u128 = 55;
pub const FEE_DEV_PCT:       u128 = 18;
pub const FEE_BURN_PCT:      u128 = 10;
pub const FEE_LIQUIDITY_PCT: u128 = 10;
pub const FEE_MARKETING_PCT: u128 =  7;

/// Developer wallet — Nexuzy Lab / David @david0154
pub const DEV_WALLET:          &str = "db1xdev_nexuzy_lab_david0154_00000000";
/// Mining pool wallet
pub const MINING_POOL_WALLET:  &str = "db1xmining_pool_devilchain_000000000";
/// Burn address — coins sent here are permanently destroyed
pub const BURN_ADDRESS:        &str = "db1x000000000000000000000000000burn";
/// Liquidity lock vault — time-locked, DAO-controlled
pub const LIQUIDITY_LOCK_VAULT: &str = "db1xliquidity_lock_vault_dao_00000000";
/// Marketing fund wallet
pub const MARKETING_WALLET:    &str = "db1xmarketing_fund_devilchain_000000";

// ── Fee Split ─────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct FeeSplit {
    pub miner:     u128,  // 55% → validator
    pub dev:       u128,  // 18% → DEV_WALLET
    pub burn:      u128,  // 10% → BURN_ADDRESS
    pub liquidity: u128,  // 10% → LIQUIDITY_LOCK_VAULT
    pub marketing: u128,  //  7% → MARKETING_WALLET
}

/// Example: 100 DVC send, 1.5 DVC fee (DEX swap 1.5%)
/// miner=0.825 DVC, dev=0.27 DVC, burn=0.15 DVC, liq=0.15 DVC, mkt=0.105 DVC
pub fn split_fee(fee: u128) -> FeeSplit {
    let miner     = fee * FEE_MINER_PCT     / 100;
    let dev       = fee * FEE_DEV_PCT       / 100;
    let burn      = fee * FEE_BURN_PCT      / 100;
    let liquidity = fee * FEE_LIQUIDITY_PCT / 100;
    // Marketing gets exact remainder to avoid rounding loss
    let marketing = fee.saturating_sub(miner + dev + burn + liquidity);
    FeeSplit { miner, dev, burn, liquidity, marketing }
}

// ── Emission ──────────────────────────────────────────────────────────────────
pub fn block_reward_at(height: u64) -> u128 {
    let halvings = height / HALVING_INTERVAL;
    if halvings >= 64 { return 0; }
    BLOCK_REWARD >> halvings
}

// ── Supply Tracker ────────────────────────────────────────────────────────────

#[derive(Debug)]
pub struct SupplyTracker {
    pub total_minted:  u128,
    pub total_burned:  u128,
    pub locked_supply: u128,
}

impl Default for SupplyTracker {
    fn default() -> Self { Self { total_minted: 0, total_burned: 0, locked_supply: 0 } }
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
        if !self.can_mint(amount) { return Err("Max supply reached"); }
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

// ── Liquidity Lock ────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct LiquidityLock {
    pub id:           String,
    pub locker:       String,
    pub amount:       u128,
    pub unlock_block: u64,
    pub unlocked:     bool,
}

#[derive(Debug, Default)]
pub struct LiquidityPool {
    locks:   Vec<LiquidityLock>,
    next_id: u64,
}

impl LiquidityPool {
    pub fn lock(&mut self, locker: String, amount: u128,
                current_block: u64, lock_blocks: u64) -> String {
        let id = format!("lock_{}", self.next_id); self.next_id += 1;
        self.locks.push(LiquidityLock {
            id: id.clone(), locker, amount,
            unlock_block: current_block + lock_blocks, unlocked: false,
        });
        id
    }
    pub fn try_unlock(&mut self, id: &str, current_block: u64)
        -> Result<u128, &'static str>
    {
        let lock = self.locks.iter_mut().find(|l| l.id == id)
            .ok_or("Lock not found")?;
        if lock.unlocked { return Err("Already unlocked"); }
        if current_block < lock.unlock_block { return Err("Still locked"); }
        lock.unlocked = true;
        Ok(lock.amount)
    }
    pub fn get_locks(&self) -> &[LiquidityLock] { &self.locks }
}

// ── NFT Engine ────────────────────────────────────────────────────────────────

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
    pub image_cid:   String,
    pub attributes:  Vec<NftAttribute>,
    pub royalty_pct: u8,   // 0–10%
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NftAttribute {
    pub trait_type: String,
    pub value:      String,
}

#[derive(Debug, Default)]
pub struct NftRegistry {
    tokens:      HashMap<String, NftToken>,
    collections: HashMap<String, Vec<String>>,
}

impl NftRegistry {
    pub fn mint(&mut self, token: NftToken) -> Result<(), &'static str> {
        if self.tokens.contains_key(&token.token_id) {
            return Err("Token ID already exists");
        }
        self.collections.entry(token.collection.clone())
            .or_default().push(token.token_id.clone());
        self.tokens.insert(token.token_id.clone(), token);
        Ok(())
    }
    pub fn transfer(&mut self, token_id: &str, from: &str, to: &str)
        -> Result<(), &'static str>
    {
        let t = self.tokens.get_mut(token_id).ok_or("Token not found")?;
        if t.owner != from { return Err("Not owner"); }
        t.owner = to.to_string();
        Ok(())
    }
    pub fn get(&self, id: &str)             -> Option<&NftToken> { self.tokens.get(id) }
    pub fn tokens_of(&self, owner: &str)    -> Vec<&NftToken> {
        self.tokens.values().filter(|t| t.owner == owner).collect()
    }
    pub fn collection(&self, name: &str)    -> Vec<&NftToken> {
        self.collections.get(name)
            .map(|ids| ids.iter().filter_map(|id| self.tokens.get(id)).collect())
            .unwrap_or_default()
    }
}
