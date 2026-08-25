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

1. **RESOLVED 2026-08-24 — which copy is authoritative?** Read the graft commit in
   full (`ac6be54d59`, 2026-08-13): it already answers this from a prior session's
   perspective — "nested's tree is authoritative and unchanged by this commit...
   nested is the confirmed real pusher (tip matches both origin-staging-j and
   origin-staging-p exactly)... root stalled at 90f5c3611 (2026-08-09) and was
   abandoned." Independently confirmed this still holds: `manifest.md`'s declared
   `clones:` list has only ever named `pointsav-monorepo/app-mediakit-knowledge/`
   (the nested sub-clone) — never a top-level archive-root copy, at any point. The
   nested sub-clone is authoritative; archive-root's copy is not a live parallel
   development to reconcile with.
2. **DONE 2026-08-24 — carried the 2 target commits through.** Merged
   `origin-staging-j`'s tip into the nested sub-clone (commit `4f62c535a8`), resolving
   all 22 real content conflicts individually (10 files "ours" — the 2 target commits'
   own work plus an already-redacted PII file; 12 files "theirs" — a real 2026-08-19
   GitHub-exposure remediation on the remote that local was missing). Pushed and
   **verified** on both `origin-staging-j`/`origin-staging-p` (`git fetch` +
   `git branch -r --contains`, not just trusting the push output). This closes the
   "5,176 commits neither local copy has" gap that motivated this Decision — nested
   now contains all of origin-staging-j's content, merged in.
3. **RESOLVED 2026-08-24 — root cause found, not an ongoing structural problem.**
   `git log --reverse -- app-mediakit-knowledge/` on the archive-root repo traces its
   earliest commit to **2026-02-28** ("Sovereign Update"/"DOCS: Structural Anchoring"),
   predating this workspace's 2026-04-21 sovereign-sync retirement (CLAUDE.md §4:
   "`sovereign_sync.sh` deleted. All 7 engineering repos bootstrapped on staging tier
   (2026-04-21)"). Archive-root's `app-mediakit-knowledge/` is dead legacy content
   from the retired pre-bootstrap sync mechanism, frozen since before the current
   nested-sub-clone architecture existed — not two live processes actively diverging.
   Nothing to fix going forward; this is historical drift, not a recurring pattern.
   Whether it's worth deleting from archive-root as cleanup is a separate, much
   lower-stakes question — not gating anything above.
4. **RESOLVED BY SEQUENCING 2026-08-24 — staging first, canonical separately, as this
   item anticipated.** Staging-mirror reconciliation (Decision 2) is done and verified.
   Canonical reconciliation was picked up narrowly rather than via the archive-root/
   sub-clone/remote 3-way lens at all: since archive-root is confirmed non-authoritative
   dead content (Decision 3), the canonical gap only concerns nested vs. `origin/main`.
   Built `reconcile-canonical-app-mediakit-knowledge-2026-08-24` (commit `e5fc54aa62`)
   directly off `origin/main`'s current tip, replacing only `app-mediakit-knowledge/` +
   the workspace `Cargo.lock` — none of the other 394 files in the 428-file tree diff
   (other archives' real work) touched. **This branch's premise still holds** — Command
   asked this be re-checked given the full 3-way picture; it does, because the branch
   never depended on archive-root or on origin-staging-j's pre-merge state, only on
   nested's current (now-reconciled) content. `cargo build`/`test` verified clean
   against canonical's real workspace state. Local only, not pushed — Command applies
   from this same filesystem path.

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

- [x] Determine authoritative source among the 3 diverged copies (Decision 1) —
      nested sub-clone; archive-root is dead legacy content, not a competing source.
- [x] Reconcile histories, carrying the 2 target commits + any other real local work
      through cleanly — merge `4f62c535a8`, verified on both staging mirrors.
- [x] Re-run `self-service-promote.sh` once local matches (or deliberately supersedes)
      `origin-staging-j`'s actual tip — done, succeeded cleanly (also needed 2 separate
      gate fixes from Command along the way: `bin/lib/data-content-filter.sh`'s
      `DATA_CONTENT_ALLOWLIST`/`SECRET_PATTERN_ALLOWLIST` gaps, commits `3694ce1`/
      `38cba20`).
- [x] Separately address the 6,722/11,230-commit canonical gap — scoped narrowly
      instead of a full branch reconciliation: `reconcile-canonical-app-mediakit-
      knowledge-2026-08-24` (commit `e5fc54aa62`), touching only this crate. Ready for
      Command to apply; not pushed anywhere (no canonical push credentials from this
      session).
- [x] Investigate the root cause of the archive-root/sub-clone/remote 3-way drift —
      not an ongoing problem; archive-root's copy is frozen pre-2026-04-21 sovereign-
      sync-era content, never part of the current nested-sub-clone architecture.
      Nothing to fix going forward. Optional lower-priority cleanup: whether to
      delete/archive the stale copy from archive-root's repo — not done here, not
      blocking anything.

**All 4 Decisions-open items resolved 2026-08-24** — see the rewritten Decisions-open
section above for full detail on each. Nothing further pending from this BRIEF; the
canonical-reconciliation branch and its application are Command's next step.
