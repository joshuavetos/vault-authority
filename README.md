Vault Authority v1.0 — Deterministic Remediation Core

A fail-closed remediation engine that makes it physically impossible to lie about success.

Vault Authority is a minimal Rust library that accepts (trace_id, failure_id), validates intent, executes a remediation through a strict boundary, and emits a cryptographically signed receipt only if the action actually succeeds.
If anything fails, nothing is written. No residue. No ambiguity.

⸻

What This Is
   •   A fail-closed remediation core
   •   Deterministic by instruction ordering, not policy
   •   Cryptographically auditable
   •   Proven by adversarial tests

What This Is Not
   •   Not an agent
   •   Not an HTTP service or daemon
   •   Not YAML-driven
   •   Not an LLM system
   •   Not a production orchestration platform

This is the core safety primitive.

⸻

The Core Guarantee

If a receipt exists, the action completed successfully.
If no receipt exists, nothing happened.

There is no partial success state.

⸻

Why This Exists

Automation lies.

Most incident automation systems can claim success even when:
   •   a command partially ran
   •   a script failed halfway through
   •   state was mutated inconsistently
   •   logs are ambiguous or missing

That creates operational debt and compliance risk.

Vault Authority eliminates this entire class of failure by making false success structurally impossible.

⸻

How Safety Is Enforced

Vault Authority uses a strictly monotonic execution path:

validate → dedupe read → execute → commit → sign → persist

   •   Validation and dedupe happen before execution
   •   Execution happens before any state mutation
   •   Signing and persistence only occur after successful execution

If any step fails, the process halts immediately.

No commit.
No signature.
No audit entry.

⸻

Proven by Adversarial Tests

Safety is not a claim — it is demonstrated.

The repository includes red-team tests that attempt to break invariants:
   •   RT-01: Malicious / unknown failure ID → rejected before execution
   •   RT-02: Duplicate incident replay → rejected before execution
   •   RT-05: Forced execution failure → no receipt, no dedupe entry

These tests prove that the system cannot generate a success artifact unless execution actually succeeded.

Evidence (Screenshots)

The following images are included in the repository and referenced here as proof artifacts:
   •   docs/images/rt_05_pass.png
RT-05 passing: execution failure produces zero receipt
   •   docs/images/redteam_summary.png
Full red-team test suite passing

⸻

Deterministic, Not Clever

Vault Authority does not attempt to “decide” what to do.
   •   No heuristics
   •   No probabilistic behavior
   •   No learning
   •   No guessing

All allowed actions are explicit.
All execution is gated.
All success is provable.

⸻

Intended Audience

This project is for:
   •   SREs burned by automation that lies
   •   Infrastructure engineers tired of unprovable fixes
   •   Security / compliance teams that need hard evidence, not logs
   •   System designers building higher-level autonomous systems safely

⸻

Status

This is a reference implementation of a deterministic remediation gate.
   •   Small by design
   •   Audited
   •   Red-team verified
   •   Intended to be studied, forked, and integrated

Everything else (HTTP layers, orchestration, metrics, workflows) belongs outside this core.

⸻

License

MIT License

Use it freely. Modify it freely.
If you deploy it wrong, that part is on you.

⸻

Vault Authority
Because success should be provable, not claimed.
