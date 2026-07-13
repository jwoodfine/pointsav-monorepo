# NEXT.md — app-workplace-workbench

> Last updated: 2026-07-13
> Read at session start. Update before session end.

---

## Foundation (complete 2026-05-27)

- [x] src-tauri/ Tauri v1.7 skeleton with `get_workbench_url` / `set_workbench_port` IPC
- [x] src/index.html loads configured URL via __TAURI__ invoke
- [x] tauri.conf.json: minimumSystemVersion 10.13, CSP allows localhost
- [x] README.md + README.es.md, CLAUDE.md, package.json

## Workspace-membership fix (2026-07-13)

- [x] `src-tauri/Cargo.toml` was missing from the root `Cargo.toml`'s `workspace.members`
      list, so `cargo check` failed with "current package believes it's in a workspace
      when it's not". Fixed with the same pattern already used for
      `app-privategit-workbench`/`app-workplace-http-prototype`: added a standalone empty
      `[workspace]` table to `src-tauri/Cargo.toml`. `cargo check` now gets past that
      error and fails only on the environment-level missing-`glib-2.0`/WebKitGTK system
      library (same blocker already confirmed for memo/proforma) —
      `[blocked: missing glib-2.0, not installed this session]`.

## Wave 1 — active

- [x] First-run port configuration screen: show a setup dialog if `workbench-config.json`
      is absent — implemented 2026-07-13 in `src/index.html` using the existing
      `has_workbench_config` (new IPC command) / `set_workbench_port` commands.
- [x] Graceful error page when workbench server is not reachable (retry button + port
      change link) — implemented 2026-07-13 in `src/index.html`; reachability checked via
      a timeout-bound `fetch(url, {mode:'no-cors'})` before navigating.
- [x] Update window title to show connected URL and connection status — implemented
      2026-07-13 via `window.__TAURI__.window.appWindow.setTitle(...)` (already exposed by
      `withGlobalTauri`+`window.all` in `tauri.conf.json`; no new Rust IPC needed).
      Title cycles: "Workplace Workbench — Connecting…" → "— Connected" or
      "— Disconnected". Not build-verified beyond `cargo check` reaching the glib-2.0
      wall — see blocked items below.
- [ ] Copy icons from `app-workplace-memo/src-tauri/icons/` before first build —
      **superseded — see `ASSET-app-workplace-icons-v1` (staged to drafts-outbound
      2026-07-13)**. Do not copy ad hoc; use the shared artifact once it clears
      project-design review.
- [ ] Run `npm install && npm run build` on macOS 10.13; verify binary opens
      `[environment-blocked: requires macOS]`
- [ ] Smoke test: workbench loads over WireGuard PPN (`http://10.8.0.9:<port>`)
      `[environment-blocked: requires macOS]`
- [ ] Add to project-software `binary-targets.yaml` once first build passes — target
      declared in `.agent/binary-targets.yaml` with `soft_enabled: false` 2026-07-13
      (build not yet passing on this host — see workspace-membership fix note above);
      flip to `soft_enabled: true` only after a real macOS build succeeds.

## Pending decisions — resolved 2026-07-13

- [x] Confirm actual port used by app-privategit-workbench — **confirmed**: STABLE
      instance on port **9210** (systemd unit `app-privategit-workbench.service`,
      externally reachable at `10.8.0.9:9200` via the nginx PPN proxy), DEV instance on
      port **9215** (systemd unit `local-workbench-dev.service`, externally at
      `10.8.0.9:9207`). The previously assumed port 3000 was wrong — `main.rs`'s
      `DEFAULT_PORT` constant still defaults to 3000 for the *first-run* case (no config
      file yet), which is fine as a bootstrap default since the first-run dialog above
      now lets the operator set 9210/9215 explicitly; not hardcoding 9210 as the
      compile-time default since this shell is meant to be portable to other configured
      instances.
- [x] Determine if `app-privategit-workbench` will run as a system service or manual
      launch — **confirmed**: runs as a **systemd service** (both instances above), not a
      manually-launched process.
