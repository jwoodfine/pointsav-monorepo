// SPDX-License-Identifier: FSL-1.1-ALv2
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! End-to-end axum handler-layer integration tests (audit finding: this
//! layer previously had zero coverage despite `tower`/`http-body-util`
//! dev-deps being declared for exactly this purpose).
//!
//! Drives `router(state)` directly via `tower::ServiceExt::oneshot`, the
//! same pattern axum's own documentation recommends for handler testing —
//! no real network socket involved.

use app_mediakit_knowledge::app::{router, AppState};
use app_mediakit_knowledge::config::Config;
use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use tower::ServiceExt;

fn write(dir: &std::path::Path, rel: &str, body: &str) {
    let p = dir.join(rel);
    std::fs::create_dir_all(p.parent().unwrap()).unwrap();
    std::fs::write(p, body).unwrap();
}

/// Build a full `AppState` over a throwaway content mount + state dir, with
/// one article, one alias, and one `redirects.yaml` entry — enough surface
/// to exercise every route this test file covers.
fn test_state() -> AppState {
    let content_dir = tempfile::tempdir().unwrap();
    let state_dir = tempfile::tempdir().unwrap();
    let root = content_dir.path();

    write(
        root,
        "architecture/zci.md",
        "---\ntitle: Zero Container Inference\nslug: zero-container-inference\ncategory: architecture\naliases:\n  - zci-old-name\n---\nBody text about zero-container inference.\n",
    );
    write(
        root,
        "redirects.yaml",
        "redirects:\n  - from: /moved-away\n    to: /wiki/zero-container-inference\n",
    );

    // Leak the tempdirs for the duration of the test process — `AppState`
    // holds owned PathBufs into these directories and this helper is only
    // ever used inside #[test] functions, each with its own throwaway state.
    let content_path = root.to_path_buf();
    let state_path = state_dir.path().to_path_buf();
    std::mem::forget(content_dir);
    std::mem::forget(state_dir);

    let toml = format!(
        "[site]\ntitle = \"Test Wiki\"\nbrand = \"pointsav\"\nbind = \"127.0.0.1:0\"\nstate_dir = \"{}\"\ninstance = \"documentation\"\n\n[[mount]]\npath = \"{}\"\nrole = \"primary\"\n",
        state_path.display(),
        content_path.display(),
    );
    let config: Config = toml::from_str(&toml).unwrap();
    AppState::build(config)
}

async fn body_text(resp: axum::response::Response) -> String {
    let bytes = resp.into_body().collect().await.unwrap().to_bytes();
    String::from_utf8_lossy(&bytes).to_string()
}

#[tokio::test]
async fn home_page_returns_200() {
    let app = router(test_state());
    let resp = app
        .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
}

#[tokio::test]
async fn healthz_returns_200_ok() {
    let app = router(test_state());
    let resp = app
        .oneshot(
            Request::builder()
                .uri("/healthz")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
}

#[tokio::test]
async fn known_article_renders_200_with_title() {
    let app = router(test_state());
    let resp = app
        .oneshot(
            Request::builder()
                .uri("/wiki/zero-container-inference")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let text = body_text(resp).await;
    assert!(text.contains("Zero Container Inference"), "got: {text}");
}

#[tokio::test]
async fn unknown_slug_returns_chrome_wrapped_404() {
    let app = router(test_state());
    let resp = app
        .oneshot(
            Request::builder()
                .uri("/wiki/does-not-exist-anywhere")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    let text = body_text(resp).await;
    assert!(text.to_lowercase().contains("not found"));
}

#[tokio::test]
async fn alias_redirects_301_to_canonical_slug() {
    let app = router(test_state());
    let resp = app
        .oneshot(
            Request::builder()
                .uri("/wiki/zci-old-name")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::MOVED_PERMANENTLY);
    let location = resp.headers().get("location").unwrap().to_str().unwrap();
    assert_eq!(location, "/wiki/zero-container-inference");
}

#[tokio::test]
async fn redirects_yaml_entry_redirects_301() {
    let app = router(test_state());
    let resp = app
        .oneshot(
            Request::builder()
                .uri("/wiki/moved-away")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::MOVED_PERMANENTLY);
    let location = resp.headers().get("location").unwrap().to_str().unwrap();
    assert_eq!(location, "/wiki/zero-container-inference");
}

#[tokio::test]
async fn search_with_no_query_returns_200() {
    let app = router(test_state());
    let resp = app
        .oneshot(
            Request::builder()
                .uri("/search")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
}

#[tokio::test]
async fn search_with_query_matching_content_returns_the_article() {
    let app = router(test_state());
    let resp = app
        .oneshot(
            Request::builder()
                .uri("/search?q=zero-container")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let text = body_text(resp).await;
    assert!(text.contains("Zero Container Inference"), "got: {text}");
}

#[tokio::test]
async fn bogus_rev_query_on_article_returns_404_not_a_crash() {
    // Also exercises the ?rev= SHA-format validation fixed alongside this
    // test suite — a git-revspec-shaped (not a plain SHA) value must be
    // rejected the same way an unresolvable-but-SHA-shaped value is.
    let app = router(test_state());
    let resp = app
        .oneshot(
            Request::builder()
                .uri("/wiki/zero-container-inference?rev=HEAD~3")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn json_content_negotiation_returns_structured_body() {
    let app = router(test_state());
    let resp = app
        .oneshot(
            Request::builder()
                .uri("/wiki/zero-container-inference")
                .header("accept", "application/json")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let text = body_text(resp).await;
    let json: serde_json::Value = serde_json::from_str(&text).unwrap();
    assert_eq!(json["frontmatter"]["title"], "Zero Container Inference");
    assert!(json.get("body_md").is_some());
    assert!(json.get("blake3").is_some());
}

#[tokio::test]
async fn unknown_static_asset_returns_404() {
    let app = router(test_state());
    let resp = app
        .oneshot(
            Request::builder()
                .uri("/static/does-not-exist.css")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}
