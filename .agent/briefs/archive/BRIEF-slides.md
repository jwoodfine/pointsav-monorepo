---
artifact: brief
schema: foundry-brief-v1
status: archived
brief-id: project-knowledge-slides
parent: project-knowledge-phase2-redesign
owner: project-knowledge
created: 2026-06-29
updated: 2026-07-09
superseded-by: project-knowledge-ng-rewrite
---

# BRIEF — Design System Slides Primitive (P5-7)

> **Parent:** [[project-knowledge-phase2-redesign]]
> **Source spec:** BRIEF-phase2-redesign.md §5.6
> **Classification:** New leapfrog primitive — no major wiki ships this

## Context

Deferred from Phase 5 (2026-06-26) due to implementation complexity. All Phase 5 items
P5-1 through P5-9 are complete except this one. Specification was authored in the parent
BRIEF §5.6. This child BRIEF captures the implementation plan ready for coding.

**Engine state at deferral:** 137 tests passing; 36/36 Playwright viewport assertions pass.
Commits `012ad9d5` / `a2d0138c` / `2cb6e9b1` queued for Stage 6 + foundry-prod rebuild
(message in outbox → Command). Sub-clone to work in:
`clones/project-knowledge/pointsav-monorepo/app-mediakit-knowledge/`

## What it is

Embed Keynote/Pitch-style slide decks as wiki articles on documentation.pointsav.com.
`slide-deck` is a first-class content type alongside TOPIC and GUIDE articles.

**Genuine leapfrog:** no major wiki platform (Wikipedia, Notion, Confluence, GitBook,
Docusaurus) ships this capability.

**Why it matters:**
- Design system documentation benefits from visual slides (token grids, colour palettes,
  spacing systems, component anatomy)
- Slides embedded in TOPICs make complex architectural concepts accessible without dumbing
  them down — the slide is a visual anchor; the prose is the explanation
- JSON-LD `Presentation` + per-slide `WebPageElement` enables rich search snippets
- Complements Doctrine claim #39 research-trail discipline: slides carry their own metadata

---

## Specification

### Content type frontmatter

```yaml
title: "Design System Color Tokens"
content_type: slides
slide_count: 12
aspect_ratio: "16:9"
transcript: true   # render <details> prose transcript alongside each slide
```

### Authoring in Markdown

Slide boundaries via `---` horizontal rules within a fenced `:::slides` block (new comrak
custom block hook). Author writes one slide per section:

```
:::slides
# Slide One Title

Content for slide one.

---

# Slide Two Title

Content for slide two.
:::
```

### HTML structure

```html
<div class="slide-deck" data-slide-count="12" role="region" aria-label="Slide deck: Title">
  <div class="slide-deck__controls">
    <button class="sd-prev" aria-label="Previous slide">←</button>
    <span class="sd-progress">1 / 12</span>
    <button class="sd-next" aria-label="Next slide">→</button>
    <button class="sd-fullscreen" aria-label="Fullscreen">⛶</button>
  </div>
  <div class="slide-deck__viewport">
    <section class="slide active" aria-label="Slide 1 of 12">
      <!-- Markdown content rendered here -->
    </section>
    <section class="slide" aria-label="Slide 2 of 12" hidden>
      <!-- ... -->
    </section>
  </div>
  <details class="slide-deck__transcript">
    <summary>Read transcript</summary>
    <!-- Full prose transcript for accessibility — JS-off readable -->
  </details>
</div>
```

### CSS tokens

```css
--slide-aspect: 16 / 9;
--slide-bg: var(--bg-subtle);
--slide-fg: var(--fg-1);
--slide-border: var(--border);
--slide-radius: var(--radius-md);
--slide-control-bg: rgba(0, 0, 0, 0.45);
--slide-control-fg: #ffffff;
```

### Keyboard navigation

| Key | Action |
|---|---|
| `←` / `ArrowLeft` | Previous slide |
| `→` / `ArrowRight` | Next slide |
| `F` | Toggle fullscreen |
| `Escape` | Exit fullscreen |

State persists in URL hash (`#slide-3`) — enables deep links to individual slides.

### Accessibility requirements

- When `transcript: true`, every slide section has a corresponding entry in the `<details>`
  transcript block
- The `.slide-deck` must be fully readable as linear text when JavaScript is disabled;
  `aspect-ratio` collapses to `height: auto` in the no-JS fallback
- `role="region"` + `aria-label` on container; each `<section>` has `aria-label="Slide N of M"`
- `sd-prev` / `sd-next` buttons: `aria-disabled="true"` at boundary slides

---

## Implementation plan

Six steps; all within `app-mediakit-knowledge`. Run full test suite + 36-check Playwright
pass at the end before committing.

| Step | File | What |
|---|---|---|
| 1 | `src/render.rs` | Add `:::slides` custom fenced block handler; emit `div.slide-deck` wrapper; split on `---` into `section.slide` elements; wire `data-slide-count` |
| 2 | `src/server/wiki_handlers.rs` | Detect `content_type: slides` in frontmatter; inject `<script src="/static/slide-deck.js" defer>` in head; set `data-quality` attribute appropriately |
| 3 | `static/slide-deck.js` | Keyboard controller (← → F Esc); `sd-progress` counter update; URL hash state (`#slide-N`); Fullscreen API with vendor prefix fallback; `aria-expanded` on controls |
| 4 | `static/style.css` | `.slide-deck`, `.slide-deck__controls`, `.slide-deck__viewport`, `.slide`, `.sd-progress`, `.slide-deck__transcript`; `aspect-ratio: var(--slide-aspect)` + no-JS fallback; dark mode block |
| 5 | `src/render.rs` tests | Unit test: `:::slides` block with 3 slides + `---` separators → verify slide count, HTML wrapper, `section.slide` elements; test JS-off fallback transcript rendering |
| 6 | `scripts/responsive-check.js` | Add 1 assertion per viewport: `document.querySelector('.slide-deck')` presence check when a slides-type article is loaded |

## Files in scope

| File | Status | Notes |
|---|---|---|
| `src/render.rs` | edit | Add comrak custom block hook for `:::slides` |
| `src/server/wiki_handlers.rs` | edit | content_type detection + JS injection |
| `static/slide-deck.js` | **new** | Full JS controller |
| `static/style.css` | edit | New CSS block at end of file (Sovereign Editorial section) |
| `scripts/responsive-check.js` | edit | +1 assertion for slide-deck |

## Complexity estimate

**Medium.** Two new integration points (comrak hook + JS controller). No database, no API,
no new dependencies. Comrak already has custom block support via `SyntaxHighlighterAdapter`
pattern; `:::` block syntax is already valid in the parser.

## Not in scope

- PDF / PNG slide export
- Slide transition animations (Phase 2 enhancement)
- Multi-deck per article (one deck per article is the initial model; compose via wikilinks)
- Server-side rendering of slide states (client-side only; graceful JS-off fallback covers it)
- `:::slides` within `:::slides` nesting (undefined; reject in renderer)

## JSON-LD schema

When `content_type: slides`, emit in `<head>`:

```json
{
  "@context": "https://schema.org",
  "@type": "Presentation",
  "name": "{{title}}",
  "url": "{{canonical_url}}",
  "numberOfItems": {{slide_count}},
  "hasPart": [
    { "@type": "WebPageElement", "position": 1, "name": "Slide 1" },
    ...
  ]
}
```

---

## Resolved questions (closed 2026-06-29)

- **Comrak `:::` syntax hook:** ✓ No native BlockParser hook needed. Used a pre-render pass
  (`render_slides_blocks()`) that transforms `:::slides ... :::` to HTML before comrak sees
  the content. comrak passes the `<div class="slide-deck">` block through verbatim as a
  CommonMark type-6 HTML block (`options.render.unsafe = true`).
- **Fullscreen API on iOS Safari:** ✓ Implemented CSS fallback. `requestFullscreen().catch()`
  adds `sd-fullscreen--active` class (`position: fixed; inset: 0; z-index: 9999`).
  Native fullscreen exit synced via `fullscreenchange` listener that removes the class.
- **Slide count metadata:** ✓ `render_slides_blocks()` splits on `\n---\n`, counts the
  resulting `Vec<&str>`, and emits `data-slide-count="{n}"` on the wrapper div. The
  pre-render pass has the full block content before any HTML is emitted.
