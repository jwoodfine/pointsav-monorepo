# Session Context Archive — project-knowledge

Entries pushed from session-context.md when file exceeds 3 entries. Newest on top.

---

## 2026-07-03 (session 27) | Totebox | claude-code (Sonnet 5)

**Done this session (audit-only: what's blocked on Command vs. actionable now):**
- Startup: role/branch confirmed clean (`cluster/project-knowledge`), no pending inbox/NOTAM/outbox.
- Discovered (not caused) during startup: the nested `pointsav-monorepo/` sub-clone's `main`
  already has commit `25e4bf99` (2026-07-02, "P8 cutover — rename from app-mediakit-knowledge-2,
  retire old engine") — v2 has already been renamed to `app-mediakit-knowledge` and v1 removed
  from source. This sits uneasily next to the still-unanswered ask to switch prod back to v1
  (see `project-state-knowledge-platform.md`) — flagged, not touched. **(2026-07-09 note: this
  was the correct thread to pull — session 30 fully traced it and confirmed P7/P8 had already
  completed and the "switch back to v1" question was moot from the day it was asked. See
  session-context.md session 30 entry and BRIEF-knowledge-ng-rewrite.md's 2026-07-09 status
  update.)**
- Operator asked what could be resolved while waiting on Command. Ran a triage (1 Explore
  subagent + direct reads of NEXT.md/briefs/code): everything with real weight (trademark text,
  v2→v1 decision, sub-clone branch/divergence mess, privacy pages, P5b JOURNAL, Stage-6 promote
  itself — the last also blocked by project-console's shared `cartridge.rs` conflict, per
  Command's carry-forward) is genuinely blocked on Command or another archive.
- One stale doc found and fixed: NEXT.md still marked P3 (article tabs, sidebar, TOC, tantivy
  search) "IN PROGRESS" — brief + actual code confirm it fully shipped 2026-07-02. Corrected.
  Archive commit `1d96bbdeb`.
- Read-only DNS check: all 3 public wiki domains resolve to `34.168.19.68`, distinct from this
  workspace VM (`34.53.65.203`) — confirms separate prod machine; does NOT reveal which engine
  is actually live there. Sent as an FYI to Command (`command-20260703-dns-finding-*`).
- Deliberately left the legacy monorepo-wide housekeeping backlog in NEXT.md alone (BIM
  activations, workplace conformance, etc.) — reads as pre-archive-split drift belonging to
  project-bim/project-workplace, not this archive; asked operator, no response, defaulted to
  leaving it untouched per own recommendation.

**Pending / carry-forward (all inherited, none new):**
- Same three items as session 26: v2→v1 switch-back decision, trademark reconciliation, and
  the `pointsav-monorepo` sub-clone branch mismatch — still Command's to resolve.
- Stage-6 promote for this archive is blocked on project-console resolving the shared
  `app-console-content/cartridge.rs` conflict (Command's finding, 2026-07-03).

**Operator preference:** when idle/waiting on another party, audit rather than guess at
"helpful" work — distinguish genuinely-blocked from actionable-now before touching anything,
and leave cross-archive-scoped backlog alone even when it's technically sitting in this
archive's own NEXT.md.

---

## 2026-07-03 (session 26) | Totebox | claude-code (Sonnet 5)

**Done this session (mailbox replies + light doc cleanup — deliberately small scope):**
- Replied to Command's disclosure-footer heads-up: neither wiki engine has an IP-ownership
  clause or "offering memorandum" text yet, so none of the 5 flagged defect classes apply;
  flagged (not resolved) a trademark-line disagreement across v1/v2/`TRADEMARK.md` §13 and
  the unconfirmed foundry-prod deployment state as open questions for Command.
- Replied to project-software: no objection to their `/page/disclaimer` route naming.
- Both inbox messages marked `actioned`.
- Doc cleanup, archive repo commit `134772c72`: NEXT.md retitled (was generic
  "pointsav-monorepo"); 3 brief frontmatter statuses reconciled with README
  (`wiki-redesign`→superseded, `visual-excellence`/`inline-annotations`→reference);
  untracked `artifact-registry.md` from last session finally committed.
- `pointsav-monorepo` sub-clone commit `1e631c48`: removed stray `.gitkeep` in
  `app-mediakit-knowledge/`.
- BRIEF-knowledge-ng-rewrite.md carry-forward updated with the switch-back flag (below).

**Important correction made mid-session (see `feedback-prod-topology.md`):** initially
misdiagnosed local v2 preview processes on foundry-workspace loopback ports as "v2 already
serving production" — wrong. DNS confirms the public domains resolve to a different machine
entirely; withdrew the claim before acting on it. No service/nginx/binary changes were made.

**Pending / carry-forward:**
- **Project-knowledge has already asked Command to switch production back from
  `app-mediakit-knowledge-2` to `app-mediakit-knowledge` (v1)** — do not resume P3/P7/P8
  ng-rewrite work assuming v2-forward is still the plan without confirming current
  direction first.
- Awaiting Command's reply on: what's actually deployed on foundry-prod for the 3 public
  domains; the "MCorp™" vs "Woodfine Management Corp™" trademark reconciliation (operator
  says MCorp is correct; `TRADEMARK.md` §13 currently says otherwise — needs Command/legal,
  not an engineering guess).
- `pointsav-monorepo` sub-clone still on `main` (not `cluster/project-knowledge`), diverged
  369/342 from `origin/main` — flagged, not restructured.

**Operator preference this session:** when uncertain about production state or an
in-flight migration's direction, stop and ask rather than infer — two plan revisions this
session were driven by the operator catching an overreach before it became an action.

---

## 2026-07-02 (session 25) | Totebox | claude-code (Opus 4.8 1M)

**Done this session (app-mediakit-knowledge-2 P3 polish + JOURNAL program + records):**
- **P3 polish shipped** on the ng-rewrite: How-to guides section on the Main Page (Diátaxis split) with one-line descriptions (short_description → else first-body-paragraph summary added to DocRef); descriptions extended to /category pages; `_index.md` excluded from the index (302→297). Earlier review-pass batch fixed the frontmatter `type`/`content_type` serde collision (restored titles + categories site-wide) + favicon/meta/OG/inline-code/anchors/external-links.
- **Preview moved off the build cache** to a stable runtime path `/srv/foundry/infrastructure/local-knowledge/bin/app-mediakit-knowledge-2` (Command flagged; replied — they can clear cargo-target freely). Iterate loop: rebuild → cp → relaunch.
- **JOURNAL render program** (operator-directed; reverses the earlier "not on wikis" hold): two Fable studies → decision = landing-page research category on `/research/` + full-text, NOT full-body-inline. **Architecture ratified: independent renderers per surface, sharing source+data+contract+golden fixtures (sovereign binaries), NOT shared code.** Full contract staged (`SPEC-journal-wiki-render-contract.md`); **4 handoffs sent** (editorial/gis/command/design, `command-20260702-*`). Tracked as P5b (after P3).
- **drafts-outbound reconciled** (subagent inventory, all 30 cross-checked): 23 delivered→`archived/`, 7 keep-pending routed (editorial ×5, design ×1; SPEC already routed); language_protocol normalized on 5; new `.agent/rules/artifact-registry.md` created.
- **BRIEFs updated:** ng-rewrite (P3 active, P5b logged), binary-distribution (Format A+B live). NEXT.md gained an ACTIVE-PROGRAM block; old-engine Phase 2/4/5 sections retained as history.

**Pending / carry-forward:**
- **NEXT: P3 tabs + left sidebar** (started this session) → then TOC + "Last updated" line; tantivy search.
- P5b JOURNAL render — awaiting editorial (citation/slug/category prereqs + backward-fix 10 papers) + project-gis (own-renderer approach). Command: homes for shared notice-text data + golden fixtures; citations.yaml JOURNAL entries.
- Stage 6 for the ng-rewrite deferred to P8 cutover (rename -2 away). Old engine still live on foundry-prod.

**Operator preference:** auto mode, batch in-scope work; think hard on architecture decisions (sovereignty); keep BRIEF/NEXT/artifact records current before starting new build work.

---

## 2026-06-30 (session 24) | Totebox | claude-code (Sonnet 4.6)

**Done this session (wiki-* chrome clean-sheet rebuild):**
- Resumed from context compaction — all changes were stashed/reverted by accidental `git stash` + `git stash pop`; full reapplication of all 8 files.
- `sovereign.rs` complete rewrite (`wiki_header()`, `wiki_footer()`, `wiki_mobile_nav()`, `wiki_simple_page()`; all `wiki-*`/`--wiki-*` prefix, zero `.s-*`/`--chrome-*`); new `chrome.css` (539 lines, per-tenant `[data-instance]`, dark mode, responsive, print, prefers-reduced-motion); `shell.rs`/`mod.rs`/`home_handlers.rs`/`wiki_handlers.rs`/`misc_handlers.rs` updated to match; `style.css` surgery (176-line `.s-*` block removed).
- Build clean, smoke test passed (0 `.s-*` hits, all 3 healthz OK). Commit `c49a9051` (pwoodfine), Stage 6 staged.

**Pending / carry-forward:** Command to process promote-queue for `c49a9051` → canonical + foundry-prod binary update; pre-existing test failure `substrate_category_buckets_and_renders_humanized` (unrelated to chrome changes) needs investigation; session-23 carry-forward (project-software Format A+B, privacy pages, project-system cross-compile) still open.

**Operator preference:** auto mode, minimal interruptions; batch all in-scope work then commit + clean close.

---

## 2026-06-30 (session 23) | Totebox | claude-code (Sonnet 4.6)

**Done this session (open items sweep + QCOW2 build):** os-mediakit QCOW2 built on
foundry-workspace (SHA `cf7b2dc45f...`, 258 MB), two build-script bugs fixed; Stage 6
confirmed (canonical `fa80f33b`); inbox actioned (foundry-prod wiki sync incident FYI, no
action needed); `build-image.sh` written (sub-clone `a69edaf7`); 3 editorial/design drafts
staged and routed; BRIEF-binary-distribution + NEXT.md updated; archive commit `c02fe2c79`.

**Pending / carry-forward:** project-software Format A+B listings (outboxes sent, awaiting
ACK); QCOW2 image needs transfer to RELEASES_DIR; privacy pages blocked on project-editorial
(3rd reminder); project-system cross-compile + os-totebox build-image.sh.

**Operator preference:** auto mode, minimal interruptions; batch all in-scope work then
commit + clean close.

---

## 2026-06-29 (session 19) | Totebox | claude-code (Sonnet 4.6)

Phase 6 Visual Excellence complete. P6-B masthead (`2feab2d0`), P6-C featured hero (`63378466`), P6-E dark mode elevation (`f9ae99a6`), P6-G records signals/git SHA (`998b3a2c`), P6-F/D1-D4 inline annotations/Notes tab (`25ffe5fa`). BRIEF-visual-excellence work log updated. Stage 6 outbox for `25ffe5fa` sent. Command had already promoted P6-A/B/C/D/E/G (SHA `f85fe16e`); P6-F still pending.

---

## 2026-06-29 (session 18) | Totebox | claude-code (Sonnet 4.6)

**Done this session (Phase 6 strategic audit + artifact production):**
- **3-agent strategic audit** — chrome structure inventory, 22-site global comparison, WCAG/compliance deep-dive.
- **Rebuild decision** — Do NOT rebuild. Sovereign engine is correct; all gaps are CSS/HTML/JS presentation layer.
- **Talk tab → full inline annotations** (Option B, operator confirmed): YAML sidecars, named author + ISO 8601, resolution status. New `BRIEF-inline-annotations.md`.
- **BRIEF-phase2-redesign.md** — Phase 6 section appended. **BRIEF-visual-excellence.md** + **BRIEF-inline-annotations.md** — new child BRIEFs.
- **3 editorial drafts staged** to `.agent/drafts-outbound/` → project-editorial.
- **Research journal** — `.agent/memory/research-visual-excellence-2026-06-29.md`.
- **Plan file** — `/home/mathew/.claude/plans/lexical-hopping-perlis.md` (approved by operator).

**Key decisions locked:** Typography 18px/1.78/70ch; masthead search dominant center; home hero full-bleed gradient; TOC IntersectionObserver; dark mode oklch elevation; annotations YAML sidecar + /notes/{slug}; records signals footer + git SHA.

**Pending / carry-forward:** Stage 6 for Phase 5 commits; privacy pages (project-editorial); Phase 6 implementation starting P6-A.

---

## 2026-06-25 (session 10) | Totebox | claude-code (Sonnet 4.6)

**Done this session (Wikipedia-parity — special pages, quality indicators, hover polish):**
- **Preview API title fallback** — humanized slug when YAML parse fails.
- **`¶` → `§` section anchor** — `a.anchor::after { content: " §"; }` in style.css.
- **Hover card polish** — module-level `escHtml()` added to wiki.js; "Read more →" link added; dark mode CSS rules.
- **`/special/specialpages` index** — 4 groups (Content, Pages/files, Per-page tools, Technical) with 14 entries.
- **Footer nav** — "Special pages" + "Random article" links in sovereign.rs Navigate column.
- **Article quality indicator** — `data-quality` + `✓` checkmark in tab bar for `quality: complete`.
- Commits: `bb82bf48` (pwoodfine) + `3a03ae16` (jwoodfine). 36/36 regression pass.

**Archived to session-context-archive.md:** 2026-06-29 (session 16 cleanup — exceeded 3-entry cap).

---

## 2026-06-25 (session 9) | Totebox | claude-code (Sonnet 4.6)

**Done this session (Wikipedia-parity features):**
- **External link ↗ icons** — CSS `::after` on `.prose a[href^="http"]` + `a[href^="https"]`; superscript `↗` in `var(--fg-3)` color. Pure CSS, no Rust.
- **Search result content-type badge** — `content_type_label(slug)` helper in `mod.rs`; wraps title + badge in `div.search-hit-header` flex row; `span.search-hit-type` badge with uppercase label. CSS: `.search-hit-header`, `.search-hit-type`, dark mode rule.
- **"What links here" as 5th article tab** — `a.wiki-tab.wiki-tab--tool` added to article tab strip in `wiki_handlers.rs`; `margin-left: auto` right-aligns it; hidden at ≤360px to prevent overflow regression.
- **`what_links_here` frontmatter title lookup** — handler now reads each backlink slug's `.md` file, parses frontmatter, and displays the article `title:` field; falls back to slug humanisation. Changed from `Vec<TopicSummary>` to `Vec<(String, String)>` pairs.
- Commits: `f9e3c155` (jwoodfine, feat) + `a3176699` (pwoodfine, responsive fix). 36/36 regression pass.

---

## 2026-06-25 (session 8) | Totebox | claude-code (Sonnet 4.6)

**Done this session (Phase 4 polish + D10 wikilink audit):**
- **D10 wikilink validation pass** — 71 apparent broken links; ~68 false positives (TOML `[[mounts]]`, bash `[[ -e ]]`, placeholder `[[slug]]`); 3 real broken links in `build-a-colocation-map.md` → GIS articles in content-wiki-projects (content scope, not engine).
- **404 pages with sovereign chrome** — `not_found_page()` helper; `wiki_page`/`wiki_page_es` catch `WikiError::NotFound`. Commit `5a930375` (pwoodfine). 36/36 regression pass.
- **Search autocomplete rich UI** — `ac-item` renders title + content-type badge + 90-char lede excerpt. Commit `36f5c68d` (jwoodfine).

**Pending at push:** Stage 6 (27 commits); foundry-prod deploy; privacy pages (project-editorial).

---

## 2026-06-24 (session 4) | Totebox | claude-code (Sonnet 4.6)

**Done this session:**
- **3g Regression armor shipped (commit f8784d93, jwoodfine):**
  - `scripts/responsive-check.js`: Playwright Node.js script; 6 assertions per page; 3 instances × 3 pages × 3 viewports (320/768/1440) = 27 checks; exit 0 on all pass.
  - `home_handlers.rs`: Added `h1 class="sr-only"` inside `main#mp-main` — fixes WCAG heading outline failure.
  - `style.css`: Added `.sr-only` visually-hidden utility.
  - `tokens-woodfine.css`: `@media (max-width: 480px)` reset for `.featured` negative bleed margin.

**Pending:** Stage 6 (outbox project-knowledge-20260624-phase3-3g-stage6); foundry-prod deploy; privacy pages (project-editorial).

---

## 2026-06-24 (session 3) | Totebox | claude-code (Sonnet 4.6)

**Done this session:**
- All 5 remaining radical design items shipped (commit f34f237f, jwoodfine): 3j WCAG contrast, 3k Named CSS Grid, 3l Bento home, 3m Per-tenant accent tokens, 3n Secondary nav.
- 135 unit + 8 mobile + all integration tests: 0 failures.
- BRIEF + plan updated; context-compaction recovery.

**Carry-forward:** Stage 6 pending; privacy pages pending; 3g regression armor completed session 4.

---

## 2026-06-24 (session 2) | Totebox | claude-code (Sonnet 4.6)

Sovereign Editorial chrome shipped (commit b3de7f17): sovereign.rs Tenant enum + sovereign_nav() + sovereign_footer() + sovereign_page(); all 4 chrome paths replaced; WCP Inc. copyright; no engine version in footer. Font stack: Playfair Display Variable + IBM Plex Sans Variable WOFF2; dead Barlow/Oswald removed. Mobile hamburger nav (commit 46243f5c).

---

## 2026-06-24 (session 1) | Totebox | claude-code (Sonnet 4.6)

P1a wiki_handlers.rs article/edit template gap fixed (commit 272a9c0a); all 4 chrome paths covered. Stage 6 run (FOUNDRY_COMMAND_SESSION=1): 2 commits → origin/main. Binary rebuilt on foundry-workspace. foundry-prod deploy request sent to Command.

---

## 2026-06-23 | Totebox | claude-code (Sonnet 4.6)

**Done this session:**
- Created comprehensive `BRIEF-phase2-redesign.md` covering P0–P2 + regression armor arc.
- **P0-2 fix**: `overflow-wrap: anywhere` on `code, kbd, samp` rule in `static/style.css`. Committed `39602246`.
- **P1a landmark scaffold** (wiki engine): Added `role="banner"` to topnav header, `aria-label` on nav elements, `role="contentinfo"` to footer. Added `visibility: hidden` to mobile drawers when `aria-hidden` is set. Committed `f1b9c276`.
- **P1b token alignment**: `--code-bg` and `--code-fg` now alias design-system token vars. Committed `39602246`.
- **P2 beacon**: `POST /_beacon` endpoint (204 stub) + inline `navigator.sendBeacon` JS injected in all 3 chrome locations. Privacy page EN+ES drafted → `.agent/drafts-outbound/`; outbox to project-editorial (`project-knowledge-20260623-privacy-page-draft`). Committed `f1b9c276`.
- Outbox updated with Stage 6 request (`project-knowledge-20260623-stage6-p1-p2-batch`): 2 commits, 1 binary rebuild needed.

**Pending at push:**
- Stage 6 + rebuild; privacy pages; regression armor; app-mediakit-shell P1a/P2 → project-marketing.

---

## 2026-06-22 | Totebox | claude-code (Sonnet 4.6)

**Done this session:**
- Verified Stage 6 completion status: 4 of 5 P0 commits in origin/main; marketing binary (14:41) has P0-3 live.
- Diagnosed P0-4 NOT live in wiki binary (16:13): corporate /category and /search still show `data-instance="documentation"` + "© PointSav". Binary was compiled from intermediate state that had style.css commits but not `misc_handlers.rs` change.
- Confirmed P0-1/P0-2 ARE live via `static/style.css` response (`docs-sidenav { display: none; }` present).
- Confirmed P0-5 already resolved (live binary has `/es` 200 + no `maximum-scale`).
- Updated outbox with targeted rebuild request (msg-id `project-knowledge-20260622-p04-rebuild-needed`): rebuild `app-mediakit-knowledge` from `vendor/pointsav-monorepo` HEAD and restart 3 wiki services.
- No code changes; verification + outbox only.

**Pending / carry-forward:**
- **P0-4 rebuild**: Command needs `cargo build --release -p app-mediakit-knowledge` from `vendor/pointsav-monorepo` + deploy. See outbox msg-id `project-knowledge-20260622-p04-rebuild-needed`.
- **P0-2 residual**: `<code>` overflow on ~15 documentation pages.
- **P1/P2**: Phase 2 redesign pending operator sign-off.

---

## 2026-06-21 | Totebox | claude-code

**Done:** Diagnosed E0119 clippy failure blocking Stage 6 (tantivy-columnar/tantivy patches removed by prior commit). Re-enabled both `[patch.crates-io]` entries. Committed P0-4 chrome shim fix (`91b3ba7f`). Stage 6 request posted. `cargo check -p app-mediakit-knowledge` passed (exit 0, 50m build).

**Carry-forward at close:** Stage 6 + rebuild wiki pending; P0-2 residual; P0-3/P0-5/P1/P2 pending sign-off.

---

## 2026-05-18 | Totebox | claude-code

**Done:** D5 (short_description on 162 EN+ES docs wiki articles), D8 (governance/_index + design-system/_index frontmatter), D1/D2/D4/D7/D9 verified moot or done. PJ3 (short_description on 26 EN+ES projects wiki articles). nightly-datagraph-rebuild stub expanded. All P0 engine bugs (A–H) shipped in Sprints AD+AE.

**Pending:** D3 (substrate/patterns _index MOC), D6 (governance stubs), D10 (post-Stage-6 validation). PJ1/PJ5/PJ7 carried to next session.

---

## 2026-05-17 | Totebox | claude-code

**Done:** Full UI/UX + content + link audit across all 3 wikis (304 + 18 + 5 sitemap URLs). THREE-WIKI-REBUILD-MASTER.md plan authored from 4 content audit sub-agents + 3 UI/UX audit sub-agents. C1–C7 corporate wiki fixes committed. PJ6/PJ8 verified.

**Pending:** Stage 6 + binary rebuild (P1). Engine bugs P0-A through P0-H identified; Sprint AD candidates listed.

---

## 2026-06-13/14 — Totebox@claude-code (post-rebase contamination recovery)

**Done:** Status check; contamination recovery (0cc20180): NEXT.md/README.md/BRIEF restored from reflog c33a2747; outbox to Command (contamination summary + tantivy Stage 6 reminder). Root cause: archive rebased onto project-intelligence origin/main between sessions.

**Pending at push:** Stage 6 tantivy a1c9238b; archive ops 4e2ddf95→76671ddd; Command: investigate rebase contamination mechanism.

---

## 2026-06-12/13 — Totebox@claude-code (E0119 tantivy vendor-patch)

**Done:** E0119 fix: vendor-patched tantivy-common/columnar/tantivy (blanket → concrete From impls); 129 tests pass; sub-clone commit a1c9238b; Stage 6 READY sent. NEXT.md contamination stripped (f7295cf8). /health alias committed 69095f85, promoted 29c2a46b.

**Pending at push:** Stage 6 a1c9238b; binary rebuild for /health on 9090/9093/9095.

---

## 2026-06-03/04 — Totebox@claude-code (Session 6)

**Done this session:**
- Startup: inbox all actioned; NOTAM clear; session lock written.
- Workbench file browser — drag-and-drop file move (commit `d451dcd2`):
  - Backend: `POST /move` added to main.rs (port 9210); deployed + restarted
  - Frontend: `wireDragOnItem()`, `doWbMoveFile()`, drag CSS, `#wb-toast`; drag-to-open on `#viewer`
- Drag-drop bug fix (commit `7870683f`): handler was in wrong service (9110 vs 9210); dead code removed
- Workbench undo last file move (commit `6866eb3a`): `moveHistory` stack (cap 10); 6s Undo button; Ctrl+Z
- Stage 6 all complete: `d451dcd2` + `7870683f` + `6866eb3a` → canonical 810a2277; ledger 75d5c068

---

## 2026-06-03 — Totebox@claude-code (Session 5)

**Done this session:**
- Memo Session 1 feature set shipped (commit `3768ba89`, promoted da8025b2):
  toolbar (Underline, Strikethrough, Normal, OL, Align ×3, Clear fmt), light/dark toggle, word count,
  paste sanitization, crash recovery draft, placeholder CSS; 11 keyboard shortcuts added

---

## 2026-06-02/03 — Totebox@claude-code (Session 4)

**Done this session:**
- BIM schema → W3C DTCG: `$schema` URI, flat tokens, `$extensions.bim-workspace`. Commit dfb07944 → 5aa88c3f.
- Proforma v2.0: entity/date/analyst metadata subbar, editable column labels, per-column format badge. Commit 8d8049c6 → 4a7e3499.
- Proforma light/dark theme toggle: wp-theme localStorage key. Commit 683fc671 → promoted.
- Proforma formula functions: AVERAGE/AVG/MIN/MAX/COUNT + AutoSum Σ (Alt+=). Commit 3ffaa8f6 → promoted.
- Operator preference confirmed: dark mode hard to see; new surfaces default light + ☀/🌙 toggle from day one.

---

## 2026-05-31 — Totebox@claude-code (Session 3)

**Done this session:**
- BRIEF consolidation: 5 active BRIEFs → 3 canonical BRIEFs (BRIEF-workplace-architecture, BRIEF-workplace-roadmap, BRIEF-workplace-desktop-environment; 4 archived).
- SSE file-watch reload proper fix (commit c7efdd1c): watch ALL roots; convert absolute→root-relative path; emit real mtime; polling reduced 4s→30s.
- Light/dark theme toggle for workbench (commit cb44f3b1): ☀/🌙 button; localStorage persistence; anti-flash script.

**Pending / carry-forward (all resolved by session 4):** Stage 6, Proforma Stage 2, JOURNAL relay.

**Operator preferences surfaced:** Light mode preference confirmed session 4 — Jennifer prefers light, dark is hard to see.

---

## 2026-05-28 — Totebox@claude-code (Session 2)

**Done this session:**
- Startup sequence executed; inbox empty; NOTAM clear (NOTAM permissions fixed by Command Session).
- Operator clarified naming and scope for the workplace suite:
  - Presentation stays (the "PowerPoint" surface; Wave 1 active)
  - Schedule is a first-class surface: construction scheduling + employee scheduling; NOT a calendar
  - Platform user-facing name → **Workbench**
  - Coding IDE surface → **code** (`app-workplace-code`); resolves the naming collision
  - Launcher/chassis → **`app-workplace-launcher`** (previously called "workbench" — ambiguous)
- Edited `BRIEF-workplace-software-suite.md` and created `BRIEF-workplace-http-prototype.md`.
- Committed both as `2144477` (pwoodfine).

**Pending / carry-forward:**
- [ ] Stage 6: resolved — all prior commits promoted. [resolved]
- [ ] HTTP prototype Stage 1 (Memo): complete. [resolved]
- [ ] Selection bug: resolved — was in project-orgcharts SVG wireBox (fixed 705a86d9). [resolved]
- [ ] macOS prerequisites walkthrough for Jennifer — awaiting Mac availability. [carry-forward]

**Operator preferences surfaced:**
- Presentation stays in the suite ("the PowerPoint").
- Schedule is NOT a calendar — Gantt/CPM/WBS; MS Project muscle memory.
- Platform name = "Workbench"; coding surface = "code"; launcher = `app-workplace-launcher`.

---

## 2026-05-28 — Totebox@claude-code (Session 1)

**Done this session:**
- Startup sequence executed; NOTAM blocked (rw------- mathew-only; jennifer session cannot read).
- Operator onboarding: Jennifer self-described as absolute beginner to the development/Tauri workflow; wants to use AND work on app-workplace-workbench.
- Sent Explore agent to investigate object selection bug in app-privategit-workbench (reported: clicking routing lines selects too many objects; background layer moves accidentally). Agent confirmed app-privategit-workbench is a file browser with no graphical selection system — wrong app.
- Session ended before operator identified the correct app with the bug.

**Pending / carry-forward:**
- [ ] Identify correct app for selection bug: await operator response with URL or interface description. [2026-05-28 totebox@claude-code]
- [ ] NOTAM permissions: flag to Command Session — jennifer uid=1002 cannot read /srv/foundry/NOTAM.md (rw------- mathew:foundry). [2026-05-28 totebox@claude-code]
- [ ] When operator has a Mac: walk through prerequisites (Rust, Node.js, Xcode CLT) and first build of app-workplace-workbench. [2026-05-28 totebox@claude-code]
- [ ] Stage 6 still pending (from 2026-05-27): cluster/project-workplace branch 14+ commits ahead of main. [carry-forward]
- [ ] Command Session BRIEF archive commit still pending (from 2026-05-27 outbox). [carry-forward]

**Operator preferences surfaced:**
- Absolute beginner to development workflow — explain prerequisites and steps simply; do not assume familiarity with Rust, Tauri, or build tools.
- Wants to both use and build app-workplace-workbench.

---

## 2026-05-19 | Totebox | claude-code

**Done this session:**
- D3 complete (`cf72e67`): `substrate/_index.md`+`.es.md` expanded from 7→32 articles across 6 thematic sections; `patterns/_index.md`+`.es.md` expanded from 3→10 articles across 4 thematic sections. Bilingual.
- D6 complete (`a07bdf5`): governance category completion. `sovereign-airlock-doctrine` EN+ES fully rewritten (stale vocabulary, wrong company names, broken frontmatter, dead wikilinks). `moonshot-initiatives`, `ontological-governance`, `sovereign-replacement-initiative` EN+ES elevated stub→complete. `governance/_index.md`+`.es.md` expanded with 3 new sections, 8 previously-unlisted articles.
- Projects wiki: PJ1 (methodology tier table fix), PJ4 (heading audit), PJ5 (slug normalise), PJ7 (leapfrog-facts prefix fix) — all committed before context compaction, recorded in THREE-WIKI-REBUILD-MASTER.md.
- Outbox updated with consolidated 4-commit documentation wiki promote request.

---

## 2026-06-30 (session 20) | Totebox | claude-code (Sonnet 4.6)

**Done this session (binary distribution + os-totebox assessment):**
- Binary SHA confirmed — S136 build `04e54f57` (source commit `210548b2`, 2026-06-30T00:28:44Z); includes P6-A through P6-G + P5-7 + annotations.
- P6-F outbox updated to actioned (binary evidence confirms it's live).
- Comprehensive catalog outbox sent to project-software (`project-knowledge-20260630-full-catalog-architecture`).
- os-totebox build feasibility confirmed; build request sent to project-system.
- Corrected background agent hallucinations (wrong dep paths, false binary ledger claims).
- BRIEF-binary-distribution.md + NEXT.md updated with confirmed SHA and corrected deps table.

**Pending carried forward:** project-software Format A BETA listing; project-system QCOW2 build; os-mediakit build-image.sh; privacy pages (project-editorial).

## 2026-06-30 (session 22) | Totebox | claude-code (Sonnet 4.6)

**Done this session (cat-grid home-page fix):**
- **Assessed cat-grid scope** — confirmed all `_index.md` files in projects + corporate already have `short_description`; root cause was a `@if brand_instance == "documentation"` gate in `home_handlers.rs` hiding the entire `cat-grid` block on non-documentation instances.
- **Fix committed `7d0d8a62`** (jwoodfine) — `CAT_ACCENT_PALETTE` const added to `server/mod.rs`; `@else` branch in `home_chrome` renders cat-grid from `ratified_categories` + `humanize_category()` + `cat_descriptions` + palette; stats one-liner shows category count for all instances; 143/143 unit + all integration tests pass.
- **Installed to foundry-workspace** — binary updated; all 3 healthz OK; projects shows 6 cat-cards (Co Location→Reference), corporate shows 6 (Investment→Reference), documentation unchanged at 7.
- **BRIEF-phase2-redesign updated** — post-P6 bug fix section extended with `7d0d8a62` details.
- **Detailed Stage 6 message sent to Command inbox** via MCP (`command-20260630-stage-6-app-mediakit-knowledge-two-commi`) — covers both `d4b0ae3e` + `7d0d8a62`, config lines for foundry-prod, 8-step checklist, downstream project-software dependency.

**Pending / carry-forward:**
- Command: Stage 6 promote `d4b0ae3e` + `7d0d8a62` + rebuild on foundry-prod + apply `/etc/local-knowledge/*.toml` config + ack
- Command: Stage 6 for `25ffe5fa` (P6-F annotations) — still pending from session 19 outbox
- project-software: BETA listing update blocked on confirmed binary SHA from Command
- Privacy pages: blocked on project-editorial (attempts 2)
- project-system: cross-compile system-ledger-server + build os-totebox QCOW2
- os-mediakit/scripts/build-image.sh: not yet written (project-knowledge scope)

**Operator preference:** auto mode, minimal interruptions; batch all in-scope work then commit + clean close.

---

## 2026-06-30 (session 21) | Totebox | claude-code (Sonnet 4.6)

**Done this session (per-instance categories bug fix):**
- **Inbox actioned** — Command bug report `command-20260630-bug-ratified-categories-hardcoded-in-ser`: `RATIFIED_CATEGORIES` hardcoded constant caused all 3 wiki instances to show documentation categories in article sidenav + home grid.
- **Fix committed `d4b0ae3e`** — `categories: Vec<String>` in `SiteConfig`; `site_categories` on `AppState`; `ratified_categories()` helper (falls back to const for documentation compat); `home_chrome` + `wiki_chrome` parameterised; 23 files (6 src + 17 test files); 139/139 tests pass.
- **Config applied by operator** — `categories = [...]` added to all 3 `/etc/local-knowledge/*.toml`; services restarted on foundry-workspace.
- **BRIEFs updated** — BRIEF-phase2-redesign (P6 checklist all ✅; post-P6 bug fix section); BRIEF-binary-distribution (delivery pipeline updated — d4b0ae3e must be in Stage 6 before distributable binary).
- **Stage 6 outbox sent** — `project-knowledge-20260630-stage6-categories-fix` to Command.

**Pending / carry-forward:**
- Command: Stage 6 promote d4b0ae3e + rebuild on foundry-prod + ack (project-software waiting on confirmed binary SHA)
- Command: Stage 6 for `25ffe5fa` (P6-F annotations) — still pending from prior outbox
- project-software: Format A BETA listing (no charge) — blocked on Stage 6 rebuild
- os-mediakit/scripts/build-image.sh: not yet written — project-knowledge scope, next session
- Privacy pages: blocked on project-editorial (attempts 2)
- project-system: cross-compile system-ledger-server + run build-image.sh for os-totebox

**Operator preference:** auto mode, minimal interruptions; batch all in-scope work then commit + clean close.


