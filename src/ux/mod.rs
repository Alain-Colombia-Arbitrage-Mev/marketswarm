//! Deterministic UX/UI audit of pen.dev documents.
//!
//! Specialists below are in-process and reproducible. A neural critic and
//! synthesis are optional and never replace measured contrast.

mod audit;
mod contrast;
mod pen;

pub use audit::{audit, enrich, render, specialist_catalog, AuditOptions, Screenshot, UxReport};
pub use contrast::{contrast_ratio, parse_hex, passes_aa};
