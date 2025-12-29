use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::process::Command;
use serde::{Deserialize, Serialize};
use ed25519_dalek::{SigningKey, Signer, Signature};
use crate::VaultEvent;

#[derive(Debug, Serialize, Deserialize, Clone)]
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

    pub async fn execute_remediation(
        &self,
        request: RemediationRequest,
        script_path: &str,
    ) -> anyhow::Result<String> {
        let tx = self.state.read().await.event_tx.clone();
        let _ = tx.send(VaultEvent::RemediationAttempted { 
            trace_id: request.trace_id.clone(), 
            failure_id: request.failure_id.clone() 
        });

        let output = Command::new("bash")
            .arg(script_path)
            .arg(&request.trace_id)
            .arg(&request.failure_id)
            .output()
            .await?;

        if !output.status.success() {
            let err = String::from_utf8_lossy(&output.stderr).to_string();
            let _ = tx.send(VaultEvent::RemediationRefused { 
                trace_id: request.trace_id.clone(), 
                reason: err.clone() 
            });
            anyhow::bail!("Playbook failed: {}", err);
        }

        let _ = tx.send(VaultEvent::RemediationExecuted { trace_id: request.trace_id.clone() });

        let state_lock = self.state.read().await;
        let signature = self.sign_receipt(&request, &state_lock.signing_key)?;
        
        let _ = tx.send(VaultEvent::ReceiptGenerated { 
            trace_id: request.trace_id.clone(), 
            signature: signature.clone() 
        });

        Ok(signature)
    }

    fn sign_receipt(&self, request: &RemediationRequest, key_bytes: &[u8]) -> anyhow::Result<String> {
        let signing_key = SigningKey::from_bytes(
            key_bytes.get(..32)
                .ok_or_else(|| anyhow::anyhow!("Invalid key length"))?
                .try_into()?
        );
        let signature: Signature = signing_key.sign(request.trace_id.as_bytes());
        Ok(hex::encode(signature.to_bytes()))
    }
}
