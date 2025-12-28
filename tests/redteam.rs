use vault_authority::*;
use vault_authority::actions::MockExecutor;

#[tokio::test]
async fn rt_01_invalid_enum_rejected() {
    let vault = Vault::new();
    let exec = MockExecutor { should_fail: false };

    let result = vault
        .remediate("t1", "EXEC_RANSOMWARE", &exec)
        .await;

    assert!(matches!(result, Err(VaultError::InvalidFailureID)));
}

#[tokio::test]
async fn rt_02_duplicate_trace_rejected() {
    let vault = Vault::new();
    let exec = MockExecutor { should_fail: false };

    let first = vault
        .remediate("t2", "ERR_AUTH_EXPIRED", &exec)
        .await;
    assert!(first.is_ok());

    let second = vault
        .remediate("t2", "ERR_AUTH_EXPIRED", &exec)
        .await;
    assert!(matches!(second, Err(VaultError::DuplicateTrace)));
}

#[tokio::test]
async fn rt_05_exec_failure_produces_no_receipt() {
    let vault = Vault::new();
    let exec = MockExecutor { should_fail: true };

    let result = vault
        .remediate("t3", "ERR_AUTH_EXPIRED", &exec)
        .await;

    assert!(result.is_err());
    assert!(!vault.has_receipt("t3"));
    assert!(!vault.has_dedupe("t3"));
}
