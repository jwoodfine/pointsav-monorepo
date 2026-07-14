# NEXT.md — app-workplace-gis

> Last updated: 2026-07-13
> Read at session start. Update before session end.

---

## Current state

`src-tauri/` scaffolded this session (Cargo.toml/build.rs/main.rs/tauri.conf.json),
plus `package.json` and a real `src/index.html` + `src/app.js` MapLibre GL frontend.
MapLibre GL JS 4.7.1 vendored locally. Not yet compiled or browser-tested — see
CLAUDE.md "Implementation status" section for full detail.

## Wave 2 — when activating

- [x] Add src-tauri/ skeleton with WebView approach (like app-workplace-workbench) — 2026-07-13; standalone `[workspace]` table added from the start (avoids the bug fixed elsewhere this session in workbench/presentation)
- [x] Add `minimumSystemVersion: "10.13"` to tauri.conf.json — 2026-07-13
- [x] Bundle MapLibre GL JS locally (BSD-3-Clause, clean for Apache-2.0 host) — 2026-07-13; v4.7.1, `src/vendor/maplibre-gl/{maplibre-gl.js,maplibre-gl.css,LICENSE.txt}`; downloaded from unpkg (network access confirmed available in this sandbox), no CDN reference in code
- [x] Implement tile viewer pointing at configured endpoint — 2026-07-13; real (non-stub) MapLibre map init in `src/app.js`: raster XYZ base layer + GeoJSON cluster overlay, pan/zoom, click-for-details popups, PNG export. **Caveat**: raster `{endpoint}/tiles/{z}/{x}/{y}.png` URL convention is a placeholder — the real tile server's actual schema/style contract is not yet documented; revisit once known
- [x] Implement configurable endpoint (default: PPN GIS address) — 2026-07-13; `get_tile_endpoint`/`set_tile_endpoint` IPC commands, config persisted to `gis-config.json` in app data dir, first-run setup dialog + "Change endpoint…" button in sidebar; default falls back to `https://gis.woodfinegroup.com` (per CLAUDE.md) — operator can point it at a PPN address at runtime
- [ ] Smoke test: clusters render on macOS 10.13 over WireGuard PPN — `[environment-blocked: requires macOS]` (headless Linux VM, no WebView/GPU surface)
- [ ] `cargo check` in `src-tauri/` — attempted 2026-07-13; hits the same `glib-2.0`/WebKitGTK pkg-config wall as every other Tauri crate on this host (memo/proforma/workbench/presentation all confirmed same wall this session). Manifest itself is well-formed — failure is in `glib-sys`'s build script, not a syntax/dependency-declaration error. Not installing `glib-2.0` this session per standing decision.
- [ ] `node --check` on `src/app.js` — passed 2026-07-13 (syntax-valid; not a runtime/browser test)
- [ ] Add to project-software binary-targets.yaml — tracked centrally in this session's `.agent/binary-targets.yaml` sweep (see BRIEF/outbox), not duplicated here
- [ ] Real tile-server schema discovery — once the actual GIS tile endpoint's contract (raster vs. vector, style.json location, T1/T2/T3 layer definitions) is documented, replace the placeholder raster-XYZ convention and the static `get_available_layers` layer list in `src-tauri/src/main.rs` with values derived from it
- [ ] Copy real app icons from `app-workplace-memo/src-tauri/icons/` before first macOS build (placeholder icon paths only referenced in tauri.conf.json so far)
