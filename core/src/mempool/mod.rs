//! DevilChain Mempool
//! Fixed: nonce-based replay protection, no f64
//!
//! Developed by Nexuzy Lab (nexuzy.tech) | Powered by Devil One (devilone.in)

use std::collections::HashMap;
use crate::blockchain::Transaction;
use crate::tokenomics::MIN_GAS_FEE;

#[derive(Debug, Default)]
pub struct Mempool {
    /// Keyed by tx_hash for O(1) dedup
    transactions: HashMap<String, Transaction>,
    /// Track highest accepted nonce per address
    addr_nonces:  HashMap<String, u64>,
    max_size: usize,
}

impl Mempool {
    pub fn new(max_size: usize) -> Self {
        Self { transactions: HashMap::new(), addr_nonces: HashMap::new(), max_size }
    }

    pub fn add_transaction(&mut self, tx: Transaction) -> Result<(), &'static str> {
        if self.transactions.len() >= self.max_size {
            return Err("Mempool full");
        }
        // Dedup
        if self.transactions.contains_key(&tx.tx_hash) {
            return Err("Duplicate TX");
        }
        // Gas fee check
        if tx.gas_fee < MIN_GAS_FEE {
            return Err("Gas fee too low");
        }
        // Nonce: must be >= last seen nonce for this address
        let last = *self.addr_nonces.get(&tx.from).unwrap_or(&0);
        if tx.nonce < last {
            return Err("Nonce too low (replay attack)");
        }
        // Signature verification
        if !tx.is_valid() {
            return Err("Invalid transaction signature");
        }
        self.addr_nonces.insert(tx.from.clone(), tx.nonce);
        self.transactions.insert(tx.tx_hash.clone(), tx);
        Ok(())
    }

    /// Pop up to `n` highest-fee transactions for block inclusion
    pub fn pop_transactions(&mut self, n: usize) -> Vec<Transaction> {
        let mut txs: Vec<Transaction> = self.transactions.values().cloned().collect();
        // Sort by gas fee descending (miners prefer highest fee)
        txs.sort_by(|a, b| b.gas_fee.cmp(&a.gas_fee));
        txs.truncate(n);
        for tx in &txs {
            self.transactions.remove(&tx.tx_hash);
        }
        txs
    }

    pub fn len(&self) -> usize { self.transactions.len() }
    pub fn is_empty(&self) -> bool { self.transactions.is_empty() }
    pub fn peek(&self) -> Vec<&Transaction> { self.transactions.values().collect() }
}
