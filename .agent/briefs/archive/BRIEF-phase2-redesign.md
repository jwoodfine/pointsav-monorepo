---
artifact: brief
schema: foundry-brief-v1
status: archived
brief-id: project-knowledge-phase2-redesign
owner: project-knowledge
created: 2026-06-23
updated: 2026-07-12
superseded-by: project-knowledge-ng-rewrite
---

## ARCHIVED 2026-07-12 — fully superseded, historical record only

Flipped from `reference` to `archived` as part of BRIEF-knowledge-ng-rewrite's P9
consolidation pass. Every file/module this BRIEF names — `wiki_handlers.rs`,
`chrome/mod.rs`, `home_handlers.rs`, `misc_handlers.rs`, `app-mediakit-shell` — belonged
to the pre-P8 engine, which was retired wholesale at the P8 cutover (commit `531d3144`,
"100% new code," 2026-07-02). None of those files exist in the current codebase; this
BRIEF's "still pending" items are not pending against anything that still exists.

Its `reference`-holding condition ("kept until foundry-prod's other consumers catch up")
was itself resolved sessions ago — P8 has been confirmed complete and live in production
since 2026-07-09 (see BRIEF-knowledge-ng-rewrite's own 2026-07-09 status update). This
BRIEF's real historical value — the P0/P1 accessibility-audit findings and the
Bloomberg/FT editorial-authority design anchor decision — predates and informed the
ng-rewrite's own chrome design; kept per brief-discipline (never deleted), superseded by
`BRIEF-knowledge-ng-rewrite.md` for all current and future work.

# BRIEF — Phase 2 Live-Site Redesign

## Context

50-agent browser-in-the-loop audit of all 5 live sites completed 2026-06-20.
Evidence at `.agent/audit/2026-06-20/`. Chosen design anchor: Editorial authority (Bloomberg/FT).
Sites scored 1.4–2.1/5 overall. Root cause: two engines sharing zero CSS; shared
`vendor/pointsav-design-system` exists, is WCAG-2.2-AAA, and is entirely unused.

**Six live sites — three archives:**

| URL | Engine | Deployment instance | Archive |
|---|---|---|---|
| documentation.pointsav.com | app-mediakit-knowledge | media-knowledge-documentation-1 (:9090) | **project-knowledge** (here) |
| projects.woodfinegroup.com | app-mediakit-knowledge | media-knowledge-projects-1 (:9093) | **project-knowledge** (here) |
| corporate.woodfinegroup.com | app-mediakit-knowledge | media-knowledge-corporate-1 (:9095) | **project-knowledge** (here) |
| home.woodfinegroup.com | app-mediakit-marketing | media-marketing-landing-1 | **project-marketing** |
| home.pointsav.com | app-mediakit-marketing | media-marketing-landing-2 | **project-marketing** |
| software.pointsav.com | app-privategit-source + app-privategit-marketplace | vault-privategit-software-1 | **project-software** |

**project-knowledge scope:** wiki sites only (documentation + projects + corporate). Marketing
and software sites are owned by their respective archives. Design direction is shared;
implementation is per-archive.

**Cross-archive coordination:** project-knowledge holds the master Sovereign Editorial design
research (fonts, tokens, footer anatomy, trademark text, audit findings). Design direction
is relayed to project-marketing and project-software via outbox on each implementation
milestone so those archives can work in parallel.

Separate BRIEFs maintained at:
- `.agent/briefs/BRIEF-sovereign-editorial-marketing.md` — project-marketing scope; sent 2026-06-24
- `.agent/briefs/BRIEF-sovereign-editorial-software.md` — project-software scope; sent 2026-06-24

**CMS verdict (2026-06-24):** Keep custom engine. The problem is CSS + chrome inconsistency,
not engine capability. A CMS migration is 3–6 months and lands in the same design state.
Doctrine prohibits third-party content runtime dependencies.

**Source of truth (important):** the live wiki source is the NESTED
`clones/project-knowledge/pointsav-monorepo/app-mediakit-knowledge/` (1654-line style.css).
The top-level `app-mediakit-knowledge/` (1311-line CSS) is STALE — never edit it.

---

## P0 — Structurally Broken (fix first)

| # | Issue | Status | Commit |
|---|---|---|---|
| P0-1 | Desktop article-column collapse — prose starved to one word/line; fixed 14em TOC floated over H1 | ✅ LIVE | `49be4356` |
| P0-2 | Mobile horizontal overflow from `<code>` blocks on ~15 docs pages at 375px | ✅ LIVE (foundry-workspace) / 🟡 foundry-prod pending | `39602246` |
| P0-3 | Marketing `/page/*` templates fixed-width, clip 538–769px off-screen at tablet | ✅ LIVE | `cc3922f6` (rebuilt 14:41 2026-06-22) |
| P0-4 | Corporate `/search` + `/category` leaking PointSav/dev chrome onto Woodfine domain | ✅ LIVE | `91b3ba7f` (rebuilt 2026-06-23) |
| P0-5 | `/es` 404 + `maximum-scale` zoom disabled on both marketing sites | ✅ LIVE | `dcd65b3a` (pre-existing) |

**P0-2 fix shipped 2026-06-23:** `overflow-wrap: anywhere` added to `code, kbd, samp` rule in style.css. Committed `39602246`. Stage 6 + rebuild in outbox (`project-knowledge-20260623-stage6-p1-p2-batch`). `.prose pre` already had `overflow-x: auto` — fenced blocks unaffected.

---

## P1 — Hyperscaler-Credibility Gaps

### P1a — Landmark scaffold (both engines)

**Issue:** ~820 wiki pages and all marketing `/page/*` templates have no `<main>`, no single `<h1>`,
no nav landmarks, no skip-to-content. axe findings: page-has-heading-one (1,020 nodes),
landmark-one-main (158 + 148), region (8 + 12). WCAG 1.3.1 / 2.4.1 / 2.4.6 Level A failures.

**Shipped + VERIFIED LIVE on foundry-workspace 2026-06-24 (wiki engine; 4 chrome paths):**

wiki_handlers.rs root cause: article pages and edit page use a fully self-contained inline HTML template (~1400 lines) in `wiki_handlers.rs` — does NOT call through `chrome/mod.rs`. Prior P1a passes missed it entirely. Four separate chrome paths now all fixed:
1. `chrome/mod.rs` nav_bar() + base_page() — `/es/` home and home page
2. `misc_handlers.rs` chrome() + page_handler() — /search, /category, /page/*
3. `home_handlers.rs` home_chrome() — `/` and `/es/`
4. `wiki_handlers.rs` article template + edit template — `/wiki/*` and `/edit/*` (committed `272a9c0a`)

All four return `role="banner"` + `role="contentinfo"` on foundry-workspace (verified 2026-06-24).

**Pending: foundry-prod deploy.** Deploy request sent to Command (msg-id `command-20260624-deploy-request-app-mediakit-knowledge-to`). Code in origin/main at `214fe486`.

**Still pending — app-mediakit-shell (`src/shell.rs`):**
- Same landmark additions for marketing sites (`home.pointsav.com`, `home.woodfinegroup.com`)
- Deferred to next session; routing via project-marketing

**After rebuild:** axe sweep to confirm `aria-hidden-focus` and `landmark-*` violations drop.

### P1b — Design-system token adoption (both engines)

**Issue:** `vendor/pointsav-design-system` CSS custom properties exist, are WCAG-2.2-AAA,
and are entirely unused. axe debt: ~3,087 color-contrast nodes, ~2,713 aria-hidden-focus nodes.
Both engines use hardcoded hex values with insufficient contrast ratios.

**Fix:**
- Import design-system token CSS file (or inline the relevant custom properties) in both
  `app-mediakit-knowledge/static/style.css` and `app-mediakit-shell/static/shell.css`
- Replace all hardcoded `color:`, `background:`, `border-color:` values with design-system tokens
- Tokens to source: text-primary, text-secondary, bg-surface, bg-elevated, accent, border-subtle
  (verify exact names against `vendor/pointsav-design-system/`)
- Verify: all foreground/background pairings meet WCAG AA (4.5:1 text, 3:1 large/UI)

**After:** commit → Stage 6 → rebuild both engines → full axe sweep; target: contrast nodes → 0.

---

## P2 — Zero-Cookie Beacon + Privacy Pages

**Issue:** No site emits the analytics beacon. No privacy page exists on any domain.
BCSC continuous-disclosure posture requires a documented data-collection statement.

**Beacon shipped 2026-06-23 (wiki engine; committed `f1b9c276`):**
- `POST /_beacon` handler in `server/mod.rs` — returns 204 immediately; no cookies, no third-party script
- Inline beacon JS in `chrome/mod.rs head()`, `misc_handlers.rs page_handler`, and `misc_handlers.rs chrome()` — fires `navigator.sendBeacon('/_beacon', JSON.stringify({u: pathname, t: ms}))` on DOMContentLoaded; silent-fail `catch` block
- No data storage on server yet (returns 204 without writing) — Phase 2.5 item: wire to `app-mediakit-telemetry` daemon

**Privacy pages status:**
- Drafts staged at `.agent/drafts-outbound/draft-page-privacy-{en,es}.md`
- Outbox message to project-editorial (`project-knowledge-20260623-privacy-page-draft`) — route commit to all 3 content repos
- `/page/privacy` route already live (no code change needed — `page_handler` reads `page-privacy.md` from content-dir)
- BLOCKED on project-editorial picking up the draft

**Still pending:**
- Marketing engine beacon (`app-mediakit-shell`) — deferred; route via project-marketing
- Privacy link in wiki footer nav (shell_footer) — add after project-editorial commits the pages

**After rebuild + content commit:** verify `curl http://127.0.0.1:9090/page/privacy` → 200 with content.

---

## Regression Armor

**Issue:** The desktop column-collapse (P0-1) was invisible to scrollWidth overflow metrics
and went unnoticed until the 50-agent audit. No automated assertions exist.

**Fix:**
- Add `tests/responsive_test.rs` (or extend existing test suite) in `app-mediakit-knowledge`
- CDP-based assertions at 320/768/1440 viewports:
  1. `scrollWidth === clientWidth` at all 3 viewports (no overflow)
  2. `document.querySelector('main')` is non-null (landmark present)
  3. `document.querySelector('h1')` is non-null (heading present)
  4. `document.querySelector('[data-instance]').dataset.instance` matches expected tenant string
- Run against local server before every Stage 6 promote

---

---

## Phase 3 — "Sovereign Editorial" Radical Redesign

**Operator directive (2026-06-24):** radical change; sites look broken and fall far short
of hyperscaler standard; unify all headers and footers; leapfrog to 2030 design posture.

**Design direction research basis:** Awwwards/Webby/CSS Design Awards 2023–2025 winners
(NASA.gov, British Museum, Guggenheim, MIT CSAIL, Nordiska Museet, FT Financier typeface);
leapfrog institutional exemplars (Guardian 2025, Le Monde, CERN, Der Spiegel, NASA JPL,
Stratechery); 2029–2032 design trend analysis (variable fonts, optical sizing, asymmetric
editorial grid, content-first navigation, container queries).

### Core principle from research

The sites that win awards at institutional scale achieve authority through *discipline*,
not decoration. One coherent type system. A structural grid that breaks the 12-column
template. Mastheads that announce weight immediately. Authority through scarcity of noise.

### Three non-negotiable commitments

**1. Dark authority masthead**

64px navy (#164679) header, white wordmark left, search centre, utility right
(language toggle, theme toggle, account). Navigation decoupled to a 48px secondary bar
that scrolls with the page — contextual, not permanent link noise.

```
┌──────────────────────────────────────────────────────────────┐
│ [WORDMARK]    [ 🔍  Search… _________________ ]  [EN|ES] [☀] │  64px · navy
└──────────────────────────────────────────────────────────────┘
┌──────────────────────────────────────────────────────────────┐
│  Documentation ▾   Getting Started   Reference   Guides  More │  48px · scrolls
└──────────────────────────────────────────────────────────────┘
```

**2. "Sovereign Editorial" variable font trio (all OFL, self-hosted in `static/fonts/`)**

| Role | Font | Variable axes | Settings |
|---|---|---|---|
| Display h1/hero | **Playfair Display** variable | `wght: 400–900` | `clamp(2rem, 5vw, 3.5rem)` · `wght: 700–800` |
| Body text | **IBM Plex Sans Variable** (upgrade) | `wght: 200–700` · **`opsz: 8–72`** · `wdth: 75–100` | `1.0625rem` · `line-height: 1.7` · `max-width: 65ch` · `opsz: 16` |
| Code / mono | **IBM Plex Mono** | keep current | unchanged |

Reference pedigree: BBC pairs IBM Plex body + custom serif display. FT uses Financier
(high-contrast serif) + MetricWeb. Our stack is the open-source institutional equivalent.

`Barlow Condensed` and `Oswald` references in `static/style.css` are **never loaded**
(no `@font-face` declarations) — causing system-font fallback on all navbars. Remove.

Install: download WOFF2 from Google Fonts → `static/fonts/` → `@font-face` in `style.css`
with `font-display: swap`. Wire CSS custom properties:
```css
--font-display: 'Playfair Display', Georgia, serif;
--font-body: 'IBM Plex Sans Variable', system-ui, sans-serif;
--font-mono: 'IBM Plex Mono', monospace;
```

**3. Named CSS Grid areas (replace 12-column template)**

```css
grid-template-areas: "sidebar content aside";
grid-template-columns: 220px minmax(65ch, 75ch) 280px;
/* mobile: single column; sidebar → drawer */
```

Home / category: asymmetric bento card grid.
Article: `content` full-bleed; `aside` for metadata/related.
Container queries: `container-type: inline-size` on card components.

### Chrome unification — 4 paths → 1 shared maud module, per-Tenant dispatch

All four current chrome paths collapse into one `src/chrome/sovereign.rs`:
- `src/chrome/mod.rs` nav_bar() + base_page()
- `src/server/home_handlers.rs` home_chrome()
- `src/server/misc_handlers.rs` chrome() + page_handler()
- `src/server/wiki_handlers.rs` inline topnav (~line 786)

**Architecture: shared base + per-Tenant dispatch.** Sites share structural chrome code now
and can diverge independently later — no forking required.

```rust
pub enum Tenant { Documentation, Projects, Corporate }

// Structural chrome — shared, maintained once
pub fn sovereign_chrome(tenant: &Tenant, content: Markup) -> Markup { … }

// Per-tenant data — different per instance; override when sites need to diverge
impl Tenant {
    fn nav_links(&self)    -> Vec<NavLink>  { … }  // each site has different nav
    fn brand_tokens(&self) -> BrandTokens   { … }  // accent colors differ
    fn wordmark(&self)     -> &str          { … }
    fn hero(&self)         -> Option<Markup> { None } // override per-site later if needed
}
```

What **never** shares: nav links, wordmark, accent color, footer trademark string — these
are per-Tenant config data, not structural code.

What **shares indefinitely**: masthead HTML skeleton, hamburger pattern, footer three-column
layout, legal copyright block structure — the "similar forever" parts.

When a site needs to diverge structurally (e.g., documentation wants a section sidebar that
projects doesn't), add a method to `Tenant` impl — no forking, no flags in shared code.

The `Tenant` is read from config at process startup (already determined by which systemd
service starts the binary — one process per site).

**Systemd service names (confirmed correct — no rename needed):**
- `local-knowledge-documentation.service` → documentation.pointsav.com (:9090)
- `local-knowledge-projects.service` → projects.woodfinegroup.com (:9093)
- `local-knowledge-corporate.service` → corporate.woodfinegroup.com (:9095)
All three follow the `local-<engine>-<tenant>.service` convention.

### Brand differentiation via tokens only

| Site | Masthead | Accent |
|---|---|---|
| documentation.pointsav.com | navy #164679 | gold #C7A961 |
| projects.woodfinegroup.com | navy #164679 | warm gold #c9a84c |
| corporate.woodfinegroup.com | near-black #0e1117 | gold #C7A961 + heavier weight |
| home.pointsav.com | navy #164679 | gold #C7A961 |
| home.woodfinegroup.com | navy #164679 | warm gold #c9a84c |
| software.pointsav.com | separate chrome — audit to establish | — |

### Footer — ONE maud function, legally correct

```
┌──────────────────────────────────────────────────────────────┐
│  (near-black #0e1117)                                         │
│  [Wordmark]      Site Map         Legal & Policy              │
│  [Tagline]       Documentation    Privacy                     │
│                  Projects         Terms                       │
│                  Corporate        Accessibility               │
│──────────────────────────────────────────────────────────────│
│  Vancouver | New York                                         │
│  Contact us  ·  Disclaimer  ·  Privacy                       │
│──────────────────────────────────────────────────────────────│
│  © 2026 Woodfine Capital Projects Inc. All rights reserved.  │
│  [trademark line]                          [EN | ES]  [☀]   │
└──────────────────────────────────────────────────────────────┘
```

**Copyright holder: Woodfine Capital Projects Inc.** — not PointSav Digital Systems.
All marks are **™** (unregistered common-law). Source: TRADEMARK.md v1.1 (2026-05-16).

*PointSav-brand sites* (documentation.pointsav.com, home.pointsav.com, software.pointsav.com):
> PointSav Digital Systems™, Totebox Orchestration™, and Totebox Archive™ are trademarks
> of Woodfine Capital Projects Inc., used in Canada, the United States, Latin America, and
> Europe. All other trademarks are the property of their respective owners.

*Woodfine-brand sites* (projects, corporate, home.woodfinegroup.com):
> Woodfine Capital Projects™, Woodfine Management Corp™, PointSav Digital Systems™,
> Totebox Orchestration™, and Totebox Archive™ are trademarks of Woodfine Capital Projects
> Inc., used in Canada, the United States, Latin America, and Europe. All other trademarks
> are the property of their respective owners.

**Zero-cookie posture — DATA-POLICY.md v1.0 (2026-06-22):**
Zero cookies architecturally (no Set-Cookie headers, no client-side cookie API). No cookie
banner — prohibited. Beacon collects URL pathname + timestamp only. Exempt from GDPR/PIPEDA/
CCPA consent requirements. Footer carries "Privacy" link only — no privacy text in footer
body. Privacy page (`/page/privacy`) carries full disclosure. Bilingual drafts staged at
`.agent/drafts-outbound/draft-page-privacy-{en,es}.md` (ready; waiting on project-editorial).

**corporate.woodfinegroup.com only:** DISCLAIMER.md §5 forward-looking statement required
on any page with planned features, timelines, or intended capabilities. Applied silently.

### Phase 3 Audit Scope

6 live sites × 6 viewports (320/375/768/1024/1440/1920) × 3 tracks:
- Track A: full-page (scores + top 5 issues per site)
- Track B: header-only (topnav zone at all viewports)
- Track C: footer-only (footer zone at all viewports)

Capture script: `.agent/audit/2026-06-24/runner/capture.js`
Evidence: `.agent/audit/2026-06-24/` (screenshots + axe + per-site checks/)

---

## Audit Findings — 2026-06-24

**7-agent workflow analysis. Evidence at `.agent/audit/2026-06-24/`.**

### Per-site scores (1–10)

| Site | Score | Top critical issue |
|---|---|---|
| documentation.pointsav.com | **4/10** | Footer absent on home page at all 6 viewports; dark masthead not implemented |
| corporate.woodfinegroup.com | **4/10** | Same as documentation; HScroll at 1024 on 4/5 pages |
| projects.woodfinegroup.com | **3/10** | HScroll at 320/375/1024; nav disappears at 768 with no hamburger |
| home.pointsav.com | **3/10** | Fixed-width inner containers on all sub-pages; system-font fallback; no Sovereign Editorial |
| home.woodfinegroup.com | **3/10** | /es disables pinch-to-zoom (WCAG 1.4.4 critical); trademark word-wraps to 1-word/line columns |
| software.pointsav.com | **2/10** | Unusable at 320–1024; fixed 1440px scrollWidth; nav off-screen; CONTACT US 404 |

### Cross-site header diagnosis

All six sites share one Rust binary but **four divergent chrome paths** produce different header
HTML on each route type. Every path emits `.topnav` with white background (`--bg-elevated`).
The CSS rule has no dark-masthead token — `--topnav-bg` does not exist in `tokens.css`.

Fastest possible fix (CSS-only, no binary rebuild): add `background: #164679` to `.topnav`
rule + invert wordmark SVG fills to `#ffffff`. Immediately transforms all 6 sites.

At 768px and below: **navigation disappears on all 6 sites with no hamburger replacement**.
Users on tablets and phones have zero navigation access — links silently dropped.

### Cross-site footer diagnosis

Three separate footer emitters produce different HTML per route type:
- `home_handlers.rs` → `shell_footer()`: light grey (`--paper-3`) background
- `wiki_handlers.rs` → `chrome/mod.rs` `full_article_page()`: `--bg-subtle` near-white
- `misc_handlers.rs` → `shell_footer()`: same as home

**All home pages return shot_footer_err across all 6 viewports** — footer either not rendered
or outside viewport clip area. No near-black background, no wordmark, no three-column nav.

**CRITICAL live defect (confirmed in screenshots):** Footer copyright line reads
"© 2026 Woodfine Management Corp.." — TWO errors:
1. Wrong copyright holder: should be "Woodfine Capital Projects Inc." (per TRADEMARK.md v1.1)
2. Double period typo: "Corp.." 

Also: `env!(CARGO_PKG_VERSION)` version string ("v0.1.0") exposed publicly in footer.

### Horizontal scroll summary

| Site | Viewports with HScroll | Pages affected |
|---|---|---|
| software-pointsav | 320, 375, 768, 1024 | home (all mobile/tablet) |
| projects | 320, 375, 1024 | home (all); search/category/disclaimer at 1024 |
| corporate | 320, 375, 1024 | same as projects |
| home-pointsav | 320, 375, 768, 1024 | /page/contact, /page/disclaimer |
| home-woodfine | 320, 375, 768, 1024 | /page/contact, /page/disclaimer |
| documentation | 320 only | /search |

### Top axe violations across all 6 sites

| Rule | Count (nodes) | Severity | Primary cause |
|---|---|---|---|
| color-contrast | 284 | serious | Article metadata, chip labels, muted text below 4.5:1 |
| region | 58 | moderate | Content outside landmarks |
| link-in-text-block | 42 | serious | Inline links not distinguishable without color |
| aria-prohibited-attr | 32 | serious | Invalid ARIA attribute usage |
| aria-hidden-focus | 16 | serious | Focusable elements inside aria-hidden containers |
| meta-viewport | 6 | **critical** | /es/ routes + software contact page: zoom disabled |
| html-has-lang | 4 | serious | Missing `lang` attribute |

### Engine-level vs CSS-level diagnosis

**Primary cause: Rust templates (engine-level).** Four divergent chrome paths produce
structurally different HTML — unfixable with CSS alone. White masthead and broken footer
structure require a unified Rust chrome module.

**Secondary cause: CSS tokens.** No `--topnav-bg` or `--footer-bg` token exists. Adding
these tokens + one CSS rule change is a CSS-only quick-win before the full sovereign.rs rewrite.

### 404 error pages have no chrome

`/wiki/<slug>` returning 404 renders with NO header and NO footer — bare "Page not found"
content only. The error path in `wiki_handlers.rs` bypasses the chrome function entirely.
Must be included in the sovereign.rs unification.

### Prioritized implementation order (synthesis agent, 2026-06-24)

| # | Action | Effort | Rationale |
|---|---|---|---|
| 1 | `src/chrome/sovereign.rs` — single shared maud chrome module | L | Fixes header colour, wordmark, search, nav at all viewports on all routes in one commit |
| 2 | `sovereign_footer()` — near-black bg, WCP Inc. legal text, three-col nav | M | Footer absent or wrong on all 6 sites; fixes copyright defect and version string leak |
| 3 | Add `--topnav-bg: #164679` + `--footer-bg: #0e1117` to tokens.css | S | CSS-only quick-win; can deploy before sovereign.rs ships |
| 4 | Fix HScroll on sub-pages — remove fixed-pixel widths from Rust templates + CSS | M | software-pointsav unusable on all mobile; contact/disclaimer broken on home sites |
| 5 | Mobile hamburger nav — `<button.nav-toggle>` + off-canvas drawer + wiki.js toggle | M | Zero navigation access at 768px and below on all 6 sites |
| 6 | Font stack replacement — Playfair Display Variable + IBM Plex Sans Variable WOFF2 | M | 0% of target fonts loaded anywhere; system-font fallback across all sites |
| 7 | WCAG color-contrast fixes — darken `--fg-3`/`--fg-4`/`--accent` against white bg | M | 284 serious violations; WCAG AA blocker |
| 8 | Add `<h1>` to every page template; fix `aria-prohibited-attr` | S | Missing h1 on every tested page; blocks screen reader outline |
| 9 | Remove `user-scalable=no` from all meta-viewport tags | XS | Critical WCAG 1.4.4 failure; /es/ routes and software contact |
| 10 | Named CSS Grid — `grid-template-areas: 'sidebar content aside'` | M | Sovereign Editorial structural requirement; lower urgency than masthead/footer/fonts |

---

## Execution Order (updated 2026-06-24 — Phase 3 clean-slate redesign)

**Operator directive: clean-slate approach. Do not patch existing CSS or chrome templates.
Write new implementations. Existing files become dead code and are removed, not edited.**

```
[DONE]  P0–P2 structural fixes + Stage 6 (214fe486 in origin/main)
[DONE]  Phase 3 research (design direction, fonts, trademark, privacy posture)
[DONE]  BRIEF updated with Phase 3 direction
[DONE]  Step 1: Browser-in-the-loop audit — 6 sites × 6 viewports × 3 tracks
        Evidence at .agent/audit/2026-06-24/ (150 screenshots + axe + checks/)
        7-agent workflow analysis: scores, cross-site diagnosis, implementation order
[DONE]  Step 2: Audit findings appended to BRIEF (this commit)

[DONE]  Step 3: Sovereign chrome implementation (2026-06-24 — commit b3de7f17)
        3a. NEW src/chrome/sovereign.rs — Tenant enum dispatch; sovereign_nav() + sovereign_footer() + sovereign_page()
            Header: dark navy masthead (#164679, 64px), wordmark left, inline search centre, lang+theme right
            Footer: near-black (#0e1117), three-column, WCP Inc. legal text, zero version string
        3b. All 4 legacy chrome paths replaced:
            home_handlers.rs: sovereign_nav() + sovereign_footer() replacing shell_footer() + old topnav block
            wiki_handlers.rs: sovereign_nav() + sovereign_footer() on article + edit paths
            misc_handlers.rs: sovereign_page() in chrome(); auth_nav_widget dead code removed
            server/mod.rs: use crate::chrome::sovereign::{sovereign_footer, sovereign_nav, sovereign_page, Tenant}
        3c. Fonts NOT yet done — Playfair Display + IBM Plex Sans WOFF2 still pending
        3d. CSS sovereign block appended to style.css (NOT a full replacement — old styles coexist)
            .s-topnav, .s-wordmark, .s-search, .s-controls, .s-footer etc. CSS rules added (end of file)
        3e. tokens.css: --topnav-bg #164679, --topnav-fg, --footer-bg #0e1117, --footer-fg, --footer-border added
        3f. data-brand=(tenant.brand()) on <html> in sovereign_page() ✅
        3g. Regression armor: DONE — commit f8784d93 (jwoodfine 2026-06-24)

        VERIFIED LIVE on foundry-workspace 2026-06-24:
        - curl 9090/9093/9095: role="banner", class="s-wordmark", class="s-search", Woodfine Capital Projects Inc.
        - No engine version string in any response
        - cargo check + cargo build --release PASS, zero warnings from our crate

[DONE]  Step 3c: Variable font stack (commits 04e27bef + 6a7c1188 — 2026-06-24)
        - PlayfairDisplay-Variable-latin.woff2 (38,888 B) + PlayfairDisplay-Variable-italic-latin.woff2 (38,460 B)
          + IBMPlexSans-Variable-latin.woff2 (40,240 B) → app-mediakit-knowledge/static/fonts/
        - @font-face rules added to static/style.css; --font-display + --font-body tokens wired
        - Dead Barlow Condensed + Oswald references removed from style.css (lines 448, 587; replace_all)
        - tokens.css: --font-family-body → IBM Plex Sans; --font-family-display → Playfair Display
        - Preload links updated consistently across ALL 4 chrome paths:
          sovereign.rs sovereign_page() + sovereign_nav() (preload in head)
          home_handlers.rs (lines 295–296)
          wiki_handlers.rs (both occurrences — replace_all)
          chrome/mod.rs head() function (lines 125–128)
        - L23 acceptance test updated (chrome/mod.rs tests): asserts IBMPlexSans-Variable + PlayfairDisplay-Variable
        - VERIFIED LIVE on foundry-workspace 2026-06-24: fonts loading, no system-font fallback

[DONE]  Step 3i: Mobile hamburger nav (commit 46243f5c — 2026-06-24)
        - sovereign_mobile_nav_drawer(tenant, site_title) added to sovereign.rs
          Renders nav#mobile-nav-drawer + div#mobile-nav-overlay; reuses existing wiki.js IDs
        - button.s-hamburger#nav-toggle added to sovereign_nav() (SVG hamburger icon,
          aria-label="Menu", aria-expanded="false", aria-controls="mobile-nav-drawer")
        - .s-hamburger CSS added to style.css (transparent bg, white icon, hover transition)
        - 480px breakpoint fixed: grid-template-columns: auto 1fr auto; s-lang + s-theme-btn hidden;
          hamburger stays visible (was hidden by overly broad s-controls rule — fixed)
        - home_handlers.rs: sovereign_mobile_nav_drawer() call added after sovereign_nav()
        - wiki_handlers.rs: duplicate button#nav-toggle removed from div.mobile-topnav-toggles
          (sovereign_nav() now provides it; article nav#mobile-nav-drawer kept — serves TOC)
        - server/mod.rs: import updated to include sovereign_mobile_nav_drawer
        - test mobile_nav_toggle_button_present was FAILING; root cause: button had class s-hamburger
          but test asserted nav-toggle-btn; FIXED 2026-06-24 by adding nav-toggle-btn as second class:
          class="s-hamburger nav-toggle-btn" (sovereign.rs line 127); retest pending

[DONE]  Steps 3j–3n committed 2026-06-24 as f34f237f (jwoodfine); 135 unit + full integration suite: 0 failures

[DONE]  Step 3j: WCAG color-contrast (2026-06-24)
        - tokens.css: --text-tertiary darkened oklch(53.03%) → oklch(46%) — from ~4.59:1 to ~5.9:1 on white
          Note: override placed in Sovereign Editorial section (after original definition); last-write wins in :root
        - style.css: 4 content-text selectors changed --fg-4 → --fg-3 (was ~2.5:1, now ~5.9:1):
          .recent__date, .browse-list__count, .peer-band__label, .peer-strip__label

[DONE]  Step 3k: Named CSS Grid (2026-06-24)
        - style.css shell desktop: grid-template-areas "sidebar content"; .docs-sidenav→sidebar; .article-wrap→content
        - style.css article-wrap: grid-template-areas "article aside"; minmax(65ch, 75ch) for reading width
          .article__body→article; .toc→aside; mobile reset: grid-template-areas "article"

[DONE]  Step 3l: Bento card home page (2026-06-24)
        - style.css .wiki-home-editorial: named areas "featured" | "recent"; __left→featured; __right→recent
        - style.css .cat-grid: container-type inline-size; @container(min-width:580px): first card spans 2 cols
          (landscape flex layout); @media(min-width:768px): repeat(3,1fr); hover uses var(--s-accent)

[DONE]  Step 3m: Per-tenant accent tokens (2026-06-24)
        - tokens.css Sovereign Editorial section: --s-accent: #C7A961 (PointSav gold)
          + --s-subnav-bg + --s-subnav-height tokens
        - tokens-woodfine.css :root: --s-accent: #c9a84c (Woodfine warm gold override)
          (tokens-woodfine.css loads after tokens.css on Woodfine tenant → overrides correctly)
        - style.css .s-search__input:focus: border-color → var(--s-accent)
        - style.css .cat-card:hover: border-color → var(--s-accent)
        - style.css .s-subnav__link--active: border-bottom-color → var(--s-accent)

[DONE]  Step 3n: Secondary sticky 48px nav bar (2026-06-24)
        - sovereign.rs: sovereign_secondary_nav(tenant) function — per-Tenant nav links, active via JS
        - server/mod.rs: added sovereign_secondary_nav to use statement
        - sovereign_page() now calls sovereign_secondary_nav(tenant) between nav + mobile drawer
        - home_handlers.rs + wiki_handlers.rs: explicit calls added (include! scope: share mod.rs imports)
        - style.css: .s-subnav / .s-subnav__inner / .s-subnav__link / .s-subnav__link--active CSS block
          Active indicator: border-bottom: 2px solid var(--s-accent); hidden on mobile (≤768px)

[DONE]  Step 3g: Regression armor (commit f8784d93 — jwoodfine 2026-06-24)
        - scripts/responsive-check.js: Playwright Node.js script; 6 assertions per page (no-hscroll,
          main-landmark, h1-present, data-instance, role-banner, role-contentinfo); 3 instances × 3
          pages × 3 viewports (320/768/1440) = 27 checks; exit 0 on all pass
        - sr-only h1 on home page (home_handlers.rs): site_title as screen-reader heading for WCAG outline
        - .sr-only utility CSS (style.css): standard visually-hidden implementation
        - 320px overflow guard (tokens-woodfine.css): @media ≤480px reset margin: 0; padding: 32px 16px
          on .featured hero panel (bleed-out assumed 32px parent padding; mobile parent has 0)
        - All 27/27 viewport assertions pass on live foundry-workspace

[NEXT]  Step 3h only (not this archive):
        3h. HScroll fix — marketing sub-pages (project-marketing scope; route via outbox)

[WAIT]  COMMAND/project-gis: deploy to foundry-prod
        (msg-id command-20260624-deploy-request-app-mediakit-knowledge-to)
[WAIT]  project-editorial: commit page-privacy.{md,es.md} in 3 content repos
        (msg-id project-knowledge-20260623-privacy-page-draft)
```

---

## Stage 6 Gate Checklist (per rebuild)

Before any `promote.sh` + cargo build:
1. `git status` clean in sub-clone
2. `cargo clippy --workspace --all-targets -D warnings` passes (or scoped to changed crate)
3. `cargo test -p app-mediakit-knowledge` passes
4. Confirm sub-clone is on `main` (not cluster branch — sub-clone uses `main` per cluster discipline)

---

## Phase 4 — Article Premium Reading Experience

**Operator directive (2026-06-24):** "the sites are better but simple not good enough." Phase 3
redesigned the chrome — header, footer, nav, home bento. The article body itself still looked
Wikipedia-plain: no content-type visual identity, no premium typography, no visual differentiation
between Topics and Guides. Five HTML structures (infobox, badge, guide-steps, callout, reading-time)
had zero CSS. This phase closes that gap.

**Research basis (3 agents completed 2026-06-24):**
1. Full article HTML/CSS structure audit — 5 critical unstyled elements identified
2. Award-winning editorial design research — Guardian category colors, Craig Mod margins, FT pull quotes
3. CSS technique sweep — `initial-letter`, `[data-content-type]` color dispatch, callout box patterns

**Implementation completed 2026-06-24 (commit pending):**

```
[DONE]  4a: Content-type CSS color system
        tokens.css: 7 new --ct-* custom properties
          --ct-topic (navy), --ct-guide (teal), --ct-research (amber),
          --ct-reference (purple), --ct-article (alias), --ct-category (muted),
          --ct-color: var(--ct-topic)  ← default; overridden by [data-content-type]
        style.css Section 5b: [data-content-type="X"] { --ct-color: var(--ct-X); } dispatch
          --ct-color cascades to badge, header band, h2 underline, hatnote border, card borders

[DONE]  4b: Article header zone
        - 4px border-top on article.article__body using var(--ct-color) — type identity on load
        - .article__title font-size: clamp(40px, 4.5vw, 64px) — more commanding presence
        - .content-type-badge: always-visible pill (was anon-only); color-mix tint background + border
        - .article__lede: left 3px border using var(--ct-color)
        - wiki_handlers.rs: <span class="reading-time" data-words="0"> added to doc-header__meta

[DONE]  4c: Drop cap
        .prose > p:first-of-type::first-letter { initial-letter: 3 2; color: var(--ct-color);
        font-weight: 700; padding-right: 6px }
        Browser support: 91%+ (Chrome 110+, Firefox 130+, Safari 9+). Graceful degradation on old.

[DONE]  4d: Infobox CSS (existing HTML; zero CSS before this phase)
        aside.infobox: float-right layout, clamp(200px,30%,280px) width, colored title bar,
        key-value table styling. Mobile: float:none, 100% width.

[DONE]  4e: GFM callout/alert boxes
        render.rs: options.extension.alerts = true (comrak 0.52; NOT pulldown-cmark)
        style.css: .markdown-alert base + 5 color variants (note/tip/important/warning/caution)
        CSS custom props --callout-color + --callout-bg per variant

[DONE]  4f: Reading progress bar + reading time estimate
        wiki.js: word-count calculation on .prose innerText → ".reading-time" span textContent
        wiki.js: scroll listener on article.article__body → #wiki-loading-bar scaleX progress
        Progress bar reuses existing loading bar; wiki:nav-start/done events gate the two uses.

[DONE]  4g: Guide steps numbered list
        ol.guide-steps: CSS counters, 44px colored circles (var(--ct-color)), grid layout,
        step content offset. ol.guide-steps had zero CSS before this phase.

[DONE]  4h: data-content-type on listing contexts
        home_handlers.rs: li.recent__item + data-content-type (recent articles)
        home_handlers.rs: li.wiki-cat-page-item + data-content-type (category list)
        home_handlers.rs: div.featured + data-content-type (featured article card)
        CSS: .recent-item[data-content-type] left border; .wiki-cat-page-item left border
        Note: TopicSummary struct has no content_type field → use existing item_type_key(&slug)
        function ("guide" | "topic" by slug prefix); FeaturedArticle same approach.
```

**Verification (2026-06-24):**
- `cargo build --release` exit 0
- All 3 services restarted and healthy (9090/9093/9095 → healthz ok)
- `scripts/responsive-check.js`: 27/27 viewport assertions PASS
- `cargo test -p app-mediakit-knowledge`: running at time of BRIEF update

---

## Phase 4 continued — Wikipedia-effect (2026-06-25 session)

**Operator directive:** "We don't really get the effect of Wikipedia Main Page or Wikipedia Articles."
Phase 4a–4h shipped functional CSS. This session added the visual polish gap items:

```
[DONE]  Logo currentColor fix (sovereign.rs)
        SVG fill="#111827" → fill="currentColor" on Woodfine + PointSav constants
        .s-wordmark { color: #fff } so logos render white on dark masthead
        Commit: 54da925d (jwoodfine)

[DONE]  Footer dark navy softened
        tokens.css --footer-bg: oklch(18% 0.04 258) — less harsh than pure near-black #0e1117
        Commit: 54da925d (jwoodfine)

[DONE]  h2 Wikipedia-style border-bottom
        .prose h2: border-bottom 1px solid var(--border); 56px solid accent overhang via ::after
        Drop cap on first paragraph: initial-letter: 3 2 float+font-weight CSS approach
        Commit: 5c7b97b6 (pwoodfine)

[DONE]  Home right column — Wikipedia-style bordered info boxes
        .wiki-home-stats: navy "Articles" header + large count box
        .wiki-home-lede: navy "About this wiki" header + lede text box
        home_handlers.rs: __head + __body sub-elements added to both boxes
        tokens-woodfine.css: starthere-chip border-top-color: #164679
        Commit: 4a117bbc (pwoodfine — toggle error: two consecutive pwoodfine commits)

[DONE]  Cat-card portal boxes — Wikipedia portal card style
        .cat-card: border-top: 3px solid var(--navy); border-radius; accent hover
        Commit: 2392ed69 (jwoodfine)

[DONE]  Prose table — outer border + cleaner separators
        .prose table: display:block; overflow-x:auto; border + border-radius outer shell
        .prose th: border-bottom: 2px solid (strong header); border-right: 1px solid
        .prose td: lighter border-right + border-bottom; last-child/last-row rules
        Commit: 01def04f (pwoodfine)

[DONE]  Article tab navigation — Wikipedia-style (Article/Talk/Edit/History)
        .wiki-page-tabs: horizontal tab row with bottom border-bottom container
        .wiki-tab: pill-shaped with bottom: -1px overlap; active tab has border on 3 sides
        Commit: ba0c63ae (jwoodfine)
```

**Build state (2026-06-25):** 4 commits queued for deploy. Binary at 00:08 includes
cat-card + table changes. Incremental build (bi7leg1f2) in progress for tab navigation CSS.
After build: deploy → 27/27 regression → session close.

**Outbox messages still pending (carry-forward):**
- Stage 6 self-service: 7 commits from Phase 3 + 7 from Phase 4 = 14 queued
- foundry-prod deploy: `command-20260624-deploy-request-app-mediakit-knowledge-to`
- Privacy pages: `project-knowledge-20260623-privacy-page-draft`

---

## Phase 4 continued — Polish + fixups (2026-06-25 session 8)

```
[DONE]  404 pages with sovereign chrome (commit 5a930375 — pwoodfine 2026-06-25)
        wiki_page / wiki_page_es catch WikiError::NotFound → not_found_page() renders
        full sovereign nav+footer; HTTP 404 status preserved; 36/36 regression pass

[DONE]  Search autocomplete rich UI (commit 36f5c68d — jwoodfine 2026-06-25)
        ac-item now renders: article title (bold) + content-type badge (Topic/Guide/Reference)
        + 90-char lede excerpt. Left accent border on hover. Dark mode rules added.
        API already returned `lede` field; only JS + CSS changes needed.

[DONE]  D10 wikilink validation pass
        Ran wikilink-audit.py on content-wiki-documentation:
        - Most "broken" links are false positives: TOML [[mounts]] in code blocks,
          bash [[ -e ]] syntax, placeholder [[slug]] in meta docs
        - Real broken links: 3 in build-a-colocation-map.md pointing to
          topic-co-location-ranking-system / topic-co-location-methodology /
          topic-od-catchment-methodology — these live in content-wiki-projects
          (GIS content, cross-wiki scope). No engine fix needed.
        - Footnotes: [^N] markers in knowledge-wiki-leapfrog-architecture.md are
          rendered by comrak correctly; definitions are simply missing from article
          (content gap, not engine bug).
```

**Outbox messages still pending (carry-forward):**
- Stage 6: 27 commits total queued (Phase 3 + Phase 4 + Wikipedia-effect + CSS audit + polish)
- foundry-prod deploy: `command-20260624-deploy-request-app-mediakit-knowledge-to`
- Privacy pages: `project-knowledge-20260623-privacy-page-draft`

---

## Session 11 — Woodfine logo fix (2026-06-25)

```
[DONE]  Woodfine logo overflow in grid masthead (commit 1bf661f7 — pwoodfine 2026-06-25)
        WORDMARK_SVG_WOODFINE had width="320" height="80" on the <svg> root element.
        The 320px intrinsic width blew out the auto-width first grid column in .s-topnav,
        causing the logo to overflow/not fully show on projects + corporate.
        Fix: removed width/height attributes (matching PointSav SVG behaviour).
        CSS height:28px;width:auto now controls sizing correctly. 36/36 regression pass.
```

**Totebox implementation scope: COMPLETE.** All remaining items are Command / editorial scope.
Pending handoffs: Stage 6 (33 commits), foundry-prod deploy, privacy pages (project-editorial).

---

## Open / Deferred

- **D10 wikilink validation pass** — ✅ COMPLETE 2026-06-25 (see Phase 4 polish section above)
- **PJ2 country index stubs** — needs real GIS data; multi-session research effort
- **JS bundle cleanup** — vendored JS in `app-mediakit-knowledge/static/vendor/`; operator decision P7b (2026-05-16) kept them; schedule explicitly when ready
- **Phase 6B DID portable identity** — gated on BP6 operator decision; plan at `.agent/plans/PHASE-6B-DID-IDENTITY.md`
- **Content gap: missing footnote definitions** — knowledge-wiki-leapfrog-architecture.md uses [^N] refs without footnote definitions; the comrak engine works correctly; definitions need to be added in content-wiki-documentation (editorial scope)
- **Content gap: cross-wiki links** — build-a-colocation-map.md links to GIS articles that live in content-wiki-projects; either use mount prefix or add redirect stubs in documentation wiki

---

## Phase 5 — Reading Experience and Visual Design Evolution

**Research basis:** Six-category web audit (2026-06-26) covering 22 sites across
editorial/news, technical docs, knowledge/reference, institutional, design-forward, and
dark-mode-excellence categories. Foundry cross-check against leapfrog research draft,
tokens files, and BRIEF Phases 3–4 implementation record. All hex values sourced from
audited live sites or their published design systems unless marked [recommended].

**Strategic answer to "drop Wikipedia or iterate?"** — Keep the Wikipedia framework for
navigation and muscle memory (the floor). Transcend it for visual quality and reading
comfort (the ceiling should be NYT/FT/Guardian editorial, not Wikipedia aesthetics). These
are orthogonal concerns. Conflating them is what made the June audit sites score 2–4/10
despite having correct structural bones. The bones stay; the skin changes.

---

### 5.1 — The Wikipedia Distinction: Navigation Framework vs. Aesthetic Ceiling

**What Wikipedia gets right and must be preserved unconditionally:**

The structural interaction contract is the one thing no competitor in the audit has improved
upon. Stripe Docs and Vercel Docs both converge on a left-sidebar-plus-right-TOC spatial
grammar that Wikipedia established. Our `grid-template-areas: "sidebar content aside"`
three-column layout from Phase 3 is structurally correct.

- **DOM landmark contract.** `mw-header`, `mw-body`, `#mw-content-text`, `.vector-toc`,
  `.infobox`, `.navbox`, `.reflist`, `.hatnote` — screen readers and power Wikipedia readers
  navigate these IDs from muscle memory. Preserve all of them.
- **Interaction contract.** `?` keyboard overlay, `/` to focus search, TOC pin/unpin with
  localStorage, hover-card previews, sticky header, mobile hamburger + TOC drawer,
  IntersectionObserver active-section highlighting — all implemented as of Phase 3. These
  are not up for reconsideration. The Stanford Encyclopedia of Philosophy's most-requested
  browser extension feature (persistent TOC rail) is something our engine already ships; it
  is a genuine advantage over the SEP.
- **Trust conventions.** Article/Talk/Edit/History tabs, numbered section headings,
  content-type badge, reading time estimate, hatnote, infobox float, footnote convention,
  `[edit]` pencils — the Phase 4 article header zone sits correctly in this tradition.

**What must now be sourced from editorial exemplars, not Wikipedia:**

The audit reveals a consistent ceiling Wikipedia cannot reach: reading comfort for sustained
30–60 minute sessions. Every editorial site audited (NYT, FT, Guardian, Bloomberg) invests
heavily in typographic warmth and spatial generosity that Wikipedia deliberately avoids in
favour of information density. Encyclopedic authority and reading comfort are not in tension
— Britannica and the Stanford Encyclopedia both demonstrate that deep information density
can coexist with excellent reading comfort when spacing and type choices are deliberate.

Specific Wikipedia deficiencies to remedy (sources in brackets):

- **Body font size.** Wikipedia's body is 14px (legacy) or 16px (Vector 2022). The NYT
  article body runs at 18px. The FT runs at 17–18px. The Guardian is 18px. The SEP runs
  at 16px but with tight max-width that creates comfortable 60–65 char lines. **Our target:
  18px (1.125rem).** Current implementation: 17px (1.0625rem) — one nudge needed.
- **Line-height.** Wikipedia: 1.6. NYT: 1.75–1.8. FT: 1.6 but with more generous paragraph
  spacing. Guardian: 1.65 with 28px paragraph bottom margin. **Our target: 1.7 line-height
  with 1.1em paragraph spacing** (currently paragraph spacing is ≤ 0.85em).
- **Section breaks.** Wikipedia has no visual separator between article sections — just a
  heading. NYT, FT, and Guardian all use generous vertical whitespace (40–56px) between
  sections. **Our target: 40px section margin-bottom** on `.prose` section elements.
- **Pull quotes.** None of the encyclopedic or editorial sites ship article pages without
  pull-quote capability. Notion, FT, Guardian, and NYT all support callout/pull-quote
  primitives as first-class block types. **Our target: `.pull-quote` CSS class** (see 5.4).

**The renamed commitment:** Henceforth "97% Wikipedia parity" refers to navigation structure
and interaction patterns only — not to visual aesthetics or reading comfort. The aesthetic
reference for new work is the editorial cluster (NYT, FT, Guardian) and the institutional
cluster (NASA 2024, CERN, MIT Media Lab). Wikipedia is the navigation floor. The editorial
sites are the reading-quality ceiling.

---

### 5.2 — Dark Mode Design Direction

**Finding:** The current dark mode changes only link colors. There are zero surface color
overrides — all backgrounds, borders, and text colors are identical to light mode when
`prefers-color-scheme: dark` fires. This is the highest-priority gap in the codebase.

**The audit consensus:** None of the six dark-mode-excellence sites (Linear, GitHub, Arc,
Raycast, Supabase, Framer) use pure black (`#000000`, `#0a0a0a`, or `#111111`). All use
chromatic dark backgrounds — that is, dark colours with a visible hue (typically blue/navy
or warm brown). The specific insight from GitHub's public Primer design system:
`--color-canvas-default: #0d1117` (dark) has a measurable blue bias (B component = 23,
vs. R = 13 and G = 17). GitHub describes this as intentional: it makes the product
instantly recognisable in dark mode and reduces the "infinite void" effect of pure black.

**Recommended dark mode palette for our platform** [recommended; derives from #164679 brand navy]:

| Token | Value | Role |
|---|---|---|
| `--dark-bg` | `#0d1520` | Base canvas (B=32, noticeably navy-tinted) |
| `--dark-bg-elevated` | `#162035` | Cards, sidebar, code blocks |
| `--dark-bg-overlay` | `#1f2d48` | Dropdowns, modals, hover cards |
| `--dark-fg` | `#eaecef` | Primary text (off-white; ~78% luminance, not #fff) |
| `--dark-fg-muted` | `#9aa3b0` | Secondary text, metadata |
| `--dark-border` | `rgba(255,255,255,0.10)` | Borders and dividers |
| `--dark-link` | `#88a9ff` | Links (lifted 30 lightness units vs light-mode link) |
| `--dark-link-visited` | `#c4b5fd` | Visited links (violet-tinted per Wikipedia convention) |
| `--dark-topnav-bg` | `#0a1628` | Masthead in dark mode (deeper than base canvas) |
| `--dark-footer-bg` | `#080f1c` | Footer in dark mode (deepest level) |

**Implementation target:** `tokens-woodfine.css` `@media (prefers-color-scheme: dark)` block
requires a complete rewrite — add all ten tokens above on `html[data-theme="dark"], @media`.
The `html[data-theme="dark"]` attribute path is already handled in `wiki.js`; only the CSS
token declarations are missing.

**Chromatic vs. neutral:** The navy-tinted chromatic dark (`#0d1520`) was chosen over a
neutral grey (`#111111`) because it creates unmistakable brand identity in dark mode,
parallels GitHub's approach, and reflects the #164679 masthead navy identity.

---

### 5.3 — Footer Design Direction

**Finding:** The current `--footer-bg: #0e1117` (near-black) was set during Phase 3 without
explicit research justification. The user has flagged it as feeling wrong, and the audit
confirms why: pure or near-pure black footers were observed on only 2 of 22 audited sites
(both legacy sites). The 2025 pattern in the audit is one of three approaches:

1. **Deep brand-chromatic dark** — Guardian uses `#041f4a` (deep navy), FT uses deep
   charcoal (`#1a1a1a` with warm tint). Both anchor the brand identity vertically.
2. **Light footer** — NYT, Stripe, Vercel, and Apple all use white or near-white footers
   (`#fafafa`–`#ffffff`). These read as clean, modern, confident.
3. **Brand-accent** — Linear uses their near-black with a subtle purple/violet tint.
   GitHub uses `#0d1117` (matching their dark canvas, so the footer is invisible in dark mode).

**Recommendation:** Shift to deep chromatic navy, paralleling Guardian's approach, because:
- Our masthead is `#164679` (navy) — the footer should echo it, creating a navy-top / navy-bottom
  frame that wraps the white content body. This is a strong visual system.
- Pure `#0e1117` reads as "forgot to design the footer." A navy footer reads as a decision.
- `oklch(16% 0.06 250)` ≈ `#0d1830` — slightly more chromatic than current, blue-shifted.

**Specific changes:**

```css
/* tokens.css */
--footer-bg: oklch(16% 0.06 250);       /* was oklch(18% 0.04 258) — more chromatic, deeper navy */
--footer-divider: oklch(30% 0.05 250);  /* new: thin border-top on .s-footer__legal subzone */
```

```css
/* style.css addition */
.s-footer__legal {
  border-top: 1px solid var(--footer-divider);
  margin-top: var(--sp-6);
  padding-top: var(--sp-4);
}
```

Three-column content anatomy is correct (Navigate / Legal / Platform) and should be retained.
The visual problem is the color, not the structure.

---

### 5.4 — Article Reading Experience

**Finding:** The user reported articles are "not comfortable to read." The audit identifies
four specific mechanisms that the best editorial sites employ which our articles currently lack.

**Mechanism 1 — Font size (highest impact):**
Body font is `1.0625rem` (17px). Target editorial sites:
- NYT article body: 18–19px, Georgia-weight (high-contrast serif)
- FT: 17–18px, their Financier (similar to Miller)
- Guardian: 18px, Guardian Text Egyptian
- Britannica: 16px but with very tight max-width
- Stanford SEP: 16px, Times New Roman, 65-char lines

Fix: change `--text-base: 17px` → `--text-base: 18px` in `tokens.css`. One change, large
perceived difference. Verify `.prose` consumes `var(--text-base)` (currently uses font-size
inherited from `body`).

**Mechanism 2 — Paragraph spacing (second-highest impact):**
Current `.prose > p + p` has no `margin-top`; paragraphs stack at the inherited line-height
gap only. NYT runs `margin-bottom: 28px` on paragraphs. FT: `margin-bottom: 24px`.
Fix: add `.prose > p + p { margin-top: 1.1em; }` and `.prose > h2, .prose > h3 { margin-top: 2em; }`.

**Mechanism 3 — Section spacing:**
No visual "chapter break" between sections. Best editorial sites use 40–56px of space between
`## Headings` as section markers. Fix: add `.prose section, .prose > h2 { margin-top: 2.5rem; }`.

**Mechanism 4 — Pull quote primitive:**
NYT, FT, Guardian, and Bloomberg all support pull quotes as a first-class editorial tool.
They are absent from our implementation. A pull quote signals "this is a serious article."

```css
.pull-quote {
  font-size: calc(var(--text-base) * 1.5);   /* 27px at 18px base */
  font-family: var(--font-display);
  font-style: italic;
  font-weight: 400;
  line-height: 1.35;
  color: var(--fg-1);
  border-left: 4px solid var(--ct-color, var(--navy));
  margin: var(--sp-8) 0;
  padding: var(--sp-3) 0 var(--sp-3) var(--sp-6);
  hanging-punctuation: first;
}
```

Markdown trigger: `> **Pull quote text**` (blockquote with bold content) — detected in
renderer and emitted as `<blockquote class="pull-quote">`.

**Mechanism 5 — Content measure:**
Current `--measure: 66ch` is correct per Bringhurst (66 characters). Verify it is applied
as `max-width: var(--measure)` on `.prose` or `.article__body` — an audit of the CSS found
the variable is declared but usage needs confirmation.

---

### 5.5 — Home Page Evolution

**Finding:** The current home page (category grid + featured article + recently-changed list)
is structurally sound but visually generic. The audit of editorial sites reveals that the
difference between a "good" home page and an "award-winning" one is usually a single
high-quality editorial element in the hero zone — not a redesign of the entire layout.

**Specific additions, in priority order:**

1. **Featured-hero image slot.** Add a `.featured-hero` modifier on the existing `.featured`
   component. When the featured article's frontmatter includes `hero_image: path/to/image.jpg`,
   render a full-width image background with a dark gradient overlay and white text. No image =
   current solid-color treatment. This is the pattern used by Guardian, NASA, and Bloomberg.
   Implementation: `background-image: linear-gradient(to bottom, transparent 30%, #000 100%),
   url({hero_image})` on `.featured-hero`.

2. **Category tiles with icons or color identity.** The current 9-tile category grid has
   tiles differentiated only by name. Add a `category_color` frontmatter field that maps to
   a `--ct-*` token, giving each category a distinct left-border or background tint (already
   used on listing cards — extend to home tiles).

3. **Editorial timestamp in banner.** Add "Last updated: {date}" in the featured section
   based on max `updated` across the corpus. This is a trust signal that costs nothing to
   implement and directly signals active editorial curation. Notion, Stripe, and NASA all
   show this.

---

### 5.6 — Design System Slides (New Leapfrog Primitive)

**Concept:** Embed Keynote/Pitch-style slide decks as wiki articles on
documentation.pointsav.com. A slide deck is a first-class content type alongside TOPIC
articles and GUIDE articles. This is a genuine leapfrog primitive — no major wiki platform
(Wikipedia, Notion, Confluence, GitBook, Docusaurus) ships this.

**Why this is worth implementing:**
- Design system documentation benefits massively from visual slides (token grids, color
  palettes, spacing systems, component anatomy diagrams)
- Slides embedded in TOPICs make complex architectural concepts accessible without dumbing
  them down — the slide is a visual anchor; the prose is the explanation
- JSON-LD `Presentation` type + per-slide `WebPageElement` enables rich search snippets
- Complements the Doctrine claim #39 research-trail discipline: slides can carry their own
  research-trail metadata

**Content type specification:**

```yaml
# Frontmatter for a slide deck article
title: "Design System Color Tokens"
content_type: slides
slide_count: 12
aspect_ratio: "16:9"
transcript: true   # render <details> prose transcript alongside each slide
```

**HTML structure:**

```html
<div class="slide-deck" data-slide-count="12" role="region" aria-label="Slide deck: Title">
  <div class="slide-deck__controls">
    <button class="sd-prev" aria-label="Previous slide">←</button>
    <span class="sd-progress">1 / 12</span>
    <button class="sd-next" aria-label="Next slide">→</button>
    <button class="sd-fullscreen" aria-label="Fullscreen">⛶</button>
  </div>
  <div class="slide-deck__viewport">
    <section class="slide" aria-label="Slide 1: Title">
      <!-- Markdown content for this slide rendered here -->
    </section>
  </div>
  <details class="slide-deck__transcript">
    <summary>Read transcript</summary>
    <!-- Full prose transcript for accessibility -->
  </details>
</div>
```

**CSS tokens:**
```css
--slide-aspect: 16 / 9;
--slide-bg: var(--bg-subtle);
--slide-fg: var(--fg-1);
--slide-border: var(--border);
--slide-radius: var(--radius-md);
```

**Keyboard navigation:** ← → arrows for prev/next; F for fullscreen; Escape to exit.
State persists in URL hash (`#slide-3`), enabling deep links to individual slides.

**Accessibility requirement:** When `transcript: true`, every slide section must have a
corresponding `<details>` block with full prose content. The slide-deck should be fully
readable as linear text when JavaScript is disabled — CSS `aspect-ratio` collapses to
`height: auto` with a fallback display.

**Authoring:** Slide boundaries in Markdown via `---` horizontal rules within a fenced
`:::slides` block (new comrak extension hook). Author writes one slide per section.

**Status:** Document intent now; plan coding after Phase 5 reading-comfort and dark-mode
work ships. Estimated complexity: medium (new template + JS controller + CSS + comrak hook).

---

### 5.7 — Font Resolution and Typography System

**Finding:** There is a token-naming split in the current CSS:

- `tokens.css` declares `--font-family-display: 'Playfair Display'` (DTCG-style)
- `style.css` line ~150 independently declares `--font-display: "Instrument Serif"` (engine alias)
- Selectors throughout `style.css` consume `var(--font-display)` — the engine alias
- Result: Playfair Display Variable (committed to `static/fonts/`) is never used; Instrument
  Serif has no committed WOFF2 and falls through to Georgia

**Both fonts currently serve Georgia in practice.** The Playfair Display WOFF2 files are in
`static/fonts/` but never loaded because the `--font-display` pointer is wrong.

**Resolution:** Instrument Serif was introduced in a later session without reconciling the
token pointer. The BRIEF commitment stands: Playfair Display Variable is the correct display
font.

**Fix (single line):**
```css
/* style.css line ~150 — change */
--font-display: "Playfair Display", Georgia, "Times New Roman", serif;
/* and remove any @font-face for Instrument Serif if present */
```

**Why Playfair Display is right (audit-confirmed):**
- FT uses Financier Display (high-contrast, neo-classical serif — same register as Playfair)
- NYT uses Georgia / Miller / NYT Cheltenham (same register)
- Guardian uses Guardian Headline (same register)
- Instrument Serif is a lower-contrast contemporary face with no editorial reference pedigree
  in the audit data — it reads as "nice but not authoritative"
- Playfair Display's `wght: 400–900` variable axis enables editorial weight modulation
  (thin article body headings at 400; bold display at 700+) from a single font file

**Typography system summary after fix:**

| Role | Font | Token |
|---|---|---|
| Display / headings | Playfair Display Variable | `--font-display` |
| Body | IBM Plex Sans Variable | `--font-body` |
| Long-form article prose | Source Serif 4 | `--font-reading` |
| Monospace / code | IBM Plex Mono | `--font-mono` |

One open question: `--font-reading: "Source Serif 4"` is declared but audit of the article
template shows the `.prose` body may be using `--font-body` (sans) rather than `--font-reading`
(serif). Wikipedia-parity principle says article prose should be serif. Verify and fix if needed.

---

### 5.8 — Coding Priority Order

Based on research synthesis. Items are ordered by impact × feasibility; earlier items
unlock later ones (dark mode must ship before home page evolution, for example).

1. **Dark mode surface tokens** (highest ROI; zero surface dark mode exists across all 3 sites)
   - `tokens.css`: add `--dark-bg`, `--dark-bg-elevated`, `--dark-bg-overlay`, `--dark-fg`,
     `--dark-fg-muted`, `--dark-border`, `--dark-link`, `--dark-link-visited`
   - `tokens-woodfine.css`: complete rewrite of `@media (prefers-color-scheme: dark)` block
   - `style.css`: wire dark tokens throughout with `@media (prefers-color-scheme: dark)` and
     `html[data-theme="dark"]` selectors

2. **Playfair Display font token fix** (single line; activates committed WOFF2 files)
   - `style.css` line ~150: change `--font-display` from `"Instrument Serif"` to `"Playfair Display"`
   - Remove Instrument Serif `@font-face` stub if present
   - Verify `.prose` body uses `var(--font-reading)` for serif article prose

3. **Article reading comfort** (three targeted changes, ~20 lines of CSS)
   - `tokens.css`: `--text-base: 18px` (was 17px)
   - `style.css`: `.prose > p + p { margin-top: 1.1em; }`
   - `style.css`: `.prose > h2, .prose > h3 { margin-top: 2.5rem; }`
   - `style.css`: add `.pull-quote` class (see 5.4 spec)

4. **Footer chromatic shift** (two token changes + one new rule)
   - `tokens.css`: `--footer-bg: oklch(16% 0.06 250)` (was `oklch(18% 0.04 258)`)
   - `tokens.css`: add `--footer-divider: oklch(30% 0.05 250)`
   - `style.css`: `.s-footer__legal { border-top: 1px solid var(--footer-divider); ... }`

5. **Article prose font verification** (confirm .prose uses --font-reading serif)
   - If `.prose` is using sans body, add: `.prose { font-family: var(--font-reading); }`
   - This is the "Source Serif 4 for article text" commitment from Phase 1

6. **Home page featured-hero image slot**
   - `src/chrome/home.rs`: add `hero_image` field handling to `home_page_documentation()`
   - `style.css`: `.featured-hero { background-image: linear-gradient(...), url({img}); }`

7. **Design System Slides primitive** (medium complexity; new content type)
   - New comrak extension hook for `:::slides` block
   - `static/slide-deck.js`: keyboard controller, fullscreen, URL hash state
   - `style.css`: `.slide-deck`, `.slide`, `.sd-controls`, `.sd-progress`, dark mode rules
   - `src/chrome/wiki_handlers.rs`: detect `content_type: slides`, render slide-deck template

8. **SEO and meta completeness**
   - `src/chrome/sovereign.rs`: add `<meta property="og:image">` slot, canonical `<link>` element
   - `src/server/wiki_handlers.rs`: emit `Last-Modified` HTTP header from content file mtime

---

*Phase 5 section added 2026-06-26 | research: 6-category web audit (22 sites) + Foundry
cross-check | by totebox@project-knowledge*

---

## Phase 5 — Implementation Log (2026-06-26 sessions 13–14)

All P5 items implemented and committed to monorepo `main`. One item deferred.

| Item | Status | Commit | Notes |
|---|---|---|---|
| P5-1 Dark mode chromatic | ✅ | `012ad9d5` (jwoodfine) | Both style.css blocks + tokens-woodfine.css surface vars |
| P5-2 Playfair Display | ✅ | `012ad9d5` | `--font-display` switched; WOFF2 already in static/fonts/ |
| P5-3 Article reading comfort | ✅ | `012ad9d5` | `p+p` margin-top 1.5em + `.pull-quote` CSS + leading-reading wire |
| P5-4 Footer chromatic | ✅ | `012ad9d5` | `oklch(16% 0.06 250)` + `--footer-divider` + `.s-footer__legal` rule |
| P5-5 Prose serif verification | ✅ | — | `.prose` already used `var(--font-reading)` = Source Serif 4; no change |
| P5-6 Featured-hero image slot | ✅ | `a2d0138c` (jwoodfine) | `hero_image` field in Frontmatter → TopicSummary → FeaturedArticle; `.featured--has-image` maud class + `--hero-img` CSS var; gradient overlay `::before` |
| P5-7 Design System Slides | ⏳ DEFERRED | — | High effort; own sub-BRIEF when started. See §5.6 for full spec. |
| P5-8 SEO + meta completeness | ✅ | `a2d0138c` | `og:type/title/description/image` in `wiki_chrome` head; `Cache-Control: public, max-age=3600, must-revalidate` on article responses |
| P5-9 BRIEF §5.4/5.5 extras | ✅ | `2cb6e9b1` (jwoodfine) | Pull-quote renderer (`> **bold**` → `.pull-quote` + 2 unit tests); 7 category tile accent colors via `--cat-accent` CSS var; featured card "Last updated" corpus-date |

**§5.4 mechanisms resolved:**
- Mechanism 1 (18px body) — already correct pre-P5; no change needed
- Mechanism 2 (paragraph spacing) — done in P5-3
- Mechanism 3 (section spacing) — `.prose h2 { margin-top: 2.8em }` already exceeded 2.5rem target; no change
- Mechanism 4 (pull-quote) — CSS in P5-3; renderer hook in P5-9
- Mechanism 5 (content measure) — `--measure: 66ch` already wired at lines 1382 + 1484; confirmed

**§5.5 home page additions resolved:**
- Item 1 (featured-hero image) — P5-6 ✅
- Item 2 (category color identity) — P5-9 ✅
- Item 3 (featured "Last updated") — P5-9 ✅

**§5.7 font resolution:** done in P5-2 (`--font-display` corrected).

**Stage 6 status:** 3 commits (`012ad9d5`, `a2d0138c`, `2cb6e9b1`) queued for Command Session promote + foundry-prod rebuild. Message in Command inbox (`command-20260627-stage-6-2-commits-foundry-prod-rebuild-p` + addendum).

**What remains:**
- P5-7 Design System Slides (see §5.6) — ✅ COMPLETE commit `a3be3bd9` (pwoodfine 2026-06-29). See `BRIEF-slides.md` for implementation record.
- Privacy pages live — blocked on project-editorial (`project-knowledge-20260623-privacy-page-draft`)
- Privacy link in `sovereign_footer()` nav — add after project-editorial commits the pages
- DESIGN-RESEARCH draft → project-design (staged; pickup pending)

---

## Phase 6 — Visual Excellence + Records Positioning

**Session:** 2026-06-29 | **Research:** 3-agent strategic audit (chrome inventory, 22-site comparison, compliance deep-dive)

**Strategic directive:** Raise the visual quality ceiling to match NYT/Guardian/Stripe editorial standards. Simultaneously signal "authoritative auditable record" for SOC3/DARP/US-EU government audit contexts. The existing architecture (sovereign Rust engine, git-backed canonical records, JSON-LD, claim rail, research trail) is CORRECT — no rebuild. All changes are presentation layer (CSS, HTML templates, JS).

**Rebuild decision (2026-06-29, confirmed):** Do NOT rebuild. The sovereign engine advantage (2ms response, git-backed records, bilingual, multi-tenant, JSON-LD, Wikipedia muscle memory) is irreversible if discarded. Gaps identified are CSS/HTML template issues, not architectural ones.

**Key findings from research:**

1. **Typography gap** — body text 18px (correct), but line-height ~1.55 (need 1.78 for editorial quality), paragraph spacing insufficient (WCAG 1.4.12 compliance requires resilience to 2x override), heading scale not using `clamp()` fluid typography.

2. **Masthead hierarchy gap** — search, wordmark, and controls at equal visual weight. Premium docs sites (Stripe, Vercel) make search the dominant center element. Wordmark anchors left at compact size; controls are subtle right cluster.

3. **Home page editorial moment** — featured article card is Wikipedia-ish (contained box). Editorial sites (Guardian, NYT, Bloomberg) use full-bleed hero with dark gradient overlay and dominant editorial headline at `clamp(2.2rem, 4vw, 3.5rem)`.

4. **Talk tab — compliance-incompatible** — research across SharePoint, Confluence, Guru, Slab confirms regulated industries use inline comments with audit trails, NOT separate Talk pages. "Talk" reads as "community forum" (casual), not "editorial review" (auditable). Replacing with full inline annotations system (git-committed sidecar files, named author + ISO 8601 timestamp, resolution status). See `BRIEF-inline-annotations.md`.

5. **TOC active tracking absent** — no IntersectionObserver scroll highlighting. Reading time JS has `data-words="0"` — never populated. Both are visible gaps against Wikipedia and all premium sites.

6. **Dark mode elevation** — current dark mode is flat surfaces. Premium dark mode (Linear, GitHub, Supabase) uses 4 visible elevation layers. `--bg-elevated`, `--bg-subtle`, `--bg-overlay` tokens exist in the dark mode block but are not applied to surfaces.

7. **WCAG 2.1 AA compliance gaps (from research):** confirmed 4.5:1 contrast minimum for body text; touch targets ≥44×44px on mobile; no `user-scalable=no` in viewport meta; max line length 80 chars (our 70ch ✓); NO justified text (left align only). Line spacing ≥1.5x (our 1.75 target ✓). WCAG 1.4.12 Text Spacing: page must not break when user sets paragraph spacing to 2x font size.

8. **Records positioning signals** — "authoritative record" design pattern (research from Cloudflare, Stripe, USWDS) uses: specificity over marketing, named frameworks with version numbers, third-party verification links, quiet confidence (restrained color). Footer should say "Git-versioned content · WCAG 2.1 AA · EN 301 549". Short git SHA near "Last edited" date in article chrome. Future: `/page/compliance` page with SOC3 attestation when available.

**Implementation plan (child BRIEFs):**
- `BRIEF-visual-excellence.md` — Sessions A–F covering typography, masthead, home page hero, TOC/reading-time, dark mode elevation
- `BRIEF-inline-annotations.md` — Sessions D1–D4 covering Talk replacement → full inline annotation system (child of BRIEF-visual-excellence)

**Phase 6 coding checklist:**

| Item | Status | Session | Notes |
|---|---|---|---|
| P6-A Typography + spacing | ✅ SHIPPED | A | commit `898470a9` — promoted + deployed |
| P6-B Masthead hierarchy | ✅ SHIPPED | B | commit `2feab2d0` — promoted + deployed |
| P6-C Home editorial hero | ✅ SHIPPED | C | commit `63378466` — promoted + deployed |
| P6-D TOC active tracking + reading time | ✅ SHIPPED | E | already live from prior phases |
| P6-E Dark mode elevation | ✅ SHIPPED | F | commit `f9ae99a6` — promoted + deployed |
| P6-F Inline annotations (Talk replacement) | ✅ SHIPPED | D1–D4 | commit `25ffe5fa` — Stage 6 + rebuild pending (Command) |
| P6-G Compliance signals in footer | ✅ SHIPPED | with A | commit `998b3a2c` — promoted + deployed |

*Phase 6 section added 2026-06-29 | research: 3-agent audit (chrome inventory + 22-site comparison + WCAG/compliance research) | by totebox@project-knowledge*

---

## Post-P6 bug fix — 2026-06-30

**Bug:** `RATIFIED_CATEGORIES` hardcoded constant in `server/mod.rs` caused all three wiki instances to show the same documentation category set in the article sidenav and home-page grid. The binary is single but the three instances serve different content repos with different categories.

**Fix:** `d4b0ae3e` — `categories: Vec<String>` added to `SiteConfig`; `site_categories` field on `AppState`; `ratified_categories()` helper falls back to documentation set when empty. TOML path `[site] categories` takes precedence over `SITE_CATEGORIES` env var. All 23 source + test files updated; 139/139 tests pass.

**Config applied:** operator added `categories = [...]` to all three `/etc/local-knowledge/*.toml` files; services restarted.

**Stage 6 pending:** `d4b0ae3e` must be promoted by Command before the distributable binary for project-software is built. See BRIEF-binary-distribution.md.

**Follow-up bug fix (cat-grid):** `7d0d8a62` — home-page category grid (`cat-grid`) was gated by `@if brand_instance == "documentation"` in `home_handlers.rs`, causing it to be absent on projects and corporate instances. Root cause: the same class of bug as `RATIFIED_CATEGORIES`. Fix: removed the gate; added `@else` branch using `ratified_categories` + `cat_descriptions` (from `_index.md`) + `CAT_ACCENT_PALETTE` cycling. Documentation unchanged. Stats one-liner now shows category count for all instances. 143/143 unit tests + all integration tests pass. Installed to foundry-workspace; all 3 healthz OK.

**Stage 6 pending:** `d4b0ae3e` + `7d0d8a62` must be promoted by Command before distributable binary for project-software is built.

*Bug fixes added 2026-06-30 | by totebox@project-knowledge*
