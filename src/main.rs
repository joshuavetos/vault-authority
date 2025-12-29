mod actions;
mod vaultd;

use vaultd::{Vault};
use actions::ShellExecutor;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: vault-auth <trace_id> <failure_id>");
        std::process::exit(1);
    }

    let vault = Vault::new();
    let executor = ShellExecutor;

    match vault.remediate(args[1].clone(), args[2].clone(), &executor).await {
        Ok(receipt) => {
            println!("SUCCESS: Receipt {}", receipt.signature);
            Ok(())
        }
        Err(e) => {
            eprintln!("REJECTED: {}", e);
            std::process::exit(1)
        }
    }
}
