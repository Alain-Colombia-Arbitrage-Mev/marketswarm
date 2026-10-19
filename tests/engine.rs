use chrono::{DateTime, Utc};
use market_swarm::{
    analysis, connectors, evaluation, inference::Inference, runtime, schema::*, store::Store,
    wtp::*,
};
use serde_json::json;
use std::{
    collections::BTreeSet,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
};

fn date(s: &str) -> DateTime<Utc> {
    DateTime::parse_from_rfc3339(s).unwrap().with_timezone(&Utc)
}
fn evidence(id: &str, kind: EvidenceKind, synthetic: bool) -> Evidence {
    Evidence {
        id: id.into(),
        source: format!("test:{id}"),
        provider: "test_fixture".into(),
        kind,
        published_at: None,
        collected_at: date("2026-04-01T00:00:00Z"),
        observed_at: date("2026-03-01T00:00:00Z"),
        segment: Some("test".into()),
        population: None,
        content: "Me parece útil pero el precio es caro. El soporte no es malo.".into(),
        limitations: vec!["Test fixture, never real research".into()],
        synthetic_fixture: synthetic,
    }
}
fn request() -> ResearchRequest {
    ResearchRequest {
        question: "Study a test product and compare hypotheses".into(),
        as_of: date("2026-04-01T00:00:00Z"),
        modules: Module::ALL.to_vec(),
        evidence_ids: vec!["e1".into()],
        context: json!({}),
        limits: Limits::default(),
        study: None,
        economy: None,
    }
}
fn study(kind: EvidenceKind) -> Study {
    let mut responses = vec![];
    for group in 0..2 {
        for i in 0..100 {
            let accepted = i < if group == 0 { 40 } else { 25 };
            responses.push(Response {
                participant_id: format!("p{group}-{i}"),
                price_cents: if group == 0 { 500 } else { 1000 },
                outcome: match kind {
                    EvidenceKind::Observed => {
                        if accepted {
                            Outcome::Paid
                        } else {
                            Outcome::NoPurchase
                        }
                    }
                    _ => {
                        if accepted {
                            Outcome::Yes
                        } else {
                            Outcome::No
                        }
                    }
                },
                evidence_id: "responses".into(),
                recorded_at: date("2026-02-01T00:00:00Z"),
            });
        }
    }
    Study {
        offer: "test-offer".into(),
        segment: "test".into(),
        currency: "USD".into(),
        billing_period: "monthly".into(),
        kind,
        protocol: Protocol {
            method: if kind == EvidenceKind::Observed {
                "randomized_offer"
            } else {
                "gabor_granger"
            }
            .into(),
            registered_at: date("2026-01-01T00:00:00Z"),
            closes_at: date("2026-03-01T00:00:00Z"),
            minimum_per_price: 100,
            maximum_half_width: 0.1,
            population: "Test fixture population".into(),
            allocation: "randomized".into(),
            maturity_days: 7,
        },
        responses,
    }
}
fn economy() -> Economy {
    Economy {
        currency: "USD".into(),
        seed: 42,
        repetitions: 100,
        scenarios: vec![Scenario {
            name: "certain".into(),
            visitors: 100,
            price_cents: 12000,
            period_months: 12,
            cycles: 1,
            conversion: 1.,
            renewal: 1.,
            refund_rate: 0.,
            billing_failure: 0.,
            fee_rate: 0.1,
            tax_inclusive_rate: 0.2,
            service_cost_per_month_cents: 100,
            acquisition_cost_per_visitor_cents: 200,
            assumption_evidence_ids: vec![],
        }],
    }
}
#[test]
fn evidence_import_is_atomic_immutable_and_idempotent() {
    let d = tempfile::tempdir().unwrap();
    let s = Store::open(d.path().join("db")).unwrap();
    let e = evidence("e1", EvidenceKind::Observed, true);
    assert_eq!(s.import(std::slice::from_ref(&e)).unwrap(), 1);
    assert_eq!(s.import(std::slice::from_ref(&e)).unwrap(), 0);
    let mut later = e.clone();
    later.collected_at += chrono::Duration::hours(1);
    assert_eq!(s.import(&[later]).unwrap(), 0);
    let mut changed = e.clone();
    changed.content = "Different".into();
    assert!(s
        .import(&[evidence("e2", EvidenceKind::Observed, true), changed])
        .is_err());
    assert_eq!(s.list_evidence().unwrap().len(), 1);
}
#[test]
fn future_publication_and_observation_are_excluded() {
    let mut e = evidence("e", EvidenceKind::Benchmark, false);
    let cutoff = date("2026-02-01T00:00:00Z");
    assert!(!e.eligible(cutoff));
    e.observed_at = date("2026-01-01T00:00:00Z");
    e.published_at = Some(date("2026-03-01T00:00:00Z"));
    assert!(!e.eligible(cutoff));
}
#[test]
fn invented_citations_are_rejected() {
    let output = AgentOutput {
        summary: "test".into(),
        findings: vec![Finding {
            statement: "claim".into(),
            evidence_ids: vec!["invented".into()],
            interpretation: "observation".into(),
        }],
        hypotheses: vec![],
        gaps: vec![],
        next_experiments: vec![],
    };
    assert!(output.validate(&BTreeSet::from(["real".into()])).is_err());
}
#[test]
fn openrouter_catalog_without_per_request_price_is_supported() {
    let entry = json!({"pricing":{"prompt":"0.000001","completion":"0.000002"}});
    assert_eq!(
        market_swarm::inference::catalog_rates(&entry).unwrap(),
        (0.000001, 0.000002, 0.)
    );
    assert!(market_swarm::inference::catalog_rates(
        &json!({"pricing":{"prompt":"NaN","completion":"0"}})
    )
    .is_err());
    assert!(market_swarm::inference::catalog_rates(&json!({"pricing":{"prompt":"0"}})).is_err());
}
#[test]
fn retrieval_selects_relevant_late_sections_and_rejects_secret_sources() {
    let content = format!(
        "{} Useful willingness pricing trial conversion evidence {}",
        "unrelated ".repeat(700),
        "filler ".repeat(300)
    );
    let selected = runtime::excerpt(&content, "pricing trial conversion");
    assert!(selected.contains("Useful willingness pricing trial conversion evidence"));
    let mut e = evidence("e1", EvidenceKind::Observed, true);
    e.source = "https://example.com?api_key=secret".into();
    assert!(e.validate().is_err());
    assert!(connectors::safe_reference("https://example.com/watch?v=public-id").is_ok());
}
#[test]
fn human_wtp_cannot_be_transferred_to_a_different_segment() {
    let mut e = evidence("responses", EvidenceKind::Observed, false);
    e.segment = Some("another segment".into());
    assert!(evaluate(
        &study(EvidenceKind::Observed),
        &[e],
        date("2026-04-01T00:00:00Z")
    )
    .is_err());
}
#[test]
fn observed_study_has_scoped_acceptance_not_individual_maximum() {
    let result = evaluate(
        &study(EvidenceKind::Observed),
        &[evidence("responses", EvidenceKind::Observed, false)],
        date("2026-04-01T00:00:00Z"),
    )
    .unwrap();
    assert_eq!(result.conclusion, "supported_in_scope");
    assert_eq!(result.curve[0].acceptance, 0.4);
    assert_eq!(result.curve[1].acceptance, 0.25);
    assert_eq!(result.curve[1].gross_per_exposed_cents, 250.);
}
#[test]
fn synthetic_or_fixture_responses_cannot_validate_humans() {
    for kind in [
        EvidenceKind::Simulated,
        EvidenceKind::Stated,
        EvidenceKind::Observed,
    ] {
        let result = evaluate(
            &study(kind),
            &[evidence("responses", kind, true)],
            date("2026-04-01T00:00:00Z"),
        )
        .unwrap();
        assert_eq!(result.conclusion, "inconclusive");
    }
    let result = evaluate(
        &study(EvidenceKind::Simulated),
        &[evidence("responses", EvidenceKind::Simulated, false)],
        date("2026-04-01T00:00:00Z"),
    )
    .unwrap();
    assert_eq!(result.conclusion, "inconclusive");
}
#[test]
fn trials_deposits_and_immature_payments_do_not_establish_price_acceptance() {
    for outcome in [Outcome::TrialStarted, Outcome::Deposit] {
        let mut s = study(EvidenceKind::Observed);
        s.responses[0].outcome = outcome;
        assert_eq!(
            evaluate(
                &s,
                &[evidence("responses", EvidenceKind::Observed, false)],
                date("2026-04-01T00:00:00Z")
            )
            .unwrap()
            .conclusion,
            "inconclusive"
        );
    }
    let s = study(EvidenceKind::Observed);
    let mut early = evidence("responses", EvidenceKind::Observed, false);
    early.observed_at = date("2026-02-01T00:00:00Z");
    assert_eq!(
        evaluate(&s, &[early], date("2026-02-02T00:00:00Z"))
            .unwrap()
            .conclusion,
        "inconclusive"
    );
}
#[test]
fn study_rejects_post_hoc_protocol_duplicate_participants_and_wrong_provenance() {
    let mut s = study(EvidenceKind::Observed);
    s.protocol.registered_at = date("2026-02-02T00:00:00Z");
    assert!(s.validate().is_err());
    let mut s = study(EvidenceKind::Observed);
    s.responses[1].participant_id = s.responses[0].participant_id.clone();
    assert!(s.validate().is_err());
    assert!(evaluate(
        &study(EvidenceKind::Observed),
        &[evidence("responses", EvidenceKind::Stated, false)],
        date("2026-04-01T00:00:00Z")
    )
    .is_err());
}
#[test]
fn wilson_intervals_handle_zero_and_all_successes() {
    assert_eq!(wilson(0, 0), [0., 1.]);
    let low = wilson(0, 100);
    let high = wilson(100, 100);
    assert!(low[0] < 1e-10 && low[1] > 0.03);
    assert!(high[1] > 0.999999 && high[0] < 0.97);
}
#[test]
fn annual_cash_is_not_counted_as_twelve_purchases_and_fees_apply_once() {
    let report = simulate(&economy()).unwrap().remove(0);
    // 100*12000 gross, /1.2 tax, -120000 fees, -120000 service, -20000 CAC.
    assert_eq!(report.expected_gross_cents, 1_200_000.);
    assert!((report.expected_net_cents - 880_000.).abs() < 1e-6);
    assert!((report.expected_contribution_cents - 740_000.).abs() < 1e-6);
    assert_eq!(report.horizon_months, 12);
    assert_eq!(report.contribution_quantiles_cents, [740_000.; 3]);
}
#[test]
fn monte_carlo_reproducibility_rates_and_cpu_limits() {
    let mut e = economy();
    e.scenarios[0].conversion = 0.3;
    e.scenarios[0].renewal = 0.8;
    e.scenarios[0].cycles = 3;
    assert_eq!(
        serde_json::to_value(simulate(&e).unwrap()).unwrap(),
        serde_json::to_value(simulate(&e).unwrap()).unwrap()
    );
    e.scenarios[0].conversion = 1.2;
    assert!(simulate(&e).is_err());
    e.scenarios[0].conversion = 0.2;
    e.scenarios[0].visitors = 100_000;
    e.repetitions = 2000;
    assert!(simulate(&e).is_err());
}
#[test]
fn sentiment_preserves_mixed_attributes_and_negation() {
    let v = analysis::sentiment(&[evidence("review", EvidenceKind::Observed, true)]);
    let rows = v["results"].as_array().unwrap();
    assert!(rows
        .iter()
        .any(|r| r["aspect"] == "utility" && r["label"] == "positive"));
    assert!(rows
        .iter()
        .any(|r| r["aspect"] == "price" && r["label"] == "negative"));
    assert!(rows
        .iter()
        .any(|r| r["aspect"] == "support" && r["label"] == "positive"));
}
#[test]
fn sentiment_separates_opposing_attributes_inside_one_clause() {
    let mut e = evidence("review", EvidenceKind::Observed, true);
    e.content = "calidad excelente y precio caro".into();
    let v = analysis::sentiment(&[e]);
    let rows = v["results"].as_array().unwrap();
    assert!(rows
        .iter()
        .any(|r| r["aspect"] == "quality" && r["label"] == "positive"));
    assert!(rows
        .iter()
        .any(|r| r["aspect"] == "price" && r["label"] == "negative"));
}
#[test]
fn financial_analysis_rejects_future_and_invalid_bars() {
    let bars: Vec<analysis::Bar> = (1..=5)
        .map(|i| analysis::Bar {
            at: date(&format!("2026-01-0{i}T00:00:00Z")),
            open: 100. + i as f64,
            high: 102. + i as f64,
            low: 99. + i as f64,
            close: 101. + i as f64,
            volume: 100.,
        })
        .collect();
    assert!(analysis::market(&bars, date("2026-01-04T00:00:00Z")).is_err());
    let result = analysis::market(&bars, date("2026-01-06T00:00:00Z")).unwrap();
    assert_eq!(result["bars"], 5);
    let mut bad = bars.clone();
    bad[0].high = 1.;
    assert!(analysis::market(&bad, date("2026-01-06T00:00:00Z")).is_err());
}
#[test]
fn recommendation_checks_currency_period_requirements_and_no_purchase() {
    let catalog = vec![analysis::Product {
        id: "annual".into(),
        evidence_id: "e1".into(),
        currency: "USD".into(),
        billing_period: "annual".into(),
        price_cents: 1000,
        available: true,
        observed_at: date("2026-01-01T00:00:00Z"),
        features: vec!["audio".into()],
    }];
    let buyer = analysis::Buyer {
        currency: "USD".into(),
        billing_period: "monthly".into(),
        budget_cents: 2000,
        required_features: vec!["audio".into()],
        desired_features: vec![],
    };
    let v = analysis::recommend(
        &catalog,
        &buyer,
        &[evidence("e1", EvidenceKind::Observed, true)],
        date("2026-04-01T00:00:00Z"),
    )
    .unwrap();
    assert!(v["options"].as_array().unwrap().is_empty());
    assert_eq!(v["no_purchase_available"], true);
}
#[test]
fn social_model_does_not_promote_adoption_to_human_wtp() {
    let s = analysis::SocialScenario {
        seed: 5,
        price_cents: 100,
        rounds: 3,
        personas: vec![analysis::Persona {
            id: "a".into(),
            budget_cents: 0,
            interest: 1.,
            influence: 1.,
            evidence_ids: vec!["e1".into()],
        }],
        edges: vec![],
    };
    let result = analysis::social(&s, &[evidence("e1", EvidenceKind::Observed, true)]).unwrap();
    assert_eq!(result["human_wtp_validated"], false);
    assert_eq!(result["history"][2]["synthetic_adopters"], 0);
}
#[test]
fn budget_is_atomic_across_workers_and_survives_reopen() {
    let d = tempfile::tempdir().unwrap();
    let path = d.path().join("db");
    let s = Store::open(&path).unwrap();
    let mut threads = vec![];
    for _ in 0..10 {
        let s = s.clone();
        threads.push(std::thread::spawn(move || {
            s.reserve("run", 0.2, 1.).is_ok()
        }));
    }
    let successes = threads
        .into_iter()
        .map(|t| t.join().unwrap())
        .filter(|b| *b)
        .count();
    assert_eq!(successes, 5);
    assert!((Store::open(&path).unwrap().reserved("run").unwrap() - 1.).abs() < 1e-9);
}
#[test]
fn duplicate_run_lease_is_rejected_and_released() {
    let d = tempfile::tempdir().unwrap();
    let s = Store::open(d.path().join("db")).unwrap();
    let lease = s.lease("run", 10).unwrap();
    assert!(s.lease("run", 10).is_err());
    drop(lease);
    assert!(s.lease("run", 10).is_ok());
}
#[test]
fn normalization_requires_timestamps_and_is_stably_deduplicated() {
    let mapping = connectors::Mapping {
        synthetic_fixture: false,
        items_pointer: "".into(),
        id_field: Some("/id".into()),
        text_fields: vec!["/text".into()],
        url_field: None,
        timestamp_field: "/at".into(),
        provider: "apify".into(),
        kind: EvidenceKind::Observed,
        segment: None,
        population: None,
    };
    let source = json!([{"id":"1","text":"public comment","at":"2026-01-01T00:00:00Z"}]);
    let first = connectors::normalize(
        &source,
        &mapping,
        "apify:dataset:test",
        date("2026-04-01T00:00:00Z"),
    )
    .unwrap();
    let second = connectors::normalize(
        &source,
        &mapping,
        "apify:dataset:test",
        date("2026-04-02T00:00:00Z"),
    )
    .unwrap();
    assert_eq!(first[0].id, second[0].id);
    let mut synthetic = source.clone();
    synthetic[0]["synthetic_fixture"] = json!(true);
    assert!(
        connectors::normalize(
            &synthetic,
            &mapping,
            "apify:dataset:test",
            date("2026-04-01T00:00:00Z")
        )
        .unwrap()[0]
            .synthetic_fixture
    );
    assert!(
        connectors::normalize(&json!([{"text":"missing date"}]), &mapping, "x", Utc::now())
            .is_err()
    );
}
#[test]
fn credential_bearing_or_insecure_endpoint_urls_are_rejected() {
    for url in [
        "https://user:secret@example.com",
        "https://example.com?token=secret",
        "http://example.com",
    ] {
        assert!(connectors::safe_url(url, false).is_err());
    }
    assert!(connectors::safe_url("http://127.0.0.1:1234/v1/chat/completions", true).is_ok());
}
#[test]
fn evaluation_is_paired_and_checks_prediction_chronology() {
    let c = evaluation::Case {
        id: "one".into(),
        predicted_at: date("2026-01-01T00:00:00Z"),
        outcome_at: date("2026-02-01T00:00:00Z"),
        actual: true,
        baseline_probability: 0.5,
        single_agent_probability: 0.7,
        swarm_probability: 0.8,
        baseline_cost_usd: 0.,
        single_agent_cost_usd: 0.1,
        swarm_cost_usd: 0.5,
    };
    let v = evaluation::compare(std::slice::from_ref(&c)).unwrap();
    assert!((v["scores"][2]["brier"].as_f64().unwrap() - 0.04).abs() < 1e-9);
    let mut bad = c.clone();
    bad.predicted_at = bad.outcome_at;
    assert!(evaluation::compare(&[bad]).is_err());
    assert!(evaluation::compare(&[c.clone(), c]).is_err());
}
#[tokio::test]
async fn all_twelve_modules_run_offline_without_claiming_neural_activity() {
    let d = tempfile::tempdir().unwrap();
    let s = Store::open(d.path().join("db")).unwrap();
    s.import(&[evidence("e1", EvidenceKind::Observed, true)])
        .unwrap();
    let report = runtime::run(&s, request(), None).await.unwrap();
    assert_eq!(report.agents.len(), 12);
    assert_eq!(report.status, "offline_analysis_only");
    assert!(report
        .agents
        .iter()
        .all(|r| r.output.is_none() && r.status == "not_run_offline"));
    let persisted = s.report(&report.run_id).unwrap().unwrap();
    assert_eq!(persisted["run_id"], report.run_id);
}
#[test]
fn reasoning_budget_preserves_room_for_visible_output_without_overflow() {
    let mut limits = Limits {
        max_output_tokens: 4096,
        reasoning_max_tokens: Some(1024),
        ..Limits::default()
    };
    assert!(limits.validate().is_ok());
    for invalid in [0, 1023, 4096, u32::MAX] {
        limits.reasoning_max_tokens = Some(invalid);
        assert!(limits.validate().is_err());
    }
    limits.reasoning_max_tokens = Some(1024);
    limits.reasoning_enabled = Some(false);
    assert!(limits.validate().is_err());
}
#[tokio::test]
async fn rejected_completion_preserves_billed_usage_and_raw_content() {
    let app = axum::Router::new().route(
        "/v1/chat/completions",
        axum::routing::post(
            |axum::Json(body): axum::Json<serde_json::Value>| async move {
                assert_eq!(body["reasoning"]["enabled"], false);
                axum::Json(json!({"id":"truncated-test","model":"test-local",
                "choices":[{"finish_reason":"length","message":{"content":"{\"summary\":"}}],
                "usage":{"total_tokens":256,"cost":0.0}}))
            },
        ),
    );
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let server = tokio::spawn(async move { axum::serve(listener, app).await.unwrap() });
    let d = tempfile::tempdir().unwrap();
    let path = d.path().join("db");
    let store = Store::open(&path).unwrap();
    let model = Inference::local(
        "test-local".into(),
        &format!("http://{address}/v1/chat/completions"),
        10,
    )
    .unwrap();
    let error = model
        .complete(
            "test",
            &json!({}),
            &Limits {
                reasoning_enabled: Some(false),
                ..Limits::default()
            },
            &store,
            "receipt-test",
        )
        .await
        .unwrap_err();
    assert!(error.to_string().contains("partial answer rejected"));
    let conn = rusqlite::Connection::open(path).unwrap();
    let body: String = conn
        .query_row(
            "SELECT body FROM inference_responses WHERE run_id=?",
            ["receipt-test"],
            |row| row.get(0),
        )
        .unwrap();
    let receipt: serde_json::Value = serde_json::from_str(&body).unwrap();
    assert_eq!(receipt["response_id"], "truncated-test");
    assert_eq!(receipt["usage"]["total_tokens"], 256);
    assert_eq!(receipt["content"], "{\"summary\":");
    assert_eq!(receipt["finish_reason"], "length");
    server.abort();
}
#[tokio::test]
async fn local_inference_executes_fourteen_stages_and_replays_without_new_calls() {
    let count = Arc::new(AtomicUsize::new(0));
    let counter = count.clone();
    let app=axum::Router::new().route("/v1/chat/completions",axum::routing::post(move|axum::Json(body):axum::Json<serde_json::Value>|{
        let counter=counter.clone();async move{
            counter.fetch_add(1,Ordering::SeqCst);
            let input:serde_json::Value=serde_json::from_str(body["messages"][1]["content"].as_str().unwrap()).unwrap();
            let id=input["evidence"][0]["id"].as_str().unwrap();
            let output=json!({"summary":"Local HTTP contract test, not real inference","findings":[{"statement":"Test observation","evidence_ids":[id],"interpretation":"observation"}],"hypotheses":[],"gaps":[],"next_experiments":[]});
            axum::Json(json!({"id":"test","model":"test-local","choices":[{"finish_reason":"stop","message":{"content":output.to_string()}}],"usage":{"total_tokens":100}}))
        }
    }));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let server = tokio::spawn(async move { axum::serve(listener, app).await.unwrap() });
    let d = tempfile::tempdir().unwrap();
    let s = Store::open(d.path().join("db")).unwrap();
    s.import(&[evidence("e1", EvidenceKind::Observed, true)])
        .unwrap();
    let model = Inference::local(
        "test-local".into(),
        &format!("http://{address}/v1/chat/completions"),
        10,
    )
    .unwrap();
    let report = runtime::run(&s, request(), Some(model.clone()))
        .await
        .unwrap();
    assert_eq!(report.status, "completed");
    assert_eq!(count.load(Ordering::SeqCst), 14);
    assert_eq!(report.evidence_graph.len(), 14);
    let replay = runtime::run(&s, request(), Some(model)).await.unwrap();
    assert_eq!(replay.run_id, report.run_id);
    assert_eq!(count.load(Ordering::SeqCst), 14);
    server.abort();
}
