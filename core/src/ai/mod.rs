//! DevilGuard AI — rule-based risk scoring, no hardcoded stubs
//! All comparisons use u128 (matches Transaction.gas_fee type)
//! Developed by Nexuzy Lab (nexuzy.tech) | Powered by Devil One (devilone.in)

use crate::blockchain::Transaction;
use crate::tokenomics::{BURN_ADDRESS, MIN_GAS_FEE};

/// Score a block's transactions 0–100 (higher = safer). Rejected if < 50.
pub fn score_transactions(txs: &[Transaction]) -> u32 {
    if txs.is_empty() { return 100; }
    let mut deductions: u32 = 0;

    // Rule 1: burn-address sends
    let burn = txs.iter().filter(|tx| tx.to == BURN_ADDRESS).count();
    deductions += (burn as u32).min(10) * 2;

    // Rule 2: whale TX (> 1M DVC)
    let max_amt = txs.iter().map(|tx| tx.amount).max().unwrap_or(0);
    if max_amt > 1_000_000 * 1_000_000u128 { deductions += 15; }

    // Rule 3: sender flood (same address > 5 txs in one block)
    let mut counts: std::collections::HashMap<&str, usize> = Default::default();
    for tx in txs { *counts.entry(&tx.from).or_insert(0) += 1; }
    let max_flood = counts.values().copied().max().unwrap_or(0);
    if max_flood > 5 {
        deductions += ((max_flood - 5) as u32).min(20) * 2;
    }

    // Rule 4: zero-amount txs
    deductions += (txs.iter().filter(|tx| tx.amount == 0).count() as u32) * 5;

    // Rule 5: low gas fee (u128 comparison — no type mismatch)
    let low_fee = txs.iter()
        .filter(|tx| tx.gas_fee < MIN_GAS_FEE * 2)  // u128 * u128 ✅
        .count();
    deductions += (low_fee as u32).min(10);

    (100u32).saturating_sub(deductions)
}

pub fn score_transaction(tx: &Transaction) -> u32 {
    score_transactions(std::slice::from_ref(tx))
}
