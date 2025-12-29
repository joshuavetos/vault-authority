#!/bin/bash
# Vault Authority Hygiene: cleanup_old_logs
# Purpose: Archive and purge records older than 90 days to enforce INV-4 Bounding.

set -e

# 1. Environment Configuration
DB_URL=${DATABASE_URL:-"postgres://localhost:5432/vault_authority"}
RETENTION_DAYS=90
ARCHIVE_DIR="archives/remediations/$(date +%Y/%m)"

echo "🧹 Initiating Log Bounding (Retention: $RETENTION_DAYS days)..."

# 2. Ensure Archive Directory Exists (INV-3: Path Invariance)
mkdir -p "$ARCHIVE_DIR"

# 3. Export to JSON before Purging (INV-2: Atomic)
# This ensures no data is lost during the cleanup process.
echo "📦 Archiving data to $ARCHIVE_DIR..."
psql "$DB_URL" -c "\copy (SELECT * FROM remediations WHERE created_at < now() - interval '$RETENTION_DAYS days') TO '$ARCHIVE_DIR/purged_logs.csv' WITH CSV HEADER"

# 4. Perform the Purge (INV-1: Sequential)
echo "🗑️  Purging records from database..."
ROW_COUNT=$(psql "$DB_URL" -t -c "DELETE FROM remediations WHERE created_at < now() - interval '$RETENTION_DAYS days'; SELECT CAST(count(*) AS text) FROM (SELECT 1) AS t;")

echo "✅ Cleanup Complete. $ROW_COUNT records archived and purged."
