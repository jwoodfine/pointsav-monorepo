---
artifact: brief
schema: foundry-brief-v1
brief-id: project-knowledge-ng-rewrite
title: Knowledge wiki engine — 100% ground-up rewrite (app-mediakit-knowledge-2)
status: active
owner: project-knowledge
parent: project-knowledge-wiki-redesign
created: 2026-07-01
updated: 2026-09-07 (high-severity Spanish-disclosure bug fixed same-session as Command's report; reconciled to canonical, awaiting merge — Phase 2, the local appliance re-test, is next)
---

## STATUS UPDATE (2026-09-07) — High-severity bug: Spanish pages rendered the English disclosure band

Command found this during a dual-model regulatory research pass against NI 51-102's Companion
Policy on forward-looking-information disclosure: every bilingual route (`/es/wiki/{slug}`,
`/es/category/{name}`) rendered the English Important Information band regardless of the
request's actual language. Flagged high-severity given corporate/projects both carry real
BCSC forward-looking-statement disclosure obligations — a disclosure not in the reader's
language arguably doesn't satisfy its purpose.

**Root cause, confirmed directly**: `AppState.important_info` was a single `Arc<Option<String>>`
loaded once at startup from `important-information.md` only — the `.es.md` sibling (which
exists in all 3 content repos) was never read at all. All 16 places that consume this value
passed the same English content unconditionally; traced which of those 16 actually sit behind
a real bilingual route — only 4 do (`serve_category`, `render_index_topic_category`,
`serve_article`'s two return points). The other 12 (`home`, `research/*`, `search`, `history`,
`special/*`, `not_found`) are English-only routes by design — no `/es/` variant is ever
registered for them — so they were correctly untouched, not part of this bug.

**Fixed same-session** (commit `e87b533558`, reconciled to canonical as
`canonical-reconcile-2026-09-07b`, `4c5e1966c8`): `AppState` now also loads
`important-information.es.md`; a new `important_info_for(lang)` method is the only sanctioned
way to read either field going forward (doc comments on both fields warn against reading them
directly, so a future bilingual route can't reintroduce this same bug by accident).

**Verified beyond `cargo test`** (176/176 pass): ran a local instance against real
corporate-wiki content and directly compared the rendered HTML before/after. Before:
`/es/wiki/disclaimers` rendered `<h2>...Important Information</h2>` (English). After: the
same URL renders `<h2>...Información Importante</h2>` — the real Spanish content from the
repo's own `.es.md` file, not a translation invented here.

## STATUS UPDATE (2026-09-07) — Queue #22/#23 chase: #22 was stale content, #23 yielded 2 real CSS fixes

Command chased two unreplied 2026-09-05/06 escalations (Queue #22: `/category/markets` flat
dump; Queue #23: sidebar/rail/prose geometry). Investigated both directly rather than assuming
either was real or fabricated.

**Queue #22 — traced to ground, not a code bug.** Ran the exact current code against the exact
current content (`project-editorial/media-knowledge-projects`, unmodified) locally — renders
correctly (4 curated groups, 25 items), not the reported flat 91-item dump. `index_scope:
markets` has been stable across recent commits, not a fresh fix. Conclusion: `foundry-prod` was
still serving a pre-restructure content snapshot because today's deploy deliberately paused the
content-repo sync (media-knowledge-documentation's uncommitted reorg). Command confirmed this
diagnosis directly. No engine code change needed; resolves once the content sync completes.

**Queue #23 — the original escalation was never actually sent.** Neither we nor Command could
find a real mailbox message for it; traced to project-editorial's own
`BRIEF-category-index-guide-redesign.md`, where it existed only as a Carry-forward "escalate
this" item, never executed via `mailbox-send.sh` — another instance of this session's tracked
mailbox-misroute pattern, this time a message that plain never went out. Real numbers recovered
from that BRIEF: prose measured ~150-156 chars/line, rail ~192px/~21-char (a separate finding
about category-name truncation, not the viewport-height complaint).

Operator also directed a fresh Playwright audit of all 3 live sites (footer check, then a full
user-journey + regulator-journey sweep — search, article rendering, category nav, mobile,
Important Information reachability + tenant-appropriate disclosure language, disclaimer pages,
footer legal marks, `/history/{slug}` reachability, `robots.txt`/`sitemap.xml`/`llms.txt`) —
all clean, no new defects, one earlier false-positive (a projects/corporate "missing investment
advice language" flag) resolved as different-but-correct tenant-specific securities-exemption
language, not an absence. That audit caught a second, independent real bug along the way (see
below), separate from Queue #23's original findings.

**Two real CSS bugs found and fixed** (commit `742af13dc8`, reconciled to canonical as
`canonical-reconcile-2026-09-07`, `4c66bdfef7`):
1. **Important Information font-size** — a CSS specificity bug, not previously reported by
   anyone: `.k-compliance__body`'s intended 13px (`--k-text-sm`) was silently losing an
   equal-specificity cascade tie to `.k-prose`'s 16px (`content.css` loads after `app.css`),
   rendering the disclosure band at full article-prose size. Operator caught this visually live;
   confirmed via direct measurement (16px, not 13px) before fixing. Fix: compound selector
   `.k-compliance__body.k-prose` (0,0,2,0), wins regardless of file order.
2. **Prose column width** — the Queue #23 finding. `.k-prose`'s own comment already documented
   the intent ("measure applied only to running paragraphs") but no rule implemented it.
   `--k-prose-measure` (44rem/~72ch) already existed and is used elsewhere for exactly this
   purpose — wired to `.k-prose > p` specifically (not the whole container, so tables/figures/
   code stay full-width per the existing comment).

**Investigated, confirmed already correct, not touched**: the sidebar rail's sticky +
viewport-height-cap CSS has existed since 2026-07-30 (predates the 2026-09-05 finding) and
verified working live — that part of the original Queue #23 report is stale.

**Deliberately not fixed — a judgment call, not an oversight**: the uncapped "In this topic"
panel's item count itself. Project-editorial's own BRIEF documents the real fix as a content
restructure on their side (split `markets/` into `index_group`s, move city profiles off the
category index), not yet executed. No one has specified an actual engine-side cap threshold;
adding an arbitrary number now would fight a fix already planned at the source. Revisit if
Command/project-editorial want a belt-and-suspenders engine cap regardless of their timeline.

Verified throughout: `cargo test -p app-mediakit-knowledge` 176/176 pass (unchanged, CSS-only
change), plus live local-instance measurement of both fixed values matching their intended
tokens exactly (704px prose width = 44rem; 13px compliance font = `--k-text-sm`).

## Path to live (2026-09-06, operator-directed — read this before touching the reconcile branch)

Two explicit, sequenced phases. **Phase 1 first, then Phase 2** — not parallel, to keep focus
tight after tonight's host-load problems (build stalls up to 2h39m, a silent commit failure
traced to the root disk hitting 99%).

### Phase 1 — get the three live wikis (documentation/projects/corporate) onto current code

These three sites already run `app-mediakit-knowledge` directly today — this is a normal
code-update cycle, **not** blocked on Phase T2A or the open host-ingress decision (those only
gate the *new* os-mediakit seL4 appliance, a separate product, see Phase 2 below).

1. ~~**Local smoke test first**~~ **DONE 2026-09-06.** Ran the actual binary against three
   real instances: (a) real content (`media-knowledge-documentation`), `history` unset (default
   off) — `/history/{slug}` and the article's own `?rev=` as-of view both confirmed 404; (b)
   same real content, `history = "public"` — `/history/{slug}` lists real revisions (200), a
   real commit's diff view renders (200, 22 diff lines), the as-of view renders (200); (c) a
   synthetic 4-commit repo (built via `git commit-tree` plumbing, not `git commit` — this VM's
   pre-commit hook blocks direct commits even in disposable scratch repos) with a
   `redactions.yaml` entry covering the first two commits. Confirmed: the history list shows
   both redacted rows as "Revision redacted — superseded by a later correction"; `?rev=` on the
   redacted boundary sha renders the redacted-notice page with the editor's reason, and the
   real bad figure it was hiding (`$999,999,999`) appears **zero times** anywhere in that
   response — grepped to confirm, not assumed; the non-redacted "fix" commit immediately after
   the boundary still renders its real diff normally (not over-redacted). All three test
   instances torn down cleanly after, no state left running. `/research/{slug}` with a real
   draft-state paper was **not** independently tested this pass — no real content currently
   carries `category: research` (per the 2026-09-05 finding that the 3 wikis never host
   JOURNALs); covered instead by the existing axum-test-client integration tests
   (`research_landing_shows_masthead_and_abstract_not_full_body`,
   `research_fulltext_resolves_citation_and_generates_references`).
2. ~~**Finish the canonical reconcile**~~ **DONE 2026-09-06.** Build+test resumed on explicit
   operator go-ahead once load dropped to 0.38 (had swung 2.4 → 76 → 38 over the preceding
   hours) — 176/176 pass in the reconciled worktree itself, same count as the source branch.
   Committed `33f0c93131` (single parent = `origin/main`'s exact tip `9767d1c31e`, confirmed
   via direct diff to differ by exactly MANIFEST.md + the 14 files, nothing else), pushed to
   both staging mirrors as `canonical-reconcile-2026-09-06`. Supersedes
   `canonical-reconcile-2026-09-05` (`767cdbf018`, still on both mirrors, never merged by
   Command) — this one carries everything that one did plus the history-exposure fix.
3. ~~**Handed off to Command**~~ **DONE — PHASE 1 FULLY LIVE, 2026-09-07.** Command executed
   all 4 checklist steps (`command-20260907-re-what-s-needed-to-get-the-3-live-wikis`):
   fast-forwarded `canonical-reconcile-2026-09-06` into `origin/main`; added
   `history = "public"` to `/etc/local-knowledge/{documentation,projects,corporate}.toml` on
   **`foundry-prod`** (the real live host — our original checklist named foundry-workspace's
   preview units by mistake, Command caught and corrected this during execution); built +
   deployed the new binary (sha256 `3b68c4e987985f99eeedffdea822661cf699d78f6b624cf16323fb73c2c5748e`,
   source commit `33f0c93131`) to all 3 live services; smoke-checked `/healthz` + a real
   `/history/{slug}` 200 (not 404) to confirm the config gate took effect *before* the restart.
   **Independently re-verified via Playwright against the real public domains** (not trusting
   the report alone): all 3 sites — `documentation.pointsav.com`, `projects.woodfinegroup.com`,
   `corporate.woodfinegroup.com` — confirmed homepage 200, 4-column footer with the new
   Machine-readable column, pager confirmed absent, `/healthz` ok, and `/history/{slug}` on a
   real article returning 200 with real revision rows (13/9/12 respectively) rather than a
   silent 404 regression. Full match with Command's own report.
   **One deliberate, correct deviation**: the normal deploy also `rsync --delete`s the 3
   content repos from project-editorial's clones. Command found `media-knowledge-documentation`
   mid-way through an uncommitted reorg (staged, not committed) and held that specific step
   rather than push a half-finished restructure to 3 live public wikis — binary + config are
   fully live regardless, only the content sync is paused pending project-editorial's
   confirmation (their session was stale when Command checked). Not our thread to chase
   further unless it stalls.
4. ~~**Hard prerequisite Command must not skip**~~ **CONFIRMED DONE** — see above, verified
   independently via the live `/history/{slug}` 200 check, not just Command's own report.

### Phase 2 — deferred until Phase 1 is handed off: rebuild + re-verify the local os-mediakit appliance

Not a production deployment — this is continuing the existing local-test-loop validation
(the standing plan: validate every `app-mediakit-*` product locally before Phase T2A, per
`BRIEF-os-mediakit-product-family.md`). The previously-built local QEMU test appliance
(`os-mediakit.qcow2`, last rebuilt 2026-08-25) bundles `app-mediakit-knowledge` as its guest
binary — now stale relative to tonight's changes. Phase 2 is: cross-compile the current
binary to `aarch64`, rebuild the guest rootfs, and re-run the same gate-ladder smoke tests
already proven once (G1-G4, `/data` persistence 2-boot proof, G-TLS nginx routing) to confirm
they still pass against the updated binary. Full technical detail and current gate status:
`BRIEF-os-mediakit-product-family.md`'s "Current Status" section — this entry is just the
sequencing note (Phase 2 of tonight's plan, not a new gate).

## STATUS UPDATE (2026-09-06) — History-exposure decision: default-off gate + redaction registry

Command flagged a real structural problem during a corporate-wiki content-correction pass:
`/history/{slug}` (git log, full old-revision content via `?rev=`, and diffs) is public and
unauthenticated, and permanently re-exposes every pre-correction version of an article after
any future edit — a standing engine feature, not a one-off content-cleanup problem. Made
sharper by this engine also being a distributed product (software.pointsav.com BETA download)
— a stranger's own content mount is a pre-existing git repo this route would otherwise
republish in full, unauthenticated, by default.

**Ran a Fable+Opus consult** on two framed options (auth-gate the route vs. a commit-trailer
redaction mechanism). Both independently rejected building a full auth layer — this engine has
zero user/session model, and it's the wrong tool for a rendering-policy problem; a customer who
genuinely wants internal-only history already has a natural path (put the instance behind their
own reverse proxy/SSO). **Opus additionally found a real technical flaw** in the naive
trailer-on-commit design: it can only cover the correcting commit's own diff, leaving every
intermediate revision between the original mistake and the fix still fully exposed via
`?rev=`, and it requires foresight at commit time — but the actual scenario (a correction sweep
discovering a historical problem) happens *after* the fact, which a trailer can't retroactively
cover without rewriting history. Both agreed a real fix needs to be editable *at any time*, not
tied to the commit that introduced or fixed the problem.

**Shipped** (commit `b6646415cd`): `[site] history = "off" | "public"` in `knowledge.toml`,
default off — any value other than the literal `"public"` degrades to off, so a typo or an
absent field never accidentally opens the route. Plus a `redactions.yaml` registry at the
content mount root (editor-maintained, updatable at any time): `slug: { through: <sha>, reason
}` — every revision at or before `through` (ancestor-or-equal, via git2's
`graph_descendant_of`) has its content/diff hidden from `file_at_rev`/`file_diff`, while
`file_history`'s row still lists the date/sha — "hide the content, never the fact a correction
happened," the one principle both models converged on independently.

**Real behavior change for the operator's own three live wikis, not yet actioned**: today they
have no gate at all (effectively "on"). Once this ships, they'll go dark unless
`history = "public"` is added to each of `/etc/local-knowledge/{documentation,projects,
corporate}.toml` — a deployment/ops action on live infra, outside this git repo's reach from
this session. Flagged to Command as a required follow-up before/at deploy time, not done here.

## STATUS UPDATE (2026-09-04) — Footer redesign + article-pager removal

Operator reported two regressions vs. two reference sites (bim.woodfinegroup.com,
design.pointsav.com): the footer looked visually off, and the prev/next "square block"
article pager at the bottom of TOPIC/GUIDE/Index pages didn't fit the site's own
documentation-site conventions. Explicitly asked for clarifying questions before touching
anything (`ask me lots of question we need to get this ocrrect`) — three research agents
(local footer implementation + local pager implementation + live-fetch of both reference
sites) ran first, followed by two full AskUserQuestion rounds before any code changed.

**Root findings, not assumed:** the footer had already been deliberately reconciled once
before (2026-08-25/26, "Command reconciliation") — the operator's earlier complaint about
heading weight was reverted for a documented reason (matching heavy bim-style headings
onto a small brand block flattens an identity→nav hierarchy). Re-litigating that call
needed real cross-model input, not just this session's judgment, so both Fable and Opus
independently proposed full footer redesigns "taking the best of" both reference sites,
verified against live production source (not training-data guesswork — Opus's first pass
cited a stale local project-bim clone that didn't match production and was caught and
corrected). Both converged on the same conclusions independently.

**What shipped** (commit `558dd74330`, landed on canonical via `d889a0b6ff`):
- 4th footer column, "Machine-readable" — `llms.txt`, Atom feed, Sitemap, `/healthz` (the
  site's real machine surface; both models independently found there's no HTTP `/mcp`
  route to link — MCP is stdio-only — so a literal bim/design-style `/mcp` link would 404).
- `.k-footer__col-title` darkened to full-ink color, kept the existing small-tracked-
  uppercase grammar rather than adopting bim's heavy uniform heading style (both models
  converged: bim's own non-footer label headings use the same small/tracked/uppercase
  grammar the site already had — bim's footer is the outlier, not this site).
- `--k-footer-bg` moved from `#eef1f5` toward `#f4f7fb` (near-white, short of bim's literal
  `#ffffff` to preserve separation from the adjacent `.k-compliance` block); the now-
  invisible gradient wash removed rather than re-tuned.
- WCAG 2.5.8 tap-target fix on footer links (`display: block; padding: 0.375rem 0`).
- `k-article-pager` removed entirely — `layout.rs` (markup + 3 params), `app.rs` (prev/
  next computation), `content.css` (~44 lines of rules) — confirmed removed on TOPIC,
  GUIDE, and Index-via-`/wiki/{slug}` pages.

**Verified**: Playwright screenshots of the live footer at multiple breakpoints, light and
dark theme; `cargo test -p app-mediakit-knowledge` 165/165 passing (no regression from the
pre-existing 2026-08-25/26 baseline).

## STATUS UPDATE (2026-07-13) — Full knowledge-platform vision reconciliation + Phase 3 (claim layer) landed

Operator directive this session: pursue the *original* `KNOWLEDGE-PLATFORM-PLAN.md` vision
in full (not the simplified scope the ground-up rewrite settled into), plus a new print-mode
feature, with a full codebase quality gate first. This section reconciles every phase of that
older plan against what the rewrite (P0-P8 above) actually built, and records what landed
this session.

### Constitutional invariants (apply regardless of engine version — restated here since this
BRIEF never asserted them explicitly; carried forward from `BRIEF-knowledge-platform-master.md`
L12-L14)

- **SYS-ADR-07** — no structured data through AI. Hard rule.
- **SYS-ADR-10** — F12 mandatory; only a human operator commits. No AI-authored commit ever
  lands without an explicit human commit action (`commit-as-next.sh`, run by the operator's
  session, never automated).
- **SYS-ADR-19** — no automated AI publishing to verified/public ledgers.

### Federation / MCP disposition — what was NOT carried into the rewrite, and why that's
deliberate, not an oversight

The **old** (pre-rewrite) engine had a 654-line write-capable MCP JSON-RPC 2.0 surface plus
an ActivityPub federation outbox and cross-instance federated search (`BRIEF-knowledge-
platform-master.md` L10, §8.5 Sprint H/M). **None of this was reinstated by the rewrite**,
and this session's Phase 3 work (below) does not reinstate it either — this is a deliberate
scope decision, not a gap:

- The old MCP surface's write-capable tools and the ActivityPub outbox sit in real tension
  with SYS-ADR-07 (no structured data through AI) and SYS-ADR-19 (no automated AI publishing
  to ledgers) — a write-capable MCP tool or an autonomous federation outbox is exactly the
  shape of thing those two rules exist to prevent.
- This session's new MCP server (`src/mcp.rs`, Phase 3.6 below) is **read-only** —
  `query_claims(topic, asof)` only, no write/edit/publish tool of any kind — consistent with
  SYS-ADR-07/19 by construction, not by omission.
- ActivityPub federation and cross-instance search are **not** rebuilt. If they're wanted
  again, that's a fresh decision for the operator to make explicitly against the current
  SYS-ADR set, not an automatic restoration.
- **Provisional, cross-project:** the new MCP server also isn't yet reconciled with
  `service-slm`'s `slm-mcp-server` (`KNOWLEDGE-PLATFORM-PLAN.md` Decision 3) — that
  reconciliation is cross-project and out of this archive's scope; tracked as backlog.

### KNOWLEDGE-PLATFORM-PLAN.md phase reconciliation (verified 2026-07-13 via direct source
read, not assumed from doc claims — see this archive's own copy of the plan at
`.agent/plans/KNOWLEDGE-PLATFORM-PLAN.md`, newly added this session; it previously existed
only in 3 *other* archives' clones)

| Phase | Status |
|---|---|
| 0/1/2 | Done — old-engine concerns (Stage 6, dead-code descope) or already ratified (claim-authoring convention, Doctrine claim #54) |
| **3 — Claim layer (MVL)** | **Landed 2026-07-13, full scope** (commit `0c0f6fcf6`): `Claim` struct + extraction from the ratified convention's `<!--claim …-->` markers (`content::claims`, 11 tests incl. all 4 convention worked-examples verbatim — discharges the convention's own Phase 3.1 Engine Verification Gate); `redb`-backed claim graph + citation-verification history (`claims_store`); `citations.yaml` registry loader (`citations` — this config field existed since P0 but nothing ever consumed it until now); automatic background citation re-verification scheduler (`verification`, 24h sweep, re-fetch+re-hash+drift detection); read-only MCP server (`mcp`, see disposition above); per-article `TechArticle`+`BreadcrumbList` JSON-LD (`jsonld`); `GET /wiki/{slug}` JSON content-negotiation with on-demand-computed backlinks (`app.rs`). 54/54 tests pass, clean clippy, clean build. New deps: `redb`, `blake3`, `reqwest`, `rmcp`, `schemars`. |
| **4 — DTCG token wiring** | **Landed later the same day** (commit `c289ecabdc`): `build.rs` generates `static/tokens.css` from `tokens/design-tokens.json` (DTCG format) + `tokens/theme-overrides.css`, verified byte-equivalent to the prior hand-authored file across all 120 vars; `wcag_contrast.rs` gates 4.5:1 AA on every shipped text/surface pair. This row previously said "Not started this session" — true when written, stale from later that same day onward; corrected 2026-09-04 after a cross-model audit flagged the contradiction with README's summary (which was already correct) |
| 5 — Bilingual `/es/` | Routing/switcher (5.1) was **not** fully shipped as this line previously claimed — `/es/category/*` 404d across all 14 categories (route never registered; `/wiki` had an `/es` twin, `/category` never did). Fixed 2026-08-23 (commit `f3eaf9238`): `category_page`/`render_index_topic_category` are now `Lang`-parameterized like `serve_article` already was. Category *content* stays English-canonical by design (`ContentIndex` is English-only), matching `/es/wiki`'s existing shallow-parity depth — true `_index.es.md` Index Topic content is a separate, larger follow-up, not covered by this fix. DYK localization (5.2) + `hreflang` tags remain open (NEXT.md backlog) |
| 6 — Three-instance split | Confirmed fully done this session via direct inspection (3 `Tenant` variants, 3 systemd services, 3 domains, correct PointSav/Woodfine ownership split) |
| **7 — Contribution model** | `os-console`/`os-mediakit` (the planned pairing-broker + contributor client) are different crates outside this archive's cluster-manifest scope — not buildable here. This archive's deliverable is the integration-contract spec below, not code in those crates |
| **8 — Editorial linter** | **Landed later the same day** (commit `c289ecabdc`): `editorial.rs` adds a provisional Bloomberg-standard + BCSC ruleset exposed via a new `validate_editorial_standards` MCP tool. Same staleness/correction note as Phase 4 above — corrected 2026-09-04 |
| **9 — Print mode (new)** | Landed 2026-07-13 — see `BRIEF-print-mode.md` (child of this brief) for the research-backed design (pure CSS, no Paged.js — verified against Wikipedia's actual approach) and what shipped |

### Phase 7 integration contract (for whoever builds `os-mediakit`/`os-console`)

Since this archive cannot build those crates, this is the target contract the engine already
satisfies today and expects the future broker to drive:

- **Pairing-token verification hook**: the engine has no edit surface at all right now (no
  `/edit` route, no `auth.rs`/`users.rs`/`pending.rs`) — a from-scratch simplification, not a
  removal. When `os-mediakit` exists, the expected integration point is a new route guarded by
  a pairing-token bearer check (verify against `os-mediakit`'s issued token, not a local
  accounts table — Decision 2's sequencing: no local auth/pending tables, ever, per the old
  plan's explicit gate).
- **Capability classes**: two, matching Vision §5 — draft-pairings (propose an edit, lands as
  a PR-equivalent / review queue entry, never direct-committed) and human-only promotion
  capability (the F12 gate above — only a human operator's own session can turn a draft into
  a real commit).
- **F12 enforcement point**: the commit boundary itself (`commit-as-next.sh`), same as every
  other artifact in this workspace — no new enforcement mechanism needed, the broker's job is
  only to gate what reaches that boundary as a proposed diff, never to commit directly.

The "possible reversal"/"NEXT UP = P7" framing in this BRIEF's body (dated 2026-07-02/07-03)
was stale. Verified 2026-07-09 via direct git/systemd/binary-ledger inspection:

- **P7 (shadow deploy) and P8 (cutover: rename `-2` away, retire old crate) both actually
  completed on 2026-07-02**, same day as the "NEXT UP = P7" note below was written — the note
  just was never updated afterward. Canonical commits `531d3144`/`25e4bf99` (P8 cutover,
  "42 commits, P0-P6, already promoted to canonical as app-mediakit-knowledge-2").
- **The rewrite is live in production right now**: `app-mediakit-knowledge` (binary sha
  `1ad9946f8ed6...`, source_commit `45039f1f`) is the actual running binary behind all 3
  systemd services (`local-knowledge-documentation/-projects/-corporate`), confirmed
  `active (running)` and sha-matched 2026-07-09.
- **The 2026-07-03 "reversal"/v1-vs-v2 carry-forward note below is moot** — v2 already is
  the deployed engine; there was no actual reversal, just an unresolved question that
  overtook itself.
- **The project-software binary handoff this BRIEF's own P8 line describes is already
  in flight, separately from this archive**: Command sent project-software the exact sha
  (`1ad9946f...`), license tier, and binary-ledger details on 2026-07-08 (msg-id
  `command-20260708-2-new-catalog-entries-requested-orchestr`, requesting a new $0-BETA
  software.pointsav.com catalog listing). As of 2026-07-09 that message is still `pending`
  in project-software's inbox — action needed on **their** side, not this archive's.
- **This archive's local `pointsav-monorepo` sub-clone does NOT contain any of this work.**
  Its `app-mediakit-knowledge/` history is a wholly separate, older "iteration-2"/"Wave 1-5"
  lineage (48 commits, unrelated incremental patches to the pre-rewrite engine) that never
  merged with the P0-P8 rewrite lineage at all (`git merge-base --is-ancestor` confirms
  neither P0 nor P1 of the rewrite is in this sub-clone's history). The real rewrite commits
  were made and promoted from a different checkout/session context this archive's
  session-context.md never captured. No git reconciliation was attempted this session
  (nothing to build here — the real deliverable already shipped elsewhere); if this
  sub-clone is ever used for further app-mediakit-knowledge work, treat it as stale and
  reset it against canonical first, not the reverse.
- **Do not re-run P0-P8, do not rebuild a binary, do not re-send a project-software handoff**
  — all three would duplicate work that is already done and already in flight.

---

## STATUS UPDATE (2026-07-12) — P9: audit-driven polish pass, 8 commits landed

A 425-agent audit (research → parallel finders across accessibility/performance/SEO/
security/content-UX × 3 sites + engine code quality + prod-sync investigation →
adversarial verification → Fable synthesis) produced an 11-item ranked roadmap. The
project-knowledge-owned items (engine code, not content/infrastructure/design-system)
are implemented, tested, and committed on this branch — sub-clone was resynced to
canonical first (see 2026-07-09 status above), each item its own commit:

1. `88d2e78b` — **legal-tokens runtime consumer** (`src/legal.rs`, new). Engine now loads
   `factory-release-engineering/tokens/legal-tokens-{brand}.yaml` at startup instead of
   hardcoding the footer's copyright/trademark text. **Known accepted regression
   window**: the canonical token files are currently stale (pre-dates the ratified
   2026-07-07 MCorp™ rename, commit `062b29e`) — our footer will show the stale
   "Woodfine Management Corp™" text until Command lands the already-drafted
   `LEGAL-RECONCILIATION-token-source-of-truth.draft.md` fix, at which point the engine
   self-corrects automatically (no further code change). Flagged to Command as a
   priority ask (msg `command-20260712-3-items-from-the-knowledge-platform-audi`).
2. `8f220405` — items 2-6: privacy footer link fix (`/wiki/privacy` → `/wiki/page-privacy`),
   HTML-comment stripping at parse time (fixes a real search-snippet leak of internal
   file/function names), metadata pack (real `rel=canonical`/`og:url`/`og:image`, home
   page always has a description, "the The" category-description bug), compliance panel
   heading demoted h1→h2 (was the single most-reported audit finding — duplicate H1 on
   every page of all 3 sites), URL/routing hygiene (trailing-slash 301s, `/favicon.ico`,
   styled 404 fallback, `/wiki/index` deduped against `/`).
3. `2950bbe5` — item 7 (scoped): static-asset `Cache-Control`+`ETag`+304 support,
   `category_counts` memoized once instead of computed twice per home-page request,
   `mounts.first()` → `mounts.primary()` correctness fix. Deferred (noted, not dropped):
   git-history SHA-lookup caching (needs a real invalidation design — this is a
   long-running process serving a git-synced tree that changes without a restart) and
   the optional ContentIndex/SearchIndex parse-sharing + render-boilerplate extraction.
4. `52dcc994` — item 8 (scoped): visible breadcrumb (`ui::breadcrumb`) on article +
   category pages, `hreflang` pairs on translated articles. Deferred: Article/
   CollectionPage JSON-LD schema — explicitly the audit's own lowest-priority item.

**32/32 tests pass** (12 new across the 4 commits). Pushed to `origin-staging-j` +
`origin-staging-p`; **Stage 6 promote flagged to Command**, not yet landed on canonical.

**Not our scope, routed via mailbox to the owning archive:**
- Command: prod-sync automation (public sites had 0 confirmed pushes since 2026-07-03
  despite 67+ content commits — the single most material finding in the whole audit),
  nginx security-header hardening, the MCorp™ cross-check + LEGAL-RECONCILIATION
  priority ask.
- project-editorial: Spanish translations for 4 core documentation pages currently
  silently serving English under `lang="es"`, the superseded `about-regional-markets`
  article still live/unmarked/linked-as-current on the investor-facing projects site, 5
  stale `topic-*` slugs on corporate's investor-access page, our compliance-band text
  requirement (input to their already-in-progress disclaimer-library design, not a
  competing proposal), and the 34-content-file embedded-footer-text finding (evidence
  for their planned future content-stripping pass, not fixed by us — some of those
  files, like `TRADEMARK.md`/`disclaimers.md`, legitimately need that text as subject
  matter, not incidental duplication).
- project-design: `og:image` brand asset files (engine now emits the tag, pointing at a
  path they need to supply the actual image for).

**Full audit dossier** (all 124 verified findings + the Fable-synthesized roadmap):
published as an Artifact this session — see session transcript; not duplicated into this
BRIEF verbatim given its length.

---

## STATUS UPDATE (2026-07-12) — P9 follow-up: structured-data SEO items, 1 commit landed

P9 item 4 (`52dcc994`) deliberately deferred Article/CollectionPage JSON-LD as the audit's
own lowest-priority item. Separately, project-editorial's `BRIEF-seo-cross-site-strategy.md`
independently drafted 7 SEO changes across the 3 sites; cross-checked line-by-line against
P9's actual shipped diffs — 4 of 7 were already covered by `8f220405`/`2950bbe5`/`52dcc994`
(canonical URLs, hreflang, og:image/og:url, breadcrumb nav). Replied to project-editorial
(msg `command-20260712-re-seo-drafts-4-of-7-items-already-shipp`) identifying the 3
genuinely-new items and implementing them as a direct follow-up rather than leaving them to
drift into a future session:

- `6dcc4c44` — `Tenant::organization_id()` (each property's JSON-LD `publisher`/`author`
  now references the brand's apex-domain Organization node by `@id` instead of an inline
  copy, matching the `is_woodfine()` split); WebSite JSON-LD restructured to the `@id` form
  + `SearchAction` added; new `article_jsonld()` (TechArticle, `dateModified`, author `@id`,
  `isPartOf`) on article pages; new `breadcrumb_jsonld()` (BreadcrumbList) on both article
  and category pages; shared `jsonld()` serialization helper neutralizes `</script>`
  breakout in any embedded string field (new test `jsonld_neutralizes_script_breakout`).

**36/36 tests pass** (4 new). Smoke-tested against a live instance on a spare port —
WebSite/TechArticle/BreadcrumbList blocks confirmed valid JSON via `python3 -m json.tool`,
`@id` wiring correct, visible breadcrumb renders alongside the structured data. Pushed to
`origin-staging-j` + `origin-staging-p`; **Stage 6 promote flagged to Command** (adds to the
P9 batch already pending canonical merge — not yet landed).

**Open question sent to project-editorial, not yet answered**: whether the apex/newsroom
domain actually resolves `https://pointsav.com/#organization` /
`https://woodfinegroup.com/#organization` as live Organization nodes yet, or whether that
piece isn't live. Doesn't block this commit — the `@id` reference is valid regardless of
whether the target node is resolvable elsewhere — but worth checking their reply once it
lands.

---

# Knowledge wiki engine — 100% ground-up rewrite

Plan of record: `~/.claude/plans/virtual-twirling-parasol.md` (approved 2026-07-01).

## Context

Every prior redesign attempt (Loop Ivory Ledger `598cf7d0`, wiki-* chrome pass
`c49a9051`, `db0169b5`) **reused the existing code and only renamed classes / swapped
colors on the same DOM.** Operator verdict this session: "this is still not a 100%
re-write as a whole new website", "even the chrome layer has not been a 100% re-write",
"all what we have now should tell you is what NOT to do."

Decision (operator-confirmed): rewrite the **entire crate** from blank files — engine,
routing, rendering, chrome, design system, JS, tests. Nothing we wrote is reused; the old
crate is a read-only contract reference and anti-pattern catalogue. Built as a parallel
crate `app-mediakit-knowledge-2`; swapped at parity; then the `-2` is renamed away and the
old crate retired (final state = one 100%-new `app-mediakit-knowledge`).

## Scope & decisions locked

- Reuse allowed: Cargo **dependencies only** (axum, maud, comrak, tantivy, git2, gix, redb…).
  No code we authored. "No old code" = none of the ~23k lines we wrote; the third-party
  libraries are not rewritten.
- Design: Wikipedia Vector 2022 **structure** (sitenotice → white header → article tabs
  above `<h1>` → 2-col sidebar+content → institutional footer). New token namespace, new
  class namespace, new DOM, new CSS/JS. Brand = accent only (PointSav `#1a4480`, Woodfine
  `#164679`), never a chrome background.
- Execution: Opus swarm (Workflow) generates new code per phase.
- Contract preserved: 40+ routes, the `knowledge.toml` schema (so the 3 live instances swap
  with zero config change), and the canonical markdown content (untouched). The pre-existing
  `substrate_category_buckets` test failure is fixed in the new bucketing logic.

## Magnitude (measured 2026-07-01)

Old crate = 16,719 Rust LOC (43 files) + 4,479 CSS + 1,786 JS + 89 tests (17 files).
Multi-session program.

## Phase program

- **P0 — Scaffold ✅ (2026-07-01).** New crate builds green; `config.rs` (schema preserved,
  2 tests); `/healthz` + `/static`; all 3 production `knowledge.toml` load; workspace member;
  registry row added.
- **P1 — Content pipeline.** mount / frontmatter / walk / comrak render; serve raw article.
- **P2 — Design system (Opus swarm).** New tokens.css, app.css, app.js, ui/layout shell.
- **P3 — Core pages ✅ (complete 2026-07-02).** Shipped: article body design system
  (ruled title, serif-heading/sans-body, wikitables, syntax-highlighted code light/dark +
  copy button, inline code, external-link marks, section-anchor permalinks); Main Page
  (Browse-by-area category grid + How-to guides section with descriptions); /category/{name}
  listing pages (title + description index); site pages (about/disclaimers/privacy/contact
  as internal topics); **article action tabs above `<h1>` (Article active; Notes/History
  disabled placeholders) + "Last updated" line + left 2-column sidebar (Vector 2022 shell)
  shipped 2026-07-02.** In-article **Table of Contents** (sidebar "Contents", h2/h3, anchors
  matching comrak ids) + **tantivy full-text search** (`/search`, in-RAM index, header box
  wired) shipped 2026-07-02. **P3 complete.**
- **P4 — Versioning + discovery ✅ COMPLETE (2026-07-02).** git article **history** (History
  tab) + **diff view** + **as-of point-in-time view** + **provenance line**; **discovery set**
  — robots.txt, sitemap.xml (lastmod), Atom feed, llms.txt, **JSON-LD** (schema.org WebSite/
  Organization; feed.json cut). **blame, git smart-HTTP, and any clone/distribution capability
  are CUT — NOT planned work** (operator-deferred 2026-07-02; revisit git smart-HTTP only if
  GitHub-independent public distribution ever becomes a hard requirement). Do not re-list them
  as "remaining."
- **P5 — Integrations (RE-SCOPED 2026-07-02 per product-fit review).** ✂️ **STRUCK: in-browser
  edit + CodeMirror, WS collab, Doorman AI endpoints, MCP authoring, glossary, redb link-graph
  — the *encyclopedia-community* metaphor. This engine is a read-only rendering/verification
  agent; a browser write path destroys the signed-commit provenance that IS the product, and
  agents author via Git like humans.** KEPT: citations (the JOURNAL resolver, P5b), jsonld
  (done 2026-07-02), atom (done). A reviewer **annotation** channel (Notes/Talk) is **CUT —
  not planned work**; if ever revisited it would be a separate audit-comment proposal, never a
  Wikipedia Talk page.
- **P5b — JOURNAL render section (new, operator-directed 2026-07-01).** Render the ~10
  `foundry-journal-v1` academic papers as a first-class **research category** — but per the
  Fable feasibility study, as **landing pages on a `/research/` namespace** (masthead +
  abstract + engine-GENERATED references + notice banners + read/download links), with the
  full ~22-section body as a **separate full-text HTML rendition (+ PDF deferred)** — NOT
  interleaved into the `/wiki/{slug}` article route (that form breaks citations + violates
  IA/field norms; declined). Net-new: journal detection + extended frontmatter (authors, doi,
  version, cite_as), a **bracket-ID citation resolver + References generator**, a notice-block
  component, a landing template, a full-text route + academic/print CSS, and the `/research/`
  namespace. Reuses comrak/CSS/heading-extraction/category machinery/tenant chrome. **Depends
  on P3 TOC+sidebar** (papers are the heaviest TOC consumers) → lands after P3 core. **Gated:**
  bodies go live only once the render section is confirmed complete (editorial's readiness
  gate).

  **STATUS UPDATE (2026-07-11) — editorial's gate is essentially CLEAR; "9/10 lack
  slug/category" below is STALE, corrected via direct ledger recovery of 2 messages that
  never reached this archive's inbox (mailbox bugs reported to Command 2026-07-11).** All
  10 papers were backward-fixed to the render-contract shape as of 2026-07-09/10:
  bracket-ID citations resolved against `citations.yaml`, `slug:`/`category:`/`abstract:`
  frontmatter, clean body structure, no body `## Abstract`. Full per-paper disposition in
  project-editorial's `.agent/rules/journal-registry.md`. Remaining real blockers, none of
  them ours or editorial's to close further: `sel4-systems` citation (capability-geometry)
  unregistered; 5 of 10 papers missing bibliographic sources entirely (content-authorship
  gap); **NOTICE-TEXT (below) is genuinely still open — on Command's plate.** Editorial
  drafted the canonical notice-text data (`notice-text-journal.yaml`, 4 templates) and
  routed it to Command 2026-07-10 for a file-placement decision — once Command places it,
  **we** wire it into the golden-fixture suite + render engine (commitment made in the
  original 2026-07-02 thread).

  **Correction to the "port the old engine's `citations.rs`" line above — confirmed wrong
  by direct code read 2026-07-11.** That file was retired at the P8 cutover (`531d3144`)
  and doesn't exist in the live crate. Its fuller predecessor (found in project-editorial's
  and project-jennifer's `app-mediakit-knowledge` clones) only resolves a **claim-annotation's**
  `cites` list (`<!--claim id=c cites=[a,b]-->`) against `citations.yaml` — no bracket-ID
  prose parser, no code-fence-awareness, no first-appearance numbering, no References
  generator. Reusable: the registry-loading infra only (`CitationRegistry`,
  `load_registry()`). The actual §1.4 resolver/generator logic is new work, not a port.
  Also confirmed: no golden-fixture test idiom exists anywhere in this crate today (all
  tests are inline `#[cfg(test)]` literal fixtures) — the SPEC's golden-fixture suite
  (§0.5/§7.11) is new infrastructure for this codebase, budget real time for it.

  **New dependency surfaced 2026-07-11, not yet resolved:** the 10 backward-fixed papers
  live in project-editorial's own working tree
  (`clones/project-editorial/JOURNAL/JOURNAL-<slug>.md`), not yet published into whatever
  content-repo location the engine actually serves `/research/` from. The copies in
  `vendor/pointsav-monorepo/JOURNAL/` and this archive's own `JOURNAL/` are confirmed
  **stale** (pre-backward-fix) — do not build against those. Whose job the publish step is
  (editorial's or Command's) is unclear — raised with Command, not assumed.

  **CORRECTION 2026-09-04 — all 6 phases below actually completed 2026-07-14, three days
  after this section was written, and never recorded here until a cross-model audit
  (Fable + Opus) surfaced the gap.** Both models' fresh survey of this archive incorrectly
  reported P5b as still-open/not-started — same mistake this BRIEF's own text below made,
  because nobody updated this section after the work landed. Verified directly, not taken
  on report: `git log --oneline --grep="P5b"` shows all 6 phases committed 2026-07-14
  (`2f60f7be02` Phase 1 → ... → phase-6 commits explicitly tagged "final P5b phase");
  `cargo test -p app-mediakit-knowledge --lib journal` (4/4 pass) and
  `cargo test -p app-mediakit-knowledge --test journal_golden` (1/1 pass) both green today.
  **Phase breakdown for our own engine-side work (mirrors SPEC §1-§10):** (1) frontmatter
  struct extension — **DONE**, `content/frontmatter.rs` has the full JOURNAL field set
  (abstract, state, version, authors, license, cite_as, preprint_posted_date,
  forbidden_terms_cleared, keywords, subject_codes, paper_class, doi); (2) citation
  resolver + References generator — **DONE**, `citations.rs`'s `bibliography_line()` +
  `check_citation_gate()`, consumed by `content::render::render_journal_doc`; (3) masthead
  + banner components — **DONE, 2026-09-05.** Command placed `notice-text-journal.yaml`
  at `vendor/factory-release-engineering/tokens/` (`factory-release-engineering` commit
  `2ab879c`); wired same day (`app-mediakit-knowledge` commit `ab33098503`) — new
  `notice_text.rs` module (mirrors `legal.rs`'s load/fallback pattern), real
  `notice_banner()` implementation (working-paper notice + FLS advisory for
  draft/under-review, citation banner for published), `AppState.notice_text` loaded at
  startup, wired into both `research_landing`/`research_fulltext`. `archived`/
  superseded-notice remains a **documented, deliberate gap** — its template needs a
  `revision_history.latest.*` frontmatter field this crate doesn't have yet, matching the
  notice-text file's own "not yet needed" comment (no paper has reached `archived`).
  Verified: 173/173 tests pass (4 new/updated notice_banner unit tests), clippy clean,
  fmt clean; (4) `/research/{slug}` landing + full-text routes — **DONE**, `app.rs` has
  both `/research` (index) and `/research/{slug}` (landing) handlers wired to
  `render_journal_doc`; (5) golden-fixture suite — **DONE**, `tests/journal_golden.rs` +
  `tests/fixtures/`, a genuinely new test pattern for this crate, passing; (6) geospatial
  class — **DONE**, `paper_class: geospatial` implemented, explicitly tagged "final P5b
  phase" in commit history.

  **P5b is now fully complete on the engine side — all 6 phases, no remaining gaps in
  this crate.** One thing outside this crate's scope, confirmed still genuinely open
  2026-09-04: whether any real paper content has actually been published into the content
  repos this engine serves from — this crate has no bundled `content/` dir of its own
  (content is external, per DOCTRINE.md §IV.e), so this session could not directly confirm
  papers are live. Command relayed the "whose job is the publish step" question to
  project-editorial directly (2026-09-05) — that pipeline, not this engine, is what
  determines whether JOURNAL content actually appears in production.

  **CRITICAL CORRECTION, 2026-09-05, same day — the answer above is wrong, and this
  engine's `/research/{slug}` machinery will likely never receive real content for any of
  its three tenants.** Operator asked directly ("there should be no JOURNALS in any of the
  three wikis???"); checked `project-editorial/.agent/briefs/BRIEF-journal-research-
  programme.md` (status: active, updated 2026-07-16, ratified same day per msg-id
  `command-20260716-re-ratification-request-retire-research-`): **"The three media-
  knowledge wikis never host JOURNALs at all."** Verified directly against the actual
  repos, not just the BRIEF text — `research/` category and directory are genuinely absent
  from all three `media-knowledge-*`'s current `categories.yaml`/content trees. JOURNALs
  are split across 6 *other* sites instead: `home.woodfinegroup.com`, `home.pointsav.com`,
  `bim.woodfinegroup.com`, `design.pointsav.com`, `gis.woodfinegroup.com` (the one live in
  production today, own Python renderer), `software.pointsav.com`. `documentation.pointsav.
  com`/`projects.woodfinegroup.com`/`corporate.woodfinegroup.com` — this crate's three
  tenants — are explicitly not on that list.

  This means: Command's "paper-publish-location" answer earlier today (pointing at
  `media-knowledge-documentation` as the real publish target) directly contradicted this
  already-ratified 2026-07-16 decision — flagged back to Command as urgent
  (`command-20260905-urgent-correction-your-paper-publish-loc`) and to project-editorial
  directly in case Command's relay reaches them first
  (`command-20260905-heads-up-command-s-relay-to-you-about-jo`). Root cause: P5b's engine
  build (2026-07-11/14) predates the 2026-07-16 retirement ratification by days, and
  nothing since cross-referenced the two — not the original build, not this session's own
  "P5b fully complete" correction earlier today, not Command's answer this morning. The
  operator's direct question is what actually caught it.

  **Consequence for this crate**: the `/research`, `/research/{slug}`, `render_journal_doc`,
  `notice_banner`, golden-fixture suite, etc. are real, tested, working code (173/173
  tests pass) — not wasted, per the operator — but they will never serve real content for
  `documentation`/`projects`/`corporate` specifically, since JOURNAL papers are never meant
  to land in those content repos. Not marking this dead code outright — worth a future
  decision (Command/operator call) on whether to formally retire the routes from this
  crate to match the 3-wikis-never-host-JOURNALs decision, or keep them as generically
  reusable reference code for a hypothetical future 7th JOURNAL-hosting site. Not decided
  this session.

  **Sub-clone note:** this archive's local `pointsav-monorepo` sub-clone was resynced to
  canonical 2026-07-11 (see BRIEF status-update block above, "reset it against canonical
  first") specifically to do this Phase 1 work — the old stale-lineage warning above no
  longer applies as of that reset.
- **P6 — Test suite.** Rewrite all 89 tests for new DOM; green.
- **P7 — Parity + shadow deploy** (9091/9094/9096); operator visual sign-off.
- **P8 — Swap + rename `-2` away + retire old crate** (deploy = Command scope, via outbox).

## Work log

- 2026-07-01 — P0 complete. Crate `app-mediakit-knowledge-2` scaffolded: `Cargo.toml`
  (lean forward dep set), `src/{main,app,config,error,assets,lib}.rs`, placeholder
  `static/{app.css,app.js}`, bilingual READMEs. `cargo build -p app-mediakit-knowledge-2`
  green; `cargo test` 2/2; `check` loads documentation/projects/corporate configs; serve
  binds and `/healthz`→200, `/static/`→served. Workspace `Cargo.toml` member + release
  profile override added. Monorepo registry row added.

- 2026-07-01 — P1 complete. content/ module: frontmatter parse (foundry-doc-v1),
  comrak render + `[[slug|label]]` wikilink resolution + h2/h3 TOC extraction, MountSet,
  content walk with (slug,lang) index + bilingual `.es.md` pairing. Verified live against
  the real corpus: 297 articles indexed, `/wiki/{slug}` renders, wikilinks resolve. 13 tests.
- 2026-07-01 — P2 complete. Design system via Opus swarm (recon Wikipedia Vector 2022 +
  both marketing sites → 3 impls → 3 cross-checks → judge; winner "Slate"+grafts). New
  `--k-*` tokens + `k-*` classes (zero reuse). static/{tokens.css, app.css, app.js,
  fonts.css} + self-hosted fonts. src/ui/{tenant.rs (real legal strings), layout.rs (maud
  shell: sitenotice/white-header/mobile-nav/footer/page)}. Article route renders inside the
  chrome. Verified on shadow 9091: white-header Wikipedia chrome, all assets 200, zero
  banned names, correct legal strings. Design law held: white header, light footer, brand
  accent-only. Commits on cluster main: 7167d441 (P0), 995f88f7 (P1), c68427a7 (P2).

- 2026-07-01 — P3 (mostly) + long operator refinement loop. Shadow moved to :9090 (the
  documentation instance port, swapping the old workspace service) for review; late in the
  session moved off the build cache to a stable runtime path (see below). Commits (all on
  cluster main, chronological):
  - `f82e2caf` mirror-marketing chrome + article-body design system (content.css: prose,
    wikitables, code panels; app.js copy button; article title/prose).
  - `2ee875c3` chrome refinements — wordmark lockup, nav IA, footer badges (MediaKit + CC).
  - `39e2bdf6` Main Page structure (Browse-by-area grid) + /category/{name} listing pages.
  - `8ed24b05` server-side syntax highlighting (comrak+syntect) + prose fixes + CC BY 4.0 badge.
  - `15527a03` Wikipedia font pairing — serif headings (Source Serif 4) + sans body (Inter).
  - `6ede6e0f` dark-mode code blocks (class-based dual-theme /static/syntax.css) + full-width.
  - `1d695079`/`59a6d3bc`/`1d709e6d`/`c2a0a809`/`fdc3c97f` — footer + top-strip IA per operator:
    entity left → corporate home; top-right = GitHub/Software/Design System; footer Network =
    own home → properties → Woodfine (cross-company) last; cities-above-copyright, badges right.
  - `7634f06e` **review pass** (fresh-eyes agent, browser-in-the-loop vs Wikipedia). Big find:
    a frontmatter `type`/`content_type` serde alias collision was failing the WHOLE parse on
    any file carrying both keys → titles/categories silently dropped site-wide. Fixed → index
    grew 298→302 and category coverage 11→264 across all 10 areas; titles restored. Same commit:
    home title no longer double-brands, pluralization, humanized slug fallback, inline code
    toned off brick-red, favicon + meta description + Open Graph in `<head>`, external-link ↗
    marks, section-anchor `#` permalinks, Playfair removed.
  - `44bf42e0`/`c1652068`/`28b3504a` How-to guides surfaced on the Main Page (Diátaxis split:
    reference topics vs how-to guides) with one-line descriptions (short_description, else a
    body-paragraph summary added to DocRef); descriptions extended to /category/{name} pages.
    `_index.md` section-landing files excluded from the index (302→297).
  - `cd0fc296` deferred the standalone "Last updated" line to the P3 heading-nav step (operator).
- 2026-07-02 — P3 tabs + sidebar (Vector 2022 shell). `page()` gained a nav-category list;
  `nav_cats()` feeds it from all handlers. Left 2-col sidebar (Main page · Browse-by-area ·
  Guides), sticky, full-width content beside it. Article action tabs above `<h1>` (Article
  active; Notes/History disabled placeholders — wire at P5/P4); "Last updated" line from
  `last_edited` (`format_date`). Sidebar breakpoint aligned to the drawer (≤768px) to close a
  769–1024px no-nav gap. **Fable browser audit (Playwright vs Vector 2022)** then found +
  fixed: (a) **font-binary defect — Inter 500/600 + Source Serif 4 700 woff2 were copies of
  Regular**; replaced with genuine subsets (weights verified on disk + served) → real
  bold/semibold site-wide; (b) sidebar 15rem→**12rem** (`--k-sidebar-width` defined) + gap
  48px→24px (dead space); (c) sidebar headings → muted uppercase caps (inverted-hierarchy fix).
  13 tests green throughout.
- 2026-07-02 — **P3 finished.** In-article **TOC** in the sticky sidebar ("Contents", h2/h3
  from `rendered.headings`, anchors matching comrak ids; `page()` gained `toc: &[Heading]`).
  **Tantivy full-text search** — new `src/search.rs`: in-RAM tantivy 0.24 index built at startup
  (title+body over all English docs), `SearchIndex` on `AppState`; `/search?q=` handler enriches
  ranked slugs with title+description from `ContentIndex` (result cards match listings); results
  page reuses `.k-cat-list`; empty/no-match handled; the existing header search box now works.
  Verified on :9090 (merkle/doorman/multi-term/empty/nonsense); 13 tests. **P3 complete.**
- 2026-07-02 — post-P3 polish (operator + agents in the loop). **Search de-duplicated to ONE
  bar** (Opus browser-in-the-loop): the header search box now echoes the query
  (`value=(query)` threaded `page()→header()/mobile_nav()→search_block()`, one param like
  cats/toc); the redundant on-page `/search` form deleted; results sit under the "Search"
  heading. Earlier a Fable pass had differentiated the two boxes, but the operator was right
  that one is cleaner. **Canonical trademark footer** — added the verbatim `TRADEMARK.md`
  notice (Woodfine Capital Projects™, MCorp™, PointSav Digital Systems™, Totebox
  Orchestration™, Totebox Archive™, Capability Geometry™) as a fine-print footer row; **no
  "all rights reserved"** (content is CC BY 4.0; marks reserved independently). Used canonical
  marks, NOT the drifted home.* wording ("Woodfine Management Corp™", missing Capability
  Geometry™) — drift + the © 2026 vs 2011–2026 year discrepancy flagged to project-marketing
  (`command-20260702-home-footer-trademark-text-drifted-from-`). `TRADEMARK.md` unchanged.
- 2026-07-01 — Sent content request to project-editorial (msg-id
  command-20260701-content-request-short-description-on-all): add `short_description:` to all
  GUIDEs (27/28 lack it; engine falls back to first body paragraph meanwhile) and ideally all
  TOPICs, for consistent card/listing/meta-description summaries.
- 2026-07-01 — Actioned Command's deploy-path flag: preview server moved off
  `cargo-target/mathew/debug` to a stable runtime path
  `/srv/foundry/infrastructure/local-knowledge/bin/app-mediakit-knowledge-2`; replied to
  Command that the ~23G debug cache can be cleared freely. Iterate loop now: rebuild → cp
  binary to stable path → relaunch (CSS/JS live from the crate `static/` dir).

**Crate size now:** ~1,864 Rust LOC + ~1,437 CSS + app.js, 37 files. 13 tests green throughout.

## Product identity + regulated-reporting decisions (2026-07-02, two Fable studies + EDGAR/SEDAR research)

**Identity:** a **read-only rendering + verification agent over a Git-canonical record** for
regulated reporting (BCSC). Content authored only via signed Git commits by professionals;
reviewers (auditors/directors/bankers/regulators) read + direct edits. The engine never writes.

**Decisions shipped:**
- **Provenance = entity, not person.** Per EDGAR/SEDAR (signatories named; preparers/editors
  never; no per-editor trail): History/Diff render `date · hash · message · <issuer>`, no
  natural-person name. `Tenant::issuer()` (PointSav Digital Systems / Woodfine Capital Projects
  Inc.). Editor identity stays in signed commits for audit. **PointSav treated as its own
  public company.**
- **Per-tenant CC licence** (two standard licences only, no bespoke): documentation → **CC BY
  4.0**; projects/corporate → **CC BY-ND 4.0** (verbatim, no altered copies). Real official CC
  marks self-hosted, badge+name+deed-link (CC's recommended marking). `Tenant::license_*()`.
- **Lean cuts** (Wikipedia cargo-cult removed): Notes/Talk tab, /random, /special/categories,
  feed.json. `#`→`§`. Built the two earned `/special` pages (Index of record, Recent changes).
  (NB: the `/es` language route was **built** 2026-07-02 — bilingual content is reachable via
  `/es/wiki/{slug}` + a toggle; it is no longer a cut. Chrome + disclaimer legal text stay
  English — chrome/legal localization is a separate counsel-gated follow-up.)

**⚠️ For counsel (surfaced, not decided):** final Woodfine CC-BY-ND vs the excerpt question;
whether commit **messages** (now public on record tenants) need a disclosure-review gate;
third-party materials; PI/consent; confirm the site isn't positioned as a filing/SEDAR+
substitute.
**✅ Counsel OK'd (2026-07-02):** the Important Information band **default draft text** is
approved — the band's fallback copy is validated. Editorial may still author per-tenant
`important-information.md` (NI 45-106 for Woodfine; IP/no-warranty for PointSav) to refine
it; the engine picks it up automatically when it lands.

**Shipped 2026-07-02:** the **"Important Information" disclaimer band** — native `<details>`
above the footer on every page, content sourced from `important-information.md` in the content
repo (else a safe issuer-aware default), forced-open in print, + a persistent footer one-liner
+ link to `/wiki/disclaimers`. Editorial tasked (with counsel) to author the per-tenant
`important-information.md` + beef up `/wiki/disclaimers` (msg
command-20260702-author-important-information-md-per-wiki).

**Shipped 2026-07-02 (completes the review, engine side):** the **point-in-time as-of view**
(`/wiki/{slug}?rev=` renders the git blob with a "Historical revision" banner + current-record
link; diff→as-of wired) and the on-page **provenance line** (`Last updated <date> · <sha>`).
**Nothing engine-side remains from the review** — only the counsel items (above) and editorial
authoring `important-information.md` + the disclaimer long-form.

## Carry-forward

- **SUPERSEDED 2026-07-09 — see "STATUS UPDATE" at top of file.** The note below was based
  on an unresolved question that turned out to be moot; P7/P8 had already completed the same
  day this note was written, it just was never checked before the note was added.
- **2026-07-03 note [totebox@claude-code]: possible reversal of the P7→P8 direction below —
  do not assume v2 is still the intended live engine.** Per the operator this session,
  project-knowledge has already asked Command to switch production back from
  `app-mediakit-knowledge-2` to `app-mediakit-knowledge` (v1) — the opposite of the
  P7/P8 plan recorded below (stand up all 3 shadows → rename `-2` → retire v1). Also
  unresolved: what's actually deployed on foundry-prod for the 3 public domains right now
  is unconfirmed from this session (see `feedback-prod-topology.md` — do not infer from
  foundry-workspace's local ports/nginx). **Confirm current direction with the operator or
  Command before resuming P3/P7/P8 work as if v2-forward is still the plan.**
- **2026-07-02 backlog sweep (done):** categories.yaml nav + alias/redirect (editorial Phase C
  gate); references→footnotes (43 articles); as-of view + provenance line; browser-tab titles +
  chrome 404; **test coverage 13→19** (sitedata, render_doc, aliases, references, exclusions);
  **JSON-LD**; **bilingual `/es` routes + toggle**. Deferred items (git smart-HTTP, cloning,
  blame, Notes) struck from the plan above. Inbox 0 pending. Coordination sent: Command (Phase C
  gate), editorial (P5b prerequisites + important-information.md), marketing/design/software/bim
  (disclaimer+footer pattern pickup).
- **SUPERSEDED 2026-07-09**: this "NEXT UP = P7" line was the last thing written before P7
  and P8 both actually happened later the same day (2026-07-02) — see "STATUS UPDATE" at top.
- ~~**NEXT UP = P7** (operator-directed 2026-07-02): after this batch, stand up all **three**
  shadow instances (documentation :9090 live; add projects + corporate) for operator visual
  sign-off, then P8 cutover (Command scope: rename `-2`→`app-mediakit-knowledge`, retire old,
  Stage 6). Engine is functionally complete for its audience.~~


- **P3 complete + polish** (2026-07-02): tabs, sidebar (+ audit/fonts), "Last updated", TOC,
  tantivy search (de-duplicated to one header bar), canonical trademark footer.
- **P4 nearly complete (2026-07-02)** — versioning/discovery. Done: **history** + **diff** +
  **as-of point-in-time view** + **provenance line**; **discovery set** (robots/sitemap/Atom/
  llms.txt; feed.json cut). **blame CUT** (per product review). **Remaining P4: git smart-HTTP**
  only — its own focused effort (CGI/protocol + read-only-security to expose the content repo
  for `git clone`). **P5b (JOURNAL render)** still queued on editorial + gis replies.
- **Editorial-gate features shipped 2026-07-02** (unblock the 3-wiki Phase C re-categorization,
  command-20260702-*): **categories.yaml consumption** (new `sitedata` module → nav names+order,
  BCSC-verbatim, content-filtered, graceful on absent dirs), **alias resolution** (`aliases:`
  → 301 canonical), **redirects.yaml** (`/{slug}` → 301). Replied to Command with the deploy
  caveat (live only on -2 shadow; moves couple to P8 cutover). **References** shipped:
  `references:` frontmatter → comrak footnotes (`render_doc`), fixing 43 dead-`[^N]` articles.
  Inbox cleared (10→0 pending).
- **Blocked on project-editorial:** `short_description` on GUIDEs/TOPICs (request sent);
  the merkle article's orphan `[^1]` markers + `references:` frontmatter → a "References"
  section is a small engine feature to build (tracked follow-up).
- **JOURNAL render (P5b) — decided + handed off 2026-07-02.** Feasibility: journals render as
  landing pages on a `/research/` namespace + linked full-text — NOT full IMRAD bodies as
  `/wiki/` articles (declined). **Architecture (operator-ratified): surfaces do NOT share
  render code** — they share SOURCE (papers in Git) + DATA (one `citations.yaml` + one
  canonical notice-text file) + the CONTRACT + a golden-fixture suite; each surface owns its
  renderer (sovereign binaries: Rust wiki, Python gis). Full contract staged at
  `.agent/drafts-outbound/SPEC-journal-wiki-render-contract.md` (§0 architecture, §§1–8 core,
  §9 cross-surface/gis, §10 geospatial `paper_class` class, per-surface publish gate). **Four
  messages sent** (all `command-20260702-*`): project-editorial (contract + citation/slug/
  category/abstract prerequisites + backward-fix 10 papers), project-gis (build own Python
  renderer conforming to contract + static-figure-export step + geospatial class), Command
  (governance: homes for notice-text data + golden fixtures; citations.yaml JOURNAL entries),
  project-design (heads-up: notice-banner/landing/academic-layout/plate components at build).
  **Our engine work (P5b) is gated behind P3 core** (TOC+sidebar). Open: awaiting editorial +
  gis replies on the prerequisites/own-renderer approach.
- Deploy/swap (P7/P8) and Stage-6 promotion are Command Session scope — route via outbox.
- Supersedes the chrome-only direction in [[BRIEF-wiki-redesign]] and the OLD-engine design
  work in [[BRIEF-visual-excellence]], [[BRIEF-phase2-redesign]], [[BRIEF-slides]],
  [[BRIEF-inline-annotations]] (those describe the retiring `app-mediakit-knowledge` crate;
  the equivalents are being rebuilt fresh here).
