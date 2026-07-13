// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

mod config;
mod content;
mod mcp;
mod render;
mod routes;
mod schema;
mod state;

use axum::{
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;
use std::path::PathBuf;
use tower_http::services::ServeDir;

/// Build the full application router. Shared by `main` and the in-process
/// route tests so the two never drift.
fn build_app(app_state: state::AppState, static_dir: PathBuf) -> Router {
    Router::new()
        // Full-page routes
        .route("/", get(routes::home::home_handler))
        // Spanish routes — thin paired registrations, each calling the same
        // handler family with lang="es" baked in. Round 11 (2026-07-12)
        // shipped Tier 1: home, method, disclaimers, tokens. Round 12
        // (2026-07-13) extended this to Objects/Search/full tokens entity
        // data, per operator request to translate everything except the
        // Research essays and the Key Plans pages themselves — those two
        // stay English-only, no /es counterpart registered for them.
        .route("/es", get(routes::home::home_handler_es))
        // "The Plan Room" (2026-07-09 v3 redesign) — Objects/Key Plans are
        // real server-rendered routes, not client-only modal state. Search
        // and facet filters are GET query params (?q=, ?uni=, ?mfr=, ?use=,
        // ?layout=), so Back/reload/link-sharing all work by construction.
        .route("/objects", get(routes::objects::objects_index_handler))
        .route("/es/objects", get(routes::objects::objects_index_handler_es))
        // Registered ahead of the `/objects/{slug}` wildcard for clarity —
        // axum/matchit already disambiguates a literal static segment over a
        // param segment at the same position regardless of registration
        // order, but this reads correctly either way.
        .route(
            "/objects/compare",
            get(routes::objects::objects_compare_handler),
        )
        .route(
            "/es/objects/compare",
            get(routes::objects::objects_compare_handler_es),
        )
        .route(
            "/objects/{slug}",
            get(routes::objects::object_detail_handler),
        )
        .route(
            "/es/objects/{slug}",
            get(routes::objects::object_detail_handler_es),
        )
        // Round 9 (2026-07-11): "Compositions" retired as a public concept —
        // these are, and always were, Key Plans (see catalog.rs module doc).
        // /key-plans is now the real index/detail; /compositions/* below
        // are permanent redirects for old links/bookmarks.
        .route(
            "/key-plans",
            get(routes::key_plans::key_plans_index_handler),
        )
        // Round 13 (2026-07-13): /es/key-plans* — chrome-only translation.
        // The Key Plan/Composition data itself (dimensions, bill of
        // materials, descriptions) stays English by design; only the page
        // shell, breadcrumbs, and status labels translate.
        .route(
            "/es/key-plans",
            get(routes::key_plans::key_plans_index_handler_es),
        )
        .route(
            "/key-plans/{slug}",
            get(routes::key_plans::key_plan_detail_handler),
        )
        .route(
            "/es/key-plans/{slug}",
            get(routes::key_plans::key_plan_detail_handler_es),
        )
        .route(
            "/key-plans/{slug}/o/{object}",
            get(routes::key_plans::key_plan_object_handler),
        )
        .route(
            "/es/key-plans/{slug}/o/{object}",
            get(routes::key_plans::key_plan_object_handler_es),
        )
        .route(
            "/key-plans/download/{filename}",
            get(routes::key_plans::kp_download_handler),
        )
        .route(
            "/compositions",
            get(routes::compositions::compositions_index_redirect),
        )
        .route(
            "/compositions/{slug}",
            get(routes::compositions::composition_detail_redirect),
        )
        .route(
            "/compositions/{slug}/o/{object}",
            get(routes::compositions::composition_object_redirect),
        )
        .route("/method", get(routes::about::about_handler))
        .route("/es/method", get(routes::about::about_handler_es))
        .route(
            "/disclaimers",
            get(routes::disclaimers::disclaimers_handler),
        )
        .route(
            "/es/disclaimers",
            get(routes::disclaimers::disclaimers_handler_es),
        )
        .route("/tokens", get(routes::tokens::tokens_index_handler))
        .route("/es/tokens", get(routes::tokens::tokens_index_handler_es))
        .route(
            "/tokens/{name}",
            get(routes::tokens::token_category_handler),
        )
        .route(
            "/es/tokens/{name}",
            get(routes::tokens::token_category_handler_es),
        )
        .route("/furniture", get(routes::furniture::furniture_handler))
        .route(
            "/furniture/download/bundle.zip",
            get(routes::furniture::bundle_handler),
        )
        .route(
            "/furniture/download/{filename}",
            get(routes::furniture::single_handler),
        )
        .route("/research", get(routes::research::research_index_handler))
        // Round 13 (2026-07-13): /es/research* — chrome-only translation.
        // Essay titles/body ("the Journals") stay English by design; only
        // page shell/breadcrumb translate.
        .route(
            "/es/research",
            get(routes::research::research_index_handler_es),
        )
        .route(
            "/research/{slug}",
            get(routes::research::research_item_handler),
        )
        .route(
            "/es/research/{slug}",
            get(routes::research::research_item_handler_es),
        )
        .route("/search", get(routes::search::search_handler))
        .route("/es/search", get(routes::search::search_handler_es))
        .route("/edit/{slug}", get(routes::editor::edit_get))
        .route("/edit/{slug}", post(routes::editor::edit_post))
        // Fragment routes (content-only; same handlers, X-Fragment header also works)
        .route(
            "/fragment/tokens",
            get(routes::tokens::tokens_index_fragment),
        )
        .route(
            "/fragment/tokens/{name}",
            get(routes::tokens::token_category_fragment),
        )
        .route(
            "/fragment/research",
            get(routes::research::research_fragment),
        )
        // API
        .route("/api/events", get(routes::api::sse_handler))
        .route("/api/tokens.json", get(routes::api::tokens_json_handler))
        .route("/api/validate", post(routes::api::validate_handler))
        .route("/healthz", get(routes::api::healthz))
        .route("/readyz", get(routes::api::readyz))
        // MCP endpoint (JSON-RPC 2.0 over HTTP)
        .route("/mcp", post(mcp::mcp_handler))
        // Static assets
        .nest_service("/static", ServeDir::new(static_dir))
        // Branded 404 — full site chrome, search, section links. Replaces
        // Chrome's raw error screen (2026-07 audit's "/research
        // undiscoverability and bare 404" finding).
        .fallback(not_found_handler)
        .with_state(app_state)
}

async fn not_found_handler(
    axum::extract::State(state): axum::extract::State<state::AppState>,
) -> (axum::http::StatusCode, axum::response::Html<String>) {
    let content = render::catalog::render_not_found();
    (
        axum::http::StatusCode::NOT_FOUND,
        axum::response::Html(render::shell::page_shell("Not found", "", &content, &state)),
    )
}

#[tokio::main]
async fn main() {
    let config = config::Config::from_env();
    let app_state = state::AppState::new(&config)
        .await
        .expect("AppState init failed");

    state::spawn_file_watcher(app_state.clone(), &config);

    let app = build_app(app_state, config.static_dir.clone());

    let addr: SocketAddr = config.bind;
    println!("app-privategit-bim listening on http://{addr}");
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[cfg(test)]
mod route_tests {
    use super::*;
    use axum::body::{to_bytes, Body};
    use axum::http::{Request, StatusCode};
    use tower::ServiceExt; // for `oneshot`

    fn lib_dir() -> PathBuf {
        // <crate>/../../woodfine-bim-library — the real library used in prod.
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .join("woodfine-bim-library")
    }

    fn test_config() -> config::Config {
        let lib = lib_dir();
        config::Config {
            design_system_dir: lib.clone(),
            vault_dir: lib.clone(),
            library_dir: lib.clone(),
            static_dir: PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/assets"),
            tenant: "test".into(),
            public_url: "http://127.0.0.1".into(),
            bind: "127.0.0.1:0".parse().unwrap(),
        }
    }

    async fn test_app() -> Router {
        let config = test_config();
        let state = state::AppState::new(&config).await.expect("AppState");
        build_app(state, config.static_dir.clone())
    }

    /// Returns the live `AppState` alongside a router built from it, so tests
    /// can assert a rendered route against the same source data it derives
    /// from (used by the /about + /disclaimers content-preservation check).
    async fn test_state_and_app() -> (state::AppState, Router) {
        let config = test_config();
        let state = state::AppState::new(&config).await.expect("AppState");
        let app = build_app(state.clone(), config.static_dir.clone());
        (state, app)
    }

    async fn body_string(resp: axum::response::Response) -> String {
        let bytes = to_bytes(resp.into_body(), usize::MAX).await.unwrap();
        String::from_utf8_lossy(&bytes).to_string()
    }

    #[tokio::test]
    async fn home_ssr_renders_registry_front_door() {
        // Replaces the pre-"Plan Room" home_ssr_has_both_tabs_with_real_data test, which
        // asserted the old tabbed-panel home layout (id="bim-panel-objects"/"bim-panel-compositions"
        // with embedded item listings). The v3 redesign (2026-07-09) replaced that with a compact
        // front door linking out to /objects and /key-plans (renamed from /compositions, Round 9
        // 2026-07-11) — those routes have their own coverage; this test only needs to confirm the
        // home page itself renders correctly.
        let app = test_app().await;
        let resp = app
            .oneshot(Request::get("/").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        let html = body_string(resp).await;
        assert!(html.contains("Objects"));
        assert!(html.contains("Key Plans"));
        assert!(html.contains("/objects"));
        assert!(html.contains("/key-plans"));
    }

    #[tokio::test]
    async fn furniture_redirects_to_home() {
        let app = test_app().await;
        let resp = app
            .oneshot(Request::get("/furniture").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert!(
            resp.status().is_redirection(),
            "/furniture should redirect, got {}",
            resp.status()
        );
        assert_eq!(resp.headers().get("location").unwrap(), "/");
    }

    // Round 9 (2026-07-11): `/compositions` inverted from a real index into a
    // legacy redirect once "Composition" retired as a public concept — it now
    // redirects to `/key-plans`, the real index, not to `/`.
    #[tokio::test]
    async fn compositions_redirects_to_key_plans() {
        let app = test_app().await;
        let resp = app
            .oneshot(Request::get("/compositions").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert!(
            resp.status().is_redirection(),
            "/compositions should redirect, got {}",
            resp.status()
        );
        assert_eq!(resp.headers().get("location").unwrap(), "/key-plans");
    }

    #[tokio::test]
    async fn downloads_still_work() {
        // Furniture single IFC.
        let app = test_app().await;
        let resp = app
            .oneshot(
                Request::get("/furniture/download/task-chair-steelcase-leap-v2.ifc")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        assert_eq!(
            resp.headers().get("content-type").unwrap(),
            "application/x-step"
        );

        // Furniture bundle zip.
        let app = test_app().await;
        let resp = app
            .oneshot(
                Request::get("/furniture/download/bundle.zip")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        assert_eq!(
            resp.headers().get("content-type").unwrap(),
            "application/zip"
        );

        // Key-plan IFC.
        let app = test_app().await;
        let resp = app
            .oneshot(
                Request::get("/key-plans/download/private-office-1.ifc")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn api_tokens_json_carries_catalog() {
        let app = test_app().await;
        let resp = app
            .oneshot(
                Request::get("/api/tokens.json")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        let raw = body_string(resp).await;
        let json: serde_json::Value = serde_json::from_str(&raw).unwrap();
        let cat = &json["_catalog"];
        assert_eq!(cat["objects"].as_array().unwrap().len(), 7);
        assert_eq!(cat["key-plans"].as_array().unwrap().len(), 23);
        // Raw DTCG token files still present alongside the derived catalog.
        assert!(json.get("interior").is_some());
        assert!(json.get("key-plans").is_some());
    }

    // ── 2026-07-06 interior-page family-treatment pass ──────────────────────

    /// Every interior page now opens with the shared catalog-family masthead
    /// (`.bim-cat-pagehead`) and carries its rebuilt, family-specific markers.
    #[tokio::test]
    async fn interior_routes_wear_family_chrome() {
        let (state, app) = test_state_and_app().await;
        let cat_slug = state.categories.first().expect("a category").slug.clone();

        // /tokens — masthead + numbered section headers + real category link.
        let html = get_html(&app, "/tokens").await;
        assert!(html.contains("bim-cat-pagehead"), "/tokens masthead");
        assert!(
            html.contains("bim-tokens-sechead"),
            "/tokens section header"
        );
        assert!(html.contains("BIM Object Catalog"));
        assert!(html.contains(&format!(r#"href="/tokens/{cat_slug}""#)));

        // /tokens/{name} — masthead + catalog-family chip + spec card + data table.
        let html = get_html(&app, &format!("/tokens/{cat_slug}")).await;
        assert!(html.contains("bim-cat-pagehead"), "detail masthead");
        assert!(html.contains("bim-cat-chip"), "family classification chip");
        assert!(html.contains("bim-spec-card"), "spec card retained");
        assert!(
            html.contains("bim-token-table"),
            "entity data table retained"
        );
        // Old chip class no longer emitted on the category page.
        assert!(!html.contains(r#"class="bim-chip bim-chip--accent""#));

        // /search (no query, then a guaranteed-hit query).
        let html = get_html(&app, "/search").await;
        assert!(html.contains("bim-cat-pagehead"), "/search empty masthead");
        let html = get_html(&app, "/search?q=ifc").await;
        assert!(
            html.contains("bim-cat-pagehead"),
            "/search results masthead"
        );
        assert!(
            html.contains("bim-search-result"),
            "search results rendered"
        );
        assert!(html.contains("Search results"));

        // /research index + a real research article.
        let html = get_html(&app, "/research").await;
        assert!(html.contains("bim-cat-pagehead"), "/research masthead");
        assert!(html.contains("bim-research-item"), "research list rendered");
        let html = get_html(&app, "/research/bim-design-philosophy").await;
        assert!(html.contains("bim-markdown"), "research article rendered");

        // /edit still forces its own light Carbon surface — untouched.
        let html = get_html(&app, "/edit/interior").await;
        assert!(html.contains("carbon.min.css"), "/edit keeps Carbon assets");
        assert!(!html.contains("bim-cat-pagehead"), "/edit is not restyled");
    }

    /// The /method and /disclaimers passes are visual only: the article body
    /// derived from `about_page` / `disclaimers_page` must be byte-identical
    /// to what the source data produces. Disclaimers is issuer-of-record
    /// disclosure (NI 51-102 / OSC SN 51-721) — its text is a content
    /// decision, not a design one, and must not drift.
    #[tokio::test]
    async fn about_disclaimers_body_byte_identical_to_source() {
        let (state, app) = test_state_and_app().await;

        for (path, page) in [
            ("/method", state.about_page.as_ref()),
            ("/disclaimers", state.disclaimers_page.as_ref()),
        ] {
            let html = get_html(&app, path).await;
            assert!(
                html.contains("bim-cat-pagehead"),
                "{path} gained the masthead"
            );

            // Extract the <article> region actually served.
            let open = r#"<article class="bim-article">"#;
            let start = html
                .find(open)
                .unwrap_or_else(|| panic!("{path} <article>"));
            let end = html[start..]
                .find("</article>")
                .map(|i| i + start)
                .unwrap_or_else(|| panic!("{path} </article>"));
            let article = &html[start..end];

            // Rebuild the section HTML exactly as the handlers do, straight
            // from the source page data. Round 6 (2026-07-10) P3: the
            // /method handler injects a real diagram <figure> right after
            // two specific sections (about.rs) — mirror that here via the
            // same render::svg calls, so this test still catches any
            // unintended drift in a section's own source-derived HTML while
            // not false-failing on the intentional diagram insertions.
            let mut expected = String::new();
            for section in page.sections.iter() {
                expected.push_str(&format!(
                    "<section><h2>{}</h2>{}</section>",
                    render::shell::esc(&section.heading),
                    section.body_html,
                ));
                if path == "/method" && section.heading == "The containment model" {
                    expected.push_str(&format!(
                        r#"<figure class="bim-method-figure">{svg}<figcaption>Containment drawn as containment: Building, Floor Plate, Tile and Key Plan nest inside one another at the same step, without remainder, at every scale. The Object is the one filled element in the drawing — a standalone part contained in its Key Plan, never summed into the frames around it.</figcaption></figure>"#,
                        svg = render::svg::render_containment_model_svg()
                    ));
                } else if path == "/method" && section.heading == "Key Plans and Tiles" {
                    expected.push_str(&format!(
                        r#"<figure class="bim-method-figure">{svg}<figcaption>Every Key Plan divides its depth into the same three zones, measured from the facade inward: Habitat (H) holds the 6.0 m daylight perimeter, Magazine (M) the 3.5 m of flexible depth behind it, and Corridor (C) the final 2.0 m of circulation — the chained dimension on the right shows the three depths summing to 11.5 m. The depths shown are illustrative; each Key Plan records its own.</figcaption></figure>"#,
                        svg = render::svg::render_method_zone_svg()
                    ));
                }
            }
            assert!(!expected.is_empty(), "{path} has source sections");
            assert!(
                article.contains(&expected),
                "{path} article body drifted from its source data"
            );
        }
    }

    async fn get_html(app: &Router, path: &str) -> String {
        let resp = app
            .clone()
            .oneshot(Request::get(path).body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK, "GET {path}");
        body_string(resp).await
    }
}
