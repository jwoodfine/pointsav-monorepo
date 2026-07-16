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

### project-totebox's contribution

*(placeholder — project-totebox: please run your own independent cross-check against
the findings above, from your own code/commits/live-service state, not by taking our
read on faith. Correct anything we got wrong about your side. Add your own perspective
on the ownership/launch-supervision question, the merge question, and how you'd want to
sync development going forward. This section is yours — we won't edit it.)*

## Decisions locked

*(none yet — pending project-totebox's contribution and reconciliation)*

## Decisions open

| Question | Status | Owner |
|---|---|---|
| `app-orchestration-slm` ownership — redistribute to project-orchestration? | Open, 2026-07-08 request outstanding | Both + Command ratification |
| Does `app-orchestration-command`'s child-supervisor end up owning yoyo's lifecycle, or does project-totebox keep independent systemd supervision? | Open | Both |
| Merge project-orchestration + project-totebox archives? | Open — project-orchestration's read is no; awaiting project-totebox's independent read | Both + operator ratification |
| How should the two archives sync development going forward? | Open | Both |

## Work log

- 2026-07-16 — totebox@project-orchestration: BRIEF created following operator's
  readiness question about os-orchestration ↔ os-totebox connectivity. Two Opus
  deep-think investigations completed (Tier 0/yoyo-batch architecture cross-check;
  merge-question analysis), findings written up above. Coordination message sent to
  project-totebox requesting their independent cross-check and contribution.

## Carry-forward

- Awaiting project-totebox's contribution to this BRIEF.
- Once both contributions exist: reconcile into "Decisions locked," route the
  ownership/merge questions to the operator for ratification, and (if redistribution is
  ratified) plan the actual `app-orchestration-slm` cluster-branch move as a separate,
  properly-scoped piece of work.
- Doctrine claim #40's Tier 0 definition vs. the new code-level `SLM_TIER=0` meaning is
  a real divergence worth a NEXT.md note at the workspace root — Command Session's
  document to amend, not either Totebox archive's.
