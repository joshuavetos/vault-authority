# Security & Safety Model — Vault Authority v1.1

## 1. Structural Invariants (SysDNA)
The system is physically constrained by four mandatory invariants to prevent runaway automation:
* **INV-1 (Enum Gating)**: Only failures explicitly listed in the taxonomy can trigger an action. Intent smuggling is rejected at the gate.
* **INV-2 (Atomic Execution)**: Remediation scripts must exit with code 0 before any state is mutated or a receipt is signed. Failure results in immediate termination.
* **INV-3 (Fail-Closed)**: If any step in the 6-step loop fails, the system aborts. No "partial" success is possible.
* **INV-4 (Idempotency)**: Every `trace_id` is logged in a persistent deduplication store. The same incident cannot trigger a fix twice.

## 2. Cryptographic Identity & Injection
* **Algorithm**: Ed25519 (256-bit Edwards-curve Digital Signature Algorithm).
* **Injection**: Secrets are never stored in the repository. They are injected via **Secret Store CSI Driver** from Google Secret Manager.
* **Hot-Reloading**: The `src/main.rs` watcher ensures that rotated keys are updated in-memory every 30 seconds without process restarts.
* **Verification**: Auditors use the public key to verify that a "Receipt of Safety" was signed by the authorized Vault instance.

## 3. Threat Model & Mitigation
* **Replay Attacks**: Prevented by the INV-4 deduplication registry.
* **Unauthorized Access**: Mitigated by **Workload Identity**, ensuring only the authorized Kubernetes Service Account can access signing material.
* **Privilege Escalation**: Remediation scripts run with restricted permissions in a non-blocking shell environment isolated from the main application thread.
