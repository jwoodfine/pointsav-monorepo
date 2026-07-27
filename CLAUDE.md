@~/Foundry/AGENT.md

# project-software — Archive Guide

> **State:** active | **Last updated:** 2026-07-09
> **Cluster manifest:** `.agent/manifest.md`
> **Workspace AGENT.md takes precedence on conflict.**

---

## Cluster mission

Software distribution substrate — `software.pointsav.com`. Owns the
Ed25519 license key pipeline, Polygon USDC payment verification, and
SOFT artifact type governance. This archive's root IS the
`pointsav-monorepo` clone (there is no separate sub-clone one level
down) — `Cargo.toml` and the crate/app/service directories sit
directly at this directory's root.

**Owns:**
- `app-privategit-marketplace` — software.pointsav.com storefront (static pages, `/v1/products`, `/v1/license`, `/v1/claim`)
- `app-privategit-source` — release server (`/releases/*` binary streaming + MANIFEST; `/git/*` smart-HTTP stub)
- `app-mediakit-distributions` — distribution catalogue server
- `tool-wallet` — Polygon USDC watcher (`eth_getLogs` + receipt writer)
- `os-privategit` — deployment target OS image (hosts marketplace + source + wallet)

See `~/Foundry/.agent/memory/project_software_distribution_substrate.md`
and `~/Foundry/.agent/memory/project_software_architecture_decisions.md`
for ratified architecture decisions (pricing, payment rail, license
key format).

## Tetrad

See `.agent/manifest.md` `tetrad:` block for the canonical declaration
across vendor / customer / deployment / wiki legs. **Reconciled
2026-07-15:** the deployment leg is now declared `active` in the
manifest — `vault-privategit-software-1` is a confirmed live,
provisioned instance (`~/Foundry/deployments/vault-privategit-software-1/`,
`state: active`, public at `software.pointsav.com` since 2026-05-17).
The second route `pairings.yaml` lists for this cluster,
`media-distribution-software-1`, has no corresponding
`~/Foundry/deployments/` instance directory — it reads as a
planned/routing declaration, not a provisioned instance; unresolved,
left as an open note in the manifest. Vendor, customer, and wiki legs
remain `leg-pending`.

## At session start

Per `~/Foundry/AGENT.md` § Session roles:

1. Confirm role: `~/Foundry/bin/foundry-role.sh` (Totebox Session expected)
2. Write session lock: `.agent/engines/<engine-id>/session.lock`
3. Read `.agent/manifest.md` — cluster mission + tetrad
4. Call `get_session_brief(role="totebox", archive="project-software")` — replaces inbox, NOTAM, session-context reads
5. Read `~/Foundry/NOTAM.md` — workspace warnings
6. Read `.agent/rules/*.md` — includes `project-registry.md`, `repo-layout.md`, `cleanup-log.md`, `datagraph-discipline.md`, `handoffs-outbound.md`

## Hard rules (workspace-level, do not duplicate; reference only)

- `~/Foundry/AGENT.md` § Hard rules — identity store immutable, never
  chmod; preview before writing; edit in place (no `_V2` files);
  one session per repo; Bloomberg standard; BCSC posture; SYS-ADR-07/10/19.
- `~/Foundry/CLAUDE.md` § Size discipline — per-archive CLAUDE.md ≤ 150 lines.

## Cluster branch + promote

This archive runs on `cluster/project-software`. `.agent/` commits stay here permanently.
Code commits promote to canonical via `~/Foundry/bin/promote.sh` (filters `.agent/` automatically).

Session start: `git branch --show-current` → must return `cluster/project-software`.
Commits via `~/Foundry/bin/commit-as-next.sh "<message>"` from archive root.
**Stage 6 self-service (this archive): `build-deploy-stage6lite`** (per `pairings.yaml`) —
`~/Foundry/bin/self-service-promote.sh` pushes code commits to staging mirrors and
appends to `promote-queue.jsonl`. Command Session processes the canonical merge.
Do NOT run `promote.sh` directly.

## Fast gates

```
cargo check -p <crate>
cargo test -p <crate>
cargo clippy -p <crate> -- -D warnings
cargo fmt -p <crate> --check
```

Substitute the crate for the active project (e.g. `app-privategit-marketplace`, `tool-wallet`).

## Conflicts

If a workspace rule conflicts with anything stated here, **stop and surface
the conflict via outbox to Command Session** — do not silently override.

## MCP tools — `foundry` server (use at startup)

`get_session_brief(role="totebox", archive="project-software")` replaces manually reading
inbox.md, outbox.md, NOTAM.md, session-context.md. Call it first.

| Tool | When to use |
|---|---|
| `get_session_brief` | **First call at startup** — inbox, outbox, NOTAM, session-context |
| `send_mailbox_message` | Send any mailbox message (M-2/M-10 audit compliant) |
| `query_datagraph` | Entity lookup before answering about people/projects |
| `ask_local` | OLMo 7B local inference — free, SYS-ADR-07-safe |

## Artifact types — bright-line rules

SOFT = Ed25519 license key + marketplace listing + price → software.pointsav.com.
CODE = runs our systems; no customer license; internal deploy only.
Storefront (`app-privategit-marketplace`) is CODE; the merchandise it sells is SOFT.
Cash register test: licensable + marketplace-listed → SOFT; everything else → CODE.

## pointsav-monorepo sub-clone

Generic sub-clone conventions (fast gates, commit rules, layout) live at
`@~/Foundry/conventions/pointsav-monorepo-subclone-guide.md` — that file is
never archive-specific and is never touched by any archive's Stage-6
promotion. This archive's own identity/mission content belongs only here,
never in the sub-clone.
