use serde_json::{json, Value};
use tokio::{
    io::{AsyncBufReadExt, BufReader},
    process::Command,
};

#[tokio::test]
async fn cli_audit_writes_a_prioritized_report_offline() {
    let temp = tempfile::tempdir().unwrap();
    let out = temp.path().join("audit");
    let fixture =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("examples/ux.meditation.pen");
    let output = Command::new(env!("CARGO_BIN_EXE_market-swarm"))
        .args([
            "--db",
            temp.path().join("ux.sqlite").to_str().unwrap(),
            "ux",
            "audit",
            fixture.to_str().unwrap(),
            "--out",
            out.to_str().unwrap(),
            "--as-of",
            "2026-10-25T00:00:00Z",
            "--category",
            "meditation",
            "--import",
        ])
        .output()
        .await
        .unwrap();
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let report: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(report["status"], "deterministic");
    assert_eq!(report["stats"]["screens"], 3);
    assert!(report["stats"]["contrast_failures"].as_u64().unwrap() >= 1);
    assert!(report["imported"].as_u64().unwrap() >= 4);
    let text = std::fs::read_to_string(out.join("report.txt")).unwrap();
    assert!(text.contains("HACER AHORA"));
    assert!(text.contains("No son usuarios reales"));
    let again = Command::new(env!("CARGO_BIN_EXE_market-swarm"))
        .args([
            "--db",
            temp.path().join("ux.sqlite").to_str().unwrap(),
            "ux",
            "audit",
            fixture.to_str().unwrap(),
            "--as-of",
            "2026-10-25T00:00:00Z",
            "--category",
            "meditation",
            "--import",
        ])
        .output()
        .await
        .unwrap();
    assert!(again.status.success());
    let second: Value = serde_json::from_slice(&again.stdout).unwrap();
    assert_eq!(second["imported"], 0);

    let catalog = Command::new(env!("CARGO_BIN_EXE_market-swarm"))
        .args([
            "--db",
            temp.path().join("ux.sqlite").to_str().unwrap(),
            "ux",
            "specialists",
        ])
        .output()
        .await
        .unwrap();
    assert!(catalog.status.success());
    let roles: Vec<Value> = serde_json::from_slice(&catalog.stdout).unwrap();
    assert!(roles.iter().any(|role| role["id"] == "visual_design"));
    assert!(roles
        .iter()
        .any(|role| role["id"] == "competitive_benchmark"));
}

#[tokio::test]
async fn ux_audit_endpoint_is_authenticated_and_deterministic() {
    let temp = tempfile::tempdir().unwrap();
    let token = "test-only-http-contract-token-12345678";
    let mut child = Command::new(env!("CARGO_BIN_EXE_market-swarm"))
        .args([
            "--db",
            temp.path().join("ux-http.sqlite").to_str().unwrap(),
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
    let line = tokio::time::timeout(std::time::Duration::from_secs(15), lines.next_line())
        .await
        .unwrap()
        .unwrap()
        .unwrap();
    let url = line.split_whitespace().nth(2).unwrap();
    let client = reqwest::Client::new();
    let body = json!({
        "pen": {
            "version": "2.21",
            "children": [{
                "id": "s",
                "type": "frame",
                "name": "Home",
                "width": 390,
                "height": 844,
                "fill": "#FFFFFF",
                "children": [{
                    "id": "t",
                    "type": "text",
                    "name": "Primary CTA",
                    "content": "Continuar",
                    "fill": "#111111",
                    "fontSize": 16
                }]
            }]
        },
        "question": "¿El CTA se lee?",
        "as_of": "2026-10-25T00:00:00Z"
    });
    let unauth = client
        .post(format!("{url}/ux/audit"))
        .json(&body)
        .send()
        .await
        .unwrap();
    assert_eq!(unauth.status(), 401);
    let ok = client
        .post(format!("{url}/ux/audit"))
        .bearer_auth(token)
        .json(&body)
        .send()
        .await
        .unwrap();
    let status = ok.status();
    let report: Value = ok.json().await.unwrap_or(json!({"error": "not json"}));
    assert_eq!(status, 200, "{report}");
    assert_eq!(report["stats"]["screens"], 1);
    assert_eq!(report["status"], "deterministic");
    assert!(report["synthetic_panel_notice"]
        .as_str()
        .unwrap()
        .contains("No son usuarios reales"));
    child.kill().await.unwrap();
}
