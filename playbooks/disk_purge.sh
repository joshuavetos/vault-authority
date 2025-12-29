#!/bin/bash
# Vault Authority Action: disk_purge
# Purpose: Safely recover disk space by clearing transient data

set -e

echo "Initiating Bounded Disk Cleanup..."

# 1. Purge /tmp files older than 24 hours (Bounded cleanup)
find /tmp -type f -atime +1 -delete

# 2. Force rotate application logs to compress current data
if command -v logrotate > /dev/null; then
    logrotate -f /etc/logrotate.d/application-logs
fi

# 3. Final Verification: Check if available space is now > 5%
FREE_SPACE=$(df / --output=pcent | tail -1 | tr -dc '0-9')
if [ "$FREE_SPACE" -lt 95 ]; then
    echo "SUCCESS: Disk pressure relieved. Current usage: $FREE_SPACE%"
    exit 0
else
    echo "FAILURE: Cleanup insufficient. Human intervention required."
    exit 1
fi
