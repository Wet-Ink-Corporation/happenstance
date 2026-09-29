---
id: kb-open-question-remint-precondition-trust-only-001
title: VT-6's documented-procedure branch asks a program to trust a document it could check
kind: open_question
status: superseded
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
  Resolved 2026-09-29: the in-process check exists, placed where it can work rather than where this
  atom looked. Every append re-reads the file's persisted identity inside its own BEGIN IMMEDIATE and
  refuses a stale handle with IdentityMoved (red in 8900697, green in f719b2a). A guard inside
  remint_identity cannot work and was not built. An unwarranted re-mint cannot break VT-6's
  uniqueness MUST. Whether VT-6 obliges every adapter to carry the check stays with the clause, which
  phase 13 owns. The gap this question sat beside is a new atom,
  kb-open-question-postgres-neon-store-id-no-restore-001.
depends_on: []
related:
  - kb-decision-0014
  - kb-decision-0022
  - kb-open-question-postgres-neon-store-id-no-restore-001
source_paths:
  - .kb/_intake/2026-09-03-pre-publication-review.md
  - references/evaluation/review-pre-publication-2026-09-03.md
  - crates/happenstance-sqlite/src/event_store.rs
  - spec/SPECIFICATION.md
  - crates/happenstance-sqlite/tests/migration.rs
  - references/adapter-shapes.md
last_reviewed: 2026-09-29
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

## Closed — 2026-09-29

**The in-process check exists, and it is not where this question looked for it.** Commit
`8900697` wrote the failing test first:
`an_append_through_a_handle_the_file_has_outgrown_is_refused`
(`crates/happenstance-sqlite/tests/migration.rs:501`). Commit `f719b2a` made it pass. `append_locked`
(`crates/happenstance-sqlite/src/event_store.rs:680-743`) re-reads the file's persisted identity
inside the append's own `BEGIN IMMEDIATE`, before any guard is probed. When the handle's identity
differs, it refuses with `SqliteEventStoreError::IdentityMoved` and names both incarnations. The
check is one indexed read on a four-row table, under a lock the writer already holds. Both commits
landed on 2026-09-04, the day this atom was last reviewed, and it was never re-read against them.

This atom's "What is not decided" paragraph looked for the check inside `remint_identity`. The doc
at `:700-708` records why it cannot go there. A process-wide open-path registry needs a canonical
path key, and symlinks, hardlinks, `file:` URIs, UNC paths and two paths to one inode all defeat
one. It also sees nothing when the re-mint happens in another process. The candidates listed above
— a cached last-known identity, a generation counter, an attestation from whatever did the restore
— would each be new state carrying its own durability claim, and none of them is needed. The
persisted row is the thing to check against, and the append is the moment the check matters.

**What the check does not do, and why that is sound.** It does not ask whether a re-mint was
*warranted*. An unwarranted re-mint cannot violate VT-6's uniqueness MUST
(`spec/SPECIFICATION.md:840-844`). It mints a new `StoreId`, positions keep rising under
`AUTOINCREMENT` (`event_store.rs:109`), and no `(StoreId, SequencePosition)` pair is ever issued
twice. That corrects the claim under "What forces it" above, that such a re-mint "reissues
positions a peer has already seen under a new identity". Applied to a live file, it reissues
nothing: its positions carry on from where they were. What a peer does see is one origin changing
name mid-stream. That is a replication cost and belongs to phase 13; it is not a
VT-6 violation. The case VT-6 actually fears is the *missing* re-mint after a restore or a copy,
and no in-process check can see that. VT-6's `Rejects:` paragraph accepts that case explicitly, and
`references/adapter-shapes.md:395-401` states the operator's procedure.

**Not answered here:** whether VT-6's documented-procedure branch *obliges* every adapter that
takes it to carry an equivalent check. That is the clause-level form of this question, which
`f719b2a`'s message briefed and did not answer. VT-6 is `[PROVISIONAL]`, phase 16's disposition
table gives it to phase 13 to freeze, and phase 13 owns this question along with the clause.

**The gap this question sat beside.** Reading the branch again turned up two published adapters
that take neither of its arms. `happenstance-postgres` and `happenstance-neon` mint once, detect
nothing, and document no re-mint. That is its own question,
`kb-open-question-postgres-neon-store-id-no-restore-001`, and phase 13 owns it.

Closed by hand in phase 16. No accepted decision was edited.
