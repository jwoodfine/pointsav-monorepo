---
schema: cluster-manifest-v1
cluster: project-input
opened: 2026-08-27
state: provisioned
slm_endpoint: http://localhost:9080
module_id: input

tetrad:
  vendor:
    repo: pointsav-monorepo
    branch: cluster/project-input
    focus: [service-input, service-fs]
    status: leg-pending
  customer:
    repo: TBD
    focus: []
    status: leg-pending
  deployment:
    instances: []
    status: leg-pending
  wiki:
    target: TBD
    planned_topics:
      published: []
      staged_for_pickup: []
    status: leg-pending
---

# project-input — Cluster Manifest

## Mission

Owns `service-input` (the "Input Machine" — file ingest, classification,
chart-of-accounts-based naming) and `service-fs` (the WORM commit/storage
layer it writes to). Provisioned 2026-08-27 per operator direction,
consolidating ownership that previously sat with `project-totebox`
(claimed under their "Ring 1" scope in their own CLAUDE.md — see
Ownership transfer below).

Deployed as a dedicated appliance instance built on the `os-totebox`
substrate, separate from the already-running `os-totebox-1`
(project-totebox's own DataGraph/audit-ledger instance).

## Ownership transfer (2026-08-27)

`service-input` and `service-fs` move from `project-totebox`'s Ring 1
scope to `project-input`. project-totebox must be notified — this
archive's own CLAUDE.md still claims both crates as of provisioning
time; that claim needs correcting once project-input's ownership is
confirmed live in `pairings.yaml`/`conventions/we-own-it-principle.md`
(if applicable).

**Not part of this transfer:** `app-console-input` (the os-console-side
counterpart cartridge, same conceptual "Input Machine" role) stays with
whoever currently owns `os-console` (understood to be project-console) —
flagged as a related, not-yet-resolved question, not assumed to move
automatically just because the backend does.

**Live deployment continuity:** `service-input` is not just a scaffold —
it is Active and currently live on `:9106`, serving real batch-migration
work for the `cluster-totebox-jennifer-2` deployment instance (real
ledger at that path, 458+ entries as of provisioning). This transfer is
an ownership/development-authority change, not a runtime change — that
deployment should keep running unaffected; project-input becomes who
makes future development decisions about the crates it runs.

## Prior art found at provisioning (2026-08-27 investigation)

Real design work already exists, scattered rather than centralized:

- **Live ledger format** (`deployments/cluster-totebox-jennifer-2/service-input/ledger.jsonl`):
  content-addressed by `sha256`, append-only, `{sha256, status, stem, ts,
  ledger_valid}` shape. `stem` is a human-readable name; the hash is the
  true permanent identity.
- **`app-console-input/src/audit.rs`** (project-console/project-totebox
  clones): a real, working append-only SQLite `ingest_log` table
  (`created_at, username, tenant, path, ledger_id, status`) — same
  underlying pattern, independently implemented on the console side.
- **Jennifer's own design notes** (`project-totebox/inputs/jw6-scratch/
  {visual,language}/worm-record-format.html`): the core WORM design
  principle — "the file name is the reference, never a database ID...
  content addressed by SHA-256" — explicitly tied to SEC Rule 17a-4 /
  FINRA Rule 4511 retention requirements. Already flags the open
  question this archive needs to answer: "What happens to the SHA-256
  hash when a source file is re-scanned at higher fidelity — does the
  original record stay addressable?"
- **Registry note** (`project-totebox/.agent/rules/project-registry.md`):
  the crate's original "generic-document-parser" design was built on a
  pre-merge `cluster/project-data` lineage that was **never merged** into
  current history — superseded, not missing. Worth checking whether that
  lineage still exists anywhere reachable before designing from scratch.

## Chart of accounts

Needs to be created fresh (operator decision, 2026-08-27) — not
invented unilaterally by an AI session. Real accounting-domain input
required (Jennifer, as the actual bookkeeping operator) before
`service-input`'s classification logic has anything real to name files
against. This is a hard dependency for the naming logic, not for the
infrastructure/ledger plumbing above, which can proceed independently.

## Pilot scope

Starting business-admin folders (operator-selected, 2026-08-27):
`project-documents`, `project-orgcharts`, `project-proforma` — their
*outputs* flow through the Input Machine for classification/naming,
then commit once to the WORM layer. Per-archive access scoping stays as
restrictive as it is today; this is storage consolidation, not a
broadening of who can read what.

## Tetrad

See `tetrad:` block in frontmatter for canonical declaration.
