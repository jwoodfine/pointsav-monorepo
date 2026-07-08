# NEXT.md — project-workplace (Totebox)

> Hot open items. ≤200 lines. Backlog at `.agent/next-backlog.md` (not yet created).
> **Scope: this archive only.** Cross-repo and workspace-level items live at `~/Foundry/NEXT.md`.

Last updated: 2026-07-07

---

## Blocked — Command Session (route via outbox)

- [ ] **Stage 6 discrepancy re: 2026-07-04 partial-promote message** — Command's message
      cites 4 deferred-item commit hashes that don't exist anywhere in the `pointsav-monorepo`
      sub-clone's history; direct file comparison shows the SessionState item already matches
      canonical, the watcher-refactor item has no trace either side, and the UI item is backwards
      (local already exceeds canonical). Replied via outbox 2026-07-06 asking Command to reconcile
      which state that message was generated against — no rework attempted pending that reply.
      **2026-07-07 update:** likely root cause found — see new duplication finding below.
- [ ] **Stage 6 pending — pointsav-monorepo sub-clone** — 20 commits ahead of origin/main as of
      2026-07-07 (14 pre-existing + 6 new this session: moonshot workspace-table fix, fmt fix,
      smoke tests, app-privategit-workbench workspace-membership regression fix, unwrap fixes,
      anyhow::Result main() refactor). Flagged via outbox
      `project-workplace-20260708-stage-6-pending-20-commits-2-new-structu`.
- [ ] **briefs/state versioning gap** — after the "Option A" gitignore change, BRIEFs + NEXT.md +
      session-context.md durability/versioning story is still unverified (NEXT.md and briefs/ are
      tracked; session-context.md is gitignored/untracked) — confirm this is the intended final
      state or needs a different versioning home.
- [ ] **NEW (2026-07-07) — archive-root vs. pointsav-monorepo sub-clone duplication:** the archive
      root directly contains its own full duplicate of ~150 monorepo directories (every
      app-*/service-*/system-*/moonshot-*/tool-*/vendor-*/os-* dir), tracked by the archive's own
      git, with independent commit history already diverged from the sub-clone's copies of the
      same crate names (confirmed for app-workplace-http-prototype and app-privategit-workbench —
      each has feature commits in one copy absent from the other). Very likely the root cause of
      the Stage-6 discrepancy above. Flagged to Command via the same outbox message; needs a
      dedicated investigation to establish canonical source per directory before any fix — not
      attempted this session.
- [ ] **NEW (2026-07-07) — pointsav-monorepo/CLAUDE.md contamination:** carries `project-design`
      content (and its `.agent/rules/brief-discipline.md` too), likely on the shared `main`
      branch — would affect every archive that clones this monorepo, not just project-workplace.
      Flagged to Command via the same outbox message; cross-archive-governance scope, not a
      unilateral Totebox fix.

## Active (Totebox scope)

- [ ] **app-workplace-aibridge Phase 3** — deeper docengine + crdt cross-crate composition layers
- [ ] **moonshot crates Phase 3** — parser incremental retokenize; crdt undo/redo hardening; bim-engine full STEP grammar

## Completed (2026-07-07 — automode cleanup pass, plan `can-we-make-a-validated-cloud`)

- [x] **BRIEF-workplace-workbench.md "Decisions open" stale line** — sub-clone branch note
      updated to reflect moonshot crates already committed+promoted to `main`; several other
      stale claims in the same BRIEF (DOCTRINE ratification, Stage-6 status) also cleared.
- [x] **moonshot-docengine + moonshot-bim-engine missing `[workspace]` table** — standalone
      `cargo check` was failing via ancestor-workspace walk; fixed to match crdt/editor/parser
      siblings, plus pre-existing `cargo fmt` drift on those three siblings.
- [x] **app-privategit-workbench nested-workspace regression** — caught and fixed same session:
      the moonshot fix above broke this crate (a root workspace member path-depending on both
      moonshot crates); made it standalone `[workspace]` too, matching aibridge/http-prototype's
      already-working pattern. Verified `cargo check --workspace` from repo root clean after.
- [x] **app-workplace-http-prototype code quality** — first-ever smoke tests for the workbench
      router; two panicking `Content-Disposition` unwraps fixed in both workbench crates'
      PDF-export handlers; `main()` converted to `anyhow::Result`, matching
      app-privategit-workbench's existing pattern.
- [x] **Outbox frontmatter dedupe** — 7 message blocks with stray duplicate `status:` lines
      (leftover from prior hand-edits) cleaned up to a single trailing status each.
- [x] **Ratified doctrine draft archived** — `DOCTRINE-AMENDMENT-workbench-as-os-surface.draft.md`
      moved to `drafts-outbound/archived/`.

## Completed (2026-07-06 — governance cleanup session)

- [x] **Archive contamination repaired** — `CLAUDE.md` (editorial-gateway/cartridge content from
      project-console/project-editorial/project-proforma/project-marketing), `.agent/session-start.md`
      (stale branch-ignore note), `.agent/briefs/README.md` (was titled project-marketing) all
      corrected to project-workplace's real mission (Workbench OS surface — app-privategit-workbench
      + moonshot crates). This file (`NEXT.md`) stripped of project-console/project-gis/
      project-editorial/project-intelligence/project-design content.
- [x] **M-17 BRIEF sweep** — `.agent/briefs/` has 1 genuine brief (`BRIEF-workplace-workbench.md`)
      + 4 foreign files (marketing-platform-master, os-orchestration-build-out, os-totebox-build-out,
      os-totebox-ppn-build-out); routed to Command via outbox for redistribution to
      project-marketing/project-data — not this archive's to delete.
- [x] **manifest cluster_branch reconcile — closed as already-resolved** — the earlier concern
      ("cluster/project-workplace is 1046 commits behind main") predates manifest.md's 2026-06-23
      update; the archive root is correctly and actively on `cluster/project-workplace` today
      (HEAD as recent as 2026-06-30) under the current workspace-wide cluster-branch policy.
- [x] **Outbox hygiene** — 5 stale threads marked `actioned` (stage6-ui-commits,
      doctrine-amendment-ivh, stage6-phases-abcdef, workbench-moonshot-stage6,
      stage6-ready-workbench-fmt-clippy-fixed); contamination-and-prototype-unit split to reflect
      only its remaining open scope.
- [x] **Sub-clone git hygiene** — restored an accidentally-deleted, out-of-scope
      `app-orchestration-gis/www/index.html`; committed 6 previously-untracked `Cargo.lock` files
      (app-workplace-aibridge, moonshot-bim-engine/crdt/docengine/editor/parser).
- [x] **`.agent/binary-targets.yaml` created** — declares both undeclared `[[bin]]` targets found
      in this archive, `app-privategit-workbench` and `app-workplace-http-prototype`, both
      `soft_enabled: false` (internal dev tools), per the 2026-07-02 binary-distribution broadcast.
- [x] **local-workplace-http-prototype.service crash loop fixed** — binary rebuilt and redeployed;
      service confirmed active.

## Completed (Sessions 1–11)

- [x] **workbench moonshot crates** — docengine/parser/crdt/editor/bim-engine v0 + app-workplace-aibridge; 53+ tests [2026-06-14 totebox]
- [x] **workbench pre-promote fixes** — cargo fmt + clippy fix (f00e676a + ec305edc) [2026-06-09 totebox]
- [x] **BRIEF audit** — all project-workplace BRIEFs updated with correct frontmatter [2026-06-15 totebox]
- [x] **archive contamination identified** — NEXT.md, session-start, briefs/README foreign content reported via outbox [2026-06-16 totebox]
- [x] **app-workplace-http-prototype** — manual start after service died on reboot [2026-06-16 totebox]
- [x] **DESIGN-TOKEN-CHANGE-wp-tokens** — 27 DTCG tokens committed + routed to project-design [2026-06-09 totebox]
- [x] **Stage 6 — workbench UI + moonshot/aibridge crates** — promoted to canonical (confirmed
      2026-07-06: content-identical commits are ancestors of `origin/main`) [2026-06-09..06-20 totebox/command]
