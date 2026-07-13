// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

use axum::{extract::State, response::Html};

use crate::{
    render,
    render::shell::{esc, t},
    state::AppState,
};

pub async fn disclaimers_handler(State(state): State<AppState>) -> Html<String> {
    Html(render_disclaimers(&state, "en"))
}

pub async fn disclaimers_handler_es(State(state): State<AppState>) -> Html<String> {
    Html(render_disclaimers(&state, "es"))
}

fn render_disclaimers(state: &AppState, lang: &str) -> String {
    // Round 11 (2026-07-12): falls back to English disclaimers_page when
    // disclaimers_page_es isn't staged yet — never a broken /es/disclaimers.
    let page = if lang == "es" {
        state
            .disclaimers_page_es
            .as_deref()
            .unwrap_or(state.disclaimers_page.as_ref())
    } else {
        state.disclaimers_page.as_ref()
    };
    let mut sections = String::new();
    for section in page.sections.iter() {
        sections.push_str(&format!(
            "<section><h2>{}</h2>{}</section>",
            esc(&section.heading),
            section.body_html,
        ));
    }

    let content = format!(
        r#"<div class="bim-breadcrumbs">
  <a href="/" data-path="/" class="bim-nav-link">{home}</a>
</div>
<header class="bim-cat-pagehead">
  <span class="bim-cat-kicker">{kicker}</span>
  <h1>{title}</h1>
</header>
<article class="bim-article">
  {sections}
</article>"#,
        home = t(lang, "Home", "Inicio"),
        kicker = t(lang, "Important information", "Información importante"),
        title = t(lang, "Disclaimers", "Avisos legales"),
    );

    let (active_path, alt_path) = if lang == "es" {
        ("/es/disclaimers", Some("/disclaimers"))
    } else {
        ("/disclaimers", Some("/es/disclaimers"))
    };
    render::shell::page_shell_lang(
        t(lang, "Disclaimers", "Avisos legales"),
        active_path,
        &content,
        state,
        lang,
        alt_path,
    )
}
