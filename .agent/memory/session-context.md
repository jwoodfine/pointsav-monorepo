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

- **Command Session** — Stage 6 canonical merge pending (app-orchestration-command v0.0.1+v0.0.2 commits on cluster branch); install local-orchestration-command.service via bootstrap.sh; project-registry.md update on monorepo main
- **project-software** — BETA listing for soft-orchestration-command; confirm download URL; disable payment gate
- **project-totebox** — Totebox-side /v1/pair endpoint design ACK; peer_type field decision
- **project-system** — J2/J5 HOLD until Bench #9 complete

## Session entries

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
