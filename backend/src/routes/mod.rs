//! # API Routes
//!
//! This module defines the main router for the application and combines
//! all the feature specific routers.
//!
//! The `api_router` function is the single entry point for all API routes.
//! It merges routers from sub-modules like `health`, `auth`, etc.
//! This modular approach keeps the routing logic organized and scalable.

pub mod health;
pub mod focus;

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
        .merge(health::router())
        .merge(focus::router())
}
