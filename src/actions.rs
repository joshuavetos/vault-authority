use async_trait::async_trait;
use serde_json::Value;
use std::process::Command;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ExecError {
    #[error("execution failed: {0}")]
    ExecutionFailed(String),

    #[error("hardware or OS error")]
    HardwareFailure,
}

#[async_trait]
pub trait ActionExecutor: Send + Sync {
    async fn execute(&self, action: &str, params: Value) -> Result<(), ExecError>;
}

/// Concrete executor (not used by MVP tests, but real)
pub struct ShellExecutor;

#[async_trait]
impl ActionExecutor for ShellExecutor {
    async fn execute(&self, action: &str, params: Value) -> Result<(), ExecError> {
        let status = Command::new(action)
            .arg(params.to_string())
            .status()
            .map_err(|_| ExecError::HardwareFailure)?;

        if status.success() {
            Ok(())
        } else {
            Err(ExecError::ExecutionFailed(format!(
                "exit code: {:?}",
                status.code()
            )))
        }
    }
}

/// Test-only executor
#[cfg(test)]
pub struct MockExecutor {
    pub should_fail: bool,
}

#[cfg(test)]
#[async_trait]
impl ActionExecutor for MockExecutor {
    async fn execute(&self, _action: &str, _params: Value) -> Result<(), ExecError> {
        if self.should_fail {
            Err(ExecError::ExecutionFailed("simulated failure".into()))
        } else {
            Ok(())
        }
    }
}
