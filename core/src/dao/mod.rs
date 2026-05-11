//! DevilChain DAO Governance

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Proposal {
    pub id: u64,
    pub title: String,
    pub description: String,
    pub proposer: String,
    pub votes_for: f64,
    pub votes_against: f64,
    pub status: ProposalStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProposalStatus {
    Active,
    Passed,
    Rejected,
    Executed,
}

/// Voting Power = Stake + Reputation + Validator Score
pub fn compute_voting_power(stake: f64, reputation: f64, validator_score: f64) -> f64 {
    stake + reputation + validator_score
}

pub struct DAOEngine {
    pub proposals: Vec<Proposal>,
    pub next_id: u64,
}

impl DAOEngine {
    pub fn new() -> Self {
        DAOEngine { proposals: vec![], next_id: 1 }
    }

    pub fn submit_proposal(&mut self, title: &str, desc: &str, proposer: &str) -> u64 {
        let id = self.next_id;
        self.proposals.push(Proposal {
            id,
            title: title.to_string(),
            description: desc.to_string(),
            proposer: proposer.to_string(),
            votes_for: 0.0,
            votes_against: 0.0,
            status: ProposalStatus::Active,
        });
        self.next_id += 1;
        id
    }

    pub fn vote(&mut self, proposal_id: u64, power: f64, in_favor: bool) {
        if let Some(p) = self.proposals.iter_mut().find(|p| p.id == proposal_id) {
            if in_favor {
                p.votes_for += power;
            } else {
                p.votes_against += power;
            }
        }
    }

    pub fn finalize(&mut self, proposal_id: u64) {
        if let Some(p) = self.proposals.iter_mut().find(|p| p.id == proposal_id) {
            if p.votes_for > p.votes_against {
                p.status = ProposalStatus::Passed;
            } else {
                p.status = ProposalStatus::Rejected;
            }
        }
    }
}
