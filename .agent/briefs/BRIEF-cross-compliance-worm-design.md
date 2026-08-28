---
artifact: brief
schema: foundry-brief-v1
brief-id: project-input-cross-compliance-worm-design
title: Cross-Compliance WORM Design — service-fs / service-input
status: active
owner: project-input
created: 2026-08-28
updated: 2026-08-29
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
- **CFTC Rule 1.31** (commodity trading/derivatives/swaps records,
  researched 2026-08-28): applies to a different registration category
  than SEC broker-dealer status (futures commission merchants, swap
  dealers, and related registrants) — confirming Woodfine/PointSav's
  actual registration status remains the same open prerequisite as for
  17a-4. Substantively, 1.31 requires only "authenticity and reliability"
  of the records system plus a records-system inventory — **it does not,
  and apparently never did (even pre-2022, unlike the earlier vintage of
  17a-4), mandate pure physical WORM.** Adds no new technical requirement
  beyond what 17a-4/FINRA 4511 already established.
- Sources: sec.gov (FAQ + amendments pages, Reg S-P press release),
  finra.org/rules-guidance/rulebooks/finra-rules/4511, cftc.gov (Rule 1.31
  text + guidance).

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

## Does any of this actually require WORM?

Direct answer to the operator's question (2026-08-28): **no — none of the
regimes surveyed mandate pure, physical, irreversible WORM as the only
compliant path.**

- **SEC 17a-4**: WORM was the *original* reading, but the 2022 amendments
  added an explicit audit-trail alternative — a system that permits
  modification/deletion is compliant if it preserves enough audit trail to
  recreate the original record.
- **FINRA 4511**: incorporates 17a-4's format rules directly — same
  conclusion, not a separate stricter requirement.
- **CFTC 1.31**: requires "authenticity and reliability" + a records-system
  inventory — never had a pure-WORM mandate, not even before 17a-4's 2022
  loosening.
- **NARA** (Bulletins 2014-04/2015-04): the closest thing to a real
  immutability requirement found — "able to capture immutable records" for
  *permanent* federal record transfers. This is the one regime where
  immutability language is closest to load-bearing, but the actual bulletin
  text hasn't been read in full yet (Decisions open #3) — don't treat this
  as confirmed-strict until that's done.
- **GDPR/EU**: the opposite of a WORM requirement — erasure rights are in
  active tension with permanent immutability, resolved via crypto-shredding
  (destroy the key, never the ciphertext), not by GDPR demanding WORM.

**Implication**: WORM/immutability is a strong, defensible *design choice*
for this appliance (integrity guarantees, tamper-evidence, audit
simplicity) — but across every regime checked so far, it is not the
*compliance floor* itself. The compliance floor is closer to "authentic,
reliable, tamper-evident, with a real audit trail," which content-addressed
immutable storage satisfies more easily than it's *required* to. This
matters for the "one mechanism + modular policy layer" framing below: the
mechanism doesn't need to be WORM to pass compliance — WORM is chosen
because it's a good mechanism, not because any surveyed regulator demands
it by name.

## Uniform encryption vs. selective classification (2026-08-28)

Operator question: should *all* content be treated like personal data
(uniformly encrypted with a destroyable per-record key), to eliminate the
"is this personal data" classification judgment call at ingest time?

**Assessment**: a reasonable simplification, not a free one — a real
tradeoff, not a default to assume either way.

Gains:
- One ingest code path instead of two (no per-file personal-data judgment
  call).
- Safer default — never accidentally under-protects a document that turns
  out to contain personal data nobody flagged (a signature block, a
  handwritten annotation, a name buried in a financial statement).

Costs:
- The register/key-store becomes a single point of failure for *all*
  content, not just the subset that has real personal data in it.
- Most business records (financial statements, floor plans, aggregate
  proforma models) have no real "right to erasure" use case at all — no
  individual could invoke GDPR Article 17 over a floor plan. Encrypting
  those anyway adds a register dependency without adding real compliance
  value.

**Recommendation**: worth doing, conditional on two things holding, both
non-negotiable if adopted:
1. **Filenames/metadata stay plaintext regardless of content encryption**
   — preserves the disaster-recovery property (a human with just a file
   browser can still interpret the archive with zero software, even if
   every file's *content* now depends on the register).
2. **The register/key-store gets a redundant backup strategy designed in
   from day one**, not bolted on later — since everything now depends on
   it, this stops being a nice-to-have and becomes the single most
   load-bearing piece of the design.

Not yet ratified — captured here as a reasoned recommendation pending
operator sign-off, same status as the "one mechanism + modular policy
layer" framing below.

## No-register alternative — retention-bounded WORM (2026-08-29)

Operator pushback (2026-08-29): "people shouldn't have to encrypt data if
they don't want to — is there a way to not have a register at all?" Re-read
the actual `os-totebox`/`service-fs` design docs before answering (not
assumed): the `content-wiki-projects` draft `topic-totebox-archive.draft.md`
states the WORM invariant as absolute — "There is no `DELETE` operation and
no `UPDATE` operation — only append... The immutability guarantee is
structural, not a configuration option." `BRIEF-OS-FAMILY.md` confirms the
same: `service-fs` is "WORM append-only," "VMs cannot [modify/delete]."
**Neither doc mentions encryption or a key-register anywhere — that
mechanism was this BRIEF's own proposal, not an existing product
commitment.** The operator is right to push back on it as an unnecessary
addition, not an established requirement.

**A genuinely register-free alternative exists, and the compliance
research already gathered supports it:**

Instead of "immutable forever + crypto-shred to forget," use **immutable
for a bounded retention window, then real physical deletion** — no
encryption, no keys, no register, for anyone, ever:

- While a record is inside its applicable legal retention window (6yr
  FINRA, 5yr MiFID II, whatever a given deployment's regime sets), it is
  fully WORM — no update, no delete, exactly as documented today.
- A GDPR erasure request arriving during that window is lawfully refused
  under Article 17(3)(b)'s legal-obligation exemption (already verified in
  §3 above) — no crypto-shred needed to "honor" it, because the law doesn't
  require honoring it yet.
- Once the retention window naturally expires (and no other legal hold
  applies), the record becomes eligible for real, physical deletion — a
  true filesystem/object delete, not a key-destruction trick.
- For a deployment where retention is meant to be genuinely permanent (a
  NARA federal permanent-record deployment, or a customer who simply wants
  "forever" as their own policy), set that deployment's retention window to
  "no expiry" — the record is then eternal WORM exactly as documented
  today, with zero behavior change from the current design.

**Why this beats crypto-shredding as the default:** it removes the register
entirely as a dependency, for all content, personal-data or not — nobody
"has to encrypt," because nothing needs a destroyable key at all. Retention
windows are just metadata (a date + a deployment policy), and losing that
metadata is a low-stakes failure (worst case: over-retention, which is
inconvenient, not a compliance violation and never a data-loss event) —
categorically safer than losing an encryption-key register, which is
irreversible data loss.

**Real tension this creates, needing explicit ratification, not a silent
override:** `topic-totebox-archive.draft.md`'s current language — "no
`DELETE` operation... structural, not a configuration option" — is written
as an absolute, permanent product identity, not merely a compliance
mechanism. It may be a deliberate sellable promise ("your record is
provably permanent, we structurally cannot alter or delete it, forever")
independent of what any specific regulation actually requires. Introducing
*any* real delete path — even one gated behind a long retention window and
disabled-by-default for permanent-record deployments — changes that
documented invariant. This needs the operator's explicit sign-off, not an
AI judgment call, because it revises a stated architectural commitment
rather than just picking an implementation detail.

**Residual edge case, out of scope unless a real customer need surfaces:**
a customer wanting to honor an erasure request *during* an active
retention-mandate window, beyond what the law requires — without a
register/key mechanism, the only way to grant that is a targeted in-place
delete, which breaks the WORM invariant on a single record rather than at
natural expiry. No research finding so far requires supporting this; not
worth designing for until a specific deployment actually asks.

**Refinement (2026-08-29, operator direction): permission-to-delete, not
automatic deletion.** Retention-window expiry does not trigger an automatic
purge job. It changes a record's state from "cannot be deleted" (structurally
enforced, identical to today) to "**eligible** for deletion" — an explicit,
deliberate, separately-authorized, logged action, never a silent background
process. This closes the litigation-hold gap for free: nothing ever
auto-fires, so a hold is simply "don't exercise the now-available
permission yet" — no separate override mechanism needs to be designed.
It also narrows the doctrine tension considerably: the attack surface
DOCTRINE.md's language worries about ("a future maintainer toggling a
flag") is a system-wide immutability switch; "permission to delete one
specific eligible record, deliberately, with a logged authorization" is a
categorically smaller, harder-to-abuse surface than that.

**Status: recommended, not yet ratified.** This supersedes crypto-shredding
+ mandatory register as the *default* direction — that mechanism remains
documented above (§"Uniform encryption...") as a fallback only for the
narrow residual edge case just described, not as the primary design.

## Doctrine tension — DOCTRINE.md §IX (2026-08-29)

`DOCTRINE.md` §IX ("External WORM standards alignment") already made a
deliberate, reasoned choice directly relevant here: for SEC Rule 17a-4(f),
Foundry "targets the **WORM path** explicitly (not the Audit-Trail
alternative introduced in the 2022 amendment)," specifically because
"a policy-layer WORM enforcement can be undone by a future maintainer
toggling a flag" while "a storage-substrate WORM enforcement... cannot."
`conventions/worm-ledger-design.md` (the spec DOCTRINE cites as
authoritative) has **zero mentions of retention, deletion, erasure, or
GDPR** — the audit-trail-vs-WORM choice was ruled on; the narrower
retention-bounded-deletion question this BRIEF is proposing was not.

Per `CLAUDE.md`'s own standing rule — "where this file and DOCTRINE.md
disagree, DOCTRINE.md wins; surface the conflict as a NEXT.md cleanup
item" — this tension is logged here and will be logged to NEXT.md, not
silently resolved.

**Operator direction (2026-08-29):** research both framings rather than
pre-committing — but the working assumption is that DOCTRINE.md's current
language may simply be out of date with what the new research and ideas
support, and updating it is in scope if the cross-check research holds up
(not staying fixed by default). A promising reconciliation hypothesis,
**to be verified by the cross-check research pass now underway**: SEC
17a-4(f)'s WORM requirement may have only ever meant "immutable for (at
least) the mandatory retention period," never literally eternal — in which
case "WORM during the mandatory window, eligible-for-deletion (not
automatic) after it expires" is fully *within* the WORM path DOCTRINE
already chose, not a departure from it, and no doctrine amendment is even
needed — only a clarification that resolves apparent-not-real tension.
This needs a primary-source check before treating it as settled either way.

## Cross-check research results (2026-08-29) — 5 parallel primary-source passes

Operator directive: "double check everything" before touching any docs.
Five independent research passes, each targeting one regime/standard.
**Net verdict across all five: the reconciliation hypothesis above holds.
No regime checked requires eternal retention. Retention-bounded WORM +
permission-to-delete (not automatic) + litigation-hold suspension is not
just compliant — it is how records management is already, formally,
supposed to work.** This is closer to a clarification of DOCTRINE.md's
existing WORM-path commitment than a reversal of it.

### 1. SEC 17a-4(f) / FINRA 4511 — bounded, not eternal

Verdict: **confirmed bounded.** The SEC's own language (2022 amendment) for
*both* the WORM path and the audit-trail alternative is scoped to "the
required retention period" — not open-ended. Standard periods: most
transaction records ≥3yr, financial ledgers ≥6yr; the one real "permanent"
exception is entity-formation records, retained for the *life of the firm*
(a different bounded window — firm existence — not literal eternity).
Disposal after expiry is standard, permitted, expected industry practice;
FINRA enforcement focus is entirely on *premature* destruction, never on
disposal-after-expiry being impermissible. **Caveat**: the Federal Register
primary text itself was bot-blocked during this pass; verdict rests on
convergent practitioner-legal secondary sources (Dechert, ACA Global,
Archive360, Smarsh), not the raw rule text directly. A follow-up primary
pull (SEC.gov rule-text page or FINRA's own PDF chart-of-changes) is worth
doing before treating this as fully closed — flagged as decisions-open #10.

### 2. CFTC 1.31 — most permissive regime checked

Verdict: **confirmed, primary source (eCFR/Cornell LII 17 CFR §1.31).** No
WORM mandate — only "authenticity and reliability" controls (§1.31(c)(2)).
Retention periods explicit, bounded minimums ("not less than" 5yr swaps,
1yr oral comms, 5yr other records) — never eternal. The rule is entirely
**silent** on disposal after expiry, which is the standard reading that
firms may dispose once the floor is met. No tension with the proposed
design at all.

### 3. GDPR Art 17 + MiFID II — confirmed, with two real refinements needed

Art 17(3)(b)'s legal-obligation exemption is confirmed via primary text
(gdpr-info.eu) plus a real regulatory-tier source found this pass: the
**EDPB's 2025 Coordinated Enforcement Action report on the right to
erasure** (32 DPAs). It confirms the exemption's validity but flags a real
enforcement failure mode worth designing against: DPAs found controllers
treating the exemption as automatically applicable *without case-by-case
assessment*. **Design requirement, new**: every deletion decision (granted
or withheld) must log *why* — the specific retention schedule/legal basis
— not just *that* a decision was made. An audit trail of reasoning, not
just of state.

MiFID II *lex specialis*: **downgrade confidence** — no ESMA-authored
primary source was found confirming this in either research pass. Every
source (Lexology, SteelEye, RBC, KSF) is practitioner/secondary; the legal
logic itself is sound and uncontested across all of them, but this BRIEF
should describe it as "well-reasoned legal argument, not adjudicated
regulatory precedent," not overclaim ESMA authority.

**New gap identified, real and unaddressed until now**: for personal data
that is genuinely NOT covered by any retention exemption, GDPR Art 17(1)'s
erasure obligation is *affirmative* — "without undue delay," ~30 days.
"Permission to delete" alone does not satisfy this; nobody is obligated to
act on a mere permission. **This needs a second, distinct trigger**,
separate from retention-window expiry: an erasure request against
non-exempt content must open a tracked, SLA-bound task (logged, escalates
if unactioned) that closes by someone exercising the deletion permission
within the legal window. Retention-expiry eligibility has no undue-delay
clock (17(3)(b) fully exempts it) — this is a second, different mechanism,
not a variant of the first.

### 4. NARA Bulletins + federal disposal practice — pattern confirmed as standard, not novel

Bulletins read in full (primary, archives.gov): 2014-04 (format
tables, now updated by 2018-01) sets required/preferred/acceptable formats
per record category; 2015-04 sets required item-level metadata — File Name,
Record ID, Title, Description, Creator, Creation Date, Rights (+ optional
Coverage, Relation), built on Dublin Core, with Appendix B platform-
independent naming conventions. **Permanent records confirmed as true
no-expiry**: records default to permanent absent an approved disposition
schedule; destruction requires an Archivist-approved schedule (44 U.S.C.
Ch. 33); a narrow reappraisal-as-temporary pathway exists but is rare and
formal, not a normal lifecycle event.

**The exact mechanic this BRIEF proposed — retention-expiry makes a record
eligible for disposal, disposal is a deliberate authorized action (never
automatic), and a litigation hold suspends the disposition cycle — is
confirmed as existing, standard federal records practice**, not an
invention (36 CFR Parts 1225/1230; agencies route eligible records through
legal-counsel litigation-hold review before destruction proceeds).

### 5. eIDAS + ISO 15489/ARMA GARP — confirmed bounded; confirmed established practice

EU CIR 2025/1946 (primary, eur-lex.europa.eu): **not eternal** — it defines
technical fidelity requirements "until the end of the preservation period,"
a defined endpoint left to the record owner or other governing law; the
regulation governs *how* preservation is done, not *how long*. No tension
with a bounded-retention design.

**ISO 15489-1 and ARMA GARP's Principle of Disposition — both primary
standards, not secondary commentary — describe this exact pattern as
correct practice**: retention runs per a documented schedule; "once the
life of a record has been satisfied... and there are no legal holds
pending, it is authorized for final disposition"; US practice extends the
retention clock automatically once litigation/claim/audit begins before
expiry. This is the litigation-hold-suspension mechanism this BRIEF
proposed, described in the field's own foundational standard.

### Synthesis

No regime surveyed — SEC, FINRA, CFTC, GDPR, MiFID II, NARA, eIDAS —
requires eternal, no-exceptions retention. Every regime's retention
requirement is a **minimum**, not a permanent mandate, and disposal after
the minimum (subject to litigation-hold suspension) is either explicitly
permitted or the field's own standard practice (ISO 15489/ARMA GARP). Two
concrete design requirements emerged that weren't in the BRIEF before this
pass: (1) log the *reasoning*, not just the state, behind every deletion
decision (EDPB finding); (2) a second, SLA-bound, "without undue delay"
trigger for GDPR erasure requests against non-exempt personal data — distinct
from the passive, no-deadline retention-expiry-eligibility trigger.

## Self-contained records — eliminating even the retention register (2026-08-29)

Operator direction (2026-08-29): "build something radically different that
covers all these scenarios but is entirely self-contained." Taken literally
against the retention-bounded-WORM proposal above: that proposal still
implied *some* external record of each file's retention window (creation
date + which regime applies) — smaller and lower-stakes than a crypto
register, but still a thing that could exist separately from the file and
drift or go missing.

**Push the same principle one level further: encode retention policy, not
just classification, directly into each record itself, so no external
register — for classification, for retention, or for anything else — is
needed to correctly interpret or correctly purge a single file.**

Proposed self-contained record shape (filename or embedded header,
whichever the format supports):

```
<sha256>--<chart-of-accounts-stem>--<retention-class>--<created-ISO8601>.<ext>
```

Example: `9f8a3c...--WCP-ADM-2026-financial-statement--finra-6y--2026-08-29.pdf`

- `sha256` — content identity (existing pattern, from the live ledger.jsonl
  format).
- `chart-of-accounts-stem` — human-readable classification (existing
  decisions-open item 6, still needs Jennifer's real input).
- `retention-class` — a short tag from a small, fixed, versioned vocabulary
  (`finra-6y`, `mifid-5y`, `nara-permanent`, `internal-90d`, etc.). The
  tag→duration mapping lives in the *product's own documentation/code* —
  a static spec, not a growing per-record database. Losing it doesn't lose
  any file or any file's identity; it only pauses the ability to compute
  new expiry dates until it's restored, and it ships with the software
  itself (recoverable from any install, any git history, any backup of the
  binary) rather than being unique per-deployment state.
- `created-ISO8601` — the retention clock's start point, carried by the
  record itself.

**What this achieves**: a purge job (or a human, or a from-scratch restore
after total data loss) can correctly determine every record's classification
*and* retention/deletion eligibility by reading that one record alone — no
lookup against any separate system. An index/database on top is still
useful for fast search, but becomes purely a performance cache: if it's
lost, it is mechanically rebuildable by rescanning the archive and
re-parsing filenames, never a data-loss event and never a compliance risk.
This is the same "entirely self-contained" principle `os-totebox` already
claims at the appliance level ("the archive's identity, keys, and data
travel with the disk image unchanged" — `topic-totebox-archive.draft.md`
§"Storage model") — this proposal just carries that same self-containment
down to the level of a single file inside the archive, rather than
introducing a new principle the product doesn't already stand for.

**Net effect across all three open threads this operator conversation has
covered:** zero mandatory encryption (nobody has to encrypt anything to
participate), zero mandatory register for classification (item 6, already
proposed), zero mandatory register for retention/deletion (this section) —
the record is the single source of truth for everything about itself. The
only thing external to any single record is a small, static,
software-shipped spec (retention-class vocabulary + duration mapping),
which is categorically different from a growing, per-deployment,
must-be-backed-up register.

**Still needs ratification** (folds into decisions-open item 9): this
still requires `service-fs` to support a real delete operation gated on
retention-class expiry, the same product-identity question raised above.

## Architectural implication — the "one unified floor" question, revisited

**Superseded 2026-08-29 — kept for reference, not the current direction.**
This section's crypto-shredding-based mechanism is superseded by
"No-register alternative" and "Self-contained records" above, both
confirmed against primary sources in the cross-check research pass. The
*reframing itself* — "one unified mechanism, modular policy layer on top"
— still holds and now maps onto the newer design: the mechanism is
retention-bounded self-contained WORM records; the modular policy layer is
each deployment's retention-class vocabulary and duration mapping.

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

1. ~~Ratify "one mechanism + modular policy layer" framing~~ — **the
   reframing survives, done 2026-08-29**; now maps onto the
   retention-bounded self-contained design, not crypto-shredding. See
   superseded-note above.
2. Confirm Woodfine/PointSav's actual registration status (broker-dealer?
   investment adviser? neither?) — determines whether SEC 17a-4/FINRA/
   Reg S-P are binding-now requirements or aspirational-for-future-
   customers requirements. Do not build as if they're binding without
   this confirmed. **Still open — no research pass has addressed this.**
3. ~~Read NARA Bulletin 2014-04/2015-04 in full~~ — **done 2026-08-29**,
   findings in "Cross-check research results" §4 above. Format/metadata
   tables summarized; `service-input`'s metadata schema should still
   consult the bulletins directly for the full field-by-field spec before
   final implementation, but the "what's required" question is answered.
4. ~~Research CFTC Rule 1.31~~ — **done 2026-08-28**, findings folded into
   §1 above and the "does any of this require WORM?" synthesis. No new
   open question from this item; commodity-trading registration status
   folds into open item 2 (registration status generally).
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
7. ~~Ratify uniform encryption vs. selective personal-data
   classification~~ — **superseded 2026-08-29** by item 8: retention-bounded
   WORM removes the register/encryption dependency entirely, for all
   content, making this question moot rather than answered either way.
8. **Done 2026-08-29** — no-register alternative proposed: retention-bounded
   WORM (immutable during the legal retention window, real physical
   deletion after natural expiry, "no expiry" setting for permanent-record
   deployments). See "No-register alternative" section above. **Ratify or
   reject as the primary direction** — recommended, not yet ratified.
9. Ratify or reject the deviation itself: item 8 requires `service-fs` to
   support a real (non-cryptographic) delete operation, gated by
   retention-window expiry — a change to `topic-totebox-archive.draft.md`'s
   current documented claim that "no `DELETE` operation... is structural,
   not a configuration option." **Substantially de-risked by the
   cross-check research (2026-08-29)**: every regime surveyed treats
   retention as a bounded minimum, and "eligible-for-disposal, deliberate
   not automatic, litigation-hold-suspended" is confirmed as the field's
   own established standard practice (ISO 15489/ARMA GARP), not a novel
   weakening. Operator's own framing: current docs may simply be out of
   date and due for an update, not a doctrine reversal. Still needs a
   final explicit sign-off before any code or DOCTRINE.md text changes —
   the research supports proceeding, but hasn't been asked to proceed yet.
10. **(2026-08-29, new)** Pull the SEC 17a-4(f) 2022-amendment primary text
    directly (Federal Register release 2022-22670, or FINRA's own
    chart-of-changes PDF) — the cross-check pass's SEC/FINRA verdict rests
    on convergent secondary/practitioner-legal sources because the Federal
    Register page itself was bot-blocked during the pass. The verdict
    (bounded, not eternal) is well-supported but not yet primary-source-closed.
11. **(2026-08-29, new)** Downgrade the MiFID II *lex specialis* claim in
    this BRIEF's §3 (EU findings) from implied regulatory precedent to
    "well-reasoned legal argument, uncontested across practitioner sources,
    not confirmed by any ESMA-authored primary document" — no primary
    source was found in either research pass despite two attempts.
12. **(2026-08-29, new)** Design a second, distinct deletion-trigger path
    for GDPR erasure requests against personal data with **no** applicable
    retention exemption: an SLA-bound, logged, escalating task (~30-day
    "without undue delay" clock) that closes by someone exercising the
    deletion permission — separate from the passive, no-deadline
    retention-expiry-eligibility trigger, which has no undue-delay clock
    at all (fully exempted under Art 17(3)(b)). A bare "permission to
    delete" satisfies nothing on its own for this specific case.
13. **(2026-08-29, new)** Every deletion decision — granted or withheld —
    must log the specific reasoning (retention schedule reference, legal
    basis, or GDPR ground), not just the resulting state. Real EDPB
    enforcement finding: DPAs found controllers applying the legal-obligation
    exemption automatically, without case-by-case assessment — the audit
    trail needs to capture *why*, not just *that*.

## Work log

- **2026-08-27**: `project-input` provisioned, consolidating service-input/
  service-fs ownership from project-totebox. Prior-art inventory done
  (live ledger format at `cluster-totebox-jennifer-2`, `app-console-input`'s
  `audit.rs` pattern, Jennifer's WORM design notes).
- **2026-08-27/28**: 3 parallel research passes (US financial, US federal,
  EU) — findings above. First research attempt hit a transient server-side
  rate limit on all 3 forks simultaneously; retried successfully.
- **2026-08-28**: CFTC Rule 1.31 researched — no WORM mandate, consistent
  with 17a-4/FINRA. Added an explicit "does any of this require WORM?"
  synthesis (answer: no, not as a hard mandate, across every regime
  checked so far — NARA is the closest, still unconfirmed pending full
  bulletin read). Operator asked whether to treat all content as personal
  data (uniform encryption) to simplify design — recommendation given,
  not yet ratified.
- **2026-08-29**: Operator pushed back on mandatory encryption/register —
  read the actual `os-totebox`/`service-fs` TOPIC/BRIEF docs directly
  (`topic-totebox-archive.draft.md`, `BRIEF-OS-FAMILY.md`) to confirm
  neither one ever specified encryption or a key-register; that was this
  BRIEF's own addition, not existing product commitment. Proposed a
  register-free alternative: retention-bounded WORM (immutable during the
  legal retention window, real deletion after natural expiry, "no expiry"
  for permanent-record deployments) — no crypto, no keys, no register, for
  anyone. Flagged one real tension needing explicit ratification: this
  requires a genuine delete capability in `service-fs`, which the current
  docs state does not exist "structurally... not a configuration option."
  Operator then asked for something "radically different... but entirely
  self-contained" — extended the proposal so retention policy, not just
  classification, is encoded directly in each record's own filename
  (content hash + chart-of-accounts stem + retention-class tag + creation
  date), eliminating the retention register too — only a small, static,
  software-shipped retention-class vocabulary remains external, and its
  loss is never a data-loss event. Matches `os-totebox`'s own existing
  self-containment claim at the appliance level, extended to file level.
  Found `DOCTRINE.md` §IX already made a deliberate, reasoned pure-WORM
  choice for SEC 17a-4(f) specifically to resist "a future maintainer
  toggling a flag" — logged as a real tension per `CLAUDE.md`'s own
  conflict-surfacing rule, not silently resolved. Corrected an earlier
  session error: "DARP" is a real, existing workspace term (Data Archive
  Retrieval Protocol, `DOCTRINE.md` §IX) meaning plain-text/self-describing
  readability — not FedRAMP, as an earlier turn incorrectly told the
  operator. Operator refined the deletion model to "permission to delete,
  not automatic" and asked for a web cross-check before any doc edits;
  launched 5 parallel primary-source research passes (SEC/FINRA, CFTC,
  GDPR/MiFID II, NARA, eIDAS+records-management standards).
- **2026-08-29 (cross-check complete)**: all 5 passes returned. Net
  verdict: no regime surveyed requires eternal retention; the proposed
  design (retention-bounded WORM, permission-to-delete not automatic,
  litigation-hold suspension) is confirmed compliant across SEC/FINRA/
  CFTC/GDPR/MiFID II/NARA/eIDAS, and is literally how ISO 15489/ARMA GARP
  define correct records disposition practice — not a novel invention.
  Full findings in "Cross-check research results" section above. Three
  real refinements surfaced and added to decisions-open: (1) SEC/FINRA
  verdict needs a primary-source follow-up (Federal Register was
  bot-blocked this pass); (2) MiFID II *lex specialis* downgraded to
  "sound legal argument, not confirmed regulatory precedent" — no ESMA
  primary source found; (3) a second, SLA-bound deletion trigger is needed
  for GDPR erasure requests on non-exempt personal data (distinct from the
  passive retention-expiry-eligibility trigger); (4) every deletion
  decision needs its specific legal reasoning logged, not just the
  resulting state (real EDPB enforcement finding). Doctrine tension
  substantially de-risked — reads as a clarification of DOCTRINE.md's
  existing WORM-path commitment, not a reversal, pending final operator
  sign-off before any DOCTRINE.md/TOPIC/GUIDE text actually changes.

## Carry-forward

- Decisions-open items 1–6 above, none yet resolved.
- No code written yet — this BRIEF is pre-implementation.
