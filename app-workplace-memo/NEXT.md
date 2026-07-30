# NEXT.md — app-workplace-memo

> Last updated: 2026-07-13
> Read at session start. Update before session end.

---

## Right now

- **Walking skeleton on Linux (Tauri v2).** The scaffold has never been
  built end-to-end on the production target. First milestone: `npm run
  tauri build` succeeds on Ubuntu 22.04 or Debian 12; the binary opens,
  creates a document, saves it, and re-opens it correctly.
  `[blocked: missing glib-2.0, not installed this session]`

- **WebKitGTK CSS `@page` verification.** Print output (`@media print`
  layout) must be verified on WebKitGTK — behaviour differs from WKWebView
  (macOS) and WebView2 (Windows).
  `[blocked: missing glib-2.0, not installed this session]` — cannot build
  the WebKitGTK target to inspect this at all until the walking-skeleton
  build above is unblocked.

## Pending

- `README.md` bilingual link audit — **finding, 2026-07-13: there is no
  `README.es.md` link to audit.** No `README.es.md` file exists anywhere in
  this app's git history (checked `git log --all --diff-filter=A` for the
  path — never added), and `README.md` contains no cross-reference to one.
  This differs from the item's premise. Compare: `app-workplace-workbench`
  and `app-workplace-gis` both carry a real `README.es.md`; memo and
  `app-workplace-proforma` are the two apps in this family missing one. This
  is a net-new-translation gap (the workspace's "bilingual READMEs" rule,
  `~/Foundry/CLAUDE.md` §6), not a broken-link fix — out of scope for a
  same-session pass here; left as an open item below rather than drafting a
  translation unreviewed.
- CHANGELOG: land v0.1.0 entry once walking skeleton is verified.
  `[blocked: missing glib-2.0, not installed this session]`
- Registry row promotion: Scaffold-coded → Active is already recorded;
  verify it reflects reality once first build is confirmed. **Partially
  confirmed 2026-07-13**: directory has a real `src-tauri/` (tauri.conf.json,
  Cargo.toml, build.rs, src/main.rs) and 33 tracked files total — substantive
  enough to support "Active" on the file-count/structure axis. The
  behavioural half of "Active" (does it actually run) is unverified and
  remains `[blocked: missing glib-2.0, not installed this session]` — see
  walking-skeleton item above. Note: this NEXT.md's own "Done" log below
  says "Scaffold created (47 files)" vs. the 33 counted just now; not
  re-litigated or corrected here since it's a historical entry and the
  discrepancy could be prior deletions rather than a miscount — flagging
  only, not resolving.
- **New finding 2026-07-13** — `src-tauri/icons/` still has no committed
  icon binaries, only `icons/README.md` explaining that icons are generated
  locally via `npx tauri icon` and deliberately not committed. Confirmed
  unchanged; no icon binaries generated or committed this session (out of
  scope — a separate shared-asset artifact covers icon consolidation across
  the app-workplace-* family elsewhere).

## Blocked

- `npm run tauri build` / `cargo check` — `[blocked: missing glib-2.0, not
  installed this session]`. Not attempted from this app's own directory this
  session (no Rust source changes were made here), but the same
  environment constraint confirmed for `app-workplace-proforma` in this
  session applies identically (same missing system library, same headless
  Linux host).
- WebKitGTK `@page` print-CSS verification — `[blocked: missing glib-2.0,
  not installed this session]` (needs a working WebKitGTK build to inspect).
- CHANGELOG v0.1.0 entry — `[blocked: missing glib-2.0, not installed this
  session]` (depends on a verified walking-skeleton build first).

## Done

- Scaffold created (47 files): JS frontend, Rust IPC, HTML/CSS, docs.
- CLAUDE.md added 2026-05-07 (Active-state conformance).
