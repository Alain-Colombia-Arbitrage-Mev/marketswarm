use crate::schema::Evidence;
use anyhow::{ensure, Context, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::BTreeSet;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Bar {
    pub at: DateTime<Utc>,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
}
pub fn market(bars: &[Bar], as_of: DateTime<Utc>) -> Result<Value> {
    ensure!(
        bars.len() >= 3 && bars.len() <= 100_000,
        "Market analysis requires 3..100000 bars"
    );
    for (i, b) in bars.iter().enumerate() {
        ensure!(
            [b.open, b.high, b.low, b.close, b.volume]
                .iter()
                .all(|n| n.is_finite())
                && b.low > 0.
                && b.volume >= 0.,
            "Invalid OHLCV number"
        );
        ensure!(
            b.low <= b.open.min(b.close) && b.high >= b.open.max(b.close) && b.low <= b.high,
            "Invalid OHLC range"
        );
        ensure!(
            b.at <= as_of && (i == 0 || b.at > bars[i - 1].at),
            "Future, duplicate or unordered market bar"
        );
    }
    let n = bars.len();
    let window = &bars[n.saturating_sub(14)..];
    let mut true_ranges = vec![];
    for i in n.saturating_sub(14)..n {
        let b = &bars[i];
        true_ranges.push(if i == 0 {
            b.high - b.low
        } else {
            (b.high - b.low)
                .max((b.high - bars[i - 1].close).abs())
                .max((b.low - bars[i - 1].close).abs())
        });
    }
    let atr = true_ranges.iter().sum::<f64>() / true_ranges.len() as f64;
    let returns: Vec<_> = bars
        .windows(2)
        .map(|p| (p[1].close / p[0].close).ln())
        .collect();
    let mean = returns.iter().sum::<f64>() / returns.len() as f64;
    let volatility =
        (returns.iter().map(|r| (r - mean).powi(2)).sum::<f64>() / returns.len() as f64).sqrt();
    let mut fvgs = vec![];
    for triple in bars.windows(3) {
        if triple[2].low > triple[0].high {
            fvgs.push(json!({"direction":"bullish","low":triple[0].high,"high":triple[2].low,"confirmed_at":triple[2].at}));
        }
        if triple[2].high < triple[0].low {
            fvgs.push(json!({"direction":"bearish","low":triple[2].high,"high":triple[0].low,"confirmed_at":triple[2].at}));
        }
    }
    let mut swings = vec![];
    for p in bars.windows(5) {
        if p.iter()
            .enumerate()
            .all(|(i, b)| i == 2 || p[2].high > b.high)
        {
            swings.push(
                json!({"kind":"high","price":p[2].high,"pivot_at":p[2].at,"confirmed_at":p[4].at}),
            );
        }
        if p.iter()
            .enumerate()
            .all(|(i, b)| i == 2 || p[2].low < b.low)
        {
            swings.push(
                json!({"kind":"low","price":p[2].low,"pivot_at":p[2].at,"confirmed_at":p[4].at}),
            );
        }
    }
    Ok(
        json!({"bars":n,"last_at":bars[n-1].at,"mean_close_window":window.iter().map(|b|b.close).sum::<f64>()/window.len() as f64,
        "atr_simple_14_or_available":atr,"log_return_volatility_per_bar":volatility,"window_return":bars[n-1].close/window[0].close-1.,
        "three_candle_fvgs":fvgs,"confirmed_swings":swings,
        "limitations":["OHLCV only; not aggressor flow or DOM. Gaps are detected, not labelled unfilled. No executable trade signal.","Input timestamps must represent completed bars; caller must identify timeframe and bar-close convention in context."]}),
    )
}

pub fn sentiment(evidence: &[Evidence]) -> Value {
    let aspects: [(&str, &[&str]); 6] = [
        (
            "price",
            &["precio", "caro", "barato", "price", "expensive", "cheap"],
        ),
        (
            "utility",
            &["util", "útil", "sirve", "funciona", "useful", "works"],
        ),
        ("quality", &["calidad", "quality"]),
        ("support", &["soporte", "support", "ayuda"]),
        ("trust", &["confianza", "trust", "estafa", "scam"]),
        (
            "experience",
            &[
                "experiencia",
                "facil",
                "fácil",
                "dificil",
                "difícil",
                "experience",
            ],
        ),
    ];
    let positive = [
        "bueno",
        "buena",
        "excelente",
        "barato",
        "útil",
        "util",
        "facil",
        "fácil",
        "good",
        "excellent",
        "cheap",
        "useful",
        "works",
        "rápido",
        "rapido",
    ];
    let negative = [
        "malo",
        "mala",
        "caro",
        "lento",
        "estafa",
        "dificil",
        "difícil",
        "bad",
        "expensive",
        "slow",
        "scam",
        "inútil",
        "inutil",
    ];
    let negations = ["no", "not", "nunca", "never"];
    let mut rows = vec![];
    for e in evidence
        .iter()
        .filter(|e| e.kind == crate::schema::EvidenceKind::Observed)
    {
        let text = e
            .content
            .to_lowercase()
            .replace(" pero ", ".")
            .replace(" but ", ".");
        for clause in text.split(['.', ';', '!', '?', '\n']) {
            let words: Vec<_> = clause
                .split(|c: char| !c.is_alphanumeric())
                .filter(|s| !s.is_empty())
                .collect();
            let positions: Vec<Vec<usize>> = aspects
                .iter()
                .map(|(_, terms)| {
                    words
                        .iter()
                        .enumerate()
                        .filter_map(|(i, w)| terms.contains(w).then_some(i))
                        .collect()
                })
                .collect();
            let mut scores = [(0usize, 0usize); 6];
            for (i, w) in words.iter().enumerate() {
                let polarity = if positive.contains(w) {
                    1
                } else if negative.contains(w) {
                    -1
                } else {
                    0
                };
                let reversed = words[i.saturating_sub(3)..i]
                    .iter()
                    .any(|w| negations.contains(w));
                let score = if reversed { -polarity } else { polarity };
                let distances: Vec<_> = positions
                    .iter()
                    .map(|p| p.iter().map(|j| j.abs_diff(i)).min().unwrap_or(usize::MAX))
                    .collect();
                let nearest = distances.iter().copied().min().unwrap_or(usize::MAX);
                for (a, d) in distances.iter().enumerate() {
                    if *d == nearest && nearest <= 4 {
                        if score > 0 {
                            scores[a].0 += 1;
                        } else if score < 0 {
                            scores[a].1 += 1;
                        }
                    }
                }
            }
            for (a, (aspect, _)) in aspects.iter().enumerate() {
                if !positions[a].is_empty() {
                    let label = match (scores[a].0 > 0, scores[a].1 > 0) {
                        (true, true) => "mixed",
                        (true, false) => "positive",
                        (false, true) => "negative",
                        _ => "uncertain",
                    };
                    rows.push(json!({"evidence_id":e.id,"aspect":aspect,"label":label,"excerpt":clause.trim(),"method":"lexicon_baseline"}));
                }
            }
        }
    }
    json!({"results":rows,"limitations":["Uncalibrated Spanish/English lexical baseline; sarcasm, context and cross-clause negation require human/model review. Not a validated sentiment classifier."]})
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Product {
    pub id: String,
    pub evidence_id: String,
    pub currency: String,
    pub billing_period: String,
    pub price_cents: u64,
    pub available: bool,
    pub observed_at: DateTime<Utc>,
    pub features: Vec<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Buyer {
    pub currency: String,
    pub billing_period: String,
    pub budget_cents: u64,
    pub required_features: Vec<String>,
    pub desired_features: Vec<String>,
}
pub fn recommend(
    catalog: &[Product],
    buyer: &Buyer,
    evidence: &[Evidence],
    as_of: DateTime<Utc>,
) -> Result<Value> {
    let ids: BTreeSet<_> = evidence.iter().map(|e| e.id.as_str()).collect();
    let mut options = vec![];
    for p in catalog {
        ensure!(
            ids.contains(p.evidence_id.as_str()),
            "Catalog item is missing evidence"
        );
        ensure!(p.observed_at <= as_of, "Future catalog price");
        if !p.available
            || p.currency != buyer.currency
            || p.billing_period != buyer.billing_period
            || p.price_cents > buyer.budget_cents
        {
            continue;
        }
        if !buyer
            .required_features
            .iter()
            .all(|f| p.features.contains(f))
        {
            continue;
        }
        let score = buyer
            .desired_features
            .iter()
            .filter(|f| p.features.contains(f))
            .count();
        options.push((score, p));
    }
    options.sort_by(|a, b| b.0.cmp(&a.0).then(a.1.price_cents.cmp(&b.1.price_cents)));
    Ok(
        json!({"options":options.iter().map(|(score,p)|json!({"product":p,"matched_desired_features":score})).collect::<Vec<_>>(),
        "no_purchase_available":true,"limitations":["Matching uses supplied dated catalog attributes and declared preferences; current availability still requires confirmation."]}),
    )
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Persona {
    pub id: String,
    pub budget_cents: u64,
    pub interest: f64,
    pub influence: f64,
    pub evidence_ids: Vec<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SocialScenario {
    pub seed: u64,
    pub price_cents: u64,
    pub rounds: usize,
    pub personas: Vec<Persona>,
    pub edges: Vec<[usize; 2]>,
}
pub fn social(s: &SocialScenario, evidence: &[Evidence]) -> Result<Value> {
    ensure!(
        s.seed > 0
            && s.price_cents > 0
            && (1..=30).contains(&s.rounds)
            && !s.personas.is_empty()
            && s.personas.len() <= 1000
            && s.edges.len() <= 10000,
        "Invalid social scenario size/seed"
    );
    let ids: BTreeSet<_> = evidence.iter().map(|e| e.id.as_str()).collect();
    let mut seen = BTreeSet::new();
    for p in &s.personas {
        ensure!(
            seen.insert(&p.id) && !p.id.trim().is_empty(),
            "Duplicate/empty persona id"
        );
        ensure!(
            (0.0..=1.0).contains(&p.interest) && (0.0..=1.0).contains(&p.influence),
            "Invalid persona probability"
        );
        ensure!(
            !p.evidence_ids.is_empty() && p.evidence_ids.iter().all(|id| ids.contains(id.as_str())),
            "Persona requires source evidence"
        );
    }
    let mut peers = vec![vec![]; s.personas.len()];
    for [a, b] in &s.edges {
        ensure!(
            *a < s.personas.len() && *b < s.personas.len() && a != b,
            "Invalid social edge"
        );
        peers[*b].push(*a);
    }
    let mut rng = s.seed;
    let mut state = vec![false; s.personas.len()];
    let mut history = vec![];
    for round in 0..s.rounds {
        let previous = state.clone();
        for (i, p) in s.personas.iter().enumerate() {
            let peer = if peers[i].is_empty() {
                0.
            } else {
                peers[i].iter().filter(|j| previous[**j]).count() as f64 / peers[i].len() as f64
            };
            let probability = (p.interest + p.influence * peer * (1. - p.interest)).clamp(0., 1.);
            rng ^= rng << 13;
            rng ^= rng >> 7;
            rng ^= rng << 17;
            state[i] = p.budget_cents >= s.price_cents
                && (rng >> 11) as f64 / ((1u64 << 53) as f64) < probability;
        }
        history
            .push(json!({"round":round,"synthetic_adopters":state.iter().filter(|v|**v).count()}));
    }
    Ok(
        json!({"kind":"simulated","history":history,"agents":s.personas.len(),"seed":s.seed,"human_wtp_validated":false,
        "limitations":["Illustrative synchronous network model with assumed probabilities. Adoption is synthetic, not sales or independent human evidence. No empirical diffusion calibration."]}),
    )
}

pub fn numerical(context: &Value, evidence: &[Evidence], as_of: DateTime<Utc>) -> Result<Value> {
    let mut out = json!({"sentiment_baseline":sentiment(evidence)});
    if let Some(bars) = context.get("market_bars") {
        ensure!(
            context
                .get("market_evidence_ids")
                .and_then(Value::as_array)
                .is_some_and(|ids| !ids.is_empty()
                    && ids.iter().all(|id| id
                        .as_str()
                        .is_some_and(|id| evidence.iter().any(|e| e.id == id)))),
            "Market bars require evidence IDs"
        );
        out["markets"] = market(&serde_json::from_value::<Vec<Bar>>(bars.clone())?, as_of)?;
    }
    if let Some(catalog) = context.get("catalog") {
        let buyer: Buyer = serde_json::from_value(
            context
                .get("buyer")
                .context("Catalog requires buyer preferences")?
                .clone(),
        )?;
        out["recommendations"] = recommend(
            &serde_json::from_value::<Vec<Product>>(catalog.clone())?,
            &buyer,
            evidence,
            as_of,
        )?;
    }
    if let Some(s) = context.get("social_scenario") {
        out["social_simulation"] = social(&serde_json::from_value(s.clone())?, evidence)?;
    }
    Ok(out)
}
