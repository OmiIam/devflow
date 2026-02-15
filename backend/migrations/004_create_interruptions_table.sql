-- Interruptions explain context shifts during focus sessions.

CREATE TABLE IF NOT EXISTS interruptions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    session_id UUID NOT NULL REFERENCES focus_sessions(id) ON DELETE CASCADE,
    category TEXT NOT NULL CHECK (category IN ('external', 'internal', 'urgent', 'break')),
    reason TEXT,
    duration_seconds INT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_interruptions_session_created
    ON interruptions (session_id, created_at);
