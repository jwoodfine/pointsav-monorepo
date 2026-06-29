# NEXT.md — moonshot-toolkit

> Last updated: 2026-06-29
> Read at session start. Update before session end.

---

## Right now

- Task #14 decisions recorded 2026-06-29 — see Unblocked section below.
- Next: implement `build` subcommand to invoke Microkit 2.2.0 SDK.
  Add x86_64_generic / x86_64_generic_vtx targets to SystemSpec.

## Queue

- `[ ]` Implement task #14 — actual seL4 cross-compile (decisions now recorded in src/main.rs)
- `[ ]` Add x86-64 build targets (`x86_64_generic`, `x86_64_generic_vtx`) to SystemSpec and BuildPlan
- `[ ]` Wire `build` subcommand to invoke Microkit 2.2.0 SDK (aarch64-linux-gnu-gcc for ARM; native gcc for x86)
- `[ ]` Ed25519-sign output images using identity/id_pointsav-administrator key
- `[ ]` Remove `build-totebox.sh` legacy shell sketch once `moonshot-toolkit build` produces a bootable image

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

## Deferred

- `build-totebox.sh` legacy shell sketch — Deferred: kept in
  place as migration reference until Phase 1B Rust replacement is
  operational. Remove when `moonshot-toolkit build` produces a
  bootable image end-to-end. Tracked as a task #14 closure
  artefact.
- `src/main.rs` legacy stub (14-line "Forging Managed Substrate"
  print routine) — Deferred to #37 rewrite this session.
- Sigstore Cosign + customer-apex cosignature emission per
  convention §6.1 — Deferred until BuildPlan output is real
  (post-#14). The plan_hash field is in v0.1.x; cosignature
  on top of plan_hash is straightforward when binary outputs
  exist.

## Recently done

- 2026-04-27: framework §9 activation — CLAUDE.md / AGENTS.md /
  NEXT.md / ARCHITECTURE.md / DEVELOPMENT.md created; bilingual
  READMEs updated; workspace member entry added; registry row
  Scaffold-coded → Active.
