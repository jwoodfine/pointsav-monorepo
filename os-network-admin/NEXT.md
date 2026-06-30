# NEXT.md — os-network-admin

> Last updated: 2026-06-29
> State: Active — Phase S3 done; daemon mode + Phase S4 next

---

## Right now

- Phase S3 + daemon mode in working tree (2026-06-29): fleet_watch.rs + daemon feature flag;
  `cargo build --release --features daemon` → ELF x86-64 526 KB; 3/3 tests pass.
- Next priority: AppImage packaging + iMac Linux Mint test (D7 gate).

## Queue

- `[x]` Add daemon build mode feature flag (done 2026-06-29):
  - `Cargo.toml`: `[features] daemon = []` + `[workspace]` self-contained
  - `src/fleet_watch.rs`: Phase S3 fleet watch loop — 30s poll, `wg set` via subprocess, WORM event append; 3/3 tests pass
  - `src/main.rs`: `#[cfg(feature = "daemon")]` guards; Phase S1 UDP path preserved under `#[cfg(not(feature = "daemon"))]`
  - `cargo build --release --features daemon` → ELF x86-64, 526 KB
  - `scripts/package-appimage.sh`: AppImage scaffold (requires appimagetool on PATH)
  - TODO: wire HTTP fleet polling (FLEET_URL) + HTTP WORM append (SERVICE_FS_URL) when fleet endpoint is live
- `[ ]` Package daemon as AppImage (Linux):
  - Install appimagetool from AppImageKit releases
  - Run `./scripts/package-appimage.sh <version>`
  - Produces `os-network-admin-<ver>-x86_64.AppImage`
- `[ ]` Test daemon on iMac Linux Mint (Intel x86-64, 2010-2012):
  - Set NODES_JSONL_PATH, WG_IFACE=wg0 — daemon reads peers from nodes.jsonl on startup
  - Install: `sudo apt install wireguard` + configure `wg0`
  - Run daemon: `./os-network-admin-<ver>-x86_64.AppImage`
  - Confirm peer joins fleet (service-vm-fleet at foundry-workspace)
  - Three-node mesh verified (D7): Laptop A + foundry-workspace + iMac
- `[ ]` Sign daemon AppImage with `identity/id_pointsav-administrator` Ed25519 key
- `[ ]` Upload to software.pointsav.com at $1 USDC (after three-node mesh test passes)

## Phase S4 — Genesis Protocol

- `[ ]` Wire `system-network-interface::conduct_pairing_ceremony()` to UDP server (:9206)
- `[ ]` CPace-based pairing ceremony: new node sends join request; os-network-admin
  operator approves via TUI (ratatui); pairing writes to `~/.local/share/ppn/nodes.jsonl`
- `[ ]` Test Genesis Protocol end-to-end on Laptop A bare-metal boot
- **Gate:** os-infrastructure must boot bare-metal first (Phase S4 requires a live genesis node)

## Test milestones

- `[ ]` **iMac Linux Mint (daemon)** — primary near-term test:
  - VT-x may not be available on oldest iMac hardware (Core 2 Duo Westmere)
  - Daemon mode requires no VT-x; pure x86-64 binary
  - WireGuard install: `sudo apt install wireguard` on Mint 21.x
  - Confirm: `wg show wg0` shows foundry-workspace as a peer
  - Confirm: `service-vm-fleet` at foundry-workspace lists iMac as a node

## Deferred

- AArch64 OS mode — after x86-64 daemon test passes.
- Windows daemon (`.exe`) — post three-node mesh test. Needs wintun driver for WireGuard.
- macOS daemon (`.pkg`) — post Windows. Needs Network Extension entitlement.
- Phase S5+ (per-tenant subnets, VXLAN-over-WG) — gated on Phase S4 + os-network-admin stability.

## Recently done

- 2026-06-29: project activation — CLAUDE.md + NEXT.md written; state: Scaffold-coded → Active.
- 2026-06-14, `13ef4654`: Phase S3 — fleet watch loop; auto WireGuard peer-table + WORM ledger; 8/8 tests.
- 2026-06-14, `3bafaec5`: Phase S2 — UDP :9206 listener; PING→PONG; PPN_PEERS env var.
