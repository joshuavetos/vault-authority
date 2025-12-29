#!/bin/bash
# Vault Authority Operational Check: verify_secrets
# Purpose: Confirm CSI mount point integrity and secret availability.

set -e

MOUNT_PATH="/etc/vault/keys"
DB_SECRET_FILE="${MOUNT_PATH}/database_url"
KEY_SECRET_FILE="${MOUNT_PATH}/private.key"

echo "🔍 Auditing Secret Injection Points..."

# 1. Check Mount Point Existence
if [ ! -d "$MOUNT_PATH" ]; then
    echo "❌ FAILURE: Mount path $MOUNT_PATH does not exist."
    exit 1
fi

# 2. Verify Database URL availability
if [ -f "$DB_SECRET_FILE" ] && [ -s "$DB_SECRET_FILE" ]; then
    echo "✅ SUCCESS: Database secret injected ($(stat -c%s "$DB_SECRET_FILE") bytes)."
else
    echo "❌ FAILURE: Database secret missing or empty at $DB_SECRET_FILE."
    exit 1
fi

# 3. Verify Private Key availability
if [ -f "$KEY_SECRET_FILE" ] && [ -s "$KEY_SECRET_FILE" ]; then
    echo "✅ SUCCESS: Signing key injected ($(stat -c%s "$KEY_SECRET_FILE") bytes)."
else
    echo "❌ FAILURE: Signing key missing or empty at $KEY_SECRET_FILE."
    exit 1
fi

echo "🛡️ All production secrets verified at the filesystem boundary."
