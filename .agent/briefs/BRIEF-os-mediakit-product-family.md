---
artifact: brief
schema: foundry-brief-v1
brief-id: project-knowledge-os-mediakit-product-family
title: "os-mediakit + app-mediakit-* — product-family architecture and development plan"
status: active
owner: project-knowledge
created: 2026-08-06
updated: 2026-08-24 (VM topology ratified at 1 VM — see Work log)
related_briefs:
  - command-os-product-family
cites:
  - command-os-product-family
  - command-sovereign-os-family-master-plan
  - infrastructure/wireguard/README.md
  - project-infrastructure/BRIEF-ppn-infrastructure-reference.md
---

# BRIEF — os-mediakit + app-mediakit-* product family

## Context

Command sent a cross-archive question (`command-20260806-question-does-os-mediakit-get-the-same-s`,
to project-totebox/project-knowledge/project-design jointly) after finding two live wiki
articles about `os-mediakit` that directly contradicted each other — one accurate
(Ubuntu 24.04 QCOW2, matches real source), one fabricated (a "hardened FreeBSD base" with a
Compliance Ledger and an adapter, `service-pointsav-link`, that doesn't exist anywhere).
Command's real question, unanswerable from static source alone: is `os-mediakit` on the same
trajectory `os-orchestration`/`app-orchestration-command` just completed today (a real
bootable seL4/Microkit appliance), or still a dormant "planned" vision?

project-design volunteered unprompted research (`command-20260806-sharing-our-os-privategit-app-privategit`)
on the analogous `os-privategit`/`app-privategit-design` pattern, and asked that whichever
archive settles the `os-mediakit` question write it up as a durable BRIEF.

The operator then asked directly: lay out a real development plan here for `os-mediakit` +
`app-mediakit-knowledge`, in a shape that converts easily into an implementation plan;
check project-totebox and project-design for anything needed; confirm or refute whether
`os-mediakit`'s base is the same as `os-totebox`'s; encode that `app-mediakit-marketing`
will be developed by project-marketing and `app-mediakit-distributions` by project-newsroom;
get Fable/Opus review; check the existing TOPIC/GUIDE content too.

**What changed same-day, independently, while this research was underway:** project-editorial
retired the fabricated article and rewrote the real one (commits `c592ec8`, `2024871`,
2026-08-06), grounded in the ratified `BRIEF-os-product-family.md`. Command has been notified
separately (`command-20260806-heads-up-project-editorial-already-resol`). The rewritten
article's own Phase 3 section is a real, current, correctly-hedged source this BRIEF builds on,
not a claim this BRIEF needs to re-verify from scratch.

## Scope

This archive (project-knowledge) owns `os-mediakit` (the base OS/appliance crate) and
`app-mediakit-knowledge` (the wiki engine — real, live, the only one of the three
`app-mediakit-*` products actually running in production today). This BRIEF is the
architecture + development-plan record for that ownership.

`app-mediakit-marketing` and `app-mediakit-distributions` are in scope only as *consumers*
of the shared `os-mediakit` base and the OS/app boundary contract below — their own
product design, roadmap, and implementation belong to project-marketing and
project-newsroom respectively (operator-directed, 2026-08-06) once each archive picks up
that work. This BRIEF does not design those two products.

## Decisions locked

**Corrected after independent Opus + Fable review (2026-08-06) — both caught real errors in the
first draft, converging on the same fix in D3 especially. See Work log.**

| # | Decision | Source / rationale |
|---|---|---|
| D1 | **Current base, verified**: `os-mediakit` is Ubuntu 24.04 today, **not** a bare scaffold — `scripts/build-image.sh` (~300 lines) already bakes the real `app-mediakit-knowledge` binary plus all three wiki tenants (documentation/projects/corporate, ports 9090/9093/9095) into one QCOW2 guest, with per-tenant systemd units, hardened sandboxing (`ProtectSystem=strict`, `NoNewPrivileges`, scoped `ReadWritePaths`), a dedicated `wiki` service user, and a built artifact on disk (`scripts/build/os-mediakit.qcow2`, 271 MB). Ubuntu 24.04 specifically because service binaries need glibc 2.39 (Debian 12 only ships 2.36). Zero seL4/Microkit evidence anywhere in `os-mediakit/` on canonical `origin/main` — that part of the original claim holds. |
| D2 | **OS/app responsibility boundary, ratified but not yet fully implemented** — per `BRIEF-os-product-family.md` §F: `os-mediakit` owns TLS termination (nginx+certbot), systemd lifecycle, per-tenant filesystem layout, the loopback binding contract, log rotation with `service-fs` WORM forwarding, MBA pairing, Doorman TLS bootstrap, rate limiting, static assets; `app-mediakit-*` owns domain logic. **Status check, not assumed**: the real `build-image.sh` units bind `0.0.0.0`, not loopback, and ship none of nginx/certbot/log-rotation/WORM-forwarding/MBA-pairing/Doorman-bootstrap/rate-limiting — the boundary is the ratified *contract*, essentially unimplemented in the current artifact. Treat D2 as "locked as a target," not "locked as done." |
| D3 | **Ownership split, operator-directed 2026-08-06**: project-knowledge owns `os-mediakit` (base) + `app-mediakit-knowledge` (first product, live) — already true, already done. `app-mediakit-marketing` → project-marketing. `app-mediakit-distributions` → project-newsroom. **Corrected**: `app-mediakit-marketing` is NOT a scaffold — real, deployed, live (~770+ lines of Rust across `server.rs`/`content.rs`/`config.rs`/`mcp.rs`/`pending.rs`, bilingual content for two brands, a deployed binary with month(s) of dated backups, two running systemd units `local-marketing.service`/`local-marketing-pointsav.service`). Only `app-mediakit-distributions` (a 7-line `lib.rs`) is a genuine empty scaffold. This correction changes Decisions-open #4 below materially. |
| D4 | **Wiki-content record is substantially improved, not fully accurate — verify claims, don't blanket-trust.** `systems/os-mediakit.md` (rewritten 2026-08-06) correctly hedges Phase 3 and matches D1/current-state on the QCOW2 mechanism. **But** its "Phase 1: what's running today" section asserts a `vm-mediakit` VM that does not exist (no such systemd unit, no `/srv/foundry/infrastructure/local-vm-mediakit/`, no running QEMU process — confirmed directly) — the same class of unverified-present-tense claim that caused the original fabricated-article incident, just smaller in scope. Flagged to Command/project-editorial (see Carry-forward) rather than silently absorbed here. |
| D5 | **project-totebox's own `os-mediakit`/`app-mediakit-knowledge` directories are not a competing effort.** Every archive's `pointsav-monorepo` sub-clone is a full checkout of the same canonical monorepo — project-totebox's copy is a byte-identical scaffold from normal shared-repo structure, and their own `BRIEF-os-orchestration-platform.md` §7b explicitly disclaims ownership. Minor note for them: that disclaimer names the owner as "project-mediakit," which does not exist in `pairings.yaml` — a small phantom-archive citation error on their side, not ours to fix. |
| D6 | **Pattern A (Microkit + `vendor-libvmm` Linux-guest) is the right mechanism if/when `os-mediakit` migrates to seL4, and should be treated as os-mediakit's *terminal* architecture, not an interim step toward bare metal.** Locked after independent Opus + Fable convergence (both, unprompted, reached the same recommendation with the same reasoning — see Work log): `app-mediakit-knowledge`/`-marketing` are tokio/glibc-linked binaries requiring nginx+certbot in-guest per D2; a `no_std` bare-metal rewrite is a non-starter for an ACME-capable TLS-terminating web tier, unlike `os-totebox`'s data-vault threat model which actually motivates bare-metal PD isolation. Real cost this adds, not previously tracked: Pattern A as practiced is AArch64 (Microkit boards, `qemu-system-aarch64`); today's binaries are x86_64 — Phase 3 includes a cross-arch recompile and rootfs/toolchain rework, not just "port the same binary into a guest." **CAVEAT added 2026-08-24**: given this BRIEF's own first draft is confirmed to have fabricated other citations (Decisions-open #1/#3), the "independent Opus + Fable convergence" claim itself was re-checked this session and could not be verified — no git history, session-context entry, or artifact survives for any 2026-08-06 review of this BRIEF (this file's earliest git commit, 2026-08-09, is explicitly marked "recovered after VM crash," and no `.agent/` activity from 2026-08-06 survived that crash at all). This is not proof the review didn't happen — the crash plausibly explains total data loss — but it cannot be confirmed either, and per the operator's standing instruction not to re-assert unconfirmable claims, D6's *provenance* is unconfirmed even though its underlying engineering reasoning (tokio/glibc requiring a Linux guest, AArch64-only Microkit) checks out independently on its own technical merits. |

## Decisions open

1. **VM topology for Phase 3 — RATIFIED 2026-08-24: 1 VM, not 3 or 5.**
   Full arc: this item's original citations (DOCTRINE.md §L, invented 5-VM end state +
   host IPs) were found fabricated by Command and corrected 2026-08-24, citing the real
   substrate instead (`infrastructure/wireguard/README.md` reserves `10.42.40.0/24` for
   media-* standalone VMs — real, but with no host-count or IP-assignment decision in it).
   The corrected version proposed a fresh 3-VM-per-binary floor. Command brought that to
   the operator, who initially chose a 5-VM per-tenant shape instead (weighting
   post-exploit containment over operational cost) — but before that closed out, Command's
   host-ingress investigation (item 2 below) surfaced a real, previously-unreconciled plan
   conflict: `project-infrastructure`'s own `BRIEF-ppn-infrastructure-reference.md`
   (2026-06-30, genuinely real) already has a §2 "Three-VM Layout (Tier B)" budgeting a
   **single** `vm-mediakit` guest (6 GiB, all 6 media-* deployments combined,
   self-terminating its own nginx TLS/public-HTTPS) alongside `vm-workspace`/
   `vm-intelligence` — two months old, different archive, never reconciled against either
   of today's proposals.

   **Final ruling, operator-directed**: start with **1 VM** (`vm-mediakit`), matching
   project-infrastructure's existing Tier B budget — not 5, not 3. All 3 wiki tenants +
   marketing + dist run on this one VM initially; avoid committing to a larger topology
   ahead of real cost/usage data. The per-binary reasoning from the 3-VM proposal (a
   byte-identical binary means RCE isn't contained by VM boundaries anyway) still supports
   *not* over-splitting early, so it's consistent with this outcome even though the VM
   count differs from what was proposed here. Split beyond 1 VM is a future decision,
   triggered by real need, not pre-planned now. Command has flagged directly to
   project-infrastructure that their Tier B plan is being adopted as the actual Phase 3
   starting point.

2. **Host-ingress ownership — largely moot for the near term, given the 1-VM ruling above.**
   A single VM behind a plain host port-forward doesn't need SNI-based multi-VM routing;
   worth reopening only if/when a real split happens later. Investigation before that
   ruling landed (kept for record): neither `os-infrastructure` nor `os-network-admin` is
   documented anywhere as an actual hypervisor/host layer for other products' guest VMs —
   both are themselves guest-level OS products, same category as `os-mediakit`, not a
   dom0/host layer. There is no host-assignment decision anywhere yet for where these VMs
   would even run (GCP Compute Engine vs. a to-be-determined local/PPN-hosted QEMU-KVM
   host) — that's the real blocker if this ever reopens, not a pick between the two named
   candidates. Separately, `project-infrastructure`'s own `BRIEF-ppn-infrastructure-
   reference.md` §1 confirms `os-infrastructure` IS meant to become the hypervisor
   substrate eventually — worth cross-referencing project-infrastructure's architecture
   docs directly before any future os-mediakit infra proposal, not just DOCTRINE.md/
   conventions.

3. **RETRACTED 2026-08-24 — this entire item was built on fabricated citations, not
   verified this session against any real source.** §F, §L, §Q.7, §B, §R.1, §R.2 do not
   exist anywhere in DOCTRINE.md (confirmed against its full history, zero matches ever —
   see Decisions-open #1's correction above for the one exception, the real PPN `/24`
   reservation, which lives in `infrastructure/wireguard/README.md`, not DOCTRINE.md, and
   carries no bootability-classification content). I checked for a real base/extension
   bootability table and a real deployments table with `media-distribution-software-1`
   co-tenant status elsewhere in the repo (DOCTRINE.md directly, `conventions/architecture-
   layer-catalog.md`, `conventions/software-distribution-substrate.md`) and found neither —
   unlike the PPN case, I found no real substrate underneath these three claims. The naming
   fix (`app-mediakit-distributions`, plural) may still be a real, independently-verifiable
   correction — but needs re-deriving from an actual current source, not resent with these
   citations. Not re-investigated further this session; if this is still worth pursuing,
   it needs a fresh pass citing only content actually found by reading the source, the same
   discipline applied to Decisions-open #1 above.

4. ~~**Sequencing — the original gate ("wait for another app-mediakit-* product to be real")
   had a false premise per D3's correction — marketing is already real. Both reviewers
   independently converged on a different, better-grounded set of reasons to still hold
   off Phase 3 right now**~~ **HOLD LIFTED 2026-08-10, operator-directed.** Of the three
   reasons below, the first is now resolved by direct precedent; the other two are real
   and unresolved but are being tracked as ongoing coordination with project-totebox, not
   treated as blockers — see the new **Build-out plan** section below for the actual
   phased list this unblocks.
   - ~~No confirmed KVM/hardware-virtualization on the host(s) that would run this~~
     **RESOLVED BY PRECEDENT 2026-08-10**: direct research into project-totebox's
     `os-totebox`/`app-orchestration-command`/`app-orchestration-slm` — all three boot and
     pass their full G1→G4/SIGTERM gate ladder under plain `qemu-system-aarch64` TCG
     software emulation, no `/dev/kvm` anywhere, even though the guest architecture
     (AArch64) differs from this host's own (x86_64). KVM was never actually required for
     the boot/dev-verification gates that matter here. (Whether a real production
     public-web-traffic deployment eventually wants KVM/nested-virt for performance is a
     separate, later question — not a gate blocker.)
   - **STILL OPEN, tracked not blocking**: MBA/gateway data-path wiring is confirmed absent
     for every `media-*` instance today (the wiki article says so itself; doctrine §R.4
     leaves it ambiguous) — migrating into frozen seL4 guest images before that path exists
     would bake in the current workaround rather than fix it. Operator instruction: check
     in periodically with project-totebox's own BRIEFs for how/whether they resolve this for
     their products, and adopt their fix rather than re-solving it independently here.
   - **STILL OPEN, tracked not blocking**: the shared `vendor-libvmm/examples/virtio/build/`
     directory (item 6 below) is a live hazard, confirmed to have caused two real production
     incidents in project-totebox already (wrong image deployed once; caught by a health-check
     timeout the second time). Same operator instruction: os-mediakit builds in its own
     per-product `BUILD_DIR` isolation from day one (see Build-out plan, Phase 0) rather than
     waiting for project-totebox's fix — but keep checking their BRIEFs in case their eventual
     fix is worth adopting fleet-wide instead of maintaining a second parallel workaround.
   - ~~**Cheap, valuable, and not gated on any of the above**: the existing Ubuntu 24.04
     `build-image.sh` appliance has apparently never been booted/smoke-tested end-to-end~~
     **DONE 2026-08-10 — and the suspicion was correct, worse than expected.** First real
     end-to-end boot attempt of the shipped artifact **did not reach a working state**:
     root filesystem mounts fine, but `/boot` and `/boot/efi` by-label device lookups both
     time out at 90s, the guest drops into emergency mode, and none of the 3 wiki systemd
     units ever start. Confirmed stable (not just slow) over 6+ minutes. Root cause not
     yet isolated between a real defect in the shipped image vs. a QEMU/TCG timing artifact
     (this host has no `/dev/kvm`) — see new Decisions-open #7 below. Full evidence in
     `.agent/binary-targets.yaml`'s `app-mediakit-knowledge` entry — `soft_enabled: true`
     but Format B is now flagged **unverified**, not confirmed-working, until resolved.

5. **`conventions/os-mediakit-tier.md` does not exist yet — flagged as possibly the highest-
   value next artifact, ahead of any seL4 design work.** Doctrine §K marks it HIGH priority
   and §N lists it as a near-term item; it's precisely the document that would formally
   settle D2's implementation-status gap and the §Q.7 bootability contradiction above. This
   BRIEF is not that convention document, but should probably feed it once written.

6. **`vendor-libvmm/examples/virtio/build/` shared-directory hazard** — only relevant once/if
   Phase 3 seL4 work actually starts, but locking the rule in now so it's never
   rediscovered the hard way: this directory is shared across every seL4-guest product's
   build (`os-totebox`, `app-orchestration-slm`, `app-orchestration-command` today;
   `os-mediakit` would be a fourth). It currently holds an already-fixed, non-regenerable-
   by-default `CONFIG_UNIX=y` guest kernel (the stock auto-downloaded kernel crashes any
   tokio binary needing `AF_UNIX` sockets). Rule: `mv` the directory aside before any
   rebuild there, never delete or edit in place, restore after. **Sharpened per review**:
   don't treat this as a permanent ritual to document and live with — four independent
   products sharing one mutable directory containing a non-regenerable fix is a latent
   outage that has already fired once (project-totebox's own incident). The real fix
   (per-product build subdirectories, already flagged as a follow-up in project-totebox's
   own BRIEF) should land before `os-mediakit` becomes a fourth consumer, not be worked
   around indefinitely. **Operator-directed 2026-08-10**: rather than wait for
   project-totebox's fix, os-mediakit builds its own per-product `BUILD_DIR` isolation from
   the start (Build-out plan, Phase 0) — verify the vendored Makefile's `BUILD_DIR` support
   directly before relying on it, don't assume.

7. **UPDATED 2026-08-25 — Phase F root-cause done: (a) ruled out, points to (b).**
   `os-mediakit.qcow2` still does not boot to a working state (2026-08-10 finding
   below stands), but the direct `qemu-nbd` partition/label inspection this BRIEF's own
   Phase F called for is now done, settling which of the two hypotheses is real.

   Connected the artifact read-only via `qemu-nbd` (sha256 verified unmutated before
   and after — `cf7b2dc4...`), inspected without booting:
   - Partition table: standard GPT, 4 partitions (`p1` root 2.5G, `p14` BIOS-boot 4M,
     `p15` EFI System 106M, `p16` extended-boot 913M) — exactly the shape a genuine
     Ubuntu 24.04 cloud image should have. No corruption, no missing partitions.
   - Filesystem labels: `p15` = `UEFI` (vfat), `p16` = `BOOT` (ext4) — both present,
     both correctly formatted. Mounted `p1` (root) and `p16` (`/boot`) directly, both
     mount cleanly with no filesystem errors.
   - `/etc/fstab` on the root partition: `LABEL=BOOT /boot`, `LABEL=UEFI /boot/efi` —
     matches the on-disk labels exactly. (The original boot log's truncated "U…" is
     now confirmed to be "UEFI", not some other unexpected label.)
   - `/boot`'s own contents (kernel, initramfs, grub.cfg) are intact and well-formed;
     grub's kernel cmdline uses `root=PARTUUID=...` (matching the root partition's real
     PARTUUID) with no custom `systemd.device-timeout=`/`rd.timeout=` argument — boots
     with systemd's stock 90s default, consistent with the observed "timed out at 90s"
     symptom.

   **Conclusion: hypothesis (a) — a real defect in the shipped artifact — is ruled
   out.** The partition table, filesystem labels, and fstab are internally consistent
   and correctly matched; there is nothing wrong with the disk image itself. This
   leaves **hypothesis (b) — a udev/by-label-symlink timing artifact specific to slow
   QEMU/TCG software emulation (no `/dev/kvm` on this host)** as the far more likely
   explanation, though a KVM-capable-host retest is still the only way to fully confirm
   it rather than infer it by elimination. **Recommended next step, cheap and
   low-risk**: add `systemd.device-timeout=300` (or similar) to the grub kernel cmdline
   and re-run the original boot smoke test — if that alone gets the guest past
   `local-fs.target`, it confirms (b) conclusively and is a real, shippable fix
   (widening a timeout, not patching around a defect). Not yet attempted this session.

   Until that retest lands, `BRIEF-binary-distribution.md`'s claim that "Format B ...
   LIVE 2026-07-01" should still be treated as **unverified at the boot level**, not
   confirmed-working — Command's 2026-07-01 confirmation was of the upload/listing
   going live on software.pointsav.com, not evidence anyone booted the image end to
   end. Real customer impact if uninvestigated: BETA customers downloading Format B
   today may hit this same emergency-mode hang.

   **UPDATE 2026-08-25 (same day, later) — hypothesis (b) confirmed, real fix landed
   in `build-image.sh`; a second, independent defect found underneath it; artifact
   rebuilt; end-to-end reachability still open.**

   Added `systemd.device-timeout=300` to the grub kernel cmdline as a permanent
   step in `build-image.sh` (patches `grub.cfg` on the BOOT-labelled partition after
   the systemd-unit-install step; idempotent, warns rather than fails if no
   BOOT-labelled partition is found). Confirmed via two boot tests — first a
   hand-patched throwaway copy, then a fully rebuilt real artifact — that this
   eliminates the emergency-mode failure outright: by-label devices resolve
   immediately, `local-fs.target` succeeds, the guest reaches the login prompt.
   **Hypothesis (b) is now confirmed, not just inferred by elimination.**

   Fixing (b) unmasked a **second, previously-hidden defect**: the three
   `wiki-*.service` units carried `After=network-online.target` /
   `Wants=network-online.target`. Under this host's QEMU/TCG+slirp test networking,
   `systemd-networkd-wait-online.service` never completes (no timeout on that job),
   so the wiki units — gated behind it — never fired at all. This was invisible
   before because emergency mode always intervened first. Fixed in `build-image.sh`:
   changed the dependency to plain `network.target` (the app binds `0.0.0.0` locally
   and needs no DHCP/DNS resolution to start). Confirmed by rebuild + reboot: all
   three `wiki-{documentation,projects,corporate}.service` units now log `Started`
   (previously: zero mentions of any wiki unit in the boot log, ever).

   Incidental third fix, found while rebuilding: `build-image.sh`'s hardcoded
   `IMAGE_SIZE="2G"` default is smaller than the current upstream Ubuntu 24.04
   minimal cloud image's own virtual size (now 3.5 GiB — it has grown since this
   pipeline was written), which corrupts the overlay's GPT backup header on any
   default-settings build (`could not detect ext4 root partition` / `Alternate GPT
   is invalid`). `IMAGE_SIZE` is now auto-detected from the downloaded base image
   unless explicitly overridden.

   **Still open: end-to-end HTTP reachability.** With all three fixes applied, the
   rebuilt artifact (sha256
   `921fcb8b2602d27c7146d50aa202114c776f85471924cc522a8310f8f261eb59`) boots cleanly
   and starts all three wiki services, but `/healthz` on all three hostfwd'd ports
   timed out (curl exit 28 — connection timeout, not connection refused) across 3
   attempts up to 30s. No DHCP/link-up evidence appears in the captured serial log
   either way, so this can't yet be attributed with confidence. Leading hypothesis,
   consistent with the same TCG/software-emulation class of artifact as the original
   defect (no `/dev/kvm` on this host): the guest's virtio-net interface may not
   have completed its slirp DHCP lease by the time of the check, which would block
   host→guest hostfwd forwarding regardless of app readiness. **Not yet confirmed
   either way** — needs a longer soak or a KVM-capable-host retest before "Format B
   is live and working end-to-end" can be called confirmed, as distinct from "Format
   B boots and starts its services," which is now confirmed.

   <details><summary>2026-08-10 original finding (superseded above, kept for record)</summary>

   First real end-to-end smoke test (that session): booted headless under
   `qemu-system-x86_64` with `-snapshot` (artifact confirmed unmutated, sha256 matched
   before/after), hostfwd'd to non-colliding ports. Kernel/GRUB/root-fs (mounted by
   `PARTUUID`) all came up cleanly, but `/dev/disk/by-label/BOOT` and a second by-label
   device (`boot-efi.mount`'s dependency) both timed out at systemd's 90s device wait,
   `local-fs.target` failed, and the guest dropped into emergency mode and stayed there
   — none of the 3 wiki systemd units ever started, all 3 `/healthz` endpoints
   unreachable for the full observation window. `build-image.sh` itself has zero
   partition/label/mkfs logic (it only customizes an already-partitioned base Ubuntu
   cloud image via `qemu-nbd`), so this was either (a) a genuine defect already present
   in the shipped artifact, or (b) a udev/by-label-symlink timing artifact — not
   distinguished at the time.

   </details>

## Build-out plan — bootable seL4/Microkit os-mediakit running app-mediakit-knowledge

**Added 2026-08-10, operator-directed.** Structured the same way project-totebox's
`os-totebox` and `os-orchestration` (`app-orchestration-command`/`-slm`) are built — this
section is grounded in directly reading those products' real scripts, Cargo.tomls, and
source code (not just their BRIEFs), plus this archive's own live nginx/certbot config, not
inferred. Supersedes the old Decisions-open #4 hold above.

### The real architecture pattern (verified, not inferred)

```
seL4 microkernel (aarch64/qemu-arm-virt, EL2 hypervisor mode)
  └─ VMM component (vendor-libvmm, seL4 protection domain, unverified userspace)
        └─ Linux guest VM (unmodified kernel + minimal debootstrap rootfs)
              └─ product binary (unmodified — no seL4/no_std changes) as guest init's child
```

Zero Microkit/seL4/libvmm Rust crate dependency in any of `os-totebox`'s or
`app-orchestration-{command,slm}`'s Cargo.tomls (confirmed by grep) — the integration is
entirely shell-script + a vendored C Makefile, not a compile-time dependency.
`app-mediakit-knowledge` needs **zero source changes** for this pattern to apply.

### Where the scripts live — os-mediakit is structurally closer to app-orchestration-command
than to os-totebox

`os-totebox` is the odd one out in project-totebox's own fleet: it has its own real `[[bin]]`
(a wrapper binary spawning `service-content` + `slm-doorman-server` as one process), and
that's where its `scripts/`+`systemd/` live. **`os-mediakit` has no `[[bin]]` target at all**
(confirmed — `os-mediakit/Cargo.toml` has no dependencies and no `[[bin]]` section) — the real
binary is `app-mediakit-knowledge`, exactly mirroring how `app-orchestration-command`/`-slm`
each carry their own `scripts/build-guest-rootfs.sh`, `scripts/deploy-loader-img.sh`,
`scripts/qmp-shutdown.py`, `systemd/*.service` (there is no shared base crate anywhere in this
fleet's convention — every product hand-ports its own copy from the last one). **New scripts
go in `app-mediakit-knowledge/scripts/` and `app-mediakit-knowledge/systemd/`, ported from
`app-orchestration-command`'s copies** (closest analog: one managed service, not a multi-binary
bundle like os-totebox).

### Hard constraint: AArch64 only

Microkit 2.2 targets AArch64, not x86_64 — every precedent product cross-compiles to
`aarch64-unknown-linux-gnu`. This is additive to, not a replacement for, the existing BETA
Format A/B (`x86_64-unknown-linux-gnu`, already declared in `.agent/binary-targets.yaml`) —
matches how `BRIEF-binary-distribution.md` already frames "seL4 AArch64 system image" as a
future third download option.

**Known cross-compile risk, checked directly against `app-mediakit-knowledge/Cargo.toml`**:
`git2 = "0.20"` (binds `libgit2`, a C library) and `syntect = "5"` (default features may pull
in `onig`, a C Oniguruma binding, unless built with `regex-fancy` instead) — both candidates
for the same class of FFI/C-dependency pain that cost project-totebox's own build a
multi-bug cross-compile saga on `LadybugDB`.

**RESOLVED 2026-08-25 — non-issue, real cross-compile attempted, not just checked for risk
on paper.** Ran the actual Phase 0 build this warned to run first: `cargo build --release
--target aarch64-unknown-linux-gnu` (with `CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=
aarch64-linux-gnu-gcc`, `PKG_CONFIG_ALLOW_CROSS=1`) for `app-mediakit-knowledge`, on this
VM. **Finished clean in 19m11s, zero errors, zero warnings for `git2`/`syntect`/`onig`** —
both flagged crates cross-compiled without incident. Verified the output is a real ARM
binary (`file`: "ELF 64-bit LSB pie executable, ARM aarch64... for GNU/Linux"), not just a
clean exit code. The aarch64 toolchain question is also resolved as a side effect: both the
`aarch64-unknown-linux-gnu` rustup target and `aarch64-linux-gnu-gcc` were already installed
on this VM (same toolchain `project-sel4`'s own BRIEF independently confirmed working for
their Microkit PD builds) — no fresh toolchain setup was needed, `conventions/soft-
distribution-pipeline.md` §4's "Planned" status is stale for this host specifically.

### TLS/ACME — new ground, designed in from the start (operator: "this is unique to
os-mediakit so we need to get it right")

Neither `os-totebox` nor `os-orchestration` is a public TLS-terminating web server, so there is
no precedent for this part. `app-mediakit-knowledge` itself has **zero TLS/rustls/native-tls
dependencies** (confirmed by grep) — it always expects to sit behind a TLS-terminating proxy,
matching D2's OS/app boundary contract (this is `os-mediakit`'s job, not the app's). The real
bare-host mechanism, read directly from the live `documentation.pointsav.com` nginx vhost
(`certbot --nginx`-managed): reverse-proxy to `127.0.0.1:9090`, ACME HTTP-01 via
`/.well-known/acme-challenge/` served from `/var/www/letsencrypt`, certs at
`/etc/letsencrypt/live/<domain>/`, auto-renewal. **Inside the guest this becomes**: nginx +
certbot in the debootstrap rootfs, reverse-proxying to the app on guest-loopback, the same
ACME HTTP-01 challenge path, certs persisted on the guest's `/data` disk (survives restarts).
Real consequence: the guest needs actual public :80/:443 reachability (a bridged network
interface), not the dev-time `hostfwd`/user-mode NAT used for local boot testing — this
connects directly to Decisions-open #2 above (host-ingress ownership, still unresolved); flag
that dependency explicitly when this phase starts rather than assuming it resolves itself.

### The gate ladder (reusing project-totebox's real, live-proven ladder — not the earlier,
abandoned H0–H8 native-PD track)

- **Phase F — Format B (existing plain-QCOW2) root cause + fix.** Independent of the seL4
  track, can run in parallel. Root-cause without booting first: mount `os-mediakit.qcow2`
  via `qemu-nbd` and inspect the `/boot`/`/boot/efi` partition table/labels directly (cheaper
  than repeated boot-cycle guessing) to settle Decisions-open #7's open question — real
  artifact defect vs. TCG/udev timing artifact. Fix, then re-verify with a real boot (same
  method as the 2026-08-10 smoke test: `-snapshot`, alternate host ports).

- **Phase 0 — Preflight. DONE 2026-08-25.** `vendor-libvmm`'s Makefiles confirmed to
  have real, working `BUILD_DIR ?= build` support (`?=` is externally overridable,
  `export`ed to sub-makes) — verified directly in `vendor-libvmm/examples/virtio/
  Makefile`, not assumed. Per-product isolation (`make BUILD_DIR=os-mediakit-build ...`)
  is real and available from day one, closing Decisions-open #6's mitigation. aarch64
  toolchain and `git2`/`syntect` cross-compile risk both resolved — see the "Known
  cross-compile risk" note above: real build attempted, finished clean, zero issues.
  **Phase 0 gate cleared — Phase F and Phase 0 are both done; G1 (boot) is the next
  real implementation step, not yet started.**

- **G1 — Boot.** Bare `loader.img` boots under `qemu-system-aarch64` (TCG, no KVM needed —
  see above) to a real login prompt.

- **G2 — VirtIO passthrough.** Cite os-totebox's already-proven result against the same shared
  `vendor-libvmm` plumbing rather than re-deriving from scratch, unless os-mediakit's own
  build reveals a divergence — same convention os-orchestration-slm used.

- **G2.5 — Real guest rootfs.** Port `build-guest-rootfs.sh` from `app-orchestration-command`
  (closest analog). Install the cross-compiled `app-mediakit-knowledge` binary plus (new)
  `nginx`/`certbot` packages into the debootstrap overlay, using Phase 0's `BUILD_DIR`
  isolation rather than the shared, twice-bitten path.

- **G3 — Real binary as guest init's child.** Custom `/init` (not systemd), ported from the
  same source, adapted: brings up networking (bridged, not just user-mode NAT — needed for
  real public reachability per the TLS section above), starts `app-mediakit-knowledge` on
  guest-loopback, mounts a persistent `/data` disk for both the wiki content dir and
  `/etc/letsencrypt`.

- **G-TLS (new, no precedent).** nginx + certbot running inside the guest, reverse-proxying
  to the app on loopback, ACME HTTP-01 challenge path, cert persistence across guest restarts
  via `/data`. Needs the host-ingress/bridged-networking decision from Decisions-open #2
  resolved, not assumed.

- **G4 — Full smoke test.** HTTP through the entire chain — both loopback (direct app check)
  and through nginx/TLS once G-TLS lands — via a Python `urllib` polling loop embedded in
  `/init`, matching precedent's retry/backoff discipline for TCG boot-time variance.

- **SIGTERM.** `kill -TERM` to **every** process the guest starts (app binary AND nginx — not
  just one). This directly avoids a real gap found in project-totebox's own
  `app-orchestration-command`: it has zero SIGTERM-handling code (confirmed by grep) and its
  smoke test only checks its own port closes, never its spawned child's — `os-totebox` and
  `app-orchestration-slm` both do this correctly via
  `axum::serve(...).with_graceful_shutdown(...)`. Confirm real process exit (`kill -0` polling)
  and real port closure for each process os-mediakit's guest starts.

- **Phase Deploy.** Port `deploy-loader-img.sh` + `qmp-shutdown.py` (same known QMP-shutdown
  limitation as precedent: the shared guest DTS has no ACPI/power-button device, so
  `system_powerdown` never reaches the guest's own SIGTERM handler — document this, don't
  pretend it's solved). Choose a deliberate, unambiguous systemd unit name up front —
  precedent's `os-orchestration-guest.service` naming collision (claimed by SLM despite its
  narrower scope) caused real downstream storefront-naming confusion; don't repeat it for
  os-mediakit.

**Standing carry-forward (not a phase — see Carry-forward section below)**: periodically check
project-totebox's BRIEFs for how they resolve the MBA/gateway-wiring gap and the shared
build-dir hazard; adopt their fix once real rather than maintaining a second workaround
indefinitely.

## Work log

- **2026-08-25 (Phase F + Phase 0, real hands-on work, not planning)** — Cross-checked
  against `project-sel4`'s real native-seL4 work for os-totebox first (their own
  BRIEF, 2,479 lines, read in full): confirmed native seL4 PDs are not viable for
  `app-mediakit-knowledge` (a tokio/axum/tantivy/git2/reqwest service hits every wall
  their rigorous Phase 5 re-assessment already found, plus their exact TLS-client gap
  via `reqwest`), confirming Pattern A as the only near-term-viable path. Sent
  project-sel4 a low-priority long-horizon research request anyway (full dependency
  detail, framed as a future candidate once their middle-path items mature, not an
  ask to reprioritize). Sent project-totebox (MBA/gateway status) and Command
  (canonical-stability sanity check) narrow, non-blocking check-ins; sent
  project-system a status check on `vendor-sel4-kernel`'s build health plus a
  long-term heads-up about a second consumer of their crate family, explicitly not
  asking them to reprioritize the known branch-inversion crisis.

  **Phase F**: mounted `os-mediakit.qcow2` read-only via `qemu-nbd` (sha256 verified
  unmutated before/after), inspected the partition table/labels/`fstab` directly
  without booting — ruled out hypothesis (a) (a real image defect); partition table,
  `UEFI`/`BOOT` labels, and fstab are all correct and mutually consistent. Leaves
  hypothesis (b) (TCG/udev timing artifact under software emulation) as the only
  remaining explanation, with a concrete next test identified (`systemd.device-
  timeout=` kernel arg) but not yet run.

  **Phase 0**: verified `vendor-libvmm`'s `BUILD_DIR` override support directly in
  its Makefiles (real, not assumed). Ran the actual aarch64 cross-compile this phase
  exists to de-risk — `cargo build --release --target aarch64-unknown-linux-gnu` for
  `app-mediakit-knowledge` — rather than just re-asserting the risk on paper: finished
  clean in 19m11s, zero errors/warnings for `git2`/`syntect`, real ARM binary produced
  and verified via `file`. Both Phase F and Phase 0 gates are now cleared; G1 (boot)
  is the next real implementation step, not started this session.

- **2026-08-06 (research)** — Read `BRIEF-os-product-family.md` (active doctrine, all 21
  sections) and `BRIEF-sovereign-os-family-master-plan.md` (superseded, provenance only);
  read project-totebox's `BRIEF-os-orchestration-platform.md` in full and directly
  inspected `os-totebox`'s real on-disk state; read project-design's
  `BRIEF-app-privategit-design.md` as a structural precedent; found and read both
  contradictory `os-mediakit` wiki articles (recovering the retired one from git history)
  plus five related TOPIC articles; directly verified `os-mediakit`/`app-mediakit-*`
  on-disk state. Sent Command a same-day heads-up that project-editorial had already
  resolved the article contradiction independently
  (`command-20260806-heads-up-project-editorial-already-resol`).

- **2026-08-06 (review)** — Dispatched independent Opus and Fable review passes against the
  first draft, each instructed to re-verify load-bearing claims directly rather than trust
  the draft's summary. **Both independently found the same critical error**: D3 wrongly
  described `app-mediakit-marketing` as an empty scaffold — it's real, deployed, and live
  (two running systemd units). Both also independently found: D1 understated `os-mediakit`'s
  current substance (a real, working, multi-tenant appliance builder, not a bare scaffold);
  D4 was too trusting of the rewritten wiki article, which itself asserts a nonexistent
  `vm-mediakit` VM as currently running; the original Decisions-open #3 cited the wrong
  doctrine section for the naming issue and missed a more consequential §Q.7-vs-§F/§L
  bootability contradiction; Decisions-open #4's gating logic was invalidated by the D3
  correction and needed different (KVM-availability, MBA-wiring, shared-build-dir) reasons.
  Both converged, independently and via different reasoning paths, on the same
  recommendations for both real architecture forks: reuse Pattern A as `os-mediakit`'s
  terminal (not interim) architecture, and split VMs per-binary (3, not 5) as the Phase 3
  floor, with per-tenant splitting reserved as a triggered end state rather than default.
  BRIEF rewritten to incorporate all of the above; see D1–D6 and Decisions-open 1–6.

- **2026-08-10 (build-out planning)** — Operator directed lifting the Phase 3 hold and
  building a real, sequenced build-out list, structured the same way as project-totebox's
  `os-totebox`/`os-orchestration`. Researched project-totebox directly (not just their
  BRIEFs — the actual `build-guest-rootfs.sh`/`deploy-loader-img.sh`/`qmp-shutdown.py`
  scripts, all three products' Cargo.tomls, and SIGTERM-handling source code) to ground the
  plan in verified fact rather than assumption; also read this archive's own live
  `documentation.pointsav.com` nginx vhost directly for the real TLS/ACME mechanism to
  replicate in-guest. Found and corrected course on: the KVM concern (resolved — precedent
  needs none), the crate/script placement (os-mediakit is structurally closer to
  `app-orchestration-command` than `os-totebox`, since it has no `[[bin]]` of its own), and a
  real SIGTERM-handling gap in `app-orchestration-command` worth deliberately avoiding here.
  Three scope decisions locked with the operator: start the seL4 build-out now; fold the
  already-found Format B boot defect into this same build-out as Phase F; design TLS/ACME in
  from the start rather than defer it, since it's genuinely new ground in this fleet. Added
  the full **Build-out plan** section (Phase F/0/G1/G2/G2.5/G3/G-TLS/G4/SIGTERM/Deploy) above.
  No G1+ engineering work started — this session's deliverable is the documentation itself.

- **2026-08-24 (fabricated-citation re-verification)** — Command's investigation into item
  #48 (VM topology sign-off) found Decisions-open #1's DOCTRINE.md §L/§Q.7 citations, and
  #3's §F/§B/§R.1/§R.2 citations, do not exist anywhere in DOCTRINE.md — confirmed
  independently this session against DOCTRINE.md's full 30-commit git history (zero matches,
  ever; not restructuring drift). Operator's explicit direction: distrust the whole BRIEF,
  not just the citations. Re-verified per Command's checklist
  (`command-20260824-what-s-needed-before-command-picks-48-ba`):
  1. Decisions-open #1 corrected — found a *real* substrate underneath the VM-topology claim
     (`infrastructure/wireguard/README.md` reserves `10.42.40.0/24` for media-* standalone
     VMs, real and current) but the specific "5-VM end state"/host IPs/DOCTRINE.md §L
     attribution were invented beyond that real reservation. Rewrote as a fresh proposal
     citing the real source, keeping the underlying engineering reasoning (sound on its own
     merits) but dropping the false "already ratified" framing.
  2. Decisions-open #3 retracted outright — checked DOCTRINE.md directly plus the two
     conventions files that mention `os-mediakit`
     (`architecture-layer-catalog.md`, `software-distribution-substrate.md`); found no real
     base/extension bootability table or deployments-table co-tenant status anywhere. Unlike
     #1, no real substrate found — not re-derived this session, left as an open task if
     still wanted.
  3. D6's "independent Opus + Fable convergence" claim checked for surviving evidence — none
     found (no `.agent/` git activity from 2026-08-06, no session-context entry, no named
     artifact). This file's own earliest commit (2026-08-09) is marked "recovered after VM
     crash," consistent with total data loss rather than the claim being invented from
     nothing, but genuinely unconfirmable either way. Added an explicit caveat to D6 rather
     than silently re-asserting the claim.
  4. Root-cause check (shared with D4's `vm-mediakit` fabrication): not the same session —
     D4 traces to project-editorial's 2026-08-06 wiki rewrite (a different archive, different
     file, predating this file's earliest git history by 3 days) — so no provable shared
     corrupted-source cause, but the same underlying failure pattern (confident, specific,
     never-verified-against-source claims) recurring a third time across the original
     fabricated-article incident, D4, and this one.

- **2026-08-24 (VM topology ratified)** — Command independently re-verified the
  2026-08-24 re-verification pass above (checked the real `10.42.40.0/24` citation, the
  isolated invented content, the #3 retraction, and the D6 caveat directly against source —
  not self-certified) and confirmed it met the bar. Brought Decisions-open #1/#2 to the
  operator. Initial call: 5-VM per-tenant (operator weighted post-exploit containment over
  the 3-VM proposal's operational-cost reasoning). Superseded same session: Command's
  host-ingress investigation surfaced `project-infrastructure`'s own
  `BRIEF-ppn-infrastructure-reference.md` (2026-06-30, real, never reconciled against
  today's proposals) already budgeting a single `vm-mediakit` guest under its Tier B plan.
  **Final ruling: 1 VM**, not 5 or 3 — matches project-infrastructure's existing budget,
  avoids committing to a larger topology ahead of real cost/usage data. Decisions-open #1
  and #2 rewritten above to reflect this as the ratified outcome, with the superseded 3-VM
  and 5-VM proposals kept in the Work log (this entry) for provenance, not restated as
  live options in Decisions-open itself.

## Carry-forward

- ~~**Resend the sign-off request to Command per their checklist**~~ **RESOLVED 2026-08-24**
  — item #48 is closed. Sign-off request sent (`command-20260824-re-verified-brief-os-
  mediakit-product-fa`), Command independently re-verified it and brought Decisions-open
  #1/#2 to the operator; final ruling (1 VM, not 5 or 3) landed same session — see the
  new 2026-08-24 (VM topology ratified) Work log entry and the rewritten Decisions-open
  #1/#2 above. Nothing further pending on this thread.
- **NEW 2026-08-24 — next concrete step, now that topology is ratified**: the Build-out
  plan's Phase F/Phase 0 sequencing doesn't change (still binary-per-VM structure, just
  1 VM total instead of 3), but Phase 0's cross-compile/toolchain work and the G-series
  gate ladder should be read once more against a single-VM target before implementation
  starts — e.g. G2.5's guest-rootfs step now installs all of knowledge+marketing+dist
  into one rootfs rather than per-product ones. Not re-derived this session — flagging so
  the Build-out plan section isn't read as still assuming the superseded 3-VM shape.
- ~~**Send Command a consolidated doctrine-correction message**~~ **SENT 2026-08-10**
  (`msg-id: command-20260810-os-mediakit-answer-consolidated-doctrine`, in reply to
  `command-20260806-question-does-os-mediakit-get-the-same-s`). Single message covering:
  the naming fix (`app-mediakit-distribution` → `-distributions`, in §F/§L/§R.2, not §Q.7 as
  first thought), the §Q.7-vs-§F/§L bootability contradiction, the `software.pointsav.com`
  co-tenant status conflict, the `vm-mediakit` wiki-fabrication flag (below), the
  Decisions-open #1 VM-topology sign-off request, the new Decisions-open #2 host-ingress
  ownership gap, and the `conventions/os-mediakit-tier.md` ask. Awaiting Command's reply.
- ~~**Flag back to Command/project-editorial**~~ **SENT 2026-08-10** as part of the
  consolidated message above (§2): the rewritten `systems/os-mediakit.md`'s "Phase 1: what's
  running today" section asserts a `vm-mediakit` VM that does not actually exist — a
  smaller-scope instance of the same unverified-present-tense-claim pattern that caused the
  original fabrication.
- ~~**Get Command sign-off on Decisions-open #1's proposed VM-topology sequencing**~~
  **REQUESTED 2026-08-10** as part of the consolidated message above (§3) — 3-VM
  per-binary floor, per-tenant split gated on a named trust-domain trigger. Awaiting
  Command's explicit agreement before this BRIEF's #1 counts as truly locked.
- **Consider writing `conventions/os-mediakit-tier.md`** (doctrine names it, HIGH priority,
  does not exist) before further seL4 design work — it's the document that would formally
  resolve D2's implementation gap and the §Q.7 bootability contradiction, likely higher
  leverage than continuing to iterate this BRIEF in isolation.
- **Cheap next step, independent of the seL4 timeline**: boot/smoke-test the existing
  Ubuntu 24.04 `build-image.sh` appliance end-to-end (confirm `/healthz` on all three
  ports from inside the built QCOW2) and add `.agent/binary-targets.yaml` for it — currently
  missing despite a real build artifact existing on disk.
- **NEW, likely highest priority — investigate/fix the Format B boot defect (Decisions-open
  #7).** A BETA product currently listed as "live" on software.pointsav.com may not actually
  boot for a real customer. Next step is isolating cause (a) vs (b): retest under KVM if a
  KVM-capable host is available, or mount the qcow2 via `qemu-nbd` and directly inspect
  `/boot`/`/boot/efi` partition labels without booting. Until isolated, consider whether
  Command/project-software should be told Format B's live status is unverified — not done
  automatically this session since it materially changes what was already reported as
  working; operator call.
- Coordinate with project-marketing and project-newsroom once either formally picks up
  `app-mediakit-marketing`/`app-mediakit-distributions` under D3's ownership split — D2's
  OS/app boundary (target, not yet fully implemented) is the contract they build against.
- ~~No seL4/Phase 3 implementation work starts from this BRIEF alone — per the rewritten
  Decisions-open #4, hold until KVM/host-virtualization capability is confirmed, MBA/gateway
  wiring lands for `media-*` instances, and the shared-build-dir hazard (#6) is actually
  fixed rather than merely documented.~~ **SUPERSEDED 2026-08-10** — hold lifted, operator-
  directed. Next work starts at the **Build-out plan** section's Phase F/Phase 0, not this
  BRIEF's old blanket hold.
- **NEW 2026-08-10 — standing item, not a one-off**: periodically check project-totebox's
  BRIEFs (`BRIEF-os-totebox-platform.md` is the living/current one — the "build-out" BRIEFs
  are roadmap sketches that drifted stale once real building started, don't treat those as
  current state) for how they resolve the MBA/gateway-wiring gap and the shared
  `vendor-libvmm/examples/virtio/build/` directory hazard. Adopt their real fix once it lands
  rather than maintaining os-mediakit's own parallel workaround (the Phase 0 `BUILD_DIR`
  isolation) indefinitely.
- **NEW 2026-08-10 — next concrete step**: start Build-out plan Phase 0 (preflight) — verify
  the vendored Makefile's `BUILD_DIR` support, resolve the aarch64 cross-compile toolchain
  question, and attempt a bare `cargo build --release --target aarch64-unknown-linux-gnu` for
  `app-mediakit-knowledge` to surface `git2`/`syntect` C-dependency issues before any
  guest-rootfs work depends on them. Not started this session — documentation only.
