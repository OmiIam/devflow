use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// Represents an application user persisted in PostgreSQL.
///
/// We keep this struct focused on the domain attributes (no passwords in plain
/// text, only the derived hash) so it can safely cross service/repository
/// boundaries.  The struct derives `Serialize`/`Deserialize` for test fixtures
/// and JSON conversions.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    /// Primary key generated as a UUID v4.
    pub id: Uuid,
    /// Public facing email address.  Unique constraint enforced via SQL.
    pub email: String,
    /// Bcrypt/Argon2 hash stored instead of the raw password.
    pub password_hash: String,
    /// Display name shown in UI.
    pub name: String,
    /// Optional GitHub user identifier when linked.
    pub github_id: Option<String>,
    /// OAuth access token required for GitHub API access.
    pub github_access_token: Option<String>,
    /// Timestamp for auditing when the record was inserted.
    pub created_at: DateTime<Utc>,
    /// Timestamp updated via database trigger/default on mutation.
    pub updated_at: DateTime<Utc>,
}
