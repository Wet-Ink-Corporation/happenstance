---
id: kb-decision-0012
title: append keeps its borrowed batch, and phase 4 declines what it cannot measure
kind: decision
status: accepted
authority_tier: decision
adr_id: ADR-0012
reversibility: low
phase: 4
supersedes: null
superseded_by: null
summary: >-
  append keeps events: &[Event]; lifting that clause to frozen is declined this phase because
  the question is a measurement phase 8 owns, and the decision states the five things that
  measurement must produce, naming Vec<Event> as the only surviving successor if it fires. An
  empty batch is refused before any condition is evaluated. A condition is evaluated only
  against events the store already held when append began, never against the batch being
  appended, so a batch can never conflict with itself. Dropping the future does not cancel the
  append: an adapter may have committed before or after the drop, a caller must not treat a
  drop as evidence the append did not land, and the store is left fully applied or unchanged.
  A verbatim reissue of a conditional, self-matching batch lands at most once; an unconditional
  reissue lands twice; a reissue conditioned on other events also lands twice — the third case
  is new here and makes a payload-deduplicating store non-conformant on purpose. append returns
  the position of the batch's last event, assigned in slice order and strictly ascending but
  not necessarily contiguous; it is not a sound append-condition boundary, only a read is.
  conflicting_position is a hint that may be absent and must not be parsed out of a message.
  AppendCondition becomes a non-empty sequence of Guard values behind a private field and a
  guards() accessor, because a public field was compiled to reach zero guards from downstream
  in one line, which silently makes a conditional append unconditional. CF-39 is minted for
  mid-batch fault injection: a fixture declaring the capability must make the fault land inside
  the store's own write path, and a fixture whose store absorbs every fault it can arm must
  decline instead of passing vacuously.
depends_on:
  - kb-decision-0010
related:
  - kb-concept-torn-read-append-boundary-001
  - kb-playbook-repair-frozen-clause-001
source_paths:
  - .kb/_intake/0012-append-shape-and-preconditions.md
  - references/adr/0012-append-shape-and-preconditions.md
  - crates/happenstance-core/src/append.rs
  - crates/happenstance-core/src/store.rs
  - crates/happenstance-testkit/src/fixtures.rs
  - spec/SPECIFICATION.md
last_reviewed: 2026-08-10
---

# append keeps its borrowed batch, and phase 4 declines what it cannot measure

## Decision

`EventStore::append` keeps `events: &[Event]`. The RUNBOOK's own phase-4 instruction asked for
a by-value signature, but `SPECIFICATION.md` says the opposite and wins under this
repository's rule that a frozen clause outranks a phase body. The clause carrying `&[Event]`
stays `[PROVISIONAL]` rather than lifting to `[FROZEN]`, because the evidence its own marker
asks for — an adapter with a real write path and a benchmark harness, measured under a
realistic batch and rejection mix — does not exist yet; nothing in the workspace can produce
it before phase 8 builds the SQLite adapter's multi-row insert benchmark. A phase that decides
a measurement question without the measurement has not decided it, it has guessed and frozen
the guess.

The two alternatives the RUNBOOK offered are both independently dead rather than merely
unmeasured. `impl IntoIterator<Item = Event>` is a generic method, and a generic method has no
single address a `dyn EventStore` vtable can hold — `dynosaur`'s erasure fails on it with
`error[E0191]`, foreclosing runtime store selection permanently. An owned `EventBatch` newtype
is forbidden in terms by an existing frozen clause, which rejects a constructor placed in front
of every application's `append` call site to enforce a bound that is adapter-specific. So the
question phase 8 inherits is binary — keep the borrow, or move to `Vec<Event>` — and the
successor is named now so a later measurement cannot manufacture a third option.

Four frozen clauses are transcribed into checkable rules rather than re-argued: an empty batch
is refused before any condition is evaluated; a condition never considers the events in the
batch it is evaluating, only what the store already held; a dropped append future leaves no
partial batch, though the caller-facing cancellation guarantee itself has no rule and cannot
have one, because a fixture cannot observe what happens after a future is silently dropped; and
reissue is at-most-once only where the condition's query matches the batch's own events, with
the two weaker shapes now given rules of their own so a store cannot quietly strengthen a
guarantee the contract disclaims.

`AppendCondition` moves to a non-empty sequence of `Guard { query, after }` values. The
sequence's field is private with a `guards()` accessor rather than public, because a public
field was compiled and found to let downstream code write `c.guards = Box::new([])` — a
conditional append that silently becomes unconditional, with no compiler diagnostic anywhere.
`is_violated_by` becomes a fold across guards: violated if any guard is violated.

## Provisional

`AppendCondition::guards` narrows what two frozen-adjacent clauses currently describe, and is
recorded as a human-confirmed amendment for that reason. CF-39's requirement that an armed
fault make `append` return `Err` narrows an existing frozen rule's permitted outcomes in
practice, even though it changes no MUST about stores directly — an existing frozen rule
permitted either outcome from a faulted store, and CF-39 makes the `Ok` branch of that rule
unreachable for any fixture declaring the capability. Both were signed off explicitly rather
than accepted by default.

## Alternatives rejected

Lifting the borrow clause to frozen on its existing three grounds was rejected because those
grounds were already in the clause; lifting on no new evidence moves a maturity marker because
a phase wanted it moved, which is the one thing a marker must never do. Demoting
`conflicting_position` to nothing was rejected because a compiled Postgres strategy keeps the
field usefully; `Option` already spares the adapters that cannot afford it.
