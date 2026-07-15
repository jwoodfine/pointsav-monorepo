# CLAUDE.md — app-workplace-presentation

> **State:** Active — Wave 1 | **Last updated:** 2026-05-27
> **Registry row:** `.agent/rules/project-registry.md`

---

## What this project is

`app-workplace-presentation` is a sovereign, offline-first desktop presentation
tool. PowerPoint/Keynote muscle memory on the outside; canonical output format
TBD (PPTX-compatible JSON or self-contained HTML) on the inside.

Platform: macOS 10.15 Catalina or later (Tauri v2). EUPL-1.2 licence.

> **Tauri v1 → v2, 2026-07-14 (operator-approved).** Tauri 1.x is unbuildable on this host
> (Ubuntu 24.04 dropped webkit2gtk-4.0). This crate is the highest-risk of the six: its
> `main.rs` is 11 lines with **no** IPC commands — ALL privileged work is in `src/app.js`
> via the plugin JS globals (`t.dialog.open/save`, `t.fs.readFile/readTextFile/writeTextFile`).
> So both `tauri-plugin-dialog` and `tauri-plugin-fs` are registered AND frontend-gated in
> `capabilities/default.json`. `fs.readBinaryFile` was renamed to `fs.readFile` in v2.
> **Runtime-UNVERIFIED (cargo check cannot cover this):** whether the fs scope
> (`$HOME/**`, `$APPDATA/**`) actually permits the dialog-picked paths at runtime — a file
> picked outside those roots will be denied. Needs an operator build+run pass. Floor 10.13 → 10.15.

## Current state

Foundation scaffold: Tauri v1.7 `src-tauri/` skeleton added 2026-05-27.
Frontend (`src/`) is a placeholder page. No IPC commands implemented yet.
First milestone: editor UI that creates a slide, adds text, and exports.

## Build

1. Copy icons: `cp -r ../app-workplace-memo/src-tauri/icons src-tauri/`
2. `npm install` (if package.json present) or use `cargo tauri` directly
3. `cargo tauri build` from the `src-tauri/` directory

## Wave 1 scope

- Slide editor: create, reorder, delete slides
- Text blocks with basic formatting (bold, italic, size)
- Image insert from local file
- Export to self-contained HTML (opens in any browser, prints to PDF)
- Open/save presentation files

## Hard rules

- ~~`minimumSystemVersion: "10.13"`~~ — **retired 2026-07-14**; floor is now **10.15** (Tauri v2).
- CSP is `connect-src ipc: http://ipc.localhost` (was `'none'` in v1) — required so the
  frontend's plugin calls (dialog/fs ride v2's custom-protocol IPC) work. Still zero
  *outbound network*: `ipc.localhost` is Tauri's local bridge, not a network origin. Never
  add an `https:`/wildcard origin.
- Both `tauri-plugin-dialog` and `tauri-plugin-fs` must stay registered in `main.rs`, and
  their capabilities (`dialog:allow-open/save`, `fs:allow-read-file`/`-read-text-file`/
  `-write-text-file` with the `$HOME`/`$APPDATA` scope) must stay in `capabilities/default.json`
  — the frontend calls these plugin JS APIs directly, so removing either breaks it at runtime.
- EUPL-1.2 licence: all contributions must be compatible
