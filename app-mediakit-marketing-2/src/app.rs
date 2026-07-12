// SPDX-License-Identifier: FSL-1.1-ALv2
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! Application state and axum Router. P0 mounted `/healthz` and
//! `/static/*path`; P1 added the content pipeline; P3 wired real chrome;
//! P4 added SEO/discovery; P5 adds the MCP JSON-RPC surface + review queue
//! (mounted only when `enable_mcp` is set).

use std::path::PathBuf;
use std::sync::Arc;

use axum::extract::{Path as AxumPath, State};
use axum::http::{header, HeaderName, HeaderValue, StatusCode};
use axum::response::{IntoResponse, Json, Response};
use axum::routing::{get, post};
use axum::Router;
use tower_http::compression::CompressionLayer;
use tower_http::set_header::SetResponseHeaderLayer;

use crate::config::Config;
use crate::content;
use crate::error::MarketingError;
use crate::legal_tokens::LegalTokens;
use crate::mcp::{self, RpcRequest};
use crate::pending::Queue;
use crate::ui::{page_shell, Tenant};

pub struct AppStateInner {
    pub content_dir: PathBuf,
    pub module_id: String,
    /// `SERVICE_MARKETING_GOOGLE_VERIFY` — read directly from the
    /// environment (not a clap flag) to match the retired engine's contract.
    pub google_verify: Option<String>,
    pub pending: Queue,
    /// Canonical trademark/copyright facts, loaded once at startup from
    /// `factory-release-engineering`. See `crate::legal_tokens`.
    pub legal_tokens: LegalTokens,
    /// Per-process CSP nonce for the one inline `<script>` this engine emits
    /// (the JSON-LD block — see `ui::page_shell`). Generated once at startup
    /// rather than per-request: this engine has no user-generated-content
    /// rendering path (all content is trusted YAML behind the F12 approval
    /// queue, not public form submissions), so the marginal protection of a
    /// per-request nonce over a per-process one is low, and a startup nonce
    /// lets the CSP header stay a static `SetResponseHeaderLayer` value
    /// instead of per-response middleware.
    pub csp_nonce: String,
}

pub type AppState = Arc<AppStateInner>;

pub fn build_state(cfg: &Config) -> Result<AppState, MarketingError> {
    let legal_tokens = LegalTokens::load(&cfg.legal_tokens_dir, &cfg.module_id)?;
    Ok(Arc::new(AppStateInner {
        content_dir: cfg.content_dir.clone(),
        module_id: cfg.module_id.clone(),
        google_verify: std::env::var("SERVICE_MARKETING_GOOGLE_VERIFY").ok(),
        pending: Queue::open(&cfg.state_dir)?,
        legal_tokens,
        csp_nonce: uuid::Uuid::new_v4().to_string(),
    }))
}

/// Content-Security-Policy value for this process's lifetime. `script-src`
/// allows only same-origin files and the one nonce'd inline JSON-LD block —
/// no `'unsafe-inline'`/`'unsafe-eval'` for scripts. `style-src` allows
/// `'unsafe-inline'`: three call sites in `ui.rs` set an inline `style`
/// attribute with a runtime-computed CSS custom property (grid/icon column
/// counts, one per-icon scale hack pending removal) — CSP nonces don't cover
/// inline style *attributes* (only `<style>` blocks), and converting these
/// three to an enumerated class set is a real but separable refactor, not a
/// fast-path fix. CSS-only injection risk (no code execution) is materially
/// lower than script injection, which is why this asymmetry is an accepted
/// trade-off here, not an oversight.
fn content_security_policy(nonce: &str) -> String {
    format!(
        "default-src 'self'; script-src 'self' 'nonce-{nonce}'; style-src 'self' 'unsafe-inline'; \
         img-src 'self' data:; object-src 'none'; base-uri 'self'; form-action 'self'; \
         frame-ancestors 'none'"
    )
}

pub fn router(state: AppState, enable_mcp: bool) -> Router {
    // /es route for the home page (2026-07-12, reverses the 2026-07-02
    // "English only on home for now" decision) — the page.es.yaml content
    // was kept in sync the whole time, just unrouted; this turns it on.
    // Other pages (/page/{slug}) keep their existing /es/page/{slug}
    // pattern.
    let mut router = Router::new()
        .route("/healthz", get(healthz))
        .route("/static/{*path}", get(crate::assets::serve))
        .route("/", get(home))
        .route("/es", get(home_es))
        .route("/page/{slug}", get(page))
        .route("/es/page/{slug}", get(page_es))
        .route("/robots.txt", get(robots_txt))
        .route("/sitemap.xml", get(sitemap_xml));

    if enable_mcp {
        router = router
            .route("/api/mcp", post(mcp_rpc))
            .route("/api/pending", get(list_pending))
            .route("/api/pending/{id}/manifest", get(pending_manifest))
            .route("/api/pending/{id}/approve", post(approve_pending));
    }

    // Security headers, site-wide. Not `Strict-Transport-Security` — this
    // dev/local listener serves plain HTTP, and browsers ignore HSTS over
    // http:// anyway; it belongs at the production TLS-termination layer
    // (see BRIEF-production-server-separation.md), not this app's router.
    let csp = HeaderValue::from_str(&content_security_policy(&state.csp_nonce))
        .expect("CSP value is always valid ASCII header content");
    router
        .layer(SetResponseHeaderLayer::if_not_present(
            HeaderName::from_static("content-security-policy"),
            csp,
        ))
        .layer(SetResponseHeaderLayer::if_not_present(
            header::X_CONTENT_TYPE_OPTIONS,
            HeaderValue::from_static("nosniff"),
        ))
        .layer(SetResponseHeaderLayer::if_not_present(
            header::X_FRAME_OPTIONS,
            HeaderValue::from_static("DENY"),
        ))
        .layer(SetResponseHeaderLayer::if_not_present(
            header::REFERRER_POLICY,
            HeaderValue::from_static("strict-origin-when-cross-origin"),
        ))
        .with_state(state)
}

async fn healthz() -> &'static str {
    "ok"
}

async fn home(State(state): State<AppState>) -> Response {
    render_slug(&state, "home", None)
}

async fn home_es(State(state): State<AppState>) -> Response {
    render_slug(&state, "home", Some("es"))
}

async fn page(State(state): State<AppState>, AxumPath(slug): AxumPath<String>) -> Response {
    render_slug(&state, &slug, None)
}

async fn page_es(State(state): State<AppState>, AxumPath(slug): AxumPath<String>) -> Response {
    render_slug(&state, &slug, Some("es"))
}

fn render_slug(state: &AppStateInner, slug: &str, lang: Option<&str>) -> Response {
    let page = match content::load_page(&state.content_dir, slug, lang) {
        Ok(page) => page,
        // Branded 404, not the bare-text default `MarketingError` response
        // — audit finding. Other error variants (genuine server-side
        // failures, not "this page doesn't exist") still fall through to
        // the default response.
        Err(MarketingError::PageNotFound(_)) => {
            let tenant = Tenant::by_module_id(&state.module_id, &state.legal_tokens);
            let page_lang = lang.unwrap_or("en");
            return (
                StatusCode::NOT_FOUND,
                crate::ui::not_found_page(&tenant, page_lang),
            )
                .into_response();
        }
        Err(err) => return err.into_response(),
    };
    let tenant = Tenant::by_module_id(&state.module_id, &state.legal_tokens);
    let (en_path, es_path) = slug_paths(slug);
    page_shell(
        &tenant,
        &page,
        &state.module_id,
        &en_path,
        es_path.as_deref(),
        state.google_verify.as_deref(),
        &state.csp_nonce,
    )
    .into_response()
}

/// `home`'s ES variant lives at `/es` (not `/es/page/home`, since home
/// itself lives at `/` not `/page/home`) — every other slug uses
/// `/es/page/{slug}`.
fn slug_paths(slug: &str) -> (String, Option<String>) {
    if slug == "home" {
        ("/".to_string(), Some("/es".to_string()))
    } else {
        (format!("/page/{slug}"), Some(format!("/es/page/{slug}")))
    }
}

async fn robots_txt(State(state): State<AppState>) -> Response {
    let tenant = Tenant::by_module_id(&state.module_id, &state.legal_tokens);
    let body = format!(
        "User-agent: *\nAllow: /\nSitemap: {}/sitemap.xml\n",
        tenant.canonical_base
    );
    ([(header::CONTENT_TYPE, "text/plain; charset=utf-8")], body).into_response()
}

async fn sitemap_xml(State(state): State<AppState>) -> Response {
    let tenant = Tenant::by_module_id(&state.module_id, &state.legal_tokens);
    let mut slugs = content::list_slugs(&state.content_dir);
    slugs.sort();
    let mut body = String::from(r#"<?xml version="1.0" encoding="UTF-8"?>"#);
    body.push_str(r#"<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">"#);
    for slug in &slugs {
        let (en_path, es_path) = slug_paths(slug);
        let lastmod = page_lastmod(&state.content_dir, slug, None);
        push_sitemap_url(&mut body, tenant.canonical_base, &en_path, lastmod.as_deref());
        // Audit finding: ES routes were live, indexable (index,follow,
        // self-canonical), and declared as hreflang alternates, but excluded
        // from the sitemap entirely. `home` gained its own `/es` route
        // 2026-07-12 (before that, `es_path` was `None` for it and it was
        // correctly excluded) — every slug with a real `page.es.yaml` now
        // gets its ES variant listed here too.
        if let Some(es_path) = es_path {
            if state.content_dir.join(slug).join("page.es.yaml").is_file() {
                let es_lastmod = page_lastmod(&state.content_dir, slug, Some("es"));
                push_sitemap_url(&mut body, tenant.canonical_base, &es_path, es_lastmod.as_deref());
            }
        }
    }
    body.push_str("</urlset>");
    (
        [(header::CONTENT_TYPE, "application/xml; charset=utf-8")],
        body,
    )
        .into_response()
}

fn push_sitemap_url(body: &mut String, canonical_base: &str, path: &str, lastmod: Option<&str>) {
    body.push_str(&format!("<url><loc>{canonical_base}{path}</loc>"));
    if let Some(lastmod) = lastmod {
        body.push_str(&format!("<lastmod>{lastmod}</lastmod>"));
    }
    body.push_str("</url>");
}

/// `YYYY-MM-DD` from the content manifest's filesystem mtime — a real,
/// verifiable freshness signal (when the file was actually last edited) per
/// the audit finding that sitemap entries carried no `lastmod` at all.
fn page_lastmod(content_dir: &std::path::Path, slug: &str, lang: Option<&str>) -> Option<String> {
    let filename = match lang {
        Some(lang) => format!("page.{lang}.yaml"),
        None => "page.yaml".to_string(),
    };
    let modified = std::fs::metadata(content_dir.join(slug).join(filename))
        .and_then(|m| m.modified())
        .ok()?;
    let since_epoch = modified.duration_since(std::time::UNIX_EPOCH).ok()?;
    let days = (since_epoch.as_secs() / 86400) as i64;
    let (y, m, d) = civil_date_from_days(days);
    Some(format!("{y:04}-{m:02}-{d:02}"))
}

/// Days-since-Unix-epoch → (year, month, day). Howard Hinnant's
/// `civil_from_days` algorithm ("chrono-Compatible Low-Level Date
/// Algorithms") — used instead of pulling in `chrono`/`time` for one date
/// field. Correctness is verified against known reference dates in the test
/// below, not just trusted.
fn civil_date_from_days(days: i64) -> (i64, u32, u32) {
    let z = days + 719468;
    let era = if z >= 0 { z } else { z - 146096 } / 146097;
    let doe = (z - era * 146097) as u64;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe as i64 + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let m = if mp < 10 { mp + 3 } else { mp - 9 } as u32;
    let y = if m <= 2 { y + 1 } else { y };
    (y, m, d)
}

// ---------------------------------------------------------------- P5: MCP

async fn mcp_rpc(
    State(state): State<AppState>,
    Json(req): Json<RpcRequest>,
) -> Json<mcp::RpcResponse> {
    Json(mcp::handle(&state, req))
}

async fn list_pending(State(state): State<AppState>) -> Result<Response, MarketingError> {
    let items = state.pending.list()?;
    Ok(Json(items).into_response())
}

async fn pending_manifest(
    State(state): State<AppState>,
    AxumPath(id): AxumPath<String>,
) -> Result<Response, MarketingError> {
    let manifest = state.pending.manifest(&id)?;
    Ok(([(header::CONTENT_TYPE, "application/yaml")], manifest).into_response())
}

/// The F12 human-approval endpoint. Nothing in this codebase calls this
/// automatically — it exists only to be triggered by an explicit
/// human/operator action (a UI button, a curl command a human runs).
async fn approve_pending(
    State(state): State<AppState>,
    AxumPath(id): AxumPath<String>,
) -> Result<Response, MarketingError> {
    state.pending.approve(&id, &state.content_dir)?;
    Ok(Json(serde_json::json!({ "status": "approved", "id": id })).into_response())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn civil_date_from_days_matches_known_reference_dates() {
        assert_eq!(civil_date_from_days(0), (1970, 1, 1));
        assert_eq!(civil_date_from_days(-1), (1969, 12, 31));
        assert_eq!(civil_date_from_days(10957), (2000, 1, 1));
        assert_eq!(civil_date_from_days(19723), (2024, 1, 1));
        // 2024 is a leap year — Feb 29 exists and Mar 1 follows it directly.
        assert_eq!(civil_date_from_days(19723 + 59), (2024, 2, 29));
        assert_eq!(civil_date_from_days(19723 + 60), (2024, 3, 1));
    }

    #[test]
    fn page_lastmod_returns_none_for_missing_file() {
        let dir = tempfile::tempdir().unwrap();
        assert_eq!(page_lastmod(dir.path(), "nonexistent", None), None);
    }

    #[test]
    fn page_lastmod_reads_real_mtime_for_existing_file() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::create_dir_all(dir.path().join("home")).unwrap();
        std::fs::write(dir.path().join("home/page.yaml"), "title: Home\n").unwrap();
        let lastmod = page_lastmod(dir.path(), "home", None).unwrap();
        // Format check (YYYY-MM-DD) rather than an exact date — the file was
        // just written by this test, so "today" is the only correct answer,
        // and asserting the literal date would make this test time-bomb.
        assert_eq!(lastmod.len(), 10);
        assert_eq!(lastmod.chars().nth(4), Some('-'));
        assert_eq!(lastmod.chars().nth(7), Some('-'));
    }

    #[test]
    fn push_sitemap_url_includes_lastmod_only_when_present() {
        let mut with_lastmod = String::new();
        push_sitemap_url(&mut with_lastmod, "https://example.com", "/page/contact", Some("2026-07-11"));
        assert_eq!(
            with_lastmod,
            "<url><loc>https://example.com/page/contact</loc><lastmod>2026-07-11</lastmod></url>"
        );

        let mut without_lastmod = String::new();
        push_sitemap_url(&mut without_lastmod, "https://example.com", "/", None);
        assert_eq!(without_lastmod, "<url><loc>https://example.com/</loc></url>");
    }
}
