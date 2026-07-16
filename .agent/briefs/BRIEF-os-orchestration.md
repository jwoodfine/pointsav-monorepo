---
artifact: brief
schema: foundry-brief-v1
brief-id: project-orchestration-os-orchestration
title: "os-orchestration build-out — app-orchestration-command (v0.0.1/v0.0.2 in BETA)"
status: active
owner: project-orchestration
created: 2026-06-29
updated: 2026-07-09
---

> **Scope note (2026-07-09):** this BRIEF is the concrete shipping BRIEF for
> `app-orchestration-command` — implementation, deployment, and licensing decisions
> for the binary actually running today. For the long-range os-orchestration
> architecture (seL4 PDs, capability-broker, five-app activation roadmap, journal
> tie-ins) see `BRIEF-os-orchestration-build-out.md`.

## Context

os-orchestration is the commercial tier of the Totebox OS family — the only paid OS
product. os-console and os-totebox are free. The commercial line is drawn at the aggregator
(cross-archive federation = paid; single-archive WORM vault = free).

**Deployment model — project-scoped:** An operator spins up one os-orchestration instance,
pairs it with the Totebox Archives relevant to a project, and then invites team members
(architect, contractor, reviewer) to pair their os-console to it. When the project ends,
the os-orchestration instance is decommissioned. The instance IS the group — no group
management layer is needed because the deployment itself defines the access scope.

**Permission model — runtime surface of pairings.yaml:** os-orchestration reads structural
topology (pairings.yaml + archive manifests) and exposes it as a queryable API. It does
not own permissions; it surfaces them. "Pairing is the permission. Topology is the audit."
Consistent with DOCTRINE Claim #43 (single-boundary compute) and Claim #52 (moduleId
isolation). No central Permission Database.

**License:** Ed25519-signed token (same pattern as app-orchestration-slm); channel_expiry
gates updates only — never kills a running binary (Doctrine §54/§48/§28). Distribution:
software.pointsav.com, $1 or $19 USDC one-time, Polygon USDC payment rail.

**Why:** Phase 3 of the Totebox Orchestration transition (NEXT.md P3.3). This archive owns
the `app-orchestration-command` crate. v0.0.1 establishes the binary and ships it to the
marketplace. [[BRIEF-OS-FAMILY]] [[BRIEF-sovereign-os-family-master-plan]]

---

## Scope

- Implement `app-orchestration-command` v0.0.1 as a workspace crate in
  `pointsav-monorepo/app-orchestration-command/` on branch `cluster/project-orchestration`
- Workspace: 3 crates — `orchestration-command-core` (wire types), `orchestration-command`
  (library), `orchestration-command-server` (binary at port 8020 loopback)
- Invite token pairing protocol — P1 generates one-time Ed25519-signed token; team member
  enters it in os-console; os-console POSTs to `/v1/pair`; server verifies + creates pairing
- Process supervisor: server spawns and health-monitors `app-orchestration-slm` as a child
- `app-orchestration-graph` stub: single crate, healthz only, placeholder for v0.1.0
- Cross-archive coordination: outbox messages to project-console, project-totebox,
  project-infrastructure with the invite token wire spec so all three build compatible protocol
- Update topic-os-orchestration.draft.md with project-scoped model + invite token UX
- Stage 6 lite via self-service-promote.sh after first clean build

**OUT OF SCOPE for v0.0.1:**
- Full PSP capability-based binary protocol (planned v0.2.0)
- Phase 4 VPN bind (10.42.0.9:8021 — awaits WireGuard Part A from Command)
- Multi-tenancy (serving multiple operators from one instance)
- app-orchestration-graph federation logic
- software.pointsav.com listing — outbox sent to project-software 2026-06-29 for BETA upload (no payment gate)

---

## Decisions locked

| Decision | Value | Rationale |
|---|---|---|
| Crate pattern | Workspace (server + lib + core) | Mirrors app-orchestration-slm; separation of wire types from business logic |
| Tokio flavor | `current_thread` | BRIEF-OS-FAMILY mandate; 4–8 MB RSS vs 30–40 MB for multi-thread |
| Release profile | `opt-level="z" lto=true codegen-units=1 panic="abort" strip=true` | Phase 2 idle target: ≤96 MB disk/RAM |
| Port | 127.0.0.1:8020 (loopback) | Phase 3 constraint; VPN bind added at Phase 4 |
| Permission model | pairings.yaml-as-topology; no runtime policy DB | DOCTRINE + PairingAsPermission TOPIC |
| Pairing UX | Invite token (Ed25519 signed, 24h TTL, single-use nonce) | Low friction for team onboarding without breaking structural model |
| License gate | Ed25519 embedded public key; no network call; air-gap safe | Matches slm pattern; Doctrine §54 no-kill constraint |
| Pricing | $1 or $19 USDC one-time; Polygon USDC; no subscriptions | BRIEF-software-distribution-substrate spec |
| Reference impl | app-orchestration-slm (license.rs, node_circuit.rs, wire type patterns) | Canonical existing pattern |
| user-pairings.yaml | Separate file from pairings.yaml (same directory) | Infrastructure ACK (2026-06-29): topology vs application-layer separation; prevents scope contamination and unbounded growth in pairings.yaml |
| WORM ledger schema_version | `"schema_version": "1"` in every JSONL append entry | Infrastructure ACK (2026-06-29): forward compatibility for future schema evolution (e.g., `revoked_on`, key rotation) |

---

## Architecture rationale

**Why current_thread Tokio:**
The workload is IO-bound — file reads for pairings.yaml, archive manifests, and inbox
counts; HTTP request routing to archive inboxes. No CPU-parallel computation exists at
v0.0.1. current_thread avoids spawning N OS threads and their associated stack memory
(~8 MB per thread). Observed difference: 4–8 MB RSS vs 30–40 MB for multi-thread.
Mandatory per BRIEF-OS-FAMILY Phase 2 idle target of ≤96 MB.

**Why no central permission database:**
DOCTRINE Claim #43 (single-boundary compute) + Claim #52 (moduleId isolation). A
permissions database becomes a second source of truth that can drift from the pairing
topology. pairings.yaml IS the topology. os-orchestration reads it at startup and
exposes it — it does not own it. This is also what makes decommission structurally
clean: delete the instance; the access scope evaporates with no database rows to clean.

**Why invite token over TOFU fingerprinting:**
TOFU (trust-on-first-use) was the Phase 1–2 model for Totebox pairing. It works when
the operator is online for the team member's first connection. For os-orchestration, the
operator may not be available at that moment. The token model allows asynchronous grants:
issue a token → share via any channel → team member redeems on their own schedule.
Single-use nonce prevents replay. Ed25519 signature ties the grant to the operator's key.
TOFU is retained as a fallback for Totebox peers not yet issuing tokens.

**Why user-pairings.yaml separate from pairings.yaml:**
pairings.yaml is cluster topology (infrastructure scope) — it describes which archives
are paired with which orchestration instances. user-pairings.yaml is application-layer
runtime state — it records which humans/devices have paired via the invite token ceremony.
Mixing them causes: (1) unbounded growth in a file meant to describe static topology;
(2) scope contamination — infrastructure tooling reading pairings.yaml might misinterpret
user entries as archive topology rows. Infrastructure ACK confirmed this separation
2026-06-29.

---

## v0.1.0 planned scope

- **app-orchestration-graph:** real federation queries replacing the v0.0.1-stub
  - Design open: DataGraph proxy vs local entity graph vs hybrid (see Decisions open)
- **PairingStore startup load:** read user-pairings.yaml at startup to restore
  in-process state across restarts (currently rebuilt from empty on each start)
- **WORM log revocation:** `pairing_revoked` event type; schema_version "2" upgrade
- **Phase 4 VPN bind:** 10.42.0.9:8020 when WireGuard Part A lands from Command
- **Fingerprint upgrade:** real SHA-256 via `sha2` crate (currently FNV-1a, correctly labeled)
- **peer_type in PairResponse:** after project-totebox ACK on Totebox-side /v1/pair
- **Multi-tenant license tier:** number-of-archives gating in license payload (Command scope)

---

## Decisions open

| Question | Status | Owner | Target |
|---|---|---|---|
| graph stub port | Closed — port 8021 (implemented) | — | — |
| token wire format version | Closed — resolved as `schema_version: "1"` in WORM ledger (infrastructure ACK) | — | — |
| Phase 4 VPN bind timing | Open — when does WireGuard Part A land? | Command Session | NEXT.md T1 |
| Multi-tenant license tier | Open — number-of-archives gating in license payload? | Command Session | v0.1.0 |
| peer_type in PairResponse | Open — add `"peer_type": "orchestration"` to PairResponse wire type | project-totebox must ACK Totebox-side /v1/pair first | Before v0.0.2 |
| Totebox-side /v1/pair | Open — does Totebox issue same Ed25519 invite token format? peer_type in payload or response? | project-totebox | Before v0.0.2; outbox sent 2026-06-29 |
| Graph federation design | Open — what queries does app-orchestration-graph answer? DataGraph proxy? full entity graph? hybrid? | project-orchestration | v0.1.0 |
| WORM log revocation | Open — pairing_revoked event? schema_version "2" migration? backward compat? | project-orchestration | v0.1.0 |
| Fingerprint upgrade | Open — upgrade key_fingerprint FNV-1a → SHA-256 (sha2 crate dep); is the dep cost worth it? | project-orchestration | v0.1.0 |
| PairingStore startup load | Open — read user-pairings.yaml at startup to restore in-process state across restarts? | project-orchestration | v0.1.0 |

---

## Work log

- 2026-06-29 — BRIEF created; research complete; plan approved; Step 1 complete
- 2026-06-29 — Implementation complete: 3-crate workspace, 7 tests pass, 1.7 MB binary
  Axum route fix (`:param` → `{param}`). Committed to cluster/project-orchestration.
  Stage 6 pending — staging mirror rejected (remote main 18+ commits ahead);
  needs Command Session rebase + canonical merge via promote.sh.
  project-registry.md needs update on monorepo main branch via Command outbox.
- 2026-06-29 — Actioned two high-priority Command relay messages:
  (1) peer-agnostic pairing protocol amendment — unified /v1/pair wire protocol required;
  (2) infrastructure schema ACK — user-pairings.yaml + schema_version + pairing-write.sh.
  Locked two new decisions. Opened two new decisions (peer_type field, Totebox-side /v1/pair).
  Outbox sent to project-totebox requesting Totebox-side /v1/pair design ACK.
- 2026-06-29 — v0.0.2 patch applied: pairing.rs — schema_version in WORM ledger, write-through
  to user-pairings.yaml, renamed sha256_hex → key_fingerprint. 7/7 tests pass. Release binary rebuilt.
- 2026-06-29 — Deployment provisioned: gateway-orchestration-command-1/ (MANIFEST + READMEs).
  Infrastructure draft committed to cluster (systemd unit + bootstrap.sh). bootstrap.sh updated
  to curl-download binary from software.pointsav.com (BINARY_URL/BINARY_SRC/default URL priority).
  Outbox to Command for install; outbox to project-software for BETA listing (no payment gate).
- 2026-06-29 — NEXT.md updated: P3.4 corrected (right deployment name, partial status), P3.5 closed.
  Inbox: J5 + J2 HOLD messages actioned (confirmed HOLD; carry-forward in BRIEF).
- 2026-07-09 — Backlog sweep: BRIEFs re-scoped (this one = shipping state;
  build-out BRIEF = architecture/roadmap), brief-audit-2026-06 archived, NEXT.md P3.4
  checkboxes corrected against ledger evidence (install/smoke-test/BETA-URL were already
  done, just unchecked). Confirmed v0.0.1 in canonical `origin/main`; v0.0.2 was NOT —
  pushed to staging mirrors via `self-service-promote.sh` and queued for Command
  (commit `95f4ca2f`). Discovered the formal `bin/build-soft.sh` signed pipeline has
  never run for this binary; requested from Command via outbox. Re-pinged project-totebox
  on the stalled `/v1/pair` design ACK (11 days silent).
- 2026-07-15 — Re-verified (not just re-read) the two blockers: `origin/main` still only
  has v0.0.1 (`29d0b4a1`); v0.0.2 (`95f4ca2f`) remains queued in `promote-queue.jsonl`,
  `verified: false` — Command has not merged it. `peer_type`/Totebox-side `/v1/pair` ACK
  confirmed still unanswered (16 days since original ask, 6 since the 2026-07-09 ping) —
  sent a second status-check re-ping to project-totebox
  (`command-20260716-status-check-totebox-side-v1-pair-design`). Starting v0.1.0 work that
  does not depend on either blocker: SHA-256 fingerprint upgrade, `PairingStore` startup
  load from `user-pairings.yaml`, and WORM ledger `pairing_revoked` event
  (`schema_version: "2"`, old `"1"` entries treated as not-revoked for backward compat).
- 2026-07-15/16 — Operator asked whether os-orchestration was ready to test. Checked the
  live deployment (`gateway-orchestration-command-1`, port 8020): running, but stale
  (v0.0.1, `29d0b4a1`), observation mode (no license token), and **fleet loading silently
  broken since first deploy 2026-06-29** — `fleet.rs`'s `PairingsYaml` struct expected a
  top-level `archives:` key; the real `pairings.yaml` uses `pairings:`. `/v1/archives` has
  returned an empty list in production for the entire deployment's life; only a WARN in
  the log, never a startup failure. Fixed (`dc2899b1`), added a regression test against
  the real `pairings.yaml` shape. Verified live end-to-end on a scratch instance (port
  18021, dev-minted Ed25519 license token — the `[0u8;32]` "dev key" in `main.rs` is a
  placeholder `VerifyingKey` with no matching `SigningKey`, generated a real keypair
  instead): `/v1/archives` now returns real fleet data; full invite→pair flow produces a
  `sha256:`-prefixed `key_fingerprint` in the WORM ledger; **restart test confirmed
  `PairingStore::load()` genuinely restores prior pairings** — re-pairing the same key
  post-restart returned `already_paired` with the original pre-restart `paired_on`
  timestamp, not just a restored count. Production `/usr/local/bin` swap + systemd
  restart is Command Session scope (`deploy-binary.sh` guards against running from a
  Totebox clone and requires HEAD already on canonical `origin/main`) — not done here;
  `dc2899b1` is staged via Stage 6 lite, awaiting Command's canonical merge same as the
  earlier v0.1.0 commit.

---

## Carry-forward

- **J5 instrumentation (HOLD):** Collect session isolation timing + archive provisioning latency
  during Phase 3 implementation. Flag to totebox@project-system when data available.
  Msg-id: `project-system-20260527-j2-critical-bench9-blocker` (ref in project-system outbox)
- **J2 journal (HOLD):** Read J2 §3–§4 before expanding J5 or designing Phase 3
  instrumentation suite. J2 blocked on Bench #9 at project-system.
- **HOLD — peer_type wire change (v0.0.2):** Do NOT add `peer_type` field to PairResponse
  or activate unified pairing protocol until project-totebox ACKs Totebox-side /v1/pair design.
  Msg ref: `command-20260629-relay-pairing-protocol-must-be-peer-agnostic`
  Outbox sent to project-totebox: `re: Totebox-side /v1/pair endpoint design — peer-agnostic pairing protocol`
- **Code patch (v0.0.2) — after Totebox ACK:**
  - `orchestration-command-core/src/lib.rs` → add `peer_type: String` to `PairResponse`
  - F11 Peers tab: confirm with project-console
- **Stage 6 → Command:**
  - v0.0.1 confirmed present in canonical `origin/main` (commit `29d0b4a1`) — done.
  - v0.0.2 (pairing.rs schema_version + user-pairings.yaml write-through) pushed to
    staging mirrors and queued in `promote-queue.jsonl` 2026-07-09 (commit `95f4ca2f`);
    awaiting Command's next canonical-merge pass.
  - project-registry.md update also pending at Command.
- **Formal SOFT- pipeline (2026-07-09 finding):** `bin/build-soft.sh` has never actually
  run for this binary — `data/app-repository/registry.yaml` is `packages: {}`. The live
  BETA listing is an informal handoff (project-software manually installed the dev
  binary; ledger `source_commit: "pending-stage6"`). Requested from Command via outbox
  2026-07-09: run `build-soft.sh` once v0.0.2 is canonical, producing a real signed
  `data/app-repository/` entry + `registry-update` to project-software.
- **Binary distribution pipeline (v0.0.1 BETA):**
  - Send binary `orchestration-command-server` (Linux x86_64, 1.7 MB) to project-software
    for upload to software.pointsav.com. Product slug: `soft-orchestration-command`.
  - **BETA mode — no payment gate.** Outbox sent to project-software (2026-06-29).
    project-software must list the product at $0 / free download during BETA and confirm
    the canonical download URL so bootstrap.sh can be updated with the real path.
  - Install model: operators run bootstrap.sh which curl-downloads the binary from
    software.pointsav.com. No `cargo build` required on target machine.
  - **BETA → production flip:** when BETA ends, project-software re-enables $1/$19 USDC
    pricing on software.pointsav.com. No code change in this crate needed.
  - bootstrap.sh updated (2026-06-29) with download-first logic: `BINARY_URL` env var
    overrides the default URL; `BINARY_SRC` env var allows local dev builds as fallback.
- **Download URL confirmation (open):** project-software must reply with the canonical
  software.pointsav.com download URL for `orchestration-command-server` Linux x86_64 once
  the BETA listing is live. Update `DEFAULT_BINARY_URL` in bootstrap.sh when received.
