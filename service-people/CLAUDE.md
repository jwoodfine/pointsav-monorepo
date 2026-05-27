# CLAUDE.md — service-people

> **State:** Active  —  **Last updated:** 2026-05-27
> **Version:** 0.0.1  (per `~/Foundry/CLAUDE.md` §7 and DOCTRINE.md §VIII)
> **Registry row:** `pointsav-monorepo/.claude/rules/project-registry.md`
>
> When state changes, update this header AND the registry row in the
> same commit. Drift between the two is a documentation defect.
>
> Per-commit: bump PATCH; tag `vservice-people-MAJOR.MINOR.PATCH`
> annotated and SSH-signed; commit message ends with
> `Version: M.m.P` trailer; `CHANGELOG.md` records one line per PATCH.

---

## What this project is

Ring 1 boundary-ingest service: the per-tenant Identity Ledger.
Manages the canonical identity records (people, organisations,
roles, communication endpoints) that downstream services attach
events and documents to. Per
`~/Foundry/conventions/three-ring-architecture.md`, identity records
are persisted through `service-fs` (WORM) and read by Ring 2
services as MCP clients.

## Current state

**ACS engine absorbed, legacy code retired (2026-05-27 session 4). 31 tests pass.**

Three MCP tools over `POST /mcp` (JSON-RPC 2.0):

- `identity.append` — name + primary_email + optional aliases + organisation
  → `Person` (UUIDv5 from email) → `FsClient` → service-fs `/v1/append` + local
  `PeopleStore` cache
- `identity.lookup` — email or UUID → `Person` (from `PeopleStore` in-process index)
- `identity.scan_text` — raw text + source_id → email regex scan → Anchor+Claim
  pairs → service-fs `/v1/append` × 2 per match (one anchor, one claim)

**31 tests:** 30 unit tests across `acs`, `fs_client`, `mcp`, `people_store`,
`person` modules + 1 integration test (`tests/end_to_end_fs_round_trip.rs`).

Env vars: `PEOPLE_MODULE_ID` (required), `PEOPLE_FS_URL` (required, e.g.
`http://127.0.0.1:9100`), `PEOPLE_BIND_ADDR` (optional, default 127.0.0.1:9300).

systemd unit not yet deployed — next session.

## Build and test

```
cargo test --manifest-path service-people/Cargo.toml
```

Expected: 31 tests pass (30 unit + 1 integration). Run from the monorepo root.
The integration test spins up a real service-fs PosixTileLedger on an ephemeral port
using `tower::ServiceExt::oneshot` — no external services required.

## File layout

```
service-people/
├── Cargo.toml              — axum, tokio, serde, ureq 3.x, uuid, chrono, regex
├── README.md, README.es.md — bilingual overview
├── CLAUDE.md, NEXT.md
├── src/
│   ├── lib.rs              — module re-exports (acs, fs_client, http, mcp, people_store, person)
│   ├── main.rs             — Tokio entrypoint; reads PEOPLE_MODULE_ID, PEOPLE_FS_URL,
│   │                         PEOPLE_BIND_ADDR; spins axum HTTP server
│   ├── acs.rs              — Anchor + Claim structs; scan_text() email regex → UUIDv5
│   ├── fs_client.rs        — FsClient: append_record<T>, append, append_anchor, append_claim
│   ├── http.rs             — AppState { module_id, fs_client, people_store }; axum router
│   ├── mcp.rs              — MCP JSON-RPC 2.0; identity.append + identity.lookup + identity.scan_text
│   ├── people_store.rs     — PeopleStore: RwLock HashMap by email + UUID; conflict detection
│   └── person.rs           — Person struct; UUIDv5 from primary_email; builder pattern
└── tests/
    └── end_to_end_fs_round_trip.rs  — integration: identity.append → service-fs → verify
```

## Hard constraints — do not violate

- **ADR-07: zero AI in Ring 1.** No LLM-assisted entity
  resolution, no embedding-based identity matching, no AI-driven
  schema inference. Identity matching is deterministic
  (canonical-key based).
- **WORM via `service-fs`.** Identity records are persisted
  through `service-fs`'s MCP append surface. This crate does not
  write to disk directly.
- **Per-tenant boundary.** One process per `moduleId`. Cross-tenant
  identity sharing is out of scope for Ring 1; if it ever lands,
  it lives in Ring 2 / Ring 3.
- **Schema stability is doctrinal.** Once the Identity Ledger
  schema is published in a version, breaking changes require a
  MAJOR bump and migration plan — downstream Ring 2 services
  depend on it.

## Dependencies on other projects

- Writes to: `service-fs` (Ring 1, this cluster) — every identity
  record lands in the WORM ledger.
- Read by: `service-extraction` (Ring 2, `project-slm` cluster) —
  resolves contact/organisation references in extracted documents.
- Read by: `service-email` (Ring 1, this cluster) — attaches
  message senders/recipients to canonical identities.

## What not to do

- Do not import AI/ML inference dependencies. ADR-07 applies.
- Do not duplicate Identity Ledger persistence inside this crate
  when the WORM ledger is `service-fs`. One persistence boundary,
  one append-only invariant.
- Do not silently merge conflicting identity records. `PeopleStore`
  returns a conflict error — surface it to the caller (ADR-10 / F12).

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
