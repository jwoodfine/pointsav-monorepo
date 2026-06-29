---
artifact: brief
schema: foundry-brief-v1
brief-id: project-orchestration-os-orchestration
title: "os-orchestration build-out — app-orchestration-command v0.0.1"
status: active
owner: project-orchestration
created: 2026-06-29
updated: 2026-06-29
---

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
- software.pointsav.com listing (Command Session scope; request via outbox after Stage 6)

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

---

## Decisions open

| Question | Options | Owner | Target |
|---|---|---|---|
| graph stub port | Does app-orchestration-graph need a port for v0.0.1? | This archive | Before implementation |
| Phase 4 VPN bind timing | When does WireGuard Part A land from Command? | Command Session | NEXT.md T1 |
| Multi-tenant license tier | Number-of-archives gating in license payload? | Command Session | v0.1.0 |
| token wire format version | Does token need a `version:` field for future compatibility? | project-console | Before outbox ACK |

---

## Work log

- 2026-06-29 — BRIEF created; research complete; plan approved; Step 1 in progress

---

## Carry-forward

- **J5 instrumentation (HOLD):** Collect session isolation timing + archive provisioning latency
  during Phase 3 implementation. Flag to totebox@project-system when data available.
  Msg-id: `project-system-20260527-j2-critical-bench9-blocker` (ref in project-system outbox)
- **Cross-archive ACKs needed:** project-console, project-totebox, project-infrastructure
  must ACK the invite token spec before any of those archives begin implementation.
  Watch for outbox replies.
- **Stage 6 → Command:** After first clean build, write outbox to command@claude-code:
  "Stage 6 pending — project-orchestration — app-orchestration-command v0.0.1"
- **Marketplace listing:** Command Session action; request via outbox after Stage 6 confirmed.
