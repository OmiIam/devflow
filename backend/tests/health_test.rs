mod common;

use serde_json::Value;

#[tokio::test]
async fn test_health_endpoint_returns_200_ok_and_correct_body() {
    let addr = common::spawn_app(false).await;
    let client = reqwest::Client::new();
    let url = format!("http://{}/health", addr);

    let response = client
        .get(&url)
        .send()
        .await
        .expect("Failed to send request to health endpoint.");

    assert_eq!(response.status(), reqwest::StatusCode::OK);

    let body: Value = response
        .json()
        .await
        .expect("Failed to parse response body as JSON.");

    assert_eq!(body["status"], "ok");
    assert!(body["timestamp"].is_string());
    assert_eq!(body["version"], env!("CARGO_PKG_VERSION"));
}
