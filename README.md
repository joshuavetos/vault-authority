# Vault Authority v1.0 — Deterministic Remediation Gate

Vault Authority is a **fail-closed, deterministic remediation core** written in Rust.  
It enforces safety **by instruction ordering**, not configuration, policy text, or operator discretion.

This repository contains **only** the audited core and its adversarial test harness.  
There is no daemon, no HTTP service, and no configuration engine.

The guarantee is simple:

> **If an action fails, the system cannot lie about it.**

---

## What This System Does

Vault Authority accepts a `(trace_id, failure_id)` pair and attempts a remediation **exactly once**.

A cryptographically signed receipt is emitted **only if**:
- the failure is explicitly allowed
- the action executes successfully
- state mutation occurs in the correct order

If **any step fails**, no receipt exists and no state is mutated.

---

## Core Properties

- **Fail-Closed** — success is provable; failure leaves no residue  
- **Atomic** — execution and state mutation are inseparable  
- **Idempotent** — duplicate `trace_id` values are physically rejected  
- **Auditable** — correctness is proven via adversarial tests, not claims  

---

## Enforcement Model (Instruction Ordering)

The remediation path is strictly monotonic.  
If any step fails, the process terminates immediately.

1. **Validate** — enum gate (`INV-1`)
2. **Check** — dedupe read (`INV-4`)
3. **Execute** — external action (fallible)
4. **Commit** — dedupe write (point of no return)
5. **Sign** — cryptographic receipt
6. **Persist** — audit record

Safety is enforced by **where code is allowed to execute**, not by conditionals.

---

## Invariants

- **INV-1 (Enum Gating)**  
  Only failures explicitly defined in the taxonomy may proceed.

- **INV-2 (Atomicity)**  
  If execution fails, state remains unchanged and no receipt exists.

- **INV-3 (Boundary Control)**  
  All external effects are constrained to a controlled executor interface.

- **INV-4 (Idempotency)**  
  Duplicate executions for the same `trace_id` are rejected before execution.

---

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

---

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

---

## License

MIT
