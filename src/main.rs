use anyhow::{Context, Result};
use clap::{Parser, Subcommand, ValueEnum};
use market_swarm::{
    connectors, evaluation,
    inference::{select_openrouter_model, Inference},
    mcp::Mcp,
    runtime,
    schema::{Evidence, EvidenceKind, Limits, Module, ResearchRequest},
    store::Store,
};
use serde::de::DeserializeOwned;
use serde_json::{json, Value};
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

#[derive(Parser)]
#[command(
    version,
    about = "Evidence-based Rust swarm: markets, products, social research, pricing and WTP"
)]
struct Cli {
    #[arg(long, global = true, default_value = "state/swarm.sqlite")]
    db: PathBuf,
    #[arg(long, global = true)]
    env_file: Option<PathBuf>,
    #[command(subcommand)]
    command: Command,
}
#[derive(Clone, Copy, Debug, ValueEnum)]
enum Mode {
    Openrouter,
    Local,
    Offline,
}
#[derive(Clone, Copy, Debug, ValueEnum)]
enum McpAction {
    Initialize,
    ListTools,
    Call,
}
#[derive(Subcommand)]
enum Command {
    Doctor,
    Modules,
    Models,
    Import {
        file: PathBuf,
    },
    Evidence,
    Fetch {
        url: String,
    },
    Search {
        query: String,
        #[arg(long, default_value_t = 5)]
        count: u8,
    },
    Normalize {
        file: PathBuf,
        #[arg(long)]
        mapping: PathBuf,
        #[arg(long)]
        source: String,
    },
    ApifyDataset {
        dataset: String,
        #[arg(long)]
        mapping: PathBuf,
        #[arg(long, default_value_t = 500)]
        max_items: usize,
    },
    ApifyRun {
        actor: String,
        #[arg(long)]
        input: PathBuf,
        #[arg(long)]
        max_charge_usd: f64,
    },
    ApifyStatus {
        run_id: String,
    },
    Investing {
        #[arg(long)]
        endpoint: String,
        #[arg(long)]
        mapping: PathBuf,
        #[arg(long)]
        query: Option<PathBuf>,
    },
    Run {
        request: PathBuf,
        #[arg(long, value_enum, default_value = "openrouter")]
        mode: Mode,
        #[arg(long, env = "OPENROUTER_MODEL")]
        model: Option<String>,
        #[arg(long, default_value = "http://127.0.0.1:11434/v1/chat/completions")]
        local_url: String,
        #[arg(long, default_value = "output")]
        output: PathBuf,
    },
    Describe {
        files: Vec<PathBuf>,
        #[arg(long, default_value = "Describe each supplied app screenshot: what it shows, all visible text, and any UX or conversion problems.")]
        prompt: String,
        #[arg(long, value_enum, default_value = "openrouter")]
        mode: Mode,
        #[arg(long, env = "OPENROUTER_MODEL")]
        model: Option<String>,
        #[arg(long, default_value = "http://127.0.0.1:11434/v1/chat/completions")]
        local_url: String,
        #[arg(long)]
        import: bool,
    },
    Mcp {
        #[arg(long)]
        command: String,
        #[arg(long)]
        tool: Option<String>,
        #[arg(long)]
        args: Option<String>,
        #[arg(long, value_enum, default_value = "list-tools")]
        action: McpAction,
    },
    Report {
        run_id: String,
        #[arg(long)]
        text: bool,
    },
    Evaluate {
        file: PathBuf,
    },
    Serve {
        #[arg(long, default_value_t = 8787)]
        port: u16,
        #[arg(long, value_enum, default_value = "openrouter")]
        mode: Mode,
        #[arg(long, env = "OPENROUTER_MODEL")]
        model: Option<String>,
        #[arg(long, default_value = "http://127.0.0.1:11434/v1/chat/completions")]
        local_url: String,
    },
}
fn read<T: DeserializeOwned>(path: &Path) -> Result<T> {
    let size = std::fs::metadata(path)
        .context("Cannot read input file metadata")?
        .len();
    anyhow::ensure!(size <= 16_000_000, "Input file exceeds 16 MB");
    serde_json::from_slice(&std::fs::read(path)?)
        .context("Input JSON does not match the required contract")
}
fn print(value: &impl serde::Serialize) -> Result<()> {
    println!("{}", serde_json::to_string_pretty(value)?);
    Ok(())
}
async fn inference(
    mode: Mode,
    model: Option<String>,
    local_url: &str,
    timeout: u64,
) -> Result<Option<Inference>> {
    match mode {
        Mode::Offline => Ok(None),
        Mode::Openrouter => Ok(Some(
            Inference::openrouter(select_openrouter_model(model), timeout).await?,
        )),
        Mode::Local => Ok(Some(Inference::local(
            model.context("Local --model is required")?,
            local_url,
            timeout,
        )?)),
    }
}
#[tokio::main]
async fn main() {
    if let Err(error) = execute().await {
        eprintln!("Error: {error}");
        std::process::exit(1);
    }
}
async fn execute() -> Result<()> {
    let cli = Cli::parse();
    if let Some(path) = &cli.env_file {
        dotenvy::from_path(path).map_err(|_| anyhow::anyhow!("Cannot load selected env file"))?;
    }
    let store = Store::open(&cli.db)?;
    match cli.command {
        Command::Doctor => {
            let credentials = [
                "OPENROUTER_API_KEY",
                "OPENROUTER_KEY",
                "openrouter_key",
                "APIFY_API_TOKEN",
                "APIFY_TOKEN",
                "INVESTING_API_KEY",
                "BRAVE_SEARCH_API_KEY",
                "SWARM_API_TOKEN",
            ]
            .iter()
            .map(|k| {
                (
                    *k,
                    std::env::var(k).ok().is_some_and(|v| !v.trim().is_empty()),
                )
            })
            .collect::<std::collections::BTreeMap<_, _>>();
            print(
                &json!({"rust_runtime":true,"mirofish_dependency":false,"db_ready":true,"modules":Module::ALL,
                "credentials":credentials,
                "openrouter_model_configured":std::env::var("OPENROUTER_MODEL").ok().is_some_and(|s|!s.trim().is_empty()),
                "effective_openrouter_model":select_openrouter_model(std::env::var("OPENROUTER_MODEL").ok()),
                "openrouter_credential_available":connectors::openrouter_credential().is_ok(),
                "integration_status":"Presence checks only; actor/schema and Investing provider still require source-specific live verification"}),
            )?;
        }
        Command::Modules => print(
            &Module::ALL
                .iter()
                .map(|m| json!({"id":m,"brief":m.brief()}))
                .collect::<Vec<_>>(),
        )?,
        Command::Models => {
            let v = connectors::get_json(
                &connectors::client(45)?,
                reqwest::Url::parse("https://openrouter.ai/api/v1/models")?,
                None,
                &[],
            )
            .await?;
            let rows = v["data"].as_array().context("Missing model catalog")?;
            print(&rows.iter().map(|m|json!({"id":m["id"],"pricing":m["pricing"],"supported_parameters":m["supported_parameters"]})).collect::<Vec<_>>())?;
        }
        Command::Import { file } => {
            let e: Vec<Evidence> = read(&file)?;
            print(&json!({"imported":store.import(&e)?,"total_input":e.len()}))?;
        }
        Command::Evidence => print(&store.list_evidence()?)?,
        Command::Fetch { url } => {
            let e = connectors::public_document(&url).await?;
            let count = store.import(std::slice::from_ref(&e))?;
            print(
                &json!({"imported":count,"id":e.id,"source":e.source,"provider":e.provider,"limitations":e.limitations}),
            )?;
        }
        Command::Search { query, count } => print(&connectors::search(&query, count).await?)?,
        Command::Normalize {
            file,
            mapping,
            source,
        } => {
            let e = connectors::normalize(
                &read::<Value>(&file)?,
                &read(&mapping)?,
                &source,
                chrono::Utc::now(),
            )?;
            print(
                &json!({"imported":store.import(&e)?,"ids":e.iter().map(|e|&e.id).collect::<Vec<_>>()}),
            )?;
        }
        Command::ApifyDataset {
            dataset,
            mapping,
            max_items,
        } => {
            let v = connectors::apify_dataset(&dataset, max_items).await?;
            let e = connectors::normalize(
                &v,
                &read(&mapping)?,
                &format!("apify:dataset:{dataset}"),
                chrono::Utc::now(),
            )?;
            print(
                &json!({"imported":store.import(&e)?,"ids":e.iter().map(|e|&e.id).collect::<Vec<_>>()}),
            )?;
        }
        Command::ApifyRun {
            actor,
            input,
            max_charge_usd,
        } => print(&connectors::apify_run(&actor, read(&input)?, max_charge_usd).await?)?,
        Command::ApifyStatus { run_id } => print(&connectors::apify_status(&run_id).await?)?,
        Command::Investing {
            endpoint,
            mapping,
            query,
        } => {
            let query = query
                .map(|p| read::<std::collections::BTreeMap<String, String>>(&p))
                .transpose()?
                .unwrap_or_default();
            let v = connectors::investing(&endpoint, query.into_iter().collect()).await?;
            let e = connectors::normalize(&v, &read(&mapping)?, &endpoint, chrono::Utc::now())?;
            print(
                &json!({"imported":store.import(&e)?,"ids":e.iter().map(|e|&e.id).collect::<Vec<_>>(),"adapter":"generic_bearer_requires_provider_verification"}),
            )?;
        }
        Command::Run {
            request,
            mode,
            model,
            local_url,
            output,
        } => {
            let request: ResearchRequest = read(&request)?;
            request.validate()?;
            let provider =
                inference(mode, model, &local_url, request.limits.timeout_seconds).await?;
            let report = runtime::run(&store, request, provider).await?;
            let dir = output.join(&report.run_id);
            std::fs::create_dir_all(&dir)?;
            std::fs::write(dir.join("report.json"), serde_json::to_vec_pretty(&report)?)?;
            std::fs::write(dir.join("report.txt"), runtime::render(&report))?;
            print(
                &json!({"run_id":report.run_id,"status":report.status,"agents":report.agents.len(),"reserved_cost_usd":report.reserved_cost_usd,"report_json":dir.join("report.json"),"report_text":dir.join("report.txt")}),
            )?;
        }
        Command::Describe {
            files,
            prompt,
            mode,
            model,
            local_url,
            import,
        } => {
            anyhow::ensure!(
                !files.is_empty() && files.len() <= 20,
                "Provide 1..20 image files"
            );
            let mut images = Vec::new();
            for path in &files {
                let bytes = std::fs::read(path)
                    .with_context(|| format!("Cannot read image file: {}", path.display()))?;
                anyhow::ensure!(bytes.len() <= 8_000_000, "Image exceeds 8 MB");
                let ext = path
                    .extension()
                    .and_then(|e| e.to_str())
                    .unwrap_or("")
                    .to_ascii_lowercase();
                let mime = match ext.as_str() {
                    "png" => "image/png",
                    "jpg" | "jpeg" => "image/jpeg",
                    "webp" => "image/webp",
                    "gif" => "image/gif",
                    other => anyhow::bail!("Unsupported image extension: {other}"),
                };
                images.push((mime.to_owned(), bytes));
            }
            let provider = inference(mode, model, &local_url, 90)
                .await?
                .context("Image description requires a live model (openrouter or local)")?;
            let limits = Limits {
                max_output_tokens: 4096,
                ..Default::default()
            };
            let run = format!(
                "describe-{}",
                &market_swarm::digest(
                    format!(
                        "{:?}|{prompt}",
                        std::time::SystemTime::now()
                            .duration_since(std::time::UNIX_EPOCH)
                            .map(|d| d.as_nanos())
                            .unwrap_or_default()
                    )
                    .as_bytes()
                )[..24]
            );
            store.start(
                &run,
                &serde_json::to_string(&json!({
                    "prompt": prompt,
                    "files": files.iter().map(|p| p.display().to_string()).collect::<Vec<_>>()
                }))?,
            )?;
            let description = provider
                .describe_images(&prompt, &images, &limits, &store, &run)
                .await?;
            if import {
                let content = serde_json::to_string(&description)?;
                let now = chrono::Utc::now();
                let evidence = Evidence {
                    id: format!("vision-{}", &market_swarm::digest(content.as_bytes())[..24]),
                    source: files
                        .iter()
                        .map(|p| p.display().to_string())
                        .collect::<Vec<_>>()
                        .join(", "),
                    provider: "vision_ocr".into(),
                    kind: EvidenceKind::Simulated,
                    published_at: None,
                    collected_at: now,
                    observed_at: now,
                    segment: None,
                    population: None,
                    content,
                    limitations: vec!["Synthetic vision/OCR description of supplied screenshots; not independent human observation.".into()],
                    synthetic_fixture: true,
                };
                let imported = store.import(std::slice::from_ref(&evidence))?;
                print(
                    &json!({"imported":imported,"evidence_id":evidence.id,"description":description}),
                )?;
            } else {
                print(&description)?;
            }
        }
        Command::Mcp {
            command,
            tool,
            args,
            action,
        } => {
            let mut client = Mcp::connect(&command).await?;
            match action {
                McpAction::Initialize => print(&client.initialize().await?)?,
                McpAction::ListTools => print(&client.list_tools().await?)?,
                McpAction::Call => {
                    let tool = tool.context("--tool is required for --action call")?;
                    let args: Value = args
                        .map(|a| serde_json::from_str(&a))
                        .transpose()
                        .context("--args must be a JSON object or array")?
                        .unwrap_or_else(|| json!({}));
                    print(&client.call_tool(&tool, args).await?)?;
                }
            }
        }
        Command::Report { run_id, text } => {
            let report = store.report(&run_id)?.context("Report not found")?;
            if text {
                println!("{}", runtime::render(&serde_json::from_value(report)?));
            } else {
                print(&report)?;
            }
        }
        Command::Evaluate { file } => print(&evaluation::compare(
            &read::<Vec<evaluation::Case>>(&file)?,
        )?)?,
        Command::Serve {
            port,
            mode,
            model,
            local_url,
        } => {
            let token = connectors::credential("SWARM_API_TOKEN")?;
            anyhow::ensure!(
                token.len() >= 24,
                "SWARM_API_TOKEN must contain at least 24 characters"
            );
            let inference = inference(mode, model, &local_url, 90).await?;
            serve(store, inference, token, port).await?;
        }
    }
    Ok(())
}

#[derive(Clone)]
struct App {
    store: Store,
    inference: Option<Inference>,
    token: Arc<String>,
    gate: Arc<tokio::sync::Semaphore>,
}
type ApiResult =
    std::result::Result<axum::Json<Value>, (axum::http::StatusCode, axum::Json<Value>)>;
fn api_error(
    status: axum::http::StatusCode,
    message: &str,
) -> (axum::http::StatusCode, axum::Json<Value>) {
    (status, axum::Json(json!({"error":message})))
}
fn auth(
    headers: &axum::http::HeaderMap,
    state: &App,
) -> std::result::Result<(), (axum::http::StatusCode, axum::Json<Value>)> {
    let header = headers
        .get("authorization")
        .and_then(|s| s.to_str().ok())
        .unwrap_or("");
    if header != format!("Bearer {}", state.token) {
        return Err(api_error(
            axum::http::StatusCode::UNAUTHORIZED,
            "Unauthorized",
        ));
    }
    Ok(())
}
async fn health() -> axum::Json<Value> {
    axum::Json(json!({"status":"ok","runtime":"rust","version":env!("CARGO_PKG_VERSION")}))
}
async fn api_evidence(
    axum::extract::State(state): axum::extract::State<App>,
    headers: axum::http::HeaderMap,
) -> ApiResult {
    auth(&headers, &state)?;
    state
        .store
        .list_evidence()
        .map(|e| axum::Json(json!(e)))
        .map_err(|_| {
            api_error(
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                "Storage error",
            )
        })
}
async fn api_import(
    axum::extract::State(state): axum::extract::State<App>,
    headers: axum::http::HeaderMap,
    axum::Json(e): axum::Json<Vec<Evidence>>,
) -> ApiResult {
    auth(&headers, &state)?;
    state
        .store
        .import(&e)
        .map(|n| axum::Json(json!({"imported":n})))
        .map_err(|_| {
            api_error(
                axum::http::StatusCode::BAD_REQUEST,
                "Invalid or conflicting evidence",
            )
        })
}
async fn api_report(
    axum::extract::State(state): axum::extract::State<App>,
    headers: axum::http::HeaderMap,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> ApiResult {
    auth(&headers, &state)?;
    match state.store.report(&id) {
        Ok(Some(r)) => Ok(axum::Json(r)),
        Ok(None) => Err(api_error(
            axum::http::StatusCode::NOT_FOUND,
            "Report not found",
        )),
        Err(_) => Err(api_error(
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            "Storage error",
        )),
    }
}
async fn api_research(
    axum::extract::State(state): axum::extract::State<App>,
    headers: axum::http::HeaderMap,
    axum::Json(request): axum::Json<ResearchRequest>,
) -> ApiResult {
    auth(&headers, &state)?;
    let _permit = state.gate.try_acquire().map_err(|_| {
        api_error(
            axum::http::StatusCode::TOO_MANY_REQUESTS,
            "Research worker busy",
        )
    })?;
    runtime::run(&state.store, request, state.inference.clone())
        .await
        .map(|r| axum::Json(json!(r)))
        .map_err(|_| {
            api_error(
                axum::http::StatusCode::BAD_REQUEST,
                "Research failed validation or execution; inspect persisted task status",
            )
        })
}
#[derive(serde::Deserialize)]
struct VisionRequest {
    #[serde(default)]
    prompt: Option<String>,
    images: Vec<VisionImage>,
}
#[derive(serde::Deserialize)]
struct VisionImage {
    mime: String,
    data_base64: String,
}
async fn api_vision(
    axum::extract::State(state): axum::extract::State<App>,
    headers: axum::http::HeaderMap,
    axum::Json(request): axum::Json<VisionRequest>,
) -> ApiResult {
    auth(&headers, &state)?;
    let Some(inference) = &state.inference else {
        return Err(api_error(
            axum::http::StatusCode::SERVICE_UNAVAILABLE,
            "Vision requires a live model; this server was started offline",
        ));
    };
    if request.images.is_empty() || request.images.len() > 20 {
        return Err(api_error(
            axum::http::StatusCode::BAD_REQUEST,
            "Provide 1..20 images",
        ));
    }
    let mut images = Vec::with_capacity(request.images.len());
    for image in &request.images {
        let mime = image.mime.trim().to_ascii_lowercase();
        if !["image/png", "image/jpeg", "image/webp", "image/gif"]
            .contains(&mime.as_str())
        {
            return Err(api_error(
                axum::http::StatusCode::BAD_REQUEST,
                "Unsupported image mime type",
            ));
        }
        let bytes = match market_swarm::base64::decode(image.data_base64.trim()) {
            Ok(bytes) if !bytes.is_empty() && bytes.len() <= 8_000_000 => bytes,
            _ => {
                return Err(api_error(
                    axum::http::StatusCode::BAD_REQUEST,
                    "Invalid or oversized base64 image data",
                ))
            }
        };
        images.push((mime, bytes));
    }
    let prompt = request.prompt.clone().unwrap_or_else(|| {
        "Describe each supplied app screenshot: what it shows, all visible text, and any UX or conversion problems.".into()
    });
    let limits = Limits {
        max_output_tokens: 4096,
        ..Default::default()
    };
    let run = format!(
        "vision-{}",
        &market_swarm::digest(
            format!(
                "{:?}|{prompt}",
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .map(|d| d.as_nanos())
                    .unwrap_or_default()
            )
            .as_bytes()
        )[..24]
    );
    let meta =
        serde_json::to_string(&json!({"prompt": prompt, "images": request.images.len()}))
            .unwrap_or_default();
    state.store.start(&run, &meta).map_err(|_| {
        api_error(
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            "Storage error",
        )
    })?;
    match inference
        .describe_images(&prompt, &images, &limits, &state.store, &run)
        .await
    {
        Ok(description) => Ok(axum::Json(json!({"run_id": run, "description": description}))),
        Err(_) => Err(api_error(
            axum::http::StatusCode::BAD_REQUEST,
            "Vision failed; inspect the persisted response for details",
        )),
    }
}
async fn serve(store: Store, inference: Option<Inference>, token: String, port: u16) -> Result<()> {
    let state = App {
        store,
        inference,
        token: Arc::new(token),
        gate: Arc::new(tokio::sync::Semaphore::new(1)),
    };
    let app = axum::Router::new()
        .route("/health", axum::routing::get(health))
        .route(
            "/evidence",
            axum::routing::get(api_evidence).post(api_import),
        )
        .route("/research", axum::routing::post(api_research))
        .route(
            "/vision",
            axum::routing::post(api_vision)
                .layer(axum::extract::DefaultBodyLimit::max(32_000_000)),
        )
        .route("/reports/{id}", axum::routing::get(api_report))
        .layer(axum::extract::DefaultBodyLimit::max(2_000_000))
        .with_state(state);
    let listener = tokio::net::TcpListener::bind((std::net::Ipv4Addr::LOCALHOST, port)).await?;
    println!(
        "Listening on http://{} (authenticated API)",
        listener.local_addr()?
    );
    axum::serve(listener, app)
        .with_graceful_shutdown(async {
            let _ = tokio::signal::ctrl_c().await;
        })
        .await?;
    Ok(())
}
