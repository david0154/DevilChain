//! DevilChain RocksDB Persistent Storage Layer

use rocksdb::{DB, Options};
use serde::{Serialize, de::DeserializeOwned};
use std::path::Path;

pub struct ChainStorage {
    db: DB,
}

impl ChainStorage {
    pub fn open(path: &str) -> anyhow::Result<Self> {
        let mut opts = Options::default();
        opts.create_if_missing(true);
        opts.set_compression_type(rocksdb::DBCompressionType::Lz4);
        let db = DB::open(&opts, Path::new(path))?;
        Ok(ChainStorage { db })
    }

    pub fn put<T: Serialize>(&self, key: &str, value: &T) -> anyhow::Result<()> {
        let encoded = serde_json::to_vec(value)?;
        self.db.put(key.as_bytes(), encoded)?;
        Ok(())
    }

    pub fn get<T: DeserializeOwned>(&self, key: &str) -> anyhow::Result<Option<T>> {
        match self.db.get(key.as_bytes())? {
            Some(bytes) => Ok(Some(serde_json::from_slice(&bytes)?)),
            None => Ok(None),
        }
    }

    pub fn delete(&self, key: &str) -> anyhow::Result<()> {
        self.db.delete(key.as_bytes())?;
        Ok(())
    }

    pub fn put_block(&self, height: u64, block: &crate::blockchain::Block) -> anyhow::Result<()> {
        self.put(&format!("block:{}", height), block)?;
        self.put("chain:latest_height", &height)?;
        Ok(())
    }

    pub fn get_block(&self, height: u64) -> anyhow::Result<Option<crate::blockchain::Block>> {
        self.get(&format!("block:{}", height))
    }

    pub fn get_latest_height(&self) -> anyhow::Result<u64> {
        Ok(self.get::<u64>("chain:latest_height")?.unwrap_or(0))
    }

    pub fn put_tx(&self, hash: &str, tx: &crate::blockchain::Transaction) -> anyhow::Result<()> {
        self.put(&format!("tx:{}", hash), tx)
    }

    pub fn get_tx(&self, hash: &str) -> anyhow::Result<Option<crate::blockchain::Transaction>> {
        self.get(&format!("tx:{}", hash))
    }

    pub fn put_balance(&self, address: &str, balance: f64) -> anyhow::Result<()> {
        self.put(&format!("balance:{}", address), &balance)
    }

    pub fn get_balance(&self, address: &str) -> anyhow::Result<f64> {
        Ok(self.get::<f64>(&format!("balance:{}", address))?.unwrap_or(0.0))
    }
}
