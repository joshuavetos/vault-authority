Vault Authority v1.0
Fail-closed remediation library. Proves safety through ordering invariants, not trust.
Minimal Rust library and adversarial test suite that accepts (trace_id, failure_id) to enforce mechanical guarantees. This is a deterministic safety gate for repetitive failures like Stripe 401s or Twilio 429s.
Repository Structure
vault-authority/
├── Cargo.toml      # Dependency lock (tokio, serde, ed25519-dalek)
├── src/
│   ├── lib.rs      # API surface
│   ├── vaultd.rs   # INV-1..4 enforcement logic
│   └── actions.rs  # Mockable execution boundary
└── tests/
    └── redteam.rs  # Proof by adversarial abuse

Invariants (Ordering-Based)
The system enforces truth through a strictly monotonic state path:
 * INV-1 (Gating): failure_id must exist in the defined ENUM only.
 * INV-2 (Atomic): Execution failure results in zero state changes (no dedupe, no receipt).
 * INV-4 (Dedupe): A unique trace_id is guaranteed to execute \le 1 time.
> Receipt exists = action succeeded. No receipt = system did nothing.
> 
The Core Interface
The vault sits between diagnosis and execution, requiring an external executor to perform mutations.
pub async fn remediate(
    &self, 
    trace_id: &str, 
    failure_id: &str, 
    executor: &dyn ActionExecutor
) -> Result<Receipt, VaultError>

Critical Execution Ordering:
validate → dedupe (read) → exec → dedupe (write) → sign → audit
Any failure before the dedupe write results in a REJECT with no system footprint.
Proof of Safety
Verification is handled through executable abuse, not promises.
Run the suite:
cargo test redteam -- --nocapture

Expected Outcomes:
 * RT-01 (Invalid Enum): INV-1 violation → Err(InvalidFailureID).
 * RT-02 (Duplicate Trace): INV-4 violation → Err(DuplicateTrace).
 * RT-05 (Exec Failure): INV-2 violation → Err(ExecutionFailed) + no audit/receipt created.
Reality Check
 * NOT: An HTTP API, kernel hook (eBPF), or LLM orchestration layer.
 * IS: A hardware-ready safety gate that proves an "LLM observer" cannot bypass invariants.
 * Status: v1.0 Sealed. The tests serve as the final contract.

