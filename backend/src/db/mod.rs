//! Database utilities wrapping SQLx connection pools.
//!
//! Axum handlers should not be responsible for constructing or configuring
//! database pools.  Centralizing the logic keeps pooling parameters consistent
//! and allows tests to stub things easily.

use crate::config::{DatabaseConfig, EnvironmentKind};
use sqlx::{postgres::PgPoolOptions, PgPool};

/// Type alias so the rest of the codebase can depend on `DbPool` instead of
/// SQLx's concrete type.  This mirrors how repositories are typically
/// parameterized in layered architectures.
pub type DbPool = PgPool;

/// Build a PostgreSQL connection pool using SQLx.
///
/// The `EnvironmentKind` flag lets us avoid attempting a real connection when
/// tests spin up the pool.  In development/production we still perform a
/// lightweight `pool.acquire().await` to fail fast if PostgreSQL is
/// unavailable.
pub async fn create_pool(
    config: &DatabaseConfig,
    env: EnvironmentKind,
) -> Result<DbPool, sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(config.max_connections)
        // `connect_lazy` defers the TCP handshake so tests without PostgreSQL
        // running can still build a pool.
        .connect_lazy(&config.url)?;

    if env != EnvironmentKind::Test {
        // Acquire/release a connection to ensure credentials/network are valid.
        pool.acquire().await?;
    }

    Ok(pool)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_environment_creates_pool_without_live_database() {
        let cfg = DatabaseConfig {
            url: "postgres://does-not-matter/devflow".into(),
            max_connections: 1,
        };

        let pool = create_pool(&cfg, EnvironmentKind::Test)
            .await
            .expect("Test environment should allow lazy pool creation");

        assert_eq!(pool.size(), 0);
    }
}
