//! # API Routes
//!
//! This module defines the main router for the application and combines
//! all the feature specific routers.
//!
//! The `api_router` function is the single entry point for all API routes.
//! It merges routers from sub-modules like `health`, `auth`, etc.
//! This modular approach keeps the routing logic organized and scalable.

pub mod health;

use axum::Router;

/// # Combine all API routes
///
/// This function builds the master API router by merging routers from
/// the different feature modules.
///
/// ## Returns
/// * `Router` - The combined Axum router for the entire API.
pub fn api_router() -> Router {
    Router::new()
        // Merge the health check router.
        // The `.merge()` method is a convenient way to combine routers.
        .merge(health::router())
    // will need to add more routers here as i build them.
    // .merge(auth::router())
    // .merge(tasks::router())
}
