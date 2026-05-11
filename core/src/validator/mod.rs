//! DevilChain Validator System
//! Handles validator registration, selection, reputation scoring

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Validator {
    pub address: String,
    pub stake: f64,
    pub reputation: f64,
    pub blocks_validated: u64,
    pub uptime_percent: f64,
    pub active: bool,
    pub registered_at: u64,
}

impl Validator {
    pub fn new(address: String, stake: f64) -> Self {
        Validator {
            address,
            stake,
            reputation: 0.0,
            blocks_validated: 0,
            uptime_percent: 100.0,
            active: true,
            registered_at: crate::blockchain::now_timestamp(),
        }
    }

    /// Voting power = stake + reputation + validator_bonus
    pub fn voting_power(&self) -> f64 {
        self.stake + self.reputation + (self.blocks_validated as f64 * 0.01)
    }

    pub fn update_reputation(&mut self, success: bool) {
        if success {
            self.reputation = (self.reputation + 0.5).min(100.0);
            self.blocks_validated += 1;
        } else {
            self.reputation = (self.reputation - 1.0).max(0.0);
        }
    }
}

pub struct ValidatorSet {
    pub validators: HashMap<String, Validator>,
    pub min_stake: f64,
}

impl ValidatorSet {
    pub fn new() -> Self {
        ValidatorSet {
            validators: HashMap::new(),
            min_stake: 100.0,
        }
    }

    pub fn register(&mut self, address: String, stake: f64) -> Result<(), String> {
        if stake < self.min_stake {
            return Err(format!("Minimum stake is {} DVC", self.min_stake));
        }
        if self.validators.contains_key(&address) {
            return Err("Validator already registered".to_string());
        }
        self.validators.insert(address.clone(), Validator::new(address, stake));
        Ok(())
    }

    pub fn get_active(&self) -> Vec<&Validator> {
        self.validators.values().filter(|v| v.active).collect()
    }

    /// Select validator by weighted stake (PoS)
    pub fn select_validator(&self) -> Option<&Validator> {
        let active: Vec<&Validator> = self.get_active();
        if active.is_empty() { return None; }
        // Select highest voting power validator
        active.iter().max_by(|a, b| {
            a.voting_power().partial_cmp(&b.voting_power()).unwrap()
        }).copied()
    }

    pub fn slash(&mut self, address: &str, amount: f64) {
        if let Some(v) = self.validators.get_mut(address) {
            v.stake = (v.stake - amount).max(0.0);
            if v.stake < self.min_stake {
                v.active = false;
            }
        }
    }

    pub fn deactivate(&mut self, address: &str) {
        if let Some(v) = self.validators.get_mut(address) {
            v.active = false;
        }
    }

    pub fn list(&self) -> Vec<&Validator> {
        self.validators.values().collect()
    }
}
