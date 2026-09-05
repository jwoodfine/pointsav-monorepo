---
schema: foundry-cluster-manifest-v1
cluster: project-orchestration
cluster_name: project-orchestration
cluster_branch: cluster/project-orchestration
created: 2026-05-08
state: active
doctrine_version: 0.1.x
doctrine_claims_codified: [37, 43, 44, 52]
doctrine_claims_proposed: []

operator: woodfine (Mathew)
working_pattern: production-first-mvp
input_shape: totebox-orchestration-transition-phases-1-3

slm_endpoint: http://localhost:9080
module_id: orchestration

software_footprint:
  target_os: os-orchestration   # planned; ~/Foundry itself will run on this OS (P1.4/P1.5)
  monorepo: pointsav-monorepo   # archive-root .git only ever held .agent/ governance content (orphan branch, no merge-base with origin/main, confirmed 2026-09-04) — this field describes the .agent/ tracking home, not where owned source lives
  sub_clone: pointsav-orchestration-private   # added 2026-09-04; nested sub-clone at ./pointsav-orchestration-private/ (own .git, gitignored), same multi-clone pattern as project-bim's ./pointsav-monorepo/. Extracted 2026-09-01 (security purge, NOTAM 2026-09-01) from the now-purged pointsav-monorepo public repo.
  branch: cluster/project-orchestration
  owns:
    - app-orchestration-command/  # v0.0.1 shipped 2026-06-29; confirmed in canonical origin/main (29d0b4a1) before the 2026-09-01 relocation. v0.0.2 (pairing.rs WORM ledger schema_version + write-through) confirmed PRESENT in pointsav-orchestration-private, re-verified 2026-09-04 (schema_version "1"/"2" pairing_created/pairing_revoked logic live in pairing.rs). peer_type (2026-07-17 BRIEF decision) implemented same session, commit 6a6f6f7.
    - app-orchestration-slm/   # KNOWN STALE in pointsav-orchestration-private as of 2026-09-04: project-totebox's own residual copy has real newer work not in the private repo (fleet.rs, license.rs, membership.rs, yoyo_proxy.rs, orchestration-slm-server/{http,main}.rs, plus build-microkit-image.sh/deploy-loader-img.sh/qmp-shutdown.py/systemd/ entirely absent) — physical relocation ratified 2026-07-16 but reconciliation with project-totebox's active development not yet done; see BRIEF-orchestration-totebox-integration.md carry-forward
    - os-interface/
    - os-orchestration/   # also diverged from project-totebox's copy (Cargo.toml, src/lib.rs differ) — same reconciliation gap as app-orchestration-slm above
    - app-orchestration-graph/   # ownership formalized 2026-09-05 (Command confirmed no objection, msg-id command-20260802-closing-out-3-long-open-coordination-ite); fork with project-totebox's copy confirmed already resolved 2026-09-04 (byte-identical, correct LicenseRef-PointSav-ARR license). Real, tested code (Ed25519 fan-out signing in capability.rs) — DataGraph federation design signed off 2026-09-04, conditional on the pairing-bypass fix (see BRIEF-orchestration-totebox-integration.md Decisions locked)

# Cluster mission:
# Implement the Totebox Orchestration transition — Phases 1, 2, and 3.
#
# Phase 1 (vocabulary): update CLAUDE.md §11, AGENT.md, bin/claude-role.sh to
#   use Command/Totebox vocabulary; add MANIFEST.md "As a Totebox Orchestration"
#   section; correct user-guide article on NetworkAdminOS/MBA.
#
# Phase 2 (formalize): create pairings.yaml; add slm_endpoint to all 13 cluster
#   manifests; create slm/ dirs; provision project-source + project-woodfine archives.
#
# Phase 3 (instrument): write bin/open-archive.sh, bin/list-archives.sh;
#   scaffold app-orchestration-command v0.0.1 in this cluster's pointsav-monorepo.

tetrad:
  vendor:
    - source_repo: pointsav-orchestration-private   # corrected 2026-09-04 (was pointsav-monorepo, stale since the 2026-09-01 relocation — see software_footprint.sub_clone above for the actual nested-clone path)
      project_path: app-orchestration-command/, app-orchestration-slm/, os-interface/, os-orchestration/, app-orchestration-graph/
      status: app-orchestration-command v0.0.1 (2026-06-29) + v0.0.2 WORM ledger (schema_version "1"/"2") both confirmed present and current in pointsav-orchestration-private (re-verified 2026-09-04); peer_type implemented same session (commit 6a6f6f7). app-orchestration-graph fork also confirmed resolved — byte-identical to project-totebox's copy, correct license, closed as of 2026-09-04 (see Decisions locked). app-orchestration-slm and os-orchestration confirmed STALE relative to project-totebox's own residual copies (diffed directly 2026-09-04) — real reconciliation work needed before pointsav-orchestration-private can be treated as fully authoritative for those two; not yet scheduled. Stage 6 promotion path for this sub-clone does not exist yet (no staging forks provisioned for the private repo) — flagged to Command, not blocking.
  customer:
    - fleet_deployment_repo: woodfine-fleet-deployment
      catalog_subfolder: gateway-orchestration-command/
      status: leg-pending — v0.0.1 has shipped; catalog_subfolder not yet created, guides not yet authored
  deployment:
    - instance_name: orchestration-command-1
      instance_path: ~/Foundry/deployments/gateway-orchestration-command-1/
      status: live — provisioned 2026-06-29; systemd unit local-orchestration-command active on port 8020; binary ledger confirms smoke_test pass (data/binary-ledger/orchestration-command-server.jsonl)
  wiki:
    - target_repo: media-knowledge-documentation   # renamed from content-wiki-documentation (DOCTRINE.md §IV.e, ratified 2026-05-21)
      articles:
        - architecture/totebox-orchestration-development.md
        - architecture/pairing-as-permission.md
        - systems/os-orchestration.md
        - architecture/totebox-session.md
        - architecture/personnel-permissions.md
      status: leg-pending — 2 of 5 drafts staged at ~/Foundry/.agent/drafts-outbound/ (topic-totebox-orchestration-development.draft.md, topic-os-orchestration.draft.md, last updated 2026-06-29); remaining 3 articles not yet drafted; none yet landed in media-knowledge-documentation
