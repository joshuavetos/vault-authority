mod actions;
mod vaultd;

use vaultd::Vault;
use actions::ShellExecutor;
use std::env;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: vault-auth <trace_id> <failure_id>");
        return;
    }

    let vault = Vault::new();
    let executor = ShellExecutor;

    match vault.remediate(args[1].clone(), args[2].clone(), &executor).await {
        Ok(receipt) => println!("STATUS: EXECUTED | RECEIPT: {}", receipt.signature),
        Err(e) => eprintln!("STATUS: REJECTED | REASON: {}", e),
    }
}
