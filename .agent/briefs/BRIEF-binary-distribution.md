---
artifact: brief
schema: foundry-brief-v1
status: active
brief-id: project-knowledge-binary-distribution
owner: project-knowledge
destination: project-software
created: 2026-06-30
updated: 2026-07-13
parent: project-knowledge-ng-rewrite
---

# BRIEF — Distribution: app-mediakit-knowledge + os-mediakit VM on software.pointsav.com

> **For:** project-software Totebox session  
> **Site:** software.pointsav.com  
> **Engine:** `app-privategit-marketplace` (port 9202) — product catalog + license issuance  
> **Design authority:** project-knowledge (binary producer)

---

## Context

`app-mediakit-knowledge` is a production-quality, self-hosted wiki server (Rust/axum/maud)
serving the three live PointSav knowledge sites. Phase 6 (Visual Excellence) is complete.
The binary and a full VM image are ready for external distribution.

The goal for small and medium businesses: **launch one VM image → wiki is running**. No
additional infrastructure setup. Download the image, start it with QEMU or import to a
cloud provider, and the wiki server is live on port 9090.

**BETA status across all distribution formats.** No charge while in BETA. Pricing and the
payment widget must NOT be enabled on any listing until an explicit "enable pricing" outbox
from project-knowledge arrives.

---

## Architecture background (important for positioning)

The `systems/os-mediakit.md` TOPIC defines `os-mediakit` in phases:

**Phase 1 (operational today):** `vm-mediakit` is an **Ubuntu 24.04 QCOW2 guest VM** already
running on foundry-prod under QEMU/TCG. It runs `app-mediakit-knowledge` and other MediaKit
services as standard Linux systemd services. This is the basis for the distributable BETA image.

**Phase 3 (planned/intended):** seL4 Microkit 2.2 AArch64 image assembled by `moonshot-toolkit`.
Each service becomes a provably-isolated Protection Domain. AArch64 only (Microkit 2.2 does not
target x86_64). Shim crate `system-substrate-sel4` is planned but not yet built.
**BCSC posture: all seL4/Phase 3 claims must use planned/intended/target language only.**

**Production architecture vs. standalone SMB distribution:** In production, `os-mediakit` is
a guest that runs on `os-infrastructure`. The standalone SMB image is a **self-contained QCOW2**
— it does not require `os-infrastructure` underneath. This is a different (simpler) artifact:
a minimal Ubuntu 24.04 image with `app-mediakit-knowledge` pre-installed and a systemd unit,
packaged for direct customer use. The production layered architecture and the customer
distribution format are both correct — they serve different audiences.

---

## Decisions locked

1. **BETA only — no charge.** `price_cents: 0`, `payment_required: false`, `beta: true` on ALL
   listings. Do not connect payment flow until an explicit future outbox enables it.
2. **Two download formats (listed together on one product page):**
   - **Format A — bare binary** (available now): single Linux x86_64 binary; curl + chmod + run
   - **Format B — standalone VM image** (pending build, near-term): self-contained Ubuntu 24.04
     QCOW2; customer runs QEMU or imports to GCP/AWS; wiki is live on first boot
3. **seL4 is Phase 3 / planned.** Do not describe Phase 3 as current. Use "planned/intended"
   language. The seL4 security story is correct positioning once it ships.
4. **SMB standalone = self-contained QCOW2.** The image does not require os-infrastructure as
   a prerequisite. It includes everything needed (OS, binary, systemd unit, default content dir).
5. **Linux x86_64 only for BETA.** No macOS, Windows, or AArch64 builds until pipelines exist.
6. **Binary source is canonical.** project-knowledge builds and delivers binaries/images via
   outbox. project-software uploads; does not build.

---

## Product listing specification

**Product name (EN):** PointSav Knowledge Wiki  
**Product name (ES):** Wiki de Conocimiento PointSav  
**Product slug:** `app-mediakit-knowledge`  
**Category:** MediaKit surface  
**Status badge:** BETA  
**Price:** Free (BETA) — $0.00 / no payment required  
**License:** Apache 2.0  

**Short description (EN, ≤120 chars):**
> Self-hosted Wikipedia-pattern wiki server. Single binary or standalone VM. Git-versioned markdown. WCAG 2.1 AA.

**Short description (ES, ≤120 chars):**
> Servidor wiki de patrón Wikipedia. Binario único o VM lista para usar. Markdown versionado con Git. WCAG 2.1 AA.

**Feature bullets:**
- Standalone VM image — download, launch, wiki is running (no other setup)
- Single binary option — no runtime dependencies (no Node.js, no Python, no database)
- Multi-tenant — serve multiple wiki instances from one process
- Git-versioned markdown as canonical content (not a database)
- WCAG 2.1 AA compliant, bilingual EN/ES routing built in
- Full-text search (Tantivy BM25), JSON-LD, Atom feed, sitemap, REST API + MCP
- Inline annotations (Notes tab) with YAML sidecar storage
- SOC3-ready audit trail: every article change is a git commit
- Planned: seL4 microkernel isolation for provable security between components (Phase 3)

---

## Download page content

### Format A — Bare binary (Linux x86_64)

**Available now.** Must be the first option on the download page.

```bash
# Download
curl -L https://software.pointsav.com/download/app-mediakit-knowledge/latest/linux-x86_64 \
  -o app-mediakit-knowledge
chmod +x app-mediakit-knowledge

# Run (point at a directory of markdown files)
./app-mediakit-knowledge serve --content-dir /path/to/your/wiki --port 9090

# Verify
curl http://127.0.0.1:9090/healthz   # → ok
```

**SHA-256 (S136 build, 2026-06-30T00:28:44Z, source commit `210548b2`):**
`04e54f57e26f2a15eb8e31235fc2bbd11236f7914e605de7171baa90b4e165a7`

Version string: `BETA` for initial listing.

### Format B — Standalone VM image (QCOW2, Linux x86_64)

**Pending build (near-term, same BETA release).** Second option on the download page.
Label: "VM image (QCOW2) — recommended for production self-hosting"

```bash
# Download the VM image
curl -L https://software.pointsav.com/download/app-mediakit-knowledge/latest/linux-x86_64-qcow2 \
  -o os-mediakit-BETA.qcow2

# Run with QEMU (forwards port 9090)
qemu-system-x86_64 \
  -hda os-mediakit-BETA.qcow2 \
  -m 1G \
  -net user,hostfwd=tcp::9090-:9090 \
  -nographic

# OR import to GCP
gcloud compute images create os-mediakit-BETA \
  --source-uri=gs://<bucket>/os-mediakit-BETA.qcow2 \
  --guest-os-features=VIRTIO_SCSI_MULTIQUEUE
gcloud compute instances create wiki-1 \
  --image=os-mediakit-BETA --machine-type=e2-small

# Verify (once running)
curl http://localhost:9090/healthz   # → ok
```

---

## What needs to be built for Format B

The operational `vm-mediakit` Ubuntu 24.04 QCOW2 running on foundry-prod is the **reference implementation**. The distributable image is a cleaned, customer-ready variant:

| Step | Owner | Status |
|---|---|---|
| `os-mediakit/scripts/build-image.sh` — produces self-contained Ubuntu 24.04 QCOW2 | project-knowledge (next session) | ✅ Written 2026-06-30 (commit pending Stage 6) |
| `app-mediakit-knowledge` systemd unit in the image | project-knowledge | ✅ Included in build-image.sh (wiki-{documentation,projects,corporate}.service) |
| Default content directory (sample wiki articles) baked in | project-knowledge | ✅ Getting Started article + content dirs in build-image.sh |
| First-boot `cloud-init` config (set hostname, generate SSH keys) | project-knowledge | ✅ cloud-init disabled (pre-configured image); hostname set in script |
| Upload to software.pointsav.com download endpoint | project-software (after build) | ✅ **LIVE 2026-07-01** — Command confirmed QCOW2 0.1.0 deposited + Format B listing live |
| `GUIDE-installing-os-mediakit.md` (three install paths: QEMU, GCP, AWS) | project-knowledge editorial | ✅ Draft staged → project-editorial outbox 2026-06-30 |
| Update `topic-os-products-distribution-model.md` to include os-mediakit as a listed product | project-editorial | ✅ Update draft staged → project-editorial outbox 2026-06-30 |

**Pattern reference:** `os-totebox/scripts/build-image.sh` (NetBSD QCOW2 builder) and
`GUIDE-installing-os-infrastructure.md` (three install paths). Replicate the pattern.

---

## seL4 positioning (Phase 3 — planned/intended)

When the seL4 Phase 3 image ships, the product page gains a third download option:
"seL4 AArch64 system image — provable isolation between components."

The seL4 security story: each service (wiki, network, storage) runs in a formally-verified
isolated Protection Domain. A compromise of the wiki process cannot reach the network stack
or other services. seL4 formal verification scope: AArch64 EL2 + RISC-V64 (confirmed, per
`topic-sel4-capability-topology.md`). Microkit 2.2 targets AArch64, not x86_64.

**BCSC-correct product page language for seL4:**
> "Planned: seL4 microkernel isolation for AArch64 deployments — each component runs in an
> isolated Protection Domain with formally-verified memory capabilities."

Do NOT say "runs on seL4" or "seL4-certified" for Phase 1 BETA. The current QCOW2 runs Linux.

---

## Resolved (condensed 2026-07-13 — full detail in git history)

- **2026-07-01:** Format A (bare binary) + Format B (QCOW2 0.1.0) both live on
  software.pointsav.com, BETA/no charge. Format B artifact:
  `pointsav-monorepo/os-mediakit/scripts/build/os-mediakit.qcow2`. SHAs and Command's
  live-URL confirmation are in git history for this file if ever needed again.
- **2026-07-09:** the ng-rewrite's binary (P8 cutover, canonical `531d3144`) is confirmed
  live behind all 3 wiki systemd services. Command already sent project-software the
  catalog-listing handoff for this binary (msg-id
  `command-20260708-2-new-catalog-entries-requested-orchestr`) — **no further action
  needed from project-knowledge**; project-software owns landing it, Command owns
  verifying it goes live. Full trace: `BRIEF-knowledge-ng-rewrite.md`'s 2026-07-09
  status update.

---

## Open questions

- **Download endpoint:** which route in `app-privategit-source` (port 9201) accepts a QCOW2
  release? Does it need a separate product record (os-mediakit) or does it extend the existing
  `app-mediakit-knowledge` binary listing? Confirm with project-software.
- **Image size:** the operational vm-mediakit image is Ubuntu 24.04 — full install is ~4 GB.
  A stripped minimal image targeting ~1 GB is preferred for distribution. Confirm target size.
- **Sample content:** should the QCOW2 ship with sample wiki articles, or an empty content dir
  with a "Getting Started" article only? Lean toward minimal: one Getting Started article.
- **seL4 Phase 3 timeline:** out of scope for this BRIEF; tracked in `project-system`.
