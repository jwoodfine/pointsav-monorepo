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

    // Renamed from "About BIM Objects" to the bare noun "Discipline"
    // (2026-07-09 operator decision — matches the grammatical pattern of
    // the other primary nav items: Objects, Compositions, Research; no
    // article, not "The Discipline"). Page content below is unchanged from
    // before the rename — only the kicker/h1/title change, per scope. The
    // kicker keeps the "The <phrase>" lead-in pattern the Objects/
    // Compositions pages use ("The parts" / "The assemblies") ahead of
    // their own bare-noun h1.
    let content = format!(
        r#"<div class="bim-breadcrumbs">
  <a href="/" data-path="/" class="bim-nav-link">Home</a>
</div>
<header class="bim-cat-pagehead">
  <span class="bim-cat-kicker">The discipline</span>
  <h1>Discipline</h1>
</header>
<article class="bim-article">
  {sections}
</article>"#,
    );

    Html(render::shell::page_shell(
        "Discipline",
        "/discipline",
        &content,
        &state,
    ))
}
