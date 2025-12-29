Vault Authority v1.0 — Deterministic Remediation Gate
Vault Authority is a fail-closed, deterministic remediation core written in Rust. It enforces safety by instruction ordering, not configuration, policy text, or operator discretion.
This repository is certified under the Partner Reliability Benchmark (PRB) v1.1.
What This System Does
Vault Authority accepts a (trace_id, failure_id) pair and attempts a remediation exactly once. A cryptographically signed receipt is emitted only if the failure is allowed, the action executes successfully, and state mutation occurs in the correct order.
If any step fails, no receipt exists and no state is mutated.
Core Properties
 * Fail-Closed — Success is provable; failure leaves no residue.
 * Atomic — Execution and state mutation are inseparable.
 * Idempotent — Duplicate trace_id values are physically rejected.
 * Auditable — Correctness is proven via adversarial tests and bitwise SHA256 verification.
Verification Standard: PRB v1.1
This repository adheres to the Norm-v1.1 normalization standard to ensure bitwise integrity of all remediation artifacts:
 * Artifact Integrity: Verified via SHA256 hash match of normalized outputs.
 * Temporal Determinism: 100% hash stability across intra-session runs.
 * Active Silence: Confirmed 0-token response for dormant commands.
Enforcement Model (Instruction Ordering)
The remediation path is strictly monotonic. If any step fails, the process terminates immediately.
 * Validate — Enum gate (INV-1).
 * Check — Dedupe read (INV-4).
 * Execute — External action (fallible).
 * Commit — Dedupe write (point of no return).
 * Sign — Cryptographic receipt.
 * Persist — Audit record.
Documentation & Compliance
 * PRB v1.1 Specification: The deterministic testing standard.
 * CISO Risk Memo: Liability vectors eliminated by this core.
 * Legal/Compliance Appendix: Control mapping for SOC2 and ISO 27001.
 * Certification Policy: Rules for 30-day version pinning.
Red-Team Verification (RT-05)
The test suite proves the invariant by attempting to break it.
❌ Failure Before Fix
A receipt existed even though execution failed — invariant violation.
✅ Pass After Fix
Execution failure produces no receipt and no state mutation.
Run Verification
Rust Suite:
cargo test

PRB v1.1 Bitwise Check:
./prb-check.sh "[output_string]" "[expected_hash]"

License
MIT
