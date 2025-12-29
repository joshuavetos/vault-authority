#!/bin/bash
# Vault Authority Action: worker_recycle
# Purpose: Graceful reload of service workers to relieve RAM pressure

set -e

SERVICE_NAME="app-worker"

echo "Initiating Graceful Worker Recycle for $SERVICE_NAME..."

# 1. Send SIGHUP/Reload to the service (standard for Nginx/Gunicorn/systemd)
# This allows current requests to finish while spawning fresh processes
systemctl reload $SERVICE_NAME

# 2. Verify that the service is stable
sleep 2
if systemctl is-active --quiet $SERVICE_NAME; then
    echo "SUCCESS: Workers recycled. Memory footprint reset."
    exit 0
else
    echo "FAILURE: Service failed to reload."
    exit 1
fi
