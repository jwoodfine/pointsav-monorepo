// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

use axum::extract::Path;
use axum::response::Redirect;

// Round 9 (2026-07-11): `/compositions/*` retired as the public section —
// "Composition" was never a distinct data entity (every entry was 1:1 with
// a `key-plans.dtcg.json` token; the real assembly-of-Objects artifact is
// the furniture bill, already labeled "Parts list"). The real routes now
// live in `routes/key_plans.rs`; this module is a permanent legacy redirect
// for old links/bookmarks.

pub async fn compositions_index_redirect() -> Redirect {
    Redirect::permanent("/key-plans")
}

pub async fn composition_detail_redirect(Path(slug): Path<String>) -> Redirect {
    Redirect::permanent(&format!("/key-plans/{slug}"))
}

pub async fn composition_object_redirect(
    Path((slug, object_slug)): Path<(String, String)>,
) -> Redirect {
    Redirect::permanent(&format!("/key-plans/{slug}/o/{object_slug}"))
}
