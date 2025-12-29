use serde::{Deserialize, Serialize};
use tokio::sync::broadcast;

pub mod remediation;

pub const UI_EVENT_BUFFER: usize = 100;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum VaultEvent {
    RemediationAttempted { trace_id: String, failure_id: String },
    RemediationExecuted { trace_id: String },
    RemediationCommitted { trace_id: String },
    ReceiptGenerated { trace_id: String, signature: String },
    RemediationRefused { trace_id: String, reason: String },
}

pub struct AppState {
    pub signing_key: Vec<u8>,
    pub event_tx: broadcast::Sender<VaultEvent>,
}
