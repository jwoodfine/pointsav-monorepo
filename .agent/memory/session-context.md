---
schema: foundry-session-context-v1
archive: project-orchestration
---

# Session context — project-orchestration

## Operator preference digest

- Prefers concise status summaries at end of significant work chunks (not after every tool call)
- Deployment instances named `gateway-orchestration-<name>-<n>` (not short forms like `orchestration-command-1`)
- BETA mode for marketplace: no payment gate, free download, project-software handles the pricing flip when BETA ends
- Binary distribution model: curl-download from software.pointsav.com via bootstrap.sh; no `cargo build` on target

## Cross-archive carry-forward

- **Command Session** — v0.0.1 confirmed in canonical origin/main (commit 29d0b4a1, done).
  v0.0.2 pushed to staging mirrors + queued in promote-queue.jsonl 2026-07-09 (commit
  95f4ca2f) — awaiting canonical merge. Once merged, requested `bin/build-soft.sh` run
  (never actually run for this binary — data/app-repository/registry.yaml is empty) +
  registry-update routed to project-software to supersede the informal BETA handoff.
  project-registry.md update on monorepo main still pending.
- **project-totebox** — Totebox-side /v1/pair endpoint design ACK; peer_type field
  decision. Re-pinged 2026-07-09 (11 days silent as of that date) — no response yet.
- **project-system** — J2/J5 HOLD until Bench #9 complete

## Session entries

### 2026-07-09 — Backlog sweep: BRIEF consolidation + v0.0.2 Stage 6 push (totebox@claude-code)

**Done this session:**
- Interrogated operator on 4 decision points (BRIEF merge strategy, stale-brief handling,
  Stage 6 push timing, formal-vs-informal SOFT- pipeline, /v1/pair re-ping, ship scope) —
  all recommended options accepted.
- BRIEFs re-scoped to eliminate overlap: `BRIEF-os-orchestration.md` = concrete shipping
  state; `BRIEF-os-orchestration-build-out.md` = long-range architecture/roadmap. Each
  cross-references the other. `BRIEF-brief-audit-2026-06.md` archived (stale one-time
  audit log). `.agent/briefs/README.md` tables rewritten to match.
- Found and corrected drift: NEXT.md P3.4 claimed install/smoke-test/BETA-URL were
  "pending" — ledger (`data/binary-ledger/orchestration-command-server.jsonl`) showed
  they were already done. Checkboxes corrected with evidence citations.
- Found a real gap NEXT.md was hiding: v0.0.2 (pairing.rs WORM schema_version fix) was
  never promoted past the cluster branch — only v0.0.1 made it to canonical origin/main.
  Re-verified tests green (7/7), got explicit operator go-ahead, ran
  `self-service-promote.sh` — v0.0.2 now on staging mirrors + in promote-queue.jsonl
  (commit 95f4ca2f).
- Found the "BETA listing on software.pointsav.com" is informal — `bin/build-soft.sh`
  has never run for this binary; `data/app-repository/registry.yaml` is `packages: {}`.
  Sent Command a consolidated request: canonical-merge v0.0.2 → run build-soft.sh →
  route registry-update to project-software.
- Re-pinged project-totebox on the 11-day-silent `/v1/pair` design ACK.
- Marked the binary-distribution-tracking broadcast (msg-id
  command-20260702-binary-distribution-tracking...) actioned — binary-targets.yaml
  already compliant, no change needed.
- Committed all doc/BRIEF/NEXT.md changes to cluster/project-orchestration (39c50bf3).

**Pending/carry-forward:** see Cross-archive carry-forward above (all Command/project-totebox).

**Operator preferences surfaced:**
- Wants a full "GRILL ME one by one with recommendations" interrogation pass before
  large backlog/consolidation work, not just a single confirm-to-proceed — confirmed
  this style landed well (all recommended options accepted without pushback).

### 2026-06-29/30 — app-orchestration-command v0.0.1/v0.0.2 + BRIEF buildout (totebox@claude-code)

**Done this session:**
- Implemented `app-orchestration-command` v0.0.1: 3-crate Rust workspace
  (orchestration-command-core, orchestration-command, orchestration-command-server).
  Modules: fleet, personnel, invite, pairing, routing, child, license. 7 tests pass.
  Binary 1.7 MB stripped. Port 8020 loopback. current_thread Tokio.
- app-orchestration-graph stub (port 8021, v0.0.1-stub) added as companion crate.
- v0.0.2 patch: pairing.rs — schema_version in WORM ledger, write-through to
  user-pairings.yaml, renamed sha256_hex → key_fingerprint (FNV-1a, not SHA-256).
- Deployment provisioned: `gateway-orchestration-command-1/` (MANIFEST, README, README.es, .owner)
- Infrastructure draft committed to cluster: systemd unit + bootstrap.sh (curl-download
  from software.pointsav.com with BINARY_URL/BINARY_SRC fallback). MemoryMax=128M.
- Cross-archive coordination: outbox to project-totebox (peer-agnostic /v1/pair design),
  project-software (BETA listing, no payment gate), Command (Stage 6 + systemd install).
- Actioned all inbox messages: J5 + J2 HOLD confirmed; peer-agnostic protocol amendment
  absorbed into BRIEF; infrastructure schema ACK locked two decisions.
- BRIEF-os-orchestration.md fully built out: architecture rationale section (4 WHY items),
  v0.1.0 planned scope section, decisions open expanded with 4 v0.1.0 questions.
- topic-os-orchestration.draft.md language pass: CommandCentre + Pairing sections
  corrected from "is intended to" → present tense; BETA install note added.
- NEXT.md updated: P3.4 partial (deployment done; install pending Command), P3.5 closed.

**Pending/carry-forward (all blocked on external):**
- Stage 6 → Command; systemd install → Command; BETA URL → project-software;
  peer_type → project-totebox; J2/J5 → project-system Bench #9

**Operator preferences surfaced:**
- BETA distribution model (no payment gate; curl-download bootstrap; pricing flip by project-software)
- Deployment naming convention: gateway-orchestration-<name>-<N>

### 2026-06-09 — MCP v0.3.0 readiness update (Command@claude-code)

CLAUDE.md updated with MCP v0.3.0 tools table + artifact-type bright-line rules.
session-context.md stub provisioned (this file).
