//! Shared application state stored in Axum's state extension.
//!
//! Axum's `Router` allows us to attach arbitrary state that handlers can
//! retrieve via the `State<T>` extractor.  We wrap the configuration and
//! database pool in a single struct so it can be cloned cheaply via `Arc`.

use crate::{config::AppConfig, db::DbPool};
use std::sync::Arc;

/// Bundle of cross-cutting dependencies handed to request handlers.
#[derive(Clone)]
pub struct AppState {
    /// Loaded configuration.  Cloned into the state because handlers may need
    /// feature flags or auth settings.
    pub config: AppConfig,
    /// SQLx connection pool used by repositories/services.
    pub db_pool: DbPool,
}

impl AppState {
    /// Convenience constructor so callers can write `AppState::new(...)`
    /// instead of struct literal syntax every time.
    pub fn new(config: AppConfig, db_pool: DbPool) -> Self {
        Self { config, db_pool }
    }
}

/// Reference-counted pointer to `AppState` suitable for Axum's state system.
pub type SharedAppState = Arc<AppState>;
