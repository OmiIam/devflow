-- migrations/001_create_users_table.sql
--
-- The `users` table stores the core authentication information for accounts.
--
-- - `id`: A unique identifier for each user, using UUIDs for obscurity.
-- - `email`: The user's email address, used for login. It must be unique.
-- - `password_hash`: The user's hashed password. NEVER store plain text passwords.
-- - `created_at`/`updated_at`: Timestamps for record management, using `TIMESTAMPTZ`
--   to ensure time zone correctness.

CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    email VARCHAR(255) NOT NULL UNIQUE,
    password_hash VARCHAR(255) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Create an index on the email column for faster lookups during login.
CREATE INDEX idx_users_email ON users(email);
