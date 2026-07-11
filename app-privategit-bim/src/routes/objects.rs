// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

use axum::extract::{Path, Query, RawQuery, State};
use axum::http::StatusCode;
use axum::response::Html;
use std::collections::HashMap;

use crate::{render, state::AppState};

/// Parse repeated `ids=` query params (native checkbox `<form method="get">`
/// submission — `?ids=a&ids=b&ids=c`) directly from the raw query string.
///
/// axum's default `Query<T>` extractor is backed by `serde_urlencoded`,
/// which does NOT collect repeated keys into a `Vec` — it has no concept of
/// a repeated key at all, so a `#[derive(Deserialize)] struct { ids: Vec<String> }`
/// fails to deserialize `?ids=a&ids=b` and the route 400s (found live via the
/// 2026-07-09 Round 3 browser verification pass, which is how this got
/// caught — the frontend form and this comment's *previous* claim that
/// `Query<CompareParams>` "deserializes cleanly into a Vec" were both wrong).
/// Parsing the raw string ourselves sidesteps the limitation without adding
/// a `serde_qs` dependency for one field.
fn parse_repeated_ids(raw: Option<&str>) -> Vec<String> {
    let Some(raw) = raw else { return Vec::new() };
    raw.split('&')
        .filter_map(|pair| pair.strip_prefix("ids="))
        .map(|v| {
            percent_encoding::percent_decode_str(v)
                .decode_utf8_lossy()
                .replace('+', " ")
        })
        .filter(|v| !v.is_empty())
        .collect()
}

pub async fn objects_index_handler(
    Query(params): Query<HashMap<String, String>>,
    State(state): State<AppState>,
) -> Html<String> {
    let q = params.get("q").cloned().unwrap_or_default();
    let uni = params.get("uni").map(String::as_str);
    let mfr = params.get("mfr").map(String::as_str);
    let content = render::catalog::render_objects_index(&state, &q, uni, mfr);
    Html(render::shell::page_shell(
        "Objects", "/objects", &content, &state,
    ))
}

/// Dimensions-scoped compare (item 8, 2026-07-09). `ids` comes from the
/// compare form's checkboxes (`<input type="checkbox" name="ids" ...>`,
/// native `method="get"` submission — repeated `?ids=a&ids=b&ids=c`) — real,
/// shareable, reloadable, same discipline as every other filtered/faceted
/// view in this catalog (no client-only state), and works with JavaScript
/// disabled: the plain submit button always submits the form as-is.
pub async fn objects_compare_handler(
    RawQuery(raw): RawQuery,
    State(state): State<AppState>,
) -> Html<String> {
    let ids = parse_repeated_ids(raw.as_deref());
    let content = render::catalog::render_objects_compare(&state, &ids);
    Html(render::shell::page_shell(
        "Compare Objects",
        "/objects",
        &content,
        &state,
    ))
}

pub async fn object_detail_handler(
    Path(slug): Path<String>,
    State(state): State<AppState>,
) -> Result<Html<String>, (StatusCode, Html<String>)> {
    match render::catalog::render_object_detail(&state, &slug) {
        Some((name, content)) => Ok(Html(render::shell::page_shell(
            &name,
            &format!("/objects/{slug}"),
            &content,
            &state,
        ))),
        None => Err((
            StatusCode::NOT_FOUND,
            Html(render::shell::page_shell(
                "Not found",
                "",
                &render::catalog::render_not_found(),
                &state,
            )),
        )),
    }
}
