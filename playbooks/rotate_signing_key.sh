#!/bin/bash
# Vault Authority Lifecycle: rotate_signing_key
# Purpose: Generate new Ed25519 key and update Google Secret Manager.

set -e

PROJECT_ID=$(gcloud config get-value project)
SECRET_NAME="vault-signing-key"
TEMP_KEY_FILE="/tmp/new_ed25519.key"

echo "🔐 Initiating Cryptographic Key Rotation..."

# 1. Generate new Ed25519 Private Key (INV-3)
# Uses OpenSSL to generate a raw 32-byte seed or PEM encoded key
openssl genpkey -algorithm ed25519 -out "$TEMP_KEY_FILE"

# 2. Push to Google Secret Manager (INV-1)
echo "🚀 Uploading new version to Secret Manager..."
gcloud secrets versions add "$SECRET_NAME" --data-file="$TEMP_KEY_FILE"

# 3. Clean up sensitive temporary artifacts (INV-2)
rm -f "$TEMP_KEY_FILE"

echo "✅ SUCCESS: New key version created in Secret Manager."
echo "⚠️  NOTE: Pods will refresh the key based on the CSI Driver's poll interval."
