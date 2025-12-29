Vault Authority v1.0 — Deterministic Remediation Gate
Vault Authority is a high-integrity Rust library that enforces fail-closed, atomic remediation through instruction ordering, not configuration.
It is designed to sit between a diagnostic layer and a production environment and make unsafe actions physically impossible to commit.
This repository contains only the audited core and its adversarial test harness.
What This Is
 * A library, not a daemon
 * No HTTP service
 * No YAML engine
 * No runtime configuration
 * No discretionary logic
Vault Authority accepts a (trace_id, failure_id) pair and either:
 * Executes safely and emits a signed receipt, or
 * Fails with no mutation, no receipt, no lie
The system never claims success unless success actually occurred.
Core Guarantees
 * Actionable Accepts (trace_id, failure_id) for remediation.
 * Fail-Closed A cryptographic receipt is emitted only after successful execution.
 * Atomic If execution fails, no receipt exists and no state is mutated.
 * Audited Correctness is proven via an adversarial red-team test suite.
Safety Model: Enforced by Ordering
Safety is enforced by instruction sequencing, not policy.
Invariants
 * INV-1 (Enum Gating) Only failures explicitly defined in the taxonomy can proceed.
 * INV-2 (Atomicity) If execution fails, state remains unchanged and no receipt is generated.
 * INV-3 (Effect Gating) All external effects are restricted to a controlled executor boundary.
 * INV-4 (Idempotency / Dedupe) Duplicate executions for the same trace_id are physically rejected.
The Authority Loop (Monotonic)
If any step fails, the process terminates before mutation.
 * Validate — Verify failure enum (INV-1)
 * Check — Read dedupe store (INV-4)
 * Execute — Attempt external action (fallible)
 * Commit — Write dedupe key (point of no return)
 * Sign — Generate cryptographic receipt
 * Persist — Write audit record
Red-Team Verification
Safety is proven by executable abuse.
Evidence: Failure Before Fix (Violation Detected)
This image shows the system incorrectly generating a receipt after a failed execution, violating INV-2 (Atomicity).
The red-team test rt_05_no_receipt_on_failure correctly fails.
Evidence: Failure Eliminated After Fix (Receipt of Safety)
This image shows the same test passing after the fix.
Execution failure produces no receipt and no state mutation, restoring atomicity.
What This Repository Is Not
 * Not an orchestrator
 * Not a workflow engine
 * Not an AI agent
 * Not a policy framework
 * Not configurable at runtime
This is a mechanical safety core.
Status
 * Core logic: Audited
 * Invariants: Proven
 * Red-team suite: Passing
 * Surface area: Intentionally minimal
Any extension must preserve instruction ordering and fail-closed guarantees.
License
MIT
