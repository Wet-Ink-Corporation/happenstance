---
id: kb-open-question-remint-precondition-trust-only-001
title: VT-6's documented-procedure branch asks a program to trust a document it could check
kind: open_question
status: accepted
authority_tier: note
summary: >-
  VT-6 requires that a StoreId be minted when a store's persistent state is created and never
  derived from anything surviving a restore, and ADR-0014 grants an adapter three ways to satisfy
  it: mint once and re-mint only when it can detect a restore or a clone, mint fresh on every open,
  or re-mint under a procedure the deployment documents. The third branch is the one this question
  is about. SqliteEventStore::remint_identity is that branch's implementation and its precondition
  is trust-only: the caller asserts that this database file is a restore or a clone, and nothing in
  the adapter checks the assertion, including in the case a program could see - the same file,
  opened twice, in one process. What is not decided is whether an adapter taking the
  documented-procedure branch owes an in-process check, and if so what it may check against without
  inventing a durability claim of its own. ADR-0014 is accepted and grants the branch without
  addressing the obligation, so this is a gap inside a branch a decision opened rather than a
  disagreement with one. Forced by phase 5's identity work and by phase 13, where sync makes a
  duplicated StoreId a peer's problem rather than a local one - a re-minted identity that should not
  have been re-minted is exactly the collision VT-6 spends a clause preventing.
depends_on: []
related:
  - kb-decision-0014
  - kb-decision-0022
source_paths:
  - .kb/_intake/2026-09-03-pre-publication-review.md
  - references/evaluation/review-pre-publication-2026-09-03.md
  - crates/happenstance-sqlite/src/event_store.rs
  - spec/SPECIFICATION.md
last_reviewed: 2026-09-04
---

# VT-6's documented-procedure branch asks a program to trust a document it could check

## What is true today

VT-6 (`spec/SPECIFICATION.md:813`, `[PROVISIONAL]`) requires that a `StoreId` be minted when a
store's persistent state is created and never derived from anything that survives a restore, and
forbids a store from ever issuing an `EventId` whose `(StoreId, SequencePosition)` pair it has
issued before for a different event. ADR-0014 grants an adapter three ways to satisfy that: mint
once and re-mint only when the adapter can detect a restore or a clone itself, mint fresh on every
open, or — the branch this question is about — re-mint "if the deployment documents when
re-minting is invoked" (`.kb/decisions/0014-event-identity-and-recorded-time.md:21-22`). That third
branch trades an in-code guarantee for an out-of-band one: the correctness of the `StoreId` no
longer depends on what the adapter can observe, only on whether whoever operates the deployment
followed the document.

`SqliteEventStore::remint_identity` (`crates/happenstance-sqlite/src/event_store.rs:448`) is that
branch's concrete implementation. Its precondition, stated in its own doc comment, is exactly the
trust the branch describes: the caller asserts this database file is a restore or a clone of
another, and the function's job is to act on that assertion, not to verify it. It opens the file,
runs migrations, and overwrites `store_meta`'s `store_id` row with a fresh random value inside an
immediate transaction — nothing in the function inspects whether the file is actually a restore, a
clone, or simply the same live store being re-minted by mistake. `tests/migration.rs:421`
(`remint_replaces_the_persisted_identity`) exercises exactly the case a program could catch and
does not: it opens the store once to read `before`, calls `remint_identity` on the same path, then
reopens to read `after` — all three touching one file in one process, with no restore or clone
anywhere in the sequence — and the test asserts only that the operation produced a new value, not
that the operation was warranted.

## What is not decided

Whether an adapter taking the documented-procedure branch owes an in-process check before honouring
a re-mint request, and if so what it may check against without itself inventing a durability claim
`ADR-0014` never asked for. A check needs something to check *against* — a last-known identity
cached somewhere the adapter trusts, a generation counter, an external attestation from whatever
performed the restore — and every candidate is a new piece of state with its own persistence and
correctness obligations, which is exactly the complexity the documented-procedure branch exists to
avoid paying inside the adapter. The alternative, leaving the branch trust-only as it stands today,
means `VT-6`'s guarantee is only as strong as an operational discipline no type or test enforces,
for the one branch of three where an adapter chose that trade.

`ADR-0014` is accepted and grants the branch without addressing this obligation either way; it is
not wrong about anything it says, and this is a gap inside the branch it opened rather than a
disagreement with the decision itself.

## What forces it

Phase 5's identity work already shipped the branch and its first implementation; this question
outlived that phase without being asked. Phase 13, `happenstance-sync`, is where the cost of
leaving it unasked becomes concrete: a duplicated `StoreId` stops being one store's internal
bookkeeping mistake and becomes a peer's problem, because sync deduplicates events by the
`(StoreId, SequencePosition)` pair VT-6 promises is unique. A re-mint that should not have happened
— the exact case `tests/migration.rs:421` demonstrates is possible today — reissues positions a peer
has already seen under a new identity, and the peer has no way to know the two `StoreId`s were ever
the same store.
