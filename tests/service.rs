use serde_json::{json, Value};
use tokio::{
    io::{AsyncBufReadExt, BufReader},
    process::Command,
};

#[tokio::test]
async fn authenticated_service_imports_runs_and_persists_reports() {
    let temp = tempfile::tempdir().unwrap();
    // Test-only credential, not a service or provider secret.
    let token = "test-only-http-contract-token-12345678";
    let mut child = Command::new(env!("CARGO_BIN_EXE_market-swarm"))
        .args([
            "--db",
            temp.path().join("service.sqlite").to_str().unwrap(),
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
    assert_eq!(
        client
            .get(format!("{url}/health"))
            .send()
            .await
            .unwrap()
            .status(),
        200
    );
    assert_eq!(
        client
            .get(format!("{url}/evidence"))
            .send()
            .await
            .unwrap()
            .status(),
        401
    );
    let evidence: Value =
        serde_json::from_str(include_str!("../examples/evidence.synthetic.json")).unwrap();
    let response = client
        .post(format!("{url}/evidence"))
        .bearer_auth(token)
        .json(&evidence)
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), 200);
    assert_eq!(response.json::<Value>().await.unwrap()["imported"], 3);
    let request: Value =
        serde_json::from_str(include_str!("../examples/research.synthetic.json")).unwrap();
    let response = client
        .post(format!("{url}/research"))
        .bearer_auth(token)
        .json(&request)
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), 200);
    let report: Value = response.json().await.unwrap();
    assert_eq!(report["status"], "offline_analysis_only");
    assert_eq!(report["agents"].as_array().unwrap().len(), 12);
    assert_eq!(report["numerical"]["wtp"]["conclusion"], "inconclusive");
    let id = report["run_id"].as_str().unwrap();
    let fetched = client
        .get(format!("{url}/reports/{id}"))
        .bearer_auth(token)
        .send()
        .await
        .unwrap()
        .json::<Value>()
        .await
        .unwrap();
    assert_eq!(fetched["run_id"], id);
    let bad = client
        .post(format!("{url}/research"))
        .bearer_auth(token)
        .json(&json!({"question":"missing fields"}))
        .send()
        .await
        .unwrap();
    assert!(!bad.status().is_success());
    child.kill().await.unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_market-swarm"))
        .args([
            "--db",
            temp.path().join("service.sqlite").to_str().unwrap(),
            "report",
            id,
        ])
        .output()
        .await
        .unwrap();
    assert!(output.status.success());
    assert_eq!(
        serde_json::from_slice::<Value>(&output.stdout).unwrap()["run_id"],
        id
    );
}
