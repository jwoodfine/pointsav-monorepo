# NEXT.md — project-workplace (Totebox)

> Hot open items. ≤200 lines. Backlog at `.agent/next-backlog.md` (not yet created).
> **Scope: this archive only.** Cross-repo and workspace-level items live at `~/Foundry/NEXT.md`.

Last updated: 2026-07-15

---

## 2026-07-17 session — DEV feature-parity + visibility

- [x] **DEV is now a TRUE superset of PROD.** Audit ("where are all my features") found DEV had
      more features than PROD except one genuinely-missing control — the file **Sort button**
      (a July fork leftover, never in DEV's branch). Restored it (Name ↔ Date; DEV listing has
      mtime, not size). The PROD-vs-DEV functional-marker diff is now **empty**. [2026-07-17 totebox@claude-code]
- [x] **Visible Surface bar** added (Files · Memo · Proforma · BIM · Tokens) between header and
      tab-bar — surfaces were palette-only before. Buttons open each surface (new tab). [2026-07-17 totebox@claude-code]
- [ ] **🟡 OPERATOR — click-test on `:9500`:** Sort button cycles A–Z ↔ Date and the tree
      reorders (incl. expanded folders); the Surface bar buttons open each surface. [2026-07-17 totebox@claude-code]

## 2026-07-16 session — yesterday's work into DEV (chat + search + schema)

- [ ] **🟡 OPERATOR — click through DEV at `http://10.8.0.9:9500/`** (chat + search):
      (1) **AI chat** — Ctrl/Cmd+L (or palette → "Toggle AI Chat"), send a message; reply
      takes 3–50s from local OLMo. (2) **Search** — type in the sidebar search box; expect
      two bands (In file names / In contents) with snippets + a coverage line, and results
      that include `.rs`/`src/` files the old search could never find. [2026-07-16 totebox@claude-code]
- [ ] **🔴 Strike is an ephemeral bg process — needs a systemd unit (Command scope).**
      `service-search` Strike serves DEV search on `127.0.0.1:9310` but is a bare background
      process right now (like the :9119 canary) — **dies on reboot**, and DEV search then
      shows "index unavailable". Also needs the Forge on a timer to keep the index fresh.
      Routed to Command. Manual restart meanwhile:
      `/srv/foundry/cargo-target/jennifer/service-search/release/strike \
       /srv/foundry/cargo-target/jennifer/service-search/dev.toml &`
      (re-forge with `release/forge <same config>` if the index is stale/missing).
      [2026-07-16 totebox@claude-code]
- [ ] **`service-search` → monorepo root workspace member (Command scope).** It is a
      standalone `[workspace]` with its own `[patch.crates-io]` mirroring root's vendored
      Tantivy. Making it a root `members` entry (and dropping the standalone block) touches
      the shared root `Cargo.toml` — Command's. Routed. [2026-07-16 totebox@claude-code]
- [ ] **Migrate `app-privategit-design` off the legacy `InvertedIndex`.** Phase 2 kept
      `moonshot-index::InvertedIndex` alive ONLY because `app-privategit-design` still uses it
      (`src/state.rs`, `src/main.rs`). It should query the `service-search` Strike instead —
      then `InvertedIndex` can be deleted and there is truly one search. Same-archive work,
      not yet done. [2026-07-16 totebox@claude-code]
- [ ] **`os-totebox` integration for `service-search`** — its README + the operator model
      say it runs once inside an `os-*` bundle (the Doorman precedent), other `service-*` log
      in. That OS wiring is `os-totebox`'s archive scope — route a handoff. [2026-07-16 totebox@claude-code]
- [x] **Phase 1 — AI chat into DEV** (commit `a66a4059`): ported from http-prototype; toggle
      via Ctrl/Cmd+L + palette (not ShellChrome); fetch on `/_api/edit/chat` (120s timeout).
      Verified live (model returned PONG). [2026-07-16 totebox@claude-code]
- [x] **Phase 2 — one search engine** (commit `1a1dac92`): deleted the hand-rolled `SearchEngine`
      BM25 (−232 lines); `moonshot-index` = trigram floor only; added `index_dir` exclusions.
      13 tests green. [2026-07-16 totebox@claude-code]
- [x] **Phase 3 — `service-search` activated** (commits `b5a22ae1`+`cfeb5efc`): Forge + Strike;
      trigram floor ∪ Tantivy BM25; ~6MB Strike RSS; substring guarantee verified live +
      integration test. [2026-07-16 totebox@claude-code]
- [x] **Phase 4 — DEV search wired to the Strike** (this session): two-band server search
      replaces the filename-only client filter; verified on a real 7,489-file index.
      [2026-07-16 totebox@claude-code]
- [x] **Phase 5a — Design Tokens surface** (commit by Jennifer): read-only DTCG browser at
      `/tokens`, self-contained light CSS, palette entry "Open Design Tokens". [2026-07-16 totebox@claude-code]
- [x] **Phase 5b — Proforma surface** (commit by Peter): write surface on the workbench roots
      API (PUT /_api/edit/file + X-Foundry-Editor + mtime 409 guard); doc→surface routing +
      fixed the `proforma-v2.0` detection bug. Verified read/save/409/403 live. [2026-07-16 totebox@claude-code]
- [x] **Phase 5c — BIM surface** (commit by Peter): `/bim` on the roots API, full dark→light
      CSS conversion (VS Code Light+ JSON syntax colours), doc-routing `bim-workspace` →
      "Open in BIM". Verified read/save live. [2026-07-16 totebox@claude-code]
- [x] **Phase 5d — Memo surface** (commit by Peter): `/memo` rich-text editor on the roots API;
      collapsed /style.css + html.light into one self-contained light stylesheet; reduced to v1
      edit-mode (New/Save-As/recent-list/SSE dropped — needed workspace_dir endpoints); token
      panel → /_api/edit/tokens-data; doc-routing any `.html` → "Open in Memo". Verified live.
      [2026-07-16 totebox@claude-code]
- [ ] **Phase 5 follow-ups (all surfaces):** (1) a real surface BAR (not just palette entries)
      for switching between surfaces; (2) a "New <schema>" CREATE flow — v1 is edit-only (open
      an existing file via `?path=`); creating needs a target-path picker in a writable root
      (memo's Save-As modal HTML + `showSaveAsModal()` are retained-but-dead for this);
      (3) memo's theme-toggle button is now inert (no dark styles) — hide it or wire a real
      light/dark. Presentation/Schedule/Code/PDF/GIS are prototype "Coming soon" placeholders,
      nothing to port. [2026-07-16 totebox@claude-code]

---

## 2026-07-15 session — PROD/DEV locked pair

- [ ] **🟡 OPERATOR — click through DEV at `http://10.8.0.9:9500/`.** Everything below is
      verified by curl/probe only; the browser pass is yours. Confirm: the tree shows your
      real project folders; open a file; Copy name / Copy path / Duplicate / Rename;
      open a real `.docx` (**expect the "no preview" placeholder, NOT a frozen tab** — this
      is the docx fix PROD still lacks); palette on **F1**. [2026-07-15 totebox@claude-code]
- [x] **DEV rebuilt from HEAD + nginx parity fix + moved to :9500.** Root cause was **nginx,
      not the binary** — PROD/DEV binaries were byte-identical (`0d474655…`). The DEV vhost's
      `/_api/` catch-all rewrote `/_api/edit/file` → `/edit/file` (**every action 404'd**) and
      never routed `/_api/{command,clones,staged}/` to the `:9211` lister (**empty tree**).
      Rewrote DEV vhost to full parity + `:9207`→`:9500`; rebuilt DEV binary (sha
      `6e74d2d3…`). DEV now matches PROD on all 7 probe paths and is strictly ahead
      (docx fix + palette + ARIA + copy-path). PROD provably untouched: sha `0d474655…`
      unchanged, MainPID `4187400` unchanged. nginx backup:
      `/etc/nginx/backups/nginx-intranet.conf.bak-20260715`.
- [ ] **Retire the mislabeled `:9207` ufw rule** — annotated `bim-lab: app-privategit-bim
      preview` but nothing bim-related uses it; the workbench was squatting on it and has now
      moved to :9500. Deleting it is Command's audit call — routed. [2026-07-15 totebox@claude-code]

---

## 2026-07-14 session — in-scope resume items (Totebox)

> Full session state + crash recovery: `BRIEF-workplace-institutional-quality-roadmap.md` §S10.
> Out-of-scope items (Stage 6, Phase 5, infra audit) routed to Command outbox this shutdown.

- [ ] **Restart the :9119 chat canary after any reboot** (else `http://10.8.0.9/` → 502) —
      exact command in §S10 §C. [2026-07-14 totebox@claude-code]
- [x] ~~**Search v1:** wire search into a workbench UI (two-band/coverage-line); Tantivy ranked
      layer~~ — **DONE 2026-07-16:** `service-search` (Forge/Strike) fuses `moonshot-index`
      trigram floor + vendored Tantivy; two-band + coverage-line UI live on DEV `:9500`.
      (`SearchEngine` was deleted.) **Still open:** incremental reindex (`notify` + stat/hash);
      scope-chips + index-health dot; `gix` git-history (v2). [2026-07-16 totebox@claude-code]
- [ ] **CAD Phase 0 next:** more entities + DXF/SVG I/O (`dxf-rs`) + snapping, THEN the
      `wgpu` 2D renderer (first GPU/WASM step; needs a browser harness). [2026-07-14 totebox@claude-code]
- [ ] **Workbench-core #3 (§S9):** file access, back button, tabs, toolbar-stays-put,
      don't-trap-in-surface, document→schema routing — captured, none built; prerequisite for
      the `app-privategit-workbench` rebuild (MUST follow STABLE/DEV canary rule). [2026-07-14 totebox@claude-code]
- [x] ~~**AI chat:** move chat into the real workbench~~ — **DONE 2026-07-16:** live in the DEV
      `app-privategit-workbench` (Ctrl/Cmd+L → `/_api/edit/chat` → Doorman/OLMo). **Still open:**
      streaming responses; the `:9119` canary's persistent systemd unit. [2026-07-16 totebox@claude-code]
- [ ] **Operator runtime passes owed:** browser click-through of the 6 migrated Tauri apps +
      B1/B2 palette + AI chat; macOS `window.print()` (pdf/proforma/memo); presentation fs-scope. [2026-07-14 totebox@claude-code]

---

## Blocked — Command Session (route via outbox)

- [ ] **🔴 HIGH — gate PROD against the automated nightly clobber (Command scope).**
      `conventions/software-units.yaml:195` registers `app-privategit-workbench` (PROD,
      :9210) with `services: [app-privategit-workbench]`; `bin/deploy-binary.sh` does
      `systemctl stop` → `install` → `start` against it; **`foundry-nightly-build.timer`
      is active** (01:00 daily). **No `-dev` entry exists.** `queue.jsonl` is empty so
      PROD is safe tonight, but AGENT.md step 5b tells Command sessions to
      `nightly-build-plan.sh --add <binary>` on build-requests → would clobber PROD
      unattended. **This is almost certainly what burned Jennifer.** Needs: `operator_gated:
      true` on the PROD entry, enforcement in `bin/deploy-binary.sh` +
      `bin/nightly-build*.sh` (they read no gate field today — a YAML comment alone is
      decorative), and an `app-privategit-workbench-dev` entry so the default target is DEV.
      Routed 2026-07-15. [2026-07-15 totebox@claude-code]
- [ ] **🔴 docx-freeze bug STILL LIVE on 10.8.0.9:9200 — deploy HELD BY OPERATOR, not stalled.**
      Command replied 2026-07-15: fix + analysis confirmed correct; `:9200` verified to be
      Jennifer's **actual live working instance**, so the operator chose to hold the
      promote+deploy rather than interrupt her session. Timing decision, not a defect —
      **do not re-escalate; it needs an operator go-ahead, not another message.**
      Re-verified live 2026-07-15: deployed binary dated Jul 13, `strings
      /usr/local/bin/app-privategit-workbench | grep -c BINARY_EXTS` = **0**.
      **The fix IS now testable on DEV** — `http://10.8.0.9:9500/` serves it (BINARY_EXTS=2).
      Promotion is a byte-copy from the DEV binary, gated on her go-ahead (see CLAUDE.md).
- [ ] **Stage 6 pending — archive root, HEAD `330f83d9`.** 44 promotable code commits,
      strictly linear, **0 merge commits** (re-measured 2026-07-15). The root is 93 commits
      ahead of `origin/main` in total; 49 are `.agent/`-only and filtered by `promote.sh`,
      leaving the 44 — the previously-recorded figure was correct. The 31 commits since
      `ad08b8c9` are all docs-only. Note `pairings.yaml` gives this archive
      `self_service: build-deploy` (not `-stage6lite`), so both `promote.sh` (L97) and
      `self-service-promote.sh` (L98) refuse to run from here — canonical promote is
      Command's by construction.
- [x] ~~**2 tooling defects in `bin/`**~~ — **FIXED by Command 2026-07-14 (commit `a7e444a`),
      verified here 2026-07-15.** (a) `promote.sh` now counts parents (`:429`) and
      cherry-picks merge commits with `-m 1` — the silent data-loss path is closed.
      (b) `self-service-promote.sh` now takes `--repo-root` (`:51`) and prints both repos'
      state before retargeting. **Still open:** `AGENT.md` §6b's text remains stale (says
      treat the nested sub-clone as the code repo unconditionally); Command logged it in
      workspace NEXT.md.
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
- [x] ~~**`bin/capture-trajectory.sh`** (L2 trajectory capture)~~ — **BUILT by Command
      2026-07-15 (commit `890f287`)**; `conventions/trajectory-substrate.md` L2 row now reads
      `DONE 2026-07-15`. Verified here. Uses L1's real Doorman `/v1/shadow` POST transport
      (task_type `trajectory-capture`), not a direct corpus file write. Usage:
      `capture-trajectory.sh "<summary>"`, stdin, or `--file`; `FOUNDRY_NO_CAPTURE=1` opts out.
      **Not wired into any session-end hook** — that wiring is a separate, unmade decision.
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
