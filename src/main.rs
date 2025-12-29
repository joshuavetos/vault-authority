use tokio::fs;
use tokio::time::{sleep, Duration};
use std::sync::Arc;
use tokio::sync::RwLock;

// Core State Container for Hot-Reloading
struct AppState {
    signing_key: Vec<u8>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let key_path = std::env::var("VAULT_KEY_PATH")
        .unwrap_or_else(|_| "/etc/vault/keys/private.key".to_string());

    // 1. Initial Load (INV-1: Sequential Startup)
    let initial_key = fs::read(&key_path).await?;
    let state = Arc::new(RwLock::new(AppState {
        signing_key: initial_key,
    }));

    println!("🚀 Vault Authority v1.1 started. Key loaded from {}", key_path);

    // 2. Spawn the Key Watcher Task
    let watcher_state = Arc::clone(&state);
    let watcher_path = key_path.clone();
    
    tokio::spawn(async move {
        let mut last_content = watcher_state.read().await.signing_key.clone();
        
        loop {
            sleep(Duration::from_secs(30)).await; // Poll interval for CSI sync
            
            if let Ok(new_content) = fs::read(&watcher_path).await {
                if new_content != last_content {
                    let mut lock = watcher_state.write().await;
                    lock.signing_key = new_content.clone();
                    last_content = new_content;
                    println!("🔐 Rotated: New signing key loaded from disk.");
                }
            }
        }
    });

    // 3. Start your API server (Placeholder for your actix/axum logic)
    // The server uses state.read().await.signing_key for every signature
    loop {
        tokio::signal::ctrl_c().await?;
        break;
    }

    Ok(())
}
