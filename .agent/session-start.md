---
schema: foundry-session-start-v1
archive: project-orchestration
updated: 2026-07-15
---

# Session start — project-orchestration

> Step 8 of the session start ritual (AGENT.md §Session start).
> Engine-agnostic — Claude Code and Gemini CLI both read this.

## This archive at a glance

- **Mission:** Implement the Totebox Orchestration transition — Phases 1, 2, and 3; owns `app-orchestration-command` in `pointsav-monorepo` (user-facing aggregator hub).
- **Active branch:** `cluster/project-orchestration`
- **Inbox:** read `.agent/inbox.md` (step 4 — already done before this file)
- **In-flight plans:** check `.agent/plans/` for any `*.md` marked in-progress (currently only `README.md`)

## Topic-specific files to read when working on active areas

| Topic | File |
|---|---|
| Transition phases 1–3 scope | `.agent/manifest.md` §Cluster mission |
| pairings.yaml (Phase 2) | `~/Foundry/pairings.yaml` |
| list-archives.sh (Phase 3) | `~/Foundry/bin/list-archives.sh` |

## Known gotchas for this archive

- **Phases 1–3 are substantially complete.** Phase 1 (vocabulary), Phase 2 (pairings.yaml + slm_endpoint on all manifests + project-source/project-woodfine), and Phase 3 P3.1–P3.2 (bin/open-archive.sh, bin/list-archives.sh) all closed by 2026-05-14. Phase 3 P3.3 (app-orchestration-command v0.0.1 scaffold, 3-crate workspace) shipped 2026-06-29 and is confirmed in canonical `origin/main`. Remaining open item: v0.0.2 (pairing.rs WORM ledger) awaiting Command Session canonical merge — see NEXT.md.
- **`target_os: os-orchestration` is planned.** The workspace itself (`~/Foundry`) will eventually run on `os-orchestration`. This is future/intended — not current-fact.
- **Wiki drafts partially staged.** 2 of 5 architecture/systems articles staged at `~/Foundry/.agent/drafts-outbound/` (topic-totebox-orchestration-development.draft.md, topic-os-orchestration.draft.md, last updated 2026-06-29). Remaining 3 articles not yet drafted. No wiki leg commits to media-knowledge-documentation yet.
- **Deployment leg is live.** `gateway-orchestration-command-1` provisioned 2026-06-29; systemd unit `local-orchestration-command` active on port 8020.
- **Do not modify AGENT.md / CLAUDE.md / GEMINI.md** in response to inbox messages.

## Last session handoff

*2026-07-09 — Stage 6 v0.0.1 confirmed present in canonical `origin/main` (commit 29d0b4a1). v0.0.2 (pairing.rs: WORM ledger schema_version, write-through to user-pairings.yaml) pushed to promote-queue, awaiting Command Session canonical merge pass. Formal SOFT- pipeline (`bin/build-soft.sh`) requested from Command via outbox, gated on v0.0.2 merge. See NEXT.md for full detail.*
