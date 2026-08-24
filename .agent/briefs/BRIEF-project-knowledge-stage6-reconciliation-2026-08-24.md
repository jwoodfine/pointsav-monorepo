---
artifact: brief
schema: foundry-brief-v1
brief-id: project-knowledge-stage6-reconciliation-2026-08-24
title: project-knowledge — 3-way Stage 6 reconciliation (archive-root, sub-clone, origin-staging-j all diverged)
status: active
owner: project-knowledge
created: 2026-08-24
updated: 2026-08-24
related_briefs: [command-fleet-backlog-2026-08-23]
cites: []
---

# project-knowledge — Stage 6 reconciliation

## Context

The operator asked to get 2 real, valuable commits live — `f3eaf9238` (`/es/category/*`
404 fix) and `c5215b532` (8 sidebar/nav UX-review findings), both on
`cluster/project-knowledge` in the nested `pointsav-monorepo/` sub-clone, both fixing
real bugs in `app-mediakit-knowledge` (a live documentation wiki engine). Running
`bin/self-service-promote.sh` — the sanctioned Totebox-side path to get this live —
surfaced a chain of real, independent problems, each investigated and either fixed or
escalated. This BRIEF picks up where that investigation stopped: a genuine 3-way
history divergence that a Totebox session needs to reconcile properly, not something
Command should push through unilaterally.

**Everything below was investigated read-only or fixed narrowly and safely. No
force-push, no branch rewrite, no destructive operation was attempted at any point.**

## Scope

This is `project-knowledge`'s own reconciliation to run — Command surfaced it and
fixed the parts that were clearly safe and narrow (see "Already fixed" below), but the
actual history reconciliation requires judgment calls about which of three diverged
copies' content is current/correct, which only this archive's own session can make
responsibly.

## The three-way divergence

Three copies of this content currently disagree with each other:

1. **Archive-root repo** (`/srv/foundry/clones/project-knowledge/`, own `.git`) — HEAD
   `4546848ed`. **4,331 commits ahead / 5,176 commits behind** `origin-staging-j`.
   Does **not** have the 2 target commits.
2. **Nested `pointsav-monorepo/` sub-clone** (`/srv/foundry/clones/project-knowledge/
   pointsav-monorepo/`, own `.git`, own `cluster/project-knowledge` branch) — HEAD
   (after Command's 2 fixes below) `d715dd2`-adjacent on this branch. **6,911 commits
   ahead / 5,176 commits behind** `origin-staging-j`. **Has** the 2 target commits.
3. **`origin-staging-j`** (`jwoodfine/pointsav-monorepo` on GitHub, the actual current
   staging mirror) — has **5,176 commits neither local copy has**, including real,
   detailed `app-mediakit-knowledge` fixes (commit subjects matching the same class of
   UX/rendering work the 2 target commits are trying to add) and a commit literally
   titled `reconcile(project-knowledge): graft root's canonical-repair provenance onto
   nested's live-pushed lineage (no-op merge, no content change)` — strong evidence a
   prior session already did real reconciliation work directly against this remote,
   from neither of the two local copies, which never synced back to either.

Also separately confirmed: `cluster/project-knowledge` (the sub-clone) is **6,722
commits behind canonical `origin/main`** (see the companion canonical-reconciliation
note below) — a distinct gap from the staging-mirror divergence above, larger than
project-workplace's already-flagged 3,748-commit gap.

**First thing this session should determine**: which of the three states is actually
authoritative, and how the archive-root and sub-clone repos came to diverge from each
other in the first place — this looks like the same "dual-copy drift" bug class
already documented for this archive elsewhere, but needs its own root-cause pass here,
specifically for the staging-mirror relationship (not just canonical).

## Already fixed (do not redo)

Two real, independent problems were found and fixed while investigating — both
verified safe, both committed, both still correct regardless of how the reconciliation
above gets resolved:

1. **`bin/self-service-promote.sh`'s `DATA_CONTENT_ALLOWLIST` was missing 27 entries**
   that `bin/promote.sh`'s own allowlist already carries, for legitimate source code
   under `service-email-egress/{egress-ingress,egress-ledger,egress-prune,
   egress-roster}/` (Cargo.toml/main.rs/shell scripts — not business-admin content,
   verified via exact-match `comm` diff against `promote.sh`'s allowlist). Synced,
   commit `c22566a` (workspace root `bin/`). This was the actual root cause of the
   original filter block — not new business-admin exposure from this push.
2. **`service-email-egress/egress-roster/ews_payload.xml` had real PII** (Peter
   Woodfine's real email in a SOAP header) on `cluster/project-knowledge`, stale since
   2026-05-04. Canonical `origin/main` already redacted the identical field in commit
   `9f28d2391` (2026-08-17). Reset to canonical's exact redacted value, commit
   `057877470` (sub-clone). Verified: this is the only line that changed.

Neither of the 2 target commits touches this file — both fixes were pure incidental
backlog noise, unrelated to what the operator actually wants live.

**Separately found and flagged, not fixed (fleet-wide, not project-knowledge-
specific):** the same data-content filter exists as 4 independently-maintained
copies — `promote.sh`, `self-service-promote.sh`, `pre-commit-foundry-gate.sh` (a git
hook), and `pre-push-foundry-gate.sh` (another git hook) — each with its own allowlist
that can drift from the others exactly as `self-service-promote.sh`'s did here. Worth
a dedicated Command-side follow-up to share one allowlist source instead of four
copies, but out of scope for unblocking this specific push. Also worth noting: even a
correctly-synced allowlist is purely path-based with no content verification — an
allowlisted path can carry stale/unsafe content (as `ews_payload.xml` just did) and
pass silently. That's a standing design fragility in the shared filter mechanism, not
something introduced or fixed here.

## What was tested and found NOT to work (don't retry these)

- **A narrow cherry-pick of just the 2 target commits onto canonical `origin/main`**
  (tested on a throwaway scratch branch, not the real one) — **5 real conflicts**,
  including 2 modify/delete conflicts (canonical has since deleted files these commits
  modify). This confirms canonical's `app-mediakit-knowledge` has genuinely diverged
  *architecturally* from this archive's version, not just fallen numerically behind —
  see the companion canonical-drift note (`layout.rs` 1,032 lines on canonical vs.
  1,515 here; `app.rs` 809 vs. 1,484). A clean mechanical cherry-pick isn't available;
  resolving this needs the same "diff-then-decide per conflict" discipline already
  used successfully for other archives' Stage 6 backlogs this cycle, not a blind
  merge.
- **A plain `self-service-promote.sh` push after the two fixes above** — cleared both
  content gates cleanly (data-content filter and a secret-pattern false positive in
  `service-slm/crates/slm-doorman/src/redact.rs`'s own test fixtures, both
  operator-confirmed bypasses, logged in Command's inbox), then failed at the actual
  `git push` step: non-fast-forward rejection on both `origin-staging-j` and
  `origin-staging-p`, per the 3-way divergence above. Not a gate problem — a real
  history divergence. Force-pushing was considered and explicitly rejected as unsafe
  given the remote holds real, apparently-unrecovered work.

## Decisions open

1. **Which copy is authoritative?** Neither local copy (archive-root, sub-clone)
   matches `origin-staging-j`'s current tip, and archive-root doesn't even have the 2
   target commits. Does the remote's real `app-mediakit-knowledge` work need to be
   pulled into one of the local copies first, or does one of the local copies actually
   supersede the remote and this needs a deliberate, reviewed overwrite? The
   `reconcile(project-knowledge): graft root's canonical-repair provenance...` commit
   on the remote is worth reading in full first — it may describe exactly this
   situation from a prior session's perspective.
2. **How to carry the 2 target commits (and any other real un-landed local work)
   through the reconciliation** without losing anything on any of the three sides.
3. **Root cause**: why do the archive-root repo and the nested sub-clone keep
   diverging from each other and from the remote? Is this one-time drift or an
   ongoing structural problem (e.g. two different sessions/scripts writing to
   different copies without cross-syncing)? Fixing the cause matters as much as
   fixing the current state, or this recurs.
4. **Sequencing vs. the separate 6,722-commit canonical gap** — should staging-mirror
   reconciliation happen first (smaller, more tractable), with canonical
   reconciliation as a distinct follow-on, or should both be tackled together since
   they likely share root-cause context?

## Related

- Command inbox thread: `command-20260824-self-service-promote-sh-blocked-on-clust`
  (the original block + full investigation trail) and
  `command-20260824-stage-6-reconciliation-needed-app-mediak` (the canonical
  line-count drift finding for `app-mediakit-knowledge` specifically).
- `BRIEF-command-fleet-backlog-2026-08-23.md`'s project-knowledge Carry-forward entry
  (Command-side summary of the same investigation).
- Precedent for this kind of reconciliation:
  `clones/project-totebox/.agent/briefs/BRIEF-project-totebox-moonshot-sel4-vmm-reconciliation-2026-08-22.md`
  and this cycle's project-workplace (#11), project-software, project-editorial
  Stage 6 reconciliation notes in `/srv/foundry/NEXT.md` — same discipline
  (cherry-pick-onto-scratch + diff-then-decide per conflict, never a blind merge)
  applies here.

## Carry-forward

- [ ] Determine authoritative source among the 3 diverged copies (Decision 1).
- [ ] Reconcile histories, carrying the 2 target commits + any other real local work
      through cleanly.
- [ ] Re-run `self-service-promote.sh` once local matches (or deliberately supersedes)
      `origin-staging-j`'s actual tip.
- [ ] Separately address the 6,722-commit canonical gap (own item, may be sequenced
      after staging reconciliation).
- [ ] Investigate and fix the root cause of the archive-root/sub-clone/remote
      3-way drift so it doesn't recur.
