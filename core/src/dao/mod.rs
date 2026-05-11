//! DevilChain DAO Governance — proposals, voting, tally, execution
//! Developed by Nexuzy Lab (nexuzy.tech) | Powered by Devil One (devilone.in)

use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use crate::blockchain::Amount;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ProposalStatus { Active, Passed, Rejected, Executed, Expired }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProposalAction {
    UpdateBlockReward  { new_reward: Amount },
    UpdateGasFee       { new_min_fee: Amount },
    UpdateDifficulty   { new_difficulty: u32 },
    TransferTreasury   { to: String, amount: Amount },
    UnlockLiquidity    { lock_id: String },
    AddValidator       { address: String },
    Text               { description: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Proposal {
    pub id:          u64,
    pub title:       String,
    pub description: String,
    pub proposer:    String,
    pub action:      ProposalAction,
    pub votes_yes:   Amount,
    pub votes_no:    Amount,
    pub voters:      HashMap<String, bool>,
    pub status:      ProposalStatus,
    pub created_at:  u64,
    pub expires_at:  u64,
    pub quorum:      Amount,
}

impl Proposal {
    pub fn is_active(&self, current_block: u64) -> bool {
        self.status == ProposalStatus::Active && current_block < self.expires_at
    }
    pub fn tally(&mut self, current_block: u64) {
        if current_block < self.expires_at { return; }
        let total = self.votes_yes + self.votes_no;
        self.status = if total < self.quorum { ProposalStatus::Expired }
            else if self.votes_yes > self.votes_no { ProposalStatus::Passed }
            else { ProposalStatus::Rejected };
    }
}

#[derive(Debug, Default)]
pub struct DaoGovernance {
    proposals: Vec<Proposal>,
    next_id:   u64,
}

impl DaoGovernance {
    pub fn proposal_count(&self) -> u64 { self.next_id }

    pub fn create_proposal(
        &mut self, title: String, description: String,
        proposer: String, action: ProposalAction,
        current_block: u64, quorum: Amount,
    ) -> u64 {
        let id = self.next_id; self.next_id += 1;
        self.proposals.push(Proposal {
            id, title, description, proposer, action,
            votes_yes: 0, votes_no: 0,
            voters: HashMap::new(),
            status: ProposalStatus::Active,
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_secs()).unwrap_or(0),
            expires_at: current_block + 10_000,
            quorum,
        });
        id
    }

    pub fn vote(
        &mut self, proposal_id: u64, voter: String,
        yes: bool, voting_power: Amount,
    ) -> Result<(), &'static str> {
        let p = self.proposals.iter_mut()
            .find(|p| p.id == proposal_id).ok_or("Proposal not found")?;
        if p.status != ProposalStatus::Active { return Err("Not active"); }
        if p.voters.contains_key(&voter) { return Err("Already voted"); }
        p.voters.insert(voter, yes);
        if yes { p.votes_yes += voting_power; } else { p.votes_no += voting_power; }
        Ok(())
    }

    pub fn get_proposal(&self, id: u64) -> Option<&Proposal> {
        self.proposals.iter().find(|p| p.id == id)
    }
    pub fn get_active(&self, current_block: u64) -> Vec<&Proposal> {
        self.proposals.iter().filter(|p| p.is_active(current_block)).collect()
    }
    pub fn tally_all(&mut self, current_block: u64) {
        for p in self.proposals.iter_mut() {
            if p.status == ProposalStatus::Active { p.tally(current_block); }
        }
    }
}
