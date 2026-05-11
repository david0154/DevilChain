use crate::blockchain::Transaction;
use std::collections::VecDeque;

pub struct Mempool {
    pub pending: VecDeque<Transaction>,
    pub max_size: usize,
}

impl Mempool {
    pub fn new() -> Self {
        Mempool {
            pending: VecDeque::new(),
            max_size: 10000,
        }
    }

    pub fn add_transaction(&mut self, tx: Transaction) -> bool {
        if self.pending.len() >= self.max_size {
            return false;
        }
        // Basic spam filter: min gas fee
        if tx.gas_fee < 0.001 {
            return false;
        }
        self.pending.push_back(tx);
        true
    }

    pub fn get_transactions(&mut self, limit: usize) -> Vec<Transaction> {
        let mut batch = Vec::new();
        for _ in 0..limit {
            if let Some(tx) = self.pending.pop_front() {
                batch.push(tx);
            } else {
                break;
            }
        }
        batch
    }

    pub fn size(&self) -> usize {
        self.pending.len()
    }
}
