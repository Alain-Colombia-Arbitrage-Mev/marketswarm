use crate::schema::{Evidence, EvidenceKind};
use anyhow::{ensure, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Outcome {
    Yes,
    No,
    Paid,
    NoPurchase,
    Refunded,
    TrialStarted,
    Deposit,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Response {
    pub participant_id: String,
    pub price_cents: u64,
    pub outcome: Outcome,
    pub evidence_id: String,
    pub recorded_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Protocol {
    pub method: String,
    pub registered_at: DateTime<Utc>,
    pub closes_at: DateTime<Utc>,
    pub minimum_per_price: usize,
    pub maximum_half_width: f64,
    pub population: String,
    pub allocation: String,
    pub maturity_days: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Study {
    pub offer: String,
    pub segment: String,
    pub currency: String,
    pub billing_period: String,
    pub kind: EvidenceKind,
    pub protocol: Protocol,
    pub responses: Vec<Response>,
}
impl Study {
    pub fn validate(&self) -> Result<()> {
        ensure!(
            !self.offer.trim().is_empty() && !self.segment.trim().is_empty(),
            "Study requires offer and segment"
        );
        ensure!(
            self.currency.len() == 3 && self.currency.bytes().all(|b| b.is_ascii_uppercase()),
            "Use uppercase ISO currency"
        );
        ensure!(
            ["weekly", "monthly", "quarterly", "annual", "one_time"]
                .contains(&self.billing_period.as_str()),
            "Invalid billing period"
        );
        ensure!(
            self.kind != EvidenceKind::Benchmark,
            "Benchmarks cannot be respondent observations"
        );
        ensure!(self.responses.len() <= 100_000, "Study too large");
        ensure!(
            self.protocol.minimum_per_price >= 20,
            "Minimum per price must be at least 20; plan power for your effect"
        );
        ensure!(
            self.protocol.maximum_half_width > 0. && self.protocol.maximum_half_width <= 0.2,
            "Invalid precision threshold"
        );
        ensure!(
            self.protocol.registered_at <= self.protocol.closes_at
                && self.protocol.maturity_days <= 365,
            "Invalid protocol dates/maturity"
        );
        ensure!(
            !self.protocol.population.trim().is_empty()
                && !self.protocol.allocation.trim().is_empty(),
            "Protocol requires population and allocation"
        );
        ensure!(
            matches!(
                self.protocol.method.as_str(),
                "gabor_granger" | "discrete_choice" | "randomized_offer" | "synthetic_scenario"
            ),
            "Unsupported study method"
        );
        if self.kind == EvidenceKind::Observed {
            ensure!(
                self.protocol.method == "randomized_offer"
                    && self.protocol.allocation == "randomized",
                "Observed comparison requires a randomized offer protocol"
            );
        }
        let mut pairs = BTreeSet::new();
        let mut participants = BTreeSet::new();
        for r in &self.responses {
            ensure!(
                !r.participant_id.trim().is_empty()
                    && !r.evidence_id.trim().is_empty()
                    && r.price_cents > 0,
                "Invalid response"
            );
            ensure!(
                pairs.insert((r.participant_id.clone(), r.price_cents)),
                "Duplicate respondent at a price"
            );
            if self.kind == EvidenceKind::Observed {
                ensure!(
                    participants.insert(&r.participant_id),
                    "Commercial participant assigned more than once"
                );
            }
            ensure!(
                r.recorded_at >= self.protocol.registered_at,
                "Protocol was registered after observations"
            );
            match self.kind {
                EvidenceKind::Stated | EvidenceKind::Simulated => ensure!(
                    matches!(r.outcome, Outcome::Yes | Outcome::No),
                    "Stated/simulated observations must be yes/no"
                ),
                EvidenceKind::Observed => ensure!(
                    !matches!(r.outcome, Outcome::Yes | Outcome::No),
                    "A declaration is not a payment observation"
                ),
                _ => unreachable!(),
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurvePoint {
    pub price_cents: u64,
    pub participants: usize,
    pub accepted: usize,
    pub unresolved_or_partial: usize,
    pub acceptance: f64,
    pub wilson_95: [f64; 2],
    pub gross_per_exposed_cents: f64,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StudyReport {
    pub evidence_kind: EvidenceKind,
    pub conclusion: String,
    pub scope: String,
    pub curve: Vec<CurvePoint>,
    pub limitations: Vec<String>,
}

pub fn wilson(successes: usize, n: usize) -> [f64; 2] {
    if n == 0 {
        return [0., 1.];
    }
    let n = n as f64;
    let p = successes as f64 / n;
    let z2 = 1.959963984540054_f64.powi(2);
    let center = (p + z2 / (2. * n)) / (1. + z2 / n);
    let half = (z2 * (p * (1. - p) / n + z2 / (4. * n * n))).sqrt() / (1. + z2 / n);
    [(center - half).max(0.), (center + half).min(1.)]
}

pub fn evaluate(study: &Study, evidence: &[Evidence], as_of: DateTime<Utc>) -> Result<StudyReport> {
    study.validate()?;
    let by_id: BTreeMap<_, _> = evidence.iter().map(|e| (e.id.as_str(), e)).collect();
    let mut synthetic = false;
    let mut points: BTreeMap<u64, (usize, usize, usize)> = BTreeMap::new();
    for r in &study.responses {
        ensure!(
            r.recorded_at <= as_of && r.recorded_at <= study.protocol.closes_at,
            "Response outside study cutoff"
        );
        let e = by_id
            .get(r.evidence_id.as_str())
            .ok_or_else(|| anyhow::anyhow!("Study response is missing source evidence"))?;
        ensure!(
            e.kind == study.kind && e.eligible(as_of),
            "Study/evidence provenance mismatch or future evidence"
        );
        ensure!(
            e.segment.as_deref() == Some(study.segment.as_str()),
            "Response evidence does not match the declared study segment"
        );
        ensure!(
            e.observed_at >= r.recorded_at,
            "Evidence predates claimed response"
        );
        synthetic |= e.synthetic_fixture;
        let (n, yes, pending) = points.entry(r.price_cents).or_default();
        *n += 1;
        let immature = study.kind == EvidenceKind::Observed
            && (as_of - r.recorded_at).num_days() < study.protocol.maturity_days as i64;
        if immature || matches!(r.outcome, Outcome::TrialStarted | Outcome::Deposit) {
            *pending += 1;
        } else if matches!(r.outcome, Outcome::Yes | Outcome::Paid) {
            *yes += 1;
        }
    }
    let curve: Vec<_> = points
        .into_iter()
        .map(|(price, (n, yes, pending))| {
            let rate = yes as f64 / n as f64;
            CurvePoint {
                price_cents: price,
                participants: n,
                accepted: yes,
                unresolved_or_partial: pending,
                acceptance: rate,
                wilson_95: wilson(yes, n),
                gross_per_exposed_cents: rate * price as f64,
            }
        })
        .collect();
    let precise = curve.len() >= 2
        && curve.iter().all(|p| {
            p.participants >= study.protocol.minimum_per_price
                && (p.wilson_95[1] - p.wilson_95[0]) / 2. <= study.protocol.maximum_half_width
                && p.unresolved_or_partial == 0
        });
    let concluded = precise
        && !synthetic
        && study.kind != EvidenceKind::Simulated
        && as_of >= study.protocol.closes_at;
    let conclusion = if concluded {
        "supported_in_scope"
    } else {
        "inconclusive"
    };
    let mut limitations = vec![
        "Pointwise 95% intervals; not simultaneous confidence bands or a winner test.".into(),
        "Results apply only to the specified offer, sampled population, tested prices, currency and billing period.".into(),
        "Protocol registration and provenance are supplied data; verify them against your collection system.".into(),
    ];
    if study.kind == EvidenceKind::Stated {
        limitations.push("Declared acceptance is not revealed willingness to pay or an actual sale. Discrete-choice output is descriptive acceptance, not a fitted conjoint utility model.".into());
    }
    if study.kind == EvidenceKind::Simulated || synthetic {
        limitations.push("Synthetic input: cannot validate human willingness to pay; intervals describe artificial responses only.".into());
    }
    if !concluded {
        limitations.push("Insufficient precision, maturity, price coverage, real observations, or study closure for a scoped conclusion.".into());
    }
    Ok(StudyReport {
        evidence_kind: study.kind,
        conclusion: conclusion.into(),
        scope: format!(
            "{} / {} / {} / {}",
            study.offer, study.segment, study.currency, study.billing_period
        ),
        curve,
        limitations,
    })
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Economy {
    pub currency: String,
    pub seed: u64,
    pub repetitions: usize,
    pub scenarios: Vec<Scenario>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Scenario {
    pub name: String,
    pub visitors: usize,
    pub price_cents: u64,
    pub period_months: u32,
    pub cycles: u32,
    pub conversion: f64,
    pub renewal: f64,
    pub refund_rate: f64,
    pub billing_failure: f64,
    pub fee_rate: f64,
    pub tax_inclusive_rate: f64,
    pub service_cost_per_month_cents: u64,
    pub acquisition_cost_per_visitor_cents: u64,
    pub assumption_evidence_ids: Vec<String>,
}
impl Economy {
    pub fn validate(&self) -> Result<()> {
        ensure!(
            self.currency.len() == 3 && self.currency.bytes().all(|b| b.is_ascii_uppercase()),
            "Invalid economic currency"
        );
        ensure!(
            self.seed != 0 && (10..=2000).contains(&self.repetitions),
            "Invalid Monte Carlo seed/repetitions"
        );
        ensure!(
            !self.scenarios.is_empty() && self.scenarios.len() <= 12,
            "Select 1..12 economic scenarios"
        );
        let mut work = 0u128;
        for s in &self.scenarios {
            ensure!(
                !s.name.trim().is_empty()
                    && s.visitors > 0
                    && s.visitors <= 100_000
                    && s.price_cents > 0,
                "Invalid scenario"
            );
            ensure!(
                (1..=12).contains(&s.period_months) && (1..=24).contains(&s.cycles),
                "Invalid billing horizon"
            );
            for p in [
                s.conversion,
                s.renewal,
                s.refund_rate,
                s.billing_failure,
                s.fee_rate,
                s.tax_inclusive_rate,
            ] {
                ensure!(
                    p.is_finite() && (0.0..=1.0).contains(&p),
                    "Invalid scenario probability/rate"
                );
            }
            work += s.visitors as u128 * s.cycles as u128 * self.repetitions as u128;
        }
        ensure!(work <= 20_000_000, "Monte Carlo work limit exceeded");
        Ok(())
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EconomicsReport {
    pub name: String,
    pub evidence_kind: EvidenceKind,
    pub horizon_months: u32,
    pub expected_initial_buyers: f64,
    pub expected_gross_cents: f64,
    pub expected_net_cents: f64,
    pub expected_contribution_cents: f64,
    pub contribution_quantiles_cents: [f64; 3],
    pub assumptions: Scenario,
    pub limitations: Vec<String>,
}
struct Rng(u64);
impl Rng {
    fn yes(&mut self, p: f64) -> bool {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        (self.0 >> 11) as f64 / ((1u64 << 53) as f64) < p
    }
}
pub fn simulate(e: &Economy) -> Result<Vec<EconomicsReport>> {
    e.validate()?;
    let mut rng = Rng(e.seed);
    let mut reports = vec![];
    for s in &e.scenarios {
        let mut active = s.visitors as f64 * s.conversion;
        let (mut gross, mut net, mut service) = (0., 0., 0.);
        for cycle in 0..s.cycles {
            if cycle > 0 {
                active *= s.renewal;
            }
            active *= 1. - s.billing_failure;
            let billed = active * s.price_cents as f64;
            let retained = billed * (1. - s.refund_rate);
            gross += billed;
            // Price includes indirect tax. Fees are conservatively charged on gross;
            // fee refunds and recovering failed payments need separate future models.
            net += retained / (1. + s.tax_inclusive_rate) - billed * s.fee_rate;
            service += active * s.period_months as f64 * s.service_cost_per_month_cents as f64;
            active *= 1. - s.refund_rate;
        }
        let acquisition = s.visitors as f64 * s.acquisition_cost_per_visitor_cents as f64;
        let mut samples = Vec::with_capacity(e.repetitions);
        for _ in 0..e.repetitions {
            let mut contribution = -acquisition;
            for _ in 0..s.visitors {
                if !rng.yes(s.conversion) {
                    continue;
                }
                for cycle in 0..s.cycles {
                    if cycle > 0 && !rng.yes(s.renewal) {
                        break;
                    }
                    if rng.yes(s.billing_failure) {
                        break;
                    }
                    let price = s.price_cents as f64;
                    contribution -= price * s.fee_rate
                        + s.period_months as f64 * s.service_cost_per_month_cents as f64;
                    if rng.yes(s.refund_rate) {
                        break;
                    }
                    contribution += price / (1. + s.tax_inclusive_rate);
                }
            }
            samples.push(contribution);
        }
        samples.sort_by(f64::total_cmp);
        let q = |p: f64| samples[((samples.len() - 1) as f64 * p).round() as usize];
        reports.push(EconomicsReport { name: s.name.clone(), evidence_kind: EvidenceKind::Simulated,
            horizon_months: s.period_months * s.cycles,
            expected_initial_buyers: s.visitors as f64 * s.conversion * (1. - s.billing_failure),
            expected_gross_cents: gross, expected_net_cents: net,
            expected_contribution_cents: net - service - acquisition,
            contribution_quantiles_cents: [q(0.05), q(0.5), q(0.95)], assumptions: s.clone(),
            limitations: vec!["Conditional Monte Carlo prediction intervals, not confidence in assumptions.".into(),
                "Constant independent rates; failed billing ends the subscription; no acquisition of new cohorts after time zero.".into(),
                "Fees are on gross and not refunded; service is charged for the full period including refunded purchases. Contribution is not accounting profit or revenue recognition.".into()] });
    }
    Ok(reports)
}
