# CLAUDE.md — app-workplace-proforma

> **State:** Active  —  **Last updated:** 2026-05-07
> **Registry row:** `.agent/rules/project-registry.md`

---

## What this project is

Sovereign, offline-first desktop spreadsheet. Sibling application to
Workplace✦Memo in the `pointsav-monorepo`. Excel muscle memory on the
outside; canonical `.json` file as the fiduciary record on the inside.
EUPL v1.2. Tauri + Rust + vanilla JS frontend.

The two applications share licensing posture, stack, and security
discipline but **not code** — there is no `workplace-shared/`. Each app
vendors its own dependencies.

---

## Current state (as of 2026-04-18)

- Scaffold is code-complete: ~2,600 lines across JS frontend, Rust IPC,
  HTML/CSS, and docs.
- **Never built, never run, no tests.** The Phase 1 MVP has not been
  validated end-to-end on any target.
- No CI. Release workflow in `DEVELOPMENT.md` is aspirational.
- `CHANGELOG.md` shows everything under `[Unreleased]`; `0.1.0` is TBD.

The most load-bearing next step is a walking-skeleton build on Linux
(Tauri v2) that opens, edits, saves, and re-opens a proforma file with
the audit chain intact. Until that exists, the docs describe a product
that has not yet proved it runs.

---

## Stack and constraints

| Layer | Choice | Non-negotiable because |
|---|---|---|
| Desktop shell | Tauri v2 (macOS 10.15+ dev / Linux prod) | Sovereignty — Commons Conservancy, EU jurisdiction |
| Language | Rust (backend), vanilla JS (frontend) | No framework dependency, forkable end-to-end |
| Formula engine | Phase 1 JS (`src/js/engine.js`), Phase 2 IronCalc via IPC | EU-funded sovereign path |
| File format | Canonical `.json` with SHA-256 audit chain | Fifty-year archival horizon |
| Licence | EUPL v1.2 | EU copyleft, DINUM/ZenDiS alignment |
| CSP | `default-src 'self'; connect-src ipc: http://ipc.localhost` | Zero *outbound* connections — `ipc.localhost` is Tauri v2's local IPC bridge (invoke rides a custom-protocol fetch), not a network origin; was `'none'` under v1 |
| IPC surface | Exactly 3 Rust commands: `open_file`, `save_file`, `get_app_data_dir` | Every command is attack surface |

> **Tauri v1 → v2, 2026-07-14 (operator-approved).** Tauri 1.x is unbuildable on this host
> (Ubuntu 24.04 dropped webkit2gtk-4.0). Only `tauri-plugin-dialog` was added: the frontend
> calls our own commands exclusively (`__TAURI__.core.invoke`) and prints via DOM
> `window.print()`; file I/O + pickers are Rust-side (`open_file`/`save_file` via `DialogExt`).
> The v1 `fs-*`/`path-all`/`window-print` features had no frontend consumer and were dropped.
> macOS floor 10.13 → 10.15.

Hard rules when editing:
- **Never add `unsafe-eval`** to `script-src`. The formula parser is
  written to avoid `eval()` entirely.
- **Never add a network call.** The CSP is now `connect-src ipc: http://ipc.localhost`
  (v2 IPC requirement) — this permits only Tauri's local bridge, no external origin. The
  no-outbound-network invariant is unchanged; never add an `https:`/wildcard origin.
- **Never expand the IPC surface beyond six commands** (Phase 2 adds
  `evaluate_workbook` and `parse_formula`; nothing else).
- **Never introduce a runtime npm dependency.** Dev-only tooling is
  acceptable; anything that ships inside the binary must be vendored
  and EUPL/Apache/MIT compatible.
- **Never author `.xlsx` or `.csv` as a primary format.** Canonical is
  `.json`; `.pdf` is print exhaust; `.xlsx` is legacy exhaust only.

---

## File map — what each file is for

Frontend (`src/js/`):
- `schema.js` — JSON schema defaults + shape validation
- `engine.js` — Phase 1 formula engine (~630 lines); memoised recursive
  evaluator with cycle detection. Exposed on `window.WorkplaceEngine`
  for console inspection
- `grid.js` — cell grid render, selection, keyboard nav
- `formula-bar.js` — formula bar wiring
- `toolbar.js` — formatting and toolbar actions
- `export.js` — print/PDF path and XLSX stub
- `app.js` — state machine, file I/O, menus, shortcuts

Backend (`src-tauri/src/main.rs`) — three IPC commands, path
canonicalisation guard, JSON parse validation on both sides of the IPC.

Docs (`docs/`):
- `schema.md` — canonical JSON format specification
- `engine.md` — formula engine reference + known limitations
- `print-pipeline.md` — `@media print` technical notes

---

## Phase boundary (what goes in Phase 2, not Phase 1)

Do not add any of the following to Phase 1 unless explicitly asked:
- IronCalc or Formualizer integration
- XLSX export (stubbed only in Phase 1)
- Archive integration with `os-totebox` / F12 commit / `ARCHIVE.*`
  formula namespace
- AI sidebar (three-paths pattern from `service-slm`)
- Font embedding (proforma intentionally uses the OS monospace stack)
- JSON Schema validator library (`ajv` is the Phase 2 candidate)

If a change requires any of these, flag it as scope expansion before
implementing.

---

## Development workflow

```bash
npm install                    # first-time setup — no font download step
npm run tauri dev              # or `make dev`
npm run tauri build            # or `make build`
```

Dev machine is macOS 10.13 → Tauri v1. Production target is Linux →
Tauri v2. `package.json._notes` documents the migration; frontend code
is identical between versions.

Debugging the formula engine:
```javascript
WorkplaceEngine.getAllCells()
WorkplaceEngine.getCell("C5")
WorkplaceEngine.evaluateFormula("=SUM(C2:C10)")
```

Inspecting a proforma file: `jq '.' my-proforma.json` (every field has
a human-readable name; schema is self-describing).

---

## Tone and scope guidance for work in this repo

- This is a scaffold that needs to earn its first build before it earns
  new features. Prefer "make it run" over "make it richer."
- Sovereignty claims in the docs are commitments, not aspirations.
  Every change should preserve them.
- When in doubt about architecture, check `ARCHITECTURE.md` — the ADRs
  are binding decisions, not suggestions.
- The sibling app (`app-workplace-memo`) is a useful reference for
  shared patterns (ADR-001, ADR-002, ADR-004, ADR-006, ADR-008 are
  inherited). If a question has been answered there, inherit the
  answer unless there's a proforma-specific reason to diverge.
