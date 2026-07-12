# NEXT.md — project-software

This file previously read as a pointer to "pointsav-monorepo/NEXT.md" — but this
archive's root IS the monorepo (per `CLAUDE.md`), so that pointer was circular/stale
drift itself. Replaced with real content 2026-07-11.

## Drift flagged 2026-07-11 (sync + localhost catalog fix session)

- [ ] `[2026-07-11 totebox@claude-code]` **CLAUDE.md stale claim**: this archive's
  `CLAUDE.md` states "there is no separate sub-clone one level down," but a nested
  `pointsav-monorepo/` sub-clone genuinely exists here and is already documented as
  intentional in this archive's own `.gitignore` (line 46, 2026-07-08 comment,
  same pattern as project-marketing/project-design). The CLAUDE.md line needs a
  wording fix — not an active defect, just stale prose.
- [ ] `[2026-07-11 totebox@claude-code]` **push-to-prod.sh stale crate reference**:
  `~/Foundry/bin/push-to-prod.sh` (line ~357) still pulls the marketplace catalog
  from vendor's *old* (non-`-2`) `app-privategit-marketplace/catalog/products.yaml`
  path, and its own inline comment says the `/var/lib/local-software/` convention
  "no longer exists post-P8-reorg" — but it does exist and is what's actually
  running on this workspace VM. Needs reconciliation: confirm what prod
  (`foundry-prod`) actually expects vs. this workspace's local-dev convention
  before touching the script.
- [ ] `[2026-07-11 totebox@claude-code]` **post-commit hook error**: committing the
  vendor-sync change (`dae795c5`) printed
  `/srv/foundry/.git/hooks/post-commit: line 95: /usr/bin/python3: Argument list too long`.
  Commit still landed successfully, but the hook itself is broken for large diffs
  (31 files changed) — likely passing the full file list as argv instead of via
  stdin/xargs. Workspace-level hook, Command Session scope.
- [ ] `[2026-07-11 totebox@claude-code]` **os-console catalog version mismatch**:
  `products.yaml` lists `edition: "0.2.4"` / `path: os-console/0.2.4`, but no such
  version is deposited on disk (`/var/lib/local-software/releases/os-console/`
  only has `0.1.5`, `0.1.6`, `2026.05.144`). Direct version-pinned download
  (`/releases/os-console/0.2.4/linux-x86_64`) 404s live; the `/latest/` redirect
  route papers over it for `install.sh`, but the catalog's stated edition doesn't
  match reality and the JSON API's `download_url` points at the dead version.
- [ ] `[2026-07-11 totebox@claude-code]` **Systemic per-version MANIFEST.json gap**:
  every deposited product on this host (`os-console`, `os-network-admin`,
  `app-mediakit-knowledge`, `soft-orchestration-command`, `os-privategit`,
  `app-privategit-marketplace`, `app-privategit-source`, `tool-wallet`) only has a
  product-root `MANIFEST.json`, never a per-version copy inside the version
  directory. `app-privategit-source`'s `/releases/:product/:version/MANIFEST`
  route reads `<version>/MANIFEST.json` specifically and 404s for every single
  product as a result. Binary downloads themselves mostly still work (route is
  separate); this only breaks the MANIFEST-metadata endpoint, but it's broken
  everywhere, not just for the products the old brief called out.
- [ ] `[2026-07-11 totebox@claude-code]` **soft-orchestration-command naming vs.
  canonical `app-orchestration-command`**: the deposited release folder at
  `/var/lib/local-software/releases/soft-orchestration-command/0.0.1/x86_64-linux`
  (1.7MB, real binary) is still under the old name. Command confirmed
  2026-07-06 that `app-orchestration-command` is canonical
  (msg-id `command-20260706-decision-orchestration-command-naming-re`). Needs a
  rename when this product is added to the catalog (see pending inbox task).
- [ ] `[2026-07-11 totebox@claude-code]` **tool-wallet relicensing status
  contradiction**: `BRIEF-binary-library-repositioning.md` says the
  tool-wallet → Apache-2.0 relicensing PR is "drafted, not applied — needs
  admin-tier + legal review." But vendor's current `tool-wallet/Cargo.toml`
  already carries `license = "Apache-2.0"` and an SPDX header, which this
  session's sync just pulled in as-is (matching current reality). Needs
  reconciliation: was this actually approved through proper governance, or did
  it land without the stated review gate? Flagging, not resolving.
- [ ] `[2026-07-11 totebox@claude-code]` **Confirmed empty release dirs**:
  `os-infrastructure`, `os-interface`, `os-mediakit`, `os-totebox`, `os-workplace`
  have zero deposited files at all on this host — matches the old brief's
  deposited-binary gap claim for these five specifically (removed from the
  catalog 2026-07-05 rather than fixed).
- [ ] `[2026-07-11 totebox@claude-code]` **app-mediakit-distributions missing
  from workspace members**: this archive's root `Cargo.toml` doesn't list
  `app-mediakit-distributions` even though `CLAUDE.md` names it as owned and it
  has a real `Cargo.toml`/`src/`. Pre-existing gap, unrelated to today's sync.

## Done this session

- [x] `[2026-07-11 totebox@claude-code]` Full top-to-bottom audit of
  `app-privategit-marketplace` + `app-privategit-source` (Opus fan-out + Fable
  synthesis, report-only) — see
  `.agent/briefs/BRIEF-software-consolidated-service-audit.md`. **Read this
  before touching either crate again** — it found a HIGH-severity unauthenticated
  path-traversal candidate in the release server (`release_path()`, no segment
  sanitization) and confirmed the entire paid/checkout/order/download flow
  cannot complete a single transaction today for two independent reasons
  (price/product-matching breaks at nonzero price; `SIGNING_KEY_SECRET` env-var
  name mismatch causes unconditional 503 on download). Both are one-line-scale
  fixes, not rewrites, but neither is fixed yet — gated on operator review per
  the report-only decision this session.

## Next up (from the audit's Prioritized Gap List — not yet implemented)

- [ ] `[2026-07-11]` Tier 1: path traversal fix + router-level integration test
  (`release_path()` in `app-privategit-source`).
- [ ] `[2026-07-12]` Tier 1: `SIGNING_KEY_SECRET` env-var rename in the
  marketplace systemd unit — **operator asked for this fix 2026-07-12; command
  drafted (`sed` rename of `LICENSE_SIGNING_KEY` → `SIGNING_KEY_SECRET` in
  `wallet.conf` + daemon-reload + restart) but not yet confirmed run** — verify
  `wallet.conf` before assuming this is closed. Startup keypair-match self-test
  across both services still separately open regardless.
- [ ] `[2026-07-11]` Tier 1: real per-tier pricing + fix the amount→product
  matching bug it currently masks, before BETA lifts (also guard the
  empty-wallet-address invoice render).
- [ ] `[2026-07-11]` Tier 1: align `os-console`'s catalog `edition` with a real
  deposited version (or mint downloads through `/latest/`).
- [ ] `[2026-07-11]` Tier 2–4 items (chromed error pages, per-version MANIFEST
  fix, masthead nav, README rewrite, etc.) — full list in the BRIEF's
  Prioritized Gap List, tiers 2–4. **Contact-page city hardcoding closed
  2026-07-12** (see below) — struck from this list.

## 2026-07-12 — live-review UI fixes (operator-reported)

Five concrete issues found via the operator's own visual review of the marketplace, fixed in
`app-privategit-marketplace/src/ui/`:

- [x] Shelf-rail "Commercial"/"Open Source" links gave no click feedback — worse, `href="#commercial"`
  had no matching `id` anywhere (dead anchor; only "Open Source" actually jumped). Added
  `id="commercial"` to the shelf wrapper and a small click-toggle script for `.is-current`
  (`catalog.rs`).
- [x] `/licensing` didn't match other pages' typography — `wrap_static_html()` never added the
  `.sw-legal` class its column/heading CSS is scoped to. Added `class="sw-legal"` to
  `static/licensing.html`'s `<main>` (one line, only call site).
- [x] Contact + Disclaimer pages each duplicated the shared footer's copyright/trademark block
  (and Contact's copy was additionally stale — "Vancouver · New York · Berlin" vs. the footer's
  current Vancouver/New York). Removed both trailing blocks; footer already renders this once,
  correctly, on every page.
- [x] Footer's "Contact us · Disclaimer · Privacy" line removed (redundant with nav higher up);
  "Powered by PrivateGit" badge right-justified via a new wrapper div, not a global flex change.

**Correction during this work**: initially suspected "MCorp™" in `surface.rs`'s `trademark_line()`
was a factual error vs. `legal-tokens-pointsav.yaml`. Checked the actual authoritative
`TRADEMARK.md` directly — "MCorp™" is confirmed correct and deliberate (TRADEMARK.md's own
canonical short-form notice names it explicitly). The YAML's rendered `statement` field is what's
actually incomplete (drops 2 of 6 marks vs. its own `owned` list and vs. TRADEMARK.md) — flagged
to Command/project-editorial (msg-id `command-20260712-fyi-follow-up-legal-tokens-pointsav-yaml`),
not fixed here. Runtime legal-tokens consumption in `surface.rs` deliberately **not** wired up
this session — holding until the YAML itself is confirmed correct, so as not to wire in a
regression.

- [ ] `[2026-07-12]` Follow-up once Command/project-editorial resolve the YAML question: wire
  `app-privategit-marketplace` to read `legal-tokens-pointsav.yaml` for copyright/trademark text
  at runtime (env `LEGAL_TOKENS_PATH`, fallback to hardcoded values on missing/unparseable file).

### Browser-in-the-loop responsive audit (Playwright, mobile/tablet/desktop) — done, found 3 more real bugs

All 9 marketplace pages screenshotted at 375/768/1440px. Confirmed the 5 fixes above render
correctly, and found 3 additional real bugs the code-only review missed:

- [x] **Mobile/tablet horizontal overflow on `/software`** (whole page rendered at 809px on a
  375px viewport). Root cause: `.sw-cat-cmd__text` (flex item, `white-space:nowrap`) and
  `.sw-cat-card` (CSS Grid item) both lacked `min-width:0` — flex/grid items default to
  `min-width:auto` (their content's intrinsic width), which is exactly what causes this class of
  overflow. Two one-line fixes (`catalog.rs`); verified via a standalone test instance on a
  separate port (not the live service) that `document.documentElement.scrollWidth` now matches
  the viewport exactly at all 3 sizes, zero overflowing elements.
- [x] **Product-detail page (`/software/:id`) install-command box and badges were completely
  unstyled** — `product_detail_style()` never defined `.sw-cat-cmd`/`.sw-cat-badge*` at all
  (raw text, default browser button). Added the missing rules (`product_detail.rs`).
- [x] **`/licensing` was still stuck at a fixed 1440px regardless of viewport** even after the
  chrome-class fix above — turned out the live server was reading a completely different,
  ancient (1216-line, May-17, hardcoded `viewport width=1440`) deployed copy at
  `/var/lib/local-software/static/licensing.html`, not the 91-line git-tracked current file —
  same "deployed data vs. git source" drift class as the `products.yaml` catalog bug found
  earlier this session. Redeployed the correct file.

Commits: `defd3bfc` (5 UI fixes + product-detail styles + licensing static-file content),
`485a607a` (grid min-width fix). All 76 tests pass throughout.

## 2026-07-12 — SEO draft from project-editorial needs correction, not applied

`totebox@project-editorial` staged an SEO draft (msg-id
`project-editorial-20260712-seo-draft-ready-software-pointsav-com-br`, Opus+Fable reviewed,
part of `BRIEF-seo-cross-site-strategy.md`) targeting `app-privategit-marketplace/static/products.html`
and a `/products` route — **neither exists**. This crate's ground-up rewrite replaced the static-
HTML-per-page architecture the draft assumes; real routes are `/`, `/software`,
`/software/:product_id`, `/licensing` (still a static file), `/pricing`, `/page/*`. The draft's
actual intent (meta description/OG/Twitter tags, JSON-LD, `robots.txt`, `sitemap.xml` — all
genuinely missing) is sound and matches a gap this session's own audit flagged (rubric SEO
fail in `BRIEF-software-consolidated-service-audit.md`). Corrected ground truth sent back
(msg-id `command-20260712-re-seo-draft-ready-software-pointsav-com`), including that
`layout.rs`'s `render_page()` (line 315) is the single shared head-builder where meta tags
should go for every maud-rendered page, vs. `licensing.html` needing its own direct edit.
Not implemented — new scope, awaiting either project-editorial's revised draft or an explicit
ask to pick it up.

- [x] `[2026-07-12]` SEO pass implemented — see below, this is now done.

## 2026-07-12 — pre-production hardening pass (Command/foundry-prod handoff prep)

Operator asked to fix the Tier-1 security/revenue blockers from
`BRIEF-software-consolidated-service-audit.md` (not just report them), implement SEO,
add mobile nav, then run a fresh full re-audit (code + genuine browser-in-the-loop via
Playwright) before considering a Command handoff. See
`.agent/briefs/BRIEF-software-handoff-readiness.md` for the full re-audit report.

**Shipped this pass** (all with test coverage, 134 tests total across 3 crates):
- [x] Path traversal in `release_path()` (S4, HIGH) — `is_safe_segment` gate + router-level tests
- [x] Keypair self-test (S3) — `SIGNING_KEY_SECRET`/`VERIFY_KEY_PUB` fingerprint comparison at startup
- [x] tool-wallet price/product-matching bug (R-blocker-1) — live catalog lookup replaces dead `PRICE_MAP`
- [x] os-console catalog edition/size/platform alignment (S1)
- [x] Full SEO — meta/OG/Twitter/JSON-LD/robots.txt/sitemap.xml (project-editorial's draft corrected + implemented)
- [x] Mobile hamburger nav (7 links, matches footer Site column) + fixed 2 orphaned pages (`/page/privacy`, `/page/accessibility`)
- [x] Re-audit found + fixed: `Box::leak` memory leak, path disclosure in 404 bodies, `is_safe_segment` NUL/control-char gap, hamburger keyboard-inaccessibility (WCAG 2.1.1 — checkbox pattern replaced with a real `<button>`)
- [x] Both crates' READMEs rewritten (were describing a nonexistent scaffold / wrong port + phantom env vars)

**SECURITY — found during this pass, not part of the plan:**
- [ ] `[2026-07-12 HIGH]` **os-console's deposited `install.sh` leaked real infrastructure
  details** — a real GCE IP (`34.53.65.203`), real usernames (`mathew`, `jennifer`),
  `tenant = "woodfine"`, and an embedded SSH-tunnel-back mechanism, all in a
  publicly-reachable customer install script. Localhost: quarantined (moved, not deleted,
  to `/var/lib/local-software/quarantine/os-console-install.sh.leaked-20260712-161358`),
  `os-console` pulled from the catalog (commit `6077da3c`). **Flagged to Command as HIGH
  priority (msg-id `command-20260712-urgent-os-console-install-sh-may-leak-re`) — NOT YET
  VERIFIED whether the same file is deposited on foundry-prod (the real public site).**
  Do not re-add `os-console` to the catalog until the actual script content is rewritten
  and confirmed safe — this session did not investigate where the leaking script came
  from or author a replacement.

**Still open (from the re-audit, "safe to schedule after handoff"):**
- [ ] `og:image`/`twitter:image` reference `/static/og-default.png`, which doesn't exist —
  every page's social share card is currently broken. Needs a real 1200×630 asset;
  flagged, not fabricated.
- [ ] Before any real foundry-prod push: confirm prod's systemd units set
  `SOURCE_BIND=127.0.0.1:9201` (source defaults to test port 19201) and `VERIFY_KEY_PUB`
  on the marketplace unit (so the keypair self-test actually runs there instead of
  silently `Skipped`) — config verification, Command's domain, not this archive's code.
- [ ] Chromed 404/500 + router `.fallback()` (M2); order-pending auto-refresh (M3);
  desktop masthead product nav (M4 remainder); per-version MANIFEST dead route + the
  product-detail SHA-fetch's hardcoded absolute URL tripping CSP on non-prod hosts
  (S2/M10, sharpened this pass); rate limiting; Range/caching headers; RwLock poison
  handling; unified error schema; product-detail JSON-LD/BreadcrumbList; sitemap
  product-page entries. Full list with severity in the readiness BRIEF.
