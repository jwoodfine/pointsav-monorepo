# NEXT.md — app-workplace-proforma

> Last updated: 2026-07-13
> Read at session start. Update before session end.

---

## Current state

Active scaffold (~45 files). Tauri v1.7 + Rust + vanilla JS frontend.
`src-tauri/tauri.conf.json` has `minimumSystemVersion: "10.13"` ✓.
EUPL v1.2. CLAUDE.md present.

## Wave 2 scope (foundation laid)

This app is Wave 2 — foundation committed 2026-05-27 to allow testing notes
and session context to accumulate. Active development starts after Wave 1
trio (workbench, memo, presentation) reaches exit criteria.

## Pending

- [x] Verify `minimumSystemVersion: "10.13"` is present in tauri.conf.json —
  confirmed present 2026-07-13 (`src-tauri/tauri.conf.json` `tauri.bundle.macOS.minimumSystemVersion`).
  No edit needed; this was already correct.
- [ ] Smoke test: build on macOS 10.13; binary opens and creates a spreadsheet
  `[environment-blocked: requires macOS]`
- [x] Confirm EUPL-1.2 is consistent across all source files — audited 2026-07-13.
  Only one `.rs` source file exists (`src-tauri/src/main.rs`); its 3-line header
  (`Workplace*Proforma — Sovereign Spreadsheet for Institutional Analysis` /
  `Copyright © 2026 PointSav Digital Systems` / `Licensed under the European
  Union Public Licence v1.2 (EUPL-1.2)`) matches the wording/format used in
  `app-workplace-memo` and `app-workplace-presentation`. `build.rs` has no
  header in any of the three sibling crates — consistent, not a gap. No fix needed.
- [ ] Add to project-software binary-targets.yaml when Wave 2 begins
- [x] Wire endpoint configuration: connect proofreader (9097) and Doorman (9092) —
  partially done 2026-07-13. Added a `ServiceEndpoints` struct in
  `src-tauri/src/main.rs` (defaults `http://127.0.0.1:9097` /
  `http://127.0.0.1:9092`) registered via `.manage(ServiceEndpoints::default())`.
  Declaration only: no HTTP client dependency added, no code path dials either
  URL, and it is **not** exposed as a new IPC command (this crate's own
  CLAUDE.md caps the IPC surface at 6 commands total — Phase 1 has 3, Phase 2
  adds exactly 2 more; a config-fetch command would be an uncounted 4th/7th).
  Wiring an actual outbound call would also need to be reconciled first
  against this crate's own hard rule "Never add a network call" / `connect-src
  'none'` — flagging that tension rather than deciding it unilaterally.
  `cargo check` could not compile past the pre-existing `glib-2.0` wall to
  verify this by build (see below) — reviewed by hand only.

## Blocked

- `cargo check` / `npm run tauri build` — `[blocked: missing glib-2.0, not
  installed this session]`. Confirmed 2026-07-13: `cargo check` in
  `src-tauri/` fails compiling `glib-sys v0.15.10`'s build script (`pkg-config`
  cannot find `glib-2.0.pc`) — the same headless-Linux wall as before this
  session's edits, not a new failure introduced by the `ServiceEndpoints`
  change. Compiler verification of that change is therefore not possible in
  this environment; it was reviewed by hand only.
- macOS 10.13 build/open/create-spreadsheet smoke test —
  `[environment-blocked: requires macOS]`.
