# NEXT.md — moonshot-toolkit

> Last updated: 2026-06-30
> Read at session start. Update before session end.

---

## Right now

- **Task #14 IMPLEMENTED** (commit `916e918b`, 2026-06-30) — `build` subcommand is real.
- **BLOCKED: disk full** — `/srv/foundry/cargo-target/mathew/debug/` ENOSPC.
  `cargo check` passed (exit 0). `cargo test` and first `build` run blocked until disk freed.
  Command fix: `rm -rf /srv/foundry/cargo-target/mathew/debug/incremental/`
- **First build run**: `moonshot-toolkit build os-infrastructure/system-spec.toml`
  → should produce `os-infrastructure/build/loader.img` via Microkit 2.2.0.

## Queue

- `[ ]` First real build run: `moonshot-toolkit build os-infrastructure/system-spec.toml` [2026-06-30 totebox@claude-code]
- `[ ]` Run `cargo test` to confirm all 4 new spec tests + updated plan test pass [2026-06-30 totebox@claude-code]
- `[ ]` Ed25519-sign output images using identity/id_pointsav-administrator key
- `[ ]` Remove `build-totebox.sh` legacy shell sketch once `moonshot-toolkit build` produces a bootable image
- `[ ]` AArch64 path: add `qemu_virt_aarch64/debug` variant to os-infrastructure system-spec.toml once x86 path confirmed

## Unblocked — task #14 decisions recorded (2026-06-29)

Three blocking decisions from the original task #14 are now resolved:

1. **Toolchain:** Microkit 2.2.0 SDK. Primary target: `aarch64-linux-gnu-gcc`
   (AArch64 EL2 — the only verified hypervisor-mode config as of mid-2026;
   integrity proof April 2025, UK NCSC funded). Also: `x86_64_generic` +
   `x86_64_generic_vtx` (added in Microkit 2.1.0, November 2025; runtime/dev
   target — no formal verification on x86 VT-x).

2. **seL4 source vendoring:** `vendor-sel4-kernel/` (1,074 files, seL4 15.0.0,
   released 2025-03-31, already vendored). No git submodule; no network at
   build time. Microkit SDK pins to seL4 15.0.0.

3. **Reproducible-build harness:** BuildPlan content-addressed `plan_hash`
   already implemented in `src/plan.rs`. Ed25519-sign output images with
   `identity/id_pointsav-administrator` key (same key used for software.pointsav.com
   distribution).
