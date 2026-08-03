# NEXT.md — project-software

Hot open items only (≤200 lines). Historical narrative and completed work now live in the
BRIEFs (`.agent/briefs/`) and `.agent/inbox-archive.md`, not duplicated here. Pruned and
reorganized 2026-08-02 (prior version was 300 lines, mostly historical narrative and items
already confirmed resolved — see git history for the full prior text if needed).

## Verified resolved this session (2026-08-02) — closed, not tracked further

- Nested `pointsav-monorepo/` sub-clone divergence (was HIGH, 2026-07-13) — confirmed byte-identical
  to archive root now (`git rev-list --left-right --count` = 0/0).
- `os-console` install.sh leak / catalog removal — confirmed still removed from `products.yaml`,
  not re-added. (Foundry-prod verification of the same leak remains open — see Command relay below.)
- `soft-orchestration-command` naming — catalog now correctly carries `app-orchestration-command`.
- 5 empty release dirs (`os-infrastructure`/`os-interface`/`os-mediakit`/`os-totebox`/`os-workplace`)
  — confirmed still empty on disk, but correctly absent from the catalog; not a live defect.
- Spanish localization batch (`c1572659`) — confirmed present on both staging mirrors and an
  ancestor of current HEAD; not lost. Still not on canonical `origin/main` — see Command relay.
- `scratch-resolve-software` — assessed in full (see `.agent/inbox-archive.md` 2026-08-02 entry);
  not worth merging. One real finding routed to Command (xtask divergence, below).

## Routed to Command Session (out of this archive's scope per `scope-discipline.md`) — sent, awaiting action

- `self-service-promote.sh`'s nested-clone-always-wins heuristic still needs fixing (`~/Foundry/bin/`).
- `push-to-prod.sh` stale crate-path reference (pulls from old non-catalog path; comment claims a
  convention "no longer exists" that actually does).
- Workspace post-commit hook `python3` argv-too-long error on large diffs.
- **xtask/src/main.rs local/canonical divergence (new finding, 2026-08-02, HIGH)** — this archive's
  local checkout is missing the `deposit`/`characterize`/`fsl_clock` subcommands that ARE present
  and working on canonical `origin/main`; local file is stuck at a pre-2026-07-04 seL4/Tier-6
  build-orchestrator state instead. Not fixed locally — `xtask` is a shared cross-archive workspace
  member. Sent to Command (msg-id `project-software-20260802-re-scratch-resolve-software-re-assessmen`).
- Push archive-root's real HEAD to canonical `origin/main` (Spanish localization + masthead batch,
  confirmed on staging mirrors, not yet on canonical).
- Confirm prod systemd units set `SOURCE_BIND=127.0.0.1:9201` + `VERIFY_KEY_PUB` on the marketplace
  unit (keypair self-test currently silently `Skipped` without it); verify `wallet.conf`'s
  `SIGNING_KEY_SECRET` rename was actually applied (drafted 2026-07-12, never confirmed run).
- Verify `os-console`'s leaked `install.sh` isn't deposited on foundry-prod (34.168.19.68) —
  this session's host is foundry-workspace (34.53.65.203), a different machine, can't check directly.

## Governance questions relayed, not resolved unilaterally

- `os-network-admin` vs `os-infrastructure` relative FSL/$19 pricing tier — flagged in
  `BRIEF-software-hyperscaler-audit.md`'s Licensing Corrections section; needs a
  `factory-release-engineering` PR + legal review, not a project-software decision.
- tool-wallet Apache-2.0 relicensing — `Cargo.toml` already carries the license, but whether it went
  through the stated legal-review gate is unconfirmed. Flagging, not resolving.
- `legal-tokens-pointsav.yaml`'s incomplete `statement` field (drops 2 of 6 marks) — flagged to
  Command/project-editorial 2026-07-12 (msg-id `command-20260712-fyi-follow-up-legal-tokens-pointsav-yaml`).
  `app-privategit-marketplace` runtime wiring to this file deliberately held until the YAML is fixed.

## Code work — done this session (2026-08-02)

- [x] Chromed 404/500 error pages + router `.fallback()` (M2).
- [x] Order-pending page auto-refresh (M3).
- [x] Per-version `MANIFEST.json` route — falls back to product-root manifest (S2/M10).
- [x] Product-detail SHA-fetch — CSP `connect-src` allowance for the source origin (S2/M10).
- [x] `app-mediakit-distributions` added to root `Cargo.toml` workspace members.
- [x] `/es/*` extended to product-detail page (`/software/:id`) — Spanish localization follow-up.
- [x] RwLock poison recovery on the revocation set (S11).
- [x] Sitemap now includes product-detail pages (English + `/es/*`), drops the bare `/` redirect.
- [x] Product-detail JSON-LD (`SoftwareApplication` + `BreadcrumbList`).
- [x] `og:image`/`twitter:image` asset — requested from project-design, Command shipped a real
  1200×630 asset same session (`pointsav-media-assets` commit `d21ad9e`), vendored into
  `static/og-default.png`.
- [x] CLAUDE.md's stale "no nested sub-clone" claim fixed.
- [x] Rate limiting on `/verify-key` + `/admin/reload-revocation-list` (S6) — in-memory per-IP
  sliding-window limiter, 20 req/60s, no new dependency. Binary download/streaming routes
  deliberately left unlimited (see Range/caching note below).
- [x] Range/ETag/Last-Modified/Cache-Control on binary streams (S10) — all 5 download-adjacent
  routes (`.sig`, open-product, licensed download, MANIFEST, install.sh) now go through one
  `stream_file()` helper backed by `tower_http::ServeFile`, real Range support included.
- [x] Unified error schema (S15) — every error response now carries a stable `code` field
  alongside `error`; fixed the audit's own named "channel-expired" vs "channel expired"
  inconsistency along the way.

## Deliberately out of scope, not forgotten

- Full RFC 9457 Problem Details (`type`/`title`/`instance` URIs) for the error schema — considered
  during the S15 work and not pursued; no real `type` documentation pages exist to point at yet.
  `error` + `code` is the honest version of that today; revisit if/when this crate gets a real
  developer-docs surface.
- Rate limiting was deliberately NOT extended to binary download/streaming routes — those are
  the highest-legitimate-traffic paths (every real customer download goes through them), and
  tuning limits there properly wants the same context as the Range/caching work, not a separate
  bolt-on pass.
- `ui::surface::trademark_line()`'s hardcoded text is confirmed correct (matches `TRADEMARK.md`,
  home.pointsav.com's live HTML, and `factory-release-engineering`'s now-fixed
  `legal-tokens-pointsav.yaml` — commit `f169e4e`, 2026-07-27) but still a duplicate, not read
  live. Not wired this session: no crate anywhere in the workspace consumes
  `factory-release-engineering` tokens at build or runtime today, so there's no pattern to
  follow, and the two real options each carry real infra cost — a build-time `include_str!`
  creates a fragile absolute cross-repo compile dependency (breaks off this VM/layout), and a
  runtime read needs the YAML actually deployed alongside `local-software-marketplace`, which it
  isn't today. Worth doing once a real cross-repo token-consumption pattern exists somewhere in
  the workspace to follow, or once someone decides to establish one.

## New workstream — `/research` surface (2026-08-02)

Operator decision: yes, full sovereign-per-surface render model, not a narrow scope. Relayed to
Command + project-editorial. See `BRIEF-research-surface-buildout.md` for scope and status —
tracked there, not here, since it's large enough to warrant its own BRIEF.
