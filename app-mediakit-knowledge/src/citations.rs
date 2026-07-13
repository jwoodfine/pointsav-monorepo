// SPDX-License-Identifier: FSL-1.1-ALv2
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! `citations.yaml` registry resolver (Phase 3.2 of `KNOWLEDGE-PLATFORM-PLAN.md`).
//!
//! Per `conventions/citation-substrate.md`, `citations.yaml` is the canonical
//! resolver every claim's `cites:` id set resolves against. The `[citations]`
//! path in `knowledge.toml` names the file; this module is the first thing in
//! this crate to actually load and parse it — before Phase 3 it was a config
//! field with nothing consuming it.

use std::collections::HashMap;
use std::path::Path;

use serde::Deserialize;

/// One `citations.yaml` entry.
#[derive(Debug, Clone, Deserialize)]
pub struct CitationEntry {
    #[serde(rename = "type")]
    pub kind: String,
    #[serde(default)]
    pub jurisdiction: Option<String>,
    pub title: String,
    pub url: String,
    #[serde(default)]
    pub last_verified: Option<String>,
    #[serde(default)]
    pub content_hash: Option<String>,
    #[serde(default)]
    pub evidence_class: Option<String>,
    #[serde(default)]
    pub aliases: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct RawRegistry {
    citations: HashMap<String, CitationEntry>,
}

/// The parsed registry — a plain lookup table, id → entry.
#[derive(Debug, Clone, Default)]
pub struct CitationRegistry {
    entries: HashMap<String, CitationEntry>,
}

impl CitationRegistry {
    /// Load and parse `citations.yaml` from `path`. Returns an empty registry
    /// (not an error) if the file is absent or malformed — a missing registry
    /// must never take the whole wiki down; callers that need to know whether
    /// resolution is actually possible check `is_empty()`.
    pub fn load(path: &Path) -> Self {
        let Ok(text) = std::fs::read_to_string(path) else {
            tracing::warn!("citations.yaml not found at {}; citation resolution disabled", path.display());
            return Self::default();
        };
        match serde_yaml::from_str::<RawRegistry>(&text) {
            Ok(raw) => Self { entries: raw.citations },
            Err(e) => {
                tracing::warn!("citations.yaml at {} failed to parse: {e}", path.display());
                Self::default()
            }
        }
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn get(&self, id: &str) -> Option<&CitationEntry> {
        self.entries.get(id)
    }

    /// Resolve a set of citation ids to their URLs, skipping any id that
    /// doesn't resolve (an unresolvable citation is a linter concern — see
    /// convention §9 — not a reason to fail rendering).
    pub fn resolve_urls(&self, ids: &[String]) -> Vec<String> {
        ids.iter().filter_map(|id| self.get(id)).map(|e| e.url.clone()).collect()
    }

    /// Every registered citation id — the re-verification scheduler's full
    /// candidate list is `all_ids() ∩ actually-cited-by-some-claim`, computed
    /// by the caller against `ClaimStore::all_cited_citation_ids`.
    pub fn all_ids(&self) -> impl Iterator<Item = &String> {
        self.entries.keys()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loads_and_resolves_real_registry_shape() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("citations.yaml");
        std::fs::write(
            &path,
            r#"
citations:
  ni-51-102:
    type: regulatory-instrument
    jurisdiction: ca-bcsc
    title: National Instrument 51-102
    url: https://example.com/ni-51-102
    last_verified: 2026-04-26
    evidence_class: regulatory-primary
    aliases:
      - "NI 51-102"
"#,
        )
        .unwrap();
        let registry = CitationRegistry::load(&path);
        assert!(!registry.is_empty());
        let entry = registry.get("ni-51-102").unwrap();
        assert_eq!(entry.url, "https://example.com/ni-51-102");
        assert_eq!(entry.aliases, vec!["NI 51-102".to_string()]);
    }

    #[test]
    fn missing_file_yields_empty_registry_not_panic() {
        let registry = CitationRegistry::load(Path::new("/nonexistent/citations.yaml"));
        assert!(registry.is_empty());
    }

    #[test]
    fn resolve_urls_skips_unknown_ids() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("citations.yaml");
        std::fs::write(
            &path,
            "citations:\n  known:\n    type: vendor-doc\n    title: Known\n    url: https://x/known\n",
        )
        .unwrap();
        let registry = CitationRegistry::load(&path);
        let urls = registry.resolve_urls(&["known".to_string(), "unknown".to_string()]);
        assert_eq!(urls, vec!["https://x/known".to_string()]);
    }

    #[test]
    fn malformed_yaml_yields_empty_registry_not_panic() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("citations.yaml");
        std::fs::write(&path, ":::not valid:::").unwrap();
        assert!(CitationRegistry::load(&path).is_empty());
    }
}
