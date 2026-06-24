@~/Foundry/AGENT.md

# project-software — Archive Guide

> This CLAUDE.md covers the `pointsav-monorepo/` sub-clone common to all
> Totebox Archives that use the monorepo as their vendor leg.
> **Archive-level guidance (mission, tetrad, MCP startup) lives in `../CLAUDE.md`.**
> **Workspace AGENT.md takes precedence on conflict.**

---

## Sub-clone role

`pointsav-monorepo` is the vendor-leg sub-clone for any Totebox Archive whose
work lives in this monorepo. Commit here via `commit-as-next.sh`; promote to
canonical (`vendor/pointsav-monorepo`) via Command Session `promote.sh`.

The monorepo is shared — multiple Totebox Archives may have clones of it.
Never force-push; never reset --hard without operator approval and all sessions
confirming inactive.

## Fast gates

```
cargo check -p <crate>
cargo test -p <crate>
cargo clippy -p <crate> -- -D warnings
cargo fmt -p <crate> --check
```

Substitute the crate for the active project. For project-knowledge: `app-mediakit-knowledge`.

## Key layout

```
pointsav-monorepo/
├── Cargo.toml              workspace manifest (all member crates)
├── app-mediakit-knowledge/ wiki engine (project-knowledge focus)
├── app-console-*/          TUI cartridges (project-console focus)
├── service-*/              backend services
└── scripts/                xtask, dtcg-to-css.py, stage6-gate.sh
```

## Commit + promote

Commits via `~/Foundry/bin/commit-as-next.sh "<message>"` from archive root.
**Stage 6 self-service (this archive):** `~/Foundry/bin/self-service-promote.sh`
— pushes code commits to staging mirrors + appends to `promote-queue.jsonl`.
Command Session processes canonical merge. Do NOT run `promote.sh` directly.

## Commit rules

- `git add <specific files>` — never `git add .`
- `~/Foundry/bin/commit-as-next.sh "<type>(<scope>): <message>"` from sub-clone CWD
- Run `cargo test -p <crate>` before every commit
- If unpromoted commits exist, write `"Stage 6 pending — project-knowledge — <crate>"` to outbox

## Conflicts

Surface via archive outbox (`../.agent/outbox.md`) — not here.
Do not write to another archive's state files.

## MCP tools — `foundry` server (use at startup)

`get_session_brief(role="totebox", archive="project-software")` replaces manually reading
inbox.md, outbox.md, NOTAM.md, session-context.md. Call it first.

| Tool | When to use |
|---|---|
| `get_session_brief` | **First call at startup** — inbox, outbox, NOTAM, session-context |
| `send_mailbox_message` | Send any mailbox message (M-2/M-10 audit compliant) |
| `query_datagraph` | Entity lookup before answering about people/projects |
| `ask_local` | OLMo 7B local inference — free, SYS-ADR-07-safe |
