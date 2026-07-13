// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

use axum::{
    extract::{Path, State},
    http::HeaderMap,
    response::Html,
};

use crate::{render, render::shell::t, state::AppState};

pub async fn tokens_index_handler(
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Html<String> {
    let content = render::card::render_tokens_index(&state, "en");
    if is_fragment(&headers) {
        Html(content)
    } else {
        Html(render::shell::page_shell_lang(
            "BIM Object Catalog",
            "/tokens",
            &content,
            &state,
            "en",
            Some("/es/tokens"),
        ))
    }
}

pub async fn tokens_index_handler_es(State(state): State<AppState>) -> Html<String> {
    let content = render::card::render_tokens_index(&state, "es");
    Html(render::shell::page_shell_lang(
        "Catálogo de Objetos BIM",
        "/es/tokens",
        &content,
        &state,
        "es",
        Some("/tokens"),
    ))
}

pub async fn token_category_handler(
    Path(name): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Html<String> {
    let content = render::card::render_token_page(&name, &state, "en");
    if is_fragment(&headers) {
        Html(content)
    } else {
        Html(render::shell::page_shell_lang(
            &format!("{name} — BIM Objects"),
            &format!("/tokens/{name}"),
            &content,
            &state,
            "en",
            Some(&format!("/es/tokens/{name}")),
        ))
    }
}

pub async fn token_category_handler_es(
    Path(name): Path<String>,
    State(state): State<AppState>,
) -> Html<String> {
    let content = render::card::render_token_page(&name, &state, "es");
    Html(render::shell::page_shell_lang(
        &format!("{name} — {}", t("es", "BIM Objects", "Objetos BIM")),
        &format!("/es/tokens/{name}"),
        &content,
        &state,
        "es",
        Some(&format!("/tokens/{name}")),
    ))
}

pub async fn token_category_fragment(
    Path(name): Path<String>,
    State(state): State<AppState>,
) -> Html<String> {
    Html(render::card::render_token_page(&name, &state, "en"))
}

pub async fn tokens_index_fragment(State(state): State<AppState>) -> Html<String> {
    Html(render::card::render_tokens_index(&state, "en"))
}

fn is_fragment(headers: &HeaderMap) -> bool {
    headers.get("x-fragment").is_some()
}
