# CLAUDE.md — app-workplace-pdf

> **State:** Scaffold-coded — Wave 2 | **Last updated:** 2026-05-27
> **Registry row:** `.agent/rules/project-registry.md`

---

## What this project is

`app-workplace-pdf` is a sovereign desktop PDF viewer and print tool.
Uses `pdfium-render` crate (Apache 2.0 — wraps Google PDFium via FFI).

Platform: macOS 10.15 Catalina or later (Tauri v2). Apache-2.0 licence.

> **Tauri v1 → v2, 2026-07-14 (operator-approved).** Tauri 1.x is unbuildable on this
> host (Ubuntu 24.04 dropped webkit2gtk-4.0). The `dialog` allowlist feature became the
> `tauri-plugin-dialog` crate (the picker is Rust-side, `open_pdf` via `DialogExt`). The
> `window-print` feature was **removed in v2** — `appWindow.print()` no longer exists;
> `printDocument()` now calls DOM `window.print()`. **UNVERIFIED on macOS:** `window.print()`
> was historically a no-op in WKWebView; if it regresses, the fix is the Tauri print plugin.
> Accepted cost: macOS floor 10.13 → 10.15.

## Wave 2 scope

- Open PDF via file picker; navigate pages
- Zoom, pan, text selection
- Print via OS print dialogue
- Retrieve PDFs from Foundry services over WireGuard PPN

## Dependency note

`pdfium-render` requires the PDFium binary to be statically linked for
macOS distribution. The PDFium binary (Apache 2.0) must be downloaded
from pdfium-binaries releases and bundled in `src-tauri/`.

## Hard rules

- ~~`minimumSystemVersion: "10.13"`~~ — **retired 2026-07-14**; floor is now **10.15** (Tauri v2).
- `connect-src` in the CSP must keep `ipc: http://ipc.localhost` — v2 `invoke()` rides a
  custom-protocol fetch; the v1 config had no `connect-src` at all, so this was added.
- `tauri-plugin-dialog` must stay registered in `main.rs`; the picker is Rust-side so it
  needs no `dialog:*` capability, but the registration is mandatory or `app.dialog()` panics.
- Print is DOM `window.print()` (v2 removed the window print API) — **verify on macOS**.
- Apache-2.0 licence — no LGPL/GPL dependencies linked
- PDFium binary is Apache 2.0 — clean to bundle
