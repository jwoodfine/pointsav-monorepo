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
- [ ] `[2026-07-11]` Tier 1: `SIGNING_KEY_SECRET` env-var rename in the
  marketplace systemd unit + a startup keypair-match self-test across both
  services.
- [ ] `[2026-07-11]` Tier 1: real per-tier pricing + fix the amount→product
  matching bug it currently masks, before BETA lifts (also guard the
  empty-wallet-address invoice render).
- [ ] `[2026-07-11]` Tier 1: align `os-console`'s catalog `edition` with a real
  deposited version (or mint downloads through `/latest/`).
- [ ] `[2026-07-11]` Tier 2–4 items (chromed error pages, per-version MANIFEST
  fix, Contact-page city hardcoding, masthead nav, README rewrite, etc.) — full
  list in the BRIEF's Prioritized Gap List, tiers 2–4.
