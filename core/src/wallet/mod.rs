//! DevilChain Wallet — real Ed25519 (OsRng), sign/verify, miner identity
//! Developed by Nexuzy Lab (nexuzy.tech) | Powered by Devil One (devilone.in)

use ed25519_dalek::{SigningKey, VerifyingKey, Signer};
use rand::rngs::OsRng;    // ✅ cryptographically secure
use sha2::{Sha256, Digest};
use serde::{Serialize, Deserialize};

const WORDS: &[&str] = &[
    "abandon","ability","able","about","above","absent","absorb","abstract",
    "absurd","abuse","access","accident","account","accuse","achieve","acid",
    "acoustic","acquire","across","act","action","actor","actress","actual",
    "adapt","add","addict","address","adjust","admit","adult","advance",
    "advice","aerobic","affair","afford","afraid","again","age","agent",
    "agree","ahead","aim","air","airport","aisle","alarm","album",
    "alcohol","alert","alien","all","alley","allow","almost","alone",
    "alpha","already","also","alter","always","amateur","amazing","among",
];

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletKeypair {
    pub address:     String,
    pub public_key:  String,  // hex(32 bytes)
    pub private_key: String,  // hex(32 bytes seed) — never expose externally
    pub mnemonic:    String,  // 12 words
    pub balance:     u128,    // µDVC, filled from ledger
    pub nonce:       u64,
}

impl WalletKeypair {
    /// Generate new wallet using OsRng — cryptographically secure
    pub fn generate() -> Self {
        let sk  = SigningKey::generate(&mut OsRng);     // ✅ OsRng
        let vk  = sk.verifying_key();
        let addr_hash = Sha256::digest(vk.as_bytes());
        Self {
            address:     format!("db1x{}", &hex::encode(addr_hash)[..32]),
            public_key:  hex::encode(vk.as_bytes()),
            private_key: hex::encode(sk.as_bytes()),
            mnemonic:    Self::derive_mnemonic(sk.as_bytes()),
            balance: 0, nonce: 0,
        }
    }

    /// Restore from private key hex
    pub fn from_private_key(priv_hex: &str) -> Result<Self, &'static str> {
        let b: [u8; 32] = hex::decode(priv_hex)
            .map_err(|_| "Bad hex")?
            .try_into().map_err(|_| "Key must be 32 bytes")?;
        let sk  = SigningKey::from_bytes(&b);
        let vk  = sk.verifying_key();
        let addr_hash = Sha256::digest(vk.as_bytes());
        Ok(Self {
            address:     format!("db1x{}", &hex::encode(addr_hash)[..32]),
            public_key:  hex::encode(vk.as_bytes()),
            private_key: priv_hex.to_string(),
            mnemonic:    Self::derive_mnemonic(&b),
            balance: 0, nonce: 0,
        })
    }

    /// Sign a message → hex(Ed25519 sig)
    pub fn sign(&self, msg: &str) -> Result<String, &'static str> {
        let b: [u8; 32] = hex::decode(&self.private_key)
            .map_err(|_| "Bad key")?
            .try_into().map_err(|_| "Key len")?;
        let sk  = SigningKey::from_bytes(&b);
        Ok(hex::encode(sk.sign(msg.as_bytes()).to_bytes()))
    }

    fn derive_mnemonic(seed: &[u8]) -> String {
        let n = WORDS.len();
        (0..12).map(|i| {
            WORDS[(seed[i % seed.len()] as usize
                ^ (seed[(i+1) % seed.len()] as usize) << 2) % n]
        }).collect::<Vec<_>>().join(" ")
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinerIdentity {
    pub address:      String,
    pub public_key:   String,
    pub name:         String,
    pub pool_address: String,
    pub stake:        u128,
    pub blocks_mined: u64,
    pub total_earned: u128,
    pub registered_at: u64,
}

impl MinerIdentity {
    pub fn new(wallet: &WalletKeypair, name: &str) -> Self {
        Self {
            address:      wallet.address.clone(),
            public_key:   wallet.public_key.clone(),
            name:         name.to_string(),
            pool_address: crate::tokenomics::MINING_POOL_WALLET.to_string(),
            stake: 0, blocks_mined: 0, total_earned: 0,
            registered_at: now_secs(),
        }
    }
}

pub fn now_secs() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs()).unwrap_or(0)
}
