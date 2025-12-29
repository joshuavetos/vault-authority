Vault Authority v1.1 — Autonomous, Fail-Closed Remediation

Stop fixing the same failures twice.
Vault Authority detects known infrastructure failures, fixes them automatically, and emits cryptographic proof that the fix actually happened.

No tickets. No 3:00 AM pages. No manual compliance logging.

⸻

Why Teams Use This

Problem	Traditional Automation	Vault Authority
Token expiration	Manual refresh every time	Auto-remediated with signed receipt
Rate limits / spikes	On-call scramble + config edits	One-step adjustment + audit log
Known DB locks / zombies	DBA intervention	Automated cleanup, zero page alerts
Compliance proof	Screenshots in Slack threads	Ed25519 receipts and immutable logs

If an action fails: no receipt, no mutation.
If a receipt exists: the action succeeded.

⸻

Core Guarantee: Execution Cannot Lie

validate → check → execute → commit → sign → persist
(failure at any step aborts immediately with zero state change)

Invariants
   •   INV-1 Enum Gating — Only approved failure types execute
   •   INV-2 Atomicity — No partial mutation, ever
   •   INV-3 Boundary Control — Controlled executor boundary
   •   INV-4 Idempotency — Duplicate incidents rejected before execution

⸻

Red-Team Verification (RT-05)

These screenshots are taken directly from the adversarial test suite proving the invariant.

Failure Before Fix — Incorrect Behavior (Bug State)

Execution failed, but a receipt existed. This violated atomicity and was rejected.

Pass After Fix — Correct Fail-Closed Behavior

Execution failure produces no receipt and no state mutation. Invariant restored.

Run locally with:
cargo test rt_05

⸻

Quickstart

git clone https://github.com/your-org/vault-authority
cd vault-authority
cargo test redteam
cargo build –release

Integrate from your monitoring or alerting system:

use vault_authority::{Vault, ShellExecutor};

let vault = Vault::new();
let executor = ShellExecutor;

vault.remediate(
“incident-1234”,
“ERR_AUTH_EXPIRED”,
&executor
)?;

⸻

What This Is / What This Isn’t

This is
   •   A deterministic remediation gate
   •   Cryptographically auditable execution
   •   Fail-closed by construction
   •   Red-team verified

This is not
   •   A workflow engine
   •   A YAML sprawl
   •   Best-effort automation
   •   An LLM agent

⸻

License

MIT — use, modify, and deploy freely.

⸻

Vault Authority exists to make repeatable failures boring, provable, and safe.
