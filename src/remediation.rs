use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::process::Command;
use serde::{Deserialize, Serialize};
use ed25519_dalek::{SigningKey, Signer, Signature};

#[derive(Debug, Serialize, Deserialize)]
pub struct RemediationRequest {
    pub trace_id: String,
    pub failure_id: String,
    pub payload: serde_json::Value,
}

pub struct RemediationEngine {
    pub state: Arc<RwLock<crate::AppState>>,
}

impl RemediationEngine {
    pub fn new(state: Arc<RwLock<crate::AppState>>) -> Self {
        Self { state }
    }

    /// Executes a remediation playbook asynchronously and signs the result (INV-2: Atomic)
    pub async fn execute_remediation(
        &self,
        request: RemediationRequest,
        script_path: &str,
    ) -> anyhow::Result<String> {
        // 1. Execute the Playbook Non-blocking (INV-3: Boundary Control)
        let output = Command::new("bash")
            .arg(script_path)
            .arg(&request.trace_id)
            .arg(&request.failure_id)
            .output()
            .await?; // Non-blocking wait

        if !output.status.success() {
            let err = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("Playbook failed (F2: Partial): {}", err);
        }

        // 2. Generate Cryptographic Receipt using Hot-Reloaded Key
        let state_lock = self.state.read().await;
        let receipt = self.sign_receipt(&request, &state_lock.signing_key)?;

        Ok(receipt)
    }

    fn sign_receipt(&self, request: &RemediationRequest, key_bytes: &[u8]) -> anyhow::Result<String> {
        // Convert raw bytes to Ed25519 SigningKey (INV-1: Sequential Validation)
        let signing_key = SigningKey::from_bytes(
            key_bytes.get(..32)
                .ok_or_else(|| anyhow::anyhow!("Invalid key length"))?
                .try_into()?
        );

        // Sign the trace_id to provide proof of remediation
        let signature: Signature = signing_key.sign(request.trace_id.as_bytes());
        
        Ok(hex::encode(signature.to_bytes()))
    }
}
