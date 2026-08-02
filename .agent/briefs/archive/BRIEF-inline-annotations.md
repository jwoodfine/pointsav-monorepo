---
artifact: brief
schema: foundry-brief-v1
status: archived
brief-id: project-knowledge-inline-annotations
parent: project-knowledge-visual-excellence
owner: project-knowledge
created: 2026-06-29
updated: 2026-07-09
superseded-by: project-knowledge-ng-rewrite
---

# BRIEF — Inline Annotations (Talk Tab Replacement)

> **Parent:** [[project-knowledge-visual-excellence]]
> **Scope:** Replace Talk tab with a full inline annotation/review system
> **Operator decision:** Option B (full system) confirmed 2026-06-29

## Why Replace Talk

Research (2026-06-29) across SharePoint, Confluence, Guru, and Slab shows that
regulated industries and government-facing knowledge platforms use **inline comments
with audit trails** instead of separate Talk pages.

A "Talk" page reads as "community forum" (casual, Wikipedia-legacy). An inline annotation
system reads as "editorial review trail" (authoritative, auditable). The latter is the
correct signal for SOC3/DARP/US-EU government audit positioning.

**Specific research finding:** Guru ("the enterprise governance play") enforces that content
stays current by requiring named reviewers to sign off before publication. Inline comments
tied to specific content passages, with mandatory resolution before publication, is the
pattern for compliance-heavy industries "where wrong answers have regulatory stakes."

Our positioning: these wikis ARE the canonical records. The annotation system should
reflect that editorial weight.

---

## Functional Specification

### What annotations are

- Threaded inline comments tied to a specific article heading anchor (`#heading-slug`)
- Each annotation: named author (Foundry identity) + ISO 8601 timestamp + body text + resolution status
- Thread: parent annotation + replies (flat thread, not nested)
- Status: `open` | `resolved` | `wont-fix`
- Stored as git-committed markdown sidecar files (no database, no external service)

### Storage format

Per-article sidecar at `annotations/{slug}.yaml` in each content repo:

```yaml
# annotations/topic-design-tokens.yaml
schema: foundry-annotations-v1
slug: topic-design-tokens
---
- id: ann-20260629-001
  anchor: "#design-token-naming"
  author: jwoodfine
  created: 2026-06-29T14:00:00-07:00
  status: open
  body: |
    The naming convention here diverges from DTCG spec §3.2 — `--color-primary`
    should be `--color.primary` in JSON but CSS custom properties use hyphens.
    Should we add a note clarifying the CSS translation?
  replies:
    - id: ann-20260629-001-r1
      author: pwoodfine
      created: 2026-06-29T15:00:00-07:00
      body: |
        Agree. Adding a prose note to the article is the right fix. Resolving once done.
      status: open
```

**Why YAML sidecars:** git-trackable, human-readable, diff-able, no database dependency,
consistent with the "canonical record is git" principle (Doctrine §IV.e).

### Article chrome changes

**Tab bar:** rename "Talk" → "Notes (N)" where N is open annotation count for this article.

```rust
// wiki_handlers.rs tab bar
a.wiki-tab href="/notes/{slug}" {
  "Notes"
  @if open_count > 0 {
    span.wiki-tab__badge { (open_count) }
  }
}
```

**Inline annotation anchors in prose:** small annotation icon (⊕) floats right of headings
that have annotations. Click opens the notes panel filtered to that anchor.

```css
.prose h2 .annotation-anchor,
.prose h3 .annotation-anchor {
  opacity: 0;
  margin-left: var(--sp-2);
  font-size: 0.8em;
  color: var(--s-accent);
  cursor: pointer;
  transition: opacity 150ms;
}
.prose h2:hover .annotation-anchor,
.prose h3:hover .annotation-anchor { opacity: 1; }
```

### Notes page (`/notes/{slug}`)

Replaces Talk page at the same URL pattern. Renders:
- Article title + link back
- Per-anchor thread list (all annotations for this article, grouped by anchor)
- Resolution status filter (All / Open / Resolved)
- New annotation form (F12-gated — requires explicit submit action, no auto-save)
- Per-annotation: author, timestamp, status badge, body, reply thread, resolve button

**F12 gate (SYS-ADR-10):** Submitting a new annotation or changing status is an explicit
operator action. No auto-submit, no implicit save. "Post annotation" button is the F12.
Annotations are git-committed by the engine: `git commit -m "annotation(slug): add note from author"`.

### Audit trail

Every annotation create/update/resolve generates a git commit in the content repo:
```
annotation(topic-design-tokens): resolve ann-20260629-001 (jwoodfine)
annotation(topic-design-tokens): add note on #design-token-naming (jwoodfine)
```

This makes every annotation action part of the git history — a full audit trail without
any external system.

---

## Implementation Plan

### Session D1 — Storage format + reader

**Files:** `src/annotations.rs` (new), `src/server/wiki_handlers.rs`
**What:**
- `annotations.rs`: `Annotation`, `AnnotationThread` structs; YAML deserialization from sidecar
- `ContentRepo::load_annotations(slug)` → `Vec<AnnotationThread>`
- `/notes/{slug}` GET handler: read sidecar → render notes page (read-only first)
- Tab bar: rename "Talk" → "Notes (N)" with open count

**Tests:** unit test for YAML parse + empty sidecar fallback

### Session D2 — Notes page HTML + CSS

**Files:** `src/server/wiki_handlers.rs`, `static/style.css`
**What:**
- Notes page template: per-anchor sections, thread rendering, status badges
- New CSS: `.annotation-thread`, `.annotation-item`, `.annotation-status`, `.annotation-reply`
- Inline anchor icons in prose: `.annotation-anchor` floats at headings with notes

### Session D3 — Write path + F12 gate

**Files:** `src/server/wiki_handlers.rs`, `src/annotations.rs`
**What:**
- POST `/notes/{slug}/add` — form submit → write sidecar + git commit
- POST `/notes/{slug}/{id}/resolve` — status update + git commit
- POST `/notes/{slug}/{id}/reply` — reply + git commit
- CSRF token check (standard form pattern already in engine)
- Auth gate: must be authenticated (existing auth middleware)

**F12 note:** The "Post annotation" button IS the F12. No intermediate saves. Explicit submit only.

### Session D4 — Playwright assertions + integration

**Files:** `scripts/responsive-check.js`
**What:**
- R-notes-1: `/notes/{slug}` returns 200 with `role="main"` landmark
- R-notes-2: Tab bar shows "Notes" label (not "Talk")
- R-notes-3: Empty annotations page renders without error (no sidecar = empty state)
- R-notes-4: Annotation count badge absent when no open annotations

---

## Compliance alignment

**Why this matters for SOC3/DARP/gov audit:**
- Named author + timestamp on every annotation → "named reviewers" requirement
- Resolution status → "change tracked" requirement
- Git commit per action → "immutable audit trail" requirement
- F12 gate → "no automated publishing to verified ledgers" (SYS-ADR-19)

**WCAG notes:**
- Form labels required on all annotation form fields (WCAG 1.3.1)
- Status badges must have text, not color alone (WCAG 1.4.1)
- Focus management: after submit, focus moves to the new annotation item (WCAG 2.4.3)

---

## What this is NOT

- Not a CMS commenting system (no external service, no notifications, no mentions)
- Not a Wikipedia-style talk page (separate page for community debate)
- Not a review workflow (no approval gates, no publishing blocks — annotations are advisory)
- Not threaded beyond one level (no nested replies; flat thread per anchor)

The model is editorial notes on a draft article — professional, named, resolvable, git-backed.

---

## Work log

| Date | Session | Commit | What |
|---|---|---|---|
| (sessions pending) | — | — | — |

---

*Brief created 2026-06-29 by totebox@project-knowledge | operator decision: Option B (full inline annotation system) confirmed 2026-06-29*
