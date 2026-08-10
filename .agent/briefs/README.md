# Briefs — project-knowledge

Active briefs for this archive. Read at session start.

| Brief | Status | Summary |
|---|---|---|
| [BRIEF-knowledge-ng-rewrite.md](BRIEF-knowledge-ng-rewrite.md) | active | **100% ground-up rewrite** of the wiki engine — P0-P8 all complete and live in production (verified 2026-07-09). Full knowledge-platform vision build-out (claim layer, DTCG tokens, print mode, editorial linter) landed 2026-07-13 — see its own status update. P5b JOURNAL render still the only fully-open item — editorial reply pending. Supersedes the OLD-engine design briefs, now in `archive/` |
| [BRIEF-print-mode.md](BRIEF-print-mode.md) | active | Print-mode design research + decision (pure CSS, no Paged.js) and what shipped, per Phase 9 of the 2026-07-13 build-out. Browser-in-the-loop verified — found + fixed a real citation-stamp cascade bug. Child of BRIEF-knowledge-ng-rewrite |
| [BRIEF-binary-distribution.md](BRIEF-binary-distribution.md) | active | app-mediakit-knowledge on software.pointsav.com — Format A + B both live; ng-rewrite binary already built/deployed/handed off. Only the project-software catalog-listing update is still pending on their side. Condensed 2026-07-13 (resolved history moved to git log) |
| [BRIEF-sovereign-editorial.md](BRIEF-sovereign-editorial.md) | active | Parent tracker for the Sovereign Editorial design-direction handoff. Technical content lives entirely in the 2 children below |
| [BRIEF-sovereign-editorial-marketing.md](BRIEF-sovereign-editorial-marketing.md) | reference | Sovereign Editorial spec for project-marketing (home.pointsav.com + home.woodfinegroup.com) — waiting on that archive's session. Child of BRIEF-sovereign-editorial |
| [BRIEF-sovereign-editorial-software.md](BRIEF-sovereign-editorial-software.md) | reference | Sovereign Editorial spec for project-software (software.pointsav.com) — waiting on that archive's session. Child of BRIEF-sovereign-editorial |
| [BRIEF-os-mediakit-product-family.md](BRIEF-os-mediakit-product-family.md) | active | os-mediakit (base, real Ubuntu 24.04 QCOW2 appliance today, not a scaffold) + app-mediakit-knowledge (first product, live) architecture/dev-plan. Ownership locked: marketing→project-marketing (already real/deployed), distributions→project-newsroom (scaffold). Opus+Fable independently reviewed and converged: reuse os-totebox/os-orchestration's Pattern A (seL4/Microkit+vendor-libvmm) as terminal architecture if/when Phase 3 starts; VM split per-binary (3) not per-tenant (5) as the floor, full doctrine §L 5-VM end state gated on a named trigger. Found 3 doctrine/wiki inconsistencies still needing Command correction |

> **Archived (2026-07-13 cleanup pass):** `BRIEF-wiki-redesign` (superseded),
> `BRIEF-phase2-redesign`, `BRIEF-slides`, `BRIEF-visual-excellence`, `BRIEF-inline-annotations`,
> and `BRIEF-knowledge-platform-master` were already carrying `status: archived`/`superseded`
> in frontmatter but still sat in this directory — physically `git mv`'d into `archive/` this
> pass so the main folder only shows what's actually active/reference. All six describe
> OLD-engine work the ng-rewrite replaced from scratch; content and history are unchanged,
> only the file location moved. Not re-indexed here per `archive/`'s own convention below.

## `archive/` subfolder

`.agent/briefs/archive/` holds git-mv'd historical briefs (mix of project-knowledge-owned and
foreign-archive briefs from past cross-archive contamination — see NEXT.md for the foreign-owned
cleanup flagged to Command 2026-07-09). Not individually indexed here; consult the folder
directly or `git log --follow` on a specific file if you need history on an archived item.

## Foreign-owned briefs present in this folder (contamination, not ours)

As of 2026-07-09, 15+ BRIEF files owned by other archives (project-design, project-bim,
project-gis, project-data, project-console, project-marketing, project-workplace,
project-infrastructure, project-intelligence, project-editorial) are physically sitting in
this archive's `.agent/briefs/` folder (main dir and/or `archive/`). These are NOT
project-knowledge's to maintain or index here — flagged to Command 2026-07-09 for a proper
cross-archive sweep (msg-id `command-20260709-cross-archive-cleanup-needed-stray-dupli`).
Do not add them to the table above; do not relocate them from this archive without Command
coordination.
