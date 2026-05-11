//! DevilChain Mempool — nonce replay protection, fee-priority ordering
//! Developed by Nexuzy Lab (nexuzy.tech) | Powered by Devil One (devilone.in)

use std::collections::HashMap;
use crate::blockchain::Transaction;
use crate::tokenomics::MIN_GAS_FEE;

#[derive(Debug)]
pub struct Mempool {
    transactions: HashMap<String, Transaction>,
    addr_nonces:  HashMap<String, u64>,
    max_size:     usize,
}

impl Default for Mempool {
    fn default() -> Self { Self::new(500) }
}

impl Mempool {
    pub fn new(max_size: usize) -> Self {
        Self { transactions: HashMap::new(), addr_nonces: HashMap::new(), max_size }
    }

    pub fn add_transaction(&mut self, tx: Transaction) -> Result<(), &'static str> {
        if self.transactions.len() >= self.max_size { return Err("Mempool full"); }
        if self.transactions.contains_key(&tx.tx_hash) { return Err("Duplicate TX"); }
        if tx.gas_fee < MIN_GAS_FEE { return Err("Gas fee too low"); }  // u128 ✅
        let last = *self.addr_nonces.get(&tx.from).unwrap_or(&0);
        if tx.nonce < last { return Err("Nonce too low (replay)"); }
        if !tx.is_valid() { return Err("Invalid TX signature"); }
        self.addr_nonces.insert(tx.from.clone(), tx.nonce);
        self.transactions.insert(tx.tx_hash.clone(), tx);
        Ok(())
    }

    /// Pop up to n highest-fee transactions (miners earn more)
    pub fn pop_transactions(&mut self, n: usize) -> Vec<Transaction> {
        let mut txs: Vec<Transaction> = self.transactions.values().cloned().collect();
        txs.sort_by(|a, b| b.gas_fee.cmp(&a.gas_fee));  // u128 cmp ✅
        txs.truncate(n);
        for tx in &txs { self.transactions.remove(&tx.tx_hash); }
        txs
    }

    pub fn len(&self)      -> usize { self.transactions.len() }
    pub fn is_empty(&self) -> bool  { self.transactions.is_empty() }
    pub fn peek(&self)     -> Vec<&Transaction> { self.transactions.values().collect() }
}
