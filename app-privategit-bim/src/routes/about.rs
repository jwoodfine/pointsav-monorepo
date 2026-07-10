// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

use axum::{extract::State, response::Html};

use crate::{render, render::shell::esc, state::AppState};

pub async fn about_handler(State(state): State<AppState>) -> Html<String> {
    let mut sections = String::new();
    for section in state.about_page.sections.iter() {
        sections.push_str(&format!(
            "<section><h2>{}</h2>{}</section>",
            esc(&section.heading),
            section.body_html,
        ));
    }

    // Renamed from "Discipline" to the bare noun "Method" (Round 5,
    // 2026-07-10 — Discipline collided with the established BIM-coordination
    // meaning of "discipline models"/discipline-based clash detection, which
    // is confusing for exactly the AEC-literate audience this site targets;
    // "Method" carries no competing meaning and matches the site's own live
    // kicker pattern — "The parts" / "The assemblies" / "The method" — as a
    // single plain noun like Objects/Compositions/Research). Page content
    // itself was also rewritten this round to correct a real ontology error
    // (see woodfine-bim-library/site-content/pages/about.md) — this route
    // handler's structure is otherwise unchanged.
    let content = format!(
        r#"<div class="bim-breadcrumbs">
  <a href="/" data-path="/" class="bim-nav-link">Home</a>
</div>
<header class="bim-cat-pagehead">
  <span class="bim-cat-kicker">The method</span>
  <h1>Method</h1>
</header>
<article class="bim-article">
  {sections}
</article>"#,
    );

    Html(render::shell::page_shell(
        "Method",
        "/method",
        &content,
        &state,
    ))
}
