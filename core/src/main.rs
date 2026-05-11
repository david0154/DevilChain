mod blockchain;
mod consensus;
mod mempool;
mod network;
mod wallet;
mod api;
mod ai;
mod dao;
mod mining;
mod validator;
mod storage;
mod graphql;

use std::sync::Arc;
use tokio::sync::RwLock;
use log::info;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "devilchain-node", about = "DevilChain Network Node", version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Start the DevilChain node
    Start {
        #[arg(long, default_value = "lite")]
        mode: String,
        #[arg(long, default_value = "/data/devilchain")]
        db_path: String,
    },
    /// Initialize node configuration
    Init {
        #[arg(long, default_value = "lite")]
        r#type: String,
    },
    /// Show node status
    Status,
    /// Generate a new wallet address
    GenWallet,
    /// Mine blocks manually (dev mode)
    Mine {
        #[arg(long)]
        wallet: String,
        #[arg(long, default_value = "4")]
        threads: u32,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();
    let cli = Cli::parse();

    match cli.command {
        Commands::Init { r#type } => {
            println!("🔥 Initializing DevilChain node (type: {})...", r#type);
            println!("Address prefix: db1x");
            println!("Consensus: Devil Hybrid Protocol (DHP)");
            println!("Config written to: /etc/devilchain/config.toml");
            println!("✅ Node initialized. Run: devilchain-node start");
        }

        Commands::GenWallet => {
            let w = wallet::Wallet::generate();
            println!("🔐 New DevilChain Wallet");
            println!("Address    : {}", w.address);
            println!("Public Key : {}", w.public_key);
            println!("⚠️  Back up your mnemonic phrase securely!");
        }

        Commands::Status => {
            println!("DevilChain Network Status");
            println!("Coin: DevilCoin (DVC) | Symbol: DVL");
            println!("Consensus: DHP (PoS + Micro PoW + DAO + AI)");
            println!("API: http://localhost:8545");
            println!("GraphQL: http://localhost:8546/graphql");
        }

        Commands::Mine { wallet, threads } => {
            println!("⛏️ DevilMine Engine started");
            println!("Wallet: {} | Threads: {} | Algorithm: DVLHash-AI", wallet, threads);
        }

        Commands::Start { mode, db_path } => {
            info!("🔥 DevilChain Network Node Starting...");
            info!("Mode: {} | DB: {}", mode, db_path);
            info!("Native Coin: DevilCoin (DVC) | Symbol: DVL");
            info!("Consensus: Devil Hybrid Protocol (DHP)");
            info!("REST API: :8545 | GraphQL: :8546");

            let blockchain = Arc::new(RwLock::new(blockchain::Blockchain::new()));
            let mempool = Arc::new(RwLock::new(mempool::Mempool::new()));

            let bc1 = Arc::clone(&blockchain);
            let bc2 = Arc::clone(&blockchain);
            let bc3 = Arc::clone(&blockchain);
            let bc4 = Arc::clone(&blockchain);
            let mp = Arc::clone(&mempool);

            tokio::select! {
                _ = api::start_api_server(bc1) => {},
                _ = graphql::start_graphql_server(bc2) => {},
                _ = network::start_p2p(bc3) => {},
                _ = mining::start_mining(bc4, mp) => {},
            }
        }
    }

    Ok(())
}
