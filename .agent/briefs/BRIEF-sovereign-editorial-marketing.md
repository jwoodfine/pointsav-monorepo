---
artifact: brief
schema: foundry-brief-v1
status: reference
brief-id: project-knowledge-sovereign-editorial-marketing
owner: project-knowledge
destination: project-marketing
created: 2026-06-24
updated: 2026-07-09
parent: project-knowledge-sovereign-editorial
---

# BRIEF — Sovereign Editorial Redesign: project-marketing

> **For:** project-marketing Totebox session
> **Engine:** `app-mediakit-marketing`
> **Sites:** home.woodfinegroup.com (media-marketing-landing-1) + home.pointsav.com (media-marketing-landing-2)
> **Design authority:** project-knowledge holds master design research. This BRIEF
> derives from `BRIEF-phase2-redesign.md` in project-knowledge. Request updates via outbox.

---

## Context

2026-06-24 browser-in-the-loop audit (6 sites × 6 viewports × 3 tracks, 7-agent workflow).

**Scores:**
- home.woodfinegroup.com: **3/10**
- home.pointsav.com: **3/10**

Both sites are functional but nowhere near hyperscaler grade. The same structural
failures appear on both — they share the same binary and same chrome paths.

---

## Audit Findings — home.pointsav.com (3/10)

**Critical (ship-blocking):**
- Variable fonts not loading — system-font fallback everywhere. `Barlow Condensed` / `Oswald`
  referenced in CSS but no `@font-face` declarations exist. All text renders in system sans.
- HTML `lang` attribute missing — screen readers cannot determine document language.
- Sub-pages (`/page/contact`, `/page/disclaimer`) locked to fixed-pixel widths (~1440px / ~913px).
  Horizontal scroll at **320, 375, 768, 1024** on both pages.
- Sovereign Editorial chrome not implemented: white masthead (target: dark navy #164679),
  wordmark centred (target: left-aligned), no search bar, no utility controls in correct positions.
- Footer is a thin light-grey strip; no dark background, no wordmark, no three-column nav.

**Major:**
- No mobile hamburger nav — at 768px and below, nav links disappear with no replacement.
- Color-contrast axe violations: 37 nodes at 375 + 1440 combined.
- Missing `<h1>` on home page.

---

## Audit Findings — home.woodfinegroup.com (3/10)

**Critical:**
- `/es` route disables pinch-to-zoom (`user-scalable=no` or `maximum-scale=1` in meta-viewport)
  — **WCAG 1.4.4 critical violation**. Must be removed immediately.
- Sub-page fixed-width containers (~1144–1180px) cause horizontal scroll at 320/375/768/1024
  on `/page/contact` and `/page/disclaimer`. Footer trademark block word-wraps to 1-word/line
  at mobile — thousands of pixels tall.
- Header architecture does not match Sovereign Editorial at any viewport: light-grey masthead,
  dark-on-light wordmark, no search field, chrome split into two disconnected rows.
- Variable fonts not loaded (same as pointsav site).

**Major:**
- No mobile hamburger nav.
- Color-contrast axe violations: 32 nodes.
- `html-has-lang` missing on home route.

---

## "Sovereign Editorial" Design Direction

*Shared with project-knowledge. project-knowledge is the design-direction master.
Request clarification or updates via outbox.*

### Dark authority masthead

64px navy (#164679) header. White wordmark **left-aligned**. Search bar **centre**.
Utility controls **right** (language toggle EN|ES, theme toggle). Navigation moves to a
48px secondary bar below the masthead.

```
┌──────────────────────────────────────────────────────────────┐
│ [WORDMARK]    [ 🔍  Search… _________________ ]  [EN|ES] [☀] │  64px · navy #164679
└──────────────────────────────────────────────────────────────┘
┌──────────────────────────────────────────────────────────────┐
│  Home   About   Contact                                        │  48px · scrolls
└──────────────────────────────────────────────────────────────┘
```

Mobile (≤768px): masthead collapses to wordmark + hamburger. Secondary bar hides.
Off-canvas drawer for nav links.

### Variable font trio (self-hosted, zero CDN)

| Role | Font | Variable axes | Settings |
|---|---|---|---|
| Display (h1/hero) | **Playfair Display Variable** | `wght: 400–900` | `clamp(2rem, 5vw, 3.5rem)` · `wght: 700–800` |
| Body text | **IBM Plex Sans Variable** | `wght: 200–700` · **`opsz: 8–72`** · `wdth: 75–100` | `1.0625rem` · `line-height: 1.7` · `opsz: 16` |
| Code / mono | **IBM Plex Mono** | — | unchanged |

Download WOFF2 from Google Fonts → `static/fonts/`. Add `@font-face` with `font-display: swap`.
Wire CSS custom properties:
```css
--font-display: 'Playfair Display Variable', Georgia, serif;
--font-body: 'IBM Plex Sans Variable', system-ui, sans-serif;
--font-mono: 'IBM Plex Mono', monospace;
```

Remove `Barlow Condensed` and `Oswald` references — they are never loaded.

### Footer anatomy — legally correct

```
┌──────────────────────────────────────────────────────────────┐
│  (near-black #0e1117 bg)                                      │
│  [Wordmark]      Site Map         Legal & Policy              │
│  [Tagline]       Home             Privacy                     │
│                  About            Disclaimer                  │
│                  Contact          Accessibility               │
│──────────────────────────────────────────────────────────────│
│  Vancouver | New York | Berlin                                │
│  Contact us  ·  Disclaimer  ·  Privacy                       │
│──────────────────────────────────────────────────────────────│
│  © 2026 Woodfine Capital Projects Inc. All rights reserved.  │
│  [trademark line — see below]           [EN | ES]  [☀]      │
└──────────────────────────────────────────────────────────────┘
```

**Copyright holder: Woodfine Capital Projects Inc.** — NOT PointSav Digital Systems, NOT
Woodfine Management Corp. Source: TRADEMARK.md v1.1 (2026-05-16).
All marks are **™** (unregistered common-law) — NOT ®.

*Woodfine-brand trademark line* (home.woodfinegroup.com):
> Woodfine Capital Projects™, Woodfine Management Corp™, PointSav Digital Systems™,
> Totebox Orchestration™, and Totebox Archive™ are trademarks of Woodfine Capital
> Projects Inc., used in Canada, the United States, Latin America, and Europe.
> All other trademarks are the property of their respective owners.

*PointSav-brand trademark line* (home.pointsav.com):
> PointSav Digital Systems™, Totebox Orchestration™, and Totebox Archive™ are
> trademarks of Woodfine Capital Projects Inc., used in Canada, the United States,
> Latin America, and Europe. All other trademarks are the property of their
> respective owners.

Trademark line must be separate from copyright line (TRADEMARK.md §7). Use a `Brand` enum
(`Brand::PointSav` | `Brand::Woodfine`) to select the correct string at render time.

### Zero-cookie posture

Platform has ZERO cookies architecturally. No cookie banner. No consent UI.
Footer carries "Privacy" link only → `/page/privacy`. Full disclosure on that page.
Beacon: `navigator.sendBeacon('/_beacon', JSON.stringify({u: pathname, t: ms}))` — URL + timestamp only.

---

## Brand tokens (marketing sites)

| Property | home.woodfinegroup.com | home.pointsav.com |
|---|---|---|
| `--topnav-bg` | `#164679` (navy) | `#164679` (navy) |
| `--footer-bg` | `#0e1117` (near-black) | `#0e1117` (near-black) |
| `--accent` | `#c9a84c` (warm gold) | `#C7A961` (gold) |
| `--wordmark-color` | `#ffffff` (white on dark) | `#ffffff` (white on dark) |

Use `data-brand="woodfine"` / `data-brand="pointsav"` on `<html>` to switch token sets.

---

## Implementation Order (app-mediakit-marketing)

| # | Action | Effort |
|---|---|---|
| 1 | Remove `user-scalable=no` / `maximum-scale=1` from ALL meta-viewport tags | XS |
| 2 | Fix sub-page horizontal scroll — remove fixed-pixel widths from Rust templates and CSS | S |
| 3 | Add `lang` attribute to all `<html>` elements; add `<h1>` to home pages | S |
| 4 | Add `--topnav-bg: #164679` + `--footer-bg: #0e1117` tokens; update CSS rules | S |
| 5 | Write ONE shared chrome maud module with Tenant enum dispatch — see architecture below | M |
| 6 | Write `sovereign_footer()` — near-black bg, correct WCP Inc. legal text, brand enum, Privacy link | M |
| 7 | Mobile hamburger nav — off-canvas drawer, JS toggle in static JS | M |
| 8 | Font stack — download Playfair Display Variable + IBM Plex Sans Variable WOFF2; @font-face; token wiring | M |
| 9 | Fix WCAG color-contrast failures (32–37 axe nodes) | M |
| 10 | Regression armor — viewport tests at 320/375/768/1440 | S |

---

## Chrome architecture — shared base + per-Tenant dispatch

**The same pattern used in project-knowledge's wiki sites.** Sites share structural chrome
code now and can diverge independently later — no forking required.

```rust
pub enum Tenant { Woodfine, PointSav }

pub fn sovereign_chrome(tenant: &Tenant, content: Markup) -> Markup { … }

impl Tenant {
    fn nav_links(&self)     -> Vec<NavLink>  { … }  // different per site
    fn brand_tokens(&self)  -> BrandTokens   { … }  // different accent colors
    fn wordmark(&self)      -> &str          { … }
    fn trademark_line(&self) -> &str         { … }  // critical: different legal string per brand
}
```

**Trademark line MUST be per-Tenant** (different strings for Woodfine-brand vs PointSav-brand
sites — see "Footer anatomy" section above). This is the main structural difference between
the two marketing sites. Everything else (masthead layout, hamburger, footer skeleton) is
shared structure.

When one marketing site needs to evolve its chrome independently later (different hero layout,
different nav structure), add a method to `Tenant` impl — no need to fork the module.

**Systemd service naming — ONE rename required:**

```
CURRENT (asymmetric — defect):
  local-marketing.service          → home.woodfinegroup.com
  local-marketing-pointsav.service → home.pointsav.com

TARGET (consistent):
  local-marketing-woodfine.service → home.woodfinegroup.com
  local-marketing-pointsav.service → home.pointsav.com
```

Convention is `local-<engine>-<tenant>.service`. `local-marketing.service` has no tenant
suffix — rename to `local-marketing-woodfine.service`. Rename procedure:
1. Write new unit file at `/etc/systemd/system/local-marketing-woodfine.service` (copy from existing)
2. `sudo systemctl daemon-reload`
3. `sudo systemctl enable local-marketing-woodfine.service`
4. `sudo systemctl start local-marketing-woodfine.service`
5. Verify live at home.woodfinegroup.com
6. `sudo systemctl stop local-marketing.service && sudo systemctl disable local-marketing.service`
7. Remove old unit file

Do this before or alongside the chrome implementation — not as a separate later task.

---

## BCSC note

corporate.woodfinegroup.com: DISCLAIMER.md §5 forward-looking statement required on any
page with planned features, timelines, or intended capabilities. Applied silently.
home.* sites: no BCSC forward-looking statement required unless page contains specific
planned product commitments.

---

## Coordination

- **Design authority:** project-knowledge. Send questions or design conflicts via outbox
  `to: totebox@project-knowledge`.
- **Deploy path:** project-marketing builds `app-mediakit-marketing`, deploys to
  `media-marketing-landing-1` and `media-marketing-landing-2`. Command handles Stage 6 merge.
- **Privacy pages:** project-editorial committed `page-privacy.md` + `page-privacy.es.md`
  to content repos (or pending — check inbox). Same privacy page content serves both sites.
