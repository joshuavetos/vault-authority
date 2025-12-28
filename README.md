Vault Authority v1.0
Deterministic Remediation Gate
Vault Authority is a high-integrity Rust library designed to sit between a diagnostic layer and production environments. It replaces discretionary "script trust" with architectural ordering invariants, ensuring that automated actions are physically incapable of violating system safety.
Overview
Vault Authority v1.0 is an audited core focused on one guarantee: The system never lies.
 * Actionable: Accepts (trace_id, failure_id) for remediation.
 * Fail-Closed: Emits a signed receipt only after successful execution.
 * Atomic: If an action fails, no receipt exists and no state is mutated.
 * Audited: Proves correctness via an adversarial red-team test suite.
This is a library and test harness, not a daemon, HTTP service, or YAML engine.
Core Invariants
Safety is enforced by instruction ordering, not configuration:
 * INV-1 (Enum Gating): Only failures explicitly defined in the taxonomy can proceed.
 * INV-2 (Atomicity): If execution fails, state remains unchanged and no receipt is generated.
 * INV-3 (Gating): All external effects are restricted to a controlled executor boundary.
 * INV-4 (Idempotency): Duplicate executions for the same trace_id are physically rejected.
The Authority Loop
The remediation path is strictly monotonic. If any step fails, the process terminates before mutation:
 * Validate: Verify failure enum (INV-1).
 * Check: Read dedupe store (INV-4).
 * Execute: Attempt external action (Fallible).
 * Commit: Write dedupe key (Point of no return).
 * Sign: Generate cryptographic receipt.
 * Persist: Log the audit record.
Red-Team Verification
We prove safety through executable abuse. The following evidence demonstrates the transition from a vulnerable state to a hardened, fail-closed state.
Evidence: Failure Eliminated (The Receipt of Safety)
After enforcing correct ordering, the red-team suite confirms the system fails closed. No receipt is created and no dedupe key is written upon execution failure.
Repository Structure
A minimal, audited footprint designed for zero-drift instantiation:
vault-authority/
├── Cargo.toml      # Dependency locks (tokio, serde, ed25519-dalek)
├── src/
│   ├── lib.rs      # API surface
│   ├── vaultd.rs   # Core authority logic (Sealed)
│   └── actions.rs  # Execution boundary
├── tests/
│   └── redteam.rs  # Adversarial verification
└── docs/images/    # Proof of safety artifacts

Running the Proof
To verify the design contract and ensure zero false receipts, run:
cargo test

Execution Contract:
 * Success must be provable.
 * Failure must leave no residue.
 * History is append-only.
License
MIT
