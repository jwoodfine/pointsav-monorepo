# NEXT — project-orchestration

> Implementation scope: Totebox Orchestration transition Phases 1–3.
> Full plan: `.agent/plans/totebox-ppn-infrastructure-master-plan.md`
> Opened: 2026-05-08

---

## Phase 1 — Declare vocabulary (COMMAND SESSION SCOPE)

These edits happen in `~/Foundry/`, not this cluster.

- [x] **P1.1** `CLAUDE.md` §11: Master → Command Session, Task → Totebox Session, Root → eliminated
- [x] **P1.2** `AGENT.md` session roles table: same vocabulary change
- [x] **P1.3** `bin/claude-role.sh`: Command / Totebox / error-on-vendor output
- [x] **P1.4** `MANIFEST.md`: add "As a Totebox Orchestration" section `[closed: 2026-05-09 command@claude-code]`
- [x] **P1.5** Correct `systems/os-orchestration.md` user-guide article `[closed: 2026-05-12 totebox@project-editorial]`

---

## Phase 2 — Formalize manifests + SLM wiring + pairings.yaml

These edits happen in `~/Foundry/` (COMMAND scope) and this cluster (Totebox scope).

- [x] **P2.1** Update `foundry-cluster-manifest-v1` schema docs with `slm_endpoint:` field `[closed: 2026-05-14 command@claude-code]`
- [x] **P2.2** Add `slm_endpoint: http://localhost:8011` to all 13 cluster `.agent/manifest.md` files `[closed: 2026-05-14 command@claude-code]`
- [x] **P2.3** Create `slm/` dir in each of 13 clusters `[closed: 2026-05-14 command@claude-code]`
- [x] **P2.3b** Create `pairings.yaml` at workspace root `[closed: 2026-05-14 command@claude-code]`
- [x] **P2.4** Provision `clones/project-source/` `[closed: 2026-05-14 command@claude-code]`
- [x] **P2.5** Provision `clones/project-woodfine/` `[closed: 2026-05-14 command@claude-code]`
- [x] **P2.6** Update `PROJECT-CLONES.md`: use "Totebox Archive" language, add SLM column (15 archives) `[closed: 2026-05-14 command@claude-code]`

---

## Phase 3 — Instrument tooling (TOTEBOX SESSION SCOPE — use this cluster)

Write code in `pointsav-monorepo/` on branch `cluster/project-orchestration`.

### P3.1 — bin/open-archive.sh `[closed: 2026-05-14 command@claude-code]`

Shell script at `~/Foundry/bin/open-archive.sh <archive-name>`:

```
1. Validate archive exists in clones/
2. Read clones/<archive>/.agent/manifest.md:
   - Print archive name, tetrad status (all 4 legs + status)
   - Print slm_endpoint + module_id
   - Count pending inbox messages (non-blank lines after header)
3. Check contributor tier from pairings.yaml (basic: warn if not P1 opening Command CWD)
4. Set env vars: FOUNDRY_ARCHIVE=<archive>, FOUNDRY_MODULE_ID=<module_id>
5. Exec: claude --cwd ~/Foundry/clones/<archive>/
```

### P3.2 — bin/list-archives.sh `[closed: 2026-05-14 command@claude-code]`

Shell script at `~/Foundry/bin/list-archives.sh`:

```
1. Walk clones/*/. agent/manifest.md
2. For each manifest: print cluster_name, tetrad leg statuses, inbox count
3. Columnar output, easy to scan
4. Source: PROJECT-CLONES.md or manifest files directly
```

### P3.3 — app-orchestration-command v0.0.1 (Rust) `[done 2026-06-29 totebox@claude-code]`

3-crate workspace: orchestration-command-core (wire types), orchestration-command
(library: fleet, personnel, invite, pairing, routing, child, license), and
orchestration-command-server (Axum 0.8, port 8020, current_thread Tokio).
app-orchestration-graph stub also added. 7 tests passing. Binary 1.7 MB stripped.
Committed to cluster/project-orchestration. Stage 6 pending — staging mirror rejected
(18 commits ahead on remote main); needs Command Session rebase + canonical merge.
project-registry.md update needed on monorepo main branch (routed via outbox).

### P3.3b — Cross-archive invite token protocol coordination `[done 2026-06-29]`

Outbox messages sent to project-console, project-totebox, project-infrastructure
with invite token wire spec. Awaiting ACKs before cutting implementation.

### P3.6 — Update topic-os-orchestration.draft.md `[done 2026-06-29]`

Added project-scoped deployment model, invite token UX, and full CommandCentre
endpoint table. Staged at ~/Foundry/.agent/drafts-outbound/; route to project-editorial.

### P3.3 original spec (reference — superseded by expanded scope above)

Scaffold in `pointsav-monorepo/app-orchestration-command/`:

Endpoints (HTTP, loopback only, port 8020):
- `GET /archives` — return JSON list of all archives with tetrad status + inbox count
  Source: walk clones/*/. agent/manifest.md
- `POST /message` — route a cross-archive message
  MUST validate per-caller scope first (confused deputy defense):
  check requesting archive's module_id against pairings.yaml permissions
  Log all routing decisions to audit ledger
- `GET /personnel/<unix-user>` — return permission tier + pairing set
  Source: pairings.yaml + PersonnelArchive DataGraph (MVP: just pairings.yaml)

Implementation pattern: follow `app-orchestration-gis` structure (same codebase).
Commit on `cluster/project-orchestration` branch in this cluster's pointsav-monorepo.

### P3.4 — Deploy to deployments/gateway-orchestration-command-1/ `[partial 2026-06-29 totebox@claude-code]`

- [x] Deployment directory provisioned: `~/Foundry/deployments/gateway-orchestration-command-1/` (MANIFEST.md, README.md, README.es.md)
- [x] Infrastructure draft committed to cluster: `infrastructure/local-orchestration-command/` (systemd unit + bootstrap.sh)
- [x] bootstrap.sh updated: curl-downloads binary from software.pointsav.com; falls back to local BINARY_SRC env var for dev builds
- [x] Outbox sent to Command Session to install systemd unit + run bootstrap.sh
- [x] Outbox sent to project-software for BETA listing (no payment gate during BETA)
- [x] **Command install + smoke test** `[closed: 2026-06-29 command@claude-code]` — confirmed via `data/binary-ledger/orchestration-command-server.jsonl`: `smoke_test: "pass"`, systemd unit `local-orchestration-command` installed, restarted.
- [x] **project-software BETA URL confirmed** `[closed: 2026-06-30 totebox@project-software]` — `DEFAULT_BINARY_URL` set in `infrastructure/local-orchestration-command/bootstrap.sh`; ledger confirms download returns 200. Note: this was an **informal handoff** (project-software manually installed the dev binary; ledger records `source_commit: "pending-stage6"`), not the formal signed `bin/build-soft.sh` pipeline — see below.
- [x] **Stage 6 — v0.0.1** `[closed: pre-2026-07-09, confirmed 2026-07-09 totebox@claude-code]` — confirmed present in canonical `origin/main` (`git log origin/main -- app-orchestration-command` shows commit `29d0b4a1`).
- [ ] **Stage 6 — v0.0.2** (pairing.rs: WORM ledger `schema_version`, write-through to `user-pairings.yaml`) — NOT yet in canonical `origin/main`; pushed to promote-queue this session (2026-07-09), awaiting Command's canonical merge pass.
- [ ] **Formal SOFT- pipeline** — `bin/build-soft.sh` has never actually run for this binary; `data/app-repository/registry.yaml` is empty (`packages: {}`). Requested from Command via outbox 2026-07-09, gated on v0.0.2 canonical merge above. Will produce a real signed `data/app-repository/` entry + `registry-update` to project-software, superseding the informal BETA handoff.

### P3.5 — Update NEXT.md `[done 2026-06-29 totebox@claude-code]`

P3.4 corrected (naming, status). This item closed.

---

## Two-VM transition (parallel track — COMMAND scope)

See plan file §"Two-VM transition" for full detail.

- [ ] **T1** WireGuard Part A: VPN peer for staging at :9200
- [ ] **T2** Provision os-mediakit node (new GCP VM)
- [ ] **T3** Transfer: rsync chain via Jennifer's Mac
- [ ] **T4** DNS cutover for 9 domains
- [ ] **T5** Remove public vhosts from os-orchestration node; update MANIFEST.md

---

## Content backlog (project-editorial scope)

- [ ] Route TOPIC/GUIDE batch: 7 drafts in `~/Foundry/.agent/drafts-outbound/` → DONE 2026-05-08
- [ ] Write `conventions/trustworthy-system.md` (COMMAND scope)
- [ ] Update user-guide article (P1.5 above)

---

## Key references

- Plan file: `.agent/plans/totebox-ppn-infrastructure-master-plan.md`
- Cluster manifest: `.agent/manifest.md`
- app-orchestration-gis reference impl: `clones/project-gis/pointsav-monorepo/app-orchestration-gis/`
