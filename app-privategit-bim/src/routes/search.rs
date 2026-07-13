// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

use axum::extract::{Query, State};
use axum::response::Html;
use std::collections::HashMap;

use crate::{render, state::AppState};

pub async fn search_handler(
    Query(params): Query<HashMap<String, String>>,
    State(state): State<AppState>,
) -> Html<String> {
    let query = params.get("q").cloned().unwrap_or_default();
    let content = render::search::render_search_results(&query, &state, "en");
    Html(render::shell::page_shell_lang(
        "Search", "/search", &content, &state, "en", Some("/es/search"),
    ))
}

pub async fn search_handler_es(
    Query(params): Query<HashMap<String, String>>,
    State(state): State<AppState>,
) -> Html<String> {
    let query = params.get("q").cloned().unwrap_or_default();
    let content = render::search::render_search_results(&query, &state, "es");
    Html(render::shell::page_shell_lang(
        "Buscar", "/es/search", &content, &state, "es", Some("/search"),
    ))
}
