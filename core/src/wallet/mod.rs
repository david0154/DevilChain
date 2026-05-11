use sha2::{Digest, Sha256};
use rand::Rng;

/// DevilChain wallet address format: db1x...
pub struct Wallet {
    pub address: String,
    pub public_key: String,
    // private_key never stored in plain — use encrypted keystore
}

impl Wallet {
    pub fn generate() -> Self {
        let mut rng = rand::thread_rng();
        let random_bytes: Vec<u8> = (0..32).map(|_| rng.gen::<u8>()).collect();
        let hash = Sha256::digest(&random_bytes);
        let address = format!("db1x{}", hex::encode(&hash[..16]));
        let public_key = hex::encode(&hash);
        Wallet { address, public_key }
    }

    pub fn is_valid_address(addr: &str) -> bool {
        addr.starts_with("db1x") && addr.len() >= 20
    }
}
