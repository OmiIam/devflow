//! # Common Test Helpers
//!
//! This module contains shared utilities for integration tests, such as
//! functions to spawn the application server in the background for testing.
//!
//! By centralizing these helpers, i can keep the test code DRY (Don't Repeat Yourself)
//! and make integration tests cleaner and easier to write.

use axum::Router;
use devflow_backend::routes::api_router;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;

/// # Spawn App and Return Address
///
/// This function sets up the application's router, spawns the server on a
/// random available port, and returns the `SocketAddr` of the running server.
///
/// This is the standard pattern for writing integration tests for an Axum
/// application. It ensures that each test (or test suite) runs against a
/// fresh, isolated instance of the server.
///
/// ## Returns
///
/// A `SocketAddr` that can be used by an HTTP client (like `reqwest`) to
/// send requests to the test server.
pub async fn spawn_app() -> SocketAddr {
    // Build the application router with all the routes.
    let app = Router::new()
        .merge(api_router())
        .layer(TraceLayer::new_for_http());

    // Bind to a random available port on the loopback address.
    // `0` is a special port number that tells the OS to pick a random, unused port.
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();

    // Spawn the server in a background task.
    // `tokio::spawn` runs the server in a separate green thread, so the main
    // test function can continue and make requests to it.
    tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    addr
}
