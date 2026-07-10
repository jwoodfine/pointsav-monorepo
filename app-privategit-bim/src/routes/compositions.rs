// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::Html;
use std::collections::HashMap;

use crate::{render, state::AppState};

pub async fn compositions_index_handler(
    Query(params): Query<HashMap<String, String>>,
    State(state): State<AppState>,
) -> Html<String> {
    let q = params.get("q").cloned().unwrap_or_default();
    let use_case = params.get("use").map(String::as_str);
    let layout = params.get("layout").map(String::as_str);
    let content = render::catalog::render_compositions_index(&state, &q, use_case, layout);
    Html(render::shell::page_shell(
        "Compositions",
        "/compositions",
        &content,
        &state,
    ))
}

fn not_found(state: &AppState) -> (StatusCode, Html<String>) {
    (
        StatusCode::NOT_FOUND,
        Html(render::shell::page_shell(
            "Not found",
            "",
            &render::catalog::render_not_found(),
            state,
        )),
    )
}

pub async fn composition_detail_handler(
    Path(slug): Path<String>,
    State(state): State<AppState>,
) -> Result<Html<String>, (StatusCode, Html<String>)> {
    match render::catalog::render_composition_detail(&state, &slug, None) {
        Some(content) => Ok(Html(render::shell::page_shell(
            "Composition",
            &format!("/compositions/{slug}"),
            &content,
            &state,
        ))),
        None => Err(not_found(&state)),
    }
}

pub async fn composition_object_handler(
    Path((slug, object_slug)): Path<(String, String)>,
    State(state): State<AppState>,
) -> Result<Html<String>, (StatusCode, Html<String>)> {
    match render::catalog::render_composition_detail(&state, &slug, Some(&object_slug)) {
        Some(content) => Ok(Html(render::shell::page_shell(
            "Composition",
            &format!("/compositions/{slug}/o/{object_slug}"),
            &content,
            &state,
        ))),
        None => Err(not_found(&state)),
    }
}
