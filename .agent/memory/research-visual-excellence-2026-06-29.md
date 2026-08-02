---
name: research-visual-excellence-2026-06-29
description: Phase 6 visual excellence research — 22-site audit, WCAG confirmed numbers, talk-tab alternatives, SOC3/records design patterns, decisions locked
metadata:
  type: project
---

# Visual Excellence Research — 2026-06-29

## Research agents run

1. **Chrome structure audit** — full inventory of current chrome (header, article, home, footer, mobile). Tools: Read, Bash grep.
2. **22-site global comparison** — typography values, dark mode patterns, home page editorial patterns, records positioning signals. Clusters: editorial/news (NYT, FT, Guardian, Bloomberg), technical docs (Stripe, Vercel, Linear), knowledge (Britannica, SEP, Archive.org), institutional (NASA, CERN, MIT), design-forward (Apple, Figma, Notion), dark-mode-excellence (Linear, GitHub, Raycast, Supabase, Framer, Arc).
3. **WCAG/compliance deep-dive** — WCAG 2.1 AA confirmed numbers, EN 301 549, SOC3/trust signals, Talk-tab alternatives in regulated industries, USWDS government standards.

---

## Typography — confirmed values

| Property | Current | Target | Source |
|---|---|---|---|
| Body font-size | 18px (correct) | 18px | ✓ all premium publishers |
| Prose line-height | ~1.55 | 1.78 | NYT 1.75-1.8; FT 1.6; Guardian 1.65 |
| Paragraph spacing | minimal | 1.6em | NYT 28px; FT 24px; WCAG 1.4.12 resilience |
| Max prose width | 66ch | 70ch | Bringhurst 60-70ch; WCAG 80ch max |
| Mobile min font | unknown | max(16px, 1rem) | iOS zoom threshold; WCAG 1.4.4 |
| Heading scale | fixed px | clamp() fluid | All premium sites use fluid headings |
| Article title | fixed | clamp(1.75rem, 3.5vw, 2.8rem) | editorial range |
| Featured hero title | fixed | clamp(2.2rem, 4vw, 3.5rem) | Guardian/NYT hero scale |

## Dark mode — confirmed palette

Chromatic navy approach: navy-tinted dark that echoes #164679 brand masthead.

| Token | Value | Rationale |
|---|---|---|
| `--bg` | `oklch(11% 0.04 250)` | Base canvas; GitHub parallels #0d1117 (blue-tinted) |
| `--bg-elevated` | `oklch(14% 0.05 250)` | TOC, cards, sidebar |
| `--bg-subtle` | `oklch(17% 0.05 250)` | Code blocks, infobox |
| `--bg-overlay` | `oklch(19% 0.05 250)` | Dropdowns, modals |
| `--bg-hover` | `oklch(21% 0.05 250)` | Hover/active states |
| `--fg-2` | `oklch(72% 0.02 250)` | Secondary text |
| `--fg-3` | `oklch(58% 0.02 250)` | Tertiary/metadata |

Elevation layers are the main visual gap. Tokens may exist but are not applied to surfaces.

## WCAG 2.1 AA — confirmed numbers

- Contrast: 4.5:1 body text; 3:1 large text (≥18pt or ≥14pt bold); 3:1 non-text elements
- Line spacing: ≥1.5× font size (our 1.78 target ✓)
- Paragraph spacing: page must not break when user sets to 2× font size (WCAG 1.4.12 — NOT a mandate for default 2× spacing)
- Max line length: 80 chars (our 70ch ✓)
- NO justified text — text-align: left only
- Touch targets: ≥44×44 CSS pixels
- Reflow at 320px: no 2D scroll (Playwright R1 checks this ✓)
- Focus visible: ≥2px ring, ≥3:1 contrast against adjacent color
- No user-scalable=no in viewport meta (WCAG 1.4.4)

## EN 301 549

Fully adopts WCAG 2.1 AA. One addition: biometric access — not applicable to a wiki.
Next version (2026) will include WCAG 2.2 AA.

## Section 508

References WCAG 2.0 AA. WCAG 2.1 AA is strict superset — satisfying 2.1 satisfies 508.

## Masthead research

**Gap confirmed:** current masthead treats search, wordmark, controls at equal visual weight.

**Premium docs pattern (Stripe, Vercel, Linear docs):** search dominates center as hero element. Wordmark is compact left anchor (≤28px height). Controls are subtle right cluster.

**Specific values:**
- Search pill: border-radius 20px; height 40px; rgba(255,255,255,0.12) bg on dark masthead
- Search focus: rgba(255,255,255,0.22) bg; rgba(255,255,255,0.5) border; accent outline
- Placeholder text: per-tenant contextual ("Search documentation…" not generic)

## Home page editorial hero research

**Gap confirmed:** featured article is a contained card (Wikipedia-ish), not an editorial hero.

**Guardian/NYT pattern:**
- Full-bleed background image from `hero_image` frontmatter
- Dark gradient overlay: `linear-gradient(to top, rgba(0,0,0,0.85) 0%, rgba(0,0,0,0.4) 50%, transparent 100%)`
- Category label in small caps above headline (Guardian kicker pattern)
- Featured title: `clamp(2.2rem, 4vw, 3.5rem)` Playfair Display
- Excerpt: 52ch max-width, rgba(255,255,255,0.88)
- CTA: minimal inline "Read →" (not a separate button)
- Min height: 360px to give the hero room

## TOC active tracking

**Gap confirmed:** no IntersectionObserver active-section highlighting.

**Pattern (used by Wikipedia Vector 2022, Stripe docs, MDN):**
```javascript
const obs = new IntersectionObserver(callback, { rootMargin: '-20% 0px -75% 0px' });
```
rootMargin fires "active" when heading enters the upper 80% of viewport and leaves the
lower 25% — creates a "sticky" feel where current section stays highlighted while reading.

## Reading time

**Gap confirmed:** `.reading-time` has `data-words="0"` — JS never populates.
**Fix:** count `.prose.textContent.trim().split(/\s+/).length` → `Math.ceil(words / 228)` WPM.

## Talk tab — regulated industry research

**Confirmed:** inline comments > separate Talk pages for regulated industries.
Sources: SharePoint (SOC 1/2/3, ISO 27001, HIPAA BAA, FedRAMP), Guru ("the enterprise governance play"), Confluence, Slab, Notion.

**Pattern:** named author + timestamp + body tied to specific content passage + resolution status + export capability. This is what compliance auditors look for.

**Decision (2026-06-29, operator-confirmed):** Option B — full inline annotation system. Threaded comments tied to heading anchors, git-committed YAML sidecars, named author + ISO 8601 timestamp, resolution status. New `BRIEF-inline-annotations.md`.

## SOC3/trust signals

From AICPA guidelines:
- Always "SOC 3 Type II attestation" — never "certified" or "SOC 2 certified"
- AICPA logo: display unmodified; link to aicpa.org/soc4so; no resize/recolor
- Badge valid 12 months from report date; unqualified reports only
- Display near footer copyright + alongside other standards (ISO 27001, WCAG 2.1 AA)

"Authoritative record" design language (Cloudflare, Stripe, USWDS pattern):
- Specificity over marketing: named frameworks, version numbers, third-party verification links
- "Quiet confidence": restrained color; illustrative icons not certification seals
- Technical precision: specific standards (WCAG 2.1 AA, EN 301 549) not marketing language
- Transparency: git SHA visible near article "Last edited" date

## USWDS (government alignment)

USWDS (U.S. Web Design System): mandated for federal websites under 21st Century IDEA Act.
Typography: Source Sans Pro + Merriweather. Color: blue/gray/white/red system.
40+ accessible components built in. Our platform's visual alignment to this system is
intentional for government-facing use cases.

## Decisions locked (2026-06-29)

1. **No rebuild** — sovereign engine is correct; gaps are CSS/HTML
2. **Typography:** 18px/1.78/1.6em/70ch prose; fluid heading scale; mobile max(16px,1rem)
3. **Dark mode:** chromatic navy elevation layers applied to surfaces
4. **Masthead:** search dominant center (height 40px, pill shape); wordmark compact left
5. **Home hero:** full-bleed featured with gradient overlay when hero_image present
6. **TOC:** IntersectionObserver active-section with rootMargin '-20% 0px -75% 0px'
7. **Reading time:** JS count from .prose textContent ÷ 228 WPM
8. **Talk tab → inline annotations:** Option B (full system) per operator 2026-06-29
9. **Records signals:** git SHA in article chrome; "Git-versioned content · WCAG 2.1 AA · EN 301 549" in footer legal
10. **SOC3:** when attestation exists, use "SOC 3 Type II attestation" language per AICPA

## Alternatives considered and rejected

- **Full rebuild from scratch** — rejected: sovereign engine advantage irreversible if discarded; gaps are CSS not architecture
- **Talk tab rename only (Option A)** — rejected: compliance research shows inline comments > separate pages for regulated industries
- **Pure black dark mode (#000 or #111)** — rejected: "infinite void" effect; chromatic navy creates brand identity in dark mode (GitHub precedent)
- **Justified text** — rejected: WCAG 1.4.8 requirement; "never use justified text on 320px"
- **Dark mode toggle as JavaScript-only** — already handled: `html[data-theme="dark"]` attribute path in wiki.js
