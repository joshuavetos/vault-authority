# Security & Safety Model — Vault Authority v1.0

## 1. Structural Invariants (SysDNA)
The system is physically constrained by four mandatory invariants to prevent runaway automation:
* **INV-1 (Enum Gating)**: Only failures explicitly listed in `playbook.yaml` can trigger an action. Intent smuggling (e.g., injection attacks) is rejected at the gate.
* **INV-2 (Atomic Execution)**: Remediation scripts must exit with code `0` before any state is mutated or a receipt is signed. Failure results in immediate termination.
* **INV-3 (Fail-Closed)**: If any step in the 6-step loop fails, the system aborts. No "partial" success is possible.
* **INV-4 (Idempotency)**: Every trace_id is logged in a persistent deduplication store. The same incident cannot trigger a fix twice.



## 2. Cryptographic Identity
* **Algorithm**: Ed25519 (256-bit Edwards-curve Digital Signature Algorithm).
* **Identity**: The Vault uses a unique private key generated during `setup.sh` to sign "Receipts of Safety".
* **Verification**: Auditors can use the public key and `prb-check.sh` to verify that a remediation was authorized by the Vault.

## 3. Threat Model & Mitigation
* **Replay Attacks**: Prevented by the INV-4 deduplication registry.
* **Privilege Escalation**: Remediation scripts run with restricted permissions (chmod 700) and must be explicitly mapped in the taxonomy.
