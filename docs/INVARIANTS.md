Vault Authority v1.0 Invariants

INV-1 Sequential Gating
failure_id must be a member of a fixed enum before any execution or state access.

INV-2 Atomicity
If execution fails, no dedupe entry and no receipt may exist.

INV-3 Delegated Execution
The Vault never executes commands directly. All mutation is delegated through ActionExecutor.

INV-4 Idempotence
The same trace_id may never execute twice.

INV-5 No Lies
A receipt is proof of execution. Absence of a receipt is proof of non-execution.

These invariants are enforced by instruction ordering, not convention.
