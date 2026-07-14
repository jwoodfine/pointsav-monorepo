# CLAUDE.md — app-workplace-gis

> **State:** Scaffold-coded — Wave 2 | **Last updated:** 2026-07-13
> **Registry row:** `.agent/rules/project-registry.md`

---

## What this project is

`app-workplace-gis` is a sovereign desktop GIS viewer. Tauri WebView shell
loading a MapLibre GL-based tile viewer that connects to gis.woodfinegroup.com
(or a local tile server) over WireGuard PPN.

Platform: macOS 10.15 Catalina or later (Tauri v2). Apache-2.0 licence.

> **Tauri v1 → v2, 2026-07-14 (operator-approved).** Tauri 1.x cannot be built on this
> host — Ubuntu 24.04 dropped the webkit2gtk-4.0 ABI. Tauri 2 targets webkit2gtk-4.1 +
> libsoup-3.0 (already installed). The `dialog-open` allowlist feature became the
> `tauri-plugin-dialog` crate; the native GeoJSON picker (`load_geojson_file`) now uses
> `DialogExt` + `blocking_pick_file()` from Rust. Accepted cost: macOS floor 10.13 → 10.15.

## Architecture

WebView approach (same as app-workplace-workbench): the tile viewer runs as
a local web page loaded by Tauri. The GIS tile data is served from the
configured endpoint. MapLibre GL JS runs inside the WebView.

## Wave 2 scope

- View cluster map (T1/T2/T3 layers)
- Configurable endpoint (default: gis.woodfinegroup.com or PPN address)
- Navigate, zoom, click clusters for details
- Export current view

## Before first build on macOS

1. Icons are already in `src-tauri/icons/`.
2. `npm install`
3. `npm run build` (or `npm run dev` for development)

## Hard rules

- ~~`minimumSystemVersion: "10.13"`~~ — **retired 2026-07-14**; floor is now **10.15**
  (Tauri v2 migration; keeping 10.13 would mean keeping v1, which is unbuildable on this host).
- CSP allows tile server endpoint and MapLibre CDN (or bundle MapLibre locally). In v2 the
  `connect-src` directive must keep `ipc: http://ipc.localhost` — `invoke()` rides a
  custom-protocol fetch; removing it silently breaks every IPC command.
- `tauri-plugin-dialog` must stay registered in `main.rs` (`.plugin(tauri_plugin_dialog::init())`).
  The dialog is called from Rust, so it needs **no** `dialog:*` capability entry — but the
  plugin registration is still mandatory or `app.dialog()` panics at runtime.
- Apache-2.0; MapLibre GL JS is BSD-3-Clause — clean to bundle

## Implementation status (2026-07-13)

`src-tauri/` scaffolded, mirroring `app-workplace-workbench`'s pattern (standalone
`[workspace]` table in `Cargo.toml` from the start). MapLibre GL JS 4.7.1
(BSD-3-Clause) vendored locally at `src/vendor/maplibre-gl/` — no CDN reference.

IPC commands (`src-tauri/src/main.rs`): `get_tile_endpoint`/`set_tile_endpoint`
(config persisted to `gis-config.json` in app data dir, default
`https://gis.woodfinegroup.com`), `has_gis_config` (first-run detection),
`get_available_layers` (static T1/T2/T3 placeholder — see doc comment;
replace with a style-derived layer list once the real tile server's schema
is known), `load_geojson_file` (native file-picker via `dialog-open`
allowlist feature, reads a local GeoJSON overlay).

`src/app.js` initializes a real (non-stub) MapLibre GL map: raster XYZ base
layer against the configured endpoint (`{endpoint}/tiles/{z}/{x}/{y}.png` —
a placeholder tile-URL convention, not a confirmed server contract), a
`clusters` GeoJSON source/circle layer colored + filtered by `tier` property
(t1/t2/t3) with sidebar checkboxes, click-for-details popups, pan/zoom via
`NavigationControl`, and "Export current view" (canvas → PNG download).

**Not yet verified**: `cargo check` in `src-tauri/` reaches the same
`glib-2.0`/WebKitGTK pkg-config wall every other Tauri crate in this monorepo
hits on this headless Linux host (confirmed — manifest itself is well-formed,
compilation just can't proceed past `glib-sys`). No browser/WebView exists in
this environment to exercise `app.js` — treat it as hand-reviewed, not
smoke-tested. macOS build/open/render smoke test is
`[environment-blocked: requires macOS]`.
