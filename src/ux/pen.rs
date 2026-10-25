//! Reader for pen.dev `.pen` documents (JSON, optional comments).
//!
//! Resolves variables, themes and component instances enough to recover visible
//! text, solid colors and control bounds. Does not execute scripts or shaders.

use super::contrast::{self, Rgba};
use anyhow::{bail, Context, Result};
use serde_json::{Map, Value};
use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub struct Design {
    pub version: String,
    pub warnings: Vec<String>,
    pub variables: Vec<VariableView>,
    pub screens: Vec<Screen>,
    pub designer_notes: Vec<String>,
    pub has_scripts: bool,
}

#[derive(Debug, Clone)]
pub struct VariableView {
    pub name: String,
    pub kind: String,
    pub display: String,
}

#[derive(Debug, Clone)]
pub struct Screen {
    pub id: String,
    pub name: String,
    pub width: Option<f64>,
    pub height: Option<f64>,
    pub placeholder: bool,
    pub texts: Vec<TextNode>,
    pub controls: Vec<Control>,
    pub icon_only: Vec<Control>,
}

#[derive(Debug, Clone)]
pub struct TextNode {
    pub node_path: String,
    pub own_name: String,
    pub ancestor_names: Vec<String>,
    pub content: String,
    pub font_size: Option<f64>,
    pub font_weight: Option<u32>,
    pub foreground: Option<Rgba>,
    pub background: Option<Rgba>,
    pub fg_label: String,
    pub bg_label: String,
    pub non_solid: bool,
}

#[derive(Debug, Clone)]
pub struct Control {
    pub node_path: String,
    pub name: String,
    pub width: Option<f64>,
    pub height: Option<f64>,
}

pub fn parse_design(bytes: &[u8]) -> Result<Design> {
    let text = String::from_utf8(bytes.to_vec()).context("pen file is not UTF-8")?;
    let stripped = strip_json_comments(&text);
    let document: Value = serde_json::from_str(&stripped).context(
        "pen file is not JSON after removing comments. The audit reads the .pen document itself and does not call pen.dev",
    )?;
    let root = document
        .as_object()
        .context("pen document must be a JSON object")?;
    let version = match root.get("version") {
        Some(Value::String(s)) => s.clone(),
        Some(Value::Number(n)) => n.to_string(),
        _ => "unknown".into(),
    };
    let themes = root.get("themes").and_then(Value::as_object);
    let default_theme = default_theme(themes);
    let variables = root
        .get("variables")
        .and_then(Value::as_object)
        .cloned()
        .unwrap_or_default();
    let children = root
        .get("children")
        .and_then(Value::as_array)
        .context("pen document requires a children array")?;
    let mut index = BTreeMap::new();
    for child in children {
        index_ids(child, &mut index)?;
    }
    let mut warnings = Vec::new();
    if root
        .get("imports")
        .and_then(Value::as_object)
        .is_some_and(|m| !m.is_empty())
    {
        warnings.push(
            "Imported .pen libraries were not loaded; only objects in this file are audited."
                .into(),
        );
    }
    let mut screens = Vec::new();
    let mut designer_notes = Vec::new();
    let mut has_scripts = false;
    for child in children {
        let ty = child.get("type").and_then(Value::as_str).unwrap_or("");
        if matches!(ty, "note" | "prompt" | "context") {
            if let Some(text) = text_content(child) {
                designer_notes.push(text);
            }
            continue;
        }
        if ty == "script" {
            has_scripts = true;
            continue;
        }
        if child.get("reusable").and_then(Value::as_bool) == Some(true) {
            continue;
        }
        match ty {
            "frame" | "ref" => {
                screens.push(materialize_screen(
                    child,
                    &index,
                    &variables,
                    &default_theme,
                    &mut warnings,
                    &mut has_scripts,
                )?);
            }
            "group" => {
                let Some(nested) = child.get("children").and_then(Value::as_array) else {
                    continue;
                };
                for grandchild in nested {
                    let gty = grandchild.get("type").and_then(Value::as_str).unwrap_or("");
                    if grandchild.get("reusable").and_then(Value::as_bool) == Some(true) {
                        continue;
                    }
                    if matches!(gty, "frame" | "ref") {
                        screens.push(materialize_screen(
                            grandchild,
                            &index,
                            &variables,
                            &default_theme,
                            &mut warnings,
                            &mut has_scripts,
                        )?);
                    }
                }
            }
            _ => {}
        }
    }
    if screens.len() > 800 {
        bail!(
            "pen file has {} top-level screens; the audit accepts at most 800. Split the document first",
            screens.len()
        );
    }
    let variable_views = variable_views(&variables, &default_theme);
    Ok(Design {
        version,
        warnings,
        variables: variable_views,
        screens,
        designer_notes,
        has_scripts,
    })
}

fn materialize_screen(
    raw: &Value,
    index: &BTreeMap<String, Value>,
    variables: &Map<String, Value>,
    theme: &BTreeMap<String, String>,
    warnings: &mut Vec<String>,
    has_scripts: &mut bool,
) -> Result<Screen> {
    let mut stack = Vec::new();
    let expanded = expand_node(raw, index, &mut stack, 0)?;
    let mut screen = Screen {
        id: expanded
            .get("id")
            .and_then(Value::as_str)
            .unwrap_or("screen")
            .to_owned(),
        name: expanded
            .get("name")
            .and_then(Value::as_str)
            .unwrap_or("(sin nombre)")
            .to_owned(),
        width: dimension(expanded.get("width"), variables, theme),
        height: dimension(expanded.get("height"), variables, theme),
        placeholder: false,
        texts: Vec::new(),
        controls: Vec::new(),
        icon_only: Vec::new(),
    };
    let mut ctx = Walk {
        variables,
        theme: theme.clone(),
        backdrop: None,
        backdrop_label: String::new(),
        ancestors: Vec::new(),
        ancestor_names: Vec::new(),
        warnings,
        nodes: 0,
    };
    walk(&expanded, &mut ctx, &mut screen, has_scripts);
    Ok(screen)
}

struct Walk<'a> {
    variables: &'a Map<String, Value>,
    theme: BTreeMap<String, String>,
    backdrop: Option<Rgba>,
    backdrop_label: String,
    ancestors: Vec<String>,
    ancestor_names: Vec<String>,
    warnings: &'a mut Vec<String>,
    nodes: usize,
}

fn walk(
    node: &Value,
    ctx: &mut Walk<'_>,
    screen: &mut Screen,
    has_scripts: &mut bool,
) -> (bool, bool) {
    if ctx.nodes > 4000 {
        if ctx.warnings.len() < 20 {
            ctx.warnings.push(format!(
                "Screen {} stopped after 4000 nodes; deeper objects were not read.",
                screen.id
            ));
        }
        return (false, false);
    }
    ctx.nodes += 1;
    let ty = node.get("type").and_then(Value::as_str).unwrap_or("");
    if matches!(ty, "note" | "prompt" | "context") {
        return (false, false);
    }
    if ty == "script" {
        *has_scripts = true;
        return (false, false);
    }
    if node.get("enabled").and_then(Value::as_bool) == Some(false) {
        return (false, false);
    }
    let id = node
        .get("id")
        .and_then(Value::as_str)
        .unwrap_or("node")
        .to_owned();
    let name = node
        .get("name")
        .and_then(Value::as_str)
        .unwrap_or("")
        .to_owned();
    let path = if ctx.ancestors.is_empty() {
        id.clone()
    } else {
        format!("{}/{id}", ctx.ancestors.join("/"))
    };
    if node.get("placeholder").and_then(Value::as_bool) == Some(true) {
        screen.placeholder = true;
    }
    let saved_theme = ctx.theme.clone();
    ctx.theme = overlay_theme(&ctx.theme, node);
    let paint = resolve_paint(node.get("fill"), ctx.variables, &ctx.theme, ctx.warnings);
    if ty == "text" {
        if let Some(content) = node
            .get("content")
            .and_then(|value| resolve_text(value, ctx.variables, &ctx.theme, 0))
        {
            let (foreground, fg_label, non_solid) = match &paint {
                Paint::Solid(color, label) => {
                    let fg = match ctx.backdrop {
                        Some(bg) if color.a < 255 => contrast::composite(*color, bg),
                        _ => *color,
                    };
                    (Some(fg), label.clone(), false)
                }
                Paint::NonSolid(label) => (None, label.clone(), true),
                Paint::None => (None, "sin relleno".into(), false),
            };
            let font_size = node
                .get("fontSize")
                .and_then(|v| resolve_number(v, ctx.variables, &ctx.theme, 0));
            let font_weight = node
                .get("fontWeight")
                .and_then(|v| resolve_weight(v, ctx.variables, &ctx.theme));
            screen.texts.push(TextNode {
                node_path: path.clone(),
                own_name: name.clone(),
                ancestor_names: ctx.ancestor_names.clone(),
                content,
                font_size,
                font_weight,
                foreground,
                background: ctx.backdrop,
                fg_label,
                bg_label: if ctx.backdrop.is_some() {
                    ctx.backdrop_label.clone()
                } else {
                    "sin fondo sólido".into()
                },
                non_solid,
            });
        }
    }
    let saved_backdrop = ctx.backdrop;
    let saved_label = ctx.backdrop_label.clone();
    if ty != "text" {
        match &paint {
            Paint::Solid(color, label) => {
                ctx.backdrop = Some(match saved_backdrop {
                    Some(bg) if color.a < 255 => contrast::composite(*color, bg),
                    _ => *color,
                });
                ctx.backdrop_label = label.clone();
            }
            Paint::NonSolid(label) => {
                ctx.backdrop = None;
                ctx.backdrop_label = label.clone();
            }
            Paint::None => {}
        }
    }
    ctx.ancestors.push(id.clone());
    if !name.is_empty() {
        ctx.ancestor_names.push(name.clone());
    }
    let mut has_text = ty == "text"
        && node
            .get("content")
            .and_then(Value::as_str)
            .is_some_and(|s| !s.trim().is_empty());
    let mut has_icon = ty == "icon";
    if let Some(children) = node.get("children").and_then(Value::as_array) {
        for child in children {
            let (text, icon) = walk(child, ctx, screen, has_scripts);
            has_text |= text;
            has_icon |= icon;
        }
    }
    if matches!(ty, "frame" | "rectangle" | "ref") && target_name(&name) {
        let width = dimension(node.get("width"), ctx.variables, &ctx.theme);
        let height = dimension(node.get("height"), ctx.variables, &ctx.theme);
        let control = Control {
            node_path: path,
            name: name.clone(),
            width,
            height,
        };
        if has_icon && !has_text {
            screen.icon_only.push(control.clone());
        }
        if let (Some(w), Some(h)) = (width, height) {
            if w >= 8.0 && h >= 8.0 && w < 44.0 && h < 44.0 {
                screen.controls.push(control);
            }
        }
    }
    ctx.ancestors.pop();
    if !name.is_empty() {
        ctx.ancestor_names.pop();
    }
    ctx.theme = saved_theme;
    ctx.backdrop = saved_backdrop;
    ctx.backdrop_label = saved_label;
    (has_text, has_icon)
}

fn target_name(name: &str) -> bool {
    let n = name.to_lowercase();
    ["button", "cta", "btn", "icon", "chip", "tab"]
        .iter()
        .any(|hint| n.contains(hint))
}

fn text_content(node: &Value) -> Option<String> {
    let raw = node.get("content")?.as_str()?.trim();
    if raw.is_empty() {
        None
    } else {
        Some(raw.to_owned())
    }
}

fn index_ids(node: &Value, into: &mut BTreeMap<String, Value>) -> Result<()> {
    let Some(obj) = node.as_object() else {
        return Ok(());
    };
    if let Some(id) = obj.get("id").and_then(Value::as_str) {
        if into.insert(id.to_owned(), node.clone()).is_some() {
            bail!("Duplicate object id {id}");
        }
    }
    if let Some(children) = obj.get("children").and_then(Value::as_array) {
        for child in children {
            index_ids(child, into)?;
        }
    }
    Ok(())
}

fn expand_node(
    node: &Value,
    index: &BTreeMap<String, Value>,
    stack: &mut Vec<String>,
    depth: usize,
) -> Result<Value> {
    if depth > 32 {
        bail!("Component nesting exceeds 32");
    }
    let Some(obj) = node.as_object() else {
        return Ok(node.clone());
    };
    if obj.get("type").and_then(Value::as_str) == Some("ref") {
        return instantiate(obj, index, stack, depth);
    }
    let mut clone = node.clone();
    expand_children(&mut clone, index, stack, depth)?;
    Ok(clone)
}

fn instantiate(
    obj: &Map<String, Value>,
    index: &BTreeMap<String, Value>,
    stack: &mut Vec<String>,
    depth: usize,
) -> Result<Value> {
    let target = obj
        .get("ref")
        .and_then(Value::as_str)
        .context("ref is missing its target id")?;
    if stack.iter().any(|id| id == target) {
        bail!("Cyclic component ref {target}");
    }
    let component = index
        .get(target)
        .with_context(|| format!("Unknown component {target}"))?;
    stack.push(target.to_owned());
    let mut concrete = expand_node(component, index, stack, depth + 1)?;
    stack.pop();
    if let Some(map) = concrete.as_object_mut() {
        for (key, value) in obj {
            if matches!(
                key.as_str(),
                "type" | "ref" | "descendants" | "id" | "children"
            ) {
                continue;
            }
            map.insert(key.clone(), value.clone());
        }
        if let Some(id) = obj.get("id") {
            map.insert("id".into(), id.clone());
        }
        map.remove("reusable");
    }
    if let Some(descendants) = obj.get("descendants") {
        apply_descendants(&mut concrete, descendants, index, stack, depth + 1)?;
    }
    Ok(concrete)
}

fn expand_children(
    node: &mut Value,
    index: &BTreeMap<String, Value>,
    stack: &mut Vec<String>,
    depth: usize,
) -> Result<()> {
    let Some(children) = node.get("children").and_then(Value::as_array) else {
        return Ok(());
    };
    let mut expanded = Vec::with_capacity(children.len());
    for child in children.clone() {
        expanded.push(expand_node(&child, index, stack, depth)?);
    }
    if let Some(slot) = node.get_mut("children") {
        *slot = Value::Array(expanded);
    }
    Ok(())
}

fn apply_descendants(
    node: &mut Value,
    descendants: &Value,
    index: &BTreeMap<String, Value>,
    stack: &mut Vec<String>,
    depth: usize,
) -> Result<()> {
    let Some(map) = descendants.as_object() else {
        return Ok(());
    };
    for (path, override_value) in map {
        apply_path(node, path, override_value, index, stack, depth)?;
    }
    Ok(())
}

fn apply_path(
    node: &mut Value,
    path: &str,
    override_value: &Value,
    index: &BTreeMap<String, Value>,
    stack: &mut Vec<String>,
    depth: usize,
) -> Result<()> {
    let (first, rest) = path
        .split_once('/')
        .map_or((path, None), |(a, b)| (a, Some(b)));
    let children = node
        .get_mut("children")
        .and_then(Value::as_array_mut)
        .with_context(|| format!("Descendant path {path} has no children to override"))?;
    let child = children
        .iter_mut()
        .find(|child| child.get("id").and_then(Value::as_str) == Some(first))
        .with_context(|| format!("Descendant path {path} does not match a child id"))?;
    if let Some(rest) = rest {
        return apply_path(child, rest, override_value, index, stack, depth);
    }
    if override_value.get("type").is_some() {
        *child = expand_node(override_value, index, stack, depth)?;
        return Ok(());
    }
    if let Some(map) = child.as_object_mut() {
        if let Some(over) = override_value.as_object() {
            for (key, value) in over {
                map.insert(key.clone(), value.clone());
            }
        }
    }
    expand_children(child, index, stack, depth)?;
    Ok(())
}

#[derive(Clone)]
enum Paint {
    None,
    Solid(Rgba, String),
    NonSolid(String),
}

fn resolve_paint(
    fill: Option<&Value>,
    variables: &Map<String, Value>,
    theme: &BTreeMap<String, String>,
    _warnings: &mut Vec<String>,
) -> Paint {
    let Some(fill) = fill else {
        return Paint::None;
    };
    let fills: Vec<&Value> = match fill {
        Value::Array(items) => items.iter().collect(),
        other => vec![other],
    };
    let mut acc: Option<(Rgba, String)> = None;
    let mut non_solid: Option<String> = None;
    for item in fills {
        if !fill_enabled(item, variables, theme) {
            continue;
        }
        match classify_fill(item, variables, theme) {
            Paint::Solid(color, label) => {
                let color = match acc {
                    Some((bg, _)) if color.a < 255 => contrast::composite(color, bg),
                    _ => color,
                };
                acc = Some((color, label));
            }
            Paint::NonSolid(label) => non_solid = Some(label),
            Paint::None => {}
        }
    }
    if let Some(label) = non_solid {
        return Paint::NonSolid(label);
    }
    match acc {
        Some((color, label)) => Paint::Solid(color, label),
        None => Paint::None,
    }
}

fn fill_enabled(
    fill: &Value,
    variables: &Map<String, Value>,
    theme: &BTreeMap<String, String>,
) -> bool {
    match fill.get("enabled") {
        None => true,
        Some(Value::Bool(value)) => *value,
        Some(Value::String(value)) => resolve_bool(value, variables, theme).unwrap_or(true),
        Some(_) => true,
    }
}

fn classify_fill(
    fill: &Value,
    variables: &Map<String, Value>,
    theme: &BTreeMap<String, String>,
) -> Paint {
    if let Some(text) = fill.as_str() {
        return color_from_string(text, variables, theme, 0)
            .map(|(color, label)| Paint::Solid(color, label))
            .unwrap_or(Paint::NonSolid(text.to_owned()));
    }
    let kind = fill.get("type").and_then(Value::as_str).unwrap_or("color");
    if matches!(kind, "gradient" | "image" | "mesh_gradient" | "shader") {
        return Paint::NonSolid(kind.to_owned());
    }
    if kind != "color" && fill.get("color").is_none() {
        return Paint::NonSolid(kind.to_owned());
    }
    let Some(color_value) = fill.get("color").or_else(|| fill.as_str().map(|_| fill)) else {
        return Paint::None;
    };
    match color_value.as_str() {
        Some(text) => color_from_string(text, variables, theme, 0)
            .map(|(color, label)| Paint::Solid(color, label))
            .unwrap_or(Paint::NonSolid(text.to_owned())),
        None => Paint::None,
    }
}

fn color_from_string(
    text: &str,
    variables: &Map<String, Value>,
    theme: &BTreeMap<String, String>,
    depth: usize,
) -> Option<(Rgba, String)> {
    if depth > 8 {
        return None;
    }
    if let Some(name) = text.strip_prefix('$') {
        let def = variables.get(name)?;
        let inner = resolve_var_value(def, theme)?;
        let nested = inner.as_str()?;
        let (color, _) = color_from_string(nested, variables, theme, depth + 1)?;
        return Some((color, format!("${name}")));
    }
    contrast::parse_hex(text).map(|color| (color, text.to_owned()))
}

fn resolve_text(
    value: &Value,
    variables: &Map<String, Value>,
    theme: &BTreeMap<String, String>,
    depth: usize,
) -> Option<String> {
    if depth > 8 {
        return None;
    }
    let text = value.as_str()?;
    if let Some(name) = text.strip_prefix('$') {
        let inner = resolve_var_value(variables.get(name)?, theme)?;
        return resolve_text(&inner, variables, theme, depth + 1).or_else(|| match inner {
            Value::String(s) if !s.trim().is_empty() && !s.starts_with('$') => Some(s),
            _ => None,
        });
    }
    let trimmed = text.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_owned())
    }
}

fn resolve_number(
    value: &Value,
    variables: &Map<String, Value>,
    theme: &BTreeMap<String, String>,
    depth: usize,
) -> Option<f64> {
    if depth > 8 {
        return None;
    }
    match value {
        Value::Number(number) => number.as_f64(),
        Value::String(text) => {
            if let Some(name) = text.strip_prefix('$') {
                let inner = resolve_var_value(variables.get(name)?, theme)?;
                resolve_number(&inner, variables, theme, depth + 1)
            } else {
                text.parse().ok()
            }
        }
        _ => None,
    }
}

fn resolve_weight(
    value: &Value,
    variables: &Map<String, Value>,
    theme: &BTreeMap<String, String>,
) -> Option<u32> {
    if let Some(number) = resolve_number(value, variables, theme, 0) {
        return Some(number.round().clamp(1.0, 1000.0) as u32);
    }
    let text = value.as_str()?.to_lowercase();
    match text.as_str() {
        "bold" => Some(700),
        "semibold" | "semi-bold" => Some(600),
        "medium" => Some(500),
        "normal" | "regular" => Some(400),
        "light" => Some(300),
        _ => text.parse().ok(),
    }
}

fn resolve_bool(
    text: &str,
    variables: &Map<String, Value>,
    theme: &BTreeMap<String, String>,
) -> Option<bool> {
    if let Some(name) = text.strip_prefix('$') {
        let inner = resolve_var_value(variables.get(name)?, theme)?;
        return match inner {
            Value::Bool(value) => Some(value),
            Value::String(value) => resolve_bool(&value, variables, theme),
            _ => None,
        };
    }
    match text {
        "true" => Some(true),
        "false" => Some(false),
        _ => None,
    }
}

fn dimension(
    value: Option<&Value>,
    variables: &Map<String, Value>,
    theme: &BTreeMap<String, String>,
) -> Option<f64> {
    let value = value?;
    if value
        .as_str()
        .is_some_and(|s| s.contains("fit_content") || s.contains("fill_container"))
    {
        return None;
    }
    resolve_number(value, variables, theme, 0)
}

fn resolve_var_value(def: &Value, theme: &BTreeMap<String, String>) -> Option<Value> {
    let value = def.get("value")?;
    let Some(entries) = value.as_array() else {
        return Some(value.clone());
    };
    let mut chosen = None;
    for entry in entries {
        let Some(obj) = entry.as_object() else {
            chosen = Some(entry.clone());
            continue;
        };
        let ok = match obj.get("theme") {
            None => true,
            Some(cond) => cond
                .as_object()
                .is_some_and(|map| theme_matches(map, theme)),
        };
        if ok {
            if let Some(inner) = obj.get("value") {
                chosen = Some(inner.clone());
            }
        }
    }
    chosen
}

fn theme_matches(cond: &Map<String, Value>, theme: &BTreeMap<String, String>) -> bool {
    cond.iter().all(|(key, value)| {
        value
            .as_str()
            .is_some_and(|expected| theme.get(key).is_some_and(|have| have == expected))
    })
}

fn default_theme(themes: Option<&Map<String, Value>>) -> BTreeMap<String, String> {
    let mut theme = BTreeMap::new();
    let Some(themes) = themes else {
        return theme;
    };
    for (axis, values) in themes {
        if let Some(first) = values
            .as_array()
            .and_then(|items| items.first())
            .and_then(Value::as_str)
        {
            theme.insert(axis.clone(), first.to_owned());
        }
    }
    theme
}

fn overlay_theme(base: &BTreeMap<String, String>, node: &Value) -> BTreeMap<String, String> {
    let mut theme = base.clone();
    if let Some(over) = node.get("theme").and_then(Value::as_object) {
        for (key, value) in over {
            if let Some(text) = value.as_str() {
                theme.insert(key.clone(), text.to_owned());
            }
        }
    }
    theme
}

fn variable_views(
    variables: &Map<String, Value>,
    theme: &BTreeMap<String, String>,
) -> Vec<VariableView> {
    let mut views = Vec::new();
    for (name, def) in variables {
        let kind = def
            .get("type")
            .and_then(Value::as_str)
            .unwrap_or("unknown")
            .to_owned();
        let display = resolve_var_value(def, theme)
            .map(|value| match value {
                Value::String(text) => text,
                other => other.to_string(),
            })
            .unwrap_or_else(|| "unresolved".into());
        views.push(VariableView {
            name: name.clone(),
            kind,
            display,
        });
    }
    views.sort_by(|a, b| a.name.cmp(&b.name));
    views
}

pub fn strip_json_comments(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    let chars: Vec<char> = input.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        let c = chars[i];
        if c == '"' {
            out.push(c);
            i += 1;
            while i < chars.len() {
                let d = chars[i];
                out.push(d);
                if d == '\\' && i + 1 < chars.len() {
                    i += 1;
                    out.push(chars[i]);
                } else if d == '"' {
                    break;
                }
                i += 1;
            }
            i += 1;
            continue;
        }
        if c == '/' && i + 1 < chars.len() && chars[i + 1] == '/' {
            i += 2;
            while i < chars.len() && chars[i] != '\n' {
                i += 1;
            }
            continue;
        }
        if c == '/' && i + 1 < chars.len() && chars[i + 1] == '*' {
            i += 2;
            while i + 1 < chars.len() && !(chars[i] == '*' && chars[i + 1] == '/') {
                i += 1;
            }
            i = (i + 2).min(chars.len());
            continue;
        }
        out.push(c);
        i += 1;
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn comments_do_not_eat_urls_inside_strings() {
        let raw = r##"{
            // header
            "version": "2.21",
            "children": [{
                "id": "s",
                "type": "frame",
                "name": "Home",
                "fill": "#FFFFFF",
                "children": [{
                    "id": "t",
                    "type": "text",
                    "content": "See https://example.com /* not a comment */",
                    "fill": "#111111"
                }]
            }]
        }"##;
        let design = parse_design(raw.as_bytes()).unwrap();
        assert_eq!(design.screens.len(), 1);
        assert!(design.screens[0].texts[0]
            .content
            .contains("https://example.com"));
    }

    #[test]
    fn instances_apply_descendant_overrides_and_themes() {
        let raw = r##"{
            "version": "2.21",
            "themes": { "mode": ["light", "dark"] },
            "variables": {
                "color.background": {
                    "type": "color",
                    "value": [
                        { "value": "#FFFFFF", "theme": { "mode": "light" } },
                        { "value": "#000000", "theme": { "mode": "dark" } }
                    ]
                }
            },
            "children": [
                {
                    "id": "round-button",
                    "type": "frame",
                    "reusable": true,
                    "name": "Button",
                    "fill": "#FF0000",
                    "children": [{
                        "id": "label",
                        "type": "text",
                        "content": "Submit",
                        "fill": "#000000",
                        "fontSize": 16
                    }]
                },
                {
                    "id": "alert",
                    "type": "frame",
                    "reusable": true,
                    "name": "Alert",
                    "fill": "$color.background",
                    "children": [
                        {
                            "id": "banner",
                            "type": "text",
                            "content": "Hello",
                            "fill": "#FFFFFF",
                            "fontSize": 18
                        },
                        {
                            "id": "ok-button",
                            "type": "ref",
                            "ref": "round-button",
                            "descendants": { "label": { "content": "OK" } }
                        }
                    ]
                },
                {
                    "id": "save-alert",
                    "type": "ref",
                    "name": "Guardar cambios",
                    "ref": "alert",
                    "theme": { "mode": "dark" },
                    "descendants": {
                        "ok-button/label": { "content": "Save" }
                    }
                }
            ]
        }"##;
        let design = parse_design(raw.as_bytes()).unwrap();
        assert_eq!(design.screens.len(), 1);
        let screen = &design.screens[0];
        let banner = screen
            .texts
            .iter()
            .find(|text| text.content == "Hello")
            .unwrap();
        let label = screen
            .texts
            .iter()
            .find(|text| text.content == "Save")
            .unwrap();
        assert_eq!(banner.background.unwrap().hex(), "#000000");
        assert_eq!(label.background.unwrap().hex(), "#FF0000");
        assert!(design.screens.iter().all(|s| s.id != "round-button"));
    }

    #[test]
    fn unknown_and_cyclic_refs_fail() {
        let missing = r#"{"version":"2.21","children":[{"id":"a","type":"ref","ref":"nope"}]}"#;
        assert!(parse_design(missing.as_bytes())
            .unwrap_err()
            .to_string()
            .contains("Unknown component"));
        let cycle = r#"{
            "version":"2.21",
            "children":[
                {"id":"a","type":"ref","reusable":true,"ref":"b"},
                {"id":"b","type":"ref","reusable":true,"ref":"a"},
                {"id":"c","type":"ref","ref":"a"}
            ]
        }"#;
        assert!(parse_design(cycle.as_bytes())
            .unwrap_err()
            .to_string()
            .contains("Cyclic"));
    }
}
