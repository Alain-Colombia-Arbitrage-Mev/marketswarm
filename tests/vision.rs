use serde_json::json;
use tokio::{
    io::{AsyncBufReadExt, BufReader},
    process::Command,
};

#[tokio::test]
async fn vision_endpoint_requires_auth_and_rejects_offline() {
    let temp = tempfile::tempdir().unwrap();
    // Test-only token, not a service or provider secret.
    let token = "test-only-http-contract-token-12345678";
    let mut child = Command::new(env!("CARGO_BIN_EXE_market-swarm"))
        .args([
            "--db",
            temp.path().join("vision.sqlite").to_str().unwrap(),
            "serve",
            "--mode",
            "offline",
            "--port",
            "0",
        ])
        .env("SWARM_API_TOKEN", token)
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .kill_on_drop(true)
        .spawn()
        .unwrap();
    let mut lines = BufReader::new(child.stdout.take().unwrap()).lines();
    let line = tokio::time::timeout(std::time::Duration::from_secs(10), lines.next_line())
        .await
        .unwrap()
        .unwrap()
        .unwrap();
    let url = line.split_whitespace().nth(2).unwrap();
    let client = reqwest::Client::new();
    let body = json!({"images":[{"mime":"image/png","data_base64":"aGVsbG8="}]});

    // Authentication is enforced before anything else.
    let unauth = client
        .post(format!("{url}/vision"))
        .json(&body)
        .send()
        .await
        .unwrap();
    assert_eq!(unauth.status(), 401);

    // Offline mode has no live model, so a valid authenticated request is
    // rejected with 503 rather than attempting an uncredentialed call.
    let offline = client
        .post(format!("{url}/vision"))
        .bearer_auth(token)
        .json(&body)
        .send()
        .await
        .unwrap();
    assert_eq!(offline.status(), 503);

    child.kill().await.unwrap();
}
