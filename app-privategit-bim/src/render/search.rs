// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

use crate::state::AppState;
use serde_json::Value;

use super::shell::{esc, t};

struct Hit {
    kind: &'static str,
    title: String,
    url: String,
    snippet: String,
    score: i32,
    tiebreak: String,
}

/// Case-insensitive multi-word match: every query token must appear as a
/// substring in at least one of the given fields (AND across tokens, OR
/// across fields per token). Returns a score if it matches, None if not.
fn score_match(tokens: &[String], fields: &[(&str, i32)]) -> Option<i32> {
    let mut total = 0;
    for tok in tokens {
        let mut found = false;
        for (field, weight) in fields {
            if field.to_lowercase().contains(tok) {
                total += weight;
                found = true;
            }
        }
        if !found {
            return None;
        }
    }
    Some(total)
}

/// Strip the most common Markdown syntax markers so search snippets read as
/// plain prose instead of leaking '**', '#', '`', etc.
fn strip_markdown(md: &str) -> String {
    md.lines()
        .map(|l| l.trim_start_matches('#').trim_start_matches('>').trim())
        .collect::<Vec<_>>()
        .join(" ")
        .replace("**", "")
        .replace('`', "")
        .replace("*", "")
}

/// Wrap the first case-insensitive occurrence of any query token in <mark>,
/// windowed to a reasonable snippet length around the match.
fn highlight_snippet(text: &str, tokens: &[String]) -> String {
    let lower = text.to_lowercase();
    let mut best: Option<usize> = None;
    for tok in tokens {
        if let Some(pos) = lower.find(tok.as_str()) {
            best = Some(best.map_or(pos, |b| b.min(pos)));
        }
    }
    let Some(pos) = best else {
        let clipped: String = text.chars().take(160).collect();
        return esc(&clipped);
    };
    let window = 90usize;
    let start = pos.saturating_sub(window);
    let end = (pos + window).min(text.len());
    // Snap to char boundaries.
    let start = (start..=pos)
        .find(|i| text.is_char_boundary(*i))
        .unwrap_or(0);
    let end = (end..text.len().min(end + 4))
        .find(|i| text.is_char_boundary(*i))
        .unwrap_or(text.len());
    let prefix = if start > 0 { "…" } else { "" };
    let suffix = if end < text.len() { "…" } else { "" };
    let slice = &text[start..end];
    let slice_lower = slice.to_lowercase();
    let mut out = String::new();
    let mut i = 0usize;
    while i < slice.len() {
        let mut matched_len = 0usize;
        for tok in tokens {
            if !tok.is_empty() && slice_lower[i..].starts_with(tok.as_str()) {
                matched_len = matched_len.max(tok.len());
            }
        }
        if matched_len > 0 && slice.is_char_boundary(i + matched_len) {
            out.push_str("<mark>");
            out.push_str(&esc(&slice[i..i + matched_len]));
            out.push_str("</mark>");
            i += matched_len;
        } else {
            let next = (i + 1..=slice.len())
                .find(|j| slice.is_char_boundary(*j))
                .unwrap_or(slice.len());
            out.push_str(&esc(&slice[i..next]));
            i = next;
        }
    }
    format!("{prefix}{out}{suffix}")
}

/// English kind key -> display label. Kept as a translate-at-render step
/// rather than storing a localized string on `Hit`, since the same hit list
/// is scored/sorted once regardless of `lang`.
fn kind_label<'a>(kind: &'a str, lang: &str) -> &'a str {
    match kind {
        "Category" => t(lang, "Category", "Categoría"),
        "Object" => t(lang, "Object", "Objeto"),
        "Key Plan" => "Key Plan",
        "Research" => t(lang, "Research", "Investigación"),
        _ => kind,
    }
}

pub fn render_search_results(query: &str, state: &AppState, lang: &str) -> String {
    let trimmed = query.trim();
    if trimmed.is_empty() {
        return format!(
            r#"<div class="bim-search-page">
  <header class="bim-cat-pagehead">
    <span class="bim-cat-kicker">{kicker}</span>
    <h1>{title}</h1>
  </header>
  <p class="bim-empty">{hint}</p>
</div>"#,
            kicker = t(lang, "Catalog search", "Búsqueda en el catálogo"),
            title = t(lang, "Search", "Buscar"),
            hint = t(
                lang,
                "Enter a search term above — categories, entity slugs, IFC classes, and research articles are all searched.",
                "Escriba un término de búsqueda arriba — se buscan categorías, slugs de entidad, clases IFC y artículos de investigación.",
            ),
        );
    }

    let tokens: Vec<String> = trimmed
        .split_whitespace()
        .map(|t| t.to_lowercase())
        .collect();

    let mut hits: Vec<Hit> = Vec::new();

    // Categories
    for cat in state.categories.iter() {
        let fields = [
            (cat.display_name.as_str(), 100),
            (cat.card_desc.as_str(), 10),
            (cat.ifc_anchor.as_str(), 30),
        ];
        if let Some(score) = score_match(&tokens, &fields) {
            let url = if lang == "es" {
                format!("/es/tokens/{}", cat.slug)
            } else {
                format!("/tokens/{}", cat.slug)
            };
            hits.push(Hit {
                kind: "Category",
                title: cat.display_name.clone(),
                url,
                snippet: highlight_snippet(&cat.card_desc, &tokens),
                score,
                tiebreak: cat.slug.clone(),
            });
        }
    }

    // Objects + Key Plans — the same canonical catalog the /objects and
    // /key-plans routes render from (2026-07-09 fix). This used to walk
    // raw per-file DTCG entities directly via a local `collect_entities`
    // helper: it hardcoded `kind: "BIM Object"` for every hit
    // regardless of which file it came from — mis-badging Key Plans found
    // via key-plans.dtcg.json — and read a generic `$value.display_name`
    // field that Objects don't actually carry (their real name lives in
    // `model`, per catalog::build_objects), silently falling back to the raw
    // registry slug instead of the product name. Reusing
    // catalog::build_objects/build_key_plans gets the same name/kind
    // resolution the catalog pages already have right, instead of
    // re-deriving it here with different (wrong) field assumptions.
    let objects = super::catalog::build_objects(state);
    let key_plans = super::catalog::build_key_plans(state, &objects);

    for o in &objects {
        let name = o.get("name").and_then(Value::as_str).unwrap_or("");
        let id = o.get("id").and_then(Value::as_str).unwrap_or("");
        let manufacturer = o.get("manufacturer").and_then(Value::as_str).unwrap_or("");
        let ifc_class = o.get("ifc_class").and_then(Value::as_str).unwrap_or("");
        let description = o.get("description").and_then(Value::as_str).unwrap_or("");
        let fields = [
            (name, 80),
            (id, 60),
            (manufacturer, 30),
            (ifc_class, 40),
            (description, 8),
        ];
        if let Some(score) = score_match(&tokens, &fields) {
            let snippet_source = if description.is_empty() {
                ifc_class
            } else {
                description
            };
            let url = if lang == "es" {
                format!("/es/objects/{id}")
            } else {
                format!("/objects/{id}")
            };
            hits.push(Hit {
                kind: "Object",
                title: name.to_string(),
                url,
                snippet: highlight_snippet(snippet_source, &tokens),
                score,
                tiebreak: id.to_string(),
            });
        }
    }

    for c in &key_plans {
        let name = c.get("name").and_then(Value::as_str).unwrap_or("");
        let id = c.get("id").and_then(Value::as_str).unwrap_or("");
        let slug = c.get("slug").and_then(Value::as_str).unwrap_or("");
        let cat_label = c
            .get("category_label")
            .and_then(Value::as_str)
            .unwrap_or("");
        let description = c.get("description").and_then(Value::as_str).unwrap_or("");
        let fields = [(name, 80), (id, 60), (cat_label, 30), (description, 8)];
        if let Some(score) = score_match(&tokens, &fields) {
            let snippet_source = if description.is_empty() {
                cat_label
            } else {
                description
            };
            let url = if lang == "es" {
                format!("/es/key-plans/{slug}")
            } else {
                format!("/key-plans/{slug}")
            };
            hits.push(Hit {
                kind: "Key Plan",
                title: name.to_string(),
                url,
                snippet: highlight_snippet(snippet_source, &tokens),
                score,
                tiebreak: slug.to_string(),
            });
        }
    }

    // Research articles (mirrors render_research_index's own disk-read
    // pattern — kept separate from the in-memory category/entity search
    // above rather than forcing both onto one abstraction).
    let research_dir = state.config.vault_dir.join("research");
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
            let Ok(raw) = std::fs::read_to_string(research_dir.join(format!("{slug}.md"))) else {
                continue;
            };
            let title = raw
                .lines()
                .find_map(|l| l.strip_prefix("# ").map(|t| t.trim().to_string()))
                .unwrap_or_else(|| slug.replace('-', " "));
            let body = strip_markdown(&raw);
            let fields = [(title.as_str(), 90), (body.as_str(), 6)];
            if let Some(score) = score_match(&tokens, &fields) {
                let url = if lang == "es" {
                    format!("/es/research/{slug}")
                } else {
                    format!("/research/{slug}")
                };
                hits.push(Hit {
                    kind: "Research",
                    title: title.clone(),
                    url,
                    snippet: highlight_snippet(&body, &tokens),
                    score,
                    tiebreak: slug.clone(),
                });
            }
        }
    }

    hits.sort_by(|a, b| {
        b.score
            .cmp(&a.score)
            .then_with(|| a.tiebreak.cmp(&b.tiebreak))
    });

    let count = hits.len();
    let mut results_html = String::new();
    for hit in &hits {
        results_html.push_str(&format!(
            r#"<a class="bim-search-result" href="{url}" data-path="{url}">
  <span class="bim-search-result__kind">{kind}</span>
  <span class="bim-search-result__title">{title}</span>
  <span class="bim-search-result__snippet">{snippet}</span>
</a>"#,
            url = esc(&hit.url),
            kind = esc(kind_label(hit.kind, lang)),
            title = esc(&hit.title),
            snippet = hit.snippet,
        ));
    }
    if results_html.is_empty() {
        let tokens_path = if lang == "es" { "/es/tokens" } else { "/tokens" };
        results_html = format!(
            r#"<p class="bim-empty">{pre} &ldquo;{q}&rdquo;. {try_other}, {or} <a href="{tokens_path}" data-path="{tokens_path}">{browse}</a>.</p>"#,
            pre = t(lang, "No results for", "Sin resultados para"),
            q = esc(trimmed),
            try_other = t(lang, "Try a different term", "Intente con otro término"),
            or = t(lang, "or", "o"),
            tokens_path = tokens_path,
            browse = t(lang, "browse all categories", "explore todas las categorías"),
        );
    }

    format!(
        r#"<div class="bim-search-page">
  <header class="bim-cat-pagehead">
    <span class="bim-cat-kicker">{kicker}</span>
    <h1>{title}</h1>
    <p class="bim-cat-pagehead__lede">{count} {result_word} {for_word} &ldquo;{q}&rdquo;</p>
  </header>
  <div class="bim-search-results">{results_html}</div>
</div>"#,
        kicker = t(lang, "Catalog search", "Búsqueda en el catálogo"),
        title = t(lang, "Search results", "Resultados de búsqueda"),
        count = count,
        result_word = if lang == "es" {
            "resultado(s)"
        } else if count == 1 {
            "result"
        } else {
            "results"
        },
        for_word = t(lang, "for", "para"),
        q = esc(trimmed),
        results_html = results_html,
    )
}
