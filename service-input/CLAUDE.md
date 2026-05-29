# CLAUDE.md — service-input

> **State:** Active  —  **Last updated:** 2026-05-29
> **Version:** 0.0.1  (per `~/Foundry/CLAUDE.md` §7 and DOCTRINE.md §VIII)
> **Registry row:** `pointsav-monorepo/.claude/rules/project-registry.md`
>
> When state changes, update this header AND the registry row in the
> same commit. Drift between the two is a documentation defect.
>
> Per-commit: bump PATCH; tag `vservice-input-MAJOR.MINOR.PATCH`
> annotated and SSH-signed; commit message ends with
> `Version: M.m.P` trailer; `CHANGELOG.md` records one line per PATCH.

---

## What this project is

Ring 1 boundary-ingest service for generic document intake. Accepts
files of supported formats at the per-tenant boundary, dispatches
them to format-specific parsers, normalises the parsed payload, and
writes through `service-fs` into the per-tenant WORM Immutable
Ledger. Sibling to `service-people` (identity ingest) and
`service-email` (Communications Ledger). Read downstream by
`service-extraction` (Ring 2) via MCP wire protocol.

## Current state

**Parser suite complete. 33 tests pass. Workspace member.**

Full Ring 1 ingest pipeline operational as of 2026-05-20:

- **4 parsers:** PDF (oxidize-pdf 2.x, temp-file shim), Markdown
  (pulldown-cmark 0.12, pure-text), DOCX (docx-rust 0.1.x,
  Cursor reader), XLSX (calamine 0.34, Cursor reader). All
  implement the `Parser` trait; magic-byte + extension detection
  in `detect_format`.
- **FsClient** (`src/fs_client.rs`) — `submit(doc)` POSTs to
  `service-fs /v1/append` via ureq 3.3 blocking; `X-Foundry-Module-ID`
  header enforcement.
- **MCP server** (`src/mcp.rs` + `src/http.rs`) — `document.ingest`
  tool (filename, source_id, bytes_base64); `X-Foundry-Module-ID`
  enforcement; axum on `INPUT_BIND_ADDR` (default 0.0.0.0:9200).
- **33 tests:** 32 unit + 1 end-to-end integration test
  (`tests/parse_to_fs_round_trip.rs` — spins up real service-fs
  PosixTileLedger on ephemeral port, drives document.ingest, asserts
  WORM round-trip).
- **Workspace member** — added to root `Cargo.toml` `[members]`
  after openssl-sys Layer 1 audit issue was resolved.

Not yet deployed as a systemd unit. Binary build + `local-input.service`
is a Command Session task (pattern: `local-email.service`).

## Build and test

```
cargo test --manifest-path service-input/Cargo.toml
# 33 tests: format detection + dispatcher (12), multi-parser integration (1),
# PdfParser (2), MarkdownParser (5), DocxParser (2), XlsxParser (2),
# FsClient integration (1), MCP handler (5), parse_to_fs_round_trip (1 integration).
```

Also covered by workspace-level `cargo check --workspace` (service-input is a
declared workspace member as of 2026-05-29).

## File layout

```
service-input/
├── Cargo.toml            — crate manifest; serde + serde_json today;
│                           parser crates (oxidize-pdf, docx-rust,
│                           calamine, pulldown-cmark) added as each
│                           parser is wired
├── README.md, README.es.md — bilingual overview
├── CLAUDE.md, NEXT.md
└── src/
    ├── lib.rs            — Format enum, ParsedDocument struct,
    │                       ParseError enum, Parser trait,
    │                       Dispatcher (per-format registry +
    │                       dispatch + dispatch_with_detection),
    │                       detect_format (extension-first, magic-
    │                       byte fallback). 12 unit tests + 1
    │                       multi-parser integration test. Re-exports
    │                       PdfParser, MarkdownParser, DocxParser,
    │                       XlsxParser, FsClient, FsClientError.
    ├── http.rs           — AppState { module_id, dispatcher, fs_client }
    │                       + router() (GET /healthz, /readyz, POST /mcp).
    │                       Re-exported as service_input::http.
    ├── mcp.rs            — MCP JSON-RPC 2.0 handler. Tool: document.ingest
    │                       (filename, source_id, bytes_base64). Detects
    │                       format, dispatches to parser, submits via
    │                       FsClient. X-Foundry-Module-ID enforcement.
    │                       5 tests (initialize, tools/list, ingest
    │                       transport-error, wrong module_id, unknown
    │                       format). Re-exported as service_input::mcp.
    ├── fs_client.rs      — FsClient { base_url, module_id } with
    │                       submit(&self, doc) -> Result<u64, FsClientError>.
    │                       POSTs to service-fs /v1/append; ureq 3.3
    │                       blocking (json feature). Integration test
    │                       spins up real axum server on port 0.
    │                       Re-exported as service_input::FsClient +
    │                       FsClientError from lib.rs.
    ├── pdf.rs            — PdfParser via oxidize-pdf 2.x. Temp-file
    │                       shim (oxidize-pdf is file-path-only).
    │                       Returns ParsedDocument with text +
    │                       {page_count, parser: "oxidize-pdf"}.
    ├── markdown.rs       — MarkdownParser via pulldown-cmark 0.12.
    │                       Pure-text; no temp-file shim. Extracts
    │                       all text runs + headings. Returns
    │                       {headings, parser: "pulldown-cmark"}.
    ├── docx.rs           — DocxParser via docx-rust 0.1.x.
    │                       Uses DocxFile::from_reader(Cursor).
    │                       Magic check rejects non-ZIP early.
    │                       Text via body.text(); metadata:
    │                       {paragraph_count, parser: "docx-rust"}.
    └── xlsx.rs           — XlsxParser via calamine 0.34.
                            Uses open_workbook_from_rs(Cursor).
                            Magic check rejects non-ZIP early.
                            Text from all sheets, all rows, all
                            cells; space-separated per row, newline
                            between rows. Metadata: {sheet_count,
                            sheets, parser: "calamine"}. 2 tests.
                            Uses DocxFile::from_reader(Cursor)
                            (no temp-file shim — reader API accepts
                            Read + Seek). Magic check rejects non-ZIP
                            input early. Text via body.text();
                            metadata: {paragraph_count, parser:
                            "docx-rust"}. 2 tests: non-ZIP →
                            FormatMismatch; empty-ZIP →
                            ParserInternal.
```

## Hard constraints — do not violate

- **ADR-07: zero AI in Ring 1.** Parsing is deterministic. No
  LLM-assisted text extraction, no embedding-model normalisation,
  no AI-driven format detection. Format detection is by extension
  and magic-byte sniffing only.
- **WORM via `service-fs` only.** This crate does not persist to
  disk directly. Every parsed payload is written through
  `service-fs`'s MCP interface so the append-only invariant lives
  at one boundary.
- **Per-tenant boundary.** One process per `moduleId` (per
  `~/Foundry/conventions/three-ring-architecture.md`). No
  cross-tenant routing.
- **Format coverage starts narrow.** Initial four parsers per
  `SLM-STACK.md` §3.4: oxidize-pdf, docx-rust, calamine,
  pulldown-cmark. Expansion needs a NEXT.md item naming the
  customer use case driving it; not "for completeness."

## Dependencies on other projects

- Writes to: `service-fs` (Ring 1, this cluster) — every parsed
  payload goes here.
- Read by: `service-extraction` (Ring 2, `project-slm` cluster) —
  reads ledger entries via MCP.
- Future: customer-extension parsers may plug in as additional
  parser adapters behind the same trait, no fork of this crate
  needed.

## What not to do

- Do not import `anthropic`, `openai`, `candle-core` (for
  inference), or any other AI/ML inference dependency. Ring 1 is
  zero-AI by ADR-07.
- Do not write directly to disk. The WORM invariant lives in
  `service-fs`; bypassing it breaks append-only enforcement.
- Do not add a parser for a format until a customer use case
  surfaces it. Format coverage is driven by demand, not by
  speculative completeness.

---

## Inherited rules — do not duplicate, do not silently override

This project inherits rules from two parent scopes. Do NOT copy
their content into this file; reference them.

- **Repo-level:** `pointsav-monorepo/CLAUDE.md` (when added; the
  monorepo does not yet carry a repo-level `CLAUDE.md` — see
  `~/Foundry/NEXT.md` Stage 4) — prefix taxonomy, canonical names,
  ADR hard rules (SYS-ADR-07, -10, -19), Do-Not-Use vocabulary,
  bilingual README rule, BCSC / Sovereign Data Foundation
  disclosure.
- **Workspace-level:** `~/Foundry/CLAUDE.md` — identity store,
  commit flow (`bin/commit-as-next.sh`), promotion flow
  (`bin/promote.sh`), authoritative-document priority, rules of
  engagement.

If a rule at this level conflicts with an inherited rule, **stop
and surface the conflict** — do not silently override.
