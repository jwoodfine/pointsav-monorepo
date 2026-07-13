// SPDX-License-Identifier: FSL-1.1-ALv2
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! Continuous citation verification (Phase 3.4 of `KNOWLEDGE-PLATFORM-PLAN.md`).
//!
//! A background task re-fetches every cited external URL on an interval,
//! re-hashes the body, and records drift (a changed hash since the last
//! check) via `ClaimStore`. Drift is surfaced in the article chrome's
//! citation ribbon (`ui::layout`) by reading `ClaimStore::drifted_citations`.
//!
//! This module never blocks a request — it runs entirely on its own
//! `tokio::spawn`'d task, started once at `serve()` startup.

use std::time::Duration;

use crate::citations::CitationRegistry;
use crate::claims_store::{CitationVerification, ClaimStore, VerificationStatus};

/// How often the scheduler sweeps every cited citation. A wiki's cited
/// sources change slowly; checking more than once a day is unnecessary load
/// on the sources themselves.
pub const VERIFY_INTERVAL: Duration = Duration::from_secs(24 * 60 * 60);

/// Per-fetch timeout — a slow/hanging source must not stall the whole sweep.
const FETCH_TIMEOUT: Duration = Duration::from_secs(15);

/// Spawn the background re-verification loop. Fire-and-forget: the returned
/// `JoinHandle` is intentionally not awaited by the caller (the task runs
/// for the service's lifetime); callers that want graceful shutdown can
/// abort it explicitly via the handle.
pub fn spawn_scheduler(store: ClaimStore, registry: CitationRegistry) -> tokio::task::JoinHandle<()> {
    tokio::spawn(async move {
        let client = match reqwest::Client::builder().timeout(FETCH_TIMEOUT).build() {
            Ok(c) => c,
            Err(e) => {
                tracing::error!("citation verification scheduler could not build HTTP client: {e}; scheduler disabled");
                return;
            }
        };
        let mut interval = tokio::time::interval(VERIFY_INTERVAL);
        // The first tick fires immediately; skip it so we don't hammer every
        // cited source at process start on every restart.
        interval.tick().await;
        loop {
            interval.tick().await;
            run_sweep(&client, &store, &registry).await;
        }
    })
}

/// Run one verification sweep now (also used directly by tests and by a
/// future on-demand admin trigger, without waiting for the interval).
pub async fn run_sweep(client: &reqwest::Client, store: &ClaimStore, registry: &CitationRegistry) {
    let cited_ids = match store.all_cited_citation_ids() {
        Ok(ids) => ids,
        Err(e) => {
            tracing::error!("citation verification sweep: could not list cited citation ids: {e}");
            return;
        }
    };
    tracing::info!("citation verification sweep: checking {} cited source(s)", cited_ids.len());
    for citation_id in cited_ids {
        let Some(entry) = registry.get(&citation_id) else {
            // Cited by a claim but not (or no longer) in citations.yaml —
            // an editorial-linter concern (convention §9), not this
            // scheduler's to fix; skip.
            continue;
        };
        let verification = verify_one(client, &citation_id, &entry.url, store).await;
        if let Err(e) = store.record_verification(&verification) {
            tracing::error!("citation verification sweep: failed to record result for {citation_id}: {e}");
        }
        if verification.status == VerificationStatus::Drifted {
            tracing::warn!("citation drift detected: {citation_id} ({})", entry.url);
        }
    }
}

/// Fetch, hash, and compare one citation's URL against its previously
/// recorded hash (if any). Never panics on network failure — an
/// unreachable source is recorded as `Unreachable`, not a crash.
async fn verify_one(client: &reqwest::Client, citation_id: &str, url: &str, store: &ClaimStore) -> CitationVerification {
    let now = now_iso();
    let previous_hash = store
        .get_verification(citation_id)
        .ok()
        .flatten()
        .and_then(|v| v.content_hash);

    match client.get(url).send().await {
        Ok(resp) if resp.status().is_success() => match resp.text().await {
            Ok(body) => {
                let content_hash = blake3::hash(body.as_bytes()).to_hex().to_string();
                let status = match &previous_hash {
                    Some(prev) if *prev != content_hash => VerificationStatus::Drifted,
                    _ => VerificationStatus::Ok,
                };
                CitationVerification {
                    citation_id: citation_id.to_string(),
                    url: url.to_string(),
                    last_checked: now,
                    content_hash: Some(content_hash),
                    status,
                }
            }
            Err(_) => CitationVerification {
                citation_id: citation_id.to_string(),
                url: url.to_string(),
                last_checked: now,
                content_hash: None,
                status: VerificationStatus::Unreachable,
            },
        },
        _ => CitationVerification {
            citation_id: citation_id.to_string(),
            url: url.to_string(),
            last_checked: now,
            content_hash: None,
            status: VerificationStatus::Unreachable,
        },
    }
}

/// Current UTC time as an ISO 8601 string, without pulling in a full
/// date/time crate — matches this crate's existing style of plain string
/// dates (`history.rs`'s `date_iso` is likewise a hand-formatted string).
fn now_iso() -> String {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default();
    let secs = now.as_secs();
    // Minimal civil-from-days conversion (Howard Hinnant's algorithm),
    // avoiding a new chrono/time dependency for one timestamp format.
    let days = secs / 86_400;
    let rem = secs % 86_400;
    let (h, m, s) = (rem / 3600, (rem % 3600) / 60, rem % 60);
    let (y, mo, d) = civil_from_days(days as i64);
    format!("{y:04}-{mo:02}-{d:02}T{h:02}:{m:02}:{s:02}Z")
}

/// Days-since-epoch to (year, month, day). Public-domain algorithm, Howard
/// Hinnant, "chrono-Compatible Low-Level Date Algorithms".
fn civil_from_days(z: i64) -> (i64, u32, u32) {
    let z = z + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = (z - era * 146_097) as u64;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146_096) / 365;
    let y = yoe as i64 + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let m = if mp < 10 { mp + 3 } else { mp - 9 } as u32;
    (if m <= 2 { y + 1 } else { y }, m, d)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn now_iso_is_well_formed() {
        let s = now_iso();
        // "YYYY-MM-DDTHH:MM:SSZ" — 20 chars.
        assert_eq!(s.len(), 20);
        assert!(s.ends_with('Z'));
        assert_eq!(&s[4..5], "-");
        assert_eq!(&s[7..8], "-");
        assert_eq!(&s[10..11], "T");
    }

    #[test]
    fn civil_from_days_epoch_is_1970_01_01() {
        assert_eq!(civil_from_days(0), (1970, 1, 1));
    }

    #[test]
    fn civil_from_days_known_date() {
        // 2000-03-01 is day 11017 since epoch (well-known reference point
        // for the Hinnant algorithm's test suite).
        assert_eq!(civil_from_days(11017), (2000, 3, 1));
    }

    #[tokio::test]
    async fn run_sweep_with_no_cited_ids_does_nothing() {
        let dir = tempfile::tempdir().unwrap();
        let store = ClaimStore::open(&dir.path().join("claims.redb")).unwrap();
        let registry = CitationRegistry::default();
        let client = reqwest::Client::new();
        // Must not panic or hang with an empty store.
        run_sweep(&client, &store, &registry).await;
        assert!(store.drifted_citations().unwrap().is_empty());
    }

    #[tokio::test]
    async fn verify_one_records_unreachable_for_a_bad_host() {
        let dir = tempfile::tempdir().unwrap();
        let store = ClaimStore::open(&dir.path().join("claims.redb")).unwrap();
        let client = reqwest::Client::builder()
            .timeout(Duration::from_millis(500))
            .build()
            .unwrap();
        let v = verify_one(&client, "test-cite", "http://127.0.0.1.invalid.example/nope", &store).await;
        assert_eq!(v.status, VerificationStatus::Unreachable);
        assert!(v.content_hash.is_none());
    }
}
