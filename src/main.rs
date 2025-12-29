use std::sync::Arc;
use tokio::fs;
use tokio::sync::RwLock;
use tokio::time::{sleep, Duration};

mod remediation; // Integration of the remediation logic

/// Global Application State for Hot-Reloading (INV-2: Atomic)
pub struct AppState {
    pub signing_key: Vec<u8>,
}

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

    // 3. Initialize Engines
    let engine = remediation::RemediationEngine::new(Arc::clone(&state));

    // 4. Spawn Background Key-Watcher (Hot-Reloading)
    let watcher_state = Arc::clone(&state);
    let watcher_path = key_path.clone();
    
    tokio::spawn(async move {
        let mut last_content = watcher_state.read().await.signing_key.clone();
        
        loop {
            // Poll interval for CSI sync (INV-4: Bounding)
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

    // 5. Execution Boundary (Placeholder for API / Webhook Listener)
    // The 'engine' variable is now ready to handle RemediationRequests
    // Example: engine.execute_remediation(req, "playbooks/token_refresh.sh").await;

    // Keep the main thread alive
    tokio::signal::ctrl_c().await?;
    println!("🛑 Shutdown signal received. Terminating Vault Authority.");

    Ok(())
}
