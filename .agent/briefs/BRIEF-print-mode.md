---
artifact: brief
schema: foundry-brief-v1
brief-id: project-knowledge-print-mode
title: app-mediakit-knowledge print mode — research-backed design
status: active
owner: project-knowledge
parent: project-knowledge-ng-rewrite
created: 2026-07-13
updated: 2026-07-13
---

> **2026-07-13 update:** browser-in-the-loop visual verification (Playwright, print
> emulation) completed — see "Verified" section below. It caught and fixed one real bug:
> the citation stamp never actually rendered when printing.

# BRIEF — Print Mode

## Context

Operator asked for a "world class" print mode for the wiki, modeled on Wikipedia's — but
explicitly wanted the actual foundations researched first (what does Wikipedia really do,
what does the web actually support today) rather than guessed at, plus a full codebase audit
so the feature is built on a tested foundation. This BRIEF records that research, the design
decision it produced, and what shipped this session (KNOWLEDGE-PLATFORM-PLAN.md Phase 9).

## Research (dispatched as a live web-search agent, not training-data guesswork)

**What Wikipedia/MediaWiki actually does:** pure CSS, zero client-side pagination JS.
`MediaWiki:Print.css` plus skin `@media print` rules; `Help:Printing` recommends the plain
browser print dialog. The 2017 Print Styles project was a typography/density pass (cut a
sample article from 38 to 28 pages) — still CSS-only. The Collection/PDF extension (Book
Creator) is officially unmaintained; its PDF path went OCG → Proton, i.e. **server-side
headless Chromium rendering the same print CSS**. The single most-printed wiki on earth ships
no client-side pagination library at all.

**2026 browser support:** basic `@page` (size, margins) is solid across Chrome, Firefox 152+,
Safari 18.2+. Chrome 131+ added all 16 margin boxes and page counters natively. The real gap:
`string-set`/`string()` running headers, `target-counter()`, and footnote floats remain
unimplemented in every browser. Break control (`break-inside: avoid`, orphans/widows) works
cross-browser today.

**Paged.js (the JS-pagination alternative):** not healthy. Latest stable is 0.4.3 from July
2023; a "restart" was announced Sept 2025 but has produced no real releases since. ~5.9 MB
unpacked, still depends on the deprecated `@babel/polyfill`. It re-fragments the entire
document into page boxes at render time — 0.5–2s added for a 10-page document, 5–15s for
longer ones — degrading find-in-page, text selection, and accessibility on exactly the long
institutional articles this wiki serves.

## Decision

**Pure CSS. No Paged.js.** The only thing a JS pagination library buys is running
headers/footnotes — and for that gap it would make a stale, DOM-rewriting, effectively
dormant 2023 library this engine's *first-ever* client-side JS dependency, on an engine whose
entire identity is no-build-step minimalism (one small vanilla `app.js` for the mobile nav
drawer and theme toggle, nothing else). Wikipedia validated the pure-CSS posture at maximum
scale; there's no reason this wiki needs to do better than Wikipedia does for print, especially
by taking on a dependency Wikipedia itself doesn't carry.

**Stretch goal, not core scope:** a server-side `?format=pdf` route rendering the same print
CSS via headless Chromium (Wikipedia's actual Proton pattern) — the correct answer if
deterministic, archival PDFs are ever needed for this wiki's EDGAR/SEDAR-alternative
institutional-records posture. Not built this session; recorded here as the right next step
if/when that need becomes concrete, not before.

## What shipped (2026-07-13)

- `@page` rules (margins), print-specific type scale (11pt body, 20pt title).
- Orphan/widow control (`orphans`/`widows: 3`) on prose text.
- Break control: `break-inside: avoid` on tables/figures/blockquotes/code/images;
  `break-after: avoid` on headings — no split mid-table, no orphaned heading at a page foot.
- External-link URL reveal (`a[href^="http"]::after`) — a printed page has no clickable
  links, so every external citation's destination is spelled out inline. Internal `/wiki/...`
  links are left alone (same-site, no information gained by printing the path).
- A new **print-only citation/permalink block** (`.k-print-citation`, `ui::layout::article()`)
  — "Cite this record: /wiki/{slug} — revision {sha}, last updated {date}." Hidden on screen
  unconditionally, shown only in `@media print`, so a printed page is a self-contained,
  attributable record independent of its on-screen chrome. Deliberately does **not** claim a
  "printed on" date — that would need either JS (ruled out) or a server-render-time stamp that
  goes stale the moment a cached page is printed later; the revision/last-updated data is the
  honest thing to show instead.
- Existing baseline extended, not replaced: the 3 pre-existing `@media print` blocks (chrome
  hiding, B&W footer, expanded compliance disclaimer) are unchanged.

Files: `static/app.css` (print media query), `src/ui/layout.rs` (`article()`'s new
`.k-print-citation` block).

## Verified (2026-07-13)

Ran a scratch preview instance (real content mount, `content-wiki-documentation`'s
successor `media-knowledge-documentation`) and drove it with Playwright's print-media
emulation — same browser-in-the-loop discipline as this session's mobile-header fix.

**Bug found and fixed:** the citation stamp never actually appeared when printing.
`.k-print-citation { display: none; }` (unconditional, meant only to hide the block on
screen) sat *after* the `@media print { .k-print-citation { display: block; } }` rule in
`static/app.css`'s source order. Both selectors have identical specificity (one class),
so — regardless of which media context is active — the *later* rule in the cascade wins.
Because the unconditional rule came second, it always won, including while printing,
silently defeating the entire feature it was appended right next to. Fixed by moving the
unconditional rule *before* the `@media print` block, so the print-context override
(later, and only active when printing) correctly takes precedence.

Confirmed via computed-style checks in a real headless-Chromium print render:
- Screen chrome (header, utility strip, search, nav drawer) hidden in print — pass.
- `.k-print-citation` — `display: none` on screen, `display: block` in print (after the
  fix) — pass, with the actual generated text confirmed correct.
- `.k-prose` measure constraint removed in print (`max-width: none`) — pass.
- Body text forced to black in print — pass.
- External-link URL reveal (`a[href^="http"]::after`) — pass (checked against a synthetic
  fixture since the live test article happened to have no external prose links).
- Visual screenshot of the print render reviewed directly: citation stamp, headings,
  footer condensed to 3 columns, orphan/widow-safe prose — all render as designed.

## Carry-forward

- Server-side `?format=pdf` (headless Chromium) — stretch goal, not started.
- Running headers (title + page number repeating on every printed page) are left to the
  browser's own default print header, which already includes title/URL/page-number — judged
  adequate per the research above; revisit only if a concrete need for custom running headers
  (e.g. a section title in the header) emerges.
