use anyhow::{bail, ensure, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceKind {
    Benchmark,
    Simulated,
    Stated,
    Observed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Evidence {
    pub id: String,
    pub source: String,
    pub provider: String,
    pub kind: EvidenceKind,
    pub published_at: Option<DateTime<Utc>>,
    pub collected_at: DateTime<Utc>,
    pub observed_at: DateTime<Utc>,
    pub segment: Option<String>,
    pub population: Option<String>,
    pub content: String,
    pub limitations: Vec<String>,
    pub synthetic_fixture: bool,
}
impl Evidence {
    pub fn validate(&self) -> Result<()> {
        ensure!(
            !self.id.trim().is_empty() && !self.source.trim().is_empty(),
            "Evidence requires id and source"
        );
        ensure!(
            !self.provider.trim().is_empty() && !self.content.trim().is_empty(),
            "Evidence requires provider and content"
        );
        if self.source.starts_with("http") {
            let url = reqwest::Url::parse(&self.source)?;
            ensure!(
                url.username().is_empty() && url.password().is_none(),
                "Evidence source cannot contain credentials"
            );
            ensure!(
                !url.query_pairs().any(|(k, _)| matches!(
                    k.to_lowercase().as_str(),
                    "token" | "key" | "api_key" | "apikey" | "authorization" | "access_token"
                )),
                "Evidence source contains a credential query parameter"
            );
        }
        ensure!(
            self.content.len() <= 100_000,
            "Evidence too large; split into attributed excerpts"
        );
        ensure!(
            self.observed_at <= self.collected_at,
            "Observed date is after collection"
        );
        if let Some(p) = self.published_at {
            ensure!(p <= self.collected_at, "Publication is after collection");
        }
        Ok(())
    }
    pub fn eligible(&self, as_of: DateTime<Utc>) -> bool {
        self.observed_at <= as_of && self.published_at.is_none_or(|p| p <= as_of)
    }
    pub fn content_hash(&self) -> String {
        crate::digest(self.content.as_bytes())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Module {
    FinancialMarkets,
    Competition,
    SocialMedia,
    Sentiment,
    ProductDiscovery,
    Recommendations,
    ContentReputation,
    Pricing,
    WillingnessToPay,
    SocialSimulation,
    Economics,
    Experimentation,
}
impl Module {
    pub const ALL: [Self; 12] = [
        Self::FinancialMarkets,
        Self::Competition,
        Self::SocialMedia,
        Self::Sentiment,
        Self::ProductDiscovery,
        Self::Recommendations,
        Self::ContentReputation,
        Self::Pricing,
        Self::WillingnessToPay,
        Self::SocialSimulation,
        Self::Economics,
        Self::Experimentation,
    ];
    pub fn key(self) -> &'static str {
        match self {
            Self::FinancialMarkets => "financial_markets",
            Self::Competition => "competition",
            Self::SocialMedia => "social_media",
            Self::Sentiment => "sentiment",
            Self::ProductDiscovery => "product_discovery",
            Self::Recommendations => "recommendations",
            Self::ContentReputation => "content_reputation",
            Self::Pricing => "pricing",
            Self::WillingnessToPay => "willingness_to_pay",
            Self::SocialSimulation => "social_simulation",
            Self::Economics => "economics",
            Self::Experimentation => "experimentation",
        }
    }
    pub fn brief(self) -> &'static str {
        match self {
            Self::FinancialMarkets => "Analyze regime, volatility, multi-timeframe structure, liquidity and macro scenarios. Require real bid/ask or DOM for orderflow claims. State horizon and invalidation; research/paper only.",
            Self::Competition => "Compare offers, prices, positioning and unmet needs. Attribute all observations, date and market. Published offers are not sales.",
            Self::SocialMedia => "Analyze conversations, recurrent questions, diffusion and content opportunities. Sampled comments are not the entire market. Do not infer demographics.",
            Self::Sentiment => "Analyze sentiment separately by price, utility, quality, trust, support and experience. Preserve mixed sentiment, negation, uncertainty and verbatim evidence IDs.",
            Self::ProductDiscovery => "Identify needs and propose features, packages and new products. Distinguish evidence from assumed impact and effort.",
            Self::Recommendations => "Recommend catalog options only when budget, requirements, availability, currency, billing period and dated attributes are supported. Include no purchase and explain trade-offs.",
            Self::ContentReputation => "Propose content, launch and reputation hypotheses. Explain possible interpretations and objections. Synthetic reactions are not real engagement or testimonials.",
            Self::Pricing => "Research RevenueCat and Superwall evidence by category, region, store and plan. Propose pricing, packaging, trials and paywall experiments. Correlation across apps is not a causal price effect.",
            Self::WillingnessToPay => "Estimate willingness to pay for specified offer, segment, currency and billing period. Include competitors and no purchase. Separate benchmark, simulated, stated and observed. Only deterministic study evaluation may support validation.",
            Self::SocialSimulation => "Generate competing consumer interaction scenarios grounded in cited needs, budgets and alternatives. Mark ALL reactions synthetic. Do not equate agents with independent human respondents.",
            Self::Economics => "Explain scenarios using the supplied numerical engine results, if any. Keep gross, net and contribution margin separate; avoid double fees and monthly counting of annual subscriptions. Do not invent calculations.",
            Self::Experimentation => "Design Gabor-Granger or discrete-choice human studies and randomized commercial experiments. Specify population, outcome, relevant effect, maturity window, sample planning and stopping rule. Trial starts and clicks are not payments.",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Limits {
    pub concurrency: usize,
    pub timeout_seconds: u64,
    pub max_output_tokens: u32,
    pub max_cost_usd: f64,
}
impl Default for Limits {
    fn default() -> Self {
        Self {
            concurrency: 3,
            timeout_seconds: 90,
            max_output_tokens: 1800,
            max_cost_usd: 1.0,
        }
    }
}
impl Limits {
    pub fn validate(&self) -> Result<()> {
        ensure!(
            (1..=16).contains(&self.concurrency),
            "Concurrency must be 1..16"
        );
        ensure!(
            (1..=300).contains(&self.timeout_seconds),
            "Timeout must be 1..300 seconds"
        );
        ensure!(
            (128..=8192).contains(&self.max_output_tokens),
            "Output tokens must be 128..8192"
        );
        ensure!(
            self.max_cost_usd.is_finite() && self.max_cost_usd > 0. && self.max_cost_usd <= 100.,
            "Invalid run budget"
        );
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ResearchRequest {
    pub question: String,
    pub as_of: DateTime<Utc>,
    pub modules: Vec<Module>,
    pub evidence_ids: Vec<String>,
    #[serde(default)]
    pub context: serde_json::Value,
    #[serde(default)]
    pub limits: Limits,
    pub study: Option<crate::wtp::Study>,
    pub economy: Option<crate::wtp::Economy>,
}
impl ResearchRequest {
    pub fn validate(&self) -> Result<()> {
        ensure!(
            !self.question.trim().is_empty() && self.question.len() <= 8000,
            "Invalid research question"
        );
        ensure!(
            !self.modules.is_empty() && self.modules.len() <= 12,
            "Select 1..12 modules"
        );
        ensure!(
            self.modules.iter().collect::<BTreeSet<_>>().len() == self.modules.len(),
            "Duplicate modules"
        );
        ensure!(
            !self.evidence_ids.is_empty() && self.evidence_ids.len() <= 100,
            "Select 1..100 evidence records"
        );
        ensure!(
            self.evidence_ids.iter().collect::<BTreeSet<_>>().len() == self.evidence_ids.len(),
            "Duplicate evidence ids"
        );
        ensure!(
            serde_json::to_vec(&self.context)?.len() <= 32_000,
            "Context too large"
        );
        self.limits.validate()?;
        if let Some(s) = &self.study {
            s.validate()?;
        }
        if let Some(e) = &self.economy {
            e.validate()?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Finding {
    pub statement: String,
    pub evidence_ids: Vec<String>,
    pub interpretation: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AgentOutput {
    pub summary: String,
    pub findings: Vec<Finding>,
    pub hypotheses: Vec<String>,
    pub gaps: Vec<String>,
    pub next_experiments: Vec<String>,
}
impl AgentOutput {
    pub fn validate(&self, allowed: &BTreeSet<String>) -> Result<()> {
        ensure!(!self.summary.trim().is_empty(), "Empty agent summary");
        ensure!(
            self.findings.len() <= 30 && self.hypotheses.len() <= 20 && self.gaps.len() <= 30,
            "Agent output exceeds limits"
        );
        for f in &self.findings {
            ensure!(
                !f.statement.trim().is_empty() && !f.evidence_ids.is_empty(),
                "Every finding needs a statement and evidence"
            );
            for id in &f.evidence_ids {
                if !allowed.contains(id) {
                    bail!("Agent cited an unavailable evidence ID");
                }
            }
            ensure!(
                ["observation", "inference", "simulation"].contains(&f.interpretation.as_str()),
                "Invalid finding interpretation"
            );
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentResult {
    pub role: String,
    pub model: String,
    pub status: String,
    pub output: Option<AgentOutput>,
    pub error: Option<String>,
    pub usage: serde_json::Value,
    pub prompt_hash: String,
}
