---
artifact: brief
schema: foundry-brief-v1
status: superseded
brief-id: project-knowledge-wiki-redesign
title: Wiki Redesign — Three Wikis Institutional Quality & Research Infrastructure
owner: project-knowledge
created: 2026-06-30
updated: 2026-06-30
parent: project-knowledge-phase2-redesign
---

# BRIEF — Wiki Redesign: Institutional Quality & Research Infrastructure

## Context

Three wikis running on `app-mediakit-knowledge` (ports 9090/9093/9095):
- **documentation.pointsav.com** — technical platform documentation; edited by software developers and engineers
- **projects.woodfinegroup.com** — Development Markets, Architecture, Woodfine Buildings/Development Classes; edited by architects, engineers, construction professionals
- **corporate.woodfinegroup.com** — Corporate governance, legal records, Ongoing Reporting Requirements; edited by lawyers and accountants

**The design failure:** The wikis currently look like a marketing site. They should look like what they ARE — authoritative institutional records. Wikipedia does not look like software; it looks like an encyclopedia. EDGAR does not look like software; it looks like a government filing database. Our wikis must look like Ongoing Reporting Requirements.

**Strategic vision:** `app-mediakit-knowledge` as a sovereign, deployable alternative to EDGAR/SEDAR for markets and companies without access to quality centralized disclosure infrastructure. Companies would use it voluntarily if the product is genuinely good — product quality is the unlock.

**Sovereign editing path:** Companies requiring fully private editing can pair `app-mediakit-knowledge` with `app-privategit-source` for a private git backend — no GitHub dependency.

**AI connection posture:** The wiki exposes `/api/doorman/complete` and `/api/doorman/instruct` as 501 stubs — the seam is OPEN, called on demand when needed. Do not integrate app-mediakit-knowledge with the Doorman proactively; the connection is the Doorman's responsibility to initiate when the time comes.

## Scope

- Visual design direction for all three wiki instances
- Code separation (chrome module formalization — Phases 7A-7B)
- Cross-domain navigation (Phase 7C)
- CSS/maud design overhaul driven by Opus swarm findings (Phase 7D)
- Editing workflow for non-technical users (Phase 7E)

## Decisions Locked

- **Design reference:** EDGAR + Wikipedia, not Q4 Inc. + Stripe. Institutional document aesthetic, not software aesthetic.
- **Test:** Does this feel like reading a document, or using an app? The former wins.
- **Typography weight:** Content is 90% of visual weight; chrome (nav, footer, controls) is unobtrusive and recedes during reading.
- **Link discipline:** All nav links, toolbar items, and footer links must point to real, working routes. No `href="#"` placeholders, no labels that misrepresent the destination. Decided 2026-06-30.
- **Article tabs:** Article, Notes, Source (→ `/git/{slug}`), History, What links here. No "Talk" tab. No "Edit" tab until an in-browser editor is wired. Decided 2026-06-30.
- **"Subscribe" removed from home page:** The atom feed link is real but "Subscribe" implies email subscriptions we don't offer. Relabeled "Atom feed". Decided 2026-06-30.
- **inject_edit_pencils suppressed:** `href="#"` placeholder edit anchors removed from all rendered article HTML. `inject_edit_pencils()` function retained for future editor wiring. Decided 2026-06-30.

## Design Directions Under Consideration

*(Showing Loop Graphite Register top concepts — 2026-06-30. Prior loop: Loop Oxblood Docket.)*

**Home page:** Three-way contest — M-1 filing-index docket TABLE (replace entire home with one dense monochrome ruled table: date · control no. · title · type · seal) vs. M-2 bounded paper leaf on desk-gray ground (`.leaf{max-width:64rem;margin:2.5rem auto;box-shadow:0 1px 4px rgba(0,0,0,.18);border-radius:0}`) vs. M-3 Cover Sheet + Registrar's Certificate of Completeness (computed, gap-detecting attestation over the docket). Consensus audit finding: M-1 is the single biggest opportunity; M-2 is the cheapest 100x move; M-3 pairs with M-1 as the above-fold identity.

**Article page:** A-1 Bounded Filing Sheet (retire two-column app shell; single ~34rem/66ch serif column, TOC folded as dotted-leader Contents block) vs. A-2 Filing-Detail Caption Masthead (EDGAR/CM-ECF `<dl>` caption block consolidating 7D's citator/signature/last-edited) vs. A-3 Pinpoint §/¶ addressability + scholarly footnote apparatus via CSS counters.

**Chrome:** H-1 single-ink palette rewrite (retire all 7 jewel tones + blue/purple links; near-black ink on warm paper `#faf8f3`; ONE oxblood `#6a2e2e` accent reserved for currency status only) vs. H-2 self-hosted document serif at reading size (~19px, old-style figures; `@font-face` from `static/vendor/fonts/`; sans demoted to tracked-out uppercase labels) vs. H-3 de-app affordances + per-instance authority gradient (`border-radius:0`; hairlines; `body[data-instance]` tokens giving corporate/projects/documentation three voices from one stylesheet).

**Implementation order (per swarm synthesis):** Land H-1/H-2/H-3 chrome tokens first (all home + article concepts consume them), then home M-1/M-2, then article A-1, then data-threaded concepts (git-derived `control_no` into `ArticleMeta`, completeness certificate, filing-detail caption). Note: documentation home retains category/search navigation — pure docket replacement is corporate + projects only.

## Code Changes Log

| Date | SHA | Change | Effect |
|---|---|---|---|
| 2026-06-30 | c49a9051 | Clean-sheet wiki-* chrome rebuild — new chrome.css (539 lines), sovereign.rs complete rewrite (wiki_header/wiki_footer/wiki_mobile_nav/wiki_simple_page), shell.rs/mod.rs/home/wiki/misc handlers updated, style.css lines 545-720 deleted; zero .s-* or --chrome-* names remain | Build passes; smoke test clean (2 wiki-* hits, 0 .s-* hits, chrome.css served); Stage 6 complete (staging mirrors + promote-queue) |
| 2026-06-30 | 8ff9b5b1 | Add institutional letterhead to app-mediakit-shell — home.pointsav.com and home.woodfinegroup.com now show entity · seat lockup above topnav, matching Phase 7D wiki chrome | 10/10 shell tests; marketing builds clean |
| 2026-06-30 | 80e5fe89 | Fix test wiki_page_has_edit_pencils — flip assertions to match Phase 7A suppression decision | 144/144 tests passing |
| 2026-06-30 | 54aba8f0 | Regenerate Cargo.lock after 312-commit rebase (158 insertions, 388 deletions) | Lock file coherent; release build unblocked |
| 2026-06-30 | ad4b6875 | Fix triple [[bin]] sections in service-fs/Cargo.toml — rebase-replay artefact | Duplicate binary name error resolved |
| 2026-06-30 | 68914660 | Fix duplicate CAT_ACCENT_PALETTE const in server/mod.rs — rebase-replay artefact | Cargo check passes; build unblocked |
| 2026-06-30 | e380970c | Phase 7D: Implement all 7 Loop Oxblood Docket concepts — H-1 Chrome Currency State, H-2 Letterhead Lockup, H-3 Printer's Colophon, M-2 Gazette Nameplate, A-1 Citator Treatment Signal, A-2 Conformed Signature Block, A-3 Pinpoint Paragraph Numbering | Institutional chrome across all 3 wiki instances; full CSS + Rust maud; 4 new Tenant methods; 275 CSS lines added |
| 2026-06-30 | (prior) | Phase 7A: Remove inject_edit_pencils from render pipeline; relabel "Edit" tab to "Source"; fix duplicate doc-edit-row; relabel "Subscribe" to "Atom feed" | No href="#" anchors in rendered articles; tabs match actual routes |

## Debug Log

*(Append-only. Issues found during implementation.)*

## Open Questions

- **Footer page/* links:** `/page/contact`, `/page/disclaimer`, `/page/privacy` are in the footer but depend on content files existing in the production content repos. Privacy pages are known blocked (project-editorial, 3 failed attempts). Need to verify which exist in production before deciding to remove or keep. Track separately.
- **AI feedback loop from consultants:** "AI should be able to connect to app-mediakit-knowledge and take in feedback from the consultants to help make for better outputs." Architecture unclear — is this a webhook, an API endpoint, or a read-only feed? Scope: future research item.
- **Cross-wiki navigation:** What does the header property-switcher look like? What does the "← Home" back-link look like? Phase 7C scope.
- **Per-instance typography differentiation:** Should corporate use a more conservative font pairing (Source Serif 4 body only, no Playfair Display) vs. projects (which could use richer display typography for architecture content)?
- **Editing UX by audience:** Lawyers/accountants on corporate need the lowest-friction browser edit path. Architects on projects are somewhat technical. Developers on documentation are fully comfortable with GitHub. Does each instance need a different editing affordance?

## Iterations Log

*(Append-only. Each Opus swarm run adds one named block.)*

### Loop Graphite Register — 2026-06-30
**Model:** claude-opus-4-8
**Audit baseline:** documentation 3/10, projects 3/10, corporate 3/10
**Gaps closed vs. prior loop:** first ideation pass since Phase 7D shipped to localhost — carry-forward items from Loop Oxblood Docket (filing-index table, Registrar's Certificate of Completeness, per-instance authority gradient, fonds›series›file›item breadcrumb) are now developed into concrete, ranked, CSS+maud-implementable concepts rather than open questions. No concepts implemented yet (ideation loop).
**Regressions detected:** Phase 7D is NOT visible on any prod site (all three audits: phase7d_visible=false) — prod still serves a pre-7D build that also exposes edit pencils meant to be suppressed. This is a deployment/promotion blocker, not a design regression, but it holds the public grade at 3/10.
**Research findings (survived adversarial):**
  - Filing-index/docket TABLE replacing the card grid (all variants) — maud `<tr>` iteration + border-collapse/tabular-nums; named biggest opportunity by all three audits.
  - Near-monochrome single-ink palette with ONE functional accent reserved for currency status (KeyCite/Shepard's one-ink discipline) — `:root` + `body[data-instance]` token rewrite, enforced by build-time CI grep guard.
  - Self-hosted document serif at reading size (~19px, ~66ch, old-style figures) via `@font-face` — 'We Own It' sovereignty, no CDN dependency; Georgia `size-adjust` fallback kills FOIT/CLS.
  - Retire app affordances (`border-radius:0`, hairlines, no pills/chips/⌘K/SVG CTA arrows/colored active-tab).
  - Per-instance typographic authority gradient via `body[data-instance]` custom properties (corporate austere → projects richer → documentation technical) — one stylesheet, three registers.
  - Bounded paper 'leaf' on a desk ground replacing the edge-to-edge app viewport — whole-frame de-app move.
  - Registrar's Certificate of Completeness: computed, gap-detecting contiguous-accession attestation on the home.
  - EDGAR/CM-ECF Filing-Detail caption block from git + F12 + front-matter, consolidating 7D's scattered bands.
  - Pinpoint §/¶ addressability + scholarly footnote apparatus via CSS counters — serves the locked Cite mandate.
  - Git-derived zero-padded control number computed in the Rust handler, threaded into `ArticleMeta`.
**Top-ranked concepts:**
  - Main page M-1: The Register — filing-index docket TABLE as the entire home page — replace hero + DYK + feed + colored category cards with one dense monochrome ruled table (date · control no. · title · type · seal).
  - Main page M-2: The Bound Volume — set the whole home on a paper leaf floating on a desk ground — abolish the edge-to-edge app viewport; wrapper div + two CSS rules for a pre-cognitive 'filed document' gestalt.
  - Main page M-3: The Cover Sheet + Registrar's Certificate of Completeness — centered registrant lockup + statutory basis + a computed, gap-detecting contiguous-accession attestation above the register.
  - Article A-1: The Bounded Filing Sheet — one measured serif column (~66ch) with the TOC folded in as a dotted-leader Contents block, retiring the two-column gadget rail.
  - Article A-2: The Filing-Detail Caption Masthead — EDGAR/CM-ECF `<dl>` caption (control no. · date · rev sha · /s/ F12 approver · currency) replacing the collaborative-software byline.
  - Article A-3: Pinpoint §/¶ addressability + scholarly footnote apparatus — CSS-counter section/paragraph numbers with copyable permalinks and an auto-numbered hanging-indent Notes list serving the Cite mandate.
  - Header/Footer H-1: Single-ink palette rewrite — retire all seven jewel tones + blue/purple links; near-black ink on warm paper with one oxblood accent that means currency-of-record only.
  - Header/Footer H-2: Document serif everywhere at reading size — self-hosted `@font-face` default serif; sans demoted to tracked-out uppercase chrome labels ('We Own It' sovereignty).
  - Header/Footer H-3: De-app the affordances + per-instance authority gradient — `border-radius:0`, hairline rules, no pills/kbd/CTA arrows; `body[data-instance]` token sets giving corporate/projects/documentation three institutional voices from one stylesheet.
**Implemented this loop:** none (pending operator review)
**Carry to next loop:** (1) DEPLOY Phase 7D to foundry-prod — hard blocker before any concept ships publicly. (2) Implementation order: chrome tokens (H-1/H-2/H-3) first since home + article concepts consume them, then home M-1/M-2, then article A-1, then data-threaded concepts. (3) Resolve completeness-attestation canonical home (home certificate vs footer colophon) — recommend home-canonical. (4) Shared data plumbing: git-derived `control_no` into `ArticleMeta`; front-matter `record-type`/`effective-date`/`supersedes` fields; deterministic §/¶ anchor ids. (5) Next-loop survivors to rank: archival-arrangement breadcrumb, currency/supersession cross-reference apparatus, financial-statement table cluster (accountant trust), print/paged-media skeleton. (6) Preserve instance nuance: documentation home keeps category/search; pure docket replacement is corporate + projects only.

### Loop Oxblood Docket — 2026-06-30
**Model:** claude-opus-4-8
**Audit baseline:** documentation 8/10, projects 6/10, corporate 9/10
**Gaps closed vs. prior loop:** first run
**Regressions detected:** none
**Research findings (survived adversarial):**
  - Public single-ink redline diff of two git revisions on the History tab (additions underlined, deletions struck — never red/green); no public financial/legal record site does this.
  - Provenance masthead line under each title (small-caps: Committed date · Revision n · short SHA · F12 approver); EDGAR surfaces accessions but never a git hash or an approver identity.
  - Per-article record-seal line (content sha256 + git short-SHA + last-sealed date + F12 approver) as reader-visible, verifiable provenance; no public record system (EDGAR/SEDAR+/Companies House) exposes a content hash.
  - Signatory/approval table rendered from live F12 approval-gate metadata (approved/abstained/recused, with dates); binding an operational governance workflow to the visible public record has no precedent.
  - Registrar-style completeness/sequence attestation (contiguous accession range · N withheld · 0 gaps) computed from git; audit-completeness language exists only inside internal RM systems, never on a public-facing record.
  - EDGAR/CM-ECF/Companies-House single-ink filing-index table replacing the multi-hued category-card home page (date · control number · title · type · seal), generated from git log.
  - Per-instance typographic authority gradient via `body[data-instance]` CSS custom properties (one style.css, three postures: corporate most restrained, projects richer, documentation technical).
**Top-ranked concepts:**
  - Main page M-1: Registrar's Certificate of Completeness — small-caps attested band above the docket asserting the accession sequence is contiguous (WDF-CORP-2026-000001 through -000142 inclusive, none reserved/withdrawn); withheld records render as present-but-muted rows (grafted from Gamma M2) so gaps are shown, never hidden.
  - Main page M-2: Newspaper-of-Record Nameplate + Edition Folio — home masthead set as a gazette front page with a folio line (VOL. 4 · NO. 142 · TORONTO · 30 JUNE 2026 · COMPILED FROM COMMIT 3a9f2c), framing the landing as a publication of record, not a product home.
  - Main page M-3: Lifecycle-Event Docket — a court-docket register of record EVENTS (Filed · Superseded by → · Sealed · Legal hold placed/lifted), event verb in small caps and dated, derived from git log; closed with an accountant's double-rule control-total footing row (grafted from Beta M2).
  - Article A-1: Citator Treatment Signal — hairline-ruled standing box under the masthead stating current-of-record / superseded-by → / relied-upon-by N (from the what-links-here backlink graph), in type not traffic-light color; grafted with an auto-built Table of Authorities (Gamma A2) listing what the record stands on.
  - Article A-2: Conformed Signature Execution Block — the F12 approver rendered in EDGAR's conformed-signature convention ('/s/ Jennifer Woodfine — Approved and sealed 30 June 2026'), optional testimonium line, set off by a hairline signature rule; pure maud generated content, zero JS.
  - Article A-3: Pinpoint Paragraph Numbering (Neutral Citation) — pleading-paper marginal ¶ numbers down a ruled gutter, each a permanent deep-link and copy-to-cite ('WDF-CORP-2026-000142 ¶14 (rev. 4)'); grafted with a filing-grade @page print mode (Beta A1) so the same URL is both live record and paginated hard copy.
  - Header/Footer H-1: Chrome Currency State — masthead seal reads CURRENT OF RECORD on the live head, and shifts the whole chrome to a ruled ARCHIVED REVISION — superseded band (with a real link to current) whenever a pinned historical SHA is viewed; single ink, never a warning color, zero JS.
  - Header/Footer H-2: Letterhead Lockup Header — top chrome reconceived as institutional letterhead (maintaining entity · seat · statutory basis, e.g. WOODFINE MANAGEMENT CORP. · Toronto, Ontario · under OBCA and NI 51-102), utility nav demoted below a hairline; grafted with a register selector (Gamma H1) presenting the three instances as three registers of one archive.
  - Header/Footer H-3: Printer's Colophon + Completeness Attestation Footer — an edition imprint certifying the provenance of this rendering (build SHA · content commit · render time · typefaces set in), joined with a machine-readable completeness line (contiguous range · N withheld · 0 gaps · attested date) grafted from Gamma H3.
**Implemented this loop:** H-1, H-2, H-3, M-2, A-1, A-2, A-3 (all 7 high-implementability concepts — Phase 7D; SHA e380970c)
**Carry to next loop:** Go deeper on the archival-arrangement breadcrumb (fonds › series › file › item, Beta H3 / Gamma H2) as a fourth chrome concept; prototype the per-instance authority gradient so corporate reads most restrained; resolve where the completeness attestation lives canonically (home certificate M-1 vs. per-page footer H-3) to avoid duplication; evaluate Alpha's sticky-H2 / defined-term-reveal / cross-reference-peek cognitive-load aids against the "document not app" test before promoting any hover-card affordance.

**Loop naming convention:** Two-word colour-noun (e.g., "Loop Cerulean Compass").
**Block format:**
```
### Loop [Name] — [YYYY-MM-DD]
**Model:** claude-opus-4-8
**Audit baseline:** documentation X/10, projects X/10, corporate X/10
**Gaps closed vs. prior loop:** [list or "first run"]
**Regressions detected:** [list or "none"]
**Research findings (survived adversarial):** [list]
**Top-ranked concepts:**
  - Main page M-1: [title] — [one-line]
  - Main page M-2: [title] — [one-line]
  - Main page M-3: [title] — [one-line]
  - Article A-1: [title] — [one-line]
  - Article A-2: [title] — [one-line]
  - Article A-3: [title] — [one-line]
  - Header/Footer H-1: [title] — [one-line]
  - Header/Footer H-2: [title] — [one-line]
  - Header/Footer H-3: [title] — [one-line]
**Implemented this loop:** [commit SHAs or "none"]
**Carry to next loop:** [focus areas]
```

## Research Premise (stable — reuse each session)

We are building a 21st-century record-keeping platform. The platform runs as three separate wikis, each with a distinct audience:
- **documentation.pointsav.com** — PointSav technical platform documentation; audience: software developers, platform architects
- **projects.woodfinegroup.com** — Development Markets, Architecture, Woodfine Buildings/Development Classes; audience: architects, engineers, construction professionals, urban planners
- **corporate.woodfinegroup.com** — Corporate governance, legal records, Ongoing Reporting Requirements; audience: securities lawyers, corporate accountants, board members, regulators

**What makes this different from a knowledge base:** The wikis are replacing file-keeping. Lawyers no longer keep files — the wiki IS the record. Every article change is a git commit. The history tab IS the audit trail. The cite page IS the reference standard. The F12 gate IS the approval workflow.

**The design failure we are correcting:** The wikis currently look like a marketing site. They must look like Ongoing Reporting Requirements — an institutional record. Wikipedia is the aesthetic reference: it does not look like software, it looks like an encyclopedia. EDGAR is the authority reference: it does not try to impress, it is the record.

**Strategic position:** `app-mediakit-knowledge` as a deployable EDGAR/SEDAR alternative for any company or market. Companies would use it voluntarily if the product is genuinely good — product quality is the unlock.

**The competitor to beat:** Q4 Inc. (q4inc.com) — IR communications SaaS. Their product is how companies TALK TO investors. Our product is the record ITSELF. Different products.

**Technology constraint:** Rust/axum/maud server. All HTML is generated at compile time in Rust code. All CSS is in `style.css`. JavaScript is minimal. Every idea must be achievable in CSS + maud markup, with at most minimal vanilla JS. No React.

**Design constraint — institutional, not software:** Every research finding passes this test: does this feel like reading a document, or using an app? The former wins. Typography and content are 90% of the visual weight. The chrome (nav, controls) should be unobtrusive — institutional, calm, receding when reading.

**Research goal this session:** Produce 3 novel, unimplemented ideas each for:
1. Main page (the home page that records-keepers land on)
2. Article page (the document reading experience)
3. Header and footer (the persistent chrome across all pages)

"Novel" means: not currently done by Q4 Inc., implementable in CSS + maud, appropriate for lawyers/accountants/executives. "Easy" means: sophisticated but so obvious once seen that it just works.

Also audit the three live wikis and report current gaps vs. Decisions Locked.

## Carry-Forward

- **DEPLOY Phase 7D + shell letterhead to foundry-prod:** Binary on localhost (144/144 tests). Commits pending Command canonical merge: ad4b6875, 54aba8f0, 80e5fe89 (wiki); 8ff9b5b1 (shell). Shell binary (app-mediakit-marketing) also needs a prod rebuild. Deploy to prod is a Command/project-gis operation.
- **Phase 7E (Loop Graphite Register) — scope resolved:** Apply changes uniformly to ALL THREE wiki instances. Operator overrides Loop Graphite Register recommendation to differentiate documentation from projects/corporate. Awaiting operator go-ahead on which concepts to implement (see ranked list in Design Directions section above).
- **Cross-site link audit — CONFIRMED CLEAN:** All cross-property links in app-mediakit-knowledge are in sovereign.rs header elements only (s-letterhead entity link + sibling link, mobile nav drawer back-to-home). No cross-site links in body or footer. Rule satisfied.
- **Phase 7B deferred:** Chrome module formalization (article.rs, home.rs wiring). `wiki_chrome()` still ~700 lines inline in wiki_handlers.rs. Lower priority than visual work.
- **Privacy pages:** blocked on project-editorial (attempt 3).
- **Data plumbing prerequisites for Loop Graphite Register concepts:** git-derived zero-padded `control_no` into `ArticleMeta`; front-matter `record-type`/`effective-date`/`supersedes` fields; deterministic §/¶ anchor ids keyed off heading slug + ordinal.
