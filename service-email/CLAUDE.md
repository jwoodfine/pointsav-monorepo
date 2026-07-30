# CLAUDE.md — service-email

> **State:** Active  —  **Last updated:** 2026-07-14
> **Version:** 0.0.1  (per `~/Foundry/CLAUDE.md` §7 and DOCTRINE.md §VIII)
> **Registry row:** `pointsav-monorepo/.claude/rules/project-registry.md`
>
> When state changes, update this header AND the registry row in the
> same commit. Drift between the two is a documentation defect.
>
> Per-commit: bump PATCH; tag `vservice-email-MAJOR.MINOR.PATCH`
> annotated and SSH-signed; commit message ends with
> `Version: M.m.P` trailer; `CHANGELOG.md` records one line per PATCH.

---

## What this project is

Ring 1 boundary-ingest service: the per-tenant Communications
Ledger. Pulls messages from upstream Microsoft Exchange mailboxes,
parses `.eml` payloads, attaches them to identities resolved
through `service-people`, and writes them through `service-fs`
into the per-tenant WORM Immutable Ledger. Read downstream by
`service-extraction` (Ring 2) via MCP wire protocol.

Sibling crates in this cluster carry protocol-specific adapters
and ancillary functions:

- `service-email-egress/` — real, pre-existing multi-binary tool
  suite with genuine CRM/roster/corpus data ledgers under
  `data-ledgers/` and EWS/IMAP reference material
  (`README.ews.md`, `README.imap.md`, `egress-*` sub-binaries).
  **Correction (2026-07-14): the `service-email-egress-ews/` and
  `service-email-egress-imap/` paths this file previously
  referenced do not exist on disk — only `service-email-egress/`
  does, containing both protocols' material together.** Treat
  `service-email-egress/`'s `data-ledgers/` as real operational
  data, not a code reference to copy from casually.
- `service-email-template/` — message-template rendering.

## Current state

**Auth surface rebased 2026-07-14** — the operator-confirmed
2026-04-25 decision (in-process Graph OAuth is drift; auth must
run out-of-process) is now implemented: `src/auth.rs` no longer
performs an OAuth `client_credentials` handshake. It reads a
pre-acquired token directly from `AZURE_ACCESS_TOKEN`
(`auth::token_from_env()`). `src/graph_client.rs` still calls
Microsoft Graph REST endpoints (not literal EWS SOAP) — a true
protocol swap to EWS SOAP, matching `service-email-egress/`'s
reference material, remains open; only the auth *surface*
(out-of-process token, no inline handshake) was rebased this pass,
per the hard constraint this file already stated.

**Folder scoping added 2026-07-14** (`BRIEF-os-totebox-platform.md`
§14 #21) — `graph_client::folder_messages_url()` scopes the pull to
one mailbox folder (`EXCHANGE_TARGET_FOLDER` env var, default
`inbox`) instead of the whole mailbox. Deliberate minimum-viable
shape: one folder, one mailbox, a concrete bounded demonstration of
external data flowing into the DataGraph — not a full mailbox
crawl.

**service-fs landing added 2026-07-14** — new `src/fs_client.rs`
(`FsClient`) posts every fetched message to service-fs's
`POST /v1/append` (`SERVICE_EMAIL_FS_ENDPOINT`, default
`http://127.0.0.1:9100`; `SERVICE_EMAIL_MODULE_ID` required,
sent as `X-Foundry-Module-ID`). This is *additive* to the existing
`maildir::MaildirVault` write, not a replacement — per this file's
own "do not delete existing paths until reviewed" discipline.
Payload is the raw, unmodified Graph message JSON — structural
pass-through only, no classification (SYS-ADR-07).

**Not yet done / explicitly out of scope for this pass:**
- Identity resolution through `service-people` (sender/recipient →
  canonical identity) — the "What this project is" section above
  describes this as the target shape; this pass did not implement
  it. Scoped out deliberately (operator: "minimum viable... a good
  way to start", not the full Ring 1 identity pipeline).
- The EWS SOAP protocol swap itself (Graph REST → EWS SOAP) —
  still open, see above.
- **Live end-to-end validation** — this pass has no real Exchange
  mailbox or `AZURE_ACCESS_TOKEN` available to test against.
  Verified instead: the built binary starts, logs the scoped
  target correctly, and reports the missing-token condition
  clearly (`SYSTEM ERROR: Token acquisition failed: ...`); all
  three new/changed modules have unit tests using `wiremock`
  against mocked Graph/service-fs endpoints (6/6 pass, clippy
  clean). A real mailbox test is a follow-up, not done here.

The Tokio async runtime model already in `service-email/src/main.rs`
is fine — it matches the ratified Ring 1 hosted-process intent
(`~/Foundry/conventions/zero-container-runtime.md`).

**Workspace membership fixed 2026-07-14** — this crate was not a
member of the root workspace (`Cargo.toml`), so `cargo check`
failed outright regardless of any of the above; this file's own
"Build and test" section below was documenting a command that
didn't actually work. Added to root workspace `members`.

Pre-framework sub-directory inventory (2026-04-26; decisions in NEXT.md):

- `ingress-harvester/` — Rust async; EWS SOAP email harvester using
  inline OAuth client_credentials from `auth-credentials.env` +
  hardcoded folder IDs; retire-pending (deprecated auth pattern)
- `master-harvester-rs/` — Rust async; Graph API email fetcher with
  dynamic folder discovery + BATCH_SIZE=3 micro-batching; retire-pending
  (Graph API approach deprecated by EWS rebase; folder-discovery +
  micro-batching patterns worth porting)
- `sovereign-splinter/` — Rust binary; mailparse-based `.eml` parser
  that routes to `service-people/discovery-queue` (identity signals) +
  `service-slm/transient-queues` (body text) + `assets/inert-media`
  (attachments); "sovereign" prefix is Do-Not-Use; core parsing logic
  kept — superseded routing will be replaced by MCP append calls
- `scripts/` — correctly placed per repo-layout.md; contains
  `spool-daemon.sh` (watches maildir/new/, calls sovereign-splinter)
- `docs/TEMPLATE_INDEX_MSFT_ENTRA_ID.md` — Entra ID auth index
  template; moved from root to docs/ (repo-layout.md compliance)

Inventory + decisions (keep / rename / retire / relocate) for those
items run alongside the auth rebase.

## Build and test

**Corrected 2026-07-14**: this crate is now a root-workspace
member. Run from the repo root, not standalone:

```
cargo test -p service-email    # 6 tests
cargo clippy -p service-email --all-targets -- -D warnings
```

End-to-end testing against a real Exchange mailbox + valid
`AZURE_ACCESS_TOKEN` still has no automated harness — unchanged
from before. Unit-level coverage (6 tests, `wiremock`-mocked Graph
API + service-fs endpoints) does not depend on a live mailbox.

## File layout

```
service-email/
├── Cargo.toml
├── README.md, README.es.md
├── CLAUDE.md, NEXT.md
├── src/
│   ├── main.rs         — Tokio daemon loop; wires auth + graph_client + fs_client + maildir
│   ├── auth.rs         — reads pre-acquired AZURE_ACCESS_TOKEN from env (rebased 2026-07-14)
│   ├── graph_client.rs — Graph REST client; folder_messages_url() scopes to one folder
│   ├── fs_client.rs    — POSTs to service-fs /v1/append (added 2026-07-14)
│   └── maildir.rs      — Maildir vault writer (still the parallel local sink)
├── docs/
│   └── TEMPLATE_INDEX_MSFT_ENTRA_ID.md  — Entra ID auth index template
├── ingress-harvester/  — pre-framework; retire-pending (old inline OAuth)
├── master-harvester-rs/ — pre-framework; retire-pending (Graph API deprecated)
├── sovereign-splinter/ — pre-framework; keep parsing core; Do-Not-Use prefix
└── scripts/
    └── spool-daemon.sh — maildir watcher; calls sovereign-splinter
```

## Hard constraints — do not violate

- **ADR-07: zero AI in Ring 1.** No LLM-assisted parsing, no
  embedding-based message classification, no AI-driven sender
  resolution. Identity resolution is delegated to `service-people`
  (also Ring 1, also deterministic) — not yet wired in, see
  "Current state" above.
- **WORM via `service-fs`.** ✅ Implemented 2026-07-14 —
  `fs_client::FsClient` posts every message to service-fs's
  `POST /v1/append`. Additive to the existing
  `maildir::MaildirVault` write, which stays as the parallel local
  sink — not yet removed.
- **Per-tenant boundary.** One process per `moduleId`. No
  cross-tenant mailbox ingestion.
- **Auth runs out-of-process.** ✅ Implemented 2026-07-14 —
  `AZURE_ACCESS_TOKEN` is consumed from env via
  `auth::token_from_env()`; the daemon does not perform an OAuth
  handshake inline. Token refresh is upstream concern.
- **Do not delete the existing Graph code paths until the EWS
  rebase compiles and is reviewed.** Still true — the auth
  *surface* was rebased (see above); the protocol itself is still
  Graph REST, not EWS SOAP. That swap is still open.

## Dependencies on other projects

- Reads from: upstream Microsoft Exchange (per-tenant mailboxes;
  currently via Graph REST, folder-scoped — EWS SOAP protocol swap
  still open, see "Current state").
- Resolves identities through: `service-people` (Ring 1, this
  cluster) — target shape; not yet wired in (see "Current state").
- Writes to: `service-fs` (Ring 1, this cluster) — implemented
  2026-07-14 via `fs_client::FsClient`, additive to the existing
  maildir sink.
- Read by: `service-extraction` (Ring 2, `project-slm` cluster) —
  reads ledger entries via MCP.
- Sibling reference: `service-email-egress/` (corrected 2026-07-14 —
  see "What this project is" above) — the EWS/IMAP auth + SOAP
  payload reference for the still-open protocol swap. Consumption
  mode (cargo path-dep vs pattern lift) is a NEXT.md decision.

## What not to do

- Do not introduce a second OAuth client_credentials flow in this
  crate. The EWS rebase moves auth out-of-process; reintroducing
  inline auth is the very drift this rebase closes.
- Do not bypass `service-people` for sender/recipient resolution.
  Identity is a Ring 1 concern owned by `service-people`; this
  crate consumes its API, does not duplicate the schema.
- Do not couple `service-email`'s Cargo.toml to all four
  `service-email-egress-*` sub-crates wholesale. Pick the minimum
  surface needed for the EWS auth rebase; consume more only when
  a concrete need surfaces.
- Do not delete or rewrite the pre-framework sub-directories
  (`ingress-harvester/`, `master-harvester-rs/`,
  `sovereign-splinter/`) until they are inventoried. Some may
  carry the right thinking for the rebase.

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
