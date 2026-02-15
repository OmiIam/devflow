-- Focus sessions track planned versus actual focus.

CREATE TABLE IF NOT EXISTS focus_sessions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    task_id UUID REFERENCES tasks(id) ON DELETE SET NULL,
    duration_seconds INT NOT NULL CHECK (duration_seconds > 0),
    actual_duration_seconds INT,
    started_at TIMESTAMPTZ NOT NULL,
    ended_at TIMESTAMPTZ,
    completed BOOLEAN NOT NULL DEFAULT false,
    notes TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_focus_sessions_user_started
    ON focus_sessions (user_id, started_at DESC);
CREATE INDEX IF NOT EXISTS idx_focus_sessions_task ON focus_sessions (task_id);
CREATE INDEX IF NOT EXISTS idx_focus_sessions_active
    ON focus_sessions (user_id, ended_at) WHERE ended_at IS NULL;
