//! DevilChain Validator System
//! Handles: registration, reputation, selection, rewards

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Validator {
    pub address: String,
    pub staked_dvc: f64,
    pub reputation_score: f64,
    pub validator_score: f64,
    pub blocks_validated: u64,
    pub active: bool,
    pub registered_at: u64,
}

impl Validator {
    pub fn new(address: String, staked_dvc: f64) -> Self {
        Validator {
            address,
            staked_dvc,
            reputation_score: 0.0,
            validator_score: 0.0,
            blocks_validated: 0,
            active: staked_dvc >= 100.0,
            registered_at: crate::blockchain::now_timestamp(),
        }
    }

    /// Voting Power = Stake + Reputation + Validator Score
    pub fn voting_power(&self) -> f64 {
        self.staked_dvc + self.reputation_score + self.validator_score
    }

    pub fn add_block_reward(&mut self, reward: f64) {
        self.blocks_validated += 1;
        self.reputation_score += 0.1;
        self.validator_score += reward * 0.01;
    }

    pub fn slash(&mut self, amount: f64) {
        self.staked_dvc = (self.staked_dvc - amount).max(0.0);
        self.reputation_score = (self.reputation_score - 1.0).max(0.0);
        if self.staked_dvc < 100.0 {
            self.active = false;
        }
    }
}

pub struct ValidatorRegistry {
    pub validators: HashMap<String, Validator>,
}

impl ValidatorRegistry {
    pub fn new() -> Self {
        ValidatorRegistry {
            validators: HashMap::new(),
        }
    }

    pub fn register(&mut self, address: String, staked: f64) -> bool {
        if staked < 100.0 {
            return false;
        }
        let v = Validator::new(address.clone(), staked);
        self.validators.insert(address, v);
        true
    }

    pub fn get_active(&self) -> Vec<&Validator> {
        self.validators.values().filter(|v| v.active).collect()
    }

    /// Select validator by highest voting power (PoS)
    pub fn select_validator(&self) -> Option<&Validator> {
        self.validators
            .values()
            .filter(|v| v.active)
            .max_by(|a, b| a.voting_power().partial_cmp(&b.voting_power()).unwrap())
    }

    pub fn reward_validator(&mut self, address: &str, reward: f64) {
        if let Some(v) = self.validators.get_mut(address) {
            v.add_block_reward(reward);
        }
    }

    pub fn slash_validator(&mut self, address: &str, amount: f64) {
        if let Some(v) = self.validators.get_mut(address) {
            v.slash(amount);
        }
    }

    pub fn total_staked(&self) -> f64 {
        self.validators.values().map(|v| v.staked_dvc).sum()
    }

    pub fn count(&self) -> usize {
        self.validators.len()
    }
}
