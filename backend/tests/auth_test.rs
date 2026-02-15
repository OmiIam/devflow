mod common;

use reqwest::StatusCode;
use serde_json::json;
use tokio::sync::Mutex;

static TEST_GUARD: Mutex<()> = Mutex::const_new(());

#[tokio::test]
async fn register_creates_user_and_returns_token() {
    let _lock = TEST_GUARD.lock().await;
    let addr = common::spawn_app(false).await;
    let client = reqwest::Client::new();

    let payload = json!({
        "name": "New User",
        "email": "register@test.dev",
        "password": "SuperSecure123!"
    });

    let response = client
        .post(format!("http://{}/auth/register", addr))
        .json(&payload)
        .send()
        .await
        .expect("failed to hit /auth/register");

    assert_eq!(response.status(), StatusCode::OK);
    let body: serde_json::Value = response.json().await.expect("json body");
    assert!(body["token"].is_string());
    assert_eq!(body["user"]["email"], payload["email"]);
}

#[tokio::test]
async fn login_returns_token_for_seed_user() {
    let _lock = TEST_GUARD.lock().await;
    let addr = common::spawn_app(true).await;
    let client = reqwest::Client::new();

    let payload = json!({
        "email": "test@example.com",
        "password": "correcthorsebattery"
    });

    let response = client
        .post(format!("http://{}/auth/login", addr))
        .json(&payload)
        .send()
        .await
        .expect("failed to hit /auth/login");

    assert_eq!(response.status(), StatusCode::OK);
    let body: serde_json::Value = response.json().await.expect("json body");
    assert!(body["token"].is_string());
}

#[tokio::test]
async fn me_returns_current_user() {
    let _lock = TEST_GUARD.lock().await;
    let addr = common::spawn_app(true).await;
    let client = reqwest::Client::new();

    let login = client
        .post(format!("http://{}/auth/login", addr))
        .json(&json!({
            "email": "me@example.com",
            "password": "correcthorsebattery"
        }))
        .send()
        .await
        .expect("login");

    assert_eq!(login.status(), StatusCode::OK);
    let body: serde_json::Value = login.json().await.expect("json body");
    let token = body["token"].as_str().expect("token");

    let me = client
        .get(format!("http://{}/auth/me", addr))
        .bearer_auth(token)
        .send()
        .await
        .expect("me");

    assert_eq!(me.status(), StatusCode::OK);
    let me_body: serde_json::Value = me.json().await.expect("json body");
    assert_eq!(me_body["user"]["email"], "me@example.com");
}

#[tokio::test]
async fn logout_returns_success_message() {
    let _lock = TEST_GUARD.lock().await;
    let addr = common::spawn_app(true).await;
    let client = reqwest::Client::new();

    let login = client
        .post(format!("http://{}/auth/login", addr))
        .json(&json!({
            "email": "me@example.com",
            "password": "correcthorsebattery"
        }))
        .send()
        .await
        .expect("login");

    let token = login.json::<serde_json::Value>().await.expect("json body")["token"]
        .as_str()
        .unwrap()
        .to_string();

    let logout = client
        .post(format!("http://{}/auth/logout", addr))
        .bearer_auth(token)
        .send()
        .await
        .expect("logout");

    assert_eq!(logout.status(), StatusCode::OK);
    let body: serde_json::Value = logout.json().await.expect("json body");
    assert_eq!(body["message"], "success");
}
