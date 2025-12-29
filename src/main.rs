mod actions;
mod vaultd;
mod api;
mod metrics;

use vaultd::Vault;
use actions::ShellExecutor;
use std::env;
use std::sync::Arc;
use std::net::SocketAddr;
use tracing::{info, warn};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Initialize Logging & Metrics
    tracing_subscriber::fmt::init();
    info!("Vault Authority v1.1 — Initializing Production Gateway");

    // 2. Instantiate Shared Core
    let vault = Arc::new(Vault::new());
    let executor = ShellExecutor;

    // 3. Handle CLI Arguments (Manual Intervention Path)
    let args: Vec<String> = env::args().collect();
    if args.len() >= 3 {
        info!("Executing Manual CLI Remediation: Trace={}", args[1]);
        match vault.remediate(args[1].clone(), args[2].clone(), &executor).await {
            Ok(receipt) => {
                println!("STATUS: EXECUTED | RECEIPT: {}", receipt.signature);
                return Ok(());
            }
            Err(e) => {
                eprintln!("STATUS: REJECTED | REASON: {}", e);
                std::process::exit(1);
            }
        }
    }

    // 4. Start HTTP API Server (Remote Invocation Path)
    let app = api::app(vault.clone());
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    
    info!("HTTP API listening on {}", addr);
    
    // Axum server runner
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
