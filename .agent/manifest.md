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
  monorepo: pointsav-monorepo
  branch: cluster/project-orchestration
  owns:
    - app-orchestration-command/  # v0.0.1 shipped 2026-06-29; confirmed in canonical origin/main (29d0b4a1); v0.0.2 (pairing.rs WORM ledger) pending Stage 6 canonical merge

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
    - source_repo: pointsav-monorepo
      project_path: app-orchestration-command/
      status: v0.0.1 shipped 2026-06-29 (3-crate workspace, 7 tests passing, Axum 0.8 server); confirmed in canonical origin/main (commit 29d0b4a1). v0.0.2 (pairing.rs WORM ledger schema_version + write-through) pushed to promote-queue 2026-07-09, awaiting Command Session canonical merge.
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
