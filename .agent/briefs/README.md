# Briefs — project-orchestration

`BRIEF-*.md` files are permanent git-tracked artifacts. Never delete — supersede via
`status: archived` or `git mv` to `briefs/archive/`. See `conventions/brief-discipline.md`.

## Active briefs

| File | brief-id | Title | Status | Updated |
|------|----------|-------|--------|---------|
| [BRIEF-os-orchestration.md](BRIEF-os-orchestration.md) | project-orchestration-os-orchestration | os-orchestration build-out — app-orchestration-command v0.0.1 | active | 2026-06-29 |
| [BRIEF-os-orchestration-build-out.md](BRIEF-os-orchestration-build-out.md) | project-orchestration-os-orchestration-build-out | os-orchestration: Stateless Aggregation Layer — Full Build-Out | active | 2026-07-09 |

**Needs reconciliation (flagged 2026-07-09, Command redistribution):** these two BRIEFs
overlap substantially — both cover os-orchestration's commercial-tier positioning and the
`app-orchestration-command`/`app-orchestration-*` family. `BRIEF-os-orchestration.md` is
narrower and more current (the actual shipping v0.0.1/v0.0.2 deployment/permission/licensing
model). `BRIEF-os-orchestration-build-out.md` is broader and older (full 5-app architecture,
seL4 capability-broker PD design, Phase O0-O6 roadmap, journal tie-ins) — just physically
redistributed here from project-totebox, where it had been misscoped since a 2026-06-20
archive merge. Next project-orchestration session should decide: merge into one BRIEF, or
keep both with `-build-out` scoped explicitly to the broader/older architectural vision and
`BRIEF-os-orchestration.md` to the current concrete shipping work — not decided here.

## Artifact routing
When a BRIEF graduates to a deliverable, it routes here:

| Artifact type | Destination |
|---|---|
| CODE-* | monorepo sub-clone; Stage 6 READY to Command |
| TOPIC-* | project-editorial drafts-outbound |
| DESIGN-* | project-design drafts-outbound |
| JOURNAL-* | project-editorial drafts-outbound |
| GUIDE-* | Command Session (woodfine/* customer tier) |

## Archived briefs

BRIEFs with `status: archived` or `status: superseded` are listed here or moved to
`briefs/archive/`.

| File | Archived date | Notes |
|------|--------------|-------|
