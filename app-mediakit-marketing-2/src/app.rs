// SPDX-License-Identifier: FSL-1.1-ALv2
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! Application state and axum Router. P0 mounted `/healthz` and
//! `/static/*path`; P1 added the content pipeline; P3 wired real chrome;
//! P4 added SEO/discovery; P5 adds the MCP JSON-RPC surface + review queue
//! (mounted only when `enable_mcp` is set).

use std::path::PathBuf;
use std::sync::Arc;

use axum::extract::{Path as AxumPath, State};
use axum::http::{header, HeaderName, HeaderValue};
use axum::response::{IntoResponse, Json, Response};
use axum::routing::{get, post};
use axum::Router;
use maud::Markup;
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

async fn home(State(state): State<AppState>) -> Result<Markup, MarketingError> {
    render_slug(&state, "home", None)
}

async fn home_es(State(state): State<AppState>) -> Result<Markup, MarketingError> {
    render_slug(&state, "home", Some("es"))
}

async fn page(
    State(state): State<AppState>,
    AxumPath(slug): AxumPath<String>,
) -> Result<Markup, MarketingError> {
    render_slug(&state, &slug, None)
}

async fn page_es(
    State(state): State<AppState>,
    AxumPath(slug): AxumPath<String>,
) -> Result<Markup, MarketingError> {
    render_slug(&state, &slug, Some("es"))
}

fn render_slug(
    state: &AppStateInner,
    slug: &str,
    lang: Option<&str>,
) -> Result<Markup, MarketingError> {
    let page = content::load_page(&state.content_dir, slug, lang)?;
    let tenant = Tenant::by_module_id(&state.module_id, &state.legal_tokens);
    let (en_path, es_path) = slug_paths(slug);
    Ok(page_shell(
        &tenant,
        &page,
        &state.module_id,
        &en_path,
        es_path.as_deref(),
        state.google_verify.as_deref(),
        &state.csp_nonce,
    ))
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
        body.push_str(&format!(
            "<url><loc>{}{}</loc></url>",
            tenant.canonical_base, en_path
        ));
        // `home` gained its own `/es` route 2026-07-12, so it's included
        // here like every other slug now (previously excluded because
        // `es_path` was `None` for it).
        if let Some(es_path) = es_path {
            if state.content_dir.join(slug).join("page.es.yaml").is_file() {
                body.push_str(&format!(
                    "<url><loc>{}{}</loc></url>",
                    tenant.canonical_base, es_path
                ));
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
