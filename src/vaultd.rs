use crate::actions::ActionExecutor;
use ed25519_dalek::{SigningKey, Signer};
use once_cell::sync::Lazy;
use rand::rngs::OsRng;
use serde::Serialize;
use serde_json::json;
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};
use thiserror::Error;

static ALLOWED_FAILURES: Lazy<HashSet<String>> = Lazy::new(|| {
    ["ERR_AUTH_EXPIRED", "ERR_RATE_LIMIT"]
        .iter()
        .map(|s| s.to_string())
        .collect()
});

#[derive(Debug, Error)]
pub enum VaultError {
    #[error("invalid failure_id")]
    InvalidFailureID,

    #[error("duplicate trace_id")]
    DuplicateTrace,

    #[error("execution failed")]
    ExecutionFailed,

    #[error("internal error")]
    Internal,
}

#[derive(Serialize, Clone)]
pub struct Receipt {
    pub trace_id: String,
    pub failure_id: String,
    pub signature: Vec<u8>,
}

struct State {
    dedupe: HashSet<String>,
    audit: HashMap<String, Receipt>,
    signer: SigningKey,
}

#[derive(Clone)]
pub struct Vault {
    state: Arc<Mutex<State>>,
}

impl Vault {
    pub fn new() -> Self {
        let signer = SigningKey::generate(&mut OsRng);
        Self {
            state: Arc::new(Mutex::new(State {
                dedupe: HashSet::new(),
                audit: HashMap::new(),
                signer,
            })),
        }
    }

    /// Canonical MVP remediation function
    pub async fn remediate(
        &self,
        trace_id: &str,
        failure_id_raw: &str,
        executor: &dyn ActionExecutor,
    ) -> Result<Receipt, VaultError> {
        // 1. ENUM GATE (INV-1)
        if !ALLOWED_FAILURES.contains(failure_id_raw) {
            return Err(VaultError::InvalidFailureID);
        }

        // 2. DEDUPE READ (INV-4)
        {
            let state = self.state.lock().unwrap();
            if state.dedupe.contains(trace_id) {
                return Err(VaultError::DuplicateTrace);
            }
        }

        // 3. EXECUTION (FALLIBLE)
        executor
            .execute(failure_id_raw, json!({ "trace_id": trace_id }))
            .await
            .map_err(|_| VaultError::ExecutionFailed)?;

        // 4. COMMIT POINT (NO RETURN)
        let mut state = self.state.lock().unwrap();
        state.dedupe.insert(trace_id.to_string());

        // 5. SIGNATURE
        let payload = format!("{}:{}", trace_id, failure_id_raw);
        let sig = state.signer.sign(payload.as_bytes()).to_bytes().to_vec();

        let receipt = Receipt {
            trace_id: trace_id.to_string(),
            failure_id: failure_id_raw.to_string(),
            signature: sig,
        };

        // 6. PERSISTENCE
        state.audit.insert(trace_id.to_string(), receipt.clone());

        Ok(receipt)
    }

    // --- Test helpers ---

    pub fn has_receipt(&self, trace_id: &str) -> bool {
        self.state.lock().unwrap().audit.contains_key(trace_id)
    }

    pub fn has_dedupe(&self, trace_id: &str) -> bool {
        self.state.lock().unwrap().dedupe.contains(trace_id)
    }
}
