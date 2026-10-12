---
id: kb-decision-0090
title: One boundary folds one domain enum, and a tuple over two is fenced out at compile time
kind: decision
status: accepted
authority_tier: decision
adr_id: ADR-0090
reversibility: low
phase: 17b
supersedes: null
superseded_by: null
summary: >-
  Answers kb-open-question-tuple-boundary-event-type-001 with option A, taken deliberately as the
  owner's call (wi-378078, "all as recommended", 2026-10-12). A composed Boundary's members all
  fold the first member's Event: the homogeneity bound on the macro-generated tuple impls in
  crates/happenstance/src/composition.rs stays as it is, and is now a decision rather than an
  implementation detail commit 996853f introduced without a record. The product question under it
  is answered in the typed layer: one DCB consistency boundary folds one domain enum. Two bounded
  contexts that must be checked by one append condition are one enum, or are decided at the
  contract layer in happenstance-core, whose Query and AppendCondition carry no event type. A
  compile_fail doctest on the two-arity tuple impl fences the heterogeneous shape. It composes a
  model over one enum with a model over another; each is a Boundary alone, and the tuple is a
  single E0277, measured, and red when the bound is deleted. The example above it is its compiling
  twin. Relaxing the bound so the composite's Event is the first member's (option B) and splitting
  the trait (option C) are declined; either now needs a record that supersedes this one. An
  unconstrained wrapper type with a caller-chosen Event (option D) stays additive at any date, at
  the governance price the open question named, a record superseding kb-decision-0020's "no second
  composition syntax". The backwards sealed-trait rationale on Boundary::Event, owed under every
  option, is corrected in the same change.
depends_on:
  - kb-decision-0020
  - kb-decision-0066
related:
  - kb-decision-0020
  - kb-decision-0066
  - kb-decision-0072
  - kb-concept-sealed-trait-growth-001
  - kb-open-question-tuple-boundary-event-type-001
  - kb-decision-wi-378078
source_paths:
  - crates/happenstance/src/composition.rs
  - crates/happenstance/src/boundary.rs
  - .kb/open-questions/tuple-boundary-heterogeneous-event-type.md
  - references/adr/0020-fold-query-agreement.md
last_reviewed: 2026-10-12
---

# One boundary folds one domain enum, and a tuple over two is fenced out at compile time

**Accepted by the owner on 2026-10-12** as `kb-decision-wi-378078` ("all as recommended", for
#93's "option A on both questions"). No long-form record exists; this atom is the record. It
lands as a `door:one-way` change, so the owner merges it.

## Context

`crates/happenstance/src/composition.rs:29-31`, and identically `:39-41`, bind every tuple
member after the first to `Boundary<Event = <$first as Boundary>::Event>`. Neither ADR-0020's
decision text nor the signed-off design names `Boundary::Event`; commit `996853f` added both the
associated type and the bound, and nothing recorded either. The open question
`kb-open-question-tuple-boundary-event-type-001` laid out four options and recommended A at
medium confidence. It asked that A be taken deliberately, because the question beneath it is a
product one: may one DCB consistency boundary span two bounded contexts? ADR-0066 classified it
breaking-if-answered, and ADR-0072 placed its closure in phase 17b.

## Decision

1. **One boundary folds one domain enum.** The homogeneity bound stays. A caller with two bounded
   contexts that must share one append condition has two honest routes. The first is one enum,
   which widens each member's derived query to the union of the enum's event types, a cost that is
   visible in the signature. The second is the contract layer, `happenstance-core`, whose `Query`
   and `AppendCondition` carry no event type.
2. **The shape is fenced.** The two-arity tuple impl's documentation carries a `compile_fail`
   doctest, `a_tuple_over_two_domain_enums_is_not_a_boundary`
   (`crates/happenstance/src/composition.rs:175`, `compile_fail: a_tuple_over_two`). It composes
   `Seats` (over `Seat`) with `Doors` (over `Door`) and passes each alone, and then the pair, to a
   function bounded on `Boundary`. Measured with the fence compiled as plain `rust`, the only
   diagnostic is `error[E0277]: the trait bound (Seats, Doors): happenstance::Boundary is not
   satisfied`, on the tuple line. With the bound deleted from both where-clauses, the fence
   compiles and the doctest fails. The fence is spelled bare `compile_fail` and names `E0277` in
   its prose, as `crates/happenstance/src/domain.rs:65` does, because rustdoc passes a fence whose
   claimed code does not match (RS-62-1). Its compiling twin is the example directly above it,
   two models over one enum composed as a tuple.
3. **The sealed-trait rationale is corrected.** `crates/happenstance/src/boundary.rs:72-74` said
   that `type Event` had to be present from birth because growing a sealed trait a required item
   breaks downstream crates. That is backwards (`kb-concept-sealed-trait-growth-001`): the seal is
   what made the addition free, because no downstream `impl` exists to lack it. The doc now says
   so, and names what does not retract, which is the associated type's name. `commit` spells
   `B::Event`, and the tuple's members must share it.

## Consequences

- No signature, bound or behaviour changes. The published surface is the surface `0.4.1`
  shipped, and the change is documentation, one doctest and this record.
- Options B and C are closed by record, not by the compiler. Either now needs a record that
  supersedes this one.
- Option D, a wrapper type with a caller-chosen `Event` that delegates to its members, stays
  additive at any date. Taking it costs a record superseding `kb-decision-0020`'s "exactly one
  path is licensed … no second composition syntax"
  (`references/adr/0020-fold-query-agreement.md:313-315`), not a `related` link.
- A composite's `Event` is never ambiguous: it is the one enum every member names, so no member
  order carries a meaning the signature hides.

## What reopens it

A caller who must check two domain enums under one append condition, for whom one merged enum
widens the query too far and the contract layer forgoes too much. Nothing in this workspace has
needed it; the canonical example composes over one enum.

## Alternatives that lost

- **B, relax the where-clause.** The composite's `Event` would be the first member's, an
  order-dependent meaning with no name in the signature, and a decision closure over `(A, B)` and
  one over `(B, A)` would return different types.
- **C, split `Boundary` into a read trait and a sealed trait carrying `Event`.** A heterogeneous
  tuple could read and fold, but still could not commit, because `commit` needs one `Event` to
  encode. It pays a break and does not finish the capability it targets.
- **D now.** Additive later at the same price, so there is no reason to pay it before a caller
  asks.
