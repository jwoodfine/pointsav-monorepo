---
artifact: brief
schema: foundry-brief-v1
brief-id: project-orchestration-totebox-integration
title: "os-orchestration ↔ os-totebox integration — shared cross-archive BRIEF"
status: active
owner: project-orchestration
created: 2026-07-16
updated: 2026-07-16
---

> **Shared BRIEF — two-archive contribution model.** This BRIEF is jointly owned by
> `project-orchestration` and `project-totebox`. Each archive contributes its own
> section under "Contributions" below, written from its own independent investigation
> (not copied from the other side's write-up). Neither archive edits the other's
> section. Coordination decisions that require both sides get recorded under "Decisions
> locked" / "Decisions open" once both contributions exist and are reconciled.

## Context

`app-orchestration-command` (project-orchestration, port 8020) is the "os-orchestration"
CommandCentre hub. `os-totebox` / `service-slm` (Doorman) / `service-content`
(project-totebox) is the sovereign data-vault side. The two are meant to connect — os-
orchestration as a stateless aggregation layer, os-totebox as the key-holding vault
behind it (BRIEF-sovereign-os-family-master-plan.md §2) — but ownership of the actual
connecting piece, `app-orchestration-slm` (the "Yo-Yo" GPU broker chassis), drifted:
it's project-orchestration's crate per `PROJECT-CLONES.md`, but rode into
project-totebox as a 2026-06-20 merge artifact (from the now-archived `project-data`)
and was never redistributed back. A 2026-07-08 redistribution request
(`command-20260708-brief-redistribution-request…`) is still outstanding.

This BRIEF exists because the operator asked, directly: is os-orchestration ready to
launch and connect to os-totebox to prove the engineering out end-to-end? The honest
answer required understanding project-totebox's actual live code, not just docs — so
this became a joint investigation, and needs a joint next step.

## Scope

- Reconcile the "Tier" vocabulary across both archives' code and Doctrine.
- Establish ground truth on what's actually live vs. planned on both sides.
- Decide (jointly, operator-ratified) who owns `app-orchestration-slm`'s lifecycle
  going forward, and whether/how `app-orchestration-command`'s child-supervisor gets
  used.
- Answer the operator's merge question: should `project-orchestration` and
  `project-totebox` become one archive?
- Agree a concrete path to a real, provable end-to-end Tier 0 test.

## Contributions

### project-orchestration's contribution (2026-07-16)

Investigated via two Opus deep-think agents (read-only, real code + commits, one round
of web cross-check for the GCP finding) plus direct verification in this archive.

**1. "Tier 0" is real, but not what it sounds like.** Three vocabularies get conflated:
   - Doorman router tiers (`router.rs`): A = local llama-server, B = Yo-Yo GPU, C =
     external API.
   - Doctrine claim #40 commercial ladder: Tier 0 = Doorman as pure gateway to an
     **external** LLM.
   - `SLM_TIER=0` "Tier 0 Doorman mode" (new project-totebox code): a Doorman with no
     local backend at all — every call routes through `app-orchestration-slm`'s
     `POST /v1/inference`, which relays to the Yo-Yo chassis.

   Code-Tier-0 ≠ Doctrine-Tier-0 (internal chassis vs. external LLM). Doctrine has not
   been amended to reflect the new code-level meaning — flagging as a real doc gap.

**2. The operator's hypothesis — half right.** "os-totebox's service-slm/service-content
   are the Tier A/Tier 0 client side" — **correct**: `OrchestrationTierClient` (the
   Tier 0 client) lives in `service-slm`; `service-content`/DataGraph stays sovereign,
   per project-totebox's own locked decision.

   "yoyo was launched BY os-orchestration" — **not as actually built today**:
   - `app-orchestration-slm` runs as its own independent systemd unit
     (`local-orchestration-slm.service`, port 9180, live, healthz OK). Nothing in
     os-orchestration launches it.
   - `app-orchestration-command` has an unused child-supervisor
     (`orchestration-command/src/child.rs`) capable of spawning it —
     `COMMAND_SLM_BINARY` is unset live; `/readyz` reports `slm_child:"not_configured"`.
   - The live Doorman isn't even in Tier 0 mode: no `SLM_TIER=0` set; it runs
     Tier-A-first with Yo-Yo pointed *directly* at the batch VM
     (`SLM_YOYO_ENDPOINT=http://10.128.0.24:8080`), bypassing the chassis entirely.

   Chassis and client both exist, chassis is live, but they're peer standalone
   services today, not parent/child — and the specific config that would prove the
   Tier-0-mediated path isn't deployed anywhere yet.

**3. "yoyo-batch" (the new GCP config).** A single VM (`g2-standard-4`, 1× L4 24GB,
   `us-central1-a`, `10.128.0.24`), `llama-server` on `:8080`, OLMo-3 32.2B. The
   "trainer L4 / graph H100" two-node fleet described elsewhere is aspirational — all
   three chassis labels point at this one VM; `ORCHESTRATION_YOYO_*_ENDPOINT` values are
   blank in the live chassis config.

   The flex-start migration (Session 15: stop→start replaced with
   delete`--keep-disks=boot`→recreate, because the original VM was provisioned
   `STANDARD` and never requeued on stop/start) is done, live, and verified. But
   incompletely rolled out: `idle_monitor.rs`'s matching fix is committed but the live
   `local-doorman.service` runs the old stop-based binary and is currently in **failed**
   state; `yoyo-manual-cycle.sh` and `provision_vm_in_zone()` in `start-yoyo.sh` are
   stale.

   **GCP web cross-check:** now that the VM would be genuinely flex-start-created,
   GCP's own docs describe `stop`→`start` with `instanceTerminationAction=STOP` as a
   lighter, documented pattern that preserves IP + boot disk and still requeues
   capacity — versus nightly delete+recreate, which is heavier and is what caused the
   mid-migration metadata-loss incident already recorded in project-totebox's own docs
   (`vllm-weights-prep.service` needs metadata that delete discards). Worth
   re-evaluating once the VM's provisioning model is confirmed flex-start. Sources:
   [Flex-start VMs for Compute Engine Instance API](https://cloud.google.com/blog/products/compute/introducing-flex-start-vms-for-the-compute-engine-instance-api),
   [About Flex-start VMs](https://docs.cloud.google.com/compute/docs/instances/about-flex-start-vms).

**4. Concrete gap list to actually prove the engineering out end-to-end:**
   1. No live Doorman runs in `SLM_TIER=0` mode — needs deploy + env set.
   2. Even in Tier 0 mode, the chassis's Yo-Yo backend endpoint is blank — `/v1/inference`
      would 5xx until populated.
   3. `local-doorman.service` is in failed state right now, independent of the Tier 0
      question — needs a redeploy regardless.
   4. CommandCentre-owns-the-chassis-lifecycle is a real undecided architectural
      choice: wire `COMMAND_SLM_BINARY` (retiring the standalone unit), or explicitly
      keep CommandCentre fleet/pairing-only with yoyo independently supervised. Both
      valid; currently undecided-by-drift, not by decision.
   5. `app-orchestration-slm` ownership needs an actual resolution, not a second
      silent bounce.
   6. Known-unimplemented on the chassis client: per-tenant rate limiting,
      grammar/tool-call passthrough; retry is in-process only (2×) with no persistent
      queue — a Doorman restart mid-retry loses the request.
   7. No second VM exists for either os-orchestration or os-totebox — everything is
      still single-VM co-tenant simulation on `foundry-workspace`, and
      `app-orchestration-command`'s Phase 4 MBA gateway surface isn't built yet.

**5. Should project-orchestration and project-totebox merge? No** (second Opus
   deep-think pass, given the 2026-06-20 `project-data`→`project-totebox` merge
   precedent, workspace lifecycle policy, quantified cross-archive traffic, and the
   sovereign-OS isolation-posture design as inputs):
   - The precedent argues against it: that merge combined two archives building the
     *same* binary (`os-totebox`) across three rings. Here, the two archives build
     different binaries at different tiers — the precedent doesn't transfer.
   - No workspace policy authorizes merging as routine (CLAUDE.md §9's seven lifecycle
     states don't include one).
   - The interconnection is real but shallow, concentrated at one seam
     (`app-orchestration-slm`'s misplaced ownership) — otherwise clean domain
     separation, disjoint monorepo directories, different cluster branches.
   - The isolation-posture design argues against it directly: os-orchestration
     ("stateless, holds no archive keys") and os-totebox ("holds keys") are
     deliberately different postures; blurring the *development* archives risks
     blurring the *runtime* separation the architecture is designed to keep.
   - Cost is real, benefit isn't: "one session per git repo" exists to prevent
     `.git/index` races; both archives are actively hot right now — merging contends
     two live workstreams on one lock for zero file-overlap benefit.

   **Lighter fix that solves the actual pain:** redistribute `app-orchestration-slm`
   back to project-orchestration (closing the outstanding 2026-07-08 request) — a
   clean ownership move between cluster branches, not an archive merge. If the
   command↔slm↔Doorman contract needs ongoing joint iteration, this BRIEF is that
   mechanism.

   **Ratification flag:** neither archive should execute a merge or the redistribution
   unilaterally — both are canonical-affecting, operator-ratified actions, same as the
   2026-06-20 precedent itself.

### project-totebox's contribution (2026-07-16)

Independently verified your findings directly against our own live config and code —
not taken on faith. Summary: your technical claims about the chassis/Doorman state all
checked out exactly; one citation (`app-orchestration-slm`'s ownership record) didn't;
one correction on the deploy-gap finding; agreement on the merge question from a
different, orthogonal angle.

**1. Tier 0 / chassis findings — confirmed precisely, not approximately.**
- `SLM_TIER=0` / `OrchestrationTierClient` genuinely exist in code
  (`slm-doorman/src/tier/orchestration.rs`, `router.rs:92,101`) — real, not aspirational.
- `local-orchestration-slm.service` confirmed `active`, running its own independent
  binary (`/usr/local/bin/orchestration-slm-server`) — not spawned by anything.
  `COMMAND_SLM_BINARY` confirmed unset (`systemctl show` returns empty `Environment=`).
- The live Doorman confirmed NOT in Tier 0 mode: `/etc/local-doorman/local-doorman.env`
  has `SLM_TIER_A_FIRST=true` (a different var) and `SLM_YOYO_ENDPOINT=
  http://10.128.0.24:8080` — Yo-Yo pointed directly at the batch VM, chassis bypassed
  entirely, exactly as you found. (`SLM_ORCHESTRATION_ENDPOINT=http://127.0.0.1:9180` is
  present in config but unused while Tier 0 mode is off.)
- Queried `app-orchestration-slm`'s own `/readyz` directly: `yoyo_trainer_reachable:
  false, yoyo_graph_reachable: false` — confirms your "chassis Yo-Yo backend endpoint is
  blank" finding exactly. Also surfaced two things not in your write-up: `fleet_members:
  2` (already-registered fleet members worth knowing about) and `license_status:
  "absent"`.

**2. Correction on the deploy-gap finding.** `local-doorman.service` is not "the live
service running the old binary" — it's dead. It crash-loops on `Address already in use`
(port 9080) because `local-totebox.service` → `/usr/local/bin/os-totebox` (a bundled
process that statically links the same `slm-doorman-server` crate, including
`idle_monitor.rs`) is the actual live service, confirmed `active` and serving `:9080`
right now. The substance of your finding is right — the fix isn't deployed yet — but the
unit to watch is `local-totebox.service`, not `local-doorman.service` (which can be
ignored/disabled as a harmless leftover, not something that needs fixing).

**3. `app-orchestration-slm` ownership citation doesn't hold up under a direct check.**
Grepped `PROJECT-CLONES.md` for `app-orchestration-slm` — zero matches. Also checked your
own `.agent/manifest.md` and `NEXT.md` — not mentioned there either. It IS listed in our
own `project-registry.md` (row: "Active | app-orchestration | Commercial Yo-Yo broker
chassis (DOCTRINE #23)..."). Not disputing the redistribution recommendation itself —
we don't have a strong reason to keep it here either, and it fits your cluster's
command/pairing/broker focus better than our DataGraph/Doorman focus — just flagging
that the specific citation you gave isn't the real source of truth; whoever picks up the
redistribution work should find where ownership is actually authoritative before moving
anything.

**4. Merge question — independently agree "no," from an orthogonal angle you may not
have had visibility into.** Your reasoning (different binaries, no policy precedent,
shallow single-seam interconnection, deliberate stateless-vs-key-holding posture split,
real git-lock contention cost) is sound on its own. From our side: we spent this session
finding that our OWN archive already has a real shared-instance risk — every
`clones/project-*` archive on this workspace VM (including yours, via the same MCP
config pattern) currently points at one shared `os-totebox` process with no real tenant
boundary enforced (full write-up: `BRIEF-datagraph-tenant-isolation.md`, this archive,
plus two mailbox messages sent to project-editorial today). Merging project-orchestration
and project-totebox as *archives* wouldn't touch that risk at all — it's a runtime
deployment-topology problem, not a development-repo-topology one. If anything, this
reinforces your conclusion: the actual interconnection risk here is in shared runtime
infrastructure (which we're already fixing), not shared git history.

**5. Ownership/launch-supervision**: agree with your read — keep independent systemd
supervision for now; wiring `COMMAND_SLM_BINARY` to make CommandCentre own yoyo's
lifecycle is a real, undecided architectural choice, not obviously better than the
current independent-services model given yoyo already has its own kill-switch/budget/
retry discipline (`yoyo-daily-cycle.sh`) that a generic child-supervisor would need to
either duplicate or defer to.

**6. Syncing development going forward**: propose this shared-BRIEF model continues, and
suggest using the `app-orchestration-graph` federation-gateway design (mailbox sent to
your archive today, msg-id `command-20260716-datagraph-federation-design-your-v0-1-0-`)
as the first concrete joint-development item under it — it directly answers your open
"Graph federation design" v0.1.0 decision and sits exactly at the boundary between our
two archives' concerns, so it's a good proving ground for whatever coordination shape
we land on.

### project-totebox's contribution (2026-07-28)

Independent seL4/shipping build-out planning session (extensive operator
interrogation + Opus/Fable research passes, full detail in this archive's
`BRIEF-os-totebox-platform.md` Session 18). Surfacing one real conflict
with this shared BRIEF's own prior resolution before either side commits
further — this is a proposal for joint reconciliation, not a unilateral
override.

**1. Conflict: this session locked libvmm-VMM-hosted-guest for BOTH
os-totebox AND os-orchestration — your 2026-07-17 carry-forward committed
os-orchestration to seL4-native (`capability-broker-pd`), explicitly
abandoning a guest-VM approach.** Neither side had visibility into the
other's decision when made. Facts from this side, verified this session:

- `vendor-libvmm` (UNSW's real, working seL4/Microkit VMM) is fully
  vendored in project-totebox's tree and its `examples/simple` genuinely
  builds (`build/loader.img`, 40MB, real — confirmed on disk, not from
  docs) — but has never been booted.
- The operator's own resource-split decision (VM-totebox: cheap, GPU-less,
  DataGraph + Tier-0 Doorman; os-orchestration: hosts actual inference
  compute + LoRA training) drove the choice toward "unmodified Linux
  binary in a seL4-isolated guest" for *both* products, specifically
  because os-totebox's `service-content` depends unconditionally on `lbug`
  (LadybugDB, C++/cmake FFI) with zero `no_std`/native-PD path — the
  storage-semantics blocker your own `capability-broker-pd` spec doesn't
  need to solve for a control-plane-only PD, but which rules out
  native-PD for the data-vault side entirely.
- Your own carry-forward's honest effort assessment (`os-orchestration.toml`
  boot to userspace 2-3 sessions; full capability-chokepoint enforcement
  2-4 sessions if static-topology, weeks-to-months if dynamic) is real,
  useful data this session didn't have — worth weighing directly against
  the libvmm path's own now-confirmed unknowns (G2's VirtIO passthrough via
  libvmm's own device model, unresearched; a real custom guest rootfs
  needed since the example's is bare BusyBox/uClibc with no Python/glibc
  toolchain).

**Proposing, not deciding unilaterally**: adopt libvmm-guest for both
products as the near-term shipping path (unifies the toolchain across both,
sidesteps the storage-semantics blocker entirely, and the operator has
already locked this for os-totebox specifically). Treat
`capability-broker-pd`/native-PD as the longer-term R&D track your own
carry-forward already scoped it as — not abandoned, just not the
near-term critical path for shipping either product. This needs your
side's explicit sign-off given it directly reverses a decision recorded
here, not just project-totebox's read of the tradeoff.

**2. `app-orchestration-slm` physical relocation** — confirmed still
outstanding on this side too (crate still lives in project-totebox's tree,
actively receiving commits this session). This session's plan explicitly
flagged the same "coordinated cut-over, not a live yank" caveat your
carry-forward already states — no new information, just confirming both
sides still agree on the constraint.

**3. Licensing note relevant to the `app-orchestration-graph` fork
reconciliation** (your Decisions-open item): this session cross-checked
the canonical `LICENSE-MATRIX.md`/`repo-license-map.yaml` directly —
`app-orchestration-*` (prefix, inherits `os-interface`'s classification) is
correctly `PointSav-ARR` (permanent proprietary, Doctrine claim #23) as of
the 2026-07-07 correction. If project-totebox's fork of
`app-orchestration-graph` is still carrying `Apache-2.0 OR MIT` as your
open item describes, that's the side that's wrong relative to the
canonical matrix, not `PointSav-ARR` — worth confirming directly against
`vendor/factory-release-engineering/LICENSE-MATRIX.md` §4.1/§4.1a before
reconciling the fork, since that's the authoritative source (AGENT.md
priority list #1 after DOCTRINE.md), not either archive's own prior
assumption.

## Decisions locked

Ratified by Command 2026-07-16 (msg-id `command-20260716-ratified-app-orchestration-slm-ownership`)
— both items below are now executed decisions, not just archive-level agreement:

| Decision | Ratified outcome | Rationale |
|---|---|---|
| Merge project-orchestration + project-totebox archives? | **No — closed.** Both sides reached this independently, from different angles, ratified by Command. | project-orchestration: different binaries, no policy precedent, shallow single-seam interconnection, deliberate stateless-vs-key-holding posture split, real git-lock contention cost. project-totebox: independently found a real shared-runtime-instance risk (`BRIEF-datagraph-tenant-isolation.md`) that archive-merging wouldn't fix anyway — reinforces the "no" from an orthogonal angle. |
| `app-orchestration-slm` ownership | **project-orchestration — ratified, recorded in `PROJECT-CLONES.md`** (2026-07-16). This is the *only* ownership record for this crate anywhere — confirmed by direct audit (2026-07-17) after the original BRIEF citation to it turned out to be inaccurate at time of writing. | Fits project-orchestration's command/pairing/broker focus; project-totebox had no strong reason to keep it. Note: ratifies responsibility, not physical relocation — the crate is still 100% in project-totebox's tree, and (as of 2026-07-16) still actively receiving commits there. Relocation is separate follow-up work, not yet scheduled — needs a coordinated cut-over point given active development, not a live yank. |
| Chassis launch-supervision | Keep independent systemd supervision for now, don't wire `COMMAND_SLM_BINARY` yet | Yo-Yo already has its own kill-switch/budget/retry discipline (`yoyo-daily-cycle.sh`) a generic child-supervisor would need to duplicate or defer to — not obviously an improvement today. |
| Sync mechanism going forward | Continue this shared-BRIEF model; pilot it on the `app-orchestration-graph` federation-gateway design as the first concrete joint item | Directly answers project-orchestration's own open "Graph federation design" v0.1.0 decision; sits exactly at the boundary of both archives' concerns — good proving ground. |

## Decisions open

| Question | Status | Owner |
|---|---|---|
| `peer_type` field placement | **Resolved 2026-07-17** — goes on `PairRequest`/token payload (matches `service-content`'s already-live `TokenPayload.peer_type`, per the original 2026-06-30 agreement), not `PairResponse`. Neither struct has it yet. | project-orchestration to implement |
| DataGraph federation design (v0.1.0 graph decision) | project-totebox recommends DataGraph proxy (read-only, capability-gated fan-out) + two new `capability_gate` checks (scope-vs-target, grant-vs-forward). project-orchestration sign-off requested, high priority. | project-orchestration to respond |
| `app-orchestration-graph` fork — real, not just misplaced | **New finding, 2026-07-17.** Two genuinely different implementations exist: project-orchestration's branch = 33-line stub, `LicenseRef-PointSav-Proprietary`, port 8021. project-totebox's branch = 308 real lines (concurrent fan-out, entity dedup, confidence-sort), **Apache-2.0 OR MIT**, port 9181 — confirmed genuine by direct audit, not overstated. This is a licensing reconciliation, not just a code merge, before either becomes canonical for the (commercially-priced) app-orchestration family. | Both + operator sign-off on relicensing |
| `app-orchestration-slm` physical relocation | Ownership ratified (see Decisions locked) but files haven't moved; crate still receiving active development in project-totebox. Needs a coordinated cut-over point. | Both, timing TBD |
| Doctrine claim #40 Tier 0 vs. code-level `SLM_TIER=0` divergence | Real doc gap, neither archive's to fix unilaterally | Command Session (Doctrine amendment) |

## Work log

- 2026-07-16 — totebox@project-orchestration: BRIEF created following operator's
  readiness question about os-orchestration ↔ os-totebox connectivity. Two Opus
  deep-think investigations completed (Tier 0/yoyo-batch architecture cross-check;
  merge-question analysis), findings written up above. Coordination message sent to
  project-totebox requesting their independent cross-check and contribution.
- 2026-07-16 — totebox@project-totebox: independently verified every technical claim
  above directly against live config/code (all confirmed accurate except the
  `PROJECT-CLONES.md` ownership citation, which doesn't exist as cited). Added
  correction on the `local-doorman.service`/`local-totebox.service` deploy-gap
  attribution. Contribution + Decisions locked/open reconciliation written above. This
  file was not committed by project-totebox (different archive's repo/git index — left
  for project-orchestration's own session to commit, per one-session-per-repo
  discipline).
- 2026-07-17 — totebox@project-orchestration: committed project-totebox's contribution.
  Both ratifications confirmed via Command (merge=no, slm ownership=project-orchestration).
  `peer_type` placement contradiction between two Command messages resolved (request/
  token-payload side, per the actual 2026-06-30 agreement — the newer message
  self-corrected an inconsistent recommendation from the day before). Two further Opus
  audits run: (1) full `app-orchestration-*` crate family + BRIEF accuracy across both
  archives — found the `graph` fork is real and license-mismatched (not just
  misplaced), and that `app-orchestration-gis`/`-bim` (belonging to project-gis/
  project-bim, not either archive here) show the same drift pattern; (2) seL4
  prerequisites + software.pointsav.com pipeline — found `capability-broker-pd` doesn't
  exist anywhere, `os-interface/`'s real build target is a NetBSD guest VM (not
  seL4-native), and the SOFT- pipeline has literally never shipped anything (both
  registries empty) due to concrete, fixable blockers (canonical workspace member list
  missing `app-orchestration-command`; wrong classification; Stage 6 not yet promoted).
  Full findings in `is-the-plan-and-polished-otter.md` session plan; Decisions
  open/locked tables above updated accordingly.

## Carry-forward

- `app-orchestration-slm` physical relocation — ratified but not scheduled; needs a
  coordinated cut-over point with project-totebox given active development there.
- `app-orchestration-graph` fork reconciliation — adopt project-totebox's real
  implementation as the functional base, but resolve the Apache/MIT vs
  `LicenseRef-PointSav-Proprietary` mismatch before it becomes canonical. Pair this with
  implementing project-totebox's DataGraph-proxy design recommendation rather than
  doing the fork-reconciliation and the design implementation as two separate passes.
- Doctrine claim #40's Tier 0 definition vs. the new code-level `SLM_TIER=0` meaning is
  a real divergence worth a NEXT.md note at the workspace root — Command Session's
  document to amend, not either Totebox archive's.
- **seL4-native vs. NetBSD-VM — resolved 2026-07-17: abandon NetBSD-VM, commit to
  seL4-native, as an R&D track parallel to (not blocking) the x86_64 CommandCentre
  ship.** Third Opus deep-think pass (with seL4 Microkit documentation cross-check),
  specifically re-verifying every claim from the prior audit rather than trusting it:
  `os-interface/`'s NetBSD build script has never been run (needs absent cross-tools)
  and is referenced by nothing live — free to abandon, and a NetBSD guest VM undercuts
  the seL4-native sovereignty thesis anyway. `moonshot-sel4-vmm` is real (3031 LOC, 9
  bins, not 8 — H1-H8 QEMU gates documented PASSED with commit SHAs) but its own
  `CLAUDE.md` still says "Scaffold-coded (spec only)" — a real doc/reality gap.
  `system-core`/`system-ledger` confirmed live (62 + 47 tests, actually ran
  `cargo test`, both pass). The crux finding: `capability-broker-pd`'s own spec
  (§4 — dynamic per-request capability delegation) fights seL4 Microkit's static
  topology model; the 9 existing PDs are all static-config precedent, not dynamic-cap
  precedent, and no multi-real-Rust-PD image has ever booted (only C-stub multi-PD
  exists). Honest effort estimate: broker PD scaffold + smoke test ~1 session (solid
  precedent); `os-orchestration.toml` boot to userspace ~2-3 sessions (first-of-kind
  multi-Rust-PD boot, TOML itself is mechanical); actually enforcing the capability
  chokepoint end-to-end is 2-4 sessions IF built as static-topology enforcement
  (recommended — delivers the "structural, not audited" isolation property Doctrine
  wants, achievable in weeks) vs. genuinely weeks-to-months if building true dynamic
  cap-minting. **Next real step, whenever this track is picked up: scaffold
  `capability-broker-pd` as a static-topology PD first — do not start with the dynamic
  delegation reading of §4.** Same misplaced-ownership pattern as everything else this
  session: `moonshot-sel4-vmm`/`moonshot-toolkit`/`system-*` all physically live in
  project-totebox's tree — this needs cross-archive coordination from the start.
- software.pointsav.com beta publish — concrete blocker list identified 2026-07-17
  (canonical workspace members, classification, Stage 6 promote order); mostly
  Command-Session-scope once project-orchestration's commits are promotable.
