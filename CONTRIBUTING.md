# Contributing to Vault Authority

This repository is governed by mechanical legitimacy. Correctness is the only currency. We do not prioritize developer ergonomics over architectural truth.

## 1. Non-Negotiable Principles (The Gate)
All contributions must uphold the four SysDNA Invariants:
* **INV-1 (Sequential)**: No state transitions can happen out of order.
* **INV-2 (Atomic)**: No side effects without a successful return code and receipt.
* **INV-3 (Gating)**: Every execution must be validated against the taxonomy.
* **INV-4 (Bounding)**: No unbounded resource growth (logs, buffers, or state).

## 2. Acceptable Contributions
We only accept Pull Requests (PRs) that fall into these categories:
* **Invariant Strengthening**: Fixes that close a loophole or race condition.
* **Red-Team Suites**: Tests that demonstrate a failure in the current logic.
* **Operational Hardening**: Security patches or version-lock updates.

## 3. Automatic Rejection
The following will be closed without discussion:
* **Convenience Abstractions**: Any change that simplifies code at the cost of clarity or determinism.
* **UI Power-Creep**: Any attempt to add "Run" or "Mutate" capabilities to the Desktop Observer.
* **Feature Sprawl**: Requests for "agent-like" behavior or non-deterministic logic.
* **Nondeterministic Errors**: Code that relies on timing, unverified network state, or external "best effort" signals.

## 4. The Review Process
1.  **Proof of Test**: Every PR must include a test in `tests/` that passes on a clean build.
2.  **Invariant Impact**: You must explicitly state in the PR description which invariant (INV-1 through INV-4) is being affected and why the change preserves or strengthens it.
3.  **Mechanical Audit**: If a maintainer determines the change introduces architectural drift, the PR will be rejected. 

**Vault Authority exists to make failures boring, provable, and safe. If your code makes it "interesting," it does not belong here.**
