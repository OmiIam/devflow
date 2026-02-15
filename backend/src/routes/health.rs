//! # Health Check Endpoint
//!
//! This module provides a simple health check endpoint for monitoring
//! and load balancing purposes. It returns the API's status, the
//! current timestamp, and the application version.
//!
//! ## Why have a health check endpoint?
//!
//! Health checks are crucial for production systems. They allow automated
//! services (like Kubernetes, AWS Elastic Load Balancing, or uptime monitors)
//! to verify that the application is running and able to respond to requests.
//! If the health check fails, the service can be automatically restarted or
//! traffic can be routed to a healthy instance.
//!
//! ## Examples
//!
//! A `GET` request to `/health` will yield a JSON response like this:
//!
//! ```json
//! {
//!   "status": "ok",
//!   "timestamp": "2025-02-07T12:00:00Z",
//!   "version": "0.1.0"
//! }
//! ```

use crate::utils::SharedAppState;
use axum::{routing::get, Json, Router};
use chrono::Utc;
use serde::Serialize;

#[derive(Serialize)]
pub struct HealthResponse {
    /// The status of the server. Always "ok" if the server is running.
    status: &'static str,
    /// The current UTC time in ISO 8601 format.
    timestamp: String,
    /// The current version of the API, read from `Cargo.toml`.
    version: &'static str,
}

/// # Health Check Handler
///
/// This is the `async` function that gets called when a `GET` request is made
/// to the `/health` route. It constructs and returns the `HealthResponse`.
///
/// ## Return Type: `Json<HealthResponse>`
///
/// - `HealthResponse`: The struct we defined above.
/// - `Json(...)`: This is an "extractor" provided by Axum. When used as a
///   return type, it takes our `HealthResponse` struct, serializes it into a
///   JSON string, and sets the `Content-Type` HTTP header to `application/json`.
///   This is an elegant way to create JSON responses.
pub async fn health_check() -> Json<HealthResponse> {
    let response = HealthResponse {
        status: "ok",
        timestamp: Utc::now().to_rfc3339(),
        version: env!("CARGO_PKG_VERSION"),
    };

    // The `Json` wrapper handles the serialization and response headers.
    Json(response)
}

/// # Creates the Health Check Router
///
/// This function encapsulates the routing logic for the health check feature.
/// It creates a new `Router` and defines the `/health` endpoint.
///
/// ## Why a function?
///
/// By creating a `router()` function in each route module, i can keep the
/// routing logic organized and decoupled. The main `main.rs` file can then
//  merge these modular routers together, rather than having one giant list
//  of all routes in one place. This is a key pattern for building maintainable
//  web applications in Axum.
///
/// # Returns
/// * `Router` - An Axum router with the `/health` endpoint configured.
pub fn router() -> Router<SharedAppState> {
    Router::new()
        // `get()` creates a route that only matches GET requests.
        // pass the `health_check` handler function to it.
        .route("/health", get(health_check))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_health_check_returns_ok() {
        // Arrange & Act: Call the handler function directly.
        let response = health_check().await;

        // Assert: Check the value of the `status` field.
        // The `Json` wrapper is a tuple-struct, so we can access the inner
        // `HealthResponse` with `.0`.
        assert_eq!(response.0.status, "ok");
    }

    #[tokio::test]
    async fn test_health_check_has_timestamp_and_version() {
        // Arrange & Act
        let response = health_check().await;

        // Assert
        // Check that the timestamp is not an empty string.
        assert!(!response.0.timestamp.is_empty());
        // Check that the version matches what's in Cargo.toml.
        assert_eq!(response.0.version, env!("CARGO_PKG_VERSION"));
    }
}
