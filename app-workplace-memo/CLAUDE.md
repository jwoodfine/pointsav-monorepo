# CLAUDE.md — app-workplace-memo

> **State:** Active  —  **Last updated:** 2026-05-07
> **Registry row:** `.agent/rules/project-registry.md`

---

## What this project is

`app-workplace-memo` (Workplace✦Memo) is a sovereign, offline-first desktop
document editor. Word/Pages muscle memory on the outside; self-contained
`.html` output on the inside. The output file embeds all fonts as base64 and
opens in any browser in perpetuity. It prints to PDF via the OS print
dialogue. Zero network calls, zero accounts, zero kill switch.

Stack: Tauri (Rust backend + OS WebView frontend) + vanilla JS. No bundler,
no React, no npm runtime dependencies. EUPL v1.2.

Dev platform: macOS 10.15+ (Tauri v2). Production target: Linux (Tauri v2 — WebKitGTK).

> **Tauri v1 → v2, 2026-07-14 (operator-approved).** Tauri 1.x is unbuildable on this host
> (Ubuntu 24.04 dropped webkit2gtk-4.0). Only `tauri-plugin-dialog` was added: the frontend
> calls our own commands exclusively (`__TAURI__.core.invoke`), and all file I/O + pickers are
> Rust-side (`open_file`/`save_file` via `DialogExt`). The v1 `fs-*`/`path-all`/`shell-open`
> features and the fs allowlist scope had **no frontend consumer** and were dropped.
> **Two things to restore/verify:** (1) the `bundle.resources` globs (`../fonts/**`,
> `../templates/**`) were **removed** — v2's tauri-build hard-fails on globs that match no
> files, and both dirs are empty until `npm run embed-fonts` runs; **re-add `resources` once
> those dirs are populated.** (2) macOS floor 10.13 → 10.15.

## Current state

Scaffold complete (~47 files). Document editor with formatting toolbar,
bilingual READMEs, full ARCHITECTURE.md and DEVELOPMENT.md. The walking
skeleton has not been verified end-to-end on Linux. `CHANGELOG.md` is
entirely under `[Unreleased]` — v0.1.0 is not yet tagged.

## Build

```
npm install         # one-time
npm run tauri dev   # or: make dev
npm run tauri build # or: make build
```

Tauri v1 on macOS 10.13; Tauri v2 on Linux — see `DEVELOPMENT.md` for
platform-specific prerequisites.

## File layout

```
app-workplace-memo/
├── CLAUDE.md          this file
├── NEXT.md            open items
├── ARCHITECTURE.md    ADRs: Tauri, EUPL, CSP, font strategy
├── DEVELOPMENT.md     platform setup, prerequisites
├── CHANGELOG.md       unreleased; v0.1.0 pending
├── package.json       Tauri + npm scripts
├── Makefile           convenience aliases
├── src/               JS frontend (editor logic, toolbar, IPC)
├── src-tauri/         Rust backend (IPC commands, file I/O)
├── fonts/             bundled fonts (embedded at build time)
├── docs/              print pipeline, schema notes
└── scripts/           build helpers
```

## Hard constraints

- **No network calls.** In v2 the CSP is `connect-src ipc: http://ipc.localhost` (was
  `'none'` in v1). This is **not** a network relaxation — `ipc.localhost` is Tauri's local
  IPC bridge, which v2's `invoke()` requires (v1 IPC bypassed CSP; v2 rides a custom-protocol
  fetch). No external/`https:` origin is permitted; the no-outbound-network invariant holds.
- **No runtime npm dependencies.** Dev tooling only; runtime must be
  vendored and EUPL/Apache/MIT compatible.
- **No `unsafe-eval` in CSP.** The JS engine is written to avoid `eval()`.
- **HTML output is the canonical format.** Not `.docx`, not `.pdf` as
  source; those are derived outputs.
