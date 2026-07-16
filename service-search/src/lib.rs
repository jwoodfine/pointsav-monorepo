//! service-search — the one production search service for the Totebox monorepo.
//!
//! Architecture (per `BRIEF-workplace-comprehensive-search`, ratified 2026-07-16):
//! ONE search, three parts with no overlap —
//!
//! - **`moonshot-index`** (owned) — the trigram substring **correctness floor**: the
//!   no-silent-miss guarantee a token index structurally cannot provide.
//! - **Tantivy** (vendored, MIT) — the **BM25 ranking** + on-disk persistence + the
//!   stored body used for snippets and content verification.
//! - **`service-search`** (this crate) — **fuses** them: the Forge builds both indexes
//!   to disk and exits; the Strike loads them read-only and answers queries. Other
//!   `service-*` and the workbench log into the Strike over HTTP — none embed an index.
//!
//! Memory model: the Strike holds only the trigram postings + filenames (small) and
//! mmaps Tantivy. The content copy lives once, in Tantivy's stored `body` field; trigram
//! content candidates are verified by reading that stored field back — never from a RAM
//! copy, never from a second filesystem pass.

use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use tantivy::collector::TopDocs;
use tantivy::query::QueryParser;
use tantivy::schema::{Schema, Value, STORED, STRING, TEXT};
use tantivy::{doc, Index, IndexWriter, TantivyDocument};

pub const TRIGRAM_FILE: &str = "trigram.msix";
pub const TANTIVY_DIR: &str = "tantivy";
/// Cap the fused result set — v1 shows the top hits per band, not the whole corpus.
pub const MAX_HITS: usize = 200;

// ---------------------------------------------------------------------------
// Config
// ---------------------------------------------------------------------------

/// One indexable root. `url_prefix` qualifies doc ids so they don't collide across
/// roots (the trigram index ids are otherwise root-relative). `fs_path` is the tree.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RootSpec {
    pub url_prefix: String,
    pub fs_path: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    /// Where the Forge writes and the Strike reads the two indexes.
    pub index_path: String,
    /// Roots to index.
    pub roots: Vec<RootSpec>,
    /// Directory basenames to prune entirely (e.g. target, node_modules, vendor).
    #[serde(default)]
    pub exclude_dirs: Vec<String>,
    /// TCP bind for the Strike, e.g. "127.0.0.1:9310".
    #[serde(default = "default_bind")]
    pub bind: String,
    /// Per-file content cap (bytes); larger files are indexed by name only.
    #[serde(default = "default_max_bytes")]
    pub max_content_bytes: usize,
}

fn default_bind() -> String {
    "127.0.0.1:9310".to_string()
}
fn default_max_bytes() -> usize {
    moonshot_index::DEFAULT_MAX_CONTENT_BYTES
}

impl Config {
    pub fn from_toml_path(p: impl AsRef<Path>) -> std::io::Result<Self> {
        let text = std::fs::read_to_string(p)?;
        toml::from_str(&text)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e.to_string()))
    }
    pub fn index_path(&self) -> PathBuf {
        PathBuf::from(&self.index_path)
    }
    pub fn is_excluded_dir(&self, path: &Path) -> bool {
        path.file_name()
            .and_then(|n| n.to_str())
            .map(|n| self.exclude_dirs.iter().any(|e| e == n))
            .unwrap_or(false)
    }
}

// ---------------------------------------------------------------------------
// Result types (the Strike's JSON contract)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize)]
pub struct Hit {
    /// Root-qualified id, e.g. `_clones/project-x/src/foo.rs`.
    pub id: String,
    /// Filename / path shown to the user.
    pub name: String,
    /// Excerpt around the first content match (empty for filename-only hits).
    pub snippet: String,
    /// BM25 relevance when the hit came from the ranked layer; 0.0 for a
    /// guarantee-only hit that Tantivy's tokenizer did not surface.
    pub score: f32,
}

/// Two bands + a coverage line — the v1 UX contract.
#[derive(Debug, Clone, Serialize)]
pub struct SearchResponse {
    pub query: String,
    /// Matches in filenames / paths.
    pub filenames: Vec<Hit>,
    /// Matches in file contents.
    pub contents: Vec<Hit>,
    /// "Searched N files across R roots" — the trustworthy-zero coverage line.
    pub files_indexed: usize,
    pub roots_indexed: usize,
}

// ---------------------------------------------------------------------------
// Tantivy schema — shared by Forge (write) and Strike (read)
// ---------------------------------------------------------------------------

pub struct Fields {
    pub id: tantivy::schema::Field,
    pub path: tantivy::schema::Field,
    pub body: tantivy::schema::Field,
}

pub fn build_schema() -> (Schema, Fields) {
    let mut b = Schema::builder();
    // id: stored, exact (not tokenized) — the join key back to the trigram floor.
    let id = b.add_text_field("id", STRING | STORED);
    // path: tokenized for filename term search, stored for display.
    let path = b.add_text_field("path", TEXT | STORED);
    // body: tokenized for BM25 content search, stored so we can verify trigram
    // candidates and cut snippets without touching the filesystem again.
    let body = b.add_text_field("body", TEXT | STORED);
    let schema = b.build();
    (schema, Fields { id, path, body })
}

// ---------------------------------------------------------------------------
// Forge — build both indexes to disk, then return (caller exits to free RAM)
// ---------------------------------------------------------------------------

pub struct ForgeStats {
    pub files: usize,
    pub content_skipped: usize,
    pub roots: usize,
}

/// Build the trigram floor + Tantivy index for every configured root and persist both
/// under `config.index_path`. In-memory content is dropped as soon as it is written to
/// Tantivy; the trigram floor is saved lite (postings + names only).
pub fn forge(config: &Config) -> anyhow::Result<ForgeStats> {
    use std::io::BufWriter;

    let out = config.index_path();
    std::fs::create_dir_all(&out)?;
    let tv_dir = out.join(TANTIVY_DIR);
    // Fresh build: clear any prior Tantivy segments so ids never duplicate.
    if tv_dir.exists() {
        std::fs::remove_dir_all(&tv_dir)?;
    }
    std::fs::create_dir_all(&tv_dir)?;

    let (schema, f) = build_schema();
    let index = Index::create_in_dir(&tv_dir, schema)?;
    // 200 MB writer heap → Tantivy parallelizes indexing across its own threads.
    let mut writer: IndexWriter = index.writer(200_000_000)?;

    let mut trigram =
        moonshot_index::TrigramIndex::with_max_content_bytes(config.max_content_bytes);
    let mut stats = ForgeStats {
        files: 0,
        content_skipped: 0,
        roots: config.roots.len(),
    };

    for root in &config.roots {
        let base = PathBuf::from(&root.fs_path);
        if !base.is_dir() {
            eprintln!("forge: skipping missing root {}", root.fs_path);
            continue;
        }
        let mut stack = vec![base.clone()];
        while let Some(dir) = stack.pop() {
            let entries = match std::fs::read_dir(&dir) {
                Ok(e) => e,
                Err(_) => continue,
            };
            for entry in entries.flatten() {
                let path = entry.path();
                let ft = match entry.file_type() {
                    Ok(t) => t,
                    Err(_) => continue,
                };
                if ft.is_symlink() {
                    continue;
                }
                if ft.is_dir() {
                    let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
                    if name == ".git" || config.is_excluded_dir(&path) {
                        continue;
                    }
                    stack.push(path);
                    continue;
                }
                if !ft.is_file() {
                    continue;
                }
                // Root-qualified id so ids never collide across roots.
                let rel = path.strip_prefix(&base).unwrap_or(&path).to_string_lossy();
                let id = format!("{}/{}", root.url_prefix, rel);

                let over_cap = entry
                    .metadata()
                    .map(|m| m.len() as usize > config.max_content_bytes)
                    .unwrap_or(true);
                let content = if over_cap {
                    stats.content_skipped += 1;
                    String::new()
                } else {
                    match std::fs::read(&path) {
                        Ok(bytes) => String::from_utf8_lossy(&bytes).into_owned(),
                        Err(_) => {
                            stats.content_skipped += 1;
                            String::new()
                        }
                    }
                };

                // Trigram floor: id + name + content (content dropped after this scope).
                trigram.add_document(id.clone(), id.clone(), &content);
                // Tantivy: id (join key), path (filename terms), body (BM25 + stored).
                writer.add_document(doc!(
                    f.id => id.clone(),
                    f.path => id.clone(),
                    f.body => content,
                ))?;
                stats.files += 1;
            }
        }
    }

    writer.commit()?;

    // Persist the trigram floor lite (postings + names, no content).
    let tri_path = out.join(TRIGRAM_FILE);
    let mut w = BufWriter::new(std::fs::File::create(&tri_path)?);
    trigram.save_lite(&mut w)?;
    std::io::Write::flush(&mut w)?;

    Ok(stats)
}

// ---------------------------------------------------------------------------
// Strike — load both indexes read-only and answer queries
// ---------------------------------------------------------------------------

pub struct Strike {
    trigram: moonshot_index::TrigramIndex,
    index: Index,
    fields: Fields,
    files_indexed: usize,
    roots_indexed: usize,
}

impl Strike {
    /// Load the persisted indexes. Cheap: mmaps Tantivy and reads the small trigram
    /// sidecar — no filesystem walk, no content copy in RAM.
    pub fn load(config: &Config) -> anyhow::Result<Self> {
        use std::io::BufReader;
        let out = config.index_path();
        let mut r = BufReader::new(std::fs::File::open(out.join(TRIGRAM_FILE))?);
        let trigram = moonshot_index::TrigramIndex::load_lite(&mut r)?;
        let index = Index::open_in_dir(out.join(TANTIVY_DIR))?;
        let (_schema, fields) = build_schema();
        let files_indexed = trigram.len();
        Ok(Strike {
            trigram,
            index,
            fields,
            files_indexed,
            roots_indexed: config.roots.len(),
        })
    }

    /// Fuse ranked (Tantivy BM25) and guaranteed (trigram floor) hits into two bands.
    /// Recall is never traded for ranking: every trigram-verified hit is returned even
    /// if Tantivy's tokenizer did not surface it.
    pub fn search(&self, query: &str) -> anyhow::Result<SearchResponse> {
        use std::collections::{HashMap, HashSet};
        let reader = self.index.reader()?;
        let searcher = reader.searcher();

        // 1) Ranked layer — Tantivy BM25 over path + body.
        let parser = QueryParser::for_index(&self.index, vec![self.fields.path, self.fields.body]);
        let mut ranked: HashMap<String, (f32, String)> = HashMap::new(); // id -> (score, body)
        if let Ok(q) = parser.parse_query(query) {
            let top = searcher.search(&q, &TopDocs::with_limit(MAX_HITS))?;
            for (score, addr) in top {
                let d: TantivyDocument = searcher.doc(addr)?;
                let id = first_text(&d, self.fields.id);
                let body = first_text(&d, self.fields.body);
                ranked.insert(id, (score, body));
            }
        }

        // 2) Guaranteed layer — trigram candidates, verified against the stored body.
        let mut filenames: Vec<Hit> = Vec::new();
        let mut contents: Vec<Hit> = Vec::new();
        let ql = query.to_lowercase();
        let mut seen_content: HashSet<String> = HashSet::new();

        for cand in self.trigram.candidate_ids(query) {
            // Filename band: verified by the trigram layer itself.
            if cand.name_matches {
                let score = ranked.get(&cand.id).map(|(s, _)| *s).unwrap_or(0.0);
                filenames.push(Hit {
                    id: cand.id.clone(),
                    name: cand.name.clone(),
                    snippet: String::new(),
                    score,
                });
            }
            // Content band: verify against the body Tantivy stored for this id.
            let body = match ranked.get(&cand.id) {
                Some((_, b)) => Some(b.clone()),
                None => self.stored_body(&searcher, &cand.id)?,
            };
            if let Some(body) = body {
                if body.to_lowercase().contains(&ql) && seen_content.insert(cand.id.clone()) {
                    let score = ranked.get(&cand.id).map(|(s, _)| *s).unwrap_or(0.0);
                    contents.push(Hit {
                        id: cand.id.clone(),
                        name: cand.name.clone(),
                        snippet: snippet_around(&body, &ql),
                        score,
                    });
                }
            }
        }

        // Rank each band: BM25 desc, then name.
        let by_score = |a: &Hit, b: &Hit| {
            b.score
                .partial_cmp(&a.score)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then(a.name.cmp(&b.name))
        };
        filenames.sort_by(by_score);
        contents.sort_by(by_score);
        filenames.truncate(MAX_HITS);
        contents.truncate(MAX_HITS);

        Ok(SearchResponse {
            query: query.to_string(),
            filenames,
            contents,
            files_indexed: self.files_indexed,
            roots_indexed: self.roots_indexed,
        })
    }

    /// Read the stored body for one id back out of Tantivy (no filesystem access).
    fn stored_body(
        &self,
        searcher: &tantivy::Searcher,
        id: &str,
    ) -> anyhow::Result<Option<String>> {
        use tantivy::query::TermQuery;
        use tantivy::schema::IndexRecordOption;
        use tantivy::Term;
        let term = Term::from_field_text(self.fields.id, id);
        let q = TermQuery::new(term, IndexRecordOption::Basic);
        let top = searcher.search(&q, &TopDocs::with_limit(1))?;
        if let Some((_, addr)) = top.first() {
            let d: TantivyDocument = searcher.doc(*addr)?;
            return Ok(Some(first_text(&d, self.fields.body)));
        }
        Ok(None)
    }
}

fn first_text(d: &TantivyDocument, field: tantivy::schema::Field) -> String {
    d.get_first(field)
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string()
}

/// A short excerpt around the first occurrence of `needle` (lowercased) in `hay`.
fn snippet_around(hay: &str, needle: &str) -> String {
    const PAD: usize = 40;
    let hay_lc = hay.to_lowercase();
    let Some(pos) = hay_lc.find(needle) else {
        return String::new();
    };
    let start = floor_boundary(hay, pos.saturating_sub(PAD));
    let end = ceil_boundary(hay, (pos + needle.len() + PAD).min(hay.len()));
    let mut s = String::new();
    if start > 0 {
        s.push('…');
    }
    s.push_str(hay[start..end].trim());
    if end < hay.len() {
        s.push('…');
    }
    s
}
fn floor_boundary(s: &str, mut i: usize) -> usize {
    if i >= s.len() {
        return s.len();
    }
    while i > 0 && !s.is_char_boundary(i) {
        i -= 1;
    }
    i
}
fn ceil_boundary(s: &str, mut i: usize) -> usize {
    if i >= s.len() {
        return s.len();
    }
    while i < s.len() && !s.is_char_boundary(i) {
        i += 1;
    }
    i
}
