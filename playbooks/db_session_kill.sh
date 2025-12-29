#!/bin/bash
# Vault Authority Action: db_session_kill
# Purpose: Terminate hung DB sessions to prevent connection exhaustion

set -e

echo "Scanning for zombie database sessions..."

# 1. Terminate "idle in transaction" sessions older than 5 minutes
# Note: Replace $DB_NAME with your target database
psql -d $DB_NAME -c "
SELECT pg_terminate_backend(pid) 
FROM pg_stat_activity 
WHERE state = 'idle in transaction' 
AND state_change < current_timestamp - interval '5 minutes';"

echo "SUCCESS: Connection pool pressure relieved."
exit 0
