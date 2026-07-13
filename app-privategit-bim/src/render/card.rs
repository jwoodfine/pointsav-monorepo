// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

use crate::{
    content::{self, Section},
    state::AppState,
};
use serde_json::Value;

use super::shell::{esc, t};

/// The real "Browse All BIM Objects" catalog index — every category grouped
/// under the four Sections the hero's callouts route to. This used to be
/// a stub that silently rendered the homepage template instead (the
/// `/tokens` dead-link bug found in the 2026-07-03 audit); it's now a
/// distinct page with anchors (`#taxonomy`, `#objects`, `#key-plans`,
/// `#context`) matching the hero's real-fact hotspot targets.
/// Section group heading in the requested language. English `Section::label()`
/// stays the id-slug source (so URLs/anchors never change with language);
/// only the displayed `<h2>` text is picked here. "Key Plans" is kept
/// English per the Round 11 Spanish terminology glossary (2026-07-12) — a
/// Woodfine coined proper noun, not translated.
fn section_label(section: Section, lang: &str) -> &'static str {
    match section {
        Section::Taxonomy => t(lang, "Taxonomy", "Taxonomía"),
        Section::Objects => t(lang, "Objects", "Objetos"),
        Section::Compositions => "Key Plans",
        Section::Context => t(lang, "Context", "Contexto"),
    }
}

pub fn render_tokens_index(state: &AppState, lang: &str) -> String {
    let mut groups = String::new();
    for (i, section) in Section::all().into_iter().enumerate() {
        let cards = render_category_cards_for_section(state, section, lang);
        groups.push_str(&format!(
            r#"<section id="{id}" class="bim-tokens-index__group">
  <div class="bim-tokens-sechead">
    <span class="bim-tokens-secnum">{num:02}</span>
    <h2>{label}</h2>
  </div>
  <div class="bim-category-grid">{cards}</div>
</section>"#,
            id = section.label().to_lowercase().replace(' ', "-"),
            num = i + 1,
            label = section_label(section, lang),
            cards = cards,
        ));
    }

    format!(
        r#"<div class="bim-tokens-index">
  <header class="bim-cat-pagehead">
    <span class="bim-cat-kicker">{kicker}</span>
    <h1>{title}</h1>
    <p class="bim-cat-pagehead__lede">{lede}</p>
  </header>
  {groups}
</div>"#,
        kicker = t(
            lang,
            "BIM Object registry · full taxonomy",
            "Registro de Objetos BIM · taxonomía completa",
        ),
        title = t(lang, "BIM Object Catalog", "Catálogo de Objetos BIM"),
        lede = t(
            lang,
            "Every BIM Object category, grouped by the four sections the homepage's Key Plan example routes to.",
            "Cada categoría de Objeto BIM, agrupada en las cuatro secciones a las que remite el ejemplo de Key Plan de la página de inicio.",
        ),
        groups = groups,
    )
}

/// Round 9 (2026-07-11) L2: `$description` fields in the DTCG token files
/// carry real spec content plus, in many entries, a trailing internal
/// maintenance note ("Amended 2026-07-03 per tool-buildingwidth-architecture.md
/// §Internal inconsistencies #5...") — the audit trail for this session's own
/// data-correction work, never meant for public reading. Filters by internal-
/// citation content (a `.md` filename, an internal doc name, "operator
/// decision/confirmed" process language) rather than by leading verb
/// ("Corrected"/"Amended"/...), because at least one real entry
/// (`performance.dtcg.json`'s IsFireExit description) starts with "Corrected"
/// but is itself the substantive, citable spec content — a verb-based filter
/// would have deleted it.
fn strip_internal_maintenance_notes(description: &str) -> String {
    const INTERNAL_MARKERS: [&str; 7] = [
        ".md",
        "§Internal inconsistencies",
        "cleanup-log",
        "operator decision",
        "operator-confirmed",
        "operator confirmed",
        "tool-buildingwidth-architecture",
    ];
    let kept: Vec<&str> = description
        .split(". ")
        .filter(|sentence| {
            let s = sentence.trim_start();
            !INTERNAL_MARKERS.iter().any(|m| sentence.contains(m))
                && !s.starts_with("Was previously")
        })
        .collect();
    let mut out = kept.join(". ");
    if !out.is_empty() && !out.ends_with(['.', '!', '?']) {
        out.push('.');
    }
    out
}

pub fn render_token_page(category: &str, state: &AppState, lang: &str) -> String {
    let meta = state.categories.iter().find(|c| c.slug == category);

    let Some(file_val) = state.tokens.get(category) else {
        return format!(
            r#"<div class="bim-empty"><p>No token file found for category <code>{}</code>.</p></div>"#,
            esc(category)
        );
    };

    let bim = match file_val.get("bim").and_then(|v| v.as_object()) {
        Some(b) => b,
        None => {
            return format!(
                r#"<div class="bim-empty"><p>Token file for <code>{}</code> has no 'bim' root.</p></div>"#,
                esc(category)
            );
        }
    };

    // Only the lede/intro is Tier-1 scope this round (Round 11, 2026-07-12)
    // — the entity table, property sets, and IFC/Uniclass codes below stay
    // English-only regardless of `lang` (out of scope per the approved
    // plan's "Tokens-per-entity-spec-data pages" carve-out).
    let intro_html = meta
        .and_then(|m| {
            if lang == "es" {
                m.intro_html_es.as_deref()
            } else {
                Some(m.intro_html.as_str())
            }
        })
        .or_else(|| meta.map(|m| m.intro_html.as_str()))
        .unwrap_or("");
    let ifc_anchor = meta.map(|m| m.ifc_anchor.as_str()).unwrap_or("");
    let elements = meta.map(|m| m.elements.as_str()).unwrap_or("");
    let uniclass = meta.map(|m| m.uniclass.as_str()).unwrap_or("—");
    let ifc_hierarchy = meta.map(|m| m.ifc_hierarchy.as_str()).unwrap_or("—");
    let empty_psets = Vec::new();
    let property_sets = meta.map(|m| &m.property_sets).unwrap_or(&empty_psets);

    let multi_group = bim.keys().filter(|k| !k.starts_with('$')).count() > 1;
    let mut entity_count = 0usize;
    let mut rows = String::new();
    for (cat_key, cat_val) in bim {
        if let Some(entities) = cat_val.as_object() {
            let mut slugs: Vec<&String> = entities.keys().filter(|k| !k.starts_with('$')).collect();
            slugs.sort();
            for slug in slugs {
                entity_count += 1;
                let entity = &entities[slug];
                // Explicit placeholder, not a blank cell (2026-07-03 audit:
                // an empty <td> reads as a broken/half-populated table —
                // "—" makes the sparseness a legible fact about the data,
                // not something that looks like a rendering bug).
                let raw_description = entity
                    .get("$description")
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                let stripped_description = strip_internal_maintenance_notes(raw_description);
                let description = if stripped_description.is_empty() {
                    "—"
                } else {
                    stripped_description.as_str()
                };
                let ifc_class = entity
                    .get("$value")
                    .and_then(|v| v.get("ifc_class"))
                    .and_then(|v| v.as_str())
                    .unwrap_or("—");
                let display_slug = if multi_group {
                    format!("{cat_key}/{slug}")
                } else {
                    slug.clone()
                };
                rows.push_str(&format!(
                    r#"<tr>
  <td><code>{slug}</code></td>
  <td><code>{ifc_class}</code></td>
  <td>{description}</td>
</tr>"#,
                    slug = esc(&display_slug),
                    ifc_class = esc(ifc_class),
                    description = esc(description),
                ));
            }
        }
    }

    let mut pset_rows = String::new();
    for (pset, prop, ty) in property_sets {
        pset_rows.push_str(&format!(
            r#"<tr><td><code>{pset}</code></td><td><code>{prop}</code></td><td><code>{ty}</code></td></tr>"#,
            pset = esc(pset),
            prop = esc(prop),
            ty = esc(ty),
        ));
    }
    let pset_block = if pset_rows.is_empty() {
        r#"<p class="bim-empty">No property sets registered for this category yet.</p>"#.to_string()
    } else {
        format!(
            r#"<table class="bim-table-wrap bim-token-table">
  <thead><tr><th>Property set</th><th>Property</th><th>Type</th></tr></thead>
  <tbody>{pset_rows}</tbody>
</table>"#
        )
    };

    let dtcg_json = serde_json::to_string_pretty(file_val).unwrap_or_default();

    let uniclass_chip = if uniclass == "—" {
        String::new()
    } else {
        format!(
            r#"<span class="bim-cat-chip bim-cat-chip--pr"><span class="bim-cat-chip__lv">Uniclass</span>{}</span>"#,
            esc(uniclass)
        )
    };
    let uniclass_row = if uniclass == "—" {
        String::new()
    } else {
        format!("<tr><th>Uniclass 2015</th><td>{}</td></tr>", esc(uniclass))
    };

    format!(
        r#"<div class="bim-category-page">
  <div class="bim-breadcrumbs">
    <a href="/" data-path="/" class="bim-nav-link">{home}</a> / <a href="/tokens" data-path="/tokens" class="bim-nav-link">BIM Objects</a>
  </div>
  <header class="bim-cat-pagehead">
    <span class="bim-cat-kicker">IFC 4.3 · Uniclass 2015</span>
    <h1>{display_name}</h1>
    <div class="bim-chip-row">
      <span class="bim-cat-chip bim-cat-chip--plain"><span class="bim-cat-chip__lv">IFC</span><code>{ifc_anchor}</code></span>
      {uniclass_chip}
    </div>
  </header>

  <details class="bim-spec-card" open>
    <summary>Specification</summary>
    <div class="bim-spec-card__body">
      <div class="bim-intro">{intro_html}</div>
      <p class="bim-elements"><code>{elements}</code></p>
      <table class="bim-cat-spectable bim-detail-spectable">
        <tr><th>IFC entity</th><td><code>{ifc_anchor}</code></td></tr>
        {uniclass_row}
        <tr><th>IFC hierarchy</th><td class="bim-ifc-hierarchy"><code>{ifc_hierarchy}</code></td></tr>
      </table>
      <h2>Applicable property sets</h2>
      {pset_block}
    </div>
  </details>

  <details class="bim-spec-card" open>
    <summary>BIM Objects ({entity_count})</summary>
    <div class="bim-spec-card__body">
      <table class="bim-table-wrap bim-token-table">
        <thead>
          <tr>
            <th>Token slug</th>
            <th>IFC class</th>
            <th>Description</th>
          </tr>
        </thead>
        <tbody>{rows}</tbody>
      </table>
    </div>
  </details>

  <details class="bim-accordion">
    <summary>Regulation</summary>
    <div class="bim-spec-card__body"><p class="bim-empty">No regulatory overlays registered for this category.</p></div>
  </details>
  <details class="bim-accordion">
    <summary>Climate Zone</summary>
    <div class="bim-spec-card__body"><p class="bim-empty">No climate zone constraints modeled for this category.</p></div>
  </details>
  <details class="bim-accordion">
    <summary>Token Format</summary>
    <div class="bim-spec-card__body"><pre><code>{dtcg_json}</code></pre></div>
  </details>
</div>"#,
        home = t(lang, "Home", "Inicio"),
        display_name = esc(meta.map(|m| m.display_name.as_str()).unwrap_or(category)),
        intro_html = intro_html,
        ifc_anchor = esc(ifc_anchor),
        elements = esc(elements),
        uniclass_chip = uniclass_chip,
        uniclass_row = uniclass_row,
        ifc_hierarchy = esc(ifc_hierarchy),
        pset_block = pset_block,
        entity_count = entity_count,
        rows = rows,
        dtcg_json = esc(&dtcg_json),
    )
}

pub fn render_research_index(state: &AppState) -> String {
    let research_dir = state.config.vault_dir.join("research");
    let mut items = String::new();
    if let Ok(rd) = std::fs::read_dir(&research_dir) {
        let mut names: Vec<String> = rd
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension().and_then(|s| s.to_str()) == Some("md"))
            .filter_map(|e| {
                e.path()
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .map(|s| s.to_string())
            })
            .collect();
        names.sort();
        for slug in &names {
            let title = std::fs::read_to_string(research_dir.join(format!("{slug}.md")))
                .ok()
                .and_then(|content| {
                    content
                        .lines()
                        .find_map(|l| l.strip_prefix("# ").map(|t| t.trim().to_string()))
                })
                .unwrap_or_else(|| slug.replace('-', " "));
            items.push_str(&format!(
                r#"<div class="bim-research-item">
  <a href="/research/{slug}" data-path="/research/{slug}" class="bim-nav-link">
    <span class="bim-research-item__title">{title}</span>
    <span class="bim-research-item__slug">/research/{slug}</span>
  </a>
</div>"#,
                slug = esc(slug),
                title = esc(&title),
            ));
        }
    }
    if items.is_empty() {
        items = r#"<p class="bim-empty">No research documents found.</p>"#.into();
    }
    format!(
        r#"<div class="bim-research">
  <header class="bim-cat-pagehead">
    <span class="bim-cat-kicker">Research backplane</span>
    <h1>Research</h1>
    <p class="bim-cat-pagehead__lede">Background notes and working documents behind the BIM Object Library's classifications and methodology.</p>
  </header>
  <div class="bim-research-list">{items}</div>
</div>"#,
        items = items,
    )
}

pub fn render_research_item(slug: &str, state: &AppState) -> String {
    let path = state
        .config
        .vault_dir
        .join("research")
        .join(format!("{slug}.md"));
    let raw = match std::fs::read_to_string(&path) {
        Ok(s) => s,
        Err(_) => {
            return format!(
                r#"<div class="bim-empty"><p>Research document <code>{}</code> not found.</p></div>"#,
                esc(slug)
            )
        }
    };
    let html_body = content::render_markdown(&raw);
    format!(
        r#"<div class="bim-research-item-page">
  <div class="bim-breadcrumbs">
    <a href="/research" data-path="/research" class="bim-nav-link">Research</a> / <span>{slug}</span>
  </div>
  <div class="bim-markdown">{html_body}</div>
</div>"#,
        slug = esc(slug),
        html_body = html_body,
    )
}

fn render_category_cards_for_section(state: &AppState, section: Section, lang: &str) -> String {
    let mut out = String::new();
    for cat in state.categories.iter().filter(|c| c.section == section) {
        let count = count_entities_in_file(state, &cat.slug);
        out.push_str(&format!(
            r#"<a class="bim-category-card bim-nav-link" href="/tokens/{slug}" data-path="/tokens/{slug}">
  <span class="bim-category-card-name">{display}</span>
  <span class="bim-category-card-count">{count} {entity_word}</span>
</a>"#,
            slug = cat.slug,
            display = esc(&cat.display_name),
            count = count,
            entity_word = if count == 1 {
                t(lang, "entity", "entidad")
            } else {
                t(lang, "entities", "entidades")
            },
        ));
    }
    out
}

/// Recursively walk a DTCG object collecting every node that carries a
/// `$value` field, regardless of nesting depth — key-plans.dtcg.json nests
/// three levels deep (category -> subcategory -> size variant); other files
/// nest two. `slug` is set to the object key one level above the leaf.
pub(crate) fn collect_kp_leaves<'a>(
    obj: &'a serde_json::Map<String, Value>,
    out: &mut Vec<(&'a str, &'a Value)>,
) {
    for (key, val) in obj {
        if key == "$description" {
            continue;
        }
        if val.get("$value").is_some() {
            out.push((key.as_str(), val));
        } else if let Some(child) = val.as_object() {
            collect_kp_leaves(child, out);
        }
    }
}

fn count_entities_in_file(state: &AppState, category: &str) -> usize {
    let Some(file_val) = state.tokens.get(category) else {
        return 0;
    };
    let Some(bim) = file_val.get("bim").and_then(|v| v.as_object()) else {
        return 0;
    };
    bim.values()
        .filter_map(|v| v.as_object())
        .flat_map(|o| o.iter())
        .filter(|(k, _)| !k.starts_with('$'))
        .count()
}
