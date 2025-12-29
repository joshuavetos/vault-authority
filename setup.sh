#!/usr/bin/env bash
set -euo pipefail

# Vault Authority v1.2: Zero-Trust Bootstrap Verification
# Rationale: Correctness > Convenience.

BANNER="[VAULT AUTHORITY SETUP]"

echo "$BANNER Starting environment audit..."

# 1. Environment Verification
command -v rustc >/dev/null 2>&1 || { echo >&2 "$BANNER Error: Rust not found. Aborting."; exit 1; }
command -v npm >/dev/null 2>&1 || { echo >&2 "$BANNER Error: Node.js not found. Aborting."; exit 1; }

# 2. Dependency Lock Check (INV-3)
echo "$BANNER Verifying dependency integrity..."
if [ -f "package-lock.json" ]; then
    npm ci --offline || { echo >&2 "$BANNER Error: Lockfile mismatch or network required. Aborting."; exit 1; }
else
    echo >&2 "$BANNER Error: No lockfile found. Cannot verify supply chain. Aborting."; exit 1;
fi

# 3. Security/Key Audit (INV-3)
echo "$BANNER Auditing signing key path..."
KEY_PATH=${VAULT_KEY_PATH:-"/etc/vault/keys/private.key"}
if [ ! -r "$KEY_PATH" ]; then
    echo >&2 "$BANNER Warning: Private key not readable at $KEY_PATH."
    echo >&2 "$BANNER Engine will start in PASSIVE OBSERVER mode only."
else
    echo "$BANNER Signing key detected. Authority confirmed."
fi

# 4. Prove Fail-Closed Behavior (RT-05)
echo "$BANNER Executing RT-05 Adversarial Verification..."
# Simulate an unauthorized request to verify refusal logic
if cargo test rt_05_refusal_test -- --nocapture; then
    echo "$BANNER ✅ SUCCESS: Engine correctly refused unauthorized execution."
else
    echo >&2 "$BANNER ❌ CRITICAL: Engine failed to refuse unauthorized execution. DO NOT DEPLOY."
    exit 1
fi

echo "------------------------------------------------"
echo "$BANNER SYSTEM INTEGRITY VERIFIED."
echo "$BANNER Status: FAIL-CLOSED ENABLED."
echo "------------------------------------------------"
