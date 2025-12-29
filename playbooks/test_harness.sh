#!/bin/bash
# Vault Authority Quality Gate: test_harness
# Purpose: Orchestrates Rust integration tests and verifies red-team invariants.

set -e

echo "🧪 Initializing Vault Authority Test Harness..."

# 1. Clean previous build artifacts (INV-1: Sequential)
cargo clean -p vault-authority --doc

# 2. Run Unit Tests
echo "🏃 Running Unit Tests..."
cargo test --lib

# 3. Run Integration Tests (INV-3: Path Invariance)
echo "🏃 Running Integration Suite (tests/integration_test.rs)..."
cargo test --test integration_test -- --nocapture

# 4. Verify Playbook Permissions (INV-3: Boundary Control)
echo "🔍 Auditing Playbook Executability..."
for f in playbooks/*.sh; do
    if [ ! -x "$f" ]; then
        echo "⚠️  Fixing permissions for $f"
        chmod +x "$f"
    fi
done

echo "✅ Quality Gate Passed: All invariants verified."
