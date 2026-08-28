---
schema: cluster-manifest-v1
cluster: project-construction
opened: 2026-08-27
state: provisioned
slm_endpoint: http://localhost:9080
module_id: construction

tetrad:
  vendor:
    repo: pointsav-monorepo
    branch: cluster/project-construction
    focus: []
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

# project-construction — Cluster Manifest

## Mission

New sellable construction-industry software product for
software.pointsav.com — scope and crate name not yet decided. Provisioned
2026-08-27 per operator direction, kept as its own standalone archive
rather than nested inside `project-bim`, though future coordination with
`project-bim` (BIM/architecture substrate) is expected.

No code exists yet. This archive starts in the same state
`project-newsroom` did at provisioning: a real pointsav-monorepo clone
with no crate added, tetrad fully leg-pending except the vendor repo
itself.

## Tetrad

See `tetrad:` block in frontmatter for canonical declaration. Update as
each leg resolves — do not leave a leg silently undocumented once real
work starts on it.
