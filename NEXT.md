# NEXT.md — project-workplace (Totebox)

> Hot open items. ≤200 lines. Backlog at `.agent/next-backlog.md` (not yet created).
> **Scope: this archive only.** Cross-repo and workspace-level items live at `~/Foundry/NEXT.md`.

Last updated: 2026-07-06

---

## Blocked — Command Session (route via outbox)

- [ ] **Stage 6 discrepancy re: 2026-07-04 partial-promote message** — Command's message
      cites 4 deferred-item commit hashes that don't exist anywhere in the `pointsav-monorepo`
      sub-clone's history; direct file comparison shows the SessionState item already matches
      canonical, the watcher-refactor item has no trace either side, and the UI item is backwards
      (local already exceeds canonical). Replied via outbox 2026-07-06 asking Command to reconcile
      which state that message was generated against — no rework attempted pending that reply.
- [ ] **Stage 6 pending — pointsav-monorepo sub-clone** — 14 commits ahead of origin/main as of
      2026-07-06 (13 pre-existing + this session's `d9c203af` Cargo.lock fix). Flagged via outbox
      alongside the discrepancy above.
- [ ] **briefs/state versioning gap** — after the "Option A" gitignore change, BRIEFs + NEXT.md +
      session-context.md durability/versioning story is still unverified (NEXT.md and briefs/ are
      tracked; session-context.md is gitignored/untracked) — confirm this is the intended final
      state or needs a different versioning home.

## Active (Totebox scope)

- [ ] **app-workplace-aibridge Phase 3** — deeper docengine + crdt cross-crate composition layers
- [ ] **moonshot crates Phase 3** — parser incremental retokenize; crdt undo/redo hardening; bim-engine full STEP grammar
- [ ] **BRIEF-workplace-workbench.md "Decisions open"** — line noting sub-clone branch as
      "operator decision pending" is stale (moonshot crates already committed+promoted to `main`);
      update to reflect that the sub-clone intentionally stays on `main` while the archive root
      uses `cluster/project-workplace`, unless the branch model is being revisited.

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
