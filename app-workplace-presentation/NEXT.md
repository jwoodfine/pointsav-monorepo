# NEXT.md — app-workplace-presentation

> Last updated: 2026-07-13
> Read at session start. Update before session end.

---

## Foundation (complete 2026-05-27)

- [x] src-tauri/ Tauri v1.7 skeleton (Cargo.toml, build.rs, src/main.rs, tauri.conf.json)
- [x] src/index.html placeholder
- [x] CLAUDE.md added; minimumSystemVersion 10.13 confirmed

## Workspace-membership fix (2026-07-13, prior agent)

- [x] `src-tauri/Cargo.toml` was missing a standalone `[workspace]` table, so `cargo check`
      failed with "current package believes it's in a workspace when it's not". Fixed
      (same pattern as `app-privategit-workbench`/`app-workplace-workbench`). `cargo check`
      now gets past that error and fails only on the environment-level missing-`glib-2.0`/
      WebKitGTK system library (same blocker already confirmed for memo/proforma/workbench)
      — `[blocked: missing glib-2.0, not installed this session]`. Re-confirmed 2026-07-13
      after the Wave 1 frontend work below: `cargo check` still fails at the identical
      `glib-sys` pkg-config point, not at any new line — no Rust was touched this pass (see
      below), so this is unchanged, not re-verified from scratch.

## Wave 1 — 2026-07-13 pass

- [x] Copy icons from `app-workplace-memo/src-tauri/icons/` — **not applicable, and not
      superseded the way `app-workplace-workbench`'s equivalent item was**: this app
      already has its own real, committed icon set at `src-tauri/icons/` (`icon.png` is a
      genuine 512×512 RGBA PNG, plus real `.icns`/`.ico`/Windows-tile PNGs — verified
      2026-07-13, not placeholders). `app-workplace-memo/src-tauri/icons/` itself has *no*
      committed icon files (README only, generated locally and gitignored) — there was
      never anything to copy from there. If this app's icon set is later folded into the
      shared `ASSET-app-workplace-icons-v1` artifact (staged elsewhere in this sweep) for
      brand consistency across the workplace-app family, that is a deliberate design
      decision to make later, not a currently-blocked TODO.
- [x] Add package.json for Tauri CLI dev workflow — added 2026-07-13, mirroring
      `app-workplace-memo`'s convention (name/version match the pre-existing
      `package-lock.json`, which already pinned `@tauri-apps/api@1.6.0` +
      `@tauri-apps/cli@1.6.3` as devDependencies before any `package.json` existed).
      Verified: `node -e` JSON-parse round-trip confirms name/version/devDependencies
      match the lockfile exactly.
- [ ] Run `npm run build` on macOS 10.13; verify binary opens
      `[environment-blocked: requires macOS]`
- [x] Design slide canvas: HTML/CSS-based, single-file output goal — implemented
      2026-07-13: `src/index.html` + `src/style.css` + `src/app.js`. Fixed 960×540
      (16:9) canvas, slide thumbnails in a left sidebar, absolutely-positioned blocks
      within the canvas, a right-hand properties panel. Pure frontend — no new Rust IPC
      commands; uses Tauri v1's built-in `window.__TAURI__.dialog`/`.fs` JS surface
      (exposed via `tauri.conf.json`'s existing `build.withGlobalTauri: true`), since
      there is no bundler in this project and `src-tauri/src/main.rs` had zero commands
      to pattern-match against.
- [x] Implement slide create/reorder/delete — working: `+ Slide` inserts after the
      current slide; sidebar ↑/↓ buttons reorder; `×`/"Delete Slide" removes (blocked
      below 1 remaining slide, with a toast). All in `src/app.js` (`addSlide`,
      `moveSlide`, `deleteSlide`).
- [x] Text block with bold/italic/size controls — working: `+ Text` adds a block; the
      properties panel (right) has a content textarea, bold/italic checkboxes, and a
      font-size number input, all live-bound via `updateBlock()`.
- [x] Image insert from local file via Tauri dialog API — working code path:
      `t.dialog.open()` with an image-extension filter → `t.fs.readBinaryFile()` →
      base64-encode (chunked, to avoid call-stack limits on `String.fromCharCode.apply`
      for large files) → embedded as a `data:` URI image block. Not exercised end-to-end
      in this session — this host has no GUI/webview to actually click through a file
      picker; reviewed by hand instead of run. `img-src 'self' data:` is already present
      in `tauri.conf.json`'s CSP, so no config change was needed for the data URIs.
- [x] Export to self-contained HTML — working: `exportHtml()` builds one standalone
      `.html` file (inlined `<style>`, inlined base64 image data, a tiny inline nav
      script) via `t.dialog.save()` + `t.fs.writeTextFile()`. Includes an
      `@media print` rule (`page-break-after: always` per slide, absolute positioning
      forced to static-friendly `display:block`) to satisfy the CLAUDE.md "prints to
      PDF" requirement. Not opened in a real browser this session (no GUI) — reviewed by
      hand; the generated markup was eyeballed for well-formedness, not rendered.
- [x] Open/save presentation files — working: JSON schema
      `{schema: "workplace-presentation-v1", slides: [...]}`; Open validates shape before
      replacing state; Save writes to the remembered path, Save As always re-prompts.
- [ ] Smoke test on WireGuard PPN: no network calls leak
      `[environment-blocked: requires macOS]` — also structurally moot until a macOS
      build exists to smoke-test in the first place.
- [ ] Add to project-software binary-targets.yaml once first build passes — not this
      session's scope (cross-app `binary-targets.yaml` declaration is handled once for
      all app-workplace-* apps together, per the parent sweep plan); still gated on a
      real macOS build succeeding, which remains `[environment-blocked: requires macOS]`.

### Honest scaffold-vs-working assessment (2026-07-13)

All six Wave 1 feature letters (a–f: data model + open/save, canvas UI, create/reorder/
delete, text formatting, image insert, HTML export) have a genuine, non-stub
implementation in `src/app.js` — not placeholder functions. The caveat is environmental,
not implementation depth: nothing in `src/` could be exercised in a real Tauri WebView or
browser this session (headless Linux VM, no GUI, `cargo check` can't get past
`glib-2.0`), so **all of it is hand-reviewed, not runtime-verified**. `node --check
src/app.js` passed (syntax only). Treat this as a strong first pass that needs a real
click-through pass on macOS (or any machine with a GUI browser open on `src/index.html`)
before calling Wave 1 done.
