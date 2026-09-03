-- Initial Settle Database Schema
-- This migration creates the core tables for indexing Stellar contract events

-- Agreements table
CREATE TABLE agreements (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    on_chain_id TEXT NOT NULL UNIQUE,
    creator TEXT NOT NULL,
    counterparty TEXT NOT NULL,
    token TEXT NOT NULL,
    total_amount BIGINT NOT NULL,
    funded_amount BIGINT NOT NULL DEFAULT 0,
    released_amount BIGINT NOT NULL DEFAULT 0,
    refunded_amount BIGINT NOT NULL DEFAULT 0,
    status TEXT NOT NULL CHECK (status IN ('DRAFT', 'FUNDED', 'ACTIVE', 'COMPLETED', 'DISPUTED', 'RESOLVED', 'EXPIRED', 'CANCELLED')),
    created_at TIMESTAMPTZ NOT NULL,
    expires_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    milestones TEXT[] NOT NULL DEFAULT ARRAY[]::TEXT[]
);

-- Milestones table
CREATE TABLE milestones (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    on_chain_id TEXT NOT NULL UNIQUE,
    agreement_id TEXT NOT NULL,
    name TEXT NOT NULL,
    description TEXT NOT NULL,
    amount BIGINT NOT NULL,
    status TEXT NOT NULL CHECK (status IN ('PENDING', 'SUBMITTED', 'APPROVED', 'REJECTED', 'RELEASED')),
    due_date TIMESTAMPTZ NOT NULL,
    submitted_at TIMESTAMPTZ,
    approved_at TIMESTAMPTZ,
    evidence TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Disputes table
CREATE TABLE disputes (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    on_chain_id TEXT NOT NULL UNIQUE,
    agreement_id TEXT NOT NULL,
    opened_by TEXT NOT NULL,
    reason TEXT NOT NULL,
    status TEXT NOT NULL CHECK (status IN ('OPEN', 'EVIDENCE_SUBMISSION', 'UNDER_REVIEW', 'RESOLVED', 'CLOSED')),
    resolution TEXT,
    opened_at TIMESTAMPTZ NOT NULL,
    resolved_at TIMESTAMPTZ,
    arbitrator TEXT,
    evidence TEXT[] NOT NULL DEFAULT ARRAY[]::TEXT[],
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Settlement events table for complete audit trail
CREATE TABLE settlement_events (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    event_type TEXT NOT NULL,
    agreement_id TEXT,
    milestone_id TEXT,
    dispute_id TEXT,
    participant TEXT NOT NULL,
    data JSONB NOT NULL DEFAULT '{}',
    timestamp TIMESTAMPTZ NOT NULL,
    block_height BIGINT NOT NULL,
    transaction_hash TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Indexer cursor for tracking processed events
CREATE TABLE indexer_cursor (
    id SERIAL PRIMARY KEY,
    last_ledger BIGINT NOT NULL,
    last_processed_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Insert initial cursor
INSERT INTO indexer_cursor (last_ledger) VALUES (0);

-- Indexes for performance
CREATE INDEX idx_agreements_creator ON agreements(creator);
CREATE INDEX idx_agreements_counterparty ON agreements(counterparty);
CREATE INDEX idx_agreements_status ON agreements(status);
CREATE INDEX idx_agreements_created_at ON agreements(created_at);

CREATE INDEX idx_milestones_agreement_id ON milestones(agreement_id);
CREATE INDEX idx_milestones_status ON milestones(status);
CREATE INDEX idx_milestones_due_date ON milestones(due_date);

CREATE INDEX idx_disputes_agreement_id ON disputes(agreement_id);
CREATE INDEX idx_disputes_opened_by ON disputes(opened_by);
CREATE INDEX idx_disputes_status ON disputes(status);
CREATE INDEX idx_disputes_opened_at ON disputes(opened_at);

CREATE INDEX idx_settlement_events_type ON settlement_events(event_type);
CREATE INDEX idx_settlement_events_agreement_id ON settlement_events(agreement_id);
CREATE INDEX idx_settlement_events_participant ON settlement_events(participant);
CREATE INDEX idx_settlement_events_timestamp ON settlement_events(timestamp);
CREATE INDEX idx_settlement_events_block_height ON settlement_events(block_height);

-- Update triggers for updated_at columns
CREATE OR REPLACE FUNCTION trigger_set_timestamp()
RETURNS TRIGGER AS $$
BEGIN
  NEW.updated_at = NOW();
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER set_timestamp_agreements
    BEFORE UPDATE ON agreements
    FOR EACH ROW
    EXECUTE FUNCTION trigger_set_timestamp();

CREATE TRIGGER set_timestamp_milestones
    BEFORE UPDATE ON milestones
    FOR EACH ROW
    EXECUTE FUNCTION trigger_set_timestamp();

CREATE TRIGGER set_timestamp_disputes
    BEFORE UPDATE ON disputes
    FOR EACH ROW
    EXECUTE FUNCTION trigger_set_timestamp();

CREATE TRIGGER set_timestamp_indexer_cursor
    BEFORE UPDATE ON indexer_cursor
    FOR EACH ROW
    EXECUTE FUNCTION trigger_set_timestamp();