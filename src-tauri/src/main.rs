#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;
use vault_authority::AppState;
use std::sync::Arc;
use tokio::sync::RwLock;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let handle = app.handle();
            let state = app.state::<Arc<RwLock<AppState>>>(); 
            let mut event_rx = state.blocking_read().event_tx.subscribe();

            tauri::async_runtime::spawn(async move {
                let window = handle.get_window("main").expect("Main window not found");
                while let Ok(event) = event_rx.recv().await {
                    let _ = window.emit("vault_event", event);
                }
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
