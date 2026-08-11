---
id: kb-open-question-projection-id-unvalidated-001
title: ProjectionId::new is infallible, and that was never decided
kind: open_question
status: accepted
authority_tier: note
summary: >-
  ProjectionId::new is infallible and unvalidated: the empty string succeeds and becomes a checkpoint row's primary key. ADR-0015 validated every other identifier in the crate and declined to validate this one, for two stated reasons — there is no conformance suite for ProjectionStore, which is provisional and freezes at phase 6, and calling the type 'deliberately opaque' in its docstring would be false, because there was never a decision, only an omission. The docstring says instead that the question is open and names its owner. What is not decided is whether a projection id is validated at all, and if so against what: the identifier rules ADR-0015 applies to EventType and Tag are about wire safety and byte equality, and a checkpoint key's constraints come from the stores that persist it. Forced by phase 6, the ProjectionStore freeze, after which the type is on a frozen port. Settled in practice by the first projection store whose backing table rejects a key the constructor accepts.
depends_on: []
related:
  - kb-decision-0015
  - kb-decision-0007
  - kb-open-question-projection-batch-no-apply-001
source_paths:
  - .kb/_intake/0015-validated-identifiers-and-store-limits.md
  - references/adr/0015-validated-identifiers-and-store-limits.md
  - crates/happenstance-core/src/projection.rs
  - RUNBOOK.md
last_reviewed: 2026-08-10
---

# ProjectionId::new is infallible, and that was never decided

## What is true today

`projection.rs:41-56` defines `pub fn new(value: impl Into<String>) -> Self`
with no validation of any kind, while its siblings — `EventType::new`,
`Tag::new` — both return `Result`. `ProjectionId::new("")` succeeds today and
the empty string becomes the primary key of a checkpoint row. No `VT` clause
covers it.

ADR-0015 is the decision that validated every other identifier constructor in
`happenstance-core` — byte-walk character validation shared between `new`
and a new `from_static`, canonical byte equality, capacity floors — and it
looked directly at `ProjectionId` and took neither of the two exits the
RUNBOOK offered: validate it the way `EventType` is validated, or state in
the docstring that it is "a deliberately opaque operator-chosen key."

The ADR's own reasoning for declining both, stated in full: validating it now
is out of scope in a way that matters, because `ProjectionId` belongs to
`ProjectionStore`, which is `[PROVISIONAL]`, has no conformance suite at all,
and freezes at phase 6 — phase 4 has no instrument that could fail a
`ProjectionId` rule, and CLAUDE.md's own corollary that a rule no adapter can
fail is decorative applies to a validating constructor with nothing to check
it just as much as it applies to a rule. And calling the current shape
"deliberately opaque" would be worse than leaving it alone, because it is
false: "there is no decision behind the current signature; there is an
omission, and blessing it in a docstring would freeze an accident at exactly
the moment the reader is most likely to believe the document." The ADR also
declined a middle path — adding a fallible `ProjectionId::parse` beside the
infallible `new` — because that reproduces the exact defect (D2) that
`Query::Items`'s construction problem (VT-26) is closed against: an
infallible constructor able to produce a value the fallible one would reject.

What shipped instead: `new` is unchanged, and its docstring now says the
validation question is open, names phase 6 as the owner, and names the
empty-string primary-key hazard directly, so a reader learns "unresolved"
rather than "optional."

## What is not decided

Whether a `ProjectionId` is validated at all, and if it is, against what.
The identifier rules ADR-0015 built for `EventType` and `Tag` are answers to
wire-safety and byte-equality questions — control characters, bidirectional
overrides, canonical comparison across a replication boundary. A checkpoint
key has none of those exposures by the same route; its real constraints, if
any, come from the stores that have to persist it as a primary key — length
limits, character-set restrictions, collation behaviour — which are
properties of `ProjectionStore` adapters that do not exist yet.

## What forces it

Phase 6, the `ProjectionStore` freeze, is the earliest point this question
even has an instrument: freezing the port without freezing the identifier
travelling through it repeats the exact "provisional marker that outlives its
own falsifier" shape ADR-0015 was careful to avoid everywhere else. In
practice it will likely be settled sooner and informally, by the first real
projection store whose backing table rejects a key `ProjectionId::new`
happily constructed — an empty string, a value over some column width, a
character the store's collation treats specially.

## Ordered sub-questions

1. Does `ProjectionId` get its own validated constructor at phase 6, mirroring
   `EventType`/`Tag`'s shape, or does it stay a thin `String` wrapper and push
   validation entirely to the adapter boundary the way `AppendError::ExceedsStoreLimit`
   does for capacity?
2. If it is validated, is the rule a validity invariant (enforced by the
   constructor, unrepresentable otherwise) or a capacity limit (enforced at
   the store boundary only, per ADR-0015 §6's distinction) — and does the
   quarantine argument that justifies capacity-limit deferral for `Event`
   even apply to an operator-chosen key nothing else in the system generates?
3. Does the phase-6 `ProjectionStore` conformance suite need a fixture-level
   numeric or character-set declaration analogous to `CF-40`'s pattern for
   event/tag limits, or is a projection id simple enough that one validator
   suffices?
