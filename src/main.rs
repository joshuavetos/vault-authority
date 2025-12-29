use std::sync::Arc;
use tokio::fs;
use tokio::sync::RwLock;
use tokio::time::{sleep, Duration};

// Import from the library crate (vault_authority)
use vault_authority::{AppState, remediation};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 1. Configuration & Path Invariance (INV-3)
    let key_path = std::env::var("VAULT_KEY_PATH")
        .unwrap_or_else(|_| "/etc/vault/keys/private.key".to_string());

    println!("🚀 Vault Authority v1.1 initializing...");

    // 2. Initial Sequential Load (INV-1)
    let initial_key = fs::read(&key_path).await
        .map_err(|e| anyhow::anyhow!("Failed to read initial key at {}: {}", key_path, e))?;
    
    let state = Arc::new(RwLock::new(AppState {
        signing_key: initial_key,
    }));

    // 3. Initialize Engines from Library
    let engine = remediation::RemediationEngine::new(Arc::clone(&state));

    // 4. Spawn Background Key-Watcher
    let watcher_state = Arc::clone(&state);
    let watcher_path = key_path.clone();
    
    tokio::spawn(async move {
        let mut last_content = watcher_state.read().await.signing_key.clone();
        
        loop {
            sleep(Duration::from_secs(30)).await; 
            
            if let Ok(new_content) = fs::read(&watcher_path).await {
                if new_content != last_content {
                    let mut lock = watcher_state.write().await;
                    lock.signing_key = new_content.clone();
                    last_content = new_content;
                    println!("🔐 HOT-RELOAD: New Ed25519 signing key active.");
                }
            }
        }
    });

    println!("✅ Runtime established. Key path: {}", key_path);

    // Keep the main thread alive for the API/Remediation engine
    tokio::signal::ctrl_c().await?;
    println!("🛑 Shutdown signal received. Terminating Vault Authority.");

    Ok(())
}
