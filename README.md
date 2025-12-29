Vault Authority v1.0 — Deterministic Remediation Gate

Fail-closed remediation core for infrastructure automation.

Vault Authority is a minimal Rust library that prevents “automation that lies” by enforcing a strictly monotonic execution order: a success receipt cannot exist unless the action actually succeeded. If anything fails, nothing is committed and no receipt is produced.

What it does
   •   Accepts (trace_id, failure_id)
   •   Validates failure_id against an explicit taxonomy (enum-gated)
   •   Rejects duplicates before execution (idempotency)
   •   Executes via a controlled, mockable executor boundary
   •   Commits state only after successful execution
   •   Signs and persists a receipt only after commit

What it is NOT
   •   Not an agent framework
   •   Not an HTTP service / daemon
   •   Not a YAML workflow engine
   •   Not an LLM system
   •   Not “best effort” automation

The core guarantee

If a receipt exists → the action completed successfully (verifiable).
If no receipt exists → nothing happened.

No partial success.

Ordering (enforced, not promised)

validate → dedupe_read → execute → commit → sign → persist
Failure at any step aborts before mutation.

Evidence: RT-05 (no receipt on failure)

These screenshots are in the repo under images/ and should render in GitHub README:

Failure before fix (invariant violated)

Pass after fix (fail-closed restored)

Quick start

Build:

cargo build

Run adversarial suite:

cargo test redteam

If RT-05 fails, execution failures can produce false success receipts. Do not use until fixed.

Repository layout
   •   src/ — library core
   •   tests/redteam.rs — adversarial verification harness
   •   images/ — proof screenshots for RT-05

Audience

For SREs / infra engineers who have been burned by unprovable automation:
“Action attempted” ≠ “action completed.”

Vault Authority exists to make that distinction mechanically enforceable.

Status

Reference implementation. Study it. Fork it. Break it.

License

MIT License.
