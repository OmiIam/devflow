# DevFlow Database Schema (MVP)

This note captures the first real set of tables we are introducing.  
Per the implementation workflow, documentation comes before code so that we can align on intent before writing migrations.

## Guiding Principles
- **Single source of truth:** PostgreSQL remains authoritative for users/tasks/focus.
- **Append-only mindset:** prefer explicit `deleted_at` columns over hard deletes.
- **Query-driven indexes:** each index listed below corresponds to a planned API query.

## Tables

### `users`
| Column | Type | Notes |
| --- | --- | --- |
| `id` | `UUID` | PK via `uuid_generate_v4()` |
| `email` | `VARCHAR(255)` | Unique constraint |
| `password_hash` | `VARCHAR(255)` | Argon2 output |
| `name` | `VARCHAR(255)` | Display name |
| `github_id` | `VARCHAR(255)` | Nullable, used for linking |
| `github_access_token` | `TEXT` | Nullable, encrypted at rest later |
| `created_at` | `TIMESTAMPTZ` | Default `NOW()` |
| `updated_at` | `TIMESTAMPTZ` | Default `NOW()` and trigger later |

Indexes: `email` unique constraint, `github_id`.

### `tasks`
| Column | Type | Notes |
| --- | --- | --- |
| `id` | `UUID` | PK default uuid gen |
| `user_id` | `UUID` | FK → users(id) |
| `title` | `VARCHAR(500)` | Required |
| `description` | `TEXT` | Optional |
| `priority` | `TEXT` enum check (`low`, `medium`, `high`) |
| `status` | `TEXT` enum check (`todo`, `in_progress`, `done`) |
| `context_tags` | `JSONB` | Defaults to empty array |
| `github_url` | `VARCHAR(1024)` | Optional |
| `estimated_minutes` | `INT` | Optional, must be positive |
| `completed_at` | `TIMESTAMPTZ` | Nullable |
| `created_at` | `TIMESTAMPTZ` | Default `NOW()` |
| `updated_at` | `TIMESTAMPTZ` | Default `NOW()` |
| `deleted_at` | `TIMESTAMPTZ` | Soft delete |

Indexes: `(user_id, status)`, `(user_id, priority)`, `GIN(context_tags)`.

### `focus_sessions`
| Column | Type | Notes |
| --- | --- | --- |
| `id` | `UUID` | PK |
| `user_id` | `UUID` | FK → users |
| `task_id` | `UUID` | Nullable FK → tasks |
| `duration_seconds` | `INT` | Planned duration |
| `actual_duration_seconds` | `INT` | Nullable actual duration |
| `started_at` | `TIMESTAMPTZ` | Required |
| `ended_at` | `TIMESTAMPTZ` | Nullable |
| `completed` | `BOOL` | Default false |
| `notes` | `TEXT` | Optional reflection |
| `created_at` | `TIMESTAMPTZ` | Default `NOW()` |

Indexes: `(user_id, started_at DESC)`, `task_id`, partial `(user_id, ended_at)` where `ended_at IS NULL`.

### `interruptions`
| Column | Type | Notes |
| --- | --- | --- |
| `id` | `UUID` | PK |
| `session_id` | `UUID` | FK → focus_sessions |
| `category` | `TEXT` enum check (`external`, `internal`, `urgent`, `break`) |
| `reason` | `TEXT` | Optional |
| `duration_seconds` | `INT` | Optional |
| `created_at` | `TIMESTAMPTZ` | Default `NOW()` |

Index: `(session_id, created_at)`.

## Next Steps
1. Translate this document into SQLx migrations (`002_tasks.sql`, `003_focus_sessions.sql`, `004_interruptions.sql`).
2. Mirror the schema in Rust models/DTOs so repositories stay type-safe.
3. Add integration tests (later) to assert referential integrity.
