use anyhow::{ensure, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::BTreeSet;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Case {
    pub id: String,
    pub predicted_at: DateTime<Utc>,
    pub outcome_at: DateTime<Utc>,
    pub actual: bool,
    pub baseline_probability: f64,
    pub single_agent_probability: f64,
    pub swarm_probability: f64,
    pub baseline_cost_usd: f64,
    pub single_agent_cost_usd: f64,
    pub swarm_cost_usd: f64,
}
pub fn compare(cases: &[Case]) -> Result<Value> {
    ensure!(
        !cases.is_empty() && cases.len() <= 100_000,
        "Provide 1..100000 paired held-out cases"
    );
    let mut ids = BTreeSet::new();
    for c in cases {
        ensure!(
            ids.insert(&c.id) && !c.id.is_empty() && c.predicted_at < c.outcome_at,
            "Duplicate case or prediction not before outcome"
        );
        for p in [
            c.baseline_probability,
            c.single_agent_probability,
            c.swarm_probability,
        ] {
            ensure!((0.0..=1.0).contains(&p), "Invalid probability");
        }
        for cost in [
            c.baseline_cost_usd,
            c.single_agent_cost_usd,
            c.swarm_cost_usd,
        ] {
            ensure!(cost.is_finite() && cost >= 0., "Invalid evaluation cost");
        }
    }
    let n = cases.len() as f64;
    let mut scores = vec![];
    for mode in 0..3 {
        let mut brier = 0.;
        let mut loss = 0.;
        let mut correct = 0;
        let mut cost = 0.;
        let mut bins = [(0usize, 0., 0usize); 5];
        for c in cases {
            let (p, k) = match mode {
                0 => (c.baseline_probability, c.baseline_cost_usd),
                1 => (c.single_agent_probability, c.single_agent_cost_usd),
                _ => (c.swarm_probability, c.swarm_cost_usd),
            };
            let y = if c.actual { 1. } else { 0. };
            brier += (p - y).powi(2);
            let clipped = p.clamp(1e-12, 1. - 1e-12);
            loss -= y * clipped.ln() + (1. - y) * (1. - clipped).ln();
            if (p >= 0.5) == c.actual {
                correct += 1;
            }
            cost += k;
            let b = ((p * 5.).floor() as usize).min(4);
            bins[b].0 += 1;
            bins[b].1 += p;
            if c.actual {
                bins[b].2 += 1;
            }
        }
        let method = ["simple_baseline", "single_agent", "swarm"][mode];
        scores.push(json!({"method":method,"n":cases.len(),"brier":brier/n,"log_loss":loss/n,"accuracy":correct as f64/n,"total_cost_usd":cost,
            "calibration_bins":bins.iter().enumerate().filter(|(_,b)|b.0>0).map(|(i,b)|json!({"bin":i,"n":b.0,"mean_prediction":b.1/b.0 as f64,"actual_rate":b.2 as f64/b.0 as f64})).collect::<Vec<_>>()}));
    }
    Ok(
        json!({"scores":scores,"conclusion":"descriptive_only","limitations":["Paired cases must come from the same frozen held-out evaluator/population and comparable budgets. Chronology checks do not prove absence of training or selection leakage.","No statistical superiority or production readiness inferred. Cluster dependent/overlapping cases before uncertainty estimation."]}),
    )
}
