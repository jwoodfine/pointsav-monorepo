// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

use crate::state::AppState;

/// Chrome-string picker for the two-language site (Round 11, 2026-07-12).
/// `lang` is `"en"` or `"es"`; anything else falls back to English. Scoped
/// to chrome-level "furniture" strings only (nav, footer, search, 404,
/// badges) — never body content or legal/disclosure text, which is loaded
/// per-page from real `.es.md` sidecars (see `content::load_page_es`) so a
/// native-verification flag can travel with the actual drafted text.
pub fn t<'a>(lang: &str, en: &'a str, es: &'a str) -> &'a str {
    if lang == "es" {
        es
    } else {
        en
    }
}

pub fn page_shell(title: &str, active_path: &str, content: &str, state: &AppState) -> String {
    page_shell_lang(title, active_path, content, state, "en", None)
}

/// Language-aware page shell. `lang` drives `<html lang>`, hreflang tags,
/// chrome strings, and the language-switch link; `alt_path` is the URL of
/// this same page in the *other* language, or `None` when no counterpart
/// exists yet (the switch renders nothing in that case — graceful
/// degradation for partial Tier-1 coverage, matching the reference
/// implementation at app-mediakit-marketing-2's `lang_switch()`).
pub fn page_shell_lang(
    title: &str,
    active_path: &str,
    content: &str,
    state: &AppState,
    lang: &str,
    alt_path: Option<&str>,
) -> String {
    let tc = state.categories.len();
    let full_title = if title.is_empty() {
        "Woodfine BIM Library".to_string()
    } else {
        format!("{} — Woodfine BIM Library", esc(title))
    };

    // /edit/* embeds real Carbon Web Components (<cds-content-switcher> etc.)
    // that are only styled for a light Carbon theme — force light there
    // server-side rather than trying to make Carbon's chrome theme-reactive.
    let editor_route = active_path.starts_with("/edit/");
    // "Important Information" band: a short, counsel-owned summary from
    // important-information.md — NOT the full disclaimers_page content
    // (that's a separate, deliberate earlier fix for a different bug —
    // see BRIEF-app-privategit-bim.md's 2026-07-07 entry). This matches
    // Command's actual spec (2026-07-02) and the proven, counsel-approved
    // reference pattern already shipped on project-knowledge's
    // app-mediakit-knowledge: short band + "Full disclaimer" link to the
    // long-form page, with a safe issuer-aware default if the file is ever
    // missing (never a hard failure).
    let disclosure_body: &str = if lang == "es" {
        state.important_information_es.as_deref().unwrap_or(
            "<p>Este sitio presenta registros mantenidos por Woodfine Capital Projects Inc. \
La información se proporciona únicamente con fines informativos generales y no constituye \
una oferta de venta, una solicitud de oferta de compra, ni asesoría de inversión, legal, \
fiscal o contable. Las declaraciones sobre actividades futuras planeadas, previstas u \
objetivo son prospectivas y están sujetas a cambio sin previo aviso; no se actualizan salvo \
que la ley lo exija. Este texto es una traducción preparada internamente, pendiente de \
verificación profesional antes de considerarse definitiva — ver la versión en inglés como \
referencia autorizada.</p>",
        )
    } else {
        state.important_information.as_deref().unwrap_or(
            "<p>This site presents records maintained by Woodfine Capital Projects Inc. \
The information is provided for general information only and does not constitute \
an offer to sell, a solicitation of an offer to buy, or investment, legal, tax, or \
accounting advice. Statements regarding planned, intended, or targeted future \
activities are forward-looking and subject to change without notice; they are not \
undertaken to be updated except as required by law.</p>",
        )
    };
    let objects_current = if active_path.starts_with("/objects") {
        r#" aria-current="page""#
    } else {
        ""
    };
    let key_plans_current = if active_path.starts_with("/key-plans") {
        r#" aria-current="page""#
    } else {
        ""
    };
    let research_current = if active_path.starts_with("/research") {
        r#" aria-current="page""#
    } else {
        ""
    };
    let method_current = if active_path.starts_with("/method") {
        r#" aria-current="page""#
    } else {
        ""
    };
    let theme_toggle = if editor_route {
        String::new()
    } else {
        format!(
            r#"<button class="bim-theme-toggle" type="button" aria-pressed="false" aria-label="{aria}">
        <svg class="bim-theme-toggle__sun" aria-hidden="true" viewBox="0 0 20 20" fill="none" xmlns="http://www.w3.org/2000/svg">
          <circle cx="10" cy="10" r="4" stroke="currentColor" stroke-width="1.5"></circle>
          <path d="M10 1.5V3.5M10 16.5V18.5M18.5 10H16.5M3.5 10H1.5M15.9 4.1L14.5 5.5M5.5 14.5L4.1 15.9M15.9 15.9L14.5 14.5M5.5 5.5L4.1 4.1" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"></path>
        </svg>
        <svg class="bim-theme-toggle__moon" aria-hidden="true" viewBox="0 0 20 20" fill="none" xmlns="http://www.w3.org/2000/svg">
          <path d="M17 11.5A7 7 0 118.5 3a5.5 5.5 0 108.5 8.5Z" stroke="currentColor" stroke-width="1.5" stroke-linejoin="round"></path>
        </svg>
      </button>"#,
            aria = t(lang, "Switch to dark theme", "Cambiar a tema oscuro"),
        )
    };

    // hreflang + language-switch link (Round 11, 2026-07-12). `alt_path` is
    // `None` for the many English-only pages outside Tier-1 scope this
    // round (Objects/Key Plans/Research/Search) — the switch simply renders
    // nothing there, matching the reference `lang_switch()`'s own
    // graceful-degradation behavior for partial coverage.
    let base = state.config.public_url.trim_end_matches('/');
    let (en_path, es_path_opt): (String, Option<String>) = if lang == "es" {
        (
            alt_path.unwrap_or("/").to_string(),
            Some(active_path.to_string()),
        )
    } else {
        (active_path.to_string(), alt_path.map(|p| p.to_string()))
    };
    let hreflang_tags = match &es_path_opt {
        Some(es_path) => format!(
            "\n  <link rel=\"alternate\" hreflang=\"en\" href=\"{base}{en_path}\">\
             \n  <link rel=\"alternate\" hreflang=\"es\" href=\"{base}{es_path}\">\
             \n  <link rel=\"alternate\" hreflang=\"x-default\" href=\"{base}{en_path}\">"
        ),
        None => String::new(),
    };
    let lang_switch = match (lang, alt_path) {
        ("es", Some(p)) => format!(
            r#"<a class="bim-lang-switch" href="{p}" hreflang="en" lang="en">English</a>"#,
            p = esc(p)
        ),
        ("es", None) => String::new(),
        (_, Some(p)) => format!(
            r#"<a class="bim-lang-switch" href="{p}" hreflang="es" lang="es">Español</a>"#,
            p = esc(p)
        ),
        (_, None) => String::new(),
    };
    let html_lang = if lang == "es" { "es" } else { "en" };

    // Visible verification-pending notice (Round 11, 2026-07-12) — the
    // operator's own decision was to draft Spanish content directly rather
    // than route it to project-editorial, on condition it stays flagged
    // everywhere until a native-speaker/professional pass confirms it.
    // Matches the reference app-mediakit-marketing-2's own disclosed-caveat
    // pattern (ui.rs:216-219) but surfaced as real page chrome, not just a
    // code comment, since it applies sitewide rather than to one field.
    let translation_notice = if lang == "es" {
        r#"<p class="bim-translation-notice">Esta página es una traducción preparada internamente, pendiente de verificación por un hablante nativo antes de considerarse definitiva. Ante cualquier discrepancia, la <a href="{en}">versión en inglés</a> es la referencia autorizada.</p>"#
            .replace("{en}", &esc(&en_path))
    } else {
        String::new()
    };
    // Carbon Web Components + their CSS are only used by /edit/* (real
    // <cds-content-switcher> etc.) — the public catalog no longer borrows
    // Carbon's visual language, so it no longer ships Carbon's CSS either.
    let carbon_assets = if editor_route {
        r#"
  <link rel="stylesheet" href="/static/carbon.min.css">
  <link rel="stylesheet" href="/static/carbon-overrides.css">
  <script type="module" src="/static/carbon.esm.js"></script>"#
    } else {
        ""
    };
    let html_theme_attr = if editor_route {
        r#" data-theme="light""#
    } else {
        ""
    };
    let theme_preload_script = if editor_route {
        String::new()
    } else {
        r#"
  <script>
    (function () {
      var stored = null;
      try { stored = localStorage.getItem('bim-theme'); } catch (e) {}
      var theme = stored || (window.matchMedia && window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light');
      document.documentElement.setAttribute('data-theme', theme);
    })();
  </script>"#
            .to_string()
    };

    format!(
        r#"<!DOCTYPE html>
<html lang="{html_lang}"{html_theme_attr}>
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>{full_title}</title>
  <meta name="description" content="{meta_desc}">{hreflang_tags}
  <!-- Round 7 (2026-07-11): inline SVG favicon — a bounded room with a
       zone-partition line, same navy-stroke plan-drawing convention used
       throughout the site. No new binary asset to manage; browsers were
       requesting /favicon.ico and getting a 404 on every page load. -->
  <link rel="icon" type="image/svg+xml" href="data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 32 32'%3E%3Crect width='32' height='32' rx='4' fill='%23F7F9FA'/%3E%3Crect x='5' y='5' width='22' height='22' rx='1' fill='none' stroke='%23164679' stroke-width='2.5'/%3E%3Cline x1='5' y1='20' x2='27' y2='20' stroke='%23164679' stroke-width='2'/%3E%3C/svg%3E">
  <link rel="stylesheet" href="/static/fonts.css">
  <link rel="stylesheet" href="/static/tokens.css">
  <link rel="stylesheet" href="/static/bim-layout.css">
  <link rel="stylesheet" href="/static/bim-components.css">
  <link rel="stylesheet" href="/static/bim-planroom.css">{carbon_assets}{theme_preload_script}
  <script type="module" src="/static/bim.js"></script>
</head>
<body class="bim-body">
  <header class="bim-header">
    <div class="bim-header__inner">
      <a href="/" class="bim-header__brand" aria-label="Woodfine — BIM Library">Woodfine <span class="bim-header__brand-sub">BIM Library</span></a>
      <nav class="bim-header__nav" aria-label="Primary">
        <a href="/method"{method_current}>{nav_method}</a>
        <a href="/objects"{objects_current}>{nav_objects}</a>
        <a href="/key-plans"{key_plans_current}>Key Plans</a>
        <a href="/research"{research_current}>{nav_research}</a>
      </nav>
      <form class="bim-header__search" method="get" action="/search" role="search">
        <input type="search" name="q" placeholder="{search_label}" aria-label="{search_aria}">
      </form>
      <div class="bim-header__right">
        {lang_switch}
        {theme_toggle}
        <details class="bim-drawer" id="bim-drawer">
          <summary class="bim-header__hamburger" aria-label="{open_menu}">
            <svg class="bim-header__hamburger-icon" aria-hidden="true" viewBox="0 0 20 20" fill="none" xmlns="http://www.w3.org/2000/svg">
              <path d="M3 5.5H17M3 10H17M3 14.5H17" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"></path>
            </svg>
          </summary>
          <div class="bim-drawer__backdrop"></div>
          <div class="bim-drawer__panel" role="dialog" aria-modal="true" aria-label="{menu_aria}">
            <div class="bim-drawer__head">
              <span class="bim-drawer__brand">Woodfine <span class="bim-header__brand-sub">BIM Library</span></span>
              <button class="bim-drawer__close" type="button" aria-label="{close_menu}">
                <svg aria-hidden="true" viewBox="0 0 20 20" fill="none" xmlns="http://www.w3.org/2000/svg" width="16" height="16">
                  <path d="M4.5 4.5L15.5 15.5M15.5 4.5L4.5 15.5" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"></path>
                </svg>
              </button>
            </div>
            <form class="bim-drawer__search" method="get" action="/search" role="search">
              <input type="search" name="q" placeholder="{search_label}" aria-label="{search_aria}">
            </form>
            <nav class="bim-drawer__nav" aria-label="Primary">
              <a href="/method"{method_current}>{nav_method}</a>
              <a href="/objects"{objects_current}>{nav_objects}</a>
              <a href="/key-plans"{key_plans_current}>Key Plans</a>
              <a href="/research"{research_current}>{nav_research}</a>
            </nav>
            {lang_switch}
          </div>
        </details>
      </div>
    </div>
  </header>
  <div class="bim-shell">
    <main id="bim-main-content" class="bim-main">
      {translation_notice}{content}
    </main>
  </div>
  <section class="bim-disclosure" aria-label="{important_info_aria}">
    <details class="bim-disclosure__details">
      <summary class="bim-disclosure__summary">{important_info}</summary>
      <div class="bim-disclosure__body">
        <p class="bim-disclosure__label">{disclosure_label}</p>
        {disclosure_body}
        <p class="bim-disclosure__more"><a href="/disclaimers">{full_disclaimer} &rarr;</a></p>
      </div>
    </details>
  </section>
  <footer class="bim-footer">
    <div class="bim-footer__inner">
      <div>
        <p class="bim-footer__heading">Woodfine BIM Library</p>
        <ul class="bim-footer__list">
          <li>{footer_tagline}</li>
          <li>{tc} {footer_categories} &middot; {comp} Key&nbsp;Plans &middot; {rc} {footer_research_entries}</li>
          <li>IFC&nbsp;4.3 (ISO&nbsp;16739-1:2024) &middot; Uniclass&nbsp;2015 &middot; DTCG</li>
          <li>{footer_license_line}</li>
          <li><a href="https://github.com/pointsav/pointsav-monorepo">{footer_source_code}</a></li>
        </ul>
      </div>
      <div>
        <p class="bim-footer__heading">{footer_machine_heading}</p>
        <ul class="bim-footer__list bim-footer__list--machine">
          <li><a class="bim-machine-link" href="/api/tokens.json">/api/tokens.json</a> &mdash; {footer_dtcg_bundle}</li>
          <li><a class="bim-machine-link" href="/mcp">/mcp</a> &mdash; {footer_mcp_endpoint}</li>
          <li><a class="bim-machine-link" href="/research">/research</a> &mdash; {footer_research_backplane}</li>
        </ul>
      </div>
      <div>
        <p class="bim-footer__heading">{footer_network_heading}</p>
        <ul class="bim-footer__list">
          <li><a href="https://home.woodfinegroup.com" target="_blank" rel="noopener">Woodfine Capital Projects</a></li>
          <li><a href="https://corporate.woodfinegroup.com" target="_blank" rel="noopener">{footer_corporate}</a></li>
          <li><a href="https://projects.woodfinegroup.com" target="_blank" rel="noopener">{footer_projects}</a></li>
          <li><a href="https://github.com/woodfine/woodfine-bim-library" target="_blank" rel="noopener">GitHub</a></li>
          <li><a href="https://home.pointsav.com" target="_blank" rel="noopener">PointSav Digital Systems</a></li>
        </ul>
      </div>
    </div>
    <div class="bim-footer__base">
      <div class="bim-footer__base-row">
        <div class="bim-footer__cities">
          <span class="bim-footer__cities-inner">
            <span>Vancouver</span>
            <span class="bim-footer__cities-sep" aria-hidden="true">|</span>
            <span>New York</span>
          </span>
        </div>
        <div class="bim-footer__badges">
          <a class="bim-badge bim-badge--license" href="https://creativecommons.org/licenses/by-nd/4.0/"
             target="_blank" rel="noopener license" aria-label="Content licensed CC BY-ND 4.0">
            <span class="bim-badge__cc" aria-hidden="true">
              <img class="bim-cc-icon" src="/static/cc.svg" alt="" width="20" height="20">
              <img class="bim-cc-icon" src="/static/cc-by.svg" alt="" width="20" height="20">
              <img class="bim-cc-icon" src="/static/cc-nd.svg" alt="" width="20" height="20">
            </span>
            <span class="bim-badge__text">
              <span class="bim-badge__lead">{footer_licensed}</span>
              <span class="bim-badge__name">CC BY-ND 4.0</span>
            </span>
          </a>
          <span class="bim-badge">
            <svg class="bim-badge__glyph" aria-hidden="true" viewBox="0 0 20 20" fill="none" xmlns="http://www.w3.org/2000/svg">
              <path d="M5 2.5h7l3 3v12a1 1 0 01-1 1H5a1 1 0 01-1-1v-14a1 1 0 011-1Z" stroke="currentColor" stroke-width="1.3" stroke-linejoin="round"></path>
              <path d="M12 2.5v3h3" stroke="currentColor" stroke-width="1.3" stroke-linejoin="round"></path>
            </svg>
            <span class="bim-badge__text">
              <span class="bim-badge__lead">{footer_powered_by}</span>
              <span class="bim-badge__name">PrivateGit</span>
            </span>
          </span>
        </div>
      </div>
      <p>{footer_copyright}</p>
      <p class="bim-footer__disclaimer">{footer_disclaimer}</p>
      <p class="bim-footer__trademark">{footer_trademark}</p>
    </div>
  </footer>
</body>
</html>"#,
        full_title = full_title,
        meta_desc = t(
            lang,
            "Building specifications that enforce compliance at placement, not inspection after the fact. Open-standard IFC 4.3 BIM Object catalog.",
            "Especificaciones de construcción que exigen el cumplimiento normativo desde la colocación, no en una inspección posterior. Catálogo de Objetos BIM en el estándar abierto IFC 4.3.",
        ),
        hreflang_tags = hreflang_tags,
        html_lang = html_lang,
        translation_notice = translation_notice,
        html_theme_attr = html_theme_attr,
        carbon_assets = carbon_assets,
        theme_preload_script = theme_preload_script,
        theme_toggle = theme_toggle,
        lang_switch = lang_switch,
        objects_current = objects_current,
        key_plans_current = key_plans_current,
        research_current = research_current,
        method_current = method_current,
        nav_method = t(lang, "Method", "Método"),
        nav_objects = t(lang, "Objects", "Objetos"),
        nav_research = t(lang, "Research", "Investigación"),
        search_label = t(lang, "Search", "Buscar"),
        search_aria = t(lang, "Search the registry", "Buscar en el registro"),
        open_menu = t(lang, "Open menu", "Abrir menú"),
        close_menu = t(lang, "Close menu", "Cerrar menú"),
        menu_aria = t(lang, "Menu", "Menú"),
        important_info_aria = t(lang, "Important information", "Información importante"),
        important_info = t(lang, "Important Information", "Información importante"),
        disclosure_label = t(lang, "BIM Library disclosure", "Aviso de la Biblioteca BIM"),
        full_disclaimer = t(lang, "Full disclaimer", "Aviso legal completo"),
        disclosure_body = disclosure_body,
        content = content,
        tc = tc,
        comp = state.components_count,
        rc = state.research_count,
        footer_tagline = t(
            lang,
            "Specification BIM Objects for the built environment",
            "Objetos BIM de especificación para el entorno construido",
        ),
        footer_categories = t(lang, "BIM Object categories", "categorías de Objetos BIM"),
        footer_research_entries = t(lang, "research entries", "entradas de investigación"),
        footer_license_line = t(
            lang,
            "BIM Object data licensed <strong>Apache-2.0</strong> &middot; platform code <strong>AGPL-3.0-or-later</strong>",
            "Datos de Objetos BIM con licencia <strong>Apache-2.0</strong> &middot; código de la plataforma <strong>AGPL-3.0-or-later</strong>",
        ),
        footer_source_code = t(
            lang,
            "Platform source code (github.com/pointsav)",
            "Código fuente de la plataforma (github.com/pointsav)",
        ),
        footer_machine_heading = t(lang, "Machine-readable surface", "Superficie legible por máquina"),
        footer_dtcg_bundle = t(lang, "full DTCG bundle", "paquete DTCG completo"),
        footer_mcp_endpoint = t(lang, "MCP JSON-RPC endpoint", "endpoint MCP JSON-RPC"),
        footer_research_backplane = t(lang, "research backplane", "panel de investigación"),
        footer_network_heading = t(lang, "Woodfine network", "Red Woodfine"),
        footer_corporate = t(lang, "Corporate", "Corporativo"),
        footer_projects = t(lang, "Projects", "Proyectos"),
        footer_licensed = t(lang, "Licensed", "Con licencia"),
        footer_powered_by = t(lang, "Powered by", "Desarrollado con"),
        footer_copyright = t(
            lang,
            r#"Copyright &copy; 2026 Woodfine Capital Projects Inc. See <a href="https://github.com/pointsav/pointsav-monorepo/blob/main/app-privategit-bim/LICENSE" target="_blank" rel="noopener">LICENSE</a> for terms."#,
            r#"Copyright &copy; 2026 Woodfine Capital Projects Inc. Consulte <a href="https://github.com/pointsav/pointsav-monorepo/blob/main/app-privategit-bim/LICENSE" target="_blank" rel="noopener">LICENSE</a> para conocer los términos."#,
        ),
        footer_disclaimer = t(
            lang,
            "Provided for reference and coordination only — not a substitute for code review.",
            "Proporcionado únicamente para referencia y coordinación — no sustituye la revisión normativa por profesionales certificados.",
        ),
        footer_trademark = t(
            lang,
            "Woodfine Capital Projects&trade;, MCorp&trade;, PointSav Digital Systems&trade;, Totebox Orchestration&trade;, Totebox Archive&trade;, and Capability Geometry&trade; are trademarks of Woodfine Capital Projects Inc., used in Canada, the United States, Latin America, and Europe. Capability Geometry&trade; is an unregistered trademark of Woodfine Capital Projects Inc. All other trademarks are the property of their respective owners.",
            "Woodfine Capital Projects&trade;, MCorp&trade;, PointSav Digital Systems&trade;, Totebox Orchestration&trade;, Totebox Archive&trade;, y Capability Geometry&trade; son marcas comerciales de Woodfine Capital Projects Inc., utilizadas en Canadá, Estados Unidos, América Latina y Europa. Capability Geometry&trade; es una marca comercial no registrada de Woodfine Capital Projects Inc. Todas las demás marcas son propiedad de sus respectivos titulares.",
        ),
    )
}

pub fn esc(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}
