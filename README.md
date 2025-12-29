Vault Authority v1.0 — Deterministic Remediation Gate

Stop paying humans to fix the same failures over and over.

Vault Authority is a minimal, fail-closed Rust library that makes unsafe autonomous actions physically impossible. It enforces deterministic execution ordering so a system can never claim success unless the action actually completed.

If a receipt exists, the action succeeded.
If no receipt exists, nothing happened.

No partial success. No lies.

⸻

The Problem

Engineering teams lose time and money to predictable, repetitive failures:
   •   Auth tokens expire → engineer refreshes → happens again
   •   Rate limits trigger → config tweaked → happens again
   •   Zombie DB connections → DBA kills session → happens again

Every one of these creates:
   •   Tickets that should not exist
   •   On-call fatigue
   •   Compliance gaps
   •   Silent failures with no proof of what actually happened

Traditional automation lies by accident: scripts, playbooks, and agents can fail halfway through and still claim success.

That is unacceptable in production systems.

⸻

The Solution

Vault Authority is a deterministic remediation core that enforces truth by construction.

It:
	1.	Validates the failure against an explicit, versioned taxonomy
	2.	Rejects duplicate incidents before execution (idempotency)
	3.	Executes the remediation through a controlled boundary
	4.	Only after success generates a cryptographic receipt

If execution fails at any point, the system halts with zero residue.

⸻

The Core Guarantee

If a receipt exists, the action completed successfully.
If no receipt exists, nothing happened.

This is enforced by instruction ordering, not policy or configuration.

⸻

How It Works (Conceptual)

trace_id: "incident-2025-04-20-001"
failure_id: "ERR_AUTH_EXPIRED"

// Vault Authority pipeline:
1. Validate failure_id (enum-gated)
2. Check dedupe store (idempotency)
3. Execute remediation (fallible)
4. Commit mutation (point of no return)
5. Sign receipt (Ed25519)
6. Persist immutable audit record

Failure at any step aborts the process before mutation or signing.

⸻

Proven by Adversarial Tests

Vault Authority ships with red-team tests that intentionally attempt to break invariants.

Red-Team Scenarios
   •   RT-01 — Malicious or unknown failure ID → rejected before execution
   •   RT-02 — Replay same incident twice → second attempt refused
   •   RT-05 — Forced execution failure → no receipt generated, no state mutated

If RT-05 passes, the system is incapable of lying about success.

📸 Evidence (Screenshots)

These images are included in the repository under docs/images/:


⸻

Core Capabilities

✅ Fail-Closed by Design

Execution must succeed to leave any trace. Failure produces nothing.

✅ Cryptographically Auditable

Every successful remediation generates an Ed25519-signed receipt with timestamp.

✅ Idempotent Enforcement

Duplicate remediation attempts for the same incident are rejected before execution.

✅ Deterministic & Testable

Safety is demonstrated by adversarial tests, not claims or documentation.

⸻

Use Cases

Autonomous Token Refresh

Detect expired credentials and refresh automatically with proof.

Rate-Limit Auto-Adjustment

Handle predictable traffic spikes without paging humans.

Zombie Process Cleanup

Terminate hung DB connections safely and deterministically.

Compliance-Ready Incident Proof

Produce cryptographic evidence for auditors and postmortems.

⸻

Who This Is For
   •   SRE / DevOps teams eliminating toil
   •   Engineering managers reclaiming senior engineer time
   •   CTOs / VPs reducing MTTR and support costs
   •   Security & Compliance teams requiring provable remediation

⸻

What This Is (And Isn’t)

✅ This Is
   •   A deterministic remediation library
   •   A fail-closed execution gate
   •   A cryptographically auditable safety core
   •   Proven by adversarial testing

❌ This Is Not
   •   An agent framework
   •   A SaaS product
   •   A YAML workflow engine
   •   “Best-effort” automation

⸻

Technical Architecture

Safety is enforced by monotonic instruction ordering:
	1.	Validate — Explicit failure enum (INV-1)
	2.	Dedupe Check — Reject replay (INV-4)
	3.	Execute — Fallible remediation
	4.	Commit — Point of no return
	5.	Sign — Cryptographic receipt
	6.	Persist — Immutable audit record

Core Invariants
   •   INV-1 — Enum-gated execution
   •   INV-2 — Atomicity (failure = no mutation)
   •   INV-3 — Controlled execution boundary
   •   INV-4 — Idempotency enforced pre-execution

⸻

Getting Started

Requirements
   •   Rust 1.70+
   •   Existing monitoring or alerting system

Build

cargo build --release

Verify Safety

cargo test redteam

If RT-05 fails, do not deploy.

⸻

Relationship to PRB v1.1

Vault Authority implements the internal guarantees required by the Partner Reliability Benchmark (PRB) v1.1.
   •   Vault Authority provides the architecture
   •   PRB v1.1 provides the external proof

Compliance, executive, and legal artifacts are included in docs/.

⸻

License

MIT License.
Use freely. Modify freely. Deploy responsibly.

⸻

Final Word

Vault Authority exists for one reason:

Automation that cannot lie.

If your system claims it fixed something, it should be provable — or it should refuse to speak.

⸻

Vault Authority — because partial success is indistinguishable from failure.
