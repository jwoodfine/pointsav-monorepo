---
artifact: brief
schema: foundry-brief-v1
brief-id: project-knowledge-os-mediakit-product-family
title: "os-mediakit + app-mediakit-* — product-family architecture and development plan"
status: active
owner: project-knowledge
created: 2026-08-06
updated: 2026-08-10
related_briefs:
  - command-os-product-family
cites:
  - command-os-product-family
  - command-sovereign-os-family-master-plan
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
| D6 | **Pattern A (Microkit + `vendor-libvmm` Linux-guest) is the right mechanism if/when `os-mediakit` migrates to seL4, and should be treated as os-mediakit's *terminal* architecture, not an interim step toward bare metal.** Locked after independent Opus + Fable convergence (both, unprompted, reached the same recommendation with the same reasoning — see Work log): `app-mediakit-knowledge`/`-marketing` are tokio/glibc-linked binaries requiring nginx+certbot in-guest per D2; a `no_std` bare-metal rewrite is a non-starter for an ACME-capable TLS-terminating web tier, unlike `os-totebox`'s data-vault threat model which actually motivates bare-metal PD isolation. Real cost this adds, not previously tracked: Pattern A as practiced is AArch64 (Microkit boards, `qemu-system-aarch64`); today's binaries are x86_64 — Phase 3 includes a cross-arch recompile and rootfs/toolchain rework, not just "port the same binary into a guest." |

## Decisions open

1. **VM topology for Phase 3 — resolved to a specific shape, needs Command sign-off since
   it touches ratified §L.** The original framing ("1 combined vs. 5 separate") was a false
   binary — both reviewers independently converged on the same middle position:
   **VM boundary per *binary*, not per *tenant*, as the Phase 3 floor — 3 VMs
   (`mediakit-knowledge-vm`, `mediakit-marketing-vm`, `mediakit-dist-vm` once real) —
   with per-tenant splitting of the knowledge VM (→ the full 5 already named in doctrine
   §L, `mediakit-knowledge-vm-1/2/3` at PPN 10.42.40.1–.3) reserved as the end state,
   triggered explicitly rather than done upfront.** Reasoning: the three wiki tenants run
   the byte-identical `app-mediakit-knowledge` binary — any RCE is exploitable against all
   three regardless of VM boundaries, so per-tenant VMs buy mainly post-exploit containment,
   not attack-surface reduction; marketing and distribution are genuinely different code
   with different exposure (distribution eventually touches payment/license logic), where a
   VM boundary earns its cost. Doctrine's own §L already accepts same-binary co-tenancy
   elsewhere (`mediakit-marketing-vm` already co-tenants two brand sites of the same
   binary) — applying that same standard to knowledge yields 3 VMs, not 5, as the starting
   shape. **Named split trigger** (not yet operator-confirmed, proposed here): split
   `mediakit-knowledge-vm` per-tenant when a wiki tenant crosses a trust-domain line —
   concretely, when `corporate.woodfinegroup.com` starts carrying regulated/BCSC-sensitive
   content, or a tenant gets an external-facing editor audience per the Record Keeping
   product vision. **This is a proposed sequencing amendment to ratified §L, not a
   fait accompli — needs Command's explicit sign-off**, since §L already names the full
   5-VM end state with allocated PPN IPs; nothing here contradicts that end state, only
   the order/pace of getting there.

2. **New — host-ingress ownership, surfaced by both reviewers, not previously tracked
   anywhere.** Once Phase 3 lands (any VM count > 1), multiple VMs sit behind one public
   IP — port 443 binds once at the host. Nobody owns SNI-based L4 passthrough or per-VM
   public IP allocation today; D2's OS/app boundary doesn't cover this (it's a host/
   infrastructure-layer concern, not `os-mediakit`-guest-layer). Needs an explicit owner
   (likely `os-infrastructure` or `os-network-admin`) before Phase 3, not assumed to fall
   out of D2 automatically.

3. **Doctrine inconsistencies to flag to Command — three, not one, corrected/expanded from
   the original single naming-typo finding:**
   - **Naming**: `app-mediakit-distribution` (singular) appears in doctrine §F's
     deployments table and §L, and again in §R.2 — not §Q.7 as originally (wrongly) cited
     here. The real crate is `app-mediakit-distributions` (plural), matching
     `architecture/six-tier-sovereignty-matrix.md`'s own already-self-flagged correction.
   - **Bootability contradiction — more consequential than the naming issue**: doctrine
     §Q.7's base/extension table classifies `os-mediakit` as `Extension` / "Bootable alone:
     No — layers onto os-console" / Reserved. This directly contradicts §F (describes it as
     a full OS tier), §L (five standalone VMs with dedicated PPN IPs), §B (a reserved `/24`
     for media standalone VMs), and the real `build-image.sh` (a genuinely standalone
     bootable QCOW2 that layers on nothing). §R.1 already tracks an §A-vs-§Q.7 conflict but
     frames it around `os-interface`, not os-mediakit's bootability — this is a second,
     separate instance of the same class of doctrine drift, not a duplicate of R.1.
   - **`software.pointsav.com` co-tenant status is stale**: doctrine §F lists
     `media-distribution-software-1` → software.pointsav.com as `simulation_status:
     co-tenant`; the freshly rewritten wiki article states software.pointsav.com is in
     practice served entirely by `app-privategit-marketplace`/`app-privategit-source`, with
     no `app-mediakit-distribution*` instance deployed anywhere. One of these is wrong;
     doctrine is the more likely stale one given the article was written same-day against
     live verification, but this needs Command's confirmation, not this BRIEF's guess.

4. **Sequencing — rewritten. The original gate ("wait for another app-mediakit-* product to
   be real") had a false premise per D3's correction — marketing is already real. Both
   reviewers independently converged on a different, better-grounded set of reasons to
   still hold off Phase 3 right now:**
   - No confirmed KVM/hardware-virtualization on the host(s) that would run this — Pattern
     A's proven track (`os-totebox`) developed and tests under QEMU/TCG software emulation
     specifically *because* AArch64 needs no hardware-virt extension; a public web-serving
     appliance under TCG-emulated seL4 may be performance-prohibitive. Verify actual
     `foundry-prod` virtualization capability before committing to a timeline.
   - MBA/gateway data-path wiring is confirmed absent for every `media-*` instance today
     (the wiki article says so itself; doctrine §R.4 leaves it ambiguous) — migrating into
     frozen seL4 guest images before that path exists would bake in the current workaround
     rather than fix it.
   - The shared `vendor-libvmm/examples/virtio/build/` directory (item 6 below) is a live
     hazard that should be hardened *before* `os-mediakit` becomes its fourth consumer, not
     after.
   - **Cheap, valuable, and not gated on any of the above**: the existing Ubuntu 24.04
     `build-image.sh` appliance has apparently never been booted/smoke-tested end-to-end,
     and this archive has no `.agent/binary-targets.yaml` despite shipping a real build
     artifact. Proving the current appliance actually boots and serves `/healthz` on all
     three ports is effectively "G0" of whatever comes next, regardless of which seL4
     answer eventually wins — worth doing now, independent of the Phase-3 timeline question.

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
   around indefinitely.

## Work log

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

## Carry-forward

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
- Coordinate with project-marketing and project-newsroom once either formally picks up
  `app-mediakit-marketing`/`app-mediakit-distributions` under D3's ownership split — D2's
  OS/app boundary (target, not yet fully implemented) is the contract they build against.
- No seL4/Phase 3 implementation work starts from this BRIEF alone — per the rewritten
  Decisions-open #4, hold until KVM/host-virtualization capability is confirmed, MBA/gateway
  wiring lands for `media-*` instances, and the shared-build-dir hazard (#6) is actually
  fixed rather than merely documented.
