//! DevilChain RocksDB Persistence Layer
//! Stores: blocks, transactions, wallet balances, validator state

use rocksdb::{DB, Options, ColumnFamilyDescriptor};
use serde::{Serialize, de::DeserializeOwned};
use crate::blockchain::{Block, Transaction};
use anyhow::Result;

const CF_BLOCKS: &str = "blocks";
const CF_TXS: &str = "transactions";
const CF_WALLETS: &str = "wallets";
const CF_VALIDATORS: &str = "validators";
const CF_META: &str = "meta";

pub struct ChainDB {
    db: DB,
}

impl ChainDB {
    pub fn open(path: &str) -> Result<Self> {
        let mut opts = Options::default();
        opts.create_if_missing(true);
        opts.create_missing_column_families(true);

        let cfs = vec![
            ColumnFamilyDescriptor::new(CF_BLOCKS, Options::default()),
            ColumnFamilyDescriptor::new(CF_TXS, Options::default()),
            ColumnFamilyDescriptor::new(CF_WALLETS, Options::default()),
            ColumnFamilyDescriptor::new(CF_VALIDATORS, Options::default()),
            ColumnFamilyDescriptor::new(CF_META, Options::default()),
        ];

        let db = DB::open_cf_descriptors(&opts, path, cfs)?;
        Ok(ChainDB { db })
    }

    fn put_cf<T: Serialize>(&self, cf: &str, key: &str, value: &T) -> Result<()> {
        let cf_handle = self.db.cf_handle(cf).ok_or_else(|| anyhow::anyhow!("CF not found: {}", cf))?;
        let encoded = serde_json::to_vec(value)?;
        self.db.put_cf(&cf_handle, key.as_bytes(), encoded)?;
        Ok(())
    }

    fn get_cf<T: DeserializeOwned>(&self, cf: &str, key: &str) -> Result<Option<T>> {
        let cf_handle = self.db.cf_handle(cf).ok_or_else(|| anyhow::anyhow!("CF not found: {}", cf))?;
        match self.db.get_cf(&cf_handle, key.as_bytes())? {
            Some(bytes) => Ok(Some(serde_json::from_slice(&bytes)?)),
            None => Ok(None),
        }
    }

    // --- Block operations ---
    pub fn save_block(&self, block: &Block) -> Result<()> {
        let key = block.block_height.to_string();
        self.put_cf(CF_BLOCKS, &key, block)?;
        // Also index by hash
        self.put_cf(CF_BLOCKS, &block.block_hash, block)?;
        // Update latest height
        self.put_cf(CF_META, "latest_height", &block.block_height)?;
        Ok(())
    }

    pub fn get_block_by_height(&self, height: u64) -> Result<Option<Block>> {
        self.get_cf(CF_BLOCKS, &height.to_string())
    }

    pub fn get_block_by_hash(&self, hash: &str) -> Result<Option<Block>> {
        self.get_cf(CF_BLOCKS, hash)
    }

    pub fn get_latest_height(&self) -> Result<u64> {
        Ok(self.get_cf::<u64>(CF_META, "latest_height")?.unwrap_or(0))
    }

    // --- Transaction operations ---
    pub fn save_transaction(&self, tx: &Transaction) -> Result<()> {
        self.put_cf(CF_TXS, &tx.tx_hash, tx)
    }

    pub fn get_transaction(&self, hash: &str) -> Result<Option<Transaction>> {
        self.get_cf(CF_TXS, hash)
    }

    // --- Wallet / balance operations ---
    pub fn get_balance(&self, address: &str) -> Result<f64> {
        Ok(self.get_cf::<f64>(CF_WALLETS, address)?.unwrap_or(0.0))
    }

    pub fn set_balance(&self, address: &str, balance: f64) -> Result<()> {
        self.put_cf(CF_WALLETS, address, &balance)
    }

    pub fn apply_transaction(&self, tx: &Transaction) -> Result<()> {
        let from_bal = self.get_balance(&tx.from)?;
        let total = tx.amount + tx.gas_fee;
        if from_bal < total {
            return Err(anyhow::anyhow!("Insufficient balance: {} < {}", from_bal, total));
        }
        self.set_balance(&tx.from, from_bal - total)?;
        let to_bal = self.get_balance(&tx.to)?;
        self.set_balance(&tx.to, to_bal + tx.amount)?;
        self.save_transaction(tx)?;
        Ok(())
    }

    // --- Network stats ---
    pub fn get_total_transactions(&self) -> Result<u64> {
        Ok(self.get_cf::<u64>(CF_META, "total_txs")?.unwrap_or(0))
    }

    pub fn increment_total_transactions(&self, count: u64) -> Result<()> {
        let current = self.get_total_transactions()?;
        self.put_cf(CF_META, "total_txs", &(current + count))
    }
}
