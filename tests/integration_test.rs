use std::sync::Arc;
use tokio::fs;
use tokio::sync::RwLock;
use vault_authority::{AppState, remediation}; // Assumes lib.rs structure

#[tokio::test]
async fn test_full_remediation_lifecycle_with_reload() -> anyhow::Result<()> {
    // 1. Setup Temporary Environment (INV-3: Path Invariance)
    let temp_dir = tempfile::tempdir()?;
    let key_path = temp_dir.path().join("private.key");
    let script_path = temp_dir.path().join("test_remedy.sh");

    // Create Initial Key
    let initial_key = vec![0u8; 32];
    fs::write(&key_path, &initial_key).await?;

    // Create Mock Playbook (INV-3: Boundary Control)
    let script_content = "#!/bin/bash\necho \"Remediating $1\"\nexit 0";
    fs::write(&script_path, script_content).await?;
    std::process::Command::new("chmod").arg("+x").arg(&script_path).status()?;

    // 2. Initialize State and Engine (INV-1: Sequential)
    let state = Arc::new(RwLock::new(AppState {
        signing_key: initial_key.clone(),
    }));
    let engine = remediation::RemediationEngine::new(Arc::clone(&state));

    // 3. Execute First Remediation (Atomic Verification)
    let request = remediation::RemediationRequest {
        trace_id: "test-trace-001".to_string(),
        failure_id: "ERR_LOAD_001".to_string(),
        payload: serde_json::json!({}),
    };

    let receipt_1 = engine.execute_remediation(request.clone(), script_path.to_str().unwrap()).await?;
    assert!(!receipt_1.is_empty(), "Receipt should be generated on success");

    // 4. Simulate Hot-Reload (CSI-style Update)
    let new_key = vec![1u8; 32];
    fs::write(&key_path, &new_key).await?;
    
    // Manually trigger reload to simulate the watcher's action
    {
        let mut lock = state.write().await;
        lock.signing_key = new_content = fs::read(&key_path).await?;
    }

    // 5. Verify Cryptographic Transition (INV-2: Atomic)
    let receipt_2 = engine.execute_remediation(request, script_path.to_str().unwrap()).await?;
    assert_ne!(receipt_1, receipt_2, "Receipts must differ after key rotation");

    println!("✅ Integration Test Passed: Hot-reloading and execution validated.");
    Ok(())
}
