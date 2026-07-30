# NEXT.md — app-workplace-pdf

> Last updated: 2026-07-13
> Read at session start. Update before session end.

---

## Current state

`src-tauri/` scaffolded 2026-07-13 (Cargo.toml with standalone `[workspace]`
table, build.rs, main.rs, tauri.conf.json) plus frontend (`src/index.html`,
`src/main.js`, `package.json`). Open / render / navigate / zoom / print are
implemented (not stubs) against `pdfium-render` 0.9.

Verification status (Linux VM, 2026-07-13):
- `cargo check` on the full crate stops at the missing `glib-2.0`/WebKitGTK
  system libraries — the same wall as every other Tauri crate on this host.
  The Tauri-specific glue (IPC commands, `main()`) is therefore hand-reviewed
  only, never compiled.
- The PDFium-side core (worker thread, open/render/page-count handlers,
  library binding) WAS compiled and run headless via an extracted harness
  against the fetched linux-x64 `libpdfium.so` + a generated 3-page PDF:
  open, page count, 3 page renders to PNG data URIs, and the
  out-of-range / missing-file / no-document error paths all passed.
- `node --check src/main.js` passes; both JSON files parse.

Architecture note: pdfium-render 0.9 binds PDFium via a process-wide
`OnceCell` (binding twice panics) and its types are `!Send`, so a dedicated
`pdf-worker` thread owns the engine + open document; IPC commands reach it
over mpsc channels. Do not move `Pdfium`/`PdfDocument` into Tauri state.

## Wave 2 — when activating

- [x] Add src-tauri/ skeleton (Cargo.toml, build.rs, src/main.rs, tauri.conf.json) — 2026-07-13
- [x] Add `minimumSystemVersion: "10.13"` to tauri.conf.json — 2026-07-13
- [x] Add pdfium-render dependency to Cargo.toml — 2026-07-13 (v0.9; `image` pinned to 0.25 to match)
- [x] Download PDFium binary (Apache 2.0) from pdfium-binaries releases — 2026-07-13,
      **linux-x64 dev binary only** (`chromium/7947`, sha256 in `src-tauri/pdfium/README.md`;
      gitignored). Still needed for the real target: macOS binary — either build/obtain
      `libpdfium.a` + enable pdfium-render's `static` feature (required for release per
      CLAUDE.md), or `pdfium-mac-univ.tgz` dylib for dev. Cannot be fetched/verified from
      this Linux host; prebuilt mac dylibs may not honour the 10.13 deployment floor
      (verify with `otool` — see `src-tauri/pdfium/README.md`).
- [x] Implement: open PDF via file picker, render pages, navigate — 2026-07-13
      (Rust-side `tauri::api::dialog` picker; base64 PNG data URIs over IPC; prev/next/
      Home/End + keyboard nav + zoom steps in frontend. Core render pipeline
      runtime-verified headless; Tauri glue unverified on this host, see above.)
- [x] Implement: print via OS print dialogue — 2026-07-13 (renders all pages at 1600 px
      into a hidden print container, then `appWindow.print()` → native dialogue on macOS,
      `window.print()` fallback. Chosen over shelling out to `lp` because it shows the
      OS dialogue per CLAUDE.md scope. Output is raster at render width, not vector —
      revisit if print fidelity becomes a requirement.)
- [ ] Smoke test: open a multi-page PDF; all pages render on macOS 10.13
      `[environment-blocked: requires macOS]`
- [ ] First macOS build prerequisites: copy icons from `app-workplace-memo/src-tauri/icons/`;
      `npm install` `[environment-blocked: requires macOS]`
- [ ] Add to project-software binary-targets.yaml — new `[[bin]]` target `workplace-pdf`
      exists as of 2026-07-13; archive-level `.agent/binary-targets.yaml` declaration
      (`soft_enabled: false`) is handled by the cross-cutting sweep task, verify it landed
- [ ] Compile the full crate once `glib-2.0`/WebKitGTK are available (or on macOS) —
      first full-crate compile may surface issues in the hand-reviewed Tauri glue

## Deferred (Wave 2 scope not yet started)

- [ ] Text selection (needs pdfium text APIs + selection overlay — substantial feature)
- [ ] Pan beyond native scrollbars; fit-width/fit-page zoom modes
- [ ] Retrieve PDFs from Foundry services over WireGuard PPN
- [ ] Password-protected PDF support (currently surfaced as a clear error)
