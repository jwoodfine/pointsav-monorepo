# Briefs — project-orchestration

`BRIEF-*.md` files are permanent git-tracked artifacts. Never delete — supersede via
`status: archived` or `git mv` to `briefs/archive/`. See `conventions/brief-discipline.md`.

## Active briefs

| File | brief-id | Title | Status | Updated |
|------|----------|-------|--------|---------|
| [BRIEF-os-orchestration.md](BRIEF-os-orchestration.md) | project-orchestration-os-orchestration | os-orchestration build-out — app-orchestration-command (v0.0.1/v0.0.2 in BETA) | active | 2026-07-09 |
| [BRIEF-os-orchestration-build-out.md](BRIEF-os-orchestration-build-out.md) | project-orchestration-os-orchestration-build-out | os-orchestration: Stateless Aggregation Layer — Full Build-Out | active | 2026-07-09 |

**Reconciled 2026-07-09:** the two BRIEFs are scoped to be non-overlapping.
`BRIEF-os-orchestration.md` owns concrete shipping/deployment/licensing state for
`app-orchestration-command` — what's actually coded and running today.
`BRIEF-os-orchestration-build-out.md` owns the long-range architecture and roadmap —
the three-binary context, seL4 capability-broker PD design, five-app activation
sequence (Phase O0–O6), and journal (J2/J5) tie-ins. Each BRIEF cross-references the
other at the top for readers who land on the wrong one.

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
| [BRIEF-brief-audit-2026-06.md](BRIEF-brief-audit-2026-06.md) | 2026-07-09 | Point-in-time audit log (0 BRIEFs existed 2026-06-12); stale now that two real BRIEFs exist. |
