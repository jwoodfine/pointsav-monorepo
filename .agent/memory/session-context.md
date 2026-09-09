# Session Context — project-knowledge cluster

Rolling 3-session summary. Newest on top. Keep only 3 entries; push oldest to `session-context-archive.md`.

## 2026-09-08 (session 31) | Totebox | claude-code (Sonnet 5)

**Done this session (continued from a prior compacted session; mailbox/governance triage +
resolved a real branch-history incident):**
- Continued a mid-flight task from before compaction: Command's cherry-pick governance-fix
  request for `woodfine-fleet-deployment` (a byte-identical duplicate doc,
  `guide-moonshot-toolkit-phase1c-build-setup.md`, sitting in two paths). Primary copy already
  gone via a clean fast-forward from real `origin/main`; verified the sibling duplicate
  byte-identical (md5sum) before deleting, committed `07ee551`, pushed to both staging mirrors.
  Command confirmed merge to canonical the same session.
- **Real incident: orchestration-layer IP (license/metering/allocation source) found live on
  both public `pointsav-monorepo` forks on `cluster/project-knowledge`**, inherited history
  missed by the 2026-09-01 sweep. Command force-pushed a purge to both forks (new tip
  `2a2f2c2a2f`). Reset both local repos (outer clone + `pointsav-monorepo/` sub-clone) to match
  — operator-confirmed via the `FOUNDRY_CONFIRM_DESTRUCTIVE=1` gate (I cannot self-authorize
  this; had the operator run the marker command in their own shell each time), local backup
  branches made first in both repos.
- **Found and fixed a real gap while verifying the reset was safe**: `cluster/project-knowledge`
  itself hadn't been pushed to the staging forks since 2026-09-02 — merge-base against the
  backup branch came back 12,214 commits, dated to February. All code content was independently
  safe (already on canonical, or on its own untouched `canonical-reconcile-*` branch), but this
  session's `.agent/`/BRIEF commits existed only locally. Restored the 7 differing `.agent/`
  files from the backup branch as one commit (`b2c0b702b`) rather than replaying history at
  that scale, fast-forward-pushed clean. Full writeup: `BRIEF-project-knowledge-stage6-
  reconciliation-2026-08-24.md`'s new 2026-09-08 Resolution section (fully closes that BRIEF).
  New memory: `feedback-branch-divergence-and-purge-2026-09-08.md`.
- Verified the earlier BRIEF-freshness plan (`indexed-foraging-sphinx.md`, footer-redesign
  STATUS UPDATE + binary-distribution/os-mediakit-product-family supersession note + README
  table fixes) was already fully executed before the compaction — confirmed present in the
  restored `.agent/` state, nothing left to do there.
- **Mailbox triage, 2 rounds:** first round — Command chased/confirmed the reset; second round
  — project-editorial's bundled ask (full rebuild+redeploy of all 3 wikis given real stale-
  deploy evidence, a footer-validation CI-check ownership ask, apex-footer status question, O3
  status-only) plus Command's re-send of 2 UI items previously only staged-not-sent in a BRIEF
  (uncapped sidebar panel — kept deferred per prior operator direction, offered a `sibling_scope`
  frontmatter alternative to a raw threshold; prose column width — already fixed this session
  via an independent path, confirmed live on canonical, just needed to tell Command). Accepted
  the CI-check ownership (operator-confirmed) — new `BRIEF-footer-ci-check.md`, not yet scoped.
  Relayed the rebuild request to Command (operator-confirmed) rather than acting on it directly
  — deployment/VM-sysadmin is Command-Session-only territory.

**Pending / carry-forward:**
- Spanish-disclosure fix (`canonical-reconcile-2026-09-07b`, `4c5e1966c8`) still not merged to
  canonical as of session end — reconfirmed directly against `origin/main`, still missing.
- Phase 2 of the "path to live" plan (rebuild + re-verify the local `os-mediakit` appliance
  against current code) — Phase 1 is done, Phase 2 not yet started, no explicit go-ahead this
  session.
- `BRIEF-footer-ci-check.md` — accepted, not yet scoped; queued behind Phase 2.
- Asked Command whether `.agent/` durability pushes (this branch → staging forks) should become
  a routine cadence rather than only during Stage 6 promote runs — not yet answered.
- project-editorial's full-rebuild request — relayed, Command owns scheduling from here.

**Operator preference confirmed this session:** when a destructive git operation (reset --hard)
is required, do not attempt any self-authorization workaround (e.g. setting the confirm env var
in the same Bash call) even when instructed to proceed — state scope/rollback/ratification
clearly, then have the operator run the marker command themselves in their own shell. This was
the gate's designed behavior, not a failure to work around.

---

---

## 2026-07-09 (session 30) | Totebox | claude-code (Sonnet 5)

**Done this session (operator-directed cleanup sweep + BRIEF consolidation, via approved plan mode):**
- Startup clean: role/branch confirmed (`cluster/project-knowledge`), session lock written,
  `get_session_brief` MCP call worked this time (5 pending inbox messages, NOTAM inactive).
- **Mailbox hygiene:** archived ~23 already-`status:actioned` inbox messages that were never
  swept (some dating to 2026-06-29); fixed malformed duplicate-`status:` frontmatter along the
  way; removed a stray orphaned empty-mailbox header block that had been sitting mid-file.
  Reclaimed 2 misrouted DESIGN drafts (turned out we already held identical copies — routed to
  project-design). Created `.agent/binary-targets.yaml` — a real compliance gap: a 2026-07-02
  broadcast asked every archive to declare this and ours never existed despite the inbox
  message being marked actioned.
- **Contamination found, deliberately NOT touched (operator call):** ~58 of 67 files in our
  own `.agent/drafts-outbound/` belong to other archives (project-orgcharts, project-proforma,
  project-infrastructure, project-bim, project-data, project-workplace) — including real
  Bencal business-admin documents from project-orgcharts. Per the new AGENT.md business-admin
  rule, flagged to Command only, nothing moved/deleted. Same treatment for 15+ foreign-owned
  BRIEFs + a stray `.workflow.js` sitting in our `.agent/briefs/` (moved the workflow.js
  ourselves — unambiguously ours; left the foreign BRIEFs for Command).
- **BRIEF consolidation:** archived 3 fully-shipped old-engine BRIEFs (slides,
  visual-excellence, inline-annotations). Reparented `BRIEF-sovereign-editorial-marketing`
  and `-software` under a new slim `BRIEF-sovereign-editorial.md` tracker rather than literally
  merging them — they turned out to be genuinely distinct 250-line specs for two different
  destination archives (project-marketing vs project-software), not near-duplicates as an
  earlier pass's summary suggested. Fixed README.md drift, repointed the stale
  `manifest.md` wiki/vendor leg (content-wiki-documentation → project-editorial's
  media-knowledge-documentation, per the 2026-06-09 rename PROJECT-CLONES.md already recorded).
- **Major discovery — the ng-rewrite (P0-P8) is already complete and live; our own BRIEF and
  local sub-clone were just stale.** Set out (per operator-approved plan) to reconcile the
  local `pointsav-monorepo` sub-clone with canonical before building a release binary. Found:
  canonical commit `531d3144` (2026-07-02) really did land the P8 cutover; the binary running
  behind all 3 wiki systemd services (sha `1ad9946f...`) matches canonical's post-cutover
  source exactly; Command had already asked project-software (2026-07-08) to catalog-list this
  exact binary, still pending on their side. `BRIEF-knowledge-ng-rewrite.md` had simply never
  been updated after P7/P8 both completed the same day as its "NEXT UP = P7" note — the
  2026-07-03 "should we revert to v1?" carry-forward question (visible in session 27/28/29
  entries below) was moot from the moment it was written; sessions 27-29 all noticed pieces of
  this (25e4bf99 existing on the sub-clone's `main`, the sha mismatch) and flagged without
  fully tracing it. **Also separately confirmed:** this archive's local `pointsav-monorepo`
  sub-clone's own `app-mediakit-knowledge/` git history (48 commits, "iteration-2"/"Wave 1-5"
  naming) is a wholly different, older lineage that never merged with the real P0-P8 rewrite at
  all. Corrected both BRIEFs + NEXT.md with 2026-07-09 status updates; sent Command a full
  writeup so nobody re-does this work, and flagged the documentation-continuity gap (a
  completed major milestone going untracked across 3+ sessions) as worth naming.
- Also fixed archive-identity contamination in `.agent/memory/MEMORY.md` (title said
  "project-proforma"; one feedback line didn't match this archive's own history at all —
  removed as foreign, not adapted). Same recurring pattern as session 29's CLAUDE.md/NEXT.md
  fix and the earlier `content-wiki-documentation` manifest drift.

**Pending / carry-forward:**
- P5b JOURNAL render — still the only genuinely-open ng-rewrite item; editorial reply pending.
- Cross-archive cleanup flagged to Command (2026-07-09,
  `command-20260709-cross-archive-cleanup-needed-stray-dupli`) — drafts-outbound contamination
  (business-admin content mixed in) + foreign BRIEFs — not this archive's to resolve further.
- **New inbox item arrived mid-session (2026-07-09T22:31Z):** Command/project-design accepted
  ownership of a canonical compliance-band component + footer-legal block + tokens (replacing
  the currently-duplicated-with-drift pattern across wikis/software.pointsav.com/bim), asking
  us to stage a DESIGN-COMPONENT draft (component recipe + research file, accessibility targets,
  Carbon-baseline note) whenever convenient. Not yet actioned — left `status: pending`.
- **New, not investigated:** the `pointsav-monorepo` sub-clone has its own tracked `.agent/`
  tree (98 files, briefs/etc.) separate from this archive's real `.agent/` — structural oddity,
  unclear if intentional per-sub-clone bookkeeping or another contamination symptom. Also has
  9 untracked draft guides (marketing/location-intelligence/ppn/vm-mediakit topics, none
  ours) plus untracked `os-mediakit/scripts/build/` and `vendor-tantivy/` (just a Cargo.lock) —
  none committed this session, provenance unclear, left alone rather than guessed at.

**Operator preference confirmed this session:** when asked to "grill" before finalizing a plan,
group blocking/foundational questions first (engine direction, scope depth, cross-archive
write approach) before secondary ones (manifest fixes, consolidation specifics) — answers to
the first round changed what the second round needed to ask. Also confirmed: when a plan's own
mid-execution checkpoint (Phase C's "present findings before proceeding") surfaces something
that invalidates the plan's remaining phases entirely, stop and re-ask rather than trying to
salvage the original phase structure — the operator's answer ("verify + reconcile our own state
only") was a full pivot, not a tweak.

---

## 2026-07-06 (session 29) | Totebox | claude-code (Sonnet 5)

**Done this session (startup hygiene — found + fixed archive-identity contamination, no feature work):**
- Startup: role/branch confirmed (`cluster/project-knowledge`), session lock written. MCP `foundry`
  server tools did not surface via ToolSearch this session despite being configured in `.mcp.json`
  — fell back to manual reads for the whole startup sequence (inbox/outbox/briefs/session-context/
  rules all read directly).
- `~/Foundry/NOTAM.md` is unreadable (`0600` owned by `mathew`; this session runs as `jennifer`,
  group `foundry` has no read bit) — flagged, not fixed (workspace-root file, outside Totebox scope).
- **Found this clone's root `CLAUDE.md`/`NEXT.md` contained project-gis's content** (GIS/AEC-pipeline
  material) and `.mcp.json`'s `SLM_MODULE_ID` was `gis` instead of `knowledge`. Confirmed via git log
  this is a long-recurring workspace-wide pattern (10+ "restore archive identity" commits across
  other archives: project-system, project-design, project-workplace, project-intelligence,
  project-editorial, project-console, project-marketing). Root cause per commit `1dc6bc693`'s message:
  a "canonical reset" process, not yet fixed at the tooling level.
  Confirmed with operator before acting (asked: fix now / flag only / show diff first — operator chose
  fix now). Rewrote `CLAUDE.md` (archive-name references only, template/structure unchanged — same
  `self_service: build-deploy-stage6lite` tier as project-gis per `pairings.yaml`, so the body was
  reusable), rebuilt `NEXT.md` from `.agent/memory/session-context.md` + `.agent/briefs/README.md` +
  the 3 pending inbox items (none of which were affected by the contamination), fixed `.mcp.json`.
  Committed `7e9475670`.
- **New drift found, not yet acted on:** `content-wiki-documentation/` sub-clone (declared in
  `.agent/manifest.md` as this cluster's PRIMARY vendor leg) is missing entirely from disk.
  `pointsav-fleet-deployment/` is owned by `root`; `woodfine-fleet-deployment/` (not in the
  manifest's clone list at all) is owned by `mathew` — both trigger git's dubious-ownership guard
  under the `jennifer` user, so status/log can't be checked without a `git config --global
  --add safe.directory` exception, which was not applied without asking first. Recorded in personal
  memory (`archive-identity-contamination.md`, `project-knowledge-subclone-gaps.md`) since these are
  durable, non-obvious facts about this specific working directory.
- `pointsav-monorepo` sub-clone checked: clean working tree, still on `main` (not
  `cluster/project-knowledge`), 140 behind / 1 ahead of `origin/main` — unchanged from the
  2026-07-05 Command report; no new work done on it this session.

**Pending / carry-forward:**
- ng-rewrite P8 cutover reconciliation against canonical still needed before Stage 6 promote can
  retry (Command outbox `command-20260705-stage-6-promote-blocked-real-conflicts-s`) — this
  archive's action, not yet started.
- v2→v1 switch-back decision and trademark reconciliation — still Command's, unchanged.
- `content-wiki-documentation/` missing + dubious-ownership fleet-deployment dirs — new, needs a
  future session's attention (or Command, if it's a provisioning-level fix).
- Low-priority: flip `state:` field on the already-archived `research-wikipedia-toolbar-mobile.draft.md`.

**Operator preference confirmed this session:** when a significant, ambiguous-scope fix is found
during startup (not what the operator asked for), stop and present findings + options before acting,
even when the fix is well-precedented — a one-line "fix now" was all that was needed once framed
clearly with the finding, evidence, and proposed action.

