//! UX specialists, priority backlog and optional neural review.
//!
//! Contrast, copy heuristics and the backlog are deterministic. Synthetic
//! personas are rules and are stored separately from design findings.

use super::contrast::{self, Rgba};
use super::pen::{self, Design, Screen, TextNode};
use crate::{
    inference::Inference,
    schema::{AgentOutput, AgentResult, Evidence, EvidenceKind, Finding, Limits},
    store::Store,
};
use anyhow::{ensure, Context, Result};
use chrono::{DateTime, Utc};
use serde::Serialize;
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};

pub const SYNTHETIC_NOTICE: &str = "El panel sintético son personas artificiales definidas por reglas del diseño. No son usuarios reales, no son una encuesta y no miden descarga ni disposición a pagar.";
pub const NO_COMPETITIVE_EVIDENCE: &str = "No hay evidencia competitiva citada. Usa market-swarm search y fetch, importa los documentos y vuelve a auditar con --evidence-ids. Este especialista no inventa rankings ni cifras de apps líderes.";

#[derive(Debug, Clone)]
pub struct AuditOptions {
    pub source_name: String,
    pub category: Option<String>,
    pub question: String,
    pub as_of: DateTime<Utc>,
    pub competitive: Vec<Evidence>,
    pub screenshots: Vec<Screenshot>,
}

#[derive(Debug, Clone)]
pub struct Screenshot {
    pub path: String,
    pub bytes: Vec<u8>,
}

#[derive(Debug, Clone, Serialize)]
pub struct UxReport {
    pub audit: &'static str,
    pub status: String,
    pub source: String,
    pub source_sha256: String,
    pub as_of: DateTime<Utc>,
    pub question: String,
    pub category: Option<String>,
    pub pen_version: String,
    pub stats: Stats,
    pub variables: Vec<VariableInfo>,
    pub palette: Vec<PaletteEntry>,
    pub screens: Vec<ScreenView>,
    pub contrast: Vec<ContrastRow>,
    pub specialists: Vec<AgentResult>,
    pub critic: AgentResult,
    pub synthesis: AgentResult,
    pub backlog: Vec<BacklogItem>,
    pub synthetic_opinions: Vec<BacklogItem>,
    pub synthetic_panel_notice: String,
    pub sources: Vec<SourceRef>,
    pub limitations: Vec<String>,
    pub warnings: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imported: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub neural: Option<NeuralPass>,
    #[serde(skip)]
    pub evidence: Vec<Evidence>,
}

#[derive(Debug, Clone, Serialize)]
pub struct NeuralPass {
    pub status: String,
    pub critic: AgentResult,
    pub synthesis: AgentResult,
}

#[derive(Debug, Clone, Serialize)]
pub struct Stats {
    pub screens: usize,
    pub text_nodes: usize,
    pub ctas: usize,
    pub contrast_measured: usize,
    pub contrast_failures: usize,
    pub paid_price_mentions: usize,
    pub free_mentions: usize,
    pub unmeasured_non_solid: usize,
    pub screenshots_linked: usize,
    pub screenshots_unmatched: usize,
}

#[derive(Debug, Clone, Serialize)]
pub struct VariableInfo {
    pub name: String,
    pub kind: String,
    pub display: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct PaletteEntry {
    pub hex: String,
    pub uses: usize,
}

#[derive(Debug, Clone, Serialize)]
pub struct ScreenView {
    pub id: String,
    pub name: String,
    pub evidence_id: String,
    pub role: String,
    pub width: Option<f64>,
    pub height: Option<f64>,
    pub placeholder: bool,
    pub texts: Vec<String>,
    pub ctas: Vec<String>,
    pub price_mentions: Vec<String>,
    pub free_mentions: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub screenshot: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub screenshot_px: Option<[u32; 2]>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ContrastRow {
    pub screen_id: String,
    pub screen_name: String,
    pub node_path: String,
    pub text: String,
    pub font_size: Option<f64>,
    pub large_text: bool,
    pub foreground: String,
    pub background: String,
    pub ratio: f64,
    pub aa_minimum: f64,
    pub passes_aa: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct BacklogItem {
    pub id: String,
    pub specialist: String,
    pub kind: String,
    pub band: String,
    pub impact: u8,
    pub effort: u8,
    pub screen_id: String,
    pub screen_name: String,
    pub node_path: String,
    pub statement: String,
    pub recommendation: String,
    pub interpretation: String,
    pub evidence_ids: Vec<String>,
    pub not_real_users: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct SourceRef {
    pub id: String,
    pub source: String,
    pub provider: String,
    pub kind: EvidenceKind,
    pub synthetic_fixture: bool,
    pub role: String,
}

struct Issue {
    specialist: &'static str,
    kind: &'static str,
    screen_index: Option<usize>,
    also_screens: Vec<usize>,
    node_path: String,
    statement: String,
    recommendation: String,
    interpretation: &'static str,
    impact: u8,
    effort: u8,
    extra_evidence: Vec<String>,
    evidence_ids: Vec<String>,
    not_real_users: bool,
    screen_id: String,
    screen_name: String,
}

pub fn specialist_catalog() -> Vec<Value> {
    [
        ("visual_design", "Paleta resuelta y contraste WCAG 2.x sobre rellenos sólidos. El ratio no lo estima un modelo."),
        ("information_architecture", "Inventario de pantallas, secciones, etiquetas repetidas y pasos sin regreso."),
        ("cta_copy", "CTAs heurísticos, duplicados cercanos y la salida cuando es la única acción."),
        ("accessibility", "Texto pequeño, objetivos bajo 44 px, iconos sin rótulo y avisos demasiado chicos."),
        ("onboarding_paywall", "Precio, paywall y longitud de onboarding. Decir gratis no sustituye un importe."),
        ("competitive_benchmark", "Compara el diseño solo contra evidencia citada. No inventa rankings ni cifras."),
        ("synthetic_panel", "Opiniones de personas artificiales. No son usuarios reales."),
        ("critic", "Recomprueba citas, ratios y que la simulación no se presente como observación."),
        ("synthesis", "Backlog por impacto y esfuerzo: hacer ahora, planificar, pulir, diferir."),
    ]
    .into_iter()
    .map(|(id, decides)| json!({"id": id, "decides": decides, "deterministic": true}))
    .collect()
}

pub fn audit(bytes: &[u8], options: &AuditOptions) -> Result<UxReport> {
    ensure!(
        !options.question.trim().is_empty() && options.question.len() <= 2000,
        "Invalid UX question"
    );
    ensure!(
        options.category.as_ref().map(|c| c.len()).unwrap_or(0) <= 200,
        "Category is too long"
    );
    ensure!(bytes.len() <= 64_000_000, "pen file exceeds 64 MB");
    let design = pen::parse_design(bytes)?;
    let source_sha256 = crate::digest(bytes);
    let doc = &source_sha256[..12];
    let mut used = BTreeSet::new();
    let document_id = mint_id(doc, "document", &mut used);
    let panel_id = mint_id(doc, "synthetic-panel", &mut used);
    let screen_ids: Vec<String> = design
        .screens
        .iter()
        .map(|screen| mint_id(doc, &screen.id, &mut used))
        .collect();
    let mut competitive = Vec::new();
    let mut warnings = design.warnings.clone();
    for evidence in &options.competitive {
        evidence
            .validate()
            .context("Competitive evidence failed validation")?;
        if evidence.provider == "pen_design" || evidence.provider == "synthetic_panel" {
            warnings.push(format!(
                "Ignored {} as competitive evidence because it is design or panel output.",
                evidence.id
            ));
            continue;
        }
        if !evidence.eligible(options.as_of) {
            warnings.push(format!(
                "Excluded future competitive evidence {}.",
                evidence.id
            ));
            continue;
        }
        competitive.push(evidence.clone());
    }
    let mut built = build(&design, &screen_ids, &panel_id, &competitive, options)?;
    link_screenshots(&mut built, &design, options, &mut warnings);
    annotate_screens(&mut built.issues, &design);
    for issue in &mut built.issues {
        bind_evidence(issue, &screen_ids, &document_id);
    }
    let evidence = evidence_records(
        &design,
        &built,
        &screen_ids,
        &document_id,
        &panel_id,
        options,
        &source_sha256,
    );
    let allowed: BTreeSet<String> = evidence
        .iter()
        .map(|item| item.id.clone())
        .chain(competitive.iter().map(|item| item.id.clone()))
        .collect();
    let mut specialists = Vec::new();
    for role in [
        "visual_design",
        "information_architecture",
        "cta_copy",
        "accessibility",
        "onboarding_paywall",
        "competitive_benchmark",
        "synthetic_panel",
    ] {
        specialists.push(agent(
            role,
            specialist_output(role, &built.issues, &allowed)?,
        )?);
    }
    let (backlog, opinions) = priority_lists(&built.issues);
    let critic = agent(
        "critic",
        critic_output(
            &built,
            &backlog,
            &opinions,
            &document_id,
            &panel_id,
            &allowed,
        )?,
    )?;
    let synthesis = agent(
        "synthesis",
        synthesis_output(&backlog, &built.stats, &document_id, options, &allowed)?,
    )?;
    let mut sources: Vec<SourceRef> = evidence
        .iter()
        .map(|item| SourceRef {
            id: item.id.clone(),
            source: item.source.clone(),
            provider: item.provider.clone(),
            kind: item.kind,
            synthetic_fixture: item.synthetic_fixture,
            role: if item.id == document_id {
                "document".into()
            } else if item.id == panel_id {
                "synthetic_panel".into()
            } else {
                "screen".into()
            },
        })
        .collect();
    for item in &competitive {
        sources.push(SourceRef {
            id: item.id.clone(),
            source: item.source.clone(),
            provider: item.provider.clone(),
            kind: item.kind,
            synthetic_fixture: item.synthetic_fixture,
            role: "competitive".into(),
        });
    }
    let mut limitations = vec![
        "El contraste es WCAG 2.x sobre rellenos sólidos resueltos (variables, tema y alfa compuestos en sRGB). Degradados, imágenes, shaders, efectos y modos de mezcla quedan sin medir.".into(),
        "Los CTAs, el rol de cada pantalla y la navegación son heurísticos sobre el árbol .pen. No ejecutan scripts ni sustituyen una prueba con personas.".into(),
        "Una ausencia (por ejemplo de precio) vale solo para el texto extraído de este archivo, no para la app publicada.".into(),
        SYNTHETIC_NOTICE.into(),
        "La evidencia de diseño usa kind=simulated y synthetic_fixture=false: es un artefacto leído, no un resultado de mercado ni una respuesta humana. El panel usa synthetic_fixture=true.".into(),
        "Las comparaciones competitivas son inferencias sobre evidencia citada. Este motor no declara apps líderes ni cifras que no estén en esas fuentes.".into(),
    ];
    if design.has_scripts {
        limitations.push("El archivo contiene nodos script. No se ejecutaron; sus hijos generados no están en el informe.".into());
    }
    if built.stats.screenshots_linked > 0 {
        limitations.push("Las capturas se enlazan por nombre de archivo y, si el formato lo permite, por tamaño en píxeles. No se interpreta el píxel (eso sigue siendo describe y es sintético).".into());
    }
    Ok(UxReport {
        audit: "market-swarm.ux",
        status: "deterministic".into(),
        source: options.source_name.clone(),
        source_sha256,
        as_of: options.as_of,
        question: options.question.clone(),
        category: options.category.clone(),
        pen_version: design.version,
        stats: built.stats,
        variables: design
            .variables
            .iter()
            .map(|item| VariableInfo {
                name: item.name.clone(),
                kind: item.kind.clone(),
                display: item.display.clone(),
            })
            .collect(),
        palette: built.palette,
        screens: built.screens,
        contrast: built.contrast,
        specialists,
        critic,
        synthesis,
        backlog,
        synthetic_opinions: opinions,
        synthetic_panel_notice: SYNTHETIC_NOTICE.into(),
        sources,
        limitations,
        warnings,
        imported: None,
        neural: None,
        evidence,
    })
}

struct Built {
    screens: Vec<ScreenView>,
    contrast: Vec<ContrastRow>,
    palette: Vec<PaletteEntry>,
    issues: Vec<Issue>,
    stats: Stats,
    dossiers: Vec<String>,
}

fn build(
    design: &Design,
    screen_ids: &[String],
    panel_id: &str,
    competitive: &[Evidence],
    options: &AuditOptions,
) -> Result<Built> {
    let mut screens = Vec::new();
    let mut contrast = Vec::new();
    let mut issues = Vec::new();
    let mut palette: BTreeMap<String, usize> = BTreeMap::new();
    let mut dossiers = Vec::new();
    let mut text_nodes = 0;
    let mut cta_count = 0;
    let mut non_solid = 0;
    let mut paid_total = 0;
    let mut free_total = 0;
    let mut facts = Vec::new();
    for (index, screen) in design.screens.iter().enumerate() {
        let role = role_of(&screen.name);
        let ctas: Vec<&TextNode> = screen.texts.iter().filter(|text| is_cta(text)).collect();
        let backs: Vec<&TextNode> = screen
            .texts
            .iter()
            .filter(|text| is_back(&text.content))
            .collect();
        let mut paid = Vec::new();
        let mut free = Vec::new();
        for text in &screen.texts {
            if let Some(snippet) = paid_price(&text.content) {
                paid.push(snippet);
            }
            if free_mention(&text.content) {
                free.push(truncate(&text.content, 80));
            }
        }
        paid_total += paid.len();
        free_total += free.len();
        text_nodes += screen.texts.len();
        cta_count += ctas.len();
        for text in &screen.texts {
            if text.non_solid {
                non_solid += 1;
                continue;
            }
            let (Some(fg), Some(bg)) = (text.foreground, text.background) else {
                continue;
            };
            *palette.entry(fg.hex()).or_default() += 1;
            *palette.entry(bg.hex()).or_default() += 1;
            let large = contrast::is_large_text(text.font_size, text.font_weight);
            let ratio = contrast::contrast_ratio(opaque(fg), opaque(bg));
            let minimum = contrast::aa_minimum(large);
            let passes = contrast::passes_aa(ratio, large);
            let row = ContrastRow {
                screen_id: screen.id.clone(),
                screen_name: screen.name.clone(),
                node_path: text.node_path.clone(),
                text: truncate(&text.content, 80),
                font_size: text.font_size,
                large_text: large,
                foreground: fg.hex(),
                background: bg.hex(),
                ratio,
                aa_minimum: minimum,
                passes_aa: passes,
            };
            if !passes {
                let on_cta = is_cta(text);
                issues.push(Issue {
                    specialist: "visual_design",
                    kind: "contrast",
                    screen_index: Some(index),
                    also_screens: vec![],
                    node_path: text.node_path.clone(),
                    statement: format!(
                        "Pantalla \"{}\" (frame {}), nodo {}: el texto \"{}\" ({}{} sobre {}{}) mide contraste {:.2}:1, por debajo de AA {:.1}:1{}.",
                        screen.name,
                        screen.id,
                        text.node_path,
                        truncate(&text.content, 60),
                        fg.hex(),
                        color_note(&text.fg_label),
                        bg.hex(),
                        color_note(&text.bg_label),
                        ratio,
                        minimum,
                        if on_cta { " y es un CTA" } else { "" }
                    ),
                    recommendation: format!(
                        "Sube este par a al menos {minimum:.1}:1 cambiando el color del texto o del fondo. El ratio {:.2}:1 ya está medido; no lo reestimes a ojo.",
                        ratio
                    ),
                    interpretation: "observation",
                    impact: if on_cta { 5 } else { 4 },
                    effort: 1,
                    extra_evidence: vec![],
                    evidence_ids: vec![],
                    not_real_users: false,
                    screen_id: String::new(),
                    screen_name: String::new(),
                });
            }
            contrast.push(row);
        }
        if screen.placeholder {
            issues.push(screen_issue(index, screen, "accessibility", "placeholder", 3, 3, "observation",
                format!("Pantalla \"{}\" (frame {}) está marcada placeholder. El contenido visible puede no ser el definitivo.", screen.name, screen.id),
                "Sustituye el marcador por el contenido real o saca la pantalla del flujo hasta que exista.".into()));
        }
        for control in &screen.icon_only {
            issues.push(Issue {
                specialist: "accessibility",
                kind: "icon_only",
                screen_index: Some(index),
                also_screens: vec![],
                node_path: control.node_path.clone(),
                statement: format!(
                    "Pantalla \"{}\" (frame {}), control \"{}\" ({}) tiene icono y ningún texto.",
                    screen.name, screen.id, control.name, control.node_path
                ),
                recommendation: "Añade un rótulo visible o un nombre que diga la acción. Un icono solo no dice qué va a pasar.".into(),
                interpretation: "observation",
                impact: 3,
                effort: 2,
                extra_evidence: vec![],
                evidence_ids: vec![],
                not_real_users: false,
                screen_id: String::new(),
                screen_name: String::new(),
            });
        }
        for control in &screen.controls {
            issues.push(Issue {
                specialist: "accessibility",
                kind: "touch_target",
                screen_index: Some(index),
                also_screens: vec![],
                node_path: control.node_path.clone(),
                statement: format!(
                    "Pantalla \"{}\" (frame {}), control \"{}\" mide {}×{} px, por debajo de 44×44.",
                    screen.name,
                    screen.id,
                    control.name,
                    fmt_px(control.width),
                    fmt_px(control.height)
                ),
                recommendation: "Amplía el área pulsable a 44×44 px como mínimo, aunque el icono siga siendo más pequeño.".into(),
                interpretation: "observation",
                impact: 3,
                effort: 2,
                extra_evidence: vec![],
                evidence_ids: vec![],
                not_real_users: false,
                screen_id: String::new(),
                screen_name: String::new(),
            });
        }
        for text in &screen.texts {
            if text.font_size.is_some_and(|size| size < 12.0) {
                issues.push(Issue {
                    specialist: "accessibility",
                    kind: "small_text",
                    screen_index: Some(index),
                    also_screens: vec![],
                    node_path: text.node_path.clone(),
                    statement: format!(
                        "Pantalla \"{}\" (frame {}), nodo {}: \"{}\" está a {} px.",
                        screen.name,
                        screen.id,
                        text.node_path,
                        truncate(&text.content, 60),
                        fmt_px(text.font_size)
                    ),
                    recommendation: "Sube el cuerpo a 12 px o más. Por debajo de eso la lectura deja de ser cómoda en un teléfono.".into(),
                    interpretation: "observation",
                    impact: 3,
                    effort: 1,
                    extra_evidence: vec![],
                    evidence_ids: vec![],
                    not_real_users: false,
                    screen_id: String::new(),
                    screen_name: String::new(),
                });
            }
            if is_disclaimer(&text.content) && text.font_size.is_some_and(|size| size < 14.0) {
                issues.push(Issue {
                    specialist: "accessibility",
                    kind: "disclaimer",
                    screen_index: Some(index),
                    also_screens: vec![],
                    node_path: text.node_path.clone(),
                    statement: format!(
                        "Pantalla \"{}\" (frame {}), el aviso \"{}\" está a {} px.",
                        screen.name,
                        screen.id,
                        truncate(&text.content, 60),
                        fmt_px(text.font_size)
                    ),
                    recommendation:
                        "Aumenta tamaño y jerarquía del aviso para que se lea sin buscarlo.".into(),
                    interpretation: "observation",
                    impact: 3,
                    effort: 1,
                    extra_evidence: vec![],
                    evidence_ids: vec![],
                    not_real_users: false,
                    screen_id: String::new(),
                    screen_name: String::new(),
                });
            }
        }
        if ctas.is_empty() && screen.width.unwrap_or(440.0) >= 200.0 {
            issues.push(screen_issue(
                index,
                screen,
                "cta_copy",
                "missing_cta",
                3,
                2,
                "observation",
                format!(
                    "Pantalla \"{}\" (frame {}) no tiene un CTA de texto reconocible.",
                    screen.name, screen.id
                ),
                "Define una acción primaria visible. Si la acción es un icono, ponle rótulo."
                    .into(),
            ));
        }
        if !ctas.is_empty() && ctas.iter().all(|text| is_destructive(&text.content)) {
            let labels = ctas
                .iter()
                .map(|text| text.content.as_str())
                .collect::<Vec<_>>()
                .join("; ");
            issues.push(screen_issue(index, screen, "cta_copy", "destructive_cta", 4, 2, "observation",
                format!("Pantalla \"{}\" (frame {}) solo ofrece acciones de salida: {labels}.", screen.name, screen.id),
                "Añade un CTA primario de continuar o guardar y deja la salida como acción secundaria.".into()));
        }
        if let Some(step) = step_number(&screen.name) {
            if backs.is_empty() {
                issues.push(screen_issue(index, screen, "information_architecture", "step_back", 2, 1, "observation",
                    format!("Pantalla \"{}\" (frame {}) parece el paso {step} y no tiene un regreso reconocible (volver, atrás o ←).", screen.name, screen.id),
                    "Añade un control de regreso explícito en este paso.".into()));
            }
        }
        if role == "paywall" && paid.is_empty() {
            issues.push(screen_issue(index, screen, "onboarding_paywall", "paywall_price", 5, 3, "observation",
                format!("Pantalla \"{}\" (frame {}) está clasificada como paywall y no muestra un importe de pago.", screen.name, screen.id),
                "Muestra moneda, importe y periodo en esta pantalla, antes del CTA de pago.".into()));
        }
        dossiers.push(screen_dossier(
            screen,
            &screen_ids[index],
            role,
            &ctas,
            &paid,
            &free,
            &contrast,
        ));
        screens.push(ScreenView {
            id: screen.id.clone(),
            name: screen.name.clone(),
            evidence_id: screen_ids[index].clone(),
            role: role.into(),
            width: screen.width,
            height: screen.height,
            placeholder: screen.placeholder,
            texts: screen
                .texts
                .iter()
                .take(40)
                .map(|text| text.content.clone())
                .collect(),
            ctas: ctas.iter().map(|text| text.content.clone()).collect(),
            price_mentions: paid.clone(),
            free_mentions: free.clone(),
            screenshot: None,
            screenshot_px: None,
        });
        facts.push((role.to_owned(), ctas.len(), paid, free));
    }
    duplicate_names(design, &mut issues);
    duplicate_ctas(design, &mut issues);
    repeated_labels(design, &mut issues);
    let onboarding = facts
        .iter()
        .filter(|(role, _, _, _)| role == "onboarding")
        .count();
    if onboarding >= 7 {
        issues.push(doc_issue("onboarding_paywall", "onboarding_length", 3, 3, "inference",
            format!("El archivo tiene {onboarding} pantallas clasificadas como onboarding."),
            "Recorta el onboarding o permite omitirlo. La fricción es una hipótesis hasta que se mida con personas.".into(),
            (0..design.screens.len()).filter(|i| facts[*i].0 == "onboarding").collect()));
    }
    if design.screens.is_empty() {
        issues.push(doc_issue("information_architecture", "no_screens", 2, 1, "observation",
            "El archivo no tiene pantallas de producto: los componentes reusable:true no se auditan como pantallas.".into(),
            "Pasa un .pen con los frames de producto, no solo la librería de componentes.".into(),
            vec![]));
    } else if paid_total == 0 {
        let paywalls: Vec<usize> = facts
            .iter()
            .enumerate()
            .filter(|(_, (role, _, _, _))| role == "paywall" || role == "pricing")
            .map(|(i, _)| i)
            .collect();
        let free_note = if free_total > 0 {
            format!(" Hay {free_total} mención(es) de gratuidad, y eso no sustituye el precio.")
        } else {
            String::new()
        };
        issues.push(doc_issue("onboarding_paywall", "paid_price", 5, 4, "observation",
            format!("Ningún texto del archivo muestra un precio de pago (moneda o símbolo junto a un importe, o importe con periodo).{free_note}"),
            "Muestra moneda, importe y periodo de cobro antes del CTA de pago.".into(),
            paywalls));
    }
    competitive_issues(
        design,
        competitive,
        &facts,
        options.category.as_deref(),
        &mut issues,
    );
    issues.extend(persona_issues(design, &issues, panel_id));
    annotate_screens(&mut issues, design);
    contrast.sort_by(|a, b| {
        a.passes_aa.cmp(&b.passes_aa).then(
            a.ratio
                .partial_cmp(&b.ratio)
                .unwrap_or(std::cmp::Ordering::Equal),
        )
    });
    if contrast.len() > 300 {
        contrast.truncate(300);
    }
    let mut palette: Vec<PaletteEntry> = palette
        .into_iter()
        .map(|(hex, uses)| PaletteEntry { hex, uses })
        .collect();
    palette.sort_by(|a, b| b.uses.cmp(&a.uses).then(a.hex.cmp(&b.hex)));
    palette.truncate(24);
    let failures = contrast.iter().filter(|row| !row.passes_aa).count();
    Ok(Built {
        stats: Stats {
            screens: design.screens.len(),
            text_nodes,
            ctas: cta_count,
            contrast_measured: contrast.len(),
            contrast_failures: failures,
            paid_price_mentions: paid_total,
            free_mentions: free_total,
            unmeasured_non_solid: non_solid,
            screenshots_linked: 0,
            screenshots_unmatched: 0,
        },
        screens,
        contrast,
        palette,
        issues,
        dossiers,
    })
}

fn persona_issues(design: &Design, issues: &[Issue], panel_id: &str) -> Vec<Issue> {
    let rules = [
        (
            "Sofía",
            "principiante en meditación, necesita un siguiente paso obvio",
            &["missing_cta", "onboarding_length", "small_text"][..],
        ),
        (
            "Carlos",
            "sensible al precio, no decide un pago sin importe y periodo",
            &["paid_price", "paywall_price"][..],
        ),
        (
            "Miguel",
            "busca calma y se frena con salidas o avisos escondidos",
            &["destructive_cta", "disclaimer"][..],
        ),
        (
            "Raúl",
            "poca familiaridad con apps, necesita texto grande y botones claros",
            &["small_text", "contrast", "icon_only", "touch_target"][..],
        ),
        (
            "Valentina",
            "diseñadora, mira jerarquía y CTAs que parecen la misma acción",
            &["duplicate_cta", "contrast"][..],
        ),
    ];
    let mut additions = Vec::new();
    for (name, description, kinds) in rules {
        let Some(trigger) = issues
            .iter()
            .filter(|issue| kinds.contains(&issue.kind))
            .max_by_key(|issue| (issue.impact, usize::MAX - issue.effort as usize))
        else {
            continue;
        };
        let screen_name = trigger
            .screen_index
            .and_then(|i| design.screens.get(i).map(|s| s.name.clone()))
            .unwrap_or_else(|| "el archivo".into());
        additions.push(Issue {
            specialist: "synthetic_panel",
            kind: "persona",
            screen_index: trigger.screen_index,
            also_screens: trigger.also_screens.clone(),
            node_path: trigger.node_path.clone(),
            statement: format!(
                "Persona sintética {name} ({description}; no es un usuario real) reacciona a \"{screen_name}\": {}.",
                trigger.statement.chars().take(180).collect::<String>()
            ),
            recommendation: "Usa esta reacción solo como hipótesis de copy. No la cites como voz de un usuario ni como disposición a pagar.".into(),
            interpretation: "simulation",
            impact: 3,
            effort: 2,
            extra_evidence: vec![panel_id.to_owned()],
            evidence_ids: vec![],
            not_real_users: true,
            screen_id: String::new(),
            screen_name: screen_name.clone(),
        });
    }
    additions
}

fn annotate_screens(issues: &mut [Issue], design: &Design) {
    for issue in issues {
        let index = issue
            .screen_index
            .or_else(|| issue.also_screens.first().copied());
        if let Some(index) = index {
            if let Some(screen) = design.screens.get(index) {
                issue.screen_id = screen.id.clone();
                if issue.screen_name.is_empty() {
                    issue.screen_name = screen.name.clone();
                }
            }
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn screen_issue(
    index: usize,
    screen: &Screen,
    specialist: &'static str,
    kind: &'static str,
    impact: u8,
    effort: u8,
    interpretation: &'static str,
    statement: String,
    recommendation: String,
) -> Issue {
    Issue {
        specialist,
        kind,
        screen_index: Some(index),
        also_screens: vec![],
        node_path: screen.id.clone(),
        statement,
        recommendation,
        interpretation,
        impact,
        effort,
        extra_evidence: vec![],
        evidence_ids: vec![],
        not_real_users: false,
        screen_id: String::new(),
        screen_name: String::new(),
    }
}

#[allow(clippy::too_many_arguments)]
fn doc_issue(
    specialist: &'static str,
    kind: &'static str,
    impact: u8,
    effort: u8,
    interpretation: &'static str,
    statement: String,
    recommendation: String,
    also_screens: Vec<usize>,
) -> Issue {
    Issue {
        specialist,
        kind,
        screen_index: None,
        also_screens,
        node_path: String::new(),
        statement,
        recommendation,
        interpretation,
        impact,
        effort,
        extra_evidence: vec![],
        evidence_ids: vec![],
        not_real_users: false,
        screen_id: String::new(),
        screen_name: String::new(),
    }
}

fn duplicate_names(design: &Design, issues: &mut Vec<Issue>) {
    let mut groups: BTreeMap<String, Vec<usize>> = BTreeMap::new();
    for (index, screen) in design.screens.iter().enumerate() {
        groups
            .entry(screen.name.to_lowercase())
            .or_default()
            .push(index);
    }
    for (name, indexes) in groups {
        if indexes.len() < 2 {
            continue;
        }
        let ids = indexes
            .iter()
            .filter_map(|i| design.screens.get(*i).map(|s| s.id.as_str()))
            .collect::<Vec<_>>()
            .join(", ");
        issues.push(doc_issue(
            "information_architecture",
            "duplicate_name",
            2,
            1,
            "observation",
            format!(
                "El nombre \"{name}\" aparece en {} pantallas (frames {ids}).",
                indexes.len()
            ),
            "Distingue los frames con nombres únicos para poder citarlos y navegarlos.".into(),
            indexes,
        ));
    }
}

fn duplicate_ctas(design: &Design, issues: &mut Vec<Issue>) {
    let mut rows = Vec::new();
    for (index, screen) in design.screens.iter().enumerate() {
        for text in &screen.texts {
            if is_cta(text) {
                rows.push((
                    index,
                    norm(&text.content),
                    text.content.clone(),
                    text.node_path.clone(),
                ));
            }
        }
    }
    let mut emitted = 0;
    for i in 0..rows.len() {
        for j in (i + 1)..rows.len() {
            if emitted >= 8 {
                return;
            }
            let same_screen = rows[i].0 == rows[j].0;
            if !same_screen && rows[i].1 == rows[j].1 {
                continue;
            }
            if !near(&rows[i].1, &rows[j].1) {
                continue;
            }
            let screen = &design.screens[rows[i].0];
            let other = &design.screens[rows[j].0];
            issues.push(Issue {
                specialist: "cta_copy",
                kind: "duplicate_cta",
                screen_index: Some(rows[i].0),
                also_screens: if same_screen { vec![] } else { vec![rows[j].0] },
                node_path: rows[i].3.clone(),
                statement: format!(
                    "CTAs demasiado parecidos: \"{}\" en \"{}\" ({}) y \"{}\" en \"{}\" ({}).",
                    rows[i].2, screen.name, rows[i].3, rows[j].2, other.name, rows[j].3
                ),
                recommendation: "Deja un verbo primario y diferencia la acción secundaria para que no parezcan la misma.".into(),
                interpretation: "observation",
                impact: if same_screen { 4 } else { 3 },
                effort: 2,
                extra_evidence: vec![],
                evidence_ids: vec![],
                not_real_users: false,
                screen_id: String::new(),
                screen_name: String::new(),
            });
            emitted += 1;
        }
    }
}

fn repeated_labels(design: &Design, issues: &mut Vec<Issue>) {
    let mut counts: BTreeMap<String, Vec<usize>> = BTreeMap::new();
    for (index, screen) in design.screens.iter().enumerate() {
        let mut seen = BTreeSet::new();
        for text in &screen.texts {
            let key = norm(&text.content);
            let len = key.chars().count();
            if (2..=14).contains(&len) && seen.insert(key.clone()) {
                counts.entry(key).or_default().push(index);
            }
        }
    }
    let mut labels: Vec<(&String, &Vec<usize>)> = counts
        .iter()
        .filter(|(_, screens)| screens.len() >= 3)
        .collect();
    labels.sort_by(|a, b| b.1.len().cmp(&a.1.len()).then(a.0.cmp(b.0)));
    if labels.len() < 2 {
        return;
    }
    labels.truncate(8);
    let shown = labels
        .iter()
        .map(|(label, _)| label.as_str())
        .collect::<Vec<_>>()
        .join(", ");
    let mut screens_hit = BTreeSet::new();
    for (_, indexes) in &labels {
        screens_hit.extend(indexes.iter().copied());
    }
    issues.push(doc_issue(
        "information_architecture",
        "nav",
        2,
        1,
        "inference",
        format!("Las etiquetas cortas [{shown}] se repiten en al menos 3 pantallas. Pueden ser navegación persistente; el archivo no lo declara."),
        "Confirma si son la navegación y si la sección activa se distingue por más que el color.".into(),
        screens_hit.into_iter().take(8).collect(),
    ));
}

fn competitive_issues(
    design: &Design,
    competitive: &[Evidence],
    facts: &[(String, usize, Vec<String>, Vec<String>)],
    category: Option<&str>,
    issues: &mut Vec<Issue>,
) {
    if competitive.is_empty() {
        issues.push(doc_issue(
            "competitive_benchmark",
            "competitive_gap",
            2,
            3,
            "observation",
            NO_COMPETITIVE_EVIDENCE.into(),
            "Trae fuentes con fetch o search y repite la auditoría citando esos ids.".into(),
            vec![],
        ));
        return;
    }
    let onboarding = facts
        .iter()
        .filter(|(role, _, _, _)| role == "onboarding")
        .count();
    let paid: usize = facts.iter().map(|(_, _, paid, _)| paid.len()).sum();
    let free: usize = facts.iter().map(|(_, _, _, free)| free.len()).sum();
    let has_exit = design.screens.iter().any(|screen| {
        screen
            .texts
            .iter()
            .any(|text| is_destructive(&text.content))
    });
    let dims = [
        (
            "precio",
            &[
                "price",
                "pricing",
                "paywall",
                "precio",
                "billing",
                "subscription",
            ][..],
            paid == 0 || facts.iter().any(|(r, _, _, _)| r == "paywall"),
            if paid == 0 {
                "ningún texto muestra un importe de pago".to_owned()
            } else {
                format!(
                    "hay {} pantalla(s) de paywall",
                    facts.iter().filter(|(r, _, _, _)| r == "paywall").count()
                )
            },
        ),
        (
            "prueba y renovación",
            &["trial", "free trial", "prueba", "renew", "renov"],
            free > 0 || paid == 0,
            format!("el archivo tiene {free} mención(es) de gratuidad y {paid} de precio de pago"),
        ),
        (
            "onboarding",
            &["onboarding", "activation", "primeros pasos"],
            onboarding > 0,
            format!("hay {onboarding} pantalla(s) clasificadas como onboarding"),
        ),
        (
            "cancelación",
            &["cancel", "restore", "reembolso", "cancelación"],
            has_exit,
            "hay textos de salida, cancelar o terminar".to_owned(),
        ),
    ];
    let mut hits = 0;
    for evidence in competitive {
        for (_dimension, terms, applies, fact) in &dims {
            if !*applies {
                continue;
            }
            let Some(excerpt) = excerpt(&evidence.content, terms) else {
                continue;
            };
            if hits >= 6 {
                return;
            }
            let synthetic = if evidence.synthetic_fixture {
                " La fuente está marcada synthetic_fixture."
            } else {
                ""
            };
            let category_note = category
                .map(|c| format!(" Categoría pedida: {c}."))
                .unwrap_or_default();
            issues.push(Issue {
                specialist: "competitive_benchmark",
                kind: "competitive",
                screen_index: None,
                also_screens: vec![],
                node_path: String::new(),
                statement: format!(
                    "Inferencia, no un ranking medido por este motor. Fuente {}: \"{excerpt}\". En el diseño, {fact}.{synthetic}{category_note}",
                    evidence.id
                ),
                recommendation: "Cambia el diseño solo donde el archivo y la fuente citada dicen cosas distintas. No copies una cifra que la fuente no trae.".into(),
                interpretation: "inference",
                impact: 4,
                effort: 3,
                extra_evidence: vec![evidence.id.clone()],
                evidence_ids: vec![],
                not_real_users: false,
                screen_id: String::new(),
                screen_name: String::new(),
            });
            hits += 1;
        }
    }
    if hits == 0 {
        issues.push(doc_issue(
            "competitive_benchmark",
            "competitive_gap",
            2,
            3,
            "observation",
            "La evidencia citada no contiene fragmentos reconocibles sobre precio, prueba, onboarding o cancelación. No se redactó una comparación.".into(),
            "Importa un extracto que hable de esas dimensiones o no pidas un benchmark con esta fuente.".into(),
            vec![],
        ));
    }
}

fn link_screenshots(
    built: &mut Built,
    design: &Design,
    options: &AuditOptions,
    warnings: &mut Vec<String>,
) {
    let mut unmatched = 0;
    for shot in &options.screenshots {
        if shot.bytes.len() > 8_000_000 {
            warnings.push(format!("Skipped oversized screenshot {}.", shot.path));
            unmatched += 1;
            continue;
        }
        let stem = std::path::Path::new(&shot.path)
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("")
            .to_lowercase();
        let mut best: Option<(usize, u8)> = None;
        for (index, screen) in design.screens.iter().enumerate() {
            let id = screen.id.to_lowercase();
            let slug = slug(&screen.name);
            let score = if stem == id {
                3
            } else if id.len() >= 3 && stem.contains(&id) {
                2
            } else if slug.len() >= 4 && stem.contains(&slug) {
                1
            } else {
                0
            };
            if score > best.map(|(_, s)| s).unwrap_or(0) {
                best = Some((index, score));
            }
        }
        let Some((index, _)) = best else {
            unmatched += 1;
            continue;
        };
        let px = image_dimensions(&shot.bytes);
        built.screens[index].screenshot = Some(shot.path.clone());
        if let Some((w, h)) = px {
            built.screens[index].screenshot_px = Some([w, h]);
            let screen = &design.screens[index];
            if let (Some(fw), Some(fh)) = (screen.width, screen.height) {
                let width_ok =
                    (1..=4).any(|scale| (f64::from(w) - fw * f64::from(scale)).abs() <= 1.5);
                let height_ok =
                    (1..=4).any(|scale| (f64::from(h) - fh * f64::from(scale)).abs() <= 1.5);
                let same_scale = (1..=4).any(|scale| {
                    (f64::from(w) - fw * f64::from(scale)).abs() <= 1.5
                        && (f64::from(h) - fh * f64::from(scale)).abs() <= 1.5
                });
                if width_ok && height_ok && !same_scale || (!width_ok || !height_ok) {
                    built.issues.push(Issue {
                        specialist: "visual_design",
                        kind: "screenshot_mismatch",
                        screen_index: Some(index),
                        also_screens: vec![],
                        node_path: screen.id.clone(),
                        statement: format!(
                            "La captura {} mide {w}×{h} px y el frame \"{}\" mide {}×{}. No es el mismo tamaño ni un múltiplo entero 1..4.",
                            shot.path,
                            screen.name,
                            fmt_px(screen.width),
                            fmt_px(screen.height)
                        ),
                        recommendation: "Vuelve a exportar el PNG a la escala del frame antes de usarlo como evidencia visual.".into(),
                        interpretation: "inference",
                        impact: 2,
                        effort: 2,
                        extra_evidence: vec![],
                        evidence_ids: vec![],
                        not_real_users: false,
                        screen_id: String::new(),
                        screen_name: String::new(),
                    });
                }
            }
        }
        built.stats.screenshots_linked += 1;
    }
    built.stats.screenshots_unmatched = unmatched;
}

fn evidence_records(
    design: &Design,
    built: &Built,
    screen_ids: &[String],
    document_id: &str,
    panel_id: &str,
    options: &AuditOptions,
    sha: &str,
) -> Vec<Evidence> {
    let source = if options.source_name.trim().is_empty() {
        "pen:memory".to_owned()
    } else {
        options.source_name.clone()
    };
    let design_limits = vec![
        "Extracto determinista de un archivo .pen. No es un resultado de mercado, no es un encuestado humano y no es una descripción de un modelo.".into(),
        "El contraste usa rellenos sólidos y luminancia relativa WCAG 2.x. Degradados, imágenes, shaders y modos de mezcla no se miden.".into(),
        "observed_at es el corte as_of de la auditoría, no una fecha de publicación del diseño.".into(),
    ];
    let mut records = vec![record(
        document_id,
        &source,
        "pen_design",
        false,
        options,
        document_dossier(design, built, sha),
        &design_limits,
    )];
    for (index, dossier) in built.dossiers.iter().enumerate() {
        records.push(record(
            &screen_ids[index],
            &source,
            "pen_design",
            false,
            options,
            dossier.clone(),
            &design_limits,
        ));
    }
    records.push(record(
        panel_id,
        &source,
        "synthetic_panel",
        true,
        options,
        format!(
            "{SYNTHETIC_NOTICE}\nPersonas regla: Sofía (principiante), Carlos (precio), Miguel (calma), Raúl (legibilidad), Valentina (consistencia).\nCada opinión debe citar este id y una pantalla. No es WTP ni una encuesta."
        ),
        &[
            "Personas sintéticas de reglas. No son usuarios reales.".into(),
            "No miden intención de descarga ni disposición a pagar.".into(),
        ],
    ));
    records
}

fn record(
    id: &str,
    source: &str,
    provider: &str,
    synthetic: bool,
    options: &AuditOptions,
    content: String,
    limitations: &[String],
) -> Evidence {
    Evidence {
        id: id.to_owned(),
        source: source.to_owned(),
        provider: provider.to_owned(),
        kind: EvidenceKind::Simulated,
        published_at: None,
        collected_at: options.as_of,
        observed_at: options.as_of,
        segment: options.category.clone(),
        population: None,
        content: truncate(&content, 90_000),
        limitations: limitations.to_vec(),
        synthetic_fixture: synthetic,
    }
}

fn document_dossier(design: &Design, built: &Built, sha: &str) -> String {
    let mut lines = vec![
        format!("Inventario .pen sha256 {sha}"),
        format!("version {}", design.version),
    ];
    for screen in &built.screens {
        lines.push(format!(
            "- {} | {} | rol {} | evidencia {}",
            screen.id, screen.name, screen.role, screen.evidence_id
        ));
    }
    for note in &design.designer_notes {
        lines.push(format!(
            "Nota de diseño (no es UI): {}",
            truncate(note, 200)
        ));
    }
    lines.join("\n")
}

fn screen_dossier(
    screen: &Screen,
    evidence_id: &str,
    role: &str,
    ctas: &[&TextNode],
    paid: &[String],
    free: &[String],
    contrast: &[ContrastRow],
) -> String {
    let mut lines = vec![
        format!("Pantalla: {}", screen.name),
        format!("Frame: {}", screen.id),
        format!("Evidencia: {evidence_id}"),
        format!("Rol heurístico: {role}"),
        format!("Tamaño: {}×{}", fmt_px(screen.width), fmt_px(screen.height)),
        format!(
            "Placeholder: {}",
            if screen.placeholder { "sí" } else { "no" }
        ),
        "Textos:".into(),
    ];
    for text in screen.texts.iter().take(80) {
        lines.push(format!("- {}", text.content));
    }
    lines.push("CTAs heurísticos:".into());
    for text in ctas {
        lines.push(format!("- {}", text.content));
    }
    lines.push(format!(
        "Precios de pago: {}",
        if paid.is_empty() {
            "ninguno".into()
        } else {
            paid.join("; ")
        }
    ));
    lines.push(format!(
        "Gratuidad: {}",
        if free.is_empty() {
            "ninguna".into()
        } else {
            free.join("; ")
        }
    ));
    lines.push("Contraste medido:".into());
    for row in contrast
        .iter()
        .filter(|row| row.screen_id == screen.id)
        .take(40)
    {
        lines.push(format!(
            "- {} \"{}\" {} sobre {} {:.2}:1 AA {} cumple {}",
            row.node_path,
            row.text,
            row.foreground,
            row.background,
            row.ratio,
            row.aa_minimum,
            row.passes_aa
        ));
    }
    lines.join("\n")
}

fn specialist_output(
    role: &str,
    issues: &[Issue],
    allowed: &BTreeSet<String>,
) -> Result<AgentOutput> {
    let mut mine: Vec<&Issue> = issues
        .iter()
        .filter(|issue| issue.specialist == role)
        .collect();
    sort_issues(&mut mine);
    let total = mine.len();
    let shown = mine.into_iter().take(8).collect::<Vec<_>>();
    let mut gaps = Vec::new();
    if total > shown.len() {
        gaps.push(format!("Se omiten {} hallazgos de {role} en el agente; el backlog conserva los de mayor prioridad.", total - shown.len()));
    }
    if role == "competitive_benchmark" && shown.iter().any(|issue| issue.kind == "competitive_gap")
    {
        gaps.push(NO_COMPETITIVE_EVIDENCE.into());
    }
    if role == "synthetic_panel" {
        gaps.push(SYNTHETIC_NOTICE.into());
    }
    if role == "visual_design" && issues.iter().any(|issue| issue.kind == "contrast") {
        gaps.push(
            "Los ratios están en el arreglo contrast. Un modelo no debe sustituirlos.".into(),
        );
    }
    let summary = match role {
        "visual_design" => "Paleta y contraste calculados con luminancia relativa WCAG 2.x. Los fallos de abajo son mediciones, no impresiones.".into(),
        "information_architecture" => "Mapa de pantallas, nombres, etiquetas repetidas y pasos. La navegación repetida es una inferencia.".into(),
        "cta_copy" => "CTAs detectados por verbo o por nombre de control. Los duplicados son observación del texto, no una prueba de clics.".into(),
        "accessibility" => "Hallazgos de tamaño, área táctil, iconos sin rótulo y avisos. El contraste de texto vive en visual_design.".into(),
        "onboarding_paywall" => "Precio y paywall leídos del texto del archivo. Gratis no cuenta como precio de pago.".into(),
        "competitive_benchmark" => "Comparación solo con evidencia citada. No es un ranking de descargas ni de apps líderes.".into(),
        "synthetic_panel" => SYNTHETIC_NOTICE.into(),
        _ => "Especialista de auditoría UX.".into(),
    };
    let findings = shown
        .iter()
        .map(|issue| Finding {
            statement: format!("{} Cambio: {}", issue.statement, issue.recommendation),
            evidence_ids: issue.evidence_ids.clone(),
            interpretation: issue.interpretation.into(),
        })
        .collect();
    let output = AgentOutput {
        summary,
        findings,
        hypotheses: vec![],
        gaps,
        next_experiments: vec![],
    };
    output.validate(allowed)?;
    Ok(output)
}

fn critic_output(
    built: &Built,
    backlog: &[BacklogItem],
    opinions: &[BacklogItem],
    document_id: &str,
    panel_id: &str,
    allowed: &BTreeSet<String>,
) -> Result<AgentOutput> {
    let mut gaps = Vec::new();
    for issue in &built.issues {
        if issue.evidence_ids.is_empty() {
            gaps.push(format!("Hallazgo sin cita: {}.", issue.kind));
        }
        if issue.interpretation == "simulation"
            && !issue.evidence_ids.iter().any(|id| id == panel_id)
        {
            gaps.push("Una opinión sintética no cita el id del panel.".into());
        }
        if issue.specialist == "competitive_benchmark"
            && issue.kind == "competitive"
            && !issue.evidence_ids.iter().any(|id| !id.starts_with("pen-"))
        {
            gaps.push("Una comparación competitiva no cita evidencia externa.".into());
        }
    }
    if backlog
        .iter()
        .any(|item| item.interpretation == "simulation" || item.not_real_users)
    {
        gaps.push("El backlog de diseño incluye una opinión de persona sintética.".into());
    }
    if opinions
        .iter()
        .any(|item| item.interpretation != "simulation" || !item.not_real_users)
    {
        gaps.push("Una opinión sintética no está etiquetada como simulación.".into());
    }
    for row in &built.contrast {
        let (Some(fg), Some(bg)) = (
            contrast::parse_hex(&row.foreground),
            contrast::parse_hex(&row.background),
        ) else {
            gaps.push(format!("Color ilegible en {}.", row.node_path));
            continue;
        };
        let ratio = contrast::contrast_ratio(fg, bg);
        if (ratio - row.ratio).abs() > 1e-6
            || contrast::passes_aa(ratio, row.large_text) != row.passes_aa
        {
            gaps.push(format!("El contraste no coincide en {}.", row.node_path));
        }
    }
    let findings = if gaps.is_empty() {
        vec![Finding {
            statement: "El crítico recomprobó que cada hallazgo cita evidencia, que los ratios WCAG coinciden con el cálculo y que las personas sintéticas no entraron al backlog de diseño.".into(),
            evidence_ids: vec![document_id.to_owned()],
            interpretation: "observation".into(),
        }]
    } else {
        vec![Finding {
            statement: "El crítico encontró inconsistencias internas. No uses el informe hasta revisar los huecos.".into(),
            evidence_ids: vec![document_id.to_owned()],
            interpretation: "observation".into(),
        }]
    };
    let output = AgentOutput {
        summary: if gaps.is_empty() {
            "La auditoría determinista es consistente: citas, contraste y separación simulación/observación.".into()
        } else {
            "La auditoría determinista tiene inconsistencias internas.".into()
        },
        findings,
        hypotheses: vec![],
        gaps,
        next_experiments: vec![],
    };
    output.validate(allowed)?;
    Ok(output)
}

fn synthesis_output(
    backlog: &[BacklogItem],
    stats: &Stats,
    document_id: &str,
    options: &AuditOptions,
    allowed: &BTreeSet<String>,
) -> Result<AgentOutput> {
    let mut counts = BTreeMap::new();
    for item in backlog {
        *counts.entry(item.band.as_str()).or_insert(0) += 1;
    }
    let summary = format!(
        "Prioridad para cambiar el diseño (impacto contra esfuerzo). Hacer ahora: {}. Planificar: {}. Pulir: {}. Diferir: {}. Pantallas {} , textos {} , CTAs {} , contrastes medidos {} con {} fallos AA, precios de pago {}.",
        counts.get("do_now").copied().unwrap_or(0),
        counts.get("plan").copied().unwrap_or(0),
        counts.get("polish").copied().unwrap_or(0),
        counts.get("defer").copied().unwrap_or(0),
        stats.screens,
        stats.text_nodes,
        stats.ctas,
        stats.contrast_measured,
        stats.contrast_failures,
        stats.paid_price_mentions
    );
    let findings = if backlog.is_empty() {
        vec![Finding {
            statement: "No hay cambios priorizados. Las reglas deterministas no dispararon un hallazgo accionable en este archivo.".into(),
            evidence_ids: vec![document_id.to_owned()],
            interpretation: "observation".into(),
        }]
    } else {
        backlog
            .iter()
            .take(5)
            .map(|item| Finding {
                statement: format!(
                    "[{} · impacto {} · esfuerzo {}] {} ({}): {} Cambio: {}",
                    band_label(&item.band),
                    item.impact,
                    item.effort,
                    if item.screen_name.is_empty() {
                        "Archivo"
                    } else {
                        item.screen_name.as_str()
                    },
                    if item.screen_id.is_empty() {
                        "documento"
                    } else {
                        item.screen_id.as_str()
                    },
                    item.statement,
                    item.recommendation
                ),
                evidence_ids: item.evidence_ids.clone(),
                interpretation: item.interpretation.clone(),
            })
            .collect()
    };
    let category = options
        .category
        .as_deref()
        .unwrap_or("la categoría indicada");
    let output = AgentOutput {
        summary,
        findings,
        hypotheses: vec!["Un CTA primario único y un precio con moneda y periodo, visibles antes de pagar, reducirían la ambigüedad. Es una hipótesis de diseño, no un resultado medido.".into()],
        gaps: vec![
            "Los rellenos no sólidos no tienen ratio.".into(),
            SYNTHETIC_NOTICE.into(),
        ],
        next_experiments: vec![
            "Prueba moderada con personas del segmento: ¿entienden el precio y el CTA primario en las pantallas de «hacer ahora»?".into(),
            format!("Importar con fetch o search evidencia de {category} y repetir la auditoría para que el benchmark competitivo tenga cita."),
            "Exportar PNG solo para los rellenos no sólidos; describe sigue siendo una lectura sintética y no sustituye el contraste medido.".into(),
        ],
    };
    output.validate(allowed)?;
    Ok(output)
}

fn priority_lists(issues: &[Issue]) -> (Vec<BacklogItem>, Vec<BacklogItem>) {
    let mut ranked: Vec<&Issue> = issues.iter().collect();
    sort_issues(&mut ranked);
    let mut backlog = Vec::new();
    let mut opinions = Vec::new();
    let mut seen = BTreeSet::new();
    for issue in ranked {
        let item = to_item(
            issue,
            if issue.not_real_users {
                opinions.len()
            } else {
                backlog.len()
            },
        );
        if issue.not_real_users || issue.interpretation == "simulation" {
            opinions.push(item);
            continue;
        }
        let key = format!(
            "{}|{}|{}",
            issue.kind,
            issue
                .screen_index
                .map(|i| i.to_string())
                .unwrap_or_default(),
            issue.node_path
        );
        if !seen.insert(key) {
            continue;
        }
        if backlog.len() < 40 {
            backlog.push(item);
        }
    }
    (backlog, opinions)
}

fn to_item(issue: &Issue, index: usize) -> BacklogItem {
    BacklogItem {
        id: format!("ux-{index:02}-{}", issue.kind),
        specialist: issue.specialist.into(),
        kind: issue.kind.into(),
        band: band(issue.impact, issue.effort).into(),
        impact: issue.impact,
        effort: issue.effort,
        screen_id: issue.screen_id.clone(),
        screen_name: issue.screen_name.clone(),
        node_path: issue.node_path.clone(),
        statement: issue.statement.clone(),
        recommendation: issue.recommendation.clone(),
        interpretation: issue.interpretation.into(),
        evidence_ids: issue.evidence_ids.clone(),
        not_real_users: issue.not_real_users,
    }
}

fn sort_issues(issues: &mut [&Issue]) {
    issues.sort_by(|a, b| {
        band_rank(band(a.impact, a.effort))
            .cmp(&band_rank(band(b.impact, b.effort)))
            .then(b.impact.cmp(&a.impact))
            .then(a.effort.cmp(&b.effort))
            .then(a.kind.cmp(b.kind))
            .then(a.node_path.cmp(&b.node_path))
    });
}

fn agent(role: &str, output: AgentOutput) -> Result<AgentResult> {
    let body = serde_json::to_string(&output)?;
    Ok(AgentResult {
        role: role.into(),
        model: "deterministic".into(),
        status: "completed".into(),
        output: Some(output),
        error: None,
        usage: json!({"deterministic": true}),
        prompt_hash: crate::digest(body.as_bytes()),
    })
}

fn bind_evidence(issue: &mut Issue, screen_ids: &[String], document_id: &str) {
    let mut ids = Vec::new();
    if let Some(index) = issue.screen_index {
        if let Some(id) = screen_ids.get(index) {
            ids.push(id.clone());
        }
    } else {
        ids.push(document_id.to_owned());
    }
    for index in &issue.also_screens {
        if let Some(id) = screen_ids.get(*index) {
            ids.push(id.clone());
        }
    }
    ids.extend(issue.extra_evidence.iter().cloned());
    if ids.is_empty() {
        ids.push(document_id.to_owned());
    }
    ids.sort();
    ids.dedup();
    issue.evidence_ids = ids;
}

fn interpretation_label(kind: &str) -> &str {
    match kind {
        "observation" => "Observación",
        "inference" => "Inferencia",
        "simulation" => "Simulación (no son usuarios reales)",
        other => other,
    }
}

pub fn render(report: &UxReport) -> String {
    let mut out = format!(
        "Auditoría UX {}\nEstado: {}\nPregunta: {}\nCorte: {}\nArchivo: {}\nSHA256: {}\n\nAVISO\n{}\n\nRESUMEN\nPantallas: {}\nTextos: {}\nCTAs: {}\nContraste medido: {} (fallos AA: {})\nPrecios de pago: {}\nMenciones de gratuidad: {}\nRellenos no sólidos sin medir: {}\nCapturas enlazadas: {}\n\n",
        report.audit,
        report.status,
        report.question,
        report.as_of,
        report.source,
        report.source_sha256,
        report.synthetic_panel_notice,
        report.stats.screens,
        report.stats.text_nodes,
        report.stats.ctas,
        report.stats.contrast_measured,
        report.stats.contrast_failures,
        report.stats.paid_price_mentions,
        report.stats.free_mentions,
        report.stats.unmeasured_non_solid,
        report.stats.screenshots_linked
    );
    for (band_id, title) in [
        ("do_now", "HACER AHORA"),
        ("plan", "PLANIFICAR"),
        ("polish", "PULIR"),
        ("defer", "DIFERIR"),
    ] {
        out.push_str(title);
        out.push('\n');
        let items: Vec<&BacklogItem> = report
            .backlog
            .iter()
            .filter(|item| item.band == band_id)
            .collect();
        if items.is_empty() {
            out.push_str("- Nada en este bloque.\n");
        }
        for item in items {
            out.push_str(&format!(
                "- [impacto {} · esfuerzo {}] {} (frame {}) nodo {}\n  {}: {}\n  Cambio: {}\n  Evidencia: {} · tipo: {}\n",
                item.impact,
                item.effort,
                if item.screen_name.is_empty() { "Archivo".into() } else { item.screen_name.clone() },
                if item.screen_id.is_empty() { "documento" } else { item.screen_id.as_str() },
                if item.node_path.is_empty() { "-" } else { item.node_path.as_str() },
                interpretation_label(&item.interpretation),
                item.statement,
                item.recommendation,
                item.evidence_ids.join(", "),
                item.interpretation
            ));
        }
        out.push('\n');
    }
    out.push_str("OPINIONES DE PERSONAS SINTÉTICAS\nNo son usuarios reales.\n");
    if report.synthetic_opinions.is_empty() {
        out.push_str("- Ninguna regla del panel se activó.\n");
    }
    for item in &report.synthetic_opinions {
        out.push_str(&format!(
            "- {}\n  Evidencia: {}\n",
            item.statement,
            item.evidence_ids.join(", ")
        ));
    }
    out.push_str("\nSÍNTESIS\n");
    if let Some(output) = &report.synthesis.output {
        out.push_str(&output.summary);
        out.push('\n');
    }
    out.push_str("\nLÍMITES\n");
    for line in &report.limitations {
        out.push_str(&format!("- {line}\n"));
    }
    out
}

pub async fn enrich(report: &mut UxReport, inference: &Inference, store: &Store) -> Result<()> {
    let limits = Limits {
        max_output_tokens: 4096,
        reasoning_enabled: Some(false),
        max_cost_usd: 1.0,
        ..Limits::default()
    };
    limits.validate()?;
    let allowed: BTreeSet<String> = report
        .sources
        .iter()
        .map(|source| source.id.clone())
        .collect();
    let dossier = compact_dossier(report);
    let run = format!(
        "uxn-{}-{}",
        &crate::digest(
            format!(
                "{}|{}|{}",
                report.source_sha256, inference.model, report.as_of
            )
            .as_bytes()
        )[..16],
        Utc::now().timestamp_millis()
    );
    store.start(&run, &truncate(&dossier, 4000))?;
    let critic = neural_role(
        inference,
        store,
        &run,
        "ux_critic",
        &dossier,
        &limits,
        &allowed,
    )
    .await;
    let synthesis_payload = json!({
        "dossier": dossier,
        "deterministic_synthesis": report.synthesis.output,
        "neural_critic": critic.output,
        "evidence_ids": allowed.iter().collect::<Vec<_>>()
    });
    let synthesis = neural_role_payload(
        inference,
        store,
        &run,
        "ux_synthesis",
        &synthesis_payload,
        &limits,
        &allowed,
    )
    .await;
    let status = if critic.status == "completed" && synthesis.status == "completed" {
        "completed"
    } else {
        "partial"
    };
    report.status = if status == "completed" {
        "deterministic_plus_neural".into()
    } else {
        "deterministic_neural_partial".into()
    };
    report.neural = Some(NeuralPass {
        status: status.into(),
        critic,
        synthesis,
    });
    Ok(())
}

async fn neural_role(
    inference: &Inference,
    store: &Store,
    run: &str,
    role: &str,
    dossier: &str,
    limits: &Limits,
    allowed: &BTreeSet<String>,
) -> AgentResult {
    let payload = json!({"dossier": dossier, "evidence_ids": allowed.iter().collect::<Vec<_>>()});
    neural_role_payload(inference, store, run, role, &payload, limits, allowed).await
}

async fn neural_role_payload(
    inference: &Inference,
    store: &Store,
    run: &str,
    role: &str,
    payload: &Value,
    limits: &Limits,
    allowed: &BTreeSet<String>,
) -> AgentResult {
    let system = Inference::system(role, &neural_brief(role));
    let prompt_hash = crate::digest(format!("{system}\n{payload}").as_bytes());
    let mut result = AgentResult {
        role: role.into(),
        model: inference.model.clone(),
        status: "failed".into(),
        output: None,
        error: None,
        usage: json!({}),
        prompt_hash,
    };
    match inference
        .complete(&system, payload, limits, store, run)
        .await
    {
        Ok((output, usage)) => {
            result.usage = usage;
            match output.validate(allowed) {
                Ok(()) => {
                    result.status = "completed".into();
                    result.output = Some(output);
                }
                Err(_) => {
                    result.error = Some(
                        "Neural output failed evidence validation; deterministic audit stands"
                            .into(),
                    )
                }
            }
        }
        Err(_) => result.error = Some("Neural pass failed; deterministic audit stands".into()),
    }
    let _ = store.save_task(run, &result);
    result
}

fn neural_brief(role: &str) -> String {
    format!(
        "You are reviewing a deterministic UX audit, not replacing it. Role: {role}. \
Contrast ratios in the dossier are authoritative WCAG measurements; do not invent a different ratio. \
Synthetic persona opinions are not real users and are not willingness to pay; if you repeat one, its interpretation MUST be simulation. \
Competitive comparisons must cite the supplied external evidence ids and must not add download counts, star ratings or best-seller ranks that are absent from those excerpts. \
Cite only supplied evidence IDs. Design-file evidence is an artifact read from a .pen file, not a market outcome."
    )
}

fn compact_dossier(report: &UxReport) -> String {
    let backlog: Vec<Value> = report
        .backlog
        .iter()
        .take(12)
        .map(|item| {
            json!({
                "band": item.band,
                "impact": item.impact,
                "effort": item.effort,
                "screen": item.screen_name,
                "statement": item.statement,
                "recommendation": item.recommendation,
                "interpretation": item.interpretation,
                "evidence_ids": item.evidence_ids
            })
        })
        .collect();
    let body = json!({
        "question": report.question,
        "category": report.category,
        "notice": report.synthetic_panel_notice,
        "stats": report.stats,
        "backlog_top": backlog,
        "contrast_failures": report.contrast.iter().filter(|row| !row.passes_aa).take(8).collect::<Vec<_>>(),
        "source_ids": report.sources.iter().map(|s| json!({"id": s.id, "role": s.role, "synthetic_fixture": s.synthetic_fixture})).collect::<Vec<_>>()
    });
    truncate(&body.to_string(), 12_000)
}

fn mint_id(doc: &str, raw: &str, used: &mut BTreeSet<String>) -> String {
    let mut clean: String = raw
        .chars()
        .filter(|c| c.is_ascii_alphanumeric() || *c == '-' || *c == '_')
        .take(40)
        .collect();
    if clean.is_empty() {
        clean = format!("s{}", &crate::digest(raw.as_bytes())[..8]);
    }
    let mut id = format!("pen-{doc}-{clean}");
    let mut n = 2;
    while !used.insert(id.clone()) {
        id = format!("pen-{doc}-{clean}-{n}");
        n += 1;
    }
    id
}

fn role_of(name: &str) -> &'static str {
    let n = name.to_lowercase();
    if contains_any(
        &n,
        &["paywall", "suscrip", "premium", "desbloquear", "unlock"],
    ) {
        "paywall"
    } else if contains_any(&n, &["precio", "pricing", "tarifa", "planes", "plan"]) {
        "pricing"
    } else if contains_any(
        &n,
        &["onboarding", "bienvenida", "splash", "tutorial", "welcome"],
    ) {
        "onboarding"
    } else if contains_any(&n, &["config", "perfil", "settings", "cuenta", "ajustes"]) {
        "settings"
    } else if contains_any(&n, &["inicio", "home", "hoy"]) {
        "home"
    } else {
        "other"
    }
}

fn contains_any(text: &str, words: &[&str]) -> bool {
    words.iter().any(|word| text.contains(word))
}

fn is_cta(text: &TextNode) -> bool {
    if text.content.chars().count() > 60 || is_back(&text.content) {
        return false;
    }
    let hinted = text
        .ancestor_names
        .iter()
        .chain(std::iter::once(&text.own_name))
        .filter(|name| !name.is_empty())
        .any(|name| {
            let n = name.to_lowercase();
            ["button", "cta", "btn", "chip"]
                .iter()
                .any(|hint| n.contains(hint))
        });
    hinted || imperative(&text.content)
}

fn imperative(text: &str) -> bool {
    const VERBS: &[&str] = &[
        "empezar",
        "comienza",
        "comenzar",
        "continuar",
        "continua",
        "guarda",
        "guardar",
        "registra",
        "registrar",
        "termina",
        "terminar",
        "explora",
        "explorar",
        "suscribete",
        "suscríbete",
        "compra",
        "comprar",
        "prueba",
        "probar",
        "inicia",
        "iniciar",
        "escribe",
        "escribir",
        "notar",
        "acompanar",
        "acompañar",
        "integrar",
        "deshacer",
        "salir",
        "cancela",
        "cancelar",
        "omitir",
        "siguiente",
        "start",
        "continue",
        "save",
        "subscribe",
        "buy",
        "try",
        "unlock",
        "skip",
        "activar",
        "descargar",
        "pagar",
        "elegir",
        "seleccionar",
        "unirme",
        "ver",
    ];
    let word = first_word(text);
    VERBS.contains(&word.as_str())
}

fn is_back(text: &str) -> bool {
    let raw = text.trim();
    if raw == "←" || raw == "<" || raw == "‹" {
        return true;
    }
    matches!(norm(raw).as_str(), "volver" | "atras" | "back" | "regresar")
}

fn is_destructive(text: &str) -> bool {
    let word = first_word(text);
    matches!(
        word.as_str(),
        "terminar"
            | "termina"
            | "salir"
            | "cancelar"
            | "cancela"
            | "descartar"
            | "descarta"
            | "borrar"
            | "eliminar"
            | "skip"
            | "omitir"
            | "delete"
            | "discard"
            | "cerrar"
    ) || norm(text).contains("sin guardar")
}

fn is_disclaimer(text: &str) -> bool {
    let n = text.to_lowercase();
    n.contains("no es un diagnóstico")
        || n.contains("no es un diagnostico")
        || n.contains("sin resultados")
        || n.contains("not a diagnosis")
}

fn first_word(text: &str) -> String {
    norm(text)
        .split_whitespace()
        .next()
        .unwrap_or("")
        .to_owned()
}

fn norm(text: &str) -> String {
    text.chars()
        .map(|c| {
            if c.is_alphanumeric() {
                c.to_lowercase().next().unwrap_or(c)
            } else {
                ' '
            }
        })
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

fn near(a: &str, b: &str) -> bool {
    if a == b {
        return true;
    }
    let (short, long) = if a.chars().count() <= b.chars().count() {
        (a, b)
    } else {
        (b, a)
    };
    let short_len = short.chars().count();
    let long_len = long.chars().count();
    if short_len >= 6 && long.contains(short) && long_len - short_len <= 12 {
        return true;
    }
    short_len >= 8 && long_len <= 40 && levenshtein(short, long) <= 2
}

fn levenshtein(a: &str, b: &str) -> usize {
    let a: Vec<char> = a.chars().collect();
    let b: Vec<char> = b.chars().collect();
    if a.is_empty() {
        return b.len();
    }
    let mut prev: Vec<usize> = (0..=b.len()).collect();
    let mut cur = vec![0; b.len() + 1];
    for (i, ca) in a.iter().enumerate() {
        cur[0] = i + 1;
        for (j, cb) in b.iter().enumerate() {
            let cost = usize::from(ca != cb);
            cur[j + 1] = (prev[j + 1] + 1).min(cur[j] + 1).min(prev[j] + cost);
        }
        std::mem::swap(&mut prev, &mut cur);
    }
    prev[b.len()]
}

fn paid_price(text: &str) -> Option<String> {
    let lower = text.to_lowercase();
    let chars: Vec<(usize, char)> = text.char_indices().collect();
    for (index, ch) in &chars {
        if matches!(ch, '$' | '€' | '£') {
            let rest: String = text[*index..].chars().take(16).collect();
            if rest.chars().any(|c| c.is_ascii_digit()) {
                return Some(rest.trim().to_owned());
            }
        }
    }
    for token in ["us$", "mx$", "r$", "usd", "eur", "mxn", "cop", "brl"] {
        if let Some(pos) = lower.find(token) {
            let window: String = lower[pos..].chars().take(16).collect();
            if window.chars().any(|c| c.is_ascii_digit()) {
                return Some(text[pos..].chars().take(16).collect());
            }
        }
    }
    for phrase in ["/mes", "al mes", "/month", "per month", "/año", "/year"] {
        if let Some(pos) = lower.find(phrase) {
            let start = pos.saturating_sub(8);
            if lower[start..pos].chars().any(|c| c.is_ascii_digit()) {
                let mut from = start;
                while from < text.len() && !text.is_char_boundary(from) {
                    from += 1;
                }
                return Some(text[from..].chars().take(18).collect());
            }
        }
    }
    None
}

fn free_mention(text: &str) -> bool {
    norm(text).split_whitespace().any(|word| {
        matches!(
            word,
            "gratis" | "gratuito" | "gratuita" | "gratuitos" | "free"
        )
    })
}

fn excerpt(content: &str, terms: &[&str]) -> Option<String> {
    let lower = content.to_lowercase();
    let (pos, term) = terms
        .iter()
        .filter_map(|term| lower.find(&term.to_lowercase()).map(|pos| (pos, *term)))
        .min_by_key(|(pos, _)| *pos)?;
    let mut start = pos.saturating_sub(80);
    while start < content.len() && !content.is_char_boundary(start) {
        start += 1;
    }
    let mut end = (pos + term.len() + 90).min(content.len());
    while end > start && !content.is_char_boundary(end) {
        end -= 1;
    }
    let snippet = content[start..end]
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");
    if snippet.is_empty() {
        None
    } else {
        Some(snippet)
    }
}

fn step_number(name: &str) -> Option<u32> {
    name.split(|c: char| !c.is_ascii_digit())
        .filter(|token| token.len() == 2)
        .filter_map(|token| token.parse::<u32>().ok())
        .find(|n| *n >= 2)
}

fn slug(name: &str) -> String {
    let mut out = String::new();
    let mut dash = false;
    for c in name.chars() {
        if c.is_ascii_alphanumeric() {
            out.push(c.to_ascii_lowercase());
            dash = false;
        } else if !dash && !out.is_empty() {
            out.push('-');
            dash = true;
        }
    }
    out.trim_matches('-').to_owned()
}

fn image_dimensions(bytes: &[u8]) -> Option<(u32, u32)> {
    if bytes.len() >= 24 && bytes.starts_with(b"\x89PNG\r\n\x1a\n") && &bytes[12..16] == b"IHDR" {
        let width = u32::from_be_bytes(bytes[16..20].try_into().ok()?);
        let height = u32::from_be_bytes(bytes[20..24].try_into().ok()?);
        return Some((width, height));
    }
    if bytes.len() < 4 || bytes[0] != 0xFF || bytes[1] != 0xD8 {
        return None;
    }
    let mut i = 2;
    while i + 9 < bytes.len() {
        if bytes[i] != 0xFF {
            return None;
        }
        let marker = bytes[i + 1];
        if marker == 0xC0 || marker == 0xC1 || marker == 0xC2 {
            let height = u16::from_be_bytes([bytes[i + 5], bytes[i + 6]]) as u32;
            let width = u16::from_be_bytes([bytes[i + 7], bytes[i + 8]]) as u32;
            return Some((width, height));
        }
        if marker == 0xD8 || marker == 0x01 || (0xD0..=0xD9).contains(&marker) {
            i += 2;
            continue;
        }
        let len = u16::from_be_bytes([bytes[i + 2], bytes[i + 3]]) as usize;
        if len < 2 {
            return None;
        }
        i += 2 + len;
    }
    None
}

fn color_note(label: &str) -> String {
    if label.starts_with('$') {
        format!(" {label}")
    } else {
        String::new()
    }
}

fn opaque(color: Rgba) -> Rgba {
    Rgba { a: 255, ..color }
}

fn fmt_px(value: Option<f64>) -> String {
    match value {
        Some(n) if (n - n.round()).abs() < 0.05 => format!("{}", n.round() as i64),
        Some(n) => format!("{n:.1}"),
        None => "?".into(),
    }
}

fn truncate(text: &str, max_chars: usize) -> String {
    let count = text.chars().count();
    if count <= max_chars {
        return text.to_owned();
    }
    text.chars().take(max_chars).collect()
}

fn band(impact: u8, effort: u8) -> &'static str {
    if impact >= 4 && effort <= 2 {
        "do_now"
    } else if impact >= 4 {
        "plan"
    } else if effort <= 2 {
        "polish"
    } else {
        "defer"
    }
}

fn band_rank(band: &str) -> u8 {
    match band {
        "do_now" => 0,
        "plan" => 1,
        "polish" => 2,
        _ => 3,
    }
}

fn band_label(band: &str) -> &'static str {
    match band {
        "do_now" => "hacer ahora",
        "plan" => "planificar",
        "polish" => "pulir",
        _ => "diferir",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::EvidenceKind;
    use chrono::DateTime;

    const FIXTURE: &str = include_str!("../../examples/ux.meditation.pen");

    fn date(value: &str) -> DateTime<Utc> {
        DateTime::parse_from_rfc3339(value)
            .unwrap()
            .with_timezone(&Utc)
    }

    fn options(competitive: Vec<Evidence>, screenshots: Vec<Screenshot>) -> AuditOptions {
        AuditOptions {
            source_name: "examples/ux.meditation.pen".into(),
            category: Some("meditation".into()),
            question: "¿Qué se cambia primero en este diseño?".into(),
            as_of: date("2026-10-25T00:00:00Z"),
            competitive,
            screenshots,
        }
    }

    fn bench() -> Evidence {
        Evidence {
            id: "bench-calm".into(),
            source: "https://example.com/meditation-pricing".into(),
            provider: "public_document".into(),
            kind: EvidenceKind::Benchmark,
            published_at: None,
            collected_at: date("2026-10-01T00:00:00Z"),
            observed_at: date("2026-10-01T00:00:00Z"),
            segment: Some("meditation".into()),
            population: None,
            content: "Best-selling meditation apps show the subscription price and billing period before the paywall CTA. Trials disclose the renewal price.".into(),
            limitations: vec!["Fixture excerpt for the audit test.".into()],
            synthetic_fixture: false,
        }
    }

    #[test]
    fn meditation_fixture_separates_measurement_inference_and_personas() {
        let report = audit(FIXTURE.as_bytes(), &options(vec![], vec![])).unwrap();
        assert_eq!(report.status, "deterministic");
        assert_eq!(report.stats.screens, 3);
        assert!(report.stats.contrast_failures >= 1);
        assert_eq!(report.stats.paid_price_mentions, 0);
        assert!(report.stats.free_mentions >= 1);
        let title = report
            .contrast
            .iter()
            .find(|row| row.text.contains("Encuentra calma"))
            .unwrap();
        assert!(!title.passes_aa);
        assert!(title.foreground.eq_ignore_ascii_case("#5E6570"));
        assert!(title.background.eq_ignore_ascii_case("#12141A"));
        let body = report
            .contrast
            .iter()
            .find(|row| row.text.contains("Tu práctica de hoy"))
            .unwrap();
        assert!(body.passes_aa);
        for row in &report.contrast {
            let fg = contrast::parse_hex(&row.foreground).unwrap();
            let bg = contrast::parse_hex(&row.background).unwrap();
            let ratio = contrast::contrast_ratio(fg, bg);
            assert!((ratio - row.ratio).abs() < 1e-6);
            assert_eq!(row.passes_aa, contrast::passes_aa(ratio, row.large_text));
        }
        let texts: Vec<String> = report
            .screens
            .iter()
            .flat_map(|s| s.texts.clone())
            .collect();
        assert!(texts.iter().any(|t| t == "Comenzar"));
        assert!(texts.iter().any(|t| t == "Comenzar ahora"));
        assert!(texts.iter().all(|t| t != "Submit"));
        assert!(texts.iter().all(|t| !t.contains("no es UI")));
        assert!(report
            .backlog
            .iter()
            .any(|item| item.kind == "duplicate_cta" && item.band == "do_now"));
        assert!(report
            .backlog
            .iter()
            .any(|item| item.kind == "paid_price" && item.band == "plan"));
        assert!(report.backlog.iter().any(|item| item.kind == "contrast"));
        assert!(report.backlog.iter().any(|item| item.kind == "small_text"));
        assert!(report.backlog.iter().any(|item| item.kind == "icon_only"));
        assert!(report.backlog.iter().any(|item| item.kind == "placeholder"));
        assert!(report.backlog.iter().any(|item| item.kind == "missing_cta"));
        assert!(report
            .backlog
            .iter()
            .any(|item| item.kind == "destructive_cta"));
        assert!(
            report
                .backlog
                .iter()
                .all(|item| item.interpretation == "observation"
                    || item.interpretation == "inference")
        );
        assert!(report
            .backlog
            .iter()
            .all(|item| !item.not_real_users && !item.evidence_ids.is_empty()));
        assert!(report
            .synthetic_panel_notice
            .contains("No son usuarios reales"));
        assert!(report
            .synthetic_opinions
            .iter()
            .any(|item| item.statement.contains("Carlos")));
        assert!(report
            .synthetic_opinions
            .iter()
            .all(|item| item.interpretation == "simulation" && item.not_real_users));
        assert!(report.synthetic_opinions.iter().all(|item| item
            .evidence_ids
            .iter()
            .any(|id| id.contains("synthetic-panel"))));
        let competitive = report
            .specialists
            .iter()
            .find(|agent| agent.role == "competitive_benchmark")
            .unwrap();
        let gaps = &competitive.output.as_ref().unwrap().gaps;
        assert!(gaps.iter().any(|gap| gap.contains("no inventa")));
        assert!(
            report.critic.output.as_ref().unwrap().gaps.is_empty(),
            "{:?}",
            report.critic.output
        );
        let rendered = render(&report);
        assert!(rendered.contains("HACER AHORA"));
        assert!(rendered.contains("No son usuarios reales"));
        assert!(!rendered.contains("Submit"));
    }

    #[test]
    fn competitive_quotes_are_inferences_and_imports_are_stable() {
        let report = audit(FIXTURE.as_bytes(), &options(vec![bench()], vec![])).unwrap();
        let finding = report
            .specialists
            .iter()
            .find(|agent| agent.role == "competitive_benchmark")
            .unwrap()
            .output
            .as_ref()
            .unwrap()
            .findings
            .iter()
            .find(|finding| finding.evidence_ids.iter().any(|id| id == "bench-calm"))
            .unwrap();
        assert_eq!(finding.interpretation, "inference");
        assert!(finding.statement.contains("bench-calm"));
        assert!(
            finding.statement.to_lowercase().contains("price")
                || finding.statement.to_lowercase().contains("billing")
        );
        assert!(!finding.statement.contains("descargas estimadas"));
        let dir = tempfile::tempdir().unwrap();
        let store = Store::open(dir.path().join("ux.sqlite")).unwrap();
        let first = store.import(&report.evidence).unwrap();
        assert!(first >= 4);
        assert_eq!(store.import(&report.evidence).unwrap(), 0);
        let again = audit(FIXTURE.as_bytes(), &options(vec![bench()], vec![])).unwrap();
        assert_eq!(store.import(&again.evidence).unwrap(), 0);
    }

    #[test]
    fn screenshot_pixel_size_is_linked_without_a_model() {
        let mut png = vec![0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];
        png.extend_from_slice(&13u32.to_be_bytes());
        png.extend_from_slice(b"IHDR");
        png.extend_from_slice(&100u32.to_be_bytes());
        png.extend_from_slice(&80u32.to_be_bytes());
        assert_eq!(image_dimensions(&png), Some((100, 80)));
        let report = audit(
            FIXTURE.as_bytes(),
            &options(
                vec![],
                vec![Screenshot {
                    path: "out/home.png".into(),
                    bytes: png,
                }],
            ),
        )
        .unwrap();
        let home = report
            .screens
            .iter()
            .find(|screen| screen.id == "home")
            .unwrap();
        assert_eq!(home.screenshot.as_deref(), Some("out/home.png"));
        assert_eq!(home.screenshot_px, Some([100, 80]));
        assert!(report
            .backlog
            .iter()
            .any(|item| item.kind == "screenshot_mismatch"));
        assert!(neural_brief("ux_critic").contains("not real users"));
        assert!(neural_brief("ux_synthesis").contains("WCAG"));
    }
}
