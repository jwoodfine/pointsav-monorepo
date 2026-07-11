// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

use axum::{extract::State, response::Html};

use crate::{render, render::shell::esc, state::AppState};

pub async fn about_handler(State(state): State<AppState>) -> Html<String> {
    // Round 6 (2026-07-10) P3: the Method page explains the two-ladder
    // model and the three-zone cross-section in prose only — every
    // Key Plan detail page carries a real diagram, the page whose job
    // is teaching the underlying model carried none (the cohesion audit's
    // highest-leverage visual finding). Inject a real diagram right after
    // the section that introduces each concept, matching the same visual
    // grammar (navy plan strokes, mono labels) used everywhere else on the
    // site rather than adding a second illustration style.
    let mut sections = String::new();
    for section in state.about_page.sections.iter() {
        sections.push_str(&format!(
            "<section><h2>{}</h2>{}</section>",
            esc(&section.heading),
            section.body_html,
        ));
        if section.heading == "The containment model" {
            sections.push_str(&format!(
                r#"<figure class="bim-method-figure">{svg}<figcaption>An Object placed in a Key Plan stays a part — it is contained in the plan, never summed into the spatial ladder. Key Plans aggregate into Tiles, Tiles into Floor Plates, Floor Plates into the Building.</figcaption></figure>"#,
                svg = render::svg::render_containment_model_svg()
            ));
        } else if section.heading == "Key Plans and Tiles" {
            sections.push_str(&format!(
                r#"<figure class="bim-method-figure">{svg}<figcaption>Every Key Plan divides its depth into the same three zones, measured from the facade inward: Habitat holds the 6.0 m daylight perimeter, Magazine the 3.5 m of flexible depth behind it, and Corridor the final 2.0 m of circulation. The depths shown are illustrative; each Key Plan records its own.</figcaption></figure>"#,
                svg = render::svg::render_method_zone_svg()
            ));
        }
    }

    // Renamed from "Discipline" to the bare noun "Method" (Round 5,
    // 2026-07-10 — Discipline collided with the established BIM-coordination
    // meaning of "discipline models"/discipline-based clash detection, which
    // is confusing for exactly the AEC-literate audience this site targets;
    // "Method" carries no competing meaning and matches the site's own live
    // kicker pattern — "The parts" / "The plans" / "The method" — as a
    // single plain noun like Objects/Key Plans/Research). Page content
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
        "Method", "/method", &content, &state,
    ))
}
