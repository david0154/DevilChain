//! DevilGuard AI — Rule-based risk scoring (no stub hardcoding)
//! Scores 0–100: higher = safer. Block rejected if score < 50.
//!
//! Developed by Nexuzy Lab (nexuzy.tech) | Powered by Devil One (devilone.in)

use crate::blockchain::Transaction;
use crate::tokenomics::{BURN_ADDRESS, DEV_WALLET};

/// Score a batch of transactions for a candidate block
/// Returns a risk score 0–100 (100 = perfectly safe)
pub fn score_transactions(txs: &[Transaction]) -> u32 {
    if txs.is_empty() { return 100; }

    let mut deductions: u32 = 0;

    // Rule 1: Any TX to burn address counts against score
    let burn_txs = txs.iter().filter(|tx| tx.to == BURN_ADDRESS).count();
    deductions += (burn_txs as u32).min(10) * 2;

    // Rule 2: Very high value single TX (whale alert)
    let max_amount = txs.iter().map(|tx| tx.amount).max().unwrap_or(0);
    if max_amount > 1_000_000 * 1_000_000 {  // > 1M DVC
        deductions += 15;
    }

    // Rule 3: Duplicate sender flooding (spam detection)
    let mut sender_counts: std::collections::HashMap<&str, usize> = std::collections::HashMap::new();
    for tx in txs { *sender_counts.entry(&tx.from).or_insert(0) += 1; }
    let max_from_one = sender_counts.values().max().copied().unwrap_or(0);
    if max_from_one > 5 {
        deductions += ((max_from_one - 5) as u32).min(20) * 2;
    }

    // Rule 4: Zero-amount transactions
    let zero_amt = txs.iter().filter(|tx| tx.amount == 0).count();
    deductions += (zero_amt as u32) * 5;

    // Rule 5: Gas fee too low (potential spam)
    let low_fee = txs.iter()
        .filter(|tx| tx.gas_fee < crate::tokenomics::MIN_GAS_FEE * 2)
        .count();
    deductions += (low_fee as u32).min(10);

    (100u32).saturating_sub(deductions)
}

/// Score a single transaction
pub fn score_transaction(tx: &Transaction) -> u32 {
    score_transactions(std::slice::from_ref(tx))
}
