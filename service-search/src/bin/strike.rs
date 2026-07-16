//! strike — load the persisted index read-only and serve queries over HTTP.
//!
//! Usage: strike <config.toml>
//! Endpoints:
//!   GET /search?q=<query>   → JSON SearchResponse (two bands + coverage)
//!   GET /healthz            → "ok"

use std::sync::Arc;

use axum::{
    extract::{Query, State},
    response::Json,
    routing::get,
    Router,
};
use serde::Deserialize;
use service_search::{Config, SearchResponse, Strike};

#[derive(Deserialize)]
struct SearchParams {
    q: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config_path = std::env::args().nth(1).unwrap_or_else(|| {
        eprintln!("usage: strike <config.toml>");
        std::process::exit(2);
    });
    let config = Config::from_toml_path(&config_path)?;
    let bind = config.bind.clone();

    eprintln!("strike: loading index from {}", config.index_path);
    let strike = Arc::new(Strike::load(&config)?);
    eprintln!("strike: ready, listening on {bind}");

    let app = Router::new()
        .route("/search", get(search))
        .route("/healthz", get(|| async { "ok" }))
        .with_state(strike);

    let listener = tokio::net::TcpListener::bind(&bind).await?;
    axum::serve(listener, app).await?;
    Ok(())
}

async fn search(
    State(strike): State<Arc<Strike>>,
    Query(params): Query<SearchParams>,
) -> Json<SearchResponse> {
    // Blocking CPU work (Tantivy + trigram) off the async reactor.
    let resp = tokio::task::spawn_blocking(move || strike.search(&params.q))
        .await
        .unwrap_or_else(|e| Err(anyhow::anyhow!("search task panicked: {e}")))
        .unwrap_or_else(|e| SearchResponse {
            query: String::new(),
            filenames: Vec::new(),
            contents: Vec::new(),
            files_indexed: 0,
            roots_indexed: 0,
        }
        .with_error(e));
    Json(resp)
}

// Small ergonomic helper so a query error still returns a well-formed (empty) response
// rather than a 500 — the UI shows a trustworthy zero, never a broken state.
trait WithError {
    fn with_error(self, e: anyhow::Error) -> SearchResponse;
}
impl WithError for SearchResponse {
    fn with_error(mut self, e: anyhow::Error) -> SearchResponse {
        eprintln!("strike: search error: {e}");
        self.query = format!("(error) {e}");
        self
    }
}
