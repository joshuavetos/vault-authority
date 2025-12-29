# Vault Authority v1.0 — Deterministic Remediation Gate

Stop wasting time and money fixing the same damn problems.
Vault Authority is an automated repair system built in Rust. When your infrastructure keeps breaking in the same ways, it catches the pattern, fixes the issue by itself, and provides a cryptographic receipt proving it did the job correctly—no manual babysitting, no guesswork.
The project meets the Partner Reliability Benchmark (PRB) v1.1 standards for verified system dependability.

Vault Authority is a **fail-closed, deterministic remediation core** written in Rust.  
It enforces safety **by instruction ordering**, not configuration, policy text, or operator discretion.

This repository contains **only** the audited core and its adversarial test harness.  
There is no daemon, no HTTP service, and no configuration engine.

The guarantee is simple:

> **If an action fails, the system cannot lie about it.**

-----

## What This System Does

Vault Authority accepts a `(trace_id, failure_id)` pair and attempts a remediation **exactly once**.

A cryptographically signed receipt is emitted **only if**:

- the failure is explicitly allowed
- the action executes successfully
- state mutation occurs in the correct order

If **any step fails**, no receipt exists and no state is mutated.

-----

## Core Properties

- **Fail-Closed** — success is provable; failure leaves no residue
- **Atomic** — execution and state mutation are inseparable
- **Idempotent** — duplicate `trace_id` values are physically rejected
- **Auditable** — correctness is proven via adversarial tests, not claims

-----

## Enforcement Model (Instruction Ordering)

The remediation path is strictly monotonic.  
If any step fails, the process terminates immediately.

1. **Validate** — enum gate (`INV-1`)
1. **Check** — dedupe read (`INV-4`)
1. **Execute** — external action (fallible)
1. **Commit** — dedupe write (point of no return)
1. **Sign** — cryptographic receipt
1. **Persist** — audit record

Safety is enforced by **where code is allowed to execute**, not by conditionals.

-----

## Invariants

- **INV-1 (Enum Gating)**  
  Only failures explicitly defined in the taxonomy may proceed.
- **INV-2 (Atomicity)**  
  If execution fails, state remains unchanged and no receipt exists.
- **INV-3 (Boundary Control)**  
  All external effects are constrained to a controlled executor interface.
- **INV-4 (Idempotency)**  
  Duplicate executions for the same `trace_id` are rejected before execution.

-----

## Red-Team Verification (RT-05)

The following evidence demonstrates the elimination of a critical failure mode:
a receipt being generated despite execution failure.

### ❌ Failure Before Fix

*A receipt existed even though execution failed — invariant violation.*

![RT-05 failure before fix](images/rt-05-failure-before-fix.png.PNG)

### ✅ Pass After Fix

*Execution failure produces no receipt and no state mutation.*

![RT-05 pass after fix](images/rt-05-pass-after-fix.png.PNG)

The test suite proves the invariant **by attempting to break it**.

-----

## Scope (Intentional)

This project is:

- a **library**
- a **deterministic core**
- a **test-proven safety gate**

This project is **not**:

- a daemon
- an HTTP service
- a workflow engine
- a YAML-driven policy system

Anything above this layer must inherit its constraints.

Verification Extension: PRB v1.1
This repository has been extended to comply with the Partner Reliability Benchmark (PRB) v1.1. While the core library remains a deterministic safety gate, this extension provides the bitwise proof required for external audits.

- Normalization Standard: All outputs are verified using Norm-v1.1 (Trim, LF-normalization, space-collapse).
- Bitwise Integrity: Success is validated against SHA256 hashes of the canonical test vectors.
- Compliance Artifacts: Supplementary documentation for CISO review, Legal mapping, and Certification Policy is located in the docs/ directory.
  Verification Harness:
  Run the following to verify the output of a specific remediation trace:
  ./prb-check.sh “[output_string]” “[expected_hash]”

Verification Extension: PRB v1.1
This update brings the repository into full compliance with the Partner Reliability Benchmark (PRB) v1.1. The core library still acts as a deterministic safety gate—it never makes random choices—but this extension adds the low‑level, bit‑for‑bit proof needed for external audits.
•	Output Normalization: Every output is cleaned and standardized using Norm‑v1.1, which trims whitespace, normalizes line endings, and collapses extra spaces.
•	Integrity Checks: Each successful run is verified against known SHA‑256 hash values to confirm it matched the trusted reference exactly.
•	Audit Materials: Documentation for security officers, legal teams, and certification reviewers is available in the `docs/` folder.
Verification command:  
To confirm a remediation trace, run:  
`./prb-check.sh "[output_string]" "[expected_hash]"`

Operational Governance & Wiki
This repository includes a comprehensive documentation suite located in the docs/ directory to ensure high-integrity deployment and auditing:
• Security Model: Defines the structural invariants (INV-1 through INV-4) and the Ed25519 cryptographic signing process.
• Operator's Handbook: Instructions for managing rejections, performing manual overrides, and verifying receipts.
• Taxonomy Governance: The "laws" of the system, including the checklist for adding new remediation playbooks.

-----

## License

MIT
