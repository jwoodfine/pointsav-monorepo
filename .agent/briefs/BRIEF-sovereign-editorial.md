---
artifact: brief
schema: foundry-brief-v1
status: active
brief-id: project-knowledge-sovereign-editorial
parent: project-knowledge-phase2-redesign
owner: project-knowledge
created: 2026-07-09
updated: 2026-09-04 (carry-forward outbox check re-run — still unresolved, no news)
---

# BRIEF — Sovereign Editorial Design Direction (parent tracker)

> **Parent:** [[project-knowledge-phase2-redesign]]
> **Children:** [[project-knowledge-sovereign-editorial-marketing]],
> [[project-knowledge-sovereign-editorial-software]]

## Context

2026-06-24 browser-in-the-loop audit (6 sites × 6 viewports × 3 tracks, 7-agent workflow)
found all non-wiki live sites nowhere near hyperscaler grade. project-knowledge holds
master design-direction authority ("Sovereign Editorial": dark navy #164679 masthead,
near-black #0e1117 footer, Playfair Display Variable + IBM Plex Sans Variable font trio,
per-Tenant/per-Surface enum chrome dispatch) and issued one child BRIEF per consuming
archive with that archive's own audit findings and implementation order.

This parent brief exists to track the initiative as a whole and avoid it counting as
2 separate active items against the soft cap — the technical content lives entirely in
the two children, which are the actual handoff documents each destination archive works
from and should not be collapsed into one file (different sites, different engines,
different findings, different implementation orders — only the design language is shared).

## Scope

- Design direction ownership: project-knowledge (this archive).
- Consumers: project-marketing (home.woodfinegroup.com + home.pointsav.com,
  `app-mediakit-marketing`) and project-software (software.pointsav.com,
  `app-privategit-source` + `app-privategit-marketplace`).
- Both children were sent 2026-06-24 and remain externally blocked — waiting on those
  archives' own Totebox sessions to implement.

## Decisions locked

- Both children stay `status: reference` under this parent — the concrete specs remain
  fully valid and are each destination archive's working reference, not superseded.
- Any *new* Sovereign Editorial design-direction change (new token, new component pattern)
  updates this parent brief plus both children's "Coordination" sections, not a third file.

## Decisions open

- Neither child has confirmed implementation-complete as of 2026-07-09 — status unknown,
  last update from project-software's ACK (msg-id command-20260702-late-ack-sovereign-editorial-handoff-202)
  says project-software's rewrite is planning-only, no code yet. project-marketing has not
  acknowledged.

## Work log

- 2026-07-09: created as parent tracker; demoted both siblings from `active` to `reference`
  with `parent:` pointing here (session-driven BRIEF-consolidation pass).

## Carry-forward

- [x] Check project-marketing and project-software outboxes for implementation status next
  time either archive is touched. **Checked 2026-09-04 — still genuinely unresolved, no
  news since 2026-07-09.** `project-software`'s only mentions are the original 2026-06-24
  handoff (`.agent/inbox-archive.md:1044-1201`); their own reference brief
  (`project-software/.agent/briefs/BRIEF-sovereign-editorial-software.md:214`) still
  describes an unstarted chrome-only patch, dated 2026-07-02. `project-marketing` has zero
  "sovereign editorial" mentions anywhere and no equivalent BRIEF exists in that archive at
  all — still hasn't acknowledged. Re-check next time either archive is touched, per the
  standing rule above; not worth a dedicated follow-up otherwise.
