#!/bin/bash
# PRB v1.1 Deterministic Check Harness

if [ "$#" -ne 2 ]; then
    echo "Usage: ./prb-check.sh 'output_string' 'expected_hash'"
    exit 1
fi

# Norm-v1.1: Trim, Normalize, and Collapse spaces
CLEAN_OUT=$(echo "$1" | sed -e 's/^[[:space:]]*//;s/[[:space:]]*$//' | tr -s ' ' | tr -d '\r')

ACTUAL_HASH=$(echo -n "$CLEAN_OUT" | sha256sum | awk '{print $1}')

if [ "$ACTUAL_HASH" == "$2" ]; then
    echo "STATUS: PASS [Norm-1.1]"
    exit 0
else
    echo "STATUS: FAIL [Hash Mismatch]"
    exit 1
fi
