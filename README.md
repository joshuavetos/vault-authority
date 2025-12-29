<p align="center">
  <img src="images/va-logo2.PNG" alt="Vault Authority Logo" width="420">
</p>


<h1 align="center">Vault Authority v1.1</h1>
<p align="center"><strong>Autonomous, Fail-Closed Remediation</strong></p>
<p align="center">Cold, mechanical legitimacy.</p>


<p align="center">
  <code>cargo install vault-authority</code> • 
  <a href="#quickstart">Quick Start</a> • 
  <a href="#red-team-verification-rt-05">Proof</a>
</p>

⸻

Stop fixing the same failures twice.

Vault Authority detects known infrastructure failures, fixes them automatically, and emits cryptographic proof that the fix actually happened.
   •   No tickets
   •   No 3:00 AM pages
   •   No manual compliance logging
   •   No partial mutations or “best effort”

If an action fails: no receipt, no mutation.
If a receipt exists: the action succeeded.

⸻

Why Teams Use This

Problem	Traditional Automation	Vault Authority
Token expiration	Manual refresh every time	Auto-remediated with signed receipt
Rate limits / traffic spikes	On-call scramble + config edits	One-step adjustment + audit log
DB locks / zombie processes	DBA intervention	Automated cleanup, zero page alerts
Compliance proof	Slack screenshots & spreadsheets	Ed25519 receipts & immutable logs

Vault Authority is not fast — it is correct.
Speed is a side effect of not breaking.

⸻

Core Guarantee: Execution Cannot Lie

validate → check → execute → commit → sign → persist
(failure aborts instantly: zero state change)

Invariants
   •   INV-1 Enum Gating — Only approved failure types execute
   •   INV-2 Atomicity — No partial mutation, ever
   •   INV-3 Boundary Control — Controlled executor surface
   •   INV-4 Idempotency — Duplicate incidents rejected before execution

⸻

Red-Team Verification (RT-05)

These screenshots are pulled directly from the adversarial test suite.

Failure Before Fix — Incorrect Behavior (Bug State)
Execution failed, yet a receipt existed. Atomicity was violated and flagged.

<p align="center">
  <img src="images/rt-05-failure-before-fix.png.PNG" width="600">
</p>


Pass After Fix — Correct Fail-Closed Behavior
Execution failure produces no receipt and no mutation. Invariant restored.

<p align="center">
  <img src="images/rt-05-pass-after-fix.png.PNG" width="600">
</p>


Run locally:

cargo test rt_05


⸻

Quickstart

git clone https://github.com/your-org/vault-authority
cd vault-authority
cargo test redteam
cargo build --release

Integrate from your monitoring / alerting pipeline:

use vault_authority::{Vault, ShellExecutor};

let vault = Vault::new();
let executor = ShellExecutor;

vault.remediate(
    "incident-1234",
    "ERR_AUTH_EXPIRED",
    &executor
)?;

## 🏗️ Production Infrastructure

Vault Authority v1.1 is architected for zero-trust cloud environments.

* **Secret Management**: Integrated with Google Secret Manager via Secret Store CSI.
* **Hot-Reloading**: Cryptographic keys are refreshed in-memory via an asynchronous watcher, eliminating pod restarts during rotation.
* **Identity**: Bound to GCP IAM via Workload Identity (no static service keys or shared credentials).

### 🛠️ Operational Playbooks

Built-in governance for high-stakes environments:

| Playbook | Purpose | Invariant Protected |
| :--- | :--- | :--- |
| `verify_secrets.sh` | Audits CSI mount integrity | INV-3 (Boundary) |
| `rotate_signing_key.sh` | Rotates Ed25519 material | INV-4 (Bounding) |
| `remediation_audit.sh` | Cross-checks DB vs Signatures | INV-2 (Atomicity) |
| `cleanup_old_logs.sh` | Enforces 90-day data retention | INV-4 (Bounding) |

### 🔍 Cryptographic Verification

Every successful remediation produces a hex-encoded Ed25519 signature of the `trace_id`. Verification ensures that the fix was authorized by the current authority key and that the `trace_id` remains tamper-proof in the audit log.

---

**Health Check**: Run `./playbooks/verify_secrets.sh` immediately after deployment to confirm cryptographic readiness.


⸻

## Who This Is For

**DevOps/SRE Teams** — Eliminate toil, reclaim sleep  
**Engineering Managers** — Cut 20-40 hrs/month of senior time  
**Compliance Teams** — Cryptographic audit trails for SOC 2 / ISO 27001  


What This Is / What This Isn’t

This is
   •   A deterministic remediation gate
   •   Cryptographically auditable execution
   •   Fail-closed by construction
   •   Red-team verified

This is not
   •   A workflow engine
   •   A YAML sprawl
   •   “Best effort” automation
   •   An LLM agent

⸻

License

MIT — use, modify, and deploy freely.

---

## Next Steps

**Try it:** Clone, test, integrate  
**Report issues:** [GitHub Issues](https://github.com/joshuavetos/vault-authority/issues)  
**Commercial support:** jsvetos90@gmail.com

Vault Authority exists to make repeatable failures boring, provable, and safe.

