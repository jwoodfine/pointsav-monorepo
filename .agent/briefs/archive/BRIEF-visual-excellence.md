---
artifact: brief
schema: foundry-brief-v1
status: archived
brief-id: project-knowledge-visual-excellence
parent: project-knowledge-phase2-redesign
owner: project-knowledge
created: 2026-06-29
updated: 2026-07-09  # P6-G complete — all sessions shipped; archived at ng-rewrite cutover
superseded-by: project-knowledge-ng-rewrite
---

# BRIEF — Phase 6: Visual Excellence + Records Positioning

> **Parent:** [[project-knowledge-phase2-redesign]]
> **Child:** [[project-knowledge-inline-annotations]] (Talk tab replacement — separate brief)

## Mission

Raise the three live wiki sites from "functional Wikipedia-clone" to world-class editorial
reading experience, while simultaneously signalling "authoritative auditable record" to
SOC3/DARP/US-EU government audit reviewers.

All changes are presentation layer (CSS, HTML templates, JS). No rebuild. No engine changes.
Engine is correct — gaps are visual.

## Research basis (2026-06-29)

Three research agents completed:
1. **Chrome structure audit** — full inventory of current header, article chrome, home page, footer
2. **22-site global comparison** — editorial/news, technical docs, knowledge/reference, institutional, design-forward, dark-mode-excellence clusters
3. **WCAG/compliance deep-dive** — WCAG 2.1 AA confirmed numbers, EN 301 549, SOC3/trust signals, Talk-tab alternatives in regulated industries, USWDS government standards

Key sources: W3C WCAG 2.1, Section 508, EN 301 549, Cloudflare Trust Hub, Stripe Security, USWDS, AICPA SOC guidance, Guru/SharePoint/Confluence/Slab patterns.

---

## Sessions A–F: Implementation Plan

### Session A — Typography + Prose Spacing

**Files:** `static/style.css`
**Estimated effort:** 1 session (~2h)

#### Changes

**Prose reading upgrade:**
```css
.prose { font-size: 18px; line-height: 1.78; }
.prose p { max-width: 70ch; }
.prose p + p { margin-top: 1.6em; }
```

**Heading scale (fluid responsive):**
```css
.prose h1 { font-size: clamp(2rem, 4vw, 3rem); line-height: 1.1; letter-spacing: -0.02em; }
.prose h2 { font-size: clamp(1.4rem, 2.5vw, 1.9rem); margin-top: 2.4em; }
.prose h3 { font-size: clamp(1.15rem, 2vw, 1.4rem); margin-top: 1.8em; }
```

**Reading comfort:**
```css
.prose li { line-height: 1.65; }
.prose blockquote { font-size: 1.15em; line-height: 1.7; }
.prose { text-align: left; }  /* explicit: WCAG requires no justified text */
```

**Article title:**
```css
.article__title {
  font-size: clamp(1.75rem, 3.5vw, 2.8rem);
  line-height: 1.15;
  letter-spacing: -0.025em;
}
```

**Mobile safety (iOS zoom threshold):**
```css
@media (max-width: 768px) {
  .prose { font-size: max(16px, 1rem); }
}
```

**Compliance signal in article chrome** (part of Session A):
- Add short git SHA display near "Last edited" date in article chrome
- Add `text-align: left` guard to `.prose` (WCAG 1.4.8 — text must not be justified)
- Confirm no `user-scalable=no` in viewport meta (WCAG 1.4.4)

**Verification:** cargo test 139/139 + Playwright 36/36 + manual at 320/768/1440 light+dark

---

### Session B — Masthead Hierarchy Redesign

**Files:** `src/chrome/sovereign.rs`, `static/style.css`
**Estimated effort:** 1 session (~3h)

**Gap:** Wordmark, search, and controls at equal visual weight. Search is not the dominant
center element. Premium docs sites make search the hero of the header.

**Target:** Stripe docs pattern — search dominates center, wordmark is compact left anchor,
controls are subtle right cluster.

#### Changes

**sovereign.rs `sovereign_nav()`:**
- Search input container: `class="s-search"` with dominant flex sizing
- Search placeholder: per-tenant text ("Search documentation…" / "Search projects…" / "Search corporate docs…")
- Wordmark: ensure `height: 28px; width: auto;` CSS constraint (SVG has no hardcoded dimensions since P4 fix)
- Controls: consolidate into `.s-controls` compact cluster (lang toggle + theme + hamburger)

**style.css additions:**
```css
/* Masthead hierarchy */
.s-search { flex: 1; max-width: 580px; }
.s-search__input {
  height: 40px;
  background: rgba(255,255,255,0.12);
  border: 1px solid rgba(255,255,255,0.22);
  border-radius: 20px;
  color: white;
  font-size: 15px;
  padding: 0 var(--sp-4) 0 40px;  /* space for search icon */
  width: 100%;
  transition: background 150ms, border-color 150ms;
}
.s-search__input::placeholder { color: rgba(255,255,255,0.6); }
.s-search__input:focus {
  background: rgba(255,255,255,0.22);
  border-color: rgba(255,255,255,0.5);
  outline: 2px solid var(--s-accent);  /* WCAG 2.4.11 focus visible */
  outline-offset: 2px;
}
/* Wordmark compact */
.s-wordmark img, .s-wordmark svg { height: 28px; width: auto; }
/* Controls subtle */
.s-controls { gap: var(--sp-2); align-items: center; }
```

**Mobile (≤768px):** search collapses to icon-only; tap expands full-width bar below masthead.

**WCAG notes:** Focus ring on search input must be 2px solid contrast ≥3:1 against adjacent color.

---

### Session C — Home Page Editorial Hero

**Files:** `src/server/home_handlers.rs`, `static/style.css`
**Estimated effort:** 1 session (~3h)

**Gap:** Featured article is a contained card (Wikipedia-ish). Editorial sites use full-bleed
hero with dark gradient and dominant headline.

**Target:** Guardian/NYT pattern — full-bleed image (when `hero_image` present), gradient
overlay, editorial headline at `clamp(2.2rem, 4vw, 3.5rem)`, category label in small caps
above headline, minimal "Read →" CTA.

#### Changes

**style.css — `.featured` upgrade:**
```css
.featured {
  position: relative;
  min-height: 360px;
  background-image: var(--hero-img);
  background-size: cover;
  background-position: center;
  border-radius: var(--radius-md);
  overflow: hidden;
  display: flex;
  flex-direction: column;
  justify-content: flex-end;
  padding: var(--sp-8);
}
.featured::before {
  content: '';
  position: absolute;
  inset: 0;
  background: linear-gradient(to top, rgba(0,0,0,0.85) 0%, rgba(0,0,0,0.4) 50%, transparent 100%);
  pointer-events: none;
}
.featured__category-label {
  font-size: 0.72rem;
  font-weight: 600;
  letter-spacing: 0.1em;
  text-transform: uppercase;
  color: rgba(255,255,255,0.8);
  position: relative;
  margin-bottom: var(--sp-2);
}
.featured__title {
  font-size: clamp(2.2rem, 4vw, 3.5rem);
  font-family: var(--font-display);
  line-height: 1.1;
  letter-spacing: -0.03em;
  color: white;
  position: relative;
  margin-bottom: var(--sp-3);
}
.featured__excerpt {
  color: rgba(255,255,255,0.88);
  font-size: 1.05rem;
  line-height: 1.55;
  max-width: 52ch;
  position: relative;
}
.featured__read-link {
  color: rgba(255,255,255,0.9);
  font-size: 0.95rem;
  font-weight: 600;
  text-decoration: none;
  position: relative;
  margin-top: var(--sp-4);
}
```

**Fallback when no `hero_image`:** navy gradient (`.featured--no-image`) instead of breaking.

**home_handlers.rs:** Add `.featured__category-label` span using the article's category/content_type.
Move "Featured article · Updated DATE" to subtle `.featured__meta` below headline (not above).

**Category grid:** tile `min-width: 240px`, description text visible on card face,
`--cat-accent` 4px left-border stripe (already wired from P5-9).

**WCAG notes:** Text on gradient overlay must meet 4.5:1 contrast. "Read →" link must have
visible text (not color alone). Alt text on hero image (Rust template).

---

### Session E — TOC Active Tracking + Reading Time

**Files:** `static/wiki.js`, `static/style.css`
**Estimated effort:** 1 session (~1.5h, simple JS additions)

#### Gap 1: TOC active section tracking

Current TOC shows all headings with no indication of current scroll position.

**wiki.js addition:**
```javascript
// TOC active-section tracking
(function() {
  const headings = document.querySelectorAll('.prose h2[id], .prose h3[id]');
  if (!headings.length) return;
  const obs = new IntersectionObserver((entries) => {
    entries.forEach(e => {
      const link = document.querySelector(`#toc-list a[href="#${e.target.id}"]`);
      if (link) link.classList.toggle('toc-active', e.isIntersecting);
    });
  }, { rootMargin: '-20% 0px -75% 0px', threshold: 0 });
  headings.forEach(h => obs.observe(h));
})();
```

**style.css addition:**
```css
#toc-list a.toc-active {
  color: var(--s-accent);
  font-weight: 600;
  padding-left: 0.75em;
  border-left: 2px solid var(--s-accent);
  margin-left: -0.75em;
  transition: color 150ms;
}
```

#### Gap 2: Reading time JS (`.reading-time` shows "0 min" because data-words never populated)

**wiki.js addition:**
```javascript
// Reading time calculation
(function() {
  const rt = document.querySelector('.reading-time[data-words]');
  const prose = document.querySelector('.prose');
  if (!rt || !prose) return;
  const words = prose.textContent.trim().split(/\s+/).filter(Boolean).length;
  const mins = Math.max(1, Math.ceil(words / 228));
  rt.textContent = mins === 1 ? '1 min read' : `${mins} min read`;
  rt.dataset.words = words;  // update data attribute for consistency
})();
```

**New Playwright assertion (R9):** check `.reading-time` is not "0 min" on article pages.

---

### Session F — Dark Mode Elevation Layers

**Files:** `static/style.css`
**Estimated effort:** 1 session (~2h, CSS-only)

**Gap:** Dark mode uses flat surfaces. Premium dark mode (Linear, GitHub, Supabase) uses
4 visible elevation layers. The `--bg-elevated`, `--bg-subtle`, `--bg-overlay` tokens may
exist in the dark block but are not applied to surfaces.

#### Changes

**Dark mode block update:**
```css
html[data-theme="dark"],
@media (prefers-color-scheme: dark) {
  /* Elevation layers — chromatic navy (parallels #164679 brand) */
  --bg:          oklch(11% 0.04 250);    /* #0D1520 base canvas */
  --bg-elevated: oklch(14% 0.05 250);    /* +1: cards, sidebar, TOC */
  --bg-subtle:   oklch(17% 0.05 250);    /* +2: code blocks, infobox */
  --bg-overlay:  oklch(19% 0.05 250);    /* +3: dropdowns, modals */
  --bg-hover:    oklch(21% 0.05 250);    /* hover/active state */

  /* More readable muted text */
  --fg-2: oklch(72% 0.02 250);          /* secondary text */
  --fg-3: oklch(58% 0.02 250);          /* tertiary/metadata */
}
```

**Apply elevation tokens:**
```css
html[data-theme="dark"] .toc          { background: var(--bg-elevated); }
html[data-theme="dark"] pre,
html[data-theme="dark"] code          { background: var(--bg-subtle); }
html[data-theme="dark"] .s-subnav    { background: var(--bg-elevated); }
html[data-theme="dark"] aside.infobox { background: var(--bg-elevated); }
html[data-theme="dark"] #search-autocomplete-dropdown { background: var(--bg-overlay); }
html[data-theme="dark"] .wiki-home-stats,
html[data-theme="dark"] .wiki-home-lede { background: var(--bg-elevated); }
```

---

## Compliance Integration (across all sessions)

**WCAG 2.1 AA confirmed requirements (from research 2026-06-29):**
- Contrast 4.5:1 for body text; 3:1 for large text (≥18pt / ≥14pt bold)
- Line spacing ≥1.5x font size (our 1.78 target ✓)
- Paragraph spacing ≥2x at user override — page must not break (WCAG 1.4.12)
- Max line length 80 chars (our 70ch ✓); text-align: left ONLY (no justify)
- Touch targets ≥44×44px on mobile interactive elements
- Reflow at 320px without 2D scroll (Playwright R1 checks this ✓)
- Focus rings on all interactive elements: ≥2px, ≥3:1 contrast

**EU EN 301 549:** WCAG 2.1 AA fully adopted. Biometric (facial rec) clause not applicable.

**Records positioning signals (add across all sessions):**
- Footer legal line: `Git-versioned content · WCAG 2.1 AA · EN 301 549`
- Article chrome: short git SHA display near "Last edited" date
- History tab: consider "Revision history" label (more audit-appropriate than "History")

**SOC3/trust signals (when attestation exists — future):**
- Use "SOC 3 Type II attestation" (NOT "certified") per AICPA language guidelines
- AICPA badge: near footer copyright, links to aicpa.org/soc4so, unmodified logo
- Dedicated `/page/compliance` page in footer Legal column
- Named frameworks: "ISO 27001 · WCAG 2.1 AA · EN 301 549 · Git-versioned"

---

## Verification plan

Each session:
- `cargo test -p app-mediakit-knowledge` — all tests pass (139 baseline)
- `node scripts/responsive-check.js` — 36/36 Playwright assertions pass
- Manual: all 3 instances at 320/768/1440px, light + dark mode
- Lighthouse accessibility score target: ≥95 (measure after Session A)

---

## Work log

| Date | Session | Commit | What |
|---|---|---|---|
| 2026-06-29 | A — Typography + Spacing | `898470a9` | `--leading-reading` 1.78, `--measure` 70ch, `p+p` margin 1.6em, `text-align:left` WCAG guard, h2/h3 fluid clamp(), `.prose li` line-height 1.65, blockquote 1.1em/1.7, `article__title` letter-spacing −0.025em / line-height 1.15 |
| 2026-06-29 | B — Masthead Hierarchy | `2feab2d0` | `.left`/`.right`/`.topnav-center` layout; `.wordmark`, `.brand__mark`, `.brand__wordmark` styled; search pill 40px height / 20px radius / rgba bg; search max-width 560px |
| 2026-06-29 | C — Home Editorial Hero | `63378466` | `.featured` rewritten: gradient card `min-height:280px`, `display:flex flex-direction:column justify-content:flex-end`, editorial `linear-gradient` bg, `::before` scrim; category label `rgba(255,255,255,0.72)`; title `clamp(2.2rem,4vw,3.5rem)` white; excerpt `rgba(0.88)` max-width 58ch |
| 2026-06-29 | D — TOC + Reading Time | (no commit) | Both features already complete from prior phases — IntersectionObserver active tracking + reading time JS both confirmed live |
| 2026-06-29 | E — Dark Mode Elevation | `f9ae99a6` | `--bg` 11%, `--bg-elevated` 14%, `--bg-subtle` 17%, `--bg-overlay` 19% (new), `--bg-hover` 21%; `--fg-2` 74%/0.012, `--fg-3` 60%/0.010; `.s-subnav` → `var(--bg-elevated)`; dropdown `var(--bg-overlay)` |
| 2026-06-29 | G — Records Signals | `998b3a2c` | Footer: `site-footer__compliance` "Git-versioned content · WCAG 2.1 AA · EN 301 549"; article chrome: `revision_sha_short` (8 char) computed in HTML path, displayed as `.doc-header__sha` after History link; `site-footer__inner/__trademark/__compliance` CSS |
| 2026-06-29 | F/D1-D4 — Inline Annotations | `25ffe5fa` | `src/annotations.rs` YAML sidecar (load/save/count, 4 tests); GET `/notes/{slug}` page + POST `/notes/{slug}` F12-gated write; Notes tab replaces Talk with count badge; CSS notes-list/note-card/notes-form/note-count-badge; Playwright PAGES + R-notes-1/2/3; 143+ tests pass; 45/45 viewport assertions |

---

*Brief created 2026-06-29 by totebox@project-knowledge | research: chrome-audit + 22-site comparison + WCAG compliance deep-dive*
