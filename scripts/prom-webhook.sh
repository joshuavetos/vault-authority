#!/bin/bash
# Vault Authority Integration: Prometheus Webhook Adapter
# Purpose: Map incoming JSON alerts to deterministic remediation traces.

set -e

# 1. Extract Trace ID (Incident ID) and Failure Type using 'jq'
# We use the unique incident_id from the alert to satisfy INV-4 (Idempotency)
TRACE_ID=$(echo "$1" | jq -r '.alerts[0].labels.incident_id')
FAILURE_ID=$(echo "$1" | jq -r '.alerts[0].labels.alertname')

# 2. Validation Gate: Ensure parameters are not empty
if [ -z "$TRACE_ID" ] || [ -z "$FAILURE_ID" ]; then
    echo "ERROR: Missing required alert labels (incident_id or alertname)."
    exit 1
fi

# 3. Trigger the Deterministic Core
# This routes the signal through the 6-step monotonic ordering rule
./target/release/vault-auth "$TRACE_ID" "$FAILURE_ID"
