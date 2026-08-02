---
artifact: brief
schema: foundry-brief-v1
status: reference
brief-id: project-knowledge-sovereign-editorial-software
owner: project-knowledge
destination: project-software
created: 2026-06-24
updated: 2026-07-09
parent: project-knowledge-sovereign-editorial
---

# BRIEF — Sovereign Editorial Redesign: project-software

> **For:** project-software Totebox session
> **Engine:** `app-privategit-source` (port 9201) + `app-privategit-marketplace` (port 9202)
> **Site:** software.pointsav.com (vault-privategit-software-1)
> **Design authority:** project-knowledge holds master design research. This BRIEF
> derives from `BRIEF-phase2-redesign.md` in project-knowledge. Request updates via outbox.

---

## Context

2026-06-24 browser-in-the-loop audit (6 sites × 6 viewports × 3 tracks).

**Score: 2/10** — lowest of all 6 live sites.

`software.pointsav.com` is a categorically different product from the wiki and marketing
sites. It is a binary distribution catalog + crypto-payment gateway. The chrome architecture
may differ substantially from `app-mediakit-knowledge`. Inspect the engine's src/chrome or
template structure before implementing — do not assume a maud module structure identical to
the wiki sites.

---

## Audit Findings (2/10)

**Critical (ship-blocking):**

- **Complete mobile breakdown.** Horizontal scroll on the home page at **320, 375, 768, 1024**.
  The layout is fixed-width and collapses entirely at any viewport below 1440px. At 320px
  the page is approximately 2× the viewport width — the user cannot see the product catalog.
- **`/page/contact` returns HTTP 0** (connection refused or 404 routing error). The "CONTACT US"
  link in the nav is dead. This is customer-facing and directly harms product inquiries.
- **74 axe violation nodes** at 375 + 1440 combined — highest of any of the 6 sites excepting
  the wiki sites which have more pages. On a 2-page site this is severe per-page density.
- **Sovereign Editorial chrome not present.** White/light masthead with fixed-pixel widths,
  system fonts only, no dark authority masthead.
- **Footer is missing** or is a thin minimal strip — no legal text, no trademark line,
  no copyright block visible in screenshot analysis.

**Major:**
- No mobile hamburger nav — nav disappears at tablet and below.
- Color-contrast failures (estimated from axe node count).
- No `lang` attribute on `<html>`.

---

## "Sovereign Editorial" Design Direction

*Shared with project-knowledge. project-knowledge is the design-direction master.*

### Dark authority masthead

64px navy (#164679) header. White wordmark **left-aligned**. Search bar or product search
**centre**. Utility controls **right** (for software.pointsav.com: account/license status
link, language toggle EN|ES).

```
┌──────────────────────────────────────────────────────────────┐
│ [PointSav Software]  [ 🔍 Search products… ]   [Account] [☀] │  64px · navy #164679
└──────────────────────────────────────────────────────────────┘
┌──────────────────────────────────────────────────────────────┐
│  Products   Downloads   Pricing   Documentation               │  48px · scrolls
└──────────────────────────────────────────────────────────────┘
```

Mobile (≤768px): masthead collapses to wordmark + hamburger. Off-canvas drawer for links.

### Variable font trio (self-hosted, zero CDN)

| Role | Font | Variable axes | Settings |
|---|---|---|---|
| Display (h1/hero) | **Playfair Display Variable** | `wght: 400–900` | `clamp(2rem, 5vw, 3.5rem)` · `wght: 700–800` |
| Body text | **IBM Plex Sans Variable** | `wght: 200–700` · **`opsz: 8–72`** · `wdth: 75–100` | `1.0625rem` · `line-height: 1.7` · `opsz: 16` |
| Code / mono | **IBM Plex Mono** | — | Use for product version strings, hashes, checksums |

For a software distribution site, IBM Plex Mono has extra utility: display version numbers,
SHA-256 checksums, and license keys in mono to communicate technical authority.

Download WOFF2 from Google Fonts → `static/fonts/` (or equivalent in the engine structure).

### Footer anatomy — legally correct

```
┌──────────────────────────────────────────────────────────────┐
│  (near-black #0e1117 bg)                                      │
│  [PointSav Software™]    Products         Legal & Policy      │
│  [Tagline]               Downloads        Privacy             │
│                          Pricing          Disclaimer          │
│                          Support          Accessibility       │
│──────────────────────────────────────────────────────────────│
│  Vancouver | New York | Berlin                                │
│  Contact us  ·  Disclaimer  ·  Privacy                       │
│──────────────────────────────────────────────────────────────│
│  © 2026 Woodfine Capital Projects Inc. All rights reserved.  │
│  PointSav Digital Systems™, Totebox Orchestration™, and      │
│  Totebox Archive™ are trademarks of Woodfine Capital         │
│  Projects Inc., used in Canada, the United States, Latin     │
│  America, and Europe. All other trademarks are the property  │
│  of their respective owners.                                  │
│                                               [EN | ES]  [☀] │
└──────────────────────────────────────────────────────────────┘
```

**Copyright holder: Woodfine Capital Projects Inc.** — NOT PointSav Digital Systems.
Source: TRADEMARK.md v1.1 (2026-05-16).
All marks are **™** (unregistered common-law) — NOT ®.

The trademark line for PointSav-brand sites (verbatim, from TRADEMARK.md v1.1):
> PointSav Digital Systems™, Totebox Orchestration™, and Totebox Archive™ are trademarks
> of Woodfine Capital Projects Inc., used in Canada, the United States, Latin America, and
> Europe. All other trademarks are the property of their respective owners.

### Zero-cookie posture

Platform has ZERO cookies — architecturally absent. No cookie banner, no consent UI.
The "Privacy" link in the footer leads to `/page/privacy`.

**Exception for software.pointsav.com:** if `app-privategit-marketplace` sets session
tokens (for account/license authentication), those are first-party auth tokens, not
tracking cookies. DATA-POLICY.md covers analytics only; auth session handling is governed
by the service-level security design. Verify with project-software lead before assuming
zero-cookie applies to the session layer.

---

## Brand tokens (software site)

| Property | software.pointsav.com |
|---|---|
| `--topnav-bg` | `#164679` (navy) |
| `--footer-bg` | `#0e1117` (near-black) |
| `--accent` | `#C7A961` (gold) |
| `--wordmark-color` | `#ffffff` (white on dark) |

---

## Software-site specific design notes

`software.pointsav.com` is NOT just a marketing site with different content — it has
product-specific UX requirements:

1. **Product catalog page** — must be responsive grid at all 6 viewports. Cards for each
   product: name, version, price, license type, download CTA. Pricing table needs special
   care at mobile (consider stacked layout for pricing tiers at ≤768px).

2. **Download/release page** — version strings, SHA-256 checksums, platform badges
   (Linux, macOS, Windows). IBM Plex Mono for all technical values. Copy-to-clipboard button.

3. **License key display** — monospace, large, selectable, with a copy button.

4. **Crypto-payment flow** — USDC / Polygon. Any payment UI must work at mobile. Wallet
   connect dialogs must be responsive. This is a distinct UX surface that the wiki/marketing
   sites don't have.

The Sovereign Editorial direction applies to the chrome (masthead + footer + typography).
Product-specific pages within that chrome may have their own layout requirements — do not
force the wiki-style 65ch article column onto a product catalog page.

---

## Implementation Order (app-privategit-source / app-privategit-marketplace)

| # | Action | Effort |
|---|---|---|
| 1 | Fix `/page/contact` route — HTTP 0 means the route is missing or the URL resolves to the wrong port | XS |
| 2 | Fix horizontal overflow — remove ALL fixed-pixel widths from CSS and templates | S |
| 3 | Add `lang` attribute to `<html>` on all routes | XS |
| 4 | Add `--topnav-bg: #164679` + `--footer-bg: #0e1117` tokens; update CSS | S |
| 5 | Write Sovereign Editorial chrome module — dark masthead, nav bar; port to whatever template system the engine uses (maud, Tera, or other) | M |
| 6 | Write `sovereign_footer()` — near-black bg, correct WCP Inc. legal text (PointSav brand string), Privacy link | M |
| 7 | Mobile hamburger nav — off-canvas drawer at ≤768px | M |
| 8 | Font stack — download Playfair Display Variable + IBM Plex Sans Variable + IBM Plex Mono WOFF2; `@font-face`; token wiring | M |
| 9 | Fix 74 axe violation nodes — prioritize contrast failures first | M |
| 10 | Regression armor — viewport smoke tests at 320/375/768/1440 | S |

**Prerequisite step 0:** Audit how many chrome paths exist in `app-privategit-source` and
`app-privategit-marketplace`. They may each have their own chrome (the marketplace has
payment flow pages that are structurally different). Map the chrome paths before implementing
the sovereign chrome module. Send findings to project-knowledge via outbox.

## Chrome architecture — shared base + per-Tenant dispatch

**The same pattern used across all three archives.** Structural chrome is shared code;
per-tenant data drives variation. Sites can diverge independently without forking.

```rust
// For vault-privategit-software-1, tenant is always PointSav — but there are two binaries.
// Each binary (source + marketplace) may have different chrome needs.
pub enum SoftwareSurface { Source, Marketplace }

pub fn sovereign_chrome(surface: &SoftwareSurface, content: Markup) -> Markup { … }

impl SoftwareSurface {
    fn nav_links(&self)      -> Vec<NavLink>  { … }  // source: product nav; marketplace: account nav
    fn brand_tokens(&self)   -> BrandTokens   { … }  // both PointSav gold, but layout may differ
    fn show_account_nav(&self) -> bool         { … }  // marketplace has auth/account UI; source doesn't
}
```

**Pattern:** `SoftwareSurface` replaces the `Tenant` enum used in the wiki and marketing
binaries — software.pointsav.com is always PointSav-brand, but the two binaries (source
distribution vs. marketplace/payment) are the dimension of variation.

When `app-privategit-marketplace` needs payment flow pages with a stripped-down chrome
(no nav, just wordmark + footer), override via `SoftwareSurface::Marketplace` without
touching `Source`. No forking required.

**Systemd service naming (confirmed correct — no rename needed):**
- `local-software-source.service` → app-privategit-source (port 9201)
- `local-software-marketplace.service` → app-privategit-marketplace (port 9202)

Both follow the `local-<engine>-<function>.service` convention. No rename required.

---

## Architecture note

`vault-privategit-software-1` runs TWO binaries:
- `app-privategit-source` (port 9201) — binary streaming + release MANIFEST
- `app-privategit-marketplace` (port 9202) — product catalog + payment + license issuance

The public URL `software.pointsav.com` is likely proxied by nginx to one or both.
The chrome implementation may need to be done in both binaries, or shared via a common
crate. Confirm the routing before implementing — which port does the home page hit?

---

## Coordination

- **Design authority:** project-knowledge. Send questions or design conflicts via outbox
  `to: totebox@project-knowledge`.
- **Deploy path:** project-software builds and deploys to `vault-privategit-software-1`.
  Command handles Stage 6 merge.
- **Privacy page content:** project-editorial committed `page-privacy.md` + `page-privacy.es.md`
  for the wiki content repos. A separate privacy page may be needed for software.pointsav.com
  if it has its own content repo — check with project-editorial via outbox.
