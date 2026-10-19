use crate::{
    analysis,
    inference::Inference,
    schema::{AgentResult, Evidence, ResearchRequest},
    store::Store,
    wtp,
};
use anyhow::{ensure, Result};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::{collections::BTreeSet, sync::Arc};
use tokio::{sync::Semaphore, task::JoinSet};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Report {
    pub run_id: String,
    #[serde(default)]
    pub engine_revision: String,
    pub status: String,
    pub inference_mode: String,
    pub model: Option<String>,
    pub request: ResearchRequest,
    pub sources: Vec<Value>,
    pub excluded_future_evidence: Vec<String>,
    pub numerical: Value,
    pub agents: Vec<AgentResult>,
    pub critic: AgentResult,
    pub synthesis: AgentResult,
    pub reserved_cost_usd: f64,
    pub evidence_graph: Vec<Value>,
    pub limitations: Vec<String>,
}

pub fn excerpt(content: &str, query: &str) -> String {
    if content.chars().count() <= 2200 {
        return content.into();
    }
    let words: BTreeSet<_> = query
        .split(|c: char| !c.is_alphanumeric())
        .filter(|s| s.len() > 3)
        .map(str::to_lowercase)
        .collect();
    let chars: Vec<_> = content.chars().collect();
    let mut chunks: Vec<_> = chars
        .chunks(1000)
        .enumerate()
        .map(|(i, c)| {
            let text: String = c.iter().collect();
            let score = text
                .to_lowercase()
                .split(|c: char| !c.is_alphanumeric())
                .filter(|w| words.contains(*w))
                .count();
            (score, i, text)
        })
        .collect();
    chunks.sort_by(|a, b| b.0.cmp(&a.0).then(a.1.cmp(&b.1)));
    let mut selected: Vec<_> = chunks.into_iter().take(2).collect();
    selected.sort_by_key(|c| c.1);
    format!(
        "{}\n[Selected source excerpts]\n{}",
        chars.iter().take(120).collect::<String>(),
        selected
            .iter()
            .map(|c| format!("[char {}] {}", c.1 * 1000, c.2))
            .collect::<Vec<_>>()
            .join("\n")
    )
}

fn payload(
    request: &ResearchRequest,
    evidence: &[Evidence],
    numerical: &Value,
    peers: Option<Value>,
) -> Value {
    json!({"question":request.question,"as_of":request.as_of,"context":request.context,
        "evidence":evidence.iter().map(|e|json!({"id":e.id,"source":e.source,"provider":e.provider,"kind":e.kind,
            "observed_at":e.observed_at,"published_at":e.published_at,"segment":e.segment,"population":e.population,
            "excerpt":excerpt(&e.content,&request.question),"excerpt_truncated":e.content.chars().count()>2200,
            "limitations":e.limitations,"synthetic_fixture":e.synthetic_fixture})).collect::<Vec<_>>(),
        "numerical":numerical,"peer_outputs":peers})
}

async fn task(
    store: Store,
    run: String,
    role: (String, String),
    payload: Value,
    inference: Option<Inference>,
    request: ResearchRequest,
    ids: BTreeSet<String>,
) -> Result<AgentResult> {
    let (role, brief) = role;
    let system = Inference::system(&role, &brief);
    let prompt_hash = crate::digest(serde_json::to_string(&(system.clone(), &payload))?.as_bytes());
    if let Some(cached) = store.task(&run, &role)? {
        if cached.status == "completed" && cached.prompt_hash == prompt_hash {
            return Ok(cached);
        }
    }
    store.event(&run, &json!({"event":"task_started","role":role}))?;
    let model = inference
        .as_ref()
        .map(|i| i.model.clone())
        .unwrap_or_else(|| "none".into());
    let mut result = AgentResult {
        role: role.clone(),
        model,
        status: "not_run_offline".into(),
        output: None,
        error: None,
        usage: json!({}),
        prompt_hash,
    };
    if let Some(inference) = inference {
        let completion = tokio::time::timeout(
            std::time::Duration::from_secs(request.limits.timeout_seconds),
            inference.complete(&system, &payload, &request.limits, &store, &run),
        )
        .await;
        match completion {
            Ok(Ok((output, usage))) => match output.validate(&ids) {
                Ok(()) => {
                    result.status = "completed".into();
                    result.output = Some(output);
                    result.usage = usage;
                }
                Err(_) => {
                    result.status = "failed".into();
                    result.error = Some("Agent output failed evidence/schema validation".into());
                    result.usage = usage;
                }
            },
            Ok(Err(error)) => {
                result.status = "failed".into();
                result.error = Some(safe_failure(&error));
            }
            Err(_) => {
                result.status = "failed".into();
                result.error = Some("Task deadline exceeded; any reservation is retained".into());
            }
        }
    }
    store.save_task(&run, &result)?;
    store.event(
        &run,
        &json!({"event":"task_finished","role":role,"status":result.status}),
    )?;
    Ok(result)
}
fn safe_failure(error: &anyhow::Error) -> String {
    let s = error.to_string();
    if s.contains("budget") {
        "Budget reservation exhausted; no additional call dispatched".into()
    } else if s.contains("context") {
        "Context allowance exceeded; reduce evidence selection".into()
    } else if s.contains("HTTP") {
        "Provider HTTP failure; inspect service status and configuration".into()
    } else {
        "Inference failed, returned invalid/truncated output, or exceeded cost policy; reservation retained".into()
    }
}

pub async fn run(
    store: &Store,
    request: ResearchRequest,
    inference: Option<Inference>,
) -> Result<Report> {
    request.validate()?;
    let all = store.evidence(&request.evidence_ids)?;
    let evidence: Vec<_> = all
        .iter()
        .filter(|e| e.eligible(request.as_of))
        .cloned()
        .collect();
    let excluded: Vec<_> = all
        .iter()
        .filter(|e| !e.eligible(request.as_of))
        .map(|e| e.id.clone())
        .collect();
    ensure!(!evidence.is_empty(), "No evidence available before cutoff");
    let mut numerical = analysis::numerical(&request.context, &evidence, request.as_of)?;
    if let Some(study) = &request.study {
        numerical["wtp"] = serde_json::to_value(wtp::evaluate(study, &evidence, request.as_of)?)?;
    }
    if let Some(economy) = &request.economy {
        let ids: BTreeSet<_> = evidence.iter().map(|e| e.id.as_str()).collect();
        for s in &economy.scenarios {
            ensure!(
                s.assumption_evidence_ids
                    .iter()
                    .all(|id| ids.contains(id.as_str())),
                "Economic assumption cites unavailable evidence"
            );
        }
        numerical["economics"] = serde_json::to_value(wtp::simulate(economy)?)?;
    }
    let mode = inference
        .as_ref()
        .map(|i| i.mode.as_str())
        .unwrap_or("offline");
    let engine_revision = crate::digest(
        concat!(
            include_str!("runtime.rs"),
            include_str!("schema.rs"),
            include_str!("wtp.rs"),
            include_str!("analysis.rs"),
            include_str!("inference.rs"),
            include_str!("connectors.rs"),
            include_str!("store.rs"),
            include_str!("../Cargo.lock")
        )
        .as_bytes(),
    );
    let fingerprint = json!({"version":env!("CARGO_PKG_VERSION"),"engine_revision":engine_revision,"request":request,"mode":mode,
        "inference":inference.as_ref().map(Inference::identity),"evidence":all.iter().map(|e|json!({"id":e.id,"hash":e.content_hash()})).collect::<Vec<_>>()});
    let run_id = crate::digest(serde_json::to_string(&fingerprint)?.as_bytes());
    store.start(&run_id, &serde_json::to_string(&request)?)?;
    let _lease = store.lease(&run_id, request.limits.timeout_seconds * 16 + 120)?;
    if let Some(previous) = store.report(&run_id)? {
        if previous["status"] == "completed" || previous["status"] == "offline_analysis_only" {
            return Ok(serde_json::from_value(previous)?);
        }
    }
    store.event(
        &run_id,
        &json!({"event":"run_started","mode":mode,"modules":request.modules}),
    )?;
    let ids: BTreeSet<String> = evidence.iter().map(|e| e.id.clone()).collect();
    let shared = payload(&request, &evidence, &numerical, None);
    let semaphore = Arc::new(Semaphore::new(request.limits.concurrency));
    let mut set = JoinSet::new();
    for module in &request.modules {
        let (sem, store, run, role, brief, payload, inference, request, ids) = (
            semaphore.clone(),
            store.clone(),
            run_id.clone(),
            module.key().to_owned(),
            module.brief().to_owned(),
            shared.clone(),
            inference.clone(),
            request.clone(),
            ids.clone(),
        );
        set.spawn(async move {
            let _permit = sem.acquire_owned().await?;
            task(store, run, (role, brief), payload, inference, request, ids).await
        });
    }
    let mut agents = vec![];
    while let Some(result) = set.join_next().await {
        agents.push(result??);
    }
    agents.sort_by(|a, b| a.role.cmp(&b.role));
    let critic=task(store.clone(),run_id.clone(),("critic".into(),
        "Find contradictions, correlated evidence, temporal leakage, unsupported prices and inadequate WTP validation. Keep numerical engine outputs authoritative. State uncertainty; do not vote by majority.".into()),
        payload(&request,&evidence,&numerical,Some(serde_json::to_value(&agents)?)),inference.clone(),request.clone(),ids.clone()).await?;
    let synthesis=task(store.clone(),run_id.clone(),("synthesis".into(),
        "Produce an actionable final recommendation with supporting and contradicting evidence, scope, invalidation and next experiments. Report peer failures and gaps. Never promote synthetic evidence or override deterministic WTP status.".into()),
        payload(&request,&evidence,&numerical,Some(json!({"analysts":agents,"critic":critic}))),inference.clone(),request.clone(),ids).await?;
    let status = if inference.is_none() {
        "offline_analysis_only"
    } else if agents
        .iter()
        .chain([&critic, &synthesis])
        .all(|r| r.status == "completed")
    {
        "completed"
    } else {
        "partial"
    };
    let mut graph = vec![];
    for agent in agents.iter().chain([&critic, &synthesis]) {
        if let Some(out) = &agent.output {
            for (i, f) in out.findings.iter().enumerate() {
                for id in &f.evidence_ids {
                    graph.push(json!({"from":id,"to":format!("{}:finding:{i}",agent.role),"relation":"cited_by","interpretation":f.interpretation}));
                }
            }
        }
    }
    let report=Report {run_id:run_id.clone(),engine_revision,status:status.into(),inference_mode:mode.into(),model:inference.as_ref().map(|i|i.model.clone()),request,
        sources:evidence.iter().map(|e|json!({"id":e.id,"source":e.source,"provider":e.provider,"kind":e.kind,"hash":e.content_hash(),"collected_at":e.collected_at,"observed_at":e.observed_at,"published_at":e.published_at,"synthetic_fixture":e.synthetic_fixture,"limitations":e.limitations})).collect(),
        excluded_future_evidence:excluded,numerical,agents,critic,synthesis,reserved_cost_usd:store.reserved(&run_id)?,evidence_graph:graph,
        limitations:vec!["Source ID validation proves reference availability, not semantic truth; human review and labelled held-out evaluation remain necessary.".into(),
        "Cost reservations are conservative text-token estimates and remain spent on unknown outcomes. Provider billing is authoritative; configure a provider account/key spending cap as a second limit.".into(),
        "Offline reports contain deterministic tools only; no neural agents run in offline mode. Integration and predictive validity require source-specific live checks.".into()]};
    store.finish(&run_id, status, &serde_json::to_value(&report)?)?;
    store.event(&run_id, &json!({"event":"run_finished","status":status}))?;
    Ok(report)
}

pub fn render(report: &Report) -> String {
    let mut s = format!(
        "Investigacion {}\nEstado: {} | Inferencia: {}\nPregunta: {}\nFecha de corte: {}\n\n",
        report.run_id,
        report.status,
        report.inference_mode,
        report.request.question,
        report.request.as_of
    );
    for r in report
        .agents
        .iter()
        .chain([&report.critic, &report.synthesis])
    {
        s.push_str(&format!("{}: {}\n", r.role, r.status));
        if let Some(o) = &r.output {
            s.push_str(&format!("{}\n", o.summary));
            for f in &o.findings {
                s.push_str(&format!(
                    "- {} [{}; {}]\n",
                    f.statement,
                    f.interpretation,
                    f.evidence_ids.join(", ")
                ));
            }
            for gap in &o.gaps {
                s.push_str(&format!("Pendiente: {gap}\n"));
            }
        }
        if let Some(error) = &r.error {
            s.push_str(&format!("{error}\n"));
        }
        s.push('\n');
    }
    s.push_str("CALCULOS Y ESTUDIOS\n");
    s.push_str(&serde_json::to_string_pretty(&report.numerical).unwrap_or_default());
    s.push_str("\n\nFUENTES\n");
    for e in &report.sources {
        s.push_str(&format!("{}: {} ({})\n", e["id"], e["source"], e["kind"]));
    }
    s.push_str("\nLIMITACIONES\n");
    for l in &report.limitations {
        s.push_str(&format!("- {l}\n"));
    }
    s
}
