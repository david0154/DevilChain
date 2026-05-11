//! DevilChain Tokenomics Engine — burn cap + liquidity lock rules
//! All amounts in micro-DVC (1 DVC = 1_000_000 µDVC)

use std::collections::HashMap;

pub const DECIMALS: u64 = 1_000_000;
pub const MAX_SUPPLY: u128 = 1_000_000_000 * DECIMALS as u128;
pub const BLOCK_REWARD: u128 = 50 * DECIMALS as u128;
pub const HALVING_INTERVAL: u64 = 2_100_000;
pub const MIN_GAS_FEE: u128 = 10_000;

pub const GAS_STANDARD_BP: u128 = 30;
pub const GAS_DEX_BP: u128 = 150;
pub const GAS_NFT_BP: u128 = 200;
pub const GAS_CONTRACT_BP: u128 = 500;

pub const BURN_CAP: u128 = 200_000_000 * DECIMALS as u128;
pub const LIQUIDITY_TARGET: u128 = 200_000_000 * DECIMALS as u128;
pub const LIQUIDITY_AUTO_LOCK_BP: u128 = 500; // 5%
pub const FIVE_YEARS_SECS: u64 = 5 * 365 * 24 * 60 * 60;

pub fn compute_gas(amount: u128, tx_type: TxType) -> u128 {
    let bp = match tx_type {
        TxType::Transfer => GAS_STANDARD_BP,
        TxType::DexSwap => GAS_DEX_BP,
        TxType::NftSale => GAS_NFT_BP,
        TxType::Contract => GAS_CONTRACT_BP,
    };
    (amount * bp / 10_000).max(MIN_GAS_FEE)
}

#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum TxType {
    Transfer,
    DexSwap,
    NftSale,
    Contract,
}

impl Default for TxType {
    fn default() -> Self { TxType::Transfer }
}

pub const FEE_MINER_PCT: u128 = 55;
pub const FEE_DEV_PCT: u128 = 18;
pub const FEE_BURN_PCT: u128 = 10;
pub const FEE_LIQUIDITY_PCT: u128 = 10;
pub const FEE_MARKETING_PCT: u128 = 7;

pub const FEE_POST_BURN_MINER_PCT: u128 = 55;
pub const FEE_POST_BURN_DEV_PCT: u128 = 23;
pub const FEE_POST_BURN_BURN_PCT: u128 = 0;
pub const FEE_POST_BURN_LIQUIDITY_PCT: u128 = 15;
pub const FEE_POST_BURN_MARKETING_PCT: u128 = 7;

pub const DEV_WALLET: &str = "db1xdev_nexuzy_lab_david0154_00000000";
pub const MINING_POOL_WALLET: &str = "db1xmining_pool_devilchain_000000000";
pub const BURN_ADDRESS: &str = "db1x000000000000000000000000000burn";
pub const LIQUIDITY_LOCK_VAULT: &str = "db1xliquidity_lock_vault_dao_00000000";
pub const MARKETING_WALLET: &str = "db1xmarketing_fund_devilchain_000000";

#[derive(Debug, Clone)]
pub struct FeeSplit {
    pub miner: u128,
    pub dev: u128,
    pub burn: u128,
    pub liquidity: u128,
    pub marketing: u128,
}

pub fn split_fee_with_state(fee: u128, burn_stopped: bool) -> FeeSplit {
    if burn_stopped {
        let miner = fee * FEE_POST_BURN_MINER_PCT / 100;
        let dev = fee * FEE_POST_BURN_DEV_PCT / 100;
        let burn = 0;
        let liquidity = fee * FEE_POST_BURN_LIQUIDITY_PCT / 100;
        let marketing = fee.saturating_sub(miner + dev + burn + liquidity);
        FeeSplit { miner, dev, burn, liquidity, marketing }
    } else {
        let miner = fee * FEE_MINER_PCT / 100;
        let dev = fee * FEE_DEV_PCT / 100;
        let burn = fee * FEE_BURN_PCT / 100;
        let liquidity = fee * FEE_LIQUIDITY_PCT / 100;
        let marketing = fee.saturating_sub(miner + dev + burn + liquidity);
        FeeSplit { miner, dev, burn, liquidity, marketing }
    }
}

pub fn split_fee(fee: u128) -> FeeSplit {
    split_fee_with_state(fee, false)
}

pub fn block_reward_at(height: u64) -> u128 {
    let halvings = height / HALVING_INTERVAL;
    if halvings >= 64 { return 0; }
    BLOCK_REWARD >> halvings
}

#[derive(Debug, Clone)]
pub struct TimelockPosition {
    pub id: String,
    pub amount: u128,
    pub created_at: u64,
    pub unlock_at: u64,
    pub unlocked: bool,
}

#[derive(Debug)]
pub struct SupplyTracker {
    pub total_minted: u128,
    pub total_burned: u128,
    pub locked_supply: u128,
    pub burn_stopped: bool,
    pub liquidity_auto_locked: bool,
    pub timelocks: Vec<TimelockPosition>,
}

impl Default for SupplyTracker {
    fn default() -> Self {
        Self {
            total_minted: 0,
            total_burned: 0,
            locked_supply: 0,
            burn_stopped: false,
            liquidity_auto_locked: false,
            timelocks: Vec::new(),
        }
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
        if !self.can_mint(amount) { return Err("Max supply reached"); }
        self.total_minted += amount;
        Ok(())
    }

    pub fn burn(&mut self, amount: u128) -> u128 {
        if self.burn_stopped || amount == 0 { return 0; }
        let remaining = BURN_CAP.saturating_sub(self.total_burned);
        let actual = amount.min(remaining);
        self.total_burned = self.total_burned.saturating_add(actual);
        if self.total_burned >= BURN_CAP {
            self.burn_stopped = true;
        }
        actual
    }

    pub fn lock_liquidity(&mut self, amount: u128) {
        self.locked_supply = self.locked_supply.saturating_add(amount);
    }

    pub fn unlock_liquidity(&mut self, amount: u128) {
        self.locked_supply = self.locked_supply.saturating_sub(amount);
    }

    pub fn should_stop_burn(&self) -> bool {
        self.total_burned >= BURN_CAP || self.burn_stopped
    }

    pub fn should_auto_lock_liquidity(&self, liquidity_balance: u128) -> bool {
        !self.liquidity_auto_locked && liquidity_balance >= LIQUIDITY_TARGET
    }

    pub fn auto_lock_liquidity_if_needed(&mut self, liquidity_balance: u128, now_secs: u64) -> Option<TimelockPosition> {
        if !self.should_auto_lock_liquidity(liquidity_balance) {
            return None;
        }
        let amount = liquidity_balance * LIQUIDITY_AUTO_LOCK_BP / 10_000;
        if amount == 0 { return None; }
        self.locked_supply = self.locked_supply.saturating_add(amount);
        self.liquidity_auto_locked = true;
        let pos = TimelockPosition {
            id: format!("liq_timelock_{}", self.timelocks.len()),
            amount,
            created_at: now_secs,
            unlock_at: now_secs.saturating_add(FIVE_YEARS_SECS),
            unlocked: false,
        };
        self.timelocks.push(pos.clone());
        Some(pos)
    }

    pub fn unlock_matured_timelocks(&mut self, now_secs: u64) -> Vec<TimelockPosition> {
        let mut unlocked = Vec::new();
        for tl in &mut self.timelocks {
            if !tl.unlocked && now_secs >= tl.unlock_at {
                tl.unlocked = true;
                self.locked_supply = self.locked_supply.saturating_sub(tl.amount);
                unlocked.push(tl.clone());
            }
        }
        unlocked
    }
}

#[derive(Debug, Clone)]
pub struct LiquidityLock {
    pub id: String,
    pub locker: String,
    pub amount: u128,
    pub unlock_block: u64,
    pub unlocked: bool,
}

#[derive(Debug, Default)]
pub struct LiquidityPool {
    locks: Vec<LiquidityLock>,
    next_id: u64,
}

impl LiquidityPool {
    pub fn lock(&mut self, locker: String, amount: u128, current_block: u64, lock_blocks: u64) -> String {
        let id = format!("lock_{}", self.next_id);
        self.next_id += 1;
        self.locks.push(LiquidityLock {
            id: id.clone(), locker, amount,
            unlock_block: current_block + lock_blocks,
            unlocked: false,
        });
        id
    }

    pub fn try_unlock(&mut self, id: &str, current_block: u64) -> Result<u128, &'static str> {
        let lock = self.locks.iter_mut().find(|l| l.id == id).ok_or("Lock not found")?;
        if lock.unlocked { return Err("Already unlocked"); }
        if current_block < lock.unlock_block { return Err("Still locked"); }
        lock.unlocked = true;
        Ok(lock.amount)
    }

    pub fn get_locks(&self) -> &[LiquidityLock] { &self.locks }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NftToken {
    pub token_id: String,
    pub collection: String,
    pub owner: String,
    pub creator: String,
    pub metadata: NftMetadata,
    pub minted_at: u64,
    pub tx_hash: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NftMetadata {
    pub name: String,
    pub description: String,
    pub image_cid: String,
    pub attributes: Vec<NftAttribute>,
    pub royalty_pct: u8,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NftAttribute {
    pub trait_type: String,
    pub value: String,
}

#[derive(Debug, Default)]
pub struct NftRegistry {
    tokens: HashMap<String, NftToken>,
    collections: HashMap<String, Vec<String>>,
}

impl NftRegistry {
    pub fn mint(&mut self, token: NftToken) -> Result<(), &'static str> {
        if self.tokens.contains_key(&token.token_id) { return Err("Token ID already exists"); }
        self.collections.entry(token.collection.clone()).or_default().push(token.token_id.clone());
        self.tokens.insert(token.token_id.clone(), token);
        Ok(())
    }

    pub fn transfer(&mut self, token_id: &str, from: &str, to: &str) -> Result<(), &'static str> {
        let t = self.tokens.get_mut(token_id).ok_or("Token not found")?;
        if t.owner != from { return Err("Not owner"); }
        t.owner = to.to_string();
        Ok(())
    }

    pub fn get(&self, id: &str) -> Option<&NftToken> { self.tokens.get(id) }
    pub fn tokens_of(&self, owner: &str) -> Vec<&NftToken> {
        self.tokens.values().filter(|t| t.owner == owner).collect()
    }
    pub fn collection(&self, name: &str) -> Vec<&NftToken> {
        self.collections.get(name)
            .map(|ids| ids.iter().filter_map(|id| self.tokens.get(id)).collect())
            .unwrap_or_default()
    }
}
