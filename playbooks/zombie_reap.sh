#!/bin/bash
# Vault Authority Action: zombie_reap
# Purpose: Deterministically recover hung services via tiered escalation

set -e

SERVICE_NAME="application-service"

echo "Initiating Tiered Service Recovery for $SERVICE_NAME..."

# 1. Attempt Graceful Shutdown (SIGTERM)
systemctl stop $SERVICE_NAME --timeout=30s || true

# 2. Verify if process still exists
PID=$(pgrep -f $SERVICE_NAME)

if [ -z "$PID" ]; then
    echo "SUCCESS: Service stopped gracefully."
else
    # 3. Escalation: Forced Termination (SIGKILL)
    echo "WARNING: Service unresponsive. Escalating to SIGKILL..."
    kill -9 $PID
    sleep 2
fi

# 4. Restart and Verify Health
systemctl start $SERVICE_NAME
sleep 5

if systemctl is-active --quiet $SERVICE_NAME; then
    echo "SUCCESS: Service recovered and active."
    exit 0
else
    echo "FAILURE: Recovery failed. Critical system failure."
    exit 1
fi
