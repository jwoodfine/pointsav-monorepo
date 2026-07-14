# NEXT.md — project-workplace (Totebox)

> Hot open items. ≤200 lines. Backlog at `.agent/next-backlog.md` (not yet created).
> **Scope: this archive only.** Cross-repo and workspace-level items live at `~/Foundry/NEXT.md`.

Last updated: 2026-07-14

---

## Blocked — Command Session (route via outbox)

- [ ] **🔴 URGENT — docx-freeze bug is LIVE on 10.8.0.9:9200; deploy is Command-only.**
      The fix is committed and verified here (`ad08b8c9`) but `bin/deploy-binary.sh:32`
      refuses to run outside the Command workspace and `:131` requires HEAD already
      promoted. Needs: promote → `deploy-binary.sh app-privategit-workbench` → restart.
      Sent 2026-07-14, `priority: high`, msg-id
      `project-workplace-20260714-urgent-docx-freeze-bug-live-on-10-8-0-9-`.
      Post-deploy check (all are 0 today; the 0 → non-zero flip IS the fix shipping):
      `strings /usr/local/bin/app-privategit-workbench | grep -c BINARY_EXTS`.
- [ ] **Stage 6 pending — archive root, HEAD `ad08b8c9`.** 44 commits ahead of `46ad34ce`,
      strictly linear, no merge commits. Same outbox message as above. Note `pairings.yaml`
      gives this archive `self_service: build-deploy` (not `-stage6lite`), so both
      `promote.sh` (L97) and `self-service-promote.sh` (L98) refuse to run from here —
      canonical promote is Command's by construction.
- [ ] **2 tooling defects in `bin/`** — routed to Command in the same message.
      (a) `promote.sh:428` cherry-picks with no `-m 1`, so a **merge commit is silently
      dropped at Stage 6 while printing "skip (already in canonical)"** — a silent
      data-loss path. (b) `self-service-promote.sh:56-72` auto-detects a nested
      `pointsav-monorepo/.git` and silently retargets into it; **latent here** (the
      `build-deploy` guard exits first) but **LIVE for any archive with
      `build-deploy-stage6lite` + a sub-clone**. Related: `AGENT.md` §6b is now actively
      wrong for this archive (root is authoritative, per operator).
- [ ] **Retire the `pointsav-monorepo/` sub-clone** — deferred deliberately. Do **not** do
      it until the work is durable in *canonical*, not merely in a personal staging fork.
      A `.gitignore` entry is insufficient: `self-service-promote.sh` keys on the
      *directory existing*, so retirement must neutralize `pointsav-monorepo/.git` itself
      (rename → one `mv`, fully reversible). Write inside another repo's scope → ask-first.
      Retiring it also moots the `pointsav-monorepo/CLAUDE.md` contamination item.
- [ ] **briefs/state versioning gap** — after the "Option A" gitignore change, BRIEFs + NEXT.md +
      session-context.md durability/versioning story is still unverified (NEXT.md and briefs/ are
      tracked; session-context.md is gitignored/untracked) — confirm this is the intended final
      state or needs a different versioning home.

### ✅ Resolved 2026-07-14 — the fork (root cause of the whole Stage-6 thread)

The 2026-07-07 "archive-root vs. sub-clone duplication" finding, the 2026-07-04 Stage-6
discrepancy, and the 2026-07-10 "Stage 6 pending, 2 more commits" item were **all the same
bug**. Root and sub-clone are two clones of the same upstream, both on
`cluster/project-workplace`, forked at `46ad34ce` (2026-06-25). Reconciled at `ad08b8c9`
(root now authoritative, operator decision). See BRIEF §S8.

**Correction to this file's own record:** the 2026-07-10 entry claimed the copy-path +
docx-freeze work was *"both rebuilt, redeployed, verified live."* **It was not.** That
commit (`3716a786`) was never pushed to any remote and never deployed — it sat in one
un-backed-up working directory for four days while the bug stayed live in production. It
is now backed up to `origin-staging-j` and ported into the root. This false "verified live"
claim is precisely what let the bug hide; treat prior "verified" claims in this file as
unverified until re-checked against the deployed binary.

## Active (Totebox scope)

- [ ] **🔴 OPERATOR — screen-reader pass.** The ARIA work (`032a992a`) is verified by
      headless-Chromium keypresses, which proves the *keyboard contract* but not what a
      screen reader actually announces. Needs a pass with real assistive technology.
      Also worth a human eye once Command deploys: open a real `.docx` (should show the
      "no preview" placeholder, **not** freeze the tab), and the "⎘ Copy name" /
      "⎘ Copy path" buttons. Palette now opens with **F1** (or Ctrl+Shift+P outside Firefox).
- [ ] **Tauri 2 migration (6 crates)** — operator-approved 2026-07-14, replacing the
      impossible `libsoup2.4-dev` plan (BRIEF §S2). Zero new apt packages. Pilot
      `workbench` (only crate needing zero plugins); `presentation` **last** (its frontend
      does all privileged work via v1 JS globals). Pre-empt first: icons are missing in
      **5 of 6** crates (`generate_context!` hard-fails), and `proforma/src-tauri/src/main.rs`
      has a **pre-existing compile error** (missing `use tauri::Manager;`) — it could never
      have compiled on any host.
- [ ] **The real B1/B2** — the frontend shared-chrome JS/CSS module was **never built**;
      the committed `workplace-shell-chrome/` crate is a Rust *config-persistence* crate
      that merely shares the name (BRIEF §S4). `app-workplace-http-prototype`'s workbench
      (4,744 lines) still has **zero** command palette and **one** ARIA attribute. Rename
      the crate when building the real module.
- [x] ~~**5 ARIA findings (A–E)** on `app-privategit-workbench`~~ — **DONE 2026-07-14
      (`032a992a`).** F1 fallback (palette was unreachable by keyboard in Firefox), roving
      tabindex + arrow-key tree traversal, focus trap, pinned-header role, live regions.
      Verified 13/13 by driving real headless Chromium over CDP with real keypresses. Also
      fixed a pre-existing latent null-deref crash and a role-gated Enter/Space handler the
      role change would have silently killed. **Screen-reader pass still recommended.**
- [ ] **`bin/capture-trajectory.sh`** (L2 trajectory capture) — never started. Spec at
      `conventions/trajectory-substrate.md` says **one week**, not the BRIEF's "one evening".
- [ ] **Correct the schema-framework matrix** in `BRIEF-workplace-workbench.md:122-133` —
      the `schedule` row names `app-workplace-schedule`, a directory that does not exist.
- [ ] **app-workplace-aibridge Phase 3** — deeper docengine + crdt cross-crate composition layers
- [ ] **moonshot crates Phase 3** — parser incremental retokenize; crdt undo/redo hardening; bim-engine full STEP grammar
- [ ] **NEW (2026-07-09) — session-start.md stale sub-clone-branch note** — says the
      `pointsav-monorepo` sub-clone "tracks main... not a cluster branch," but it is actually on
      `cluster/project-workplace` (confirmed 2026-07-09, 18 commits ahead of origin/main),
      consistent with the newer workspace-wide cluster-branch policy. A recent sub-clone commit
      (`5b95ecb3`) fixed an adjacent copy-paste bug in the same doc but left this line stale.
      Update to match current policy.
- [ ] **NEW (2026-07-09) — 2 unactioned inbox reclaim requests from Command:** (1) 3 drafts
      misrouted to project-editorial's drafts-outbound
      (`DESIGN-TOKEN-CHANGE-wp-tokens-20260602.draft.md`, `JOURNAL-NOTES-j3-20260602.draft.md`,
      `JOURNAL-NOTES-j6-20260602.draft.md`); (2) 3 BRIEF files misfiled in project-editorial's
      briefs dir (`BRIEF-workplace-workbench.md`, `BRIEF-workplace-architecture.md` [superseded],
      `BRIEF-workplace-roadmap.md` [superseded]). Both need reclaiming/relocating into this
      archive next substantive session.

## Completed (2026-07-10 — Copy-path feature + docx-freeze bugfix)

- [x] **"Copy file path" toolbar button + context-menu entries** — both `app-privategit-workbench`
      and `app-workplace-http-prototype`; shared `copyToClipboard()` helper extracted in each
      file. Plan-mode approved (`can-we-add-the-parallel-pony.md`). Both services rebuilt +
      redeployed + verified live.
- [x] **docx-freeze bugfix** — `fileMode()` in both apps defaulted every unrecognized extension
      to `'text'` mode, rendering raw binary bytes as a giant garbled string and freezing the
      tab on `.docx` (and any other unlisted binary format). Added `BINARY_EXTS` allowlist →
      new `'unsupported'` mode with a short placeholder message instead. Applied to both apps
      (operator confirmed after the parallel bug was found in the sibling app). Both services
      rebuilt + redeployed + verified live.
- [x] **PPN port-mapping doc-drift found and memory-logged (not fixed this session):**
      `app-workplace-http-prototype/CLAUDE.md` claims `10.8.0.9:9200` reaches it; live nginx
      config actually routes that URL to `app-privategit-workbench` (:9210). See memory
      `ppn-nginx-port-mapping-drift`.

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
