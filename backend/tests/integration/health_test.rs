//! # Integration Tests for the Health Check Endpoint

// This line declares the `common` module, which makes functions from
// `tests/common/mod.rs` available to this test file.
mod common;

use reqwest;
use serde_json::Value;

#[tokio::test]
async fn test_health_endpoint_returns_200_ok_and_correct_body() {
    // Arrange
    // Spawn the server in the background and get its address.
    let addr = common::spawn_app().await;
    let client = reqwest::Client::new();
    let url = format!("http://{}/health", addr);

    // Act
    let response = client.get(&url)
        .send()
        .await
        .expect("Failed to send request to health endpoint.");

    // Assert Status
    assert_eq!(response.status(), reqwest::StatusCode::OK);

    // Assert Body
    let body: Value = response.json()
        .await
        .expect("Failed to parse response body as JSON.");

    assert_eq!(body["status"], "ok");
    assert!(body["timestamp"].is_string());
    assert_eq!(body["version"], env!("CARGO_PKG_VERSION"));
}