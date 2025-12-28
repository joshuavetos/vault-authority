Vault Authority v1.0
Deterministic Remediation Gate
Vault Authority is a high-integrity Rust library designed to sit between a fallible diagnostic layer and production environments. It replaces discretionary "script trust" with architectural ordering invariants, ensuring that automated actions are physically incapable of violating system safety.
Repository Structure
A minimal, audited footprint designed for zero-drift instantiation.
 * Cargo.toml: Specific dependency locks for tokio, serde, and ed25519-dalek.
 * src/lib.rs: Clean API surface for internal or external consumption.
 * src/vaultd.rs: The audited core enforcing the 6-step monotonic ordering rule.
 * src/actions.rs: A mockable execution boundary to isolate side effects.
 * tests/redteam.rs: An executable proof-by-abuse suite.
Core Invariants
Safety is guaranteed by the physical order of instructions, enforcing a fail-closed state at all times:
 * INV-1 (Gating): Only failures explicitly defined in the FAILURE_TAXONOMY can proceed.
 * INV-2 (Atomic): Any execution failure results in zero state changes—no receipt is signed, and no dedupe key is written.
 * INV-4 (Dedupe): Every unique trace_id is guaranteed to execute exactly once.
> The Golden Rule: A receipt exists only if the action succeeded. If it did not happen, no receipt or dedupe entry exists.
> 
The Authority Loop
The remediate function acts as a deterministic judge for automated actions.
pub async fn remediate(
    &self, 
    trace_id: String, 
    failure_id_raw: String, 
    executor: &dyn ActionExecutor
) -> Result<(), VaultError>

Critical Execution Sequence:
 * Validate: Ensure input matches the enum.
 * Dedupe Read: Block replay attacks before they start.
 * Execute: Attempt the mutation via the ActionExecutor.
 * Dedupe Write: Commit the record only after confirmed success.
 * Sign: Generate a non-repudiable Ed25519 signature.
 * Audit: Persist the final receipt.
Proof of Safety
The system relies on red-team tests that simulate hostile input and system failures to prove safety.
Run the verification suite:
cargo test redteam -- --nocapture

Verified Vectors:
 * RT-01 (Invalid Enum): Proves that intent smuggling is blocked before state mutation.
 * RT-02 (Duplicate Trace): Proves that retry storms are physically impossible.
 * RT-05 (Exec Failure): Proves that a failed action produces no receipt and no dedupe entry.
Project Reality
 * Deterministic: Replaces "probably works" with "provably safe".
 * Zero Noise: No server, no complex YAML, and no agentic "black box" behavior.
 * Hardened: Designed specifically for high-frequency API failure modes like Stripe 401s or Twilio 429s.
Status: v1.0 Sealed. The tests are the contract.
---

### Visual Proof

**The Architecture (6-Step Ordering Rule)**
![Authority Loop](docs/images/sequence_diagram.png)

**The Evidence (Red-Team Success)**
![Cargo Test Output](docs/images/test_output.png)

---
