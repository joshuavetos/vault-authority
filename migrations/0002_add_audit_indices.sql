-- Vault Authority Production Migration
-- Optimizes trace_id lookups for INV-4 compliance

CREATE INDEX IF NOT EXISTS idx_remediation_trace_id ON remediations (trace_id);
CREATE INDEX IF NOT EXISTS idx_remediation_timestamp ON remediations (created_at);

-- Add metadata column for structured logging audit
ALTER TABLE remediations ADD COLUMN IF NOT EXISTS metadata JSONB;
