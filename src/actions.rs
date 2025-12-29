use async_trait::async_trait;
use serde_json::Value;
use std::process::Command;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ExecError {
    #[error("Action failed during execution: {0}")]
    ExecutionFailed(String),
    #[error("Hardware or OS error")]
    HardwareFailure,
}

#[async_trait]
pub trait ActionExecutor: Send + Sync {
    async fn execute(&self, action_path: &str, params: Value) -> Result<(), ExecError>;
}

pub struct ShellExecutor;

#[async_trait]
impl ActionExecutor for ShellExecutor {
    async fn execute(&self, action_path: &str, params: Value) -> Result<(), ExecError> {
        let status = Command::new("sh")
            .arg("-c")
            .arg(format!("{} '{}'", action_path, params.to_string()))
            .status()
            .map_err(|_| ExecError::HardwareFailure)?;

        if status.success() {
            Ok(())
        } else {
            Err(ExecError::ExecutionFailed(format!("Exit code: {:?}", status.code())))
        }
    }
}
