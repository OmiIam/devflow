-- Tasks capture user intent and link to focus sessions.

CREATE TABLE IF NOT EXISTS tasks (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    title VARCHAR(500) NOT NULL,
    description TEXT,
    priority TEXT NOT NULL CHECK (priority IN ('low', 'medium', 'high')),
    status TEXT NOT NULL CHECK (status IN ('todo', 'in_progress', 'done')),
    context_tags JSONB NOT NULL DEFAULT '[]'::jsonb,
    github_url VARCHAR(1024),
    github_issue_number INT,
    github_repo VARCHAR(255),
    estimated_minutes INT CHECK (estimated_minutes IS NULL OR estimated_minutes > 0),
    completed_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deleted_at TIMESTAMPTZ
);

CREATE INDEX IF NOT EXISTS idx_tasks_user_status ON tasks (user_id, status) WHERE deleted_at IS NULL;
CREATE INDEX IF NOT EXISTS idx_tasks_user_priority ON tasks (user_id, priority) WHERE deleted_at IS NULL;
CREATE INDEX IF NOT EXISTS idx_tasks_context_tags ON tasks USING GIN (context_tags);
CREATE INDEX IF NOT EXISTS idx_tasks_github_issue ON tasks (github_repo, github_issue_number) WHERE github_repo IS NOT NULL;
