// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

use axum::{
    extract::{Path, State},
    http::HeaderMap,
    response::Html,
};

use crate::{render, state::AppState};

pub async fn research_index_handler(
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Html<String> {
    let content = render::card::render_research_index(&state, "en");
    if is_fragment(&headers) {
        Html(content)
    } else {
        Html(render::shell::page_shell_lang(
            "Research",
            "/research",
            &content,
            &state,
            "en",
            Some("/es/research"),
        ))
    }
}

// Round 13 (2026-07-13): chrome-only translation, same rationale as
// key_plans.rs — essay titles/body stay English (these are the "Journals"
// the operator confirmed should never translate), but the surrounding page
// (nav, breadcrumb, kicker, empty-state) now reads in Spanish, and the
// language switch stays present instead of vanishing on click-through.
pub async fn research_index_handler_es(
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Html<String> {
    let content = render::card::render_research_index(&state, "es");
    if is_fragment(&headers) {
        Html(content)
    } else {
        Html(render::shell::page_shell_lang(
            "Investigación",
            "/es/research",
            &content,
            &state,
            "es",
            Some("/research"),
        ))
    }
}

pub async fn research_item_handler(
    Path(slug): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Html<String> {
    let content = render::card::render_research_item(&slug, &state, "en");
    if is_fragment(&headers) {
        Html(content)
    } else {
        Html(render::shell::page_shell_lang(
            &format!("{slug} — Research"),
            &format!("/research/{slug}"),
            &content,
            &state,
            "en",
            Some(&format!("/es/research/{slug}")),
        ))
    }
}

pub async fn research_item_handler_es(
    Path(slug): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Html<String> {
    let content = render::card::render_research_item(&slug, &state, "es");
    if is_fragment(&headers) {
        Html(content)
    } else {
        Html(render::shell::page_shell_lang(
            &format!("{slug} — Investigación"),
            &format!("/es/research/{slug}"),
            &content,
            &state,
            "es",
            Some(&format!("/research/{slug}")),
        ))
    }
}

pub async fn research_fragment(State(state): State<AppState>) -> Html<String> {
    Html(render::card::render_research_index(&state, "en"))
}

fn is_fragment(headers: &HeaderMap) -> bool {
    headers.get("x-fragment").is_some()
}
