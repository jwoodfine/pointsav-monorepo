---
artifact: brief
schema: foundry-brief-v1
brief-id: project-knowledge-footer-ci-check
title: Footer-validation CI check — build-time footer/trademark drift detection
status: active
owner: project-knowledge
parent: project-knowledge-ng-rewrite
created: 2026-09-08
updated: 2026-09-08
---

# BRIEF — Footer-validation CI check

## Context

project-editorial asked project-knowledge to own scoping and building this
(`project-editorial-20260908-footer-governance-ci-check-build-status-`), as
part of Decision #5 in their `BRIEF-footer-badge-token-architecture.md`. We
accepted (`command-20260908-re-footer-governance-ci-check-build-stat`).

Design, per that BRIEF: `app-mediakit-shell` loads Layer-1 footer/trademark
copy from YAML at build time and renders it, so wiki articles stop embedding
footer text in body markdown — the reason an earlier trademark changeover
needed to `sed` ~75 files. The missing piece is a CI check that fails the
build if a site's rendered footer diverges from the token source.

Fits our existing build-time structural check pattern (`wiki_config_check`/
`membership_drift`-equivalent territory); we're one of `app-mediakit-shell`'s
two current consumers alongside project-marketing.

## Status

**Not yet scoped.** Queued behind Phase 2 of `BRIEF-knowledge-ng-rewrite.md`
(local os-mediakit appliance re-test). No code written, no design decisions
made yet beyond accepting ownership.

## Next steps

1. Read `project-editorial`'s `BRIEF-footer-badge-token-architecture.md`
   Decision #5 in full for the exact intended check shape (what "diverges"
   means — full text diff, structural/field diff, hash comparison, etc.).
2. Determine where this check should live: our own crate's test suite (like
   `wiki_config_check`), a shared `app-mediakit-shell` CI job, or a top-level
   workspace CI step — depends on whether project-marketing's consumption
   needs the same check independently or can share one gate.
3. Scope out, propose to editorial + project-marketing before building.

## Carry-forward

- Update project-editorial once scoping actually starts (told them we would).
