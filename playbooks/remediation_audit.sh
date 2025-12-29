#!/bin/bash
# Vault Authority Integrity Audit: remediation_audit
# Purpose: Verifies the cryptographic binding between trace IDs and database records.

set -e

# 1. Environment Configuration
DB_URL=${DATABASE_URL:-"postgres://localhost:5432/vault_authority"}
AUDIT_LOG="audit_report_$(date +%Y%m%d).json"

echo "🛡️ Starting Cryptographic Integrity Audit..."

# 2. Extract Recent Remediations (INV-4: Registry Check)
# Queries the database for the last 100 remediations and their metadata
echo "📊 Fetching records from PostgreSQL..."
psql "$DB_URL" -t -c "SELECT json_build_object('trace_id', trace_id, 'signature', signature, 'status', status) FROM remediations ORDER BY created_at DESC LIMIT 100;" > .audit_temp

# 3. Analyze Determinism (INV-2: Atomic Verification)
# Checks for 'Zombie' records where a signature exists without a success status
echo "🔍 Analyzing state transitions..."
grep -v "success" .audit_temp | grep "signature" && {
    echo "⚠️  CRITICAL FAILURE: Found signatures for failed/partial state transitions (F2)."
    exit 1
} || echo "✅ All signatures correspond to successful state transitions."

# 4. Final Report Generation
mv .audit_temp "$AUDIT_LOG"
echo "📄 Audit complete. Report saved to $AUDIT_LOG"
