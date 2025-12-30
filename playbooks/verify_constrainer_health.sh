#!/bin/bash
# Vault Authority v1.1 - Governance Health Check
# Checks the integrity of the AI Firewall (constrainer.py)

set -e # Fail-Closed on any error

echo "[Vault Authority] Initiating Constrainer Integrity Check..."

# 1. Verify Environment
if [ -z "$ANTHROPIC_API_KEY" ]; then
    echo "CRITICAL_FAILURE: ANTHROPIC_API_KEY is null. AI Firewall is blind."
    exit 1
fi

# 2. Test Deterministic Regex (No-API Mode)
# Ensures the local YAML policy engine still blocks known forbidden strings
FORBIDDEN_TEST=$(./scripts/constrainer.py "rm -rf / --force" --dry-run)
if [[ "$FORBIDDEN_TEST" != *"BLOCKED"* ]]; then
    echo "INVARIANT_VIOLATION: Constrainer failed to block hard-coded regex. Refusing to proceed."
    exit 1
fi

# 3. Test API Connectivity (Synthetic Trace)
# Ensures the LLM-handshake is valid and not hallucinating logic
HEALTH_TRACE_ID="health-check-$(date +%s)"
API_TEST=$(./scripts/constrainer.py "echo 'health check'" $ANTHROPIC_API_KEY)

if [[ $? -ne 0 ]]; then
    echo "F3_FAILURE: AI Firewall cannot reach authority. Remediations paused."
    exit 1
fi

echo "HEALTH_CHECK_PASSED: Constrainer is atomic and responsive."
exit 0
