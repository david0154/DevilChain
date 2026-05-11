//! DevilChain Validator System
//! Handles: registration, reputation, selection, rewards, slashing

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Validator {
    pub address: String,
    pub staked_amount: f64,
    pub reputation_score: f64,
    pub validator_score: f64,
    pub blocks_validated: u64,
    pub blocks_missed: u64,
    pub is_active: bool,
    pub registered_at: u64,
    pub last_active: u64,
}

impl Validator {
    pub fn new(address: String, staked: f64, timestamp: u64) -> Self {
        Validator {
            address,
            staked_amount: staked,
            reputation_score: 100.0,
            validator_score: 1.0,
            blocks_validated: 0,
            blocks_missed: 0,
            is_active: true,
            registered_at: timestamp,
            last_active: timestamp,
        }
    }

    /// Voting Power = Stake + Reputation + Validator Score
    pub fn voting_power(&self) -> f64 {
        self.staked_amount + self.reputation_score + self.validator_score
    }

    /// Slash validator for misbehavior
    pub fn slash(&mut self, amount: f64) {
        self.staked_amount = (self.staked_amount - amount).max(0.0);
        self.reputation_score = (self.reputation_score - 20.0).max(0.0);
        if self.staked_amount < 100.0 {
            self.is_active = false;
        }
    }

    /// Reward validator for successful block validation
    pub fn reward(&mut self, block_reward: f64, timestamp: u64) {
        self.staked_amount += block_reward;
        self.blocks_validated += 1;
        self.reputation_score = (self.reputation_score + 0.1).min(1000.0);
        self.last_active = timestamp;
    }

    /// Penalize for missed block
    pub fn penalize_miss(&mut self) {
        self.blocks_missed += 1;
        self.reputation_score = (self.reputation_score - 1.0).max(0.0);
        self.validator_score = (self.validator_score - 0.05).max(0.1);
    }
}

pub struct ValidatorSet {
    pub validators: HashMap<String, Validator>,
    pub min_stake: f64,
    pub block_reward: f64,
}

impl ValidatorSet {
    pub fn new() -> Self {
        ValidatorSet {
            validators: HashMap::new(),
            min_stake: 100.0,
            block_reward: 10.0,
        }
    }

    pub fn register(&mut self, address: String, stake: f64, timestamp: u64) -> Result<(), String> {
        if stake < self.min_stake {
            return Err(format!("Minimum stake is {} DVC", self.min_stake));
        }
        if self.validators.contains_key(&address) {
            return Err("Validator already registered".to_string());
        }
        self.validators.insert(address.clone(), Validator::new(address, stake, timestamp));
        Ok(())
    }

    pub fn unregister(&mut self, address: &str) {
        if let Some(v) = self.validators.get_mut(address) {
            v.is_active = false;
        }
    }

    /// Select next validator weighted by voting power (PoS)
    pub fn select_validator(&self) -> Option<&Validator> {
        let active: Vec<&Validator> = self.validators.values().filter(|v| v.is_active).collect();
        if active.is_empty() { return None; }
        // Weighted selection by voting power
        let total_power: f64 = active.iter().map(|v| v.voting_power()).sum();
        let mut rng_pick = (total_power * 0.5) as u64 % (active.len() as u64).max(1);
        for v in &active {
            let power_slots = (v.voting_power() / total_power * 100.0) as u64;
            if rng_pick < power_slots {
                return Some(v);
            }
            rng_pick = rng_pick.saturating_sub(power_slots);
        }
        active.first().copied()
    }

    pub fn reward_validator(&mut self, address: &str, timestamp: u64) {
        if let Some(v) = self.validators.get_mut(address) {
            v.reward(self.block_reward, timestamp);
        }
    }

    pub fn slash_validator(&mut self, address: &str, amount: f64) {
        if let Some(v) = self.validators.get_mut(address) {
            v.slash(amount);
        }
    }

    pub fn active_validators(&self) -> Vec<&Validator> {
        self.validators.values().filter(|v| v.is_active).collect()
    }

    pub fn total_staked(&self) -> f64 {
        self.validators.values().map(|v| v.staked_amount).sum()
    }
}
