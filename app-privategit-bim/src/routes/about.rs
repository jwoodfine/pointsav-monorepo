// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

use axum::{extract::State, response::Html};

use crate::{
    render,
    render::shell::{esc, t},
    state::AppState,
};

pub async fn about_handler(State(state): State<AppState>) -> Html<String> {
    Html(render_about(&state, "en"))
}

pub async fn about_handler_es(State(state): State<AppState>) -> Html<String> {
    Html(render_about(&state, "es"))
}

fn render_about(state: &AppState, lang: &str) -> String {
    // Round 6 (2026-07-10) P3: the Method page explains the two-ladder
    // model and the three-zone cross-section in prose only — every
    // Key Plan detail page carries a real diagram, the page whose job
    // is teaching the underlying model carried none (the cohesion audit's
    // highest-leverage visual finding). Inject a real diagram right after
    // the section that introduces each concept, matching the same visual
    // grammar (navy plan strokes, mono labels) used everywhere else on the
    // site rather than adding a second illustration style.
    // Diagram injection is keyed by section INDEX, not heading text — the
    // Spanish about.es.md carries translated headings that will never equal
    // these English literals, but the section order (containment model
    // second, Key Plans and Tiles third) is stable across both languages.
    // Round 11 (2026-07-12): Spanish falls back to the English about_page
    // when about_page_es isn't staged yet — never a broken /es/method.
    let page = if lang == "es" {
        state
            .about_page_es
            .as_deref()
            .unwrap_or(state.about_page.as_ref())
    } else {
        state.about_page.as_ref()
    };
    let mut sections = String::new();
    for (idx, section) in page.sections.iter().enumerate() {
        sections.push_str(&format!(
            "<section><h2>{}</h2>{}</section>",
            esc(&section.heading),
            section.body_html,
        ));
        if idx == 1 {
            sections.push_str(&format!(
                r#"<figure class="bim-method-figure">{svg}<figcaption>{cap}</figcaption></figure>"#,
                svg = render::svg::render_containment_model_svg(),
                cap = t(
                    lang,
                    "Containment drawn as containment: Building, Floor Plate, Tile and Key Plan nest inside one another at the same step, without remainder, at every scale. The Object is the one filled element in the drawing — a standalone part contained in its Key Plan, never summed into the frames around it.",
                    "La contención se dibuja como contención: Edificio, Placa de Piso, Tile y Key Plan se anidan uno dentro de otro en el mismo paso, sin remanente, en cada escala. El Objeto es el único elemento relleno del dibujo — una pieza independiente contenida en su Key Plan, que nunca se suma a los marcos que la rodean.",
                )
            ));
        } else if idx == 2 {
            sections.push_str(&format!(
                r#"<figure class="bim-method-figure">{svg}<figcaption>{cap}</figcaption></figure>"#,
                svg = render::svg::render_method_zone_svg(),
                cap = t(
                    lang,
                    "Every Key Plan divides its depth into the same three zones, measured from the facade inward: Habitat (H) holds the 6.0 m daylight perimeter, Magazine (M) the 3.5 m of flexible depth behind it, and Corridor (C) the final 2.0 m of circulation — the chained dimension on the right shows the three depths summing to 11.5 m. The depths shown are illustrative; each Key Plan records its own.",
                    "Cada Key Plan divide su profundidad en las mismas tres zonas, medidas desde la fachada hacia adentro: Hábitat (H) ocupa el perímetro de 6.0 m con luz natural, Magazine (M) los 3.5 m de profundidad flexible detrás de él, y Corredor (C) los últimos 2.0 m de circulación — la cota encadenada a la derecha muestra las tres profundidades sumando 11.5 m. Las profundidades mostradas son ilustrativas; cada Key Plan registra las suyas propias.",
                )
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
        kicker = t(lang, "The method", "El método"),
        title = t(lang, "Method", "Método"),
    );

    let (active_path, alt_path) = if lang == "es" {
        ("/es/method", Some("/method"))
    } else {
        ("/method", Some("/es/method"))
    };
    render::shell::page_shell_lang(
        t(lang, "Method", "Método"),
        active_path,
        &content,
        state,
        lang,
        alt_path,
    )
}
