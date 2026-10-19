use crate::schema::{Evidence, EvidenceKind};
use anyhow::{bail, ensure, Context, Result};
use chrono::{DateTime, Utc};
use reqwest::{Client, Response, Url};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::time::Duration;

pub fn client(timeout: u64) -> Result<Client> {
    Ok(Client::builder()
        .timeout(Duration::from_secs(timeout))
        .redirect(reqwest::redirect::Policy::none())
        .user_agent("market-swarm/0.1 research")
        .build()?)
}
pub fn safe_url(raw: &str, local_allowed: bool) -> Result<Url> {
    let u = Url::parse(raw).context("Invalid URL")?;
    ensure!(
        u.username().is_empty()
            && u.password().is_none()
            && u.query().is_none()
            && u.fragment().is_none(),
        "URLs cannot contain credentials, query strings or fragments; use separate API parameters"
    );
    let local = matches!(u.host_str(), Some("localhost" | "127.0.0.1" | "[::1]"));
    ensure!(
        u.scheme() == "https" || (local_allowed && local && u.scheme() == "http"),
        "HTTPS is required except for explicit local inference"
    );
    Ok(u)
}
pub fn safe_reference(raw: &str) -> Result<Url> {
    let url = Url::parse(raw).context("Invalid public source URL")?;
    ensure!(
        url.scheme() == "https" && url.username().is_empty() && url.password().is_none(),
        "Public sources require HTTPS without credentials"
    );
    ensure!(
        !url.query_pairs().any(|(k, _)| matches!(
            k.to_lowercase().as_str(),
            "token" | "key" | "api_key" | "apikey" | "authorization" | "access_token"
        )),
        "Public source URL contains credentials"
    );
    Ok(url)
}
pub async fn bounded(mut response: Response, max: usize) -> Result<Vec<u8>> {
    ensure!(
        response.status().is_success(),
        "Remote service returned HTTP {}",
        response.status().as_u16()
    );
    if let Some(n) = response.content_length() {
        ensure!(n <= max as u64, "Remote response exceeds byte limit");
    }
    let mut bytes = Vec::new();
    while let Some(chunk) = response
        .chunk()
        .await
        .map_err(|_| anyhow::anyhow!("Response body transfer failed"))?
    {
        ensure!(
            bytes.len() + chunk.len() <= max,
            "Remote response exceeds byte limit"
        );
        bytes.extend_from_slice(&chunk);
    }
    Ok(bytes)
}
pub async fn get_json(
    client: &Client,
    url: Url,
    bearer: Option<&str>,
    query: &[(&str, String)],
) -> Result<Value> {
    for attempt in 0..3 {
        let mut req = client.get(url.clone()).query(query);
        if let Some(key) = bearer {
            req = req.bearer_auth(key);
        }
        let resp = req
            .send()
            .await
            .map_err(|_| anyhow::anyhow!("HTTP GET failed (details redacted)"))?;
        if (resp.status().as_u16() == 429 || resp.status().is_server_error()) && attempt < 2 {
            tokio::time::sleep(Duration::from_millis(300 * (attempt + 1))).await;
            continue;
        }
        return serde_json::from_slice(&bounded(resp, 8_000_000).await?)
            .context("Service returned invalid JSON");
    }
    bail!("Read retries exhausted")
}
pub fn credential(name: &str) -> Result<String> {
    std::env::var(name)
        .ok()
        .filter(|v| !v.trim().is_empty())
        .ok_or_else(|| anyhow::anyhow!("Missing server credential: {name}"))
}
pub fn openrouter_credential() -> Result<String> {
    for name in ["OPENROUTER_API_KEY", "OPENROUTER_KEY", "openrouter_key"] {
        if let Ok(key) = credential(name) {
            return Ok(key);
        }
    }
    bail!("Missing server credential: OPENROUTER_API_KEY (also accepts OPENROUTER_KEY or openrouter_key)")
}
pub async fn public_document(url: &str) -> Result<Evidence> {
    let url = safe_reference(url)?;
    let now = Utc::now();
    let response = client(45)?
        .get(url.clone())
        .send()
        .await
        .map_err(|_| anyhow::anyhow!("Public document fetch failed (details redacted)"))?;
    // Explicit redirect handling prevents API credentials being forwarded and
    // ensures the stored source is the document that was actually read.
    let response = if response.status().is_redirection() {
        let location = response
            .headers()
            .get("location")
            .context("Redirect missing Location")?
            .to_str()?;
        let dest = url.join(location)?;
        let dest = safe_reference(dest.as_str())?;
        client(45)?
            .get(dest)
            .send()
            .await
            .map_err(|_| anyhow::anyhow!("Redirect fetch failed"))?
    } else {
        response
    };
    let final_url = response.url().to_string();
    let content_type = response
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("")
        .to_owned();
    ensure!(
        content_type.contains("text/") || content_type.contains("json"),
        "Unsupported document format; import an attributed text extract"
    );
    let bytes = bounded(response, 4_000_000).await?;
    let raw = String::from_utf8_lossy(&bytes);
    let text = if content_type.contains("html") {
        use scraper::{Html, Selector};
        let html = Html::parse_document(&raw);
        let select = Selector::parse("main, article").unwrap();
        let root = html.select(&select).next().unwrap_or(html.root_element());
        root.descendants()
            .filter_map(|node| {
                let text = node.value().as_text()?;
                let excluded = node
                    .ancestors()
                    .filter_map(scraper::ElementRef::wrap)
                    .any(|e| {
                        matches!(
                            e.value().name(),
                            "script" | "style" | "noscript" | "nav" | "footer"
                        )
                    });
                (!excluded).then(|| text.to_string())
            })
            .collect::<Vec<_>>()
            .join(" ")
    } else {
        raw.to_string()
    };
    let content = text.split_whitespace().collect::<Vec<_>>().join(" ");
    let content: String = content.chars().take(70_000).collect();
    ensure!(
        content.len() >= 100,
        "Document has insufficient readable content"
    );
    let hostname = Url::parse(&final_url)?
        .host_str()
        .unwrap_or_default()
        .to_owned();
    let provider = if hostname == "revenuecat.com" || hostname.ends_with(".revenuecat.com") {
        "revenuecat"
    } else if hostname == "superwall.com" || hostname.ends_with(".superwall.com") {
        "superwall"
    } else {
        "public_document"
    };
    let id = format!(
        "doc-{}",
        &crate::digest(format!("{final_url}\n{content}").as_bytes())[..24]
    );
    Ok(Evidence { id, source: final_url, provider: provider.into(), kind: EvidenceKind::Benchmark,
        published_at: None, collected_at: now, observed_at: now, segment: None, population: None,
        content, limitations: vec!["Publication date and population require extraction and verification; collection date is not publication date.".into(),
        "Text extraction may omit tables/images; partial extract limited to 70,000 characters. Correlation and selected case studies do not establish causal effects.".into()], synthetic_fixture: false })
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Mapping {
    #[serde(default)]
    pub synthetic_fixture: bool,
    pub items_pointer: String,
    pub id_field: Option<String>,
    pub text_fields: Vec<String>,
    pub url_field: Option<String>,
    pub timestamp_field: String,
    pub provider: String,
    pub kind: EvidenceKind,
    pub segment: Option<String>,
    pub population: Option<String>,
}
pub fn normalize(
    value: &Value,
    mapping: &Mapping,
    fallback_source: &str,
    collected_at: DateTime<Utc>,
) -> Result<Vec<Evidence>> {
    ensure!(
        !mapping.text_fields.is_empty() && !mapping.provider.trim().is_empty(),
        "Mapping requires text fields and provider"
    );
    let items = value
        .pointer(&mapping.items_pointer)
        .context("Missing mapped item array")?
        .as_array()
        .context("Mapped items are not an array")?;
    ensure!(items.len() <= 10_000, "Import batch too large");
    let mut out = vec![];
    for item in items {
        let time = item
            .pointer(&mapping.timestamp_field)
            .context("Missing observation timestamp")?;
        let observed_at = if let Some(s) = time.as_str() {
            DateTime::parse_from_rfc3339(s)
                .context("Timestamp must be RFC3339 or epoch seconds")?
                .with_timezone(&Utc)
        } else if let Some(n) = time.as_i64() {
            DateTime::from_timestamp(n, 0).context("Invalid epoch timestamp")?
        } else {
            bail!("Invalid timestamp field");
        };
        let parts: Vec<_> = mapping
            .text_fields
            .iter()
            .filter_map(|field| {
                item.pointer(field).map(|v| {
                    format!(
                        "{}: {}",
                        field,
                        v.as_str()
                            .map(str::to_owned)
                            .unwrap_or_else(|| v.to_string())
                    )
                })
            })
            .collect();
        ensure!(!parts.is_empty(), "Record has no mapped text fields");
        let content = parts.join("\n");
        let source = mapping
            .url_field
            .as_ref()
            .and_then(|f| item.pointer(f))
            .and_then(Value::as_str)
            .unwrap_or(fallback_source);
        if source.starts_with("http") {
            safe_reference(source)?;
        }
        let remote_id = mapping
            .id_field
            .as_ref()
            .and_then(|f| item.pointer(f))
            .map(Value::to_string)
            .unwrap_or_default();
        let id = format!(
            "{}-{}",
            mapping.provider,
            &crate::digest(format!("{source}:{remote_id}:{observed_at}:{content}").as_bytes())
                [..24]
        );
        let e = Evidence { id, source: source.into(), provider: mapping.provider.clone(), kind: mapping.kind,
            published_at: None, collected_at, observed_at, segment: mapping.segment.clone(), population: mapping.population.clone(), content,
            limitations: vec!["Fields normalized through an explicit mapping; coverage and sampling depend on the upstream source.".into()], synthetic_fixture: mapping.synthetic_fixture || item.get("synthetic_fixture").and_then(Value::as_bool).unwrap_or(false) };
        e.validate()?;
        out.push(e);
    }
    Ok(out)
}

pub async fn apify_dataset(dataset: &str, max_items: usize) -> Result<Value> {
    ensure!(
        dataset
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '-')
            && !dataset.is_empty(),
        "Invalid dataset ID"
    );
    ensure!(
        (1..=10_000).contains(&max_items),
        "Select 1..10000 dataset items"
    );
    let key = credential("APIFY_API_TOKEN").or_else(|_| credential("APIFY_TOKEN"))?;
    let c = client(60)?;
    let url = Url::parse(&format!(
        "https://api.apify.com/v2/datasets/{dataset}/items"
    ))?;
    let mut all = vec![];
    while all.len() < max_items {
        let limit = (max_items - all.len()).min(250);
        let v = get_json(
            &c,
            url.clone(),
            Some(&key),
            &[
                ("format", "json".into()),
                ("clean", "true".into()),
                ("offset", all.len().to_string()),
                ("limit", limit.to_string()),
            ],
        )
        .await?;
        let page = v.as_array().context("Unexpected Apify dataset response")?;
        ensure!(page.len() <= limit, "Apify returned too many records");
        all.extend(page.iter().cloned());
        if page.len() < limit {
            break;
        }
    }
    Ok(Value::Array(all))
}
pub async fn apify_run(actor: &str, input: Value, max_charge_usd: f64) -> Result<Value> {
    ensure!(
        !actor.is_empty()
            && actor
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '_' | '~')),
        "Use actor ID or owner~name"
    );
    ensure!(
        max_charge_usd.is_finite() && max_charge_usd > 0. && max_charge_usd <= 100.,
        "Explicit actor budget required"
    );
    let key = credential("APIFY_API_TOKEN").or_else(|_| credential("APIFY_TOKEN"))?;
    let response = client(45)?
        .post(format!("https://api.apify.com/v2/actors/{actor}/runs"))
        .bearer_auth(key)
        .query(&[
            ("maxTotalChargeUsd", max_charge_usd.to_string()),
            ("timeout", "300".into()),
        ])
        .json(&input)
        .send()
        .await
        .map_err(|_| {
            anyhow::anyhow!("Actor start outcome unknown; inspect Apify runs before retrying")
        })?;
    let body: Value = serde_json::from_slice(&bounded(response, 1_000_000).await?)?;
    let d = body
        .get("data")
        .context("Apify response missing run data")?;
    Ok(
        json!({"id":d["id"],"status":d["status"],"default_dataset_id":d["defaultDatasetId"],"max_total_charge_usd":max_charge_usd}),
    )
}
pub async fn apify_status(run: &str) -> Result<Value> {
    ensure!(
        !run.is_empty() && run.chars().all(|c| c.is_ascii_alphanumeric()),
        "Invalid Apify run ID"
    );
    let key = credential("APIFY_API_TOKEN").or_else(|_| credential("APIFY_TOKEN"))?;
    let body = get_json(
        &client(30)?,
        Url::parse(&format!("https://api.apify.com/v2/actor-runs/{run}"))?,
        Some(&key),
        &[],
    )
    .await?;
    let d = body
        .get("data")
        .context("Apify response missing run data")?;
    Ok(
        json!({"id":d["id"],"status":d["status"],"default_dataset_id":d["defaultDatasetId"],"finished_at":d["finishedAt"],"usage_total_usd":d["usageTotalUsd"]}),
    )
}
pub async fn investing(endpoint: &str, query: Vec<(String, String)>) -> Result<Value> {
    let url = safe_url(endpoint, false)?;
    let key = credential("INVESTING_API_KEY")?;
    let refs: Vec<_> = query.iter().map(|(k, v)| (k.as_str(), v.clone())).collect();
    // Generic bearer adapter: upstream authentication/schema must be verified.
    get_json(&client(45)?, url, Some(&key), &refs).await
}
pub async fn search(query: &str, count: u8) -> Result<Value> {
    ensure!(
        !query.trim().is_empty() && query.len() <= 1000 && (1..=10).contains(&count),
        "Invalid search query/count"
    );
    let response = client(30)?
        .get("https://api.search.brave.com/res/v1/web/search")
        .header("X-Subscription-Token", credential("BRAVE_SEARCH_API_KEY")?)
        .query(&[("q", query.to_owned()), ("count", count.to_string())])
        .send()
        .await
        .map_err(|_| anyhow::anyhow!("Search request failed"))?;
    let body: Value = serde_json::from_slice(&bounded(response, 2_000_000).await?)?;
    let hits = body
        .pointer("/web/results")
        .and_then(Value::as_array)
        .context("Search returned no web results")?;
    Ok(
        json!({"query":query,"results":hits.iter().map(|h|json!({"title":h["title"],"url":h["url"],"description":h["description"],"status":"discovered_not_read"})).collect::<Vec<_>>()}),
    )
}
