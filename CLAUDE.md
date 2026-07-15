@~/Foundry/AGENT.md

# project-workplace — Archive Guide

> **State:** active | **Last updated:** 2026-07-15
> **Cluster manifest:** `.agent/manifest.md`
> **Workspace AGENT.md takes precedence on conflict.**

---

## Cluster mission

Workbench OS surface — `app-privategit-workbench` + `moonshot-*` crates. Browser+localhost
staging OS for the PointSav developer experience. Hosts the Workbench launcher, code IDE, and
document surfaces. Consolidating three overlapping prototypes
(`app-workplace-workbench`, `app-privategit-workbench`, `app-workplace-http-prototype`) into
one, on a clean-sheet Rust→WASM document engine, per `BRIEF-workplace-workbench.md`. Every
third-party dependency is tracked as a `moonshot-*` full-stack-ownership target
(docengine, parser, crdt, editor, bim-engine).

Tetrad legs are all `leg-pending` (see `.agent/manifest.md`) — no vendor/customer/deployment/wiki
artifact has shipped yet; this archive is still in active prototype development.

## Tetrad

See `.agent/manifest.md` `tetrad:` block for the canonical declaration
across vendor / customer / deployment / wiki legs.

## At session start

Per `~/Foundry/AGENT.md` § Session roles:

1. Confirm role: `~/Foundry/bin/foundry-role.sh` (Totebox Session expected)
2. Write session lock: `.agent/engines/<engine-id>/session.lock`
3. Read `.agent/manifest.md` — cluster mission + tetrad
4. Call `get_session_brief(role="totebox", archive="project-workplace")` — replaces inbox, NOTAM, session-context reads
5. Read `~/Foundry/NOTAM.md` — workspace warnings
6. Read `.agent/rules/*.md` if present (may be absent for newer archives)

## Hard rules (workspace-level, do not duplicate; reference only)

- `~/Foundry/AGENT.md` § Hard rules — identity store immutable, never
  chmod; preview before writing; edit in place (no _V2 files);
  one session per repo; Bloomberg standard; BCSC posture; SYS-ADR-07/10/19.
- `~/Foundry/CLAUDE.md` § Size discipline — per-archive CLAUDE.md ≤ 150 lines.

## Commit + promote

Commits to pointsav-monorepo (the nested `pointsav-monorepo/` sub-clone; separate `.git`,
tracks `main`) use: `~/Foundry/bin/commit-as-next.sh "<msg>"`.
**Stage 6:** Build + deploy to staging is self-service (`self_service: build-deploy`).
Canonical promote is not self-service — write `"Stage 6 pending — project-workplace — <crate>"`
to outbox at shutdown. Command Session processes canonical merge.

## Deploy model

Three systemd services run today (verified 2026-07-15), all localhost/PPN-scoped — no
public-internet deployment exists yet for this archive's surfaces (Tetrad deployment leg
stays leg-pending):
- `local-workplace-http-prototype.service` — `app-workplace-http-prototype`, localhost:9110,
  for office staff to iterate with.
- `app-privategit-workbench.service` — port 9210, proxied via nginx to `10.8.0.9:9200` (PPN).
  This is Jennifer's actual live working instance (confirmed via Command 2026-07-15) — treat
  it as production-sensitive, not a throwaway dev copy.
- `local-workbench-dev.service` — `app-privategit-workbench` DEV instance, port 9215.

There is no `foundry-prod` push and no public-internet deployment yet.

## Conflicts

If a workspace rule conflicts with anything stated here, **stop and surface
the conflict via outbox to command session** — do not silently override.

## MCP tools — `foundry` server (use at startup)

`get_session_brief(role="totebox", archive="project-workplace")` replaces manually reading
inbox.md, outbox.md, NOTAM.md, session-context.md. Call it first.

| Tool | When to use |
|---|---|
| `get_session_brief` | **First call at startup** — inbox, outbox, NOTAM, session-context |
| `send_mailbox_message` | Send any mailbox message (M-2/M-10 audit compliant) |
| `query_datagraph` | Entity lookup before answering about people/projects |
| `ask_local` | OLMo 7B local inference — free, SYS-ADR-07-safe |

## Artifact types — bright-line rules

TOPIC = explains WHAT/WHY; public wiki; bilingual EN+ES.
GUIDE = instructs HOW-NOW; woodfine-fleet-deployment/<name>/; English-only.
CODE = runs our systems; no customer license; internal deploy only.
