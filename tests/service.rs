use serde_json::{json, Value};
use tokio::{
    io::{AsyncBufReadExt, BufReader},
    process::Command,
};

#[tokio::test]
async fn requested_default_model_is_used_without_sending_uncredentialed_requests() {
    let temp = tempfile::tempdir().unwrap();
    let binary = env!("CARGO_BIN_EXE_market-swarm");
    let db = temp.path().join("model-default.sqlite");
    let output = Command::new(binary)
        .args(["--db", db.to_str().unwrap(), "doctor"])
        .env_remove("OPENROUTER_MODEL")
        .env_remove("OPENROUTER_API_KEY")
        .env_remove("OPENROUTER_KEY")
        .env_remove("openrouter_key")
        .output()
        .await
        .unwrap();
    assert!(output.status.success());
    let info: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(
        info["effective_openrouter_model"],
        "nvidia/nemotron-3.5-super-vl"
    );
    assert_eq!(info["credentials"]["OPENROUTER_API_KEY"], false);

    let output = Command::new(binary)
        .args(["--db", db.to_str().unwrap(), "doctor"])
        .env("OPENROUTER_MODEL", "test/explicit-override")
        .env_remove("OPENROUTER_API_KEY")
        .env_remove("OPENROUTER_KEY")
        .env_remove("openrouter_key")
        .output()
        .await
        .unwrap();
    let info: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(info["effective_openrouter_model"], "test/explicit-override");

    let fixture =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("examples/research.synthetic.json");
    let output = Command::new(binary)
        .args([
            "--db",
            db.to_str().unwrap(),
            "run",
            fixture.to_str().unwrap(),
            "--mode",
            "openrouter",
        ])
        .env_remove("OPENROUTER_MODEL")
        .env_remove("OPENROUTER_API_KEY")
        .env_remove("OPENROUTER_KEY")
        .env_remove("openrouter_key")
        .output()
        .await
        .unwrap();
    assert!(!output.status.success());
    let error = String::from_utf8_lossy(&output.stderr);
    assert!(error.contains("Missing server credential: OPENROUTER_API_KEY"));
    assert!(!error.contains("Set --model"));
}

#[tokio::test]
async fn lowercase_openrouter_key_is_loaded_without_disclosing_it() {
    let temp = tempfile::tempdir().unwrap();
    let env_file = temp.path().join("alias.env");
    let test_value = "test-only-credential-alias-not-a-real-provider-key";
    std::fs::write(&env_file, format!("openrouter_key={test_value}\n")).unwrap();
    let db = temp.path().join("alias.sqlite");
    let output = Command::new(env!("CARGO_BIN_EXE_market-swarm"))
        .args([
            "--db",
            db.to_str().unwrap(),
            "--env-file",
            env_file.to_str().unwrap(),
            "doctor",
        ])
        .env_remove("OPENROUTER_API_KEY")
        .env_remove("OPENROUTER_KEY")
        .env_remove("openrouter_key")
        .output()
        .await
        .unwrap();
    assert!(output.status.success());
    let text = String::from_utf8(output.stdout).unwrap();
    let info: Value = serde_json::from_str(&text).unwrap();
    assert_eq!(info["credentials"]["openrouter_key"], true);
    assert_eq!(info["openrouter_credential_available"], true);
    assert!(!text.contains(test_value));
    assert!(!String::from_utf8_lossy(&output.stderr).contains(test_value));
}

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
