//! DevilChain RocksDB Persistent Storage Layer

use rocksdb::{DB, Options, IteratorMode};
use serde::{Serialize, de::DeserializeOwned};
use std::path::Path;

pub struct ChainDB {
    db: DB,
}

impl ChainDB {
    pub fn open(path: &str) -> anyhow::Result<Self> {
        let mut opts = Options::default();
        opts.create_if_missing(true);
        opts.set_compression_type(rocksdb::DBCompressionType::Lz4);
        let db = DB::open(&opts, Path::new(path))?;
        Ok(ChainDB { db })
    }

    /// Store any serializable value
    pub fn put<V: Serialize>(&self, key: &str, value: &V) -> anyhow::Result<()> {
        let encoded = serde_json::to_vec(value)?;
        self.db.put(key.as_bytes(), encoded)?;
        Ok(())
    }

    /// Get and deserialize value
    pub fn get<V: DeserializeOwned>(&self, key: &str) -> anyhow::Result<Option<V>> {
        match self.db.get(key.as_bytes())? {
            Some(bytes) => Ok(Some(serde_json::from_slice(&bytes)?)),
            None => Ok(None),
        }
    }

    /// Delete a key
    pub fn delete(&self, key: &str) -> anyhow::Result<()> {
        self.db.delete(key.as_bytes())?;
        Ok(())
    }

    /// Store block by height
    pub fn put_block(&self, block: &crate::blockchain::Block) -> anyhow::Result<()> {
        let key = format!("block:{}", block.block_height);
        self.put(&key, block)?;
        self.put("chain:latest_height", &block.block_height)?;
        self.put(&format!("blockhash:{}", block.block_hash), &block.block_height)?;
        Ok(())
    }

    /// Get block by height
    pub fn get_block(&self, height: u64) -> anyhow::Result<Option<crate::blockchain::Block>> {
        self.get(&format!("block:{}", height))
    }

    /// Get latest block height
    pub fn get_latest_height(&self) -> anyhow::Result<u64> {
        Ok(self.get::<u64>("chain:latest_height")?.unwrap_or(0))
    }

    /// Store transaction
    pub fn put_tx(&self, tx: &crate::blockchain::Transaction) -> anyhow::Result<()> {
        self.put(&format!("tx:{}", tx.tx_hash), tx)
    }

    /// Get transaction by hash
    pub fn get_tx(&self, hash: &str) -> anyhow::Result<Option<crate::blockchain::Transaction>> {
        self.get(&format!("tx:{}", hash))
    }

    /// Store wallet balance
    pub fn put_balance(&self, address: &str, balance: f64) -> anyhow::Result<()> {
        self.put(&format!("balance:{}", address), &balance)
    }

    /// Get wallet balance
    pub fn get_balance(&self, address: &str) -> anyhow::Result<f64> {
        Ok(self.get::<f64>(&format!("balance:{}", address))?.unwrap_or(0.0))
    }

    /// Update balance (add or subtract)
    pub fn update_balance(&self, address: &str, delta: f64) -> anyhow::Result<()> {
        let current = self.get_balance(address)?;
        let new_balance = (current + delta).max(0.0);
        self.put_balance(address, new_balance)
    }

    /// Iterate all keys with prefix
    pub fn scan_prefix(&self, prefix: &str) -> Vec<(String, Vec<u8>)> {
        let iter = self.db.iterator(IteratorMode::Start);
        iter.filter_map(|item| {
            if let Ok((k, v)) = item {
                let key_str = String::from_utf8_lossy(&k).to_string();
                if key_str.starts_with(prefix) {
                    return Some((key_str, v.to_vec()));
                }
            }
            None
        }).collect()
    }
}
