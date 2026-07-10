// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! Hand-authored architectural plan-view symbols for the 7 BIM Objects in
//! the furniture catalog — the "Plan Room" object-card treatment (see
//! BRIEF-bim-v3-hyperscaler-redesign.md, `object_card_treatment`). Each
//! symbol is a standard top-view drafting convention drawn as inline SVG
//! using two line weights via `.bim-plan-primary` / `.bim-plan-secondary`
//! (which resolve to `var(--bim-accent)` / `var(--bim-accent-active)` in
//! bim-planroom.css) so every thumbnail is theme-aware for free — no fills
//! outside the shared token palette. Replaces the letter-monogram
//! `bim-cat-thumb__glyph` placeholder.
//!
//! Keyed by the furniture group + slug from `interior.dtcg.json` (see
//! `render::catalog::build_objects`). One symbol per real object in the
//! catalog — not a generic per-IFC-class icon set — so a desk is drawn as
//! Woodfine's actual desk, not a generic desk silhouette.

use super::shell::esc;

/// A single drafted dimension string, e.g. "⊢ 1473 ⊣", from the object's
/// real width in millimetres. Falls back to an em-dash when no width is on
/// record — never a fabricated number.
pub fn dim_annotation(width_mm: Option<i64>) -> String {
    match width_mm {
        Some(w) => format!("⊢ {w} ⊣"),
        None => "⊢ — ⊣".to_string(),
    }
}

/// Render the plan symbol for one furniture `group`/`slug` pair. `dim_label`
/// is the drafted dimension annotation (see `dim_annotation`) set in
/// Source Code Pro along the symbol's baseline. Falls back to a generic
/// bounding-box symbol for any group not hand-drawn (none currently — all 7
/// real catalog objects are covered).
pub fn plan_symbol_svg(group: &str, slug: &str, dim_label: &str) -> String {
    let body = match (group, slug) {
        ("desk", _) => desk_symbol(),
        ("task-chair", _) => task_chair_symbol(),
        ("table", _) => round_table_symbol(),
        ("storage", s) if s.contains("bookcase") => bookcase_symbol(),
        ("storage", _) => mobile_pedestal_symbol(),
        ("lounge-chair", _) => wing_chair_symbol(),
        ("utility", _) => coat_rack_symbol(),
        _ => generic_symbol(),
    };
    format!(
        r##"<svg class="bim-plan-symbol" viewBox="0 0 120 120" xmlns="http://www.w3.org/2000/svg" role="img" aria-hidden="true">
  <rect x="1" y="1" width="118" height="118" class="bim-plan-frame" fill="none"/>
  {body}
  <text x="60" y="112" class="bim-plan-dim" text-anchor="middle">{dim}</text>
</svg>"##,
        body = body,
        dim = esc(dim_label),
    )
}

// ── Desk: worksurface rectangle + return + cable grommet tick ──────────────
fn desk_symbol() -> String {
    r##"<rect x="18" y="30" width="70" height="34" class="bim-plan-primary" fill="none"/>
  <rect x="18" y="64" width="30" height="26" class="bim-plan-primary" fill="none"/>
  <circle cx="70" cy="47" r="3.2" class="bim-plan-secondary" fill="none"/>
  <line x1="18" y1="64" x2="88" y2="64" class="bim-plan-secondary"/>"##
        .to_string()
}

// ── Task chair: universal plan-chair — seat square + backrest arc ──────────
fn task_chair_symbol() -> String {
    r##"<rect x="38" y="46" width="34" height="30" rx="3" class="bim-plan-primary" fill="none"/>
  <path d="M36 46 A30 30 0 0 1 74 46" class="bim-plan-primary" fill="none"/>
  <circle cx="55" cy="61" r="2" class="bim-plan-secondary" fill="none"/>"##
        .to_string()
}

// ── Round table: circle + four chair arcs ───────────────────────────────────
fn round_table_symbol() -> String {
    r##"<circle cx="55" cy="55" r="26" class="bim-plan-primary" fill="none"/>
  <path d="M55 17 A38 38 0 0 1 79 27" class="bim-plan-secondary" fill="none"/>
  <path d="M93 55 A38 38 0 0 1 83 79" class="bim-plan-secondary" fill="none"/>
  <path d="M55 93 A38 38 0 0 1 31 83" class="bim-plan-secondary" fill="none"/>
  <path d="M17 55 A38 38 0 0 1 27 31" class="bim-plan-secondary" fill="none"/>"##
        .to_string()
}

// ── Bookcase: shelving rectangle with depth hatching ────────────────────────
fn bookcase_symbol() -> String {
    r##"<rect x="16" y="42" width="78" height="22" class="bim-plan-primary" fill="none"/>
  <line x1="24" y1="42" x2="16" y2="64" class="bim-plan-secondary"/>
  <line x1="38" y1="42" x2="30" y2="64" class="bim-plan-secondary"/>
  <line x1="52" y1="42" x2="44" y2="64" class="bim-plan-secondary"/>
  <line x1="66" y1="42" x2="58" y2="64" class="bim-plan-secondary"/>
  <line x1="80" y1="42" x2="72" y2="64" class="bim-plan-secondary"/>
  <line x1="94" y1="42" x2="86" y2="64" class="bim-plan-secondary"/>"##
        .to_string()
}

// ── Mobile pedestal: small rectangle, drawer dashes, castor ticks ──────────
fn mobile_pedestal_symbol() -> String {
    r##"<rect x="38" y="34" width="30" height="46" class="bim-plan-primary" fill="none"/>
  <line x1="42" y1="46" x2="64" y2="46" class="bim-plan-secondary"/>
  <line x1="42" y1="57" x2="64" y2="57" class="bim-plan-secondary"/>
  <line x1="42" y1="68" x2="64" y2="68" class="bim-plan-secondary"/>
  <circle cx="41" cy="83" r="2" class="bim-plan-secondary" fill="none"/>
  <circle cx="65" cy="83" r="2" class="bim-plan-secondary" fill="none"/>"##
        .to_string()
}

// ── Wing chair: wide seat + backrest arc + flared wing tabs ────────────────
fn wing_chair_symbol() -> String {
    r##"<rect x="30" y="46" width="48" height="34" rx="3" class="bim-plan-primary" fill="none"/>
  <path d="M27 46 A36 36 0 0 1 81 46" class="bim-plan-primary" fill="none"/>
  <path d="M27 46 L18 34" class="bim-plan-secondary"/>
  <path d="M81 46 L90 34" class="bim-plan-secondary"/>
  <circle cx="54" cy="63" r="2" class="bim-plan-secondary" fill="none"/>"##
        .to_string()
}

// ── Coat rack: post symbol + radiating hook ticks ───────────────────────────
fn coat_rack_symbol() -> String {
    r##"<circle cx="55" cy="55" r="5" class="bim-plan-primary" fill="none"/>
  <line x1="55" y1="55" x2="55" y2="25" class="bim-plan-secondary"/>
  <line x1="55" y1="55" x2="79" y2="41" class="bim-plan-secondary"/>
  <line x1="55" y1="55" x2="79" y2="69" class="bim-plan-secondary"/>
  <line x1="55" y1="55" x2="55" y2="85" class="bim-plan-secondary"/>
  <line x1="55" y1="55" x2="31" y2="69" class="bim-plan-secondary"/>
  <line x1="55" y1="55" x2="31" y2="41" class="bim-plan-secondary"/>"##
        .to_string()
}

// ── Fallback: plain bounding-box rectangle for any ungrouped object ────────
fn generic_symbol() -> String {
    r##"<rect x="24" y="34" width="62" height="42" class="bim-plan-primary" fill="none"/>"##.to_string()
}
