---
artifact: brief
schema: foundry-brief-v1
brief-id: project-orchestration-totebox-integration
title: "os-orchestration ↔ os-totebox integration — shared cross-archive BRIEF"
status: active
owner: project-orchestration
created: 2026-07-16
updated: 2026-09-02
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

### project-totebox's contribution (2026-08-02)

Following the shared BRIEF's own two-archive contribution model — this is the
proper independently-authored "contribution" write-up for Sessions 18-24
(2026-07-28 through 2026-08-02), sent for project-orchestration's own session
to paste in (per one-session-per-repo discipline, project-totebox does not
edit/commit this archive's file directly).

**1. Update since the 2026-07-28 entry: the libvmm-guest-vs-native-PD
question is resolved, not just proposed.** An independent Fable research pass
confirmed the split proposed on 2026-07-28 is architecturally correct: native
PDs (`capability-broker-pd`) are right for small, security-critical, narrowly
scoped components; a real HTTP service with business logic belongs in a Linux
guest. Both tracks (os-totebox, app-orchestration-slm) reached G1-G4 + SIGTERM
verification on 2026-07-29/30 — real guest-Linux-under-vendor-libvmm, both
already G4-verified against their bare-host equivalents.

**2. Two new dedicated VMs exist and are in active use**: `os-totebox-1` and
`os-orchestration-1` (GCP, us-west1-a), each currently running its product's
real seL4/libvmm-hosted guest image via a hand-launched `qemu-system-aarch64`
invocation (interim verification, not final packaging — not yet wrapped in
systemd). A dedicated WireGuard mesh (`wg1`, `10.42.0.0/24`) connects them
plus `foundry-workspace`, separate from the existing admin `wg0` tunnel.
**Confirmed still live and in active use as of 2026-09-02** —
project-orchestration used this same mesh directly this session (via
`local-slm-wg1-forward.service`, `10.42.0.1:8080`) to resolve a live
os-orchestration-1 Yo-Yo routing request from Command.

**3. Real perpetual per-instance licensing implemented and live-verified** in
`license.rs` (`expiry: Option`, `None` = perpetual; `fleet_max` entitlement;
`update_channel_until` separate from runtime right) — minted and validated a
real Ed25519 dev license end-to-end against the deployed chassis, not just
unit tests.

**4. Yo-Yo node architecture resolved** (Fable+Opus convergence): a Yo-Yo is
a bare OpenAI-compatible inference endpoint, never runs os-totebox/
os-orchestration software, and must be mesh-only — found a real, still-open
trust gap in the existing chassis→Yo-Yo hop (`danger_accept_invalid_certs(true)`,
a single shared static bearer) worth knowing about for any fleet-facing work
on this side.

**5. Honest open item, not resolved this session**: the final real 200-OK
chat-completions round-trip through the full chain (os-totebox-1 → mesh →
chassis → mesh → foundry-workspace's real inference) has not yet been
observed — every layer up through the chassis's own routing/licensing
decision is independently proven correct, but the terminal hop is blocked by
what looks like two compounding candidates (foundry-workspace's
`local-slm.service` under genuine continuous production load, and possibly
QEMU SLIRP usermode-networking fragility on long-lived connections through
the hand-launched guest). Full write-up: `.agent/briefs/BRIEF-os-totebox-platform.md`,
Sessions 23-24, project-totebox's archive.

No action needed from this side unless project-orchestration wants the
WireGuard mesh extended to its own nodes, or wants to weigh in on the still-open
`app-orchestration-graph` fork/license reconciliation from the Decisions-open
table.

## Decisions locked

Ratified by Command 2026-07-16 (msg-id `command-20260716-ratified-app-orchestration-slm-ownership`)
— both items below are now executed decisions, not just archive-level agreement:

| Decision | Ratified outcome | Rationale |
|---|---|---|
| Merge project-orchestration + project-totebox archives? | **No — closed.** Both sides reached this independently, from different angles, ratified by Command. | project-orchestration: different binaries, no policy precedent, shallow single-seam interconnection, deliberate stateless-vs-key-holding posture split, real git-lock contention cost. project-totebox: independently found a real shared-runtime-instance risk (`BRIEF-datagraph-tenant-isolation.md`) that archive-merging wouldn't fix anyway — reinforces the "no" from an orthogonal angle. |
| `app-orchestration-slm` ownership | **project-orchestration — ratified, recorded in `PROJECT-CLONES.md`** (2026-07-16). This is the *only* ownership record for this crate anywhere — confirmed by direct audit (2026-07-17) after the original BRIEF citation to it turned out to be inaccurate at time of writing. | Fits project-orchestration's command/pairing/broker focus; project-totebox had no strong reason to keep it. Note: ratifies responsibility, not physical relocation — the crate is still 100% in project-totebox's tree, and (as of 2026-07-16) still actively receiving commits there. Relocation is separate follow-up work, not yet scheduled — needs a coordinated cut-over point given active development, not a live yank. |
| Chassis launch-supervision | Keep independent systemd supervision for now, don't wire `COMMAND_SLM_BINARY` yet | Yo-Yo already has its own kill-switch/budget/retry discipline (`yoyo-daily-cycle.sh`) a generic child-supervisor would need to duplicate or defer to — not obviously an improvement today. |
| Sync mechanism going forward | Continue this shared-BRIEF model; pilot it on the `app-orchestration-graph` federation-gateway design as the first concrete joint item | Directly answers project-orchestration's own open "Graph federation design" v0.1.0 decision; sits exactly at the boundary of both archives' concerns — good proving ground. |
| DataGraph federation design (v0.1.0) | **Signed off 2026-09-04, conditional.** Two independent reviews (Opus + Fable) confirm Command's 2026-07-16 recommendation (read-only DataGraph proxy, capability-gated fan-out over each target's `/v1/graph/context`) is **already built** — `app-orchestration-graph/src/main.rs` holds zero graph state, fans out per-target, dedups, sorts; both "missing" `capability_gate` checks (scope-vs-target, grant-vs-forward) already exist in `service-content/src/http.rs` (`scope_permits_request`, `verify_capability_grant`'s grant ceiling). Design approved as-is. **But: NOT cleared for activation** — both reviews independently found the same real, confirmed vulnerability: `service-content`'s `pair_peer` verifies an invite token against the *public key supplied in the same request body*, never a trusted issuer; `/v1/pair`/`/v1/pair/token` are ungated in the router; the gateway's own `capability.rs::build_pair_request` self-signs an ADMIN-role pairing today — this is how it pairs *now*, not a hypothetical. Any caller reaching the endpoint can self-grant and pass every downstream check, since the "ceiling" only validates against a record the caller itself wrote. Confirmed **not currently deployed anywhere** (`local-orchestration-graph.service` inactive, no live process) — a real hole to close before first activation, not an active incident. Required before go-live: (1) `pair_peer` must verify against Totebox's own `pairing_key` (or Command's), not the caller's key — `project-totebox`, blocking; (2) `build_pair_request` stops self-issuing (obtains a real invite token out-of-band) and the gateway downgrades to least-privilege `INTERFACE` role, not self-claimed `ADMIN` — `project-orchestration`; (3) the gateway authenticates its own inbound callers before it's the fleet's single read path — `project-orchestration`; (4) `service-content` needs a `REQUIRE_CAPABILITY`-style flag so headerless-passthrough isn't permanent — `project-totebox`. Full reviews on file if needed for either side's remediation work. | Joint remediation, (1) blocking on project-totebox |
| `peer_type` field placement | **Implemented 2026-09-04** — added to `InviteTokenPayload` and `PairRequest` in `orchestration-command-core` (not `PairResponse`), `#[serde(default)]`, stamped at issuance via a new `peer_type` param on `InviteIssuer::issue()`. Commit `6a6f6f7` in the `pointsav-orchestration-private` nested sub-clone (`cluster/project-orchestration`, committed not yet pushed — no staging forks exist for the private repo yet). `cargo build`/`cargo test` clean (14/14 passing, including a new assertion on the field). | Matches `service-content`'s already-live `TokenPayload.peer_type` pattern, per the original 2026-06-30 agreement and the 2026-07-17 placement resolution. |

## Decisions open

| Question | Status | Owner |
|---|---|---|
| `app-orchestration-graph` fork | **Resolved, not previously noticed — checked directly 2026-09-04.** The two copies (project-orchestration's `pointsav-orchestration-private` nested clone vs. project-totebox's residual copy) are now byte-identical: 496 lines, `license = "LicenseRef-PointSav-ARR"` in both `Cargo.toml`s. Reconciled at some point via general canonical-repair activity (commit `11a9715` "reconcile(project-totebox): land 14 preserved commits + vendor-libvmm onto repaired canonical" in the private-repo history) rather than a dedicated fork-reconciliation pass — closing this item, no further action needed. | Closed 2026-09-04 |
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
- 2026-09-02 — totebox@project-orchestration: pasted project-totebox's
  2026-08-02 contribution (Sessions 18-24) per their mailbox message
  `command-20260802-project-totebox-s-contribution-to-brief-`, marked
  actioned. Same session independently confirmed the `wg1` mesh described in
  that contribution is live and usable — used it directly to resolve a
  Command Session request to route os-orchestration-1's Yo-Yo traffic via
  `local-slm-wg1-forward.service` (`10.42.0.1:8080`) rather than any new
  exposure.
- 2026-09-02 (same session, later) — totebox@project-orchestration: routing
  decision executed. Approved routing `os-orchestration-1`'s Yo-Yo default
  endpoint to the wg1 mesh forwarder, accepting shared/queued capacity with
  `local-totebox.service`'s own Tier A traffic rather than increasing
  `--parallel`. Actual guest-config write was Command Session's to run
  (VM-sysadmin scope per this archive's own `scope-discipline.md`) — routed
  the request, then flagged before execution that project-totebox had no
  active session to coordinate with and that its `write-guest-config.sh`/
  `apply-guest-config.sh` were uncommitted, mixed into ~10 other in-flight
  files. Command confirmed the env var and disk layout independently, used a
  standalone adapted copy of the script (did not touch project-totebox's
  working tree), and executed cleanly: guest stopped, config written,
  restarted healthy in 6s, verified live (`/readyz` clean,
  `POST /v1/yoyo/proxy` 401 instead of 503). Carry-forward's end-to-end
  round-trip item updated below to reflect routing is done but inference
  completion is still unverified.
- 2026-09-04 (same session, later) — totebox@project-orchestration: restored
  source access after the 2026-09-01 relocation (see `.agent/manifest.md`) and
  implemented `peer_type` (Decisions locked). Two independent deep-think
  passes (Opus + Fable) before touching git topology, since the obvious
  approaches risked re-leaking this content back to the still-unpurged
  jwoodfine/pwoodfine forks via `self-service-promote.sh`'s missing filter
  (flagged to Command separately, high priority). Diffed all 4 owned paths
  against project-totebox's residual copy before trusting the private repo —
  found real divergence in `app-orchestration-slm`/`os-orchestration` (Carry-
  forward updated below), confirmed `app-orchestration-command` identical.
  `peer_type` implemented there since it was safe to (identical file,
  ratified decision, `cargo build`/`test` both clean). Committed locally in
  the new nested sub-clone, not pushed — no Stage 6 path exists yet for
  `pointsav-orchestration-private` (flagged to Command).
- 2026-09-04 (same session, later) — totebox@project-orchestration: closed
  two more Decisions-open items. `app-orchestration-graph` fork confirmed
  already resolved (checked directly — byte-identical, correct license), not
  previously noticed. DataGraph federation design (open since 2026-07-16):
  ran two independent deep-think reviews (Opus + Fable) rather than
  rubber-stamping a 2-month-old request — both confirmed the design is
  already built and both "missing" checks already exist, but both
  independently found the same real, confirmed pairing-bypass vulnerability
  in `service-content`'s `pair_peer` (self-signed tokens, ungated `/v1/pair`)
  that the original design request didn't account for. Signed off on the
  design conditionally; flagged the vulnerability to Command + directly to
  project-totebox given they own the affected code. Confirmed not currently
  deployed anywhere (no live process) — real gap, not an active incident.

## Carry-forward

- **`app-orchestration-slm` + `os-orchestration` + `app-orchestration-command`
  physical relocation/reconciliation — concrete 3-track proposal drafted
  2026-09-04, not yet executed, needs project-totebox + operator sign-off
  first.** The 2026-09-01 security purge extracted these into
  `pointsav-orchestration-private` (new private repo, `project-orchestration`
  restored access via a nested sub-clone this session — see
  `.agent/manifest.md`). A dedicated investigation (this session) diffed every
  divergent file's actual substance against `BRIEF-os-totebox-platform.md`'s own
  record, not just noting that files differ:
  - **`os-interface`: identical, no-op.**
  - **`app-orchestration-slm`: project-totebox's copy is ahead** — every diff
    matches a real, tested, already-documented fix from their Session 24
    Fable bug-hunt audit (`fleet.rs` TTL eviction so dead members don't hold
    licensed slots; `license.rs`'s real Perpetual Fleet License model —
    `expiry: Option`, `fleet_max`, `update_channel_until`; `membership.rs`
    persisting the Ed25519 signing seed to disk so restarts don't invalidate
    issued tokens; `yoyo_proxy.rs`/`http.rs` real status-propagation,
    streaming, auth, and timing-attack fixes) — plus entire missing files
    (`scripts/`, `systemd/`, `Cargo.lock`, `examples/mint_dev_license.rs`,
    `CLAUDE.md`). Proposal: cherry-pick project-totebox's actual commits onto
    the private repo's branch (preserves authorship as the record of record,
    not a raw file copy).
  - **`os-orchestration`: the diff is a relicense, not a bug fix** — SPDX
    header changed from `LicenseRef-PointSav-ARR` to `FSL-1.1-ALv2`, isolated
    to this crate + `build-microkit-image.sh`. Must NOT ride along silently
    with the `app-orchestration-slm` cherry-pick — needs explicit operator
    sign-off on whether this relicense was intentional and should propagate.
  - **`app-orchestration-command`: no longer identical** (diverged in both
    directions since this session's own `peer_type` commit — private repo has
    `peer_type`, project-totebox has the seL4 appliance `scripts/`/`systemd/`
    packaging). Needs a real three-way merge (cherry-pick project-totebox's
    appliance commit onto the private repo's current HEAD, resolving against
    the already-landed `peer_type` field), not a copy.
  - **Before copying anything**: needs project-totebox to confirm the
    `os-orchestration` relicense was intentional, confirm no uncommitted WIP
    beyond what's checked in (their tree is still actively worked), and
    provide or reconstruct `BRIEF-os-orchestration-command-appliance.md` —
    the mailbox message describing the appliance work cites this file but it
    does not exist in their briefs directory (the underlying artifact/commit
    is real and verified, just the cited writeup is missing).
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
  project-totebox's tree (superseded 2026-09-05, see new entry below) — this needs cross-archive coordination from the start.
- **`os-orchestration` build path — decided 2026-09-05: adopt libvmm-guest
  (Microkit + vendor-libvmm), matching os-totebox and the consolidated
  app-orchestration-command+slm appliance. Answers the 2026-07-28 conflict
  message (`command-20260728-sel4-architecture-conflict-new-contribut`).**
  Two independent deep-think reviews (Opus + Fable), both confirming the
  ground moved decisively since 2026-07-17: project-totebox's own
  `BRIEF-os-totebox-platform.md` already locked Pattern A (libvmm-guest) as
  the near-term path with a written pivot trigger back to native PDs;
  `capability-broker-pd` is "at least 6 months out with no further work
  started." More decisively — `project-sel4` (a separate, newer archive) has
  already retired the single hardest technical risk the 2026-07-17 estimate
  was built on ("no multi-real-Rust-PD image has ever booted"): its Phase 1.5
  booted a real multi-Rust-PD Microkit image with real cross-PD capability
  delegation into a durable WORM log (Gates N17, N51-N54) — exactly the
  static-topology chokepoint pattern this track wanted, already built and
  regression-tested, by a different team. `project-sel4`'s own roadmap
  explicitly lists "Phase 6 — apply the same framework to os-orchestration,
  then os-console," currently marked "do not start yet." Building
  `capability-broker-pd` here would re-derive that work from a weaker
  position. Separately (Fable): the operator's own resource-split gives
  `os-orchestration` GPU/CUDA/`llama-server` inference workloads — further
  from `no_std`-feasible than `os-totebox`'s `lbug` dependency ever was, so
  native-PD-first was arguably backwards for this specific product anyway.
  **Decision: `os-orchestration` becomes the appliance-image crate for the
  already-live `os-orchestration-1` VM.** First concrete build step: mirror
  project-totebox's `os-totebox/scripts/{build-guest-rootfs.sh,
  build-microkit-image.sh,deploy-loader-img.sh,write-guest-config.sh}` +
  `systemd/` unit as the template (same gate `app-orchestration-command`
  passed 2026-08-06), turn `os-orchestration/src/lib.rs`'s scaffold stub into
  a real `std` binary with a `/readyz` endpoint, produce one `loader.img`
  that QEMU-boots. Use a dedicated `build-os-orchestration/` directory from
  the start — `virtio.mk`'s shared-`build/`-directory fragility is a known,
  already-documented footgun across all three products. Not yet started —
  this closes the decision, not the implementation.
- software.pointsav.com beta publish — concrete blocker list identified 2026-07-17
  (canonical workspace members, classification, Stage 6 promote order); mostly
  Command-Session-scope once project-orchestration's commits are promotable.
- Chassis→Yo-Yo hop trust gap (project-totebox's 2026-08-02 finding):
  `danger_accept_invalid_certs(true)` + a single shared static bearer token —
  real, still open, worth fixing before any fleet-facing Yo-Yo traffic beyond
  occasional testing.
- **Final end-to-end 200-OK chat-completions round-trip — narrowed 2026-09-02,
  still not fully observed.** `os-orchestration-1`'s leg of the path is now live:
  `ORCHESTRATION_YOYO_DEFAULT_ENDPOINT` set to `http://10.42.0.1:8080` (the wg1
  mesh forwarder, `local-slm-wg1-forward.service`) directly on the guest's
  `/data/foundry-config.env`, chassis restarted and verified healthy
  (`/readyz`: `degraded:false`, proxy circuit `closed`,
  `yoyo_trainer_reachable`/`yoyo_graph_reachable` both `true`); `POST
  /v1/yoyo/proxy` now returns `401` (auth-gated, reachable) instead of the
  prior `503` (unconfigured). That confirms the chassis→foundry-workspace hop
  is reachable, not that a real inference call completes end-to-end — the
  auth-gated `401` and the two compounding candidate causes flagged 2026-08-02
  (local-slm.service production load; possible QEMU SLIRP long-connection
  fragility) are both still unverified/unresolved. Remaining gap: an actual
  authenticated `POST /v1/yoyo/proxy` (or equivalent) call through the full
  chain `os-totebox-1 → mesh → chassis → mesh → foundry-workspace`, and
  resolving the chassis→Yo-Yo trust gap above before this is more than
  occasional-testing traffic. Owner: joint — routing/chassis-side now
  project-orchestration's (this change), inference-completion verification
  still project-totebox's per `BRIEF-os-totebox-platform.md` Sessions 23-24.
