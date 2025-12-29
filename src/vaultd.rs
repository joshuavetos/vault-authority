use crate::actions::{ActionExecutor};
use ed25519_dalek::{SigningKey, Signer};
use once_cell::sync::Lazy;
use rand::rngs::OsRng;
use serde::Serialize;
use serde_json::json;
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};
use thiserror::Error;

static FAILURE_TAXONOMY: Lazy<HashSet<&'static str>> = Lazy::new(|| {
    ["ERR_AUTH_EXPIRED", "ERR_RATE_LIMIT", "ERR_ZOMBIE_SESSION"].into_iter().collect()
});

#[derive(Debug, Error)]
pub enum VaultError {
    #[error("Invalid failure_id (INV-1)")]
    InvalidFailureID,
    #[error("Duplicate trace_id (INV-4)")]
    DuplicateTrace,
    #[error("Execution failed")]
    ExecutionFailed,
}

#[derive(Serialize, Clone)]
pub struct Receipt {
    pub trace_id: String,
    pub signature: String,
}

struct State {
    dedupe: HashSet<String>,
    signing_key: SigningKey,
}

#[derive(Clone)]
pub struct Vault {
    state: Arc<Mutex<State>>,
}

impl Vault {
    pub fn new() -> Self {
        Self {
            state: Arc::new(Mutex::new(State {
                dedupe: HashSet::new(),
                signing_key: SigningKey::generate(&mut OsRng),
            })),
        }
    }

    pub async fn remediate(
        &self,
        trace_id: String,
        failure_id: String,
        executor: &dyn ActionExecutor,
    ) -> Result<Receipt, VaultError> {
        if !FAILURE_TAXONOMY.contains(failure_id.as_str()) {
            return Err(VaultError::InvalidFailureID);
        }

        {
            let state = self.state.lock().unwrap();
            if state.dedupe.contains(&trace_id) {
                return Err(VaultError::DuplicateTrace);
            }
        }

        executor.execute("/bin/true", json!({"trace_id": trace_id}))
            .await
            .map_err(|_| VaultError::ExecutionFailed)?;

        let mut state = self.state.lock().unwrap();
        state.dedupe.insert(trace_id.clone());
        
        let msg = json!({"trace_id": trace_id, "failure_id": failure_id}).to_string();
        let sig = state.signing_key.sign(msg.as_bytes());

        Ok(Receipt {
            trace_id,
            signature: hex::encode(sig.to_bytes()),
        })
    }
}
