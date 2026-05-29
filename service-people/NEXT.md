# NEXT.md — service-people

> Last updated: 2026-05-29 (session 7)
> Read at session start. Update before session end so the next
> session knows where to pick up.

---

## Right now

- **Deploy as systemd unit** — unit file staged at
  `infrastructure/local-people/local-people.service` (session 7).
  Outbox message to Command Session has full step-by-step instructions
  (build binary, create system user, binary-ledger entry, install).
  Port 9300 confirmed free 2026-05-29.

## Queue

- **Deterministic entity-resolution rules** — canonical-key matching only
  (ADR-07; no AI). Surfaces ambiguity to the operator (per ADR-10 / F12),
  does not silently merge. Design: conflict detection already exists in
  `PeopleStore`; extend to structured error payloads in MCP response.

## Deferred

- Cross-tenant identity sharing — Deferred: out of scope for Ring
  1 by `~/Foundry/conventions/three-ring-architecture.md`. If it
  ever lands, it lives in Ring 2 / Ring 3.
- Embedding-based fuzzy identity matching — Deferred (and
  doctrinally constrained): ADR-07 keeps Ring 1 zero-AI.
  Fuzzy matching, if needed, runs in Ring 2 with a deterministic
  read-only contract.

## Recently done

- 2026-05-27 (session 4): **ACS engine absorbed as `identity.scan_text` MCP tool.**
  New `src/acs.rs` — email regex (ADR-07 deterministic) → UUIDv5(NAMESPACE_DNS, email)
  → Anchor+Claim pairs; 6 unit tests. `src/fs_client.rs` extended with generic
  `append_record<T>` helper + typed `append_anchor` + `append_claim`; 4 new tests.
  `src/mcp.rs` — third tool `identity.scan_text` (text + source_id → anchors +
  claims written to WORM ledger); 2 new tests. **31 tests pass** (30 unit +
  1 integration). Legacy code retired: `people-acs-engine/` (logic absorbed),
  `spatial-ledger/`, `spatial-crm/`, `service-people.py`, `ledger_personnel.json`.

- 2026-05-20 (session 2): **End-to-end integration test with service-fs.**
  `tests/end_to_end_fs_round_trip.rs` — spins up real service-fs PosixTileLedger on
  ephemeral port; POST `identity.append` via tower::oneshot → assert Person record
  reaches WORM ledger and can be read back. Closes Ring 1 pipeline from identity
  input to persisted WORM.

- 2026-04-27: **MCP server interface** (`src/mcp.rs`, `src/http.rs`,
  `src/main.rs`, `src/fs_client.rs`, `src/people_store.rs`). `POST /mcp`
  JSON-RPC 2.0 endpoint with `identity.append` + `identity.lookup` tools.
  `PeopleStore`: in-process RwLock HashMap index. `FsClient`: ureq 3.x blocking
  POST with `X-Foundry-Module-ID` header. Env vars: `PEOPLE_MODULE_ID` (required),
  `PEOPLE_FS_URL` (required), `PEOPLE_BIND_ADDR` (default 127.0.0.1:9300).
  **20 tests** pass.

- 2026-04-27: **canonical person-record schema** (`src/person.rs`).
  `Person` struct — UUIDv5 from primary_email, builder pattern. **8 unit tests pass.**

- 2026-04-26: **pre-framework subdirectory inventory complete.** All five
  subdirectories + two root artefacts assessed; per-item decisions made. Legacy
  retirement deferred to session 4 (above).
