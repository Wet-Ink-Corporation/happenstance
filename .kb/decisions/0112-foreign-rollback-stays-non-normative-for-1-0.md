---
id: kb-decision-0112
title: A foreign rollback stays non-normative for 1.0, renewed against a store whose rollback acts on store-side state
kind: decision
status: accepted
authority_tier: decision
adr_id: ADR-0112
reversibility: high
phase: 21
supersedes: null
superseded_by: null
summary: >-
  Accepted 2026-11-20 for #115, on the owner's yes to every recommendation (proposed 2026-11-18). ADR-0075 took
  rollback out of PS-15's MUST and left a clause for it as phase 18's option. This record declines
  that option for 1.0. The non-normative sentence under PS-15 stays as it is. Under every in-tree
  shape, rollback consumes an owned batch and drops it, and that is PS-7's drop. The rule phase
  18's box describes (begin on one store, roll back on another, both unchanged) would pass every
  in-tree store and every plausible wrong one, so it would be decorative. The record also names
  the rule that would not be decorative, and renews against the first adapter whose rollback acts
  on store-side state keyed by something other than the batch value. Firing that is additive: one
  new clause, one new rule. So it is permitted past 1.0 under ADR-0066 §2. MemoryProjectionStore's
  missing ForeignBatch check on rollback stays an optional uniformity fix.
depends_on:
  - kb-decision-0075
  - kb-decision-0066
related:
  - kb-decision-0063
  - kb-decision-0062
source_paths:
  - runbook/phases/18-typed-runner.md
  - spec/SPECIFICATION.md
  - crates/happenstance-core/src/projection.rs
last_reviewed: 2026-11-20
---

# A foreign rollback stays non-normative for 1.0, renewed against a store whose rollback acts on store-side state

**Accepted on 2026-11-20 for #115, as recommended; proposed on 2026-11-18.** What follows describes the tree at acceptance; the follow-ups build it. Nothing under "What acceptance
changes" has been applied. This atom is the whole record.

## Context

PS-15 (`spec/SPECIFICATION.md:5969`) is `[FROZEN]`, narrowed by ADR-0075 to `commit` and `reset`.
`rollback` returns `Result<(), Self::Error>` (`crates/happenstance-core/src/projection.rs:867`),
so it cannot carry the port-level `ForeignBatch` variant, and no rule checked that leg
(`.kb/decisions/0075-the-projection-ports-1-0-clauses.md:64-70`). What PS-15 now says about rollback
is non-normative (`SPECIFICATION.md:5944-5953`). Phase 18 carried "a foreign-`rollback` clause" as
an optional box, `runbook/phases/18-typed-runner.md:121-125`, and the phase closed `done` with
that box still unticked.

What is true of every in-tree shape:

- A buffered batch is a value that `rollback(&self, batch)` takes by value and drops.
- A live batch (`LivePostgresProjectionStore`) is a transaction owned by the *other* store's pool,
  and it rolls back on drop (PS-7).
- SQLite, Postgres, live Postgres and Neon also refuse through their own error's `ForeignBatch`.
  `MemoryProjectionStore` does not check (ADR-0075, `.kb/decisions/0075-the-projection-ports-1-0-clauses.md:82-84`).

## The rule phase 18 named would be decorative

The box asks for this rule: begin on store A, roll back on store B, assert that both are unchanged.
Under PS-5 the batch is owned, so the most natural wrong `rollback`, one that does nothing or
drops the batch, *is* the right one. A store that "applied" a foreign batch on rollback would have
to go out of its way to do it, and no adapter shape makes that the path of least resistance. The
CLAUDE.md test applies: a rule that no plausible adapter fails is decorative.

**The rule that would not be decorative.** One adapter shape is plausible and unbuilt: a store
that stages writes on the *server*, in a staging table or a Durable Object's pending rows, keyed by
a per-instance counter, with `rollback` issuing `DELETE … WHERE batch = ?` on its own connection.
Handed A's batch, store B deletes its own unrelated staged rows, because the two counters collide.
The rule that catches it holds an open batch on B:

1. Begin `a` on A, then begin `b` on B and write a row into `b`.
2. Call `rollback(B, a)`.
3. Commit `b` on B, and assert that `b`'s row and checkpoint landed and that A is unchanged.

No in-tree store stages on the server. The Neon projection store buffers on the client, and the
Cloudflare adapter has no projection store.

## Options

1. **Decline for 1.0 and renew past it, naming the trigger and the rule above (chosen).**
2. Mint a clause now with phase 18's rule. Every store passes, it guards nothing, and it adds a
   frozen sentence to the census.
3. Mint a clause now with the sharpened rule. It catches a shape nothing in the tree has, at the
   cost of a testkit minor that every adapter's CI absorbs. It is also additive later, so doing it
   now buys nothing that waiting loses.
4. Decline permanently. This loses the trigger, and the sharpened rule would be rediscovered from
   scratch.

## Decision

1. `rollback` stays outside PS-15's MUST, and the non-normative sentence stands as written.
2. **Renewed past 1.0.** Reopened by the first adapter whose `rollback` acts on store-side state
   that is keyed by something other than the batch value it consumes. The clause that answers it
   carries the three-step rule above, with a mutant built in the same change, `CollidingStagingStore`.
3. Firing the trigger is additive, a new clause and a new rule, so renewal is permitted by
   ADR-0066 §2.
4. Adding the `ForeignBatch` check to `MemoryProjectionStore::rollback` is left optional, as
   ADR-0075 left it. It changes no promise.

## What acceptance changes

- `runbook/phases/18-typed-runner.md:121`: the #115 box is ticked, with *declined for 1.0 by
  ADR-0112, renewed against a store whose rollback acts on store-side state*.
- PS-15's non-normative sentence gains a pointer: *the reopening trigger and its rule are
  ADR-0112's.* This is prose only and admits no different set.
- #115 closes with this record.

No `spec_trace.rs` change, no new rule, and no ledger row. PS-15 is `[FROZEN]`, and the ledger
lists only clauses that are not frozen.

## Door

Two-way. Nothing normative moves, and a later record can mint the clause additively.
