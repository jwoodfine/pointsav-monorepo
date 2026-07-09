# NEXT.md — os-infrastructure

> Last updated: 2026-07-09
> State: Active (pre-release; three-node mesh test required before listing)

---

## Right now

- Activated 2026-06-29 per project framework §9. CLAUDE.md written.
- **moonshot-toolkit `build` subcommand landed** (task #14, commit `916e918b`, 2026-06-30) —
  the "blocking prerequisite" framing below is stale as of that commit.
- **First real `moonshot-toolkit build os-infrastructure/system-spec.toml` run succeeded
  2026-07-09** — produced `build/loader.img` (217 KB, ELF x86-64), `sel4.elf`, `vmm.elf`,
  `system.xml`, `report.txt`. This is a real seL4 boot image with the CAmkES VMM protection
  domain — but `pd/vmm.c` is still the two-line placeholder (prints debug strings only, no
  Linux guest, no WireGuard, no Genesis Protocol). Not signed, not an ISO, not release-ready.
- **`src/main.rs` clarified as orphaned/legacy 2026-07-09**: this bare-metal Multiboot2
  Rust crate predates the seL4+Microkit pipeline and is never loaded by
  `moonshot-toolkit build` (which consumes `system-spec.toml` + `pd/vmm.c` only). The
  EAPOL-monitor-mode packet-visualization code was retired (operator decision: Genesis
  Protocol, not EAPOL) — left as an inert compiling placeholder per "do not delete"
  scaffold policy, not deleted. **Real Genesis Protocol implementation belongs in
  `pd/vmm.c`**, not `src/main.rs` — this needs a CAmkES VMM Linux-guest-hosting
  implementation against the Microkit C API, which is substantial new systems work not
  attempted this session (no verified Microkit API reference was available to write it
  safely against).
- Legacy `forge_iso.sh` / `build_iso/` scaffold still in place as migration reference.

## Queue

- `[x]` Write `system-spec.toml` for x86-64 boot target — done (predates this NEXT.md entry)
- `[x]` Implement `build` subcommand in moonshot-toolkit (task #14) — done 2026-06-30
- `[ ]` **Implement the real CAmkES VMM in `pd/vmm.c`** — currently a placeholder. Needs:
  Linux (Debian 12) guest VCPU + memory region wiring, guest boot from
  `infrastructure/virt/work/debian-12*.qcow2`. This is the actual Genesis Protocol
  implementation target.
- `[ ]` Wire WireGuard inside Linux guest: install `wireguard-tools`, bring up `wg0` at boot,
  load config from `/etc/wireguard/wg0.conf`
- `[ ]` Wire `service-vm-fleet` + `service-vm-host` inside Linux guest as systemd units
- `[ ]` Extend build pipeline: `loader.img` → GRUB2 ISO wrap + `.qcow2` variant (currently
  produces only the raw seL4/Microkit loader image, not the distributable three-artifact set)
- `[ ]` Ed25519-sign build output with `identity/id_pointsav-administrator` once a real
  (non-placeholder) image exists

## Test milestones

- `[ ]` **Laptop A — bare metal** (VT-x, Sandy Bridge i5-2400S):
  write ISO to USB, boot, confirm seL4 starts, Linux guest up, WireGuard peer registers
  in `service-vm-fleet`. Confirm from foundry-workspace: `curl http://<wg-ip>:9203/nodes`
- `[ ]` **foundry-workspace — QEMU/TCG**:
  import `.qcow2` via `qemu-system-x86_64 -nographic`. No KVM (GCP TCG). seL4 boots,
  Linux guest up, WireGuard joins. Peer registers in fleet.

## Gate

Three-node mesh test (above + iMac os-network-admin daemon) unlocks software.pointsav.com listing.

## Deferred

- AArch64 build target — after x86-64 milestone passes and three-node mesh test completes.
  AArch64 is the verified production target (integrity proof April 2025) but comes second.
- UEFI boot — pending Neutrality Atoll decision. Use GRUB2 Multiboot for now.
- `forge_iso.sh` removal — deferred until `moonshot-toolkit build` replaces it end-to-end.
- software.pointsav.com upload — blocked on three-node mesh test + Ed25519 signing ceremony.

## Recently done

- 2026-06-29: project activation — CLAUDE.md + NEXT.md written; state: Scaffold-coded → Active.
