#!/bin/bash
# Vault Authority Action: log_exporter
# Purpose: Transform internal results into structured JSON for ELK/Splunk

set -e

TRACE_ID=$1
FAILURE_ID=$2
STATUS=$3

# Emit structured JSON to stdout for log collectors
echo "{\"timestamp\": \"$(date -u +%Y-%m-%dT%H:%M:%SZ)\", \"trace_id\": \"$TRACE_ID\", \"failure_id\": \"$FAILURE_ID\", \"outcome\": \"$STATUS\", \"version\": \"1.1.0\"}"
