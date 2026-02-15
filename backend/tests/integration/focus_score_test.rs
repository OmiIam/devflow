mod common;

use reqwest::StatusCode;
use serde_json::json;

#[tokio::test]
async fn focus_score_returns_expected_value() {
    let addr = common::spawn_app().await;
    let client = reqwest::Client::new();

    let payload = json!({
        "duration_minutes": 25,
        "interruptions": 1,
        "state": "completed"
    });

    let response = client
        .post(format!("http://{}/focus/score", addr))
        .json(&payload)
        .send()
        .await
        .expect("failed to send request");

    assert_eq!(response.status(), StatusCode::OK);
    let body: serde_json::Value = response.json().await.expect("invalid json body");
    assert!(body["score"].as_f64().is_some());
}

#[tokio::test]
async fn focus_score_rejects_invalid_duration() {
    let addr = common::spawn_app().await;
    let client = reqwest::Client::new();

    let payload = json!({
        "duration_minutes": 0,
        "interruptions": 1,
        "state": "completed"
    });

    let response = client
        .post(format!("http://{}/focus/score", addr))
        .json(&payload)
        .send()
        .await
        .expect("failed to send request");

    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
}
