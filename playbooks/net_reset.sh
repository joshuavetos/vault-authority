#!/bin/bash
# Vault Authority Action: net_reset
# Purpose: Recover from DNS hangs or routing staleness

set -e

echo "Initiating Network Stack Reset..."

# 1. Flush systemd-resolved DNS cache
resolvectl flush-caches || true

# 2. Restart the local networking service
systemctl restart networking || systemctl restart NetworkManager

# 3. Verify outbound connectivity to a reliable anchor (e.g., 8.8.8.8)
if ping -c 1 8.8.8.8 > /dev/null; then
    echo "SUCCESS: Connectivity restored."
    exit 0
else
    echo "FAILURE: Network remains unreachable."
    exit 1
fi
