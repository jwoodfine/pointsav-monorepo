---
artifact: brief
schema: foundry-brief-v1
brief-id: project-input-cross-compliance-worm-design
title: Cross-Compliance WORM Design — service-fs / service-input
status: active
owner: project-input
created: 2026-08-28
updated: 2026-08-28
---

# Cross-Compliance WORM Design — service-fs / service-input

## Context

`project-input` was provisioned 2026-08-27, consolidating ownership of
`service-input` (the "Input Machine" — file ingest, classification,
chart-of-accounts-based naming) and `service-fs` (the WORM commit/storage
layer it writes to) from `project-totebox`'s prior "Ring 1" scope. Both
are deployed as (or planned for) a dedicated `os-totebox`-style appliance
instance, separate from the already-running `os-totebox-1`.

The operator's original ask was narrower — consolidate scattered
business-admin files (`project-documents`, `project-orgcharts`,
`project-proforma`) into one place with proper chart-of-accounts naming.
That naming question surfaced a real architectural tension worth
resolving properly before building: **is WORM (write-once-read-many,
permanent immutability) actually the right default**, given the eventual
goal of selling this appliance into the financial industry, US federal
government, and EU markets — three regimes with real, and in one case
apparently-conflicting, requirements?

This BRIEF documents a broad-survey research pass (2026-08-27/28,
3 parallel research passes, real web research with primary-source
verification where possible) across all three regimes, and the design
implications.

## Decisions locked

**None yet.** This BRIEF captures research findings and a recommended
direction; it does not yet reflect a ratified architecture. Treat
everything below as informing a decision, not as already decided.

## Research findings (2026-08-27/28)

### 1. US financial industry (SEC 17a-4, FINRA 4511, Reg S-P)

- **SEC Rule 17a-4 applies only to registered broker-dealers, security-based
  swap dealers, and MSBSPs** — not automatically to this business. Confirming
  actual registration status is a real prerequisite before treating any of
  this as binding rather than aspirational-for-future-customers.
- **Pure WORM is no longer the only compliant path.** The 2022 amendments
  added an "audit-trail alternative" — a firm may use a system that permits
  modification/deletion, provided it preserves enough audit trail to
  recreate the original record. This was a genuine surprise relative to
  this workspace's prior assumption (Jennifer's own notes cited 17a-4 as a
  WORM justification) — the actual rule is more flexible than that.
- **FINRA 4511**: 6-year default retention (shorter for some categories,
  e.g. most business communications = 3 years); incorporates 17a-4's
  format rules rather than adding separate ones.
- **Reg S-P (2024 amendments)**: 30-day breach notification requirement,
  written incident-response + disposal policies, if any customer PII is
  involved. Compliance deadlines Dec 2025 (large entities) / June 2026
  (smaller entities).
- **Flagged, not yet researched**: CFTC Rule 1.31 — another US WORM-storage
  mandate (commodity trading records) that surfaced adjacent to 17a-4 but
  wasn't covered in this pass.
- Sources: sec.gov (FAQ + amendments pages, Reg S-P press release),
  finra.org/rules-guidance/rulebooks/finra-rules/4511.

### 2. US federal government (FedRAMP, NIST 800-53, NARA, FISMA)

- **FedRAMP likely does not apply to a self-hosted appliance model.**
  FedRAMP covers vendor-hosted cloud services under a shared-responsibility
  model; software an agency installs and runs on its own infrastructure is
  explicitly outside FedRAMP's 2026 scope guidance. Real cost data if it
  *did* apply: $800K–$2M over 18–24 months for a first-time vendor under
  the traditional path (a newer "FedRAMP 20x" automation-first path opened
  Aug 2026, could be $100K–$300K, but unproven at scale) — not a near-term
  target regardless.
- **The applicable path instead: FISMA/NIST 800-53, via each agency's own
  Authorization to Operate (ATO) process.** Since the agency runs the
  appliance itself, compliance burden shifts to their own authorization —
  slower per-deal, no vendor pre-authorization cost.
- **NARA is real, concrete, and directly relevant** — genuinely requires
  archiving solutions "able to capture immutable records" for permanent
  electronic record transfer. NARA Bulletin 2014-04 (acceptable file
  formats, 36 CFR Parts 1235/1236) and Bulletin 2015-04 (minimum required
  metadata) are concrete, checkable specs — **not yet read in full; reading
  the actual format/metadata tables is a real next step** before finalizing
  `service-input`'s metadata schema.
- **Realistic near-term bar**: target NIST 800-53 control alignment
  (families AU/audit, SC/system-integrity, MP/media-protection) + NARA
  format/metadata compliance. Skip chasing FedRAMP entirely for now.
- Sources: fedramp.gov/2026/scope, archives.gov (NARA bulletins, not yet
  opened in full), secondary sources (Schellman/Secureframe/Anchore) for
  FISMA/800-53 relationship — primary FISMA/800-53 text not directly
  pulled, flagged as lower-confidence than the FedRAMP-scope finding.

### 3. EU (GDPR, MiFID II, eIDAS, data residency)

- **The core conflict is resolved, not fundamental.** GDPR Article 17(3)(b)
  exempts erasure where "necessary for compliance with a legal obligation."
  Real precedent: MiFID II's 5-year retention requirement operates as
  *lex specialis* — it overrides an erasure request filed within the
  retention window, not the reverse.
- **Crypto-shredding is the concrete resolution pattern** for erasure
  requests on data NOT covered by a retention exemption: encrypt
  personal-data fields with a per-subject key; on a valid erasure request,
  destroy only the key. WORM ciphertext is never touched — full
  immutability intact — but plaintext becomes permanently unrecoverable.
  Real, deployed pattern (multiple independent implementer sources), not
  regulator-endorsed by name (normal for compliance engineering — the
  legal requirement is what's authoritative, not a specific technical
  pattern). Caveat: key-isolation is the entire security property; a
  leaked/backed-up key defeats shredding.
- **eIDAS**: relevant only if a customer needs qualified electronic
  timestamping/non-repudiation of record creation time. Low priority —
  revisit only if a specific customer needs it.
- **Data residency**: GDPR does NOT mandate EU/EEA-only storage — Chapter V
  governs cross-border *transfers*, not storage location. A self-hosted,
  on-premises appliance run on the customer's own EU soil mostly sidesteps
  this entirely (no transfer occurs if data never leaves the customer's own
  infrastructure) — a genuine structural advantage of the sovereign-
  appliance model over a vendor-hosted cloud service.
- Sources: gdpr-info.eu (Article 17 primary text), Lexology + SteelEye
  (MiFID II/GDPR precedence legal analysis), VeritasChain/Granit/Conduktor
  (crypto-shredding pattern, secondary/practitioner sources), EDPB
  (data-transfer guidance, primary).

## Architectural implication — the "one unified floor" question, revisited

The operator initially chose "one maximally-strict unified floor," then
reconsidered toward "modular may be the only way to achieve cross-
compliance" once the GDPR/WORM tension surfaced. The research suggests a
third position, more precise than either:

**A single core storage mechanism — content-addressed, immutable
ciphertext blobs, with per-record/per-subject encryption keys held in a
separate, destroyable key store — can plausibly satisfy all three regimes
simultaneously**, because:
- US financial: immutable ciphertext trivially satisfies even the stricter
  pure-WORM reading of 17a-4, and easily clears the more lenient
  audit-trail-alternative reading too.
- US federal/NARA: immutable records requirement satisfied directly.
- EU/GDPR: erasure satisfied via key destruction (crypto-shredding),
  without ever touching the WORM layer itself.

What remains genuinely **modular/per-deployment** is not the storage
mechanism but the *policy layer* on top of it: which retention period
applies, whether/when key-destruction requests get honored (a legal-
obligation exemption may mean "not yet, this record is still under a
retention mandate"), which metadata schema/format standard a given
deployment's records must conform to (NARA's for a federal deployment,
whatever a financial customer's own internal standard is, etc.).

This reframes "modular vs. unified" from an either/or into: **one unified
mechanism, with a modular, per-deployment policy configuration on top.**
Worth ratifying explicitly with the operator before locking in as a
design decision — this BRIEF proposes it, does not decide it.

## Decisions open

1. Ratify (or reject) the "one mechanism + modular policy layer" framing
   above as the actual design direction.
2. Confirm Woodfine/PointSav's actual registration status (broker-dealer?
   investment adviser? neither?) — determines whether SEC 17a-4/FINRA/
   Reg S-P are binding-now requirements or aspirational-for-future-
   customers requirements. Do not build as if they're binding without
   this confirmed.
3. Read NARA Bulletin 2014-04's actual format table and Bulletin 2015-04's
   actual metadata table in full — `service-input`'s metadata schema
   should be designed against these directly, not from this summary alone.
4. Research CFTC Rule 1.31 (flagged, not yet done) if commodity-trading-
   adjacent records are ever in scope.
5. Decide the chart-of-accounts naming question this BRIEF grew out of:
   confirmed (2026-08-27, separate conversation) that the chart of
   accounts needs to be developed fresh with real accounting-domain input
   (Jennifer, as the actual bookkeeping operator) — not invented
   unilaterally. This is a hard dependency for `service-input`'s
   classification logic, independent of the WORM/crypto-shredding
   plumbing above, which can proceed without it.
6. Resolve the human-readable-filename question raised in the same
   conversation: bake the chart-of-accounts name directly into the
   permanent (immutable) filename alongside the content hash, rather than
   only in a separate mutable index — improves disaster-recoverability
   (a human with just a file browser can interpret the archive even if
   all custom tooling is lost) at the cost of never being able to silently
   fix a wrong name (corrections become additive: a new, correctly-named
   record plus an explicit supersession note, never a rename). This
   appears compatible with the crypto-shredding design above (the
   plaintext filename itself may need the same per-subject-key treatment
   as other personal-data fields, if it ever encodes personal data) —
   not yet fully reasoned through.

## Work log

- **2026-08-27**: `project-input` provisioned, consolidating service-input/
  service-fs ownership from project-totebox. Prior-art inventory done
  (live ledger format at `cluster-totebox-jennifer-2`, `app-console-input`'s
  `audit.rs` pattern, Jennifer's WORM design notes).
- **2026-08-27/28**: 3 parallel research passes (US financial, US federal,
  EU) — findings above. First research attempt hit a transient server-side
  rate limit on all 3 forks simultaneously; retried successfully.

## Carry-forward

- Decisions-open items 1–6 above, none yet resolved.
- No code written yet — this BRIEF is pre-implementation.
