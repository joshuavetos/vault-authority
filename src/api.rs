use axum::{routing::post, Json, Router};
use serde::{Deserialize, Serialize};
use crate::vaultd::{Vault, VaultError};
use crate::actions::ShellExecutor;
use std::sync::Arc;

#[derive(Deserialize)]
pub struct RemediateReq {
    pub trace_id: String,
    pub failure_id: String,
}

#[derive(Serialize)]
pub struct RemediateResp {
    pub status: String,
    pub receipt: Option<String>,
    pub reason: Option<String>,
}

pub fn app(vault: Arc<Vault>) -> Router {
    Router::new().route("/remediate", post(move |Json(req): Json<RemediateReq>| {
        let vault = vault.clone();
        let executor = ShellExecutor;
        async move {
            match vault.remediate(req.trace_id, req.failure_id, &executor).await {
                Ok(receipt) => Json(RemediateResp {
                    status: "EXECUTED".into(),
                    receipt: Some(receipt.signature),
                    reason: None,
                }),
                Err(e) => Json(RemediateResp {
                    status: "REJECTED".into(),
                    receipt: None,
                    reason: Some(e.to_string()),
                }),
            }
        }
    }))
}
