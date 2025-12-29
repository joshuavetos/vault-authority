use std::sync::Arc;
use tokio::fs;
use tokio::sync::{RwLock, broadcast};
use tokio::time::{sleep, Duration};
use vault_authority::{AppState, UI_EVENT_BUFFER};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let key_path = std::env::var("VAULT_KEY_PATH")
        .unwrap_or_else(|_| "/etc/vault/keys/private.key".to_string());

    let initial_key = fs::read(&key_path).await?;
    let (event_tx, _) = broadcast::channel(UI_EVENT_BUFFER);

    let state = Arc::new(RwLock::new(AppState {
        signing_key: initial_key,
        event_tx,
    }));

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
                }
            }
        }
    });

    tokio::signal::ctrl_c().await?;
    Ok(())
}
