// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

use axum::{extract::State, response::Html};

use crate::{render, state::AppState};

pub async fn home_handler(State(state): State<AppState>) -> Html<String> {
    let content = render::catalog::render_home(&state, "en");
    Html(render::shell::page_shell_lang(
        "",
        "/",
        &content,
        &state,
        "en",
        Some("/es"),
    ))
}

pub async fn home_handler_es(State(state): State<AppState>) -> Html<String> {
    let content = render::catalog::render_home(&state, "es");
    Html(render::shell::page_shell_lang(
        "",
        "/es",
        &content,
        &state,
        "es",
        Some("/"),
    ))
}
