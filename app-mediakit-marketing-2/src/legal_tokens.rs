// SPDX-License-Identifier: FSL-1.1-ALv2
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! Loads canonical trademark/copyright facts from
//! `factory-release-engineering/tokens/legal-tokens-<brand>.yaml` at startup,
//! so the chrome's copyright and trademark lines come from the single
//! canonical source instead of being hardcoded in `ui.rs`. See
//! `.agent/drafts-outbound/LEGAL-RECONCILIATION-token-source-of-truth.draft.md`
//! for the reconciliation work this consumes — this loader picks up whatever
//! that draft lands upstream with no code change required here.

use std::path::{Path, PathBuf};

use serde::Deserialize;

const EXPECTED_SCHEMA: &str = "foundry-legal-tokens-v1";

#[derive(Debug, Clone, Deserialize)]
pub struct LegalTokens {
    pub schema: String,
    pub brand: String,
    pub copyright: Copyright,
    pub website: Website,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Copyright {
    pub holder: String,
    pub year_current: i32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Website {
    pub footer_trademark_en: String,
    pub footer_trademark_es: String,
}

#[derive(Debug, thiserror::Error)]
pub enum LegalTokensError {
    #[error("could not read legal tokens file {path}: {source}")]
    Io {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("could not parse legal tokens file {path}: {source}")]
    Parse {
        path: PathBuf,
        #[source]
        source: serde_yaml::Error,
    },
    #[error("legal tokens file {path} has schema {found:?}, expected {EXPECTED_SCHEMA:?}")]
    UnexpectedSchema { path: PathBuf, found: String },
}

impl LegalTokens {
    /// Load `<dir>/legal-tokens-<module_id>.yaml`. `module_id` is the same
    /// tenant selector already used for chrome (`"woodfine"`, `"pointsav"`).
    /// Fails loudly (propagates to the caller, which aborts startup) rather
    /// than silently falling back — a missing/malformed canonical source is
    /// a real problem, not something to paper over with stale defaults.
    pub fn load(dir: &Path, module_id: &str) -> Result<Self, LegalTokensError> {
        let path = dir.join(format!("legal-tokens-{module_id}.yaml"));
        let raw = std::fs::read_to_string(&path).map_err(|source| LegalTokensError::Io {
            path: path.clone(),
            source,
        })?;
        let tokens: LegalTokens =
            serde_yaml::from_str(&raw).map_err(|source| LegalTokensError::Parse {
                path: path.clone(),
                source,
            })?;
        if tokens.schema != EXPECTED_SCHEMA {
            return Err(LegalTokensError::UnexpectedSchema {
                path,
                found: tokens.schema,
            });
        }
        Ok(tokens)
    }

    /// `"© 2026 Woodfine Capital Projects Inc."` — the year/holder half of
    /// the footer copyright line. The "All rights reserved." tail stays
    /// language-switched by the caller via `t()`, same as before this loader
    /// existed — that phrase isn't part of the canonical token schema.
    pub fn copyright_line(&self) -> String {
        format!("\u{00a9} {} {}", self.copyright.year_current, self.copyright.holder)
    }

    pub fn trademark_line_en(&self) -> &str {
        self.website.footer_trademark_en.trim()
    }

    pub fn trademark_line_es(&self) -> &str {
        self.website.footer_trademark_es.trim()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn write_sample(dir: &Path, module_id: &str, holder: &str, year: i32, trademark_en: &str, trademark_es: &str) {
        let contents = format!(
            "schema: {EXPECTED_SCHEMA}\nbrand: {module_id}\ncopyright:\n  holder: \"{holder}\"\n  year_current: {year}\nwebsite:\n  footer_trademark_en: \"{trademark_en}\"\n  footer_trademark_es: \"{trademark_es}\"\n"
        );
        std::fs::write(dir.join(format!("legal-tokens-{module_id}.yaml")), contents).unwrap();
    }

    #[test]
    fn loads_valid_tokens() {
        let dir = tempfile::tempdir().unwrap();
        write_sample(dir.path(), "woodfine", "Test Holder Inc.", 2030, "Test Mark\u{2122}", "marca de prueba");
        let tokens = LegalTokens::load(dir.path(), "woodfine").unwrap();
        assert_eq!(tokens.brand, "woodfine");
        assert_eq!(tokens.copyright.holder, "Test Holder Inc.");
        assert_eq!(tokens.copyright.year_current, 2030);
        assert_eq!(tokens.trademark_line_en(), "Test Mark\u{2122}");
        assert_eq!(tokens.trademark_line_es(), "marca de prueba");
    }

    #[test]
    fn copyright_line_formats_year_and_holder() {
        let dir = tempfile::tempdir().unwrap();
        write_sample(dir.path(), "pointsav", "Test Holder Inc.", 2030, "Mark\u{2122}", "marca");
        let tokens = LegalTokens::load(dir.path(), "pointsav").unwrap();
        assert_eq!(tokens.copyright_line(), "\u{00a9} 2030 Test Holder Inc.");
    }

    #[test]
    fn missing_file_errors_loudly_not_silently() {
        let dir = tempfile::tempdir().unwrap();
        let err = LegalTokens::load(dir.path(), "nonexistent-tenant").unwrap_err();
        assert!(matches!(err, LegalTokensError::Io { .. }));
    }

    #[test]
    fn wrong_schema_errors_loudly() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(
            dir.path().join("legal-tokens-woodfine.yaml"),
            "schema: some-other-schema-v1\nbrand: woodfine\ncopyright:\n  holder: \"X\"\n  year_current: 2030\nwebsite:\n  footer_trademark_en: \"x\"\n  footer_trademark_es: \"y\"\n",
        )
        .unwrap();
        let err = LegalTokens::load(dir.path(), "woodfine").unwrap_err();
        assert!(matches!(err, LegalTokensError::UnexpectedSchema { .. }));
    }

    #[test]
    fn malformed_yaml_errors_loudly() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("legal-tokens-woodfine.yaml"), "not: valid: yaml: [").unwrap();
        let err = LegalTokens::load(dir.path(), "woodfine").unwrap_err();
        assert!(matches!(err, LegalTokensError::Parse { .. }));
    }

    #[test]
    fn loads_real_canonical_files_if_present() {
        // Not a hard failure if the canonical repo isn't mounted at this path
        // (e.g. a laptop dev environment) — this test only asserts when it is,
        // which is always true on the workspace VM this engine actually runs on.
        let canonical_dir = Path::new("/srv/foundry/vendor/factory-release-engineering/tokens");
        if !canonical_dir.is_dir() {
            return;
        }
        for module_id in ["woodfine", "pointsav"] {
            let tokens = LegalTokens::load(canonical_dir, module_id)
                .unwrap_or_else(|e| panic!("failed to load real {module_id} tokens: {e}"));
            assert_eq!(tokens.schema, EXPECTED_SCHEMA);
            assert!(!tokens.trademark_line_en().is_empty());
            assert!(!tokens.trademark_line_es().is_empty());
        }
    }
}
