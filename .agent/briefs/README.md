# Briefs — project-knowledge

Active briefs for this archive. Read at session start.

| Brief | Status | Summary |
|---|---|---|
| [BRIEF-knowledge-ng-rewrite.md](BRIEF-knowledge-ng-rewrite.md) | active | **100% ground-up rewrite** of the wiki engine — P0-P8 all complete and live in production (verified 2026-07-09). Full knowledge-platform vision build-out (claim layer, DTCG tokens, print mode, editorial linter) landed 2026-07-13 — see its own status update. Footer redesign + article-pager removal landed 2026-09-04, live on canonical. **P5b JOURNAL render: engine-complete but architecturally orphaned, discovered 2026-09-05** — all 6 phases built and tested (173/173 pass), but per project-editorial's ratified 2026-07-16 decision, "the three media-knowledge wikis never host JOURNALs at all" (JOURNALs live on 6 other sites instead — see BRIEF for the list). This crate's `/research/{slug}` machinery predates that ratification by days and will likely never serve real content for its three tenants. Flagged to Command + project-editorial; future call on whether to retire the routes or keep them as reusable reference code. Supersedes the OLD-engine design briefs, now in `archive/` |
| [BRIEF-print-mode.md](BRIEF-print-mode.md) | active | Print-mode design research + decision (pure CSS, no Paged.js) and what shipped, per Phase 9 of the 2026-07-13 build-out. Browser-in-the-loop verified — found + fixed a real citation-stamp cascade bug. Child of BRIEF-knowledge-ng-rewrite |
| [BRIEF-binary-distribution.md](BRIEF-binary-distribution.md) | active | app-mediakit-knowledge on software.pointsav.com — Format A + B both live (2026-07-01); ng-rewrite binary already built/deployed/handed off. **Pending supersession (2026-09-04 status note added)** — `BRIEF-os-mediakit-product-family.md` item 8 has ratified seL4/Microkit as the sole os-mediakit, replacing both formats; no-gap cutover, this listing stays live unchanged until that lands |
| [BRIEF-sovereign-editorial.md](BRIEF-sovereign-editorial.md) | active | Parent tracker for the Sovereign Editorial design-direction handoff. Technical content lives entirely in the 2 children below |
| [BRIEF-sovereign-editorial-marketing.md](BRIEF-sovereign-editorial-marketing.md) | reference | Sovereign Editorial spec for project-marketing (home.pointsav.com + home.woodfinegroup.com) — waiting on that archive's session. Child of BRIEF-sovereign-editorial |
| [BRIEF-sovereign-editorial-software.md](BRIEF-sovereign-editorial-software.md) | reference | Sovereign Editorial spec for project-software (software.pointsav.com) — waiting on that archive's session. Child of BRIEF-sovereign-editorial |
| [BRIEF-os-mediakit-product-family.md](BRIEF-os-mediakit-product-family.md) | active | os-mediakit (base) + app-mediakit-knowledge (first product, live) architecture. **Ratified 2026-09-01**: seL4/Microkit becomes the sole os-mediakit, replacing bare-binary + plain-QCOW2 entirely (see `BRIEF-binary-distribution.md`'s pending-supersession note). **Build-out plan**: G1→G4/SIGTERM gates all DONE + live-verified; `/data` persistence DONE (2-boot-proof); G-TLS reverse-proxy scaffolding DONE (real cert issuance still blocked, no target VM); Phase Deploy scripts written/committed, not yet run (needs a real target). Blocked only on Decisions-open item 2 (host-ingress) / Phase T2A timing, deliberately deferred by the operator — not further scripting work available locally |
| [BRIEF-footer-ci-check.md](BRIEF-footer-ci-check.md) | active | New 2026-09-08 — project-editorial asked us to own scoping/building a CI check that fails the build if a site's rendered footer diverges from `app-mediakit-shell`'s YAML token source (their BRIEF-footer-badge-token-architecture.md Decision #5). Accepted. Not yet scoped — queued behind Phase 2 (local os-mediakit appliance re-test) |

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
