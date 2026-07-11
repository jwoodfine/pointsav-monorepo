// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::Html;
use std::collections::HashMap;

use crate::{render, state::AppState};

// Round 9 (2026-07-11): this module now holds the real Key Plan index/detail
// routes, moved from `routes/compositions.rs` — "Composition" was retired as
// a public top-line concept (it was never a distinct data entity; every
// entry here is 1:1 with a `key-plans.dtcg.json` token). `/compositions/*`
// is now the legacy-redirect module; see `routes/compositions.rs`.

pub async fn key_plans_index_handler(
    Query(params): Query<HashMap<String, String>>,
    State(state): State<AppState>,
) -> Html<String> {
    let q = params.get("q").cloned().unwrap_or_default();
    let use_case = params.get("use").map(String::as_str);
    let layout = params.get("layout").map(String::as_str);
    let content = render::catalog::render_key_plans_index(&state, &q, use_case, layout);
    Html(render::shell::page_shell(
        "Key Plans",
        "/key-plans",
        &content,
        &state,
    ))
}

fn not_found(state: &AppState) -> (StatusCode, Html<String>) {
    (
        StatusCode::NOT_FOUND,
        Html(render::shell::page_shell(
            "Not found",
            "",
            &render::catalog::render_not_found(),
            state,
        )),
    )
}

pub async fn key_plan_detail_handler(
    Path(slug): Path<String>,
    State(state): State<AppState>,
) -> Result<Html<String>, (StatusCode, Html<String>)> {
    match render::catalog::render_key_plan_detail(&state, &slug, None) {
        Some((name, content)) => Ok(Html(render::shell::page_shell(
            &name,
            &format!("/key-plans/{slug}"),
            &content,
            &state,
        ))),
        None => Err(not_found(&state)),
    }
}

pub async fn key_plan_object_handler(
    Path((slug, object_slug)): Path<(String, String)>,
    State(state): State<AppState>,
) -> Result<Html<String>, (StatusCode, Html<String>)> {
    match render::catalog::render_key_plan_detail(&state, &slug, Some(&object_slug)) {
        Some((name, content)) => Ok(Html(render::shell::page_shell(
            &name,
            &format!("/key-plans/{slug}/o/{object_slug}"),
            &content,
            &state,
        ))),
        None => Err(not_found(&state)),
    }
}

pub async fn kp_download_handler(
    Path(filename): Path<String>,
    State(state): State<AppState>,
) -> axum::response::Response {
    use axum::response::IntoResponse;

    let kp_dir = state.config.library_dir.join("key-plans");
    let safe_name = filename.replace("..", "").replace('/', "");
    let file_path = kp_dir.join(&safe_name);
    match std::fs::read(&file_path) {
        Ok(bytes) => {
            let content_type = if safe_name.ends_with(".ifc") {
                "application/x-step"
            } else if safe_name.ends_with(".dxf") {
                "image/vnd.dxf"
            } else {
                "application/octet-stream"
            };
            (
                axum::http::StatusCode::OK,
                [
                    ("Content-Type", content_type),
                    (
                        "Content-Disposition",
                        &format!("attachment; filename=\"{safe_name}\""),
                    ),
                ],
                bytes,
            )
                .into_response()
        }
        Err(_) => (axum::http::StatusCode::NOT_FOUND, "file not found").into_response(),
    }
}
