# CLAUDE.md — app-workplace-workbench

> **State:** Active — Wave 1 | **Last updated:** 2026-05-27
> **Registry row:** `.agent/rules/project-registry.md`

---

## What this project is

`app-workplace-workbench` is a Tauri v1.7 WebView shell for the privategit
development workbench. It loads `app-privategit-workbench` (the HTTP web IDE,
managed by project-development) at a configurable localhost port in a native
macOS window.

No logic from the workbench is forked into this crate. The HTTP server runs
independently; this app provides a desktop window to access it.

Platform: macOS 10.15 Catalina or later (Tauri v2). Apache-2.0 licence.

> **Tauri v1 → v2, 2026-07-14 (operator-approved).** This crate previously pinned
> Tauri v1.7 for macOS 10.13 High Sierra support. **Tauri 1.x cannot be built on this
> host at all:** it requires the webkit2gtk-4.0 ABI, which Ubuntu 24.04 "noble" dropped
> entirely — `libwebkit2gtk-4.0-dev` and `libjavascriptcoregtk-4.0-dev` no longer exist
> as installable packages, only transitional dummies. No apt install can fix it. Tauri 2
> targets webkit2gtk-4.1 + libsoup-3.0, both already installed, and needs zero new
> packages. **The accepted cost is that the macOS floor moves 10.13 → 10.15.**

## Architecture

- `src-tauri/src/main.rs`: reads configured port from app data dir; exposes
  `get_workbench_url` and `set_workbench_port` IPC commands
- `src/index.html`: invokes `get_workbench_url` via `__TAURI__`; navigates
  to the returned URL on load
- `src-tauri/tauri.conf.json`: CSP allows `http://127.0.0.1:*`; window starts
  at `index.html` which then redirects

## Before first build on macOS

1. Icons are already in `src-tauri/icons/` (added 2026-07-14). The old instruction to
   copy them from `app-workplace-memo` was wrong — memo had no icons either.
2. `npm install`
3. `npm run build` (or `npm run dev` for development)

## Wave 1 scope

- Port configurability: first-run UX to set the workbench port
- Window title update to show the connected URL
- Graceful error page when workbench server is not running

## Hard rules

- No code import from `app-privategit-workbench`; it remains an independent process
- ~~`minimumSystemVersion: "10.13"` must stay in tauri.conf.json~~ — **retired 2026-07-14.**
  Superseded by the operator-approved Tauri v2 migration; the floor is now **10.15**.
  Keeping 10.13 would mean keeping Tauri v1, which cannot be built on this host at all
  (see the note above). This rule is struck rather than deleted so the reversal is visible.
- CSP narrows to a specific port when the workbench port is fixed in production
- **`connect-src` in the CSP must keep `ipc: http://ipc.localhost`.** In Tauri v2 `invoke()`
  travels over a custom-protocol fetch; removing it silently breaks every IPC command with
  no build error.
- **`capabilities/default.json` must list `core:window:allow-set-title`.** `core:default`
  grants window *getters* only. Without it `setTitle()` is denied at runtime — and the call
  site swallows the rejection with `.catch(() => {})`, so the failure is silent.
