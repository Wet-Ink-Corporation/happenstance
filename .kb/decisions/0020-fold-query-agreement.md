---
id: kb-decision-0020
title: A decision model folds a domain enum, and its query is derived on a sealed trait the caller cannot override
kind: decision
status: accepted
authority_tier: decision
adr_id: ADR-0020
reversibility: medium
phase: 7
supersedes: null
superseded_by: null
summary: >-
  A DCB handler names its event set twice - once in the query, once in the fold - and nothing
  checks the two agree; the worked example does exactly this today and the gate is green either
  way. The resolution is structural rather than detective: query() is not on DecisionModel at
  all. The derivation is Boundary::query, on a sealed trait blanket-implemented for every
  DecisionModel and macro-implemented for tuples of arity 2..=8, so there is nowhere to put a
  hand-maintained query. It returns Result<Query, InvalidQuery> because QueryItem::new is
  fallible and happenstance-core is frozen; the other route to that error, an empty EVENT_TYPES,
  is turned into a compile error by a const item evaluated per-monomorphisation rather than left
  to the first read. DecisionModel::scope(&self) -> &Tags, so the validation is paid once in the
  constructor the caller already writes, Tags::from_pairs being the only and fallible way in.
  DecisionModel: Clone, not Default, so the command loop re-folds from the pristine model on
  retry without a default-constructible Tags re-opening the invalid-value hole. The residual Err
  is kept and re-read as "this boundary constrains nothing, which is Query::all() and must be
  said out loud"; commit and commit_with absorb it so the first program writes no extra
  question-mark. No unwrap and no edit to the frozen crate: the shortfall is logged as defect
  candidate D-1, that happenstance-core has no infallible QueryItem constructor for pre-validated
  inputs, with VT-18 as its nearest clause subject and a decision record as its route. Rejected:
  a provided method on DecisionModel (overridable, so hand-maintained again); a free
  derive_query::<M>() (ignorable); an infallible query() (unreachable without an unwrap);
  fallible-without-the-const-assertion (defers a compile-time-decidable mistake to the first
  read); scope() -> Tags by value (unwrap inside an infallible signature); scope() -> &[(&str,
  &str)] (revalidates, moves the error to the read); a Default supertrait (re-opens the invalid
  Tags hole); an exported compose! macro (caller-visible ceremony where a tuple would do).
  DT-2 resolves toward explicit declaration, and the price is on the record: 2.4:1 mapping
  ceremony to domain logic in the first program, carried as the falsifiable prediction that
  AC-013's verdict lands "happenstance-macros is in scope for 0.1" - a consequence, not a second
  decision.
depends_on:
  - kb-decision-0003
  - kb-decision-0006
related:
  - kb-decision-0007
  - kb-decision-0015
  - kb-decision-0021
  - kb-open-question-projection-id-unvalidated-001
source_paths:
  - .kb/_intake/0020-fold-query-agreement.md
  - references/adr/0020-fold-query-agreement.md
  - .bklg/from-contract-to-published-library/typed-layer-and-alpha-release/_design.md
  - crates/happenstance-core/src/query.rs
  - crates/happenstance-core/src/tag.rs
  - examples/course-subscriptions/src/main.rs
last_reviewed: 2026-08-15
---

# A decision model folds a domain enum, and its query is derived on a sealed trait the caller cannot override

## Context

A DCB handler names its event set twice: once in the `Query` that scopes the read, once in the
fold that interprets what comes back. Nothing checks the two agree. `examples/course-subscriptions/
src/main.rs:114-125` builds a `Query` naming `COURSE_DEFINED`, `STUDENT_SUBSCRIBED` and
`STUDENT_UNSUBSCRIBED`; the fold at `:140-155` names the same three a second time, with a `_ => {}`
arm absorbing any difference. Neither the compiler, clippy, nor the conformance suite signals a
divergence. A query selecting a type the fold ignores only widens the boundary — spurious
conflicts, safe. A fold interpreting a type the query never selected narrows the log the decision
is made on: the append condition protects less than the handler assumes, and that direction
corrupts.

## Decision

`query()` is not a method on `DecisionModel` at all, so there is nowhere to put a hand-maintained
one. The derivation lives on `Boundary::query`, a **sealed** trait blanket-implemented for every
`DecisionModel` and macro-implemented for tuples of arity 2..=8 by an internal `macro_rules!`
(zero caller-visible syntax; the ceiling is 8 because rustdoc renders one impl block per arity).
The seal is what keeps this the *only* route (RS-40-2).

`Boundary::query(&self) -> Result<Query, InvalidQuery>` is fallible because `QueryItem::new`
(`crates/happenstance-core/src/query.rs:56`) is fallible and `happenstance-core` is frozen — an
infallible `query()` is unreachable without an `unwrap` in library code. The other route to
`InvalidQuery`, an empty `EVENT_TYPES`, is closed at compile time instead of left to the first
read: a `const` item evaluated per-monomorphisation turns it into a build failure naming the
event-type set (RS-61-4).

`DecisionModel::scope(&self) -> &Tags` — the model *holds* validated tags, because
`Tags::from_pairs` (`crates/happenstance-core/src/tag.rs:304`) is fallible and is the only way in.
Validation is paid once, in the constructor the caller already writes, not on every read.
`DecisionModel: Clone`, not `Default`: the command loop re-folds from the pristine model on retry,
and a default-constructible `Tags` would re-open the invalid-value hole `from_pairs` exists to
close.

The residual `Err` is kept, not eliminated, and re-read as `InvalidQuery::UnconstrainedItem` —
"this boundary constrains nothing, which is `Query::all()` and must be said out loud." `commit`
and `commit_with` absorb it into `CommandError::Boundary`, so the first program written against
this API adds no extra question-mark for it. No `unwrap`, and no edit to the frozen crate: the
shortfall — `happenstance-core` has no infallible `QueryItem` constructor for pre-validated
inputs — is logged as defect candidate D-1 for AC-012's log, with VT-18
(`spec/SPECIFICATION.md:1371-1375`) as its nearest clause subject and a decision record, not a
line edit, as its route.

The exactly-one-path claim is the contract crate's own words, at
`crates/happenstance-core/src/projection.rs:152-154`: "two constructors enforcing different rules
is the defect that makes an invalid value reachable through the weaker one." `ProjectionId::new`
was decided against that reasoning; `Boundary::query` is built out of the same premise applied to
`Query`.

## Consequences and alternatives rejected

`EVENT_TYPES` ↔ `event_type()` agreement is not compiler-enforced and is tested by
`assert_domain_event::<E>(&[…])` — the residual is named rather than hidden. DT-2 resolves toward
**explicit declaration**, and its price is measured in the signed-off first program: 11 lines of
domain logic to 26 of mapping ceremony, 2.4:1, carried forward as the falsifiable prediction that
AC-013's verdict lands "`happenstance-macros` is in scope for 0.1" — a consequence stated here,
not a second decision; the record itself asserts no *must* about the derive.

Rejected: a **provided method** on `DecisionModel` (overridable, so hand-maintained again, and
DR-02 becomes a convention rather than a guarantee); a free **`derive_query::<M>()`** (ignorable —
the caller can pass their own query to the read instead); an **infallible `query()`** (unreachable
without an `unwrap`); **fallible with no `const` assertion** (defers a compile-time-decidable
mistake to the first read while still charging explicit declaration's ceremony); **`scope() ->
Tags` by value** (a fallible construction inside an infallible signature, forcing an `unwrap`);
**`scope() -> &[(&str, &str)]`** (revalidates on every read and moves the error there); a
**`Default` supertrait** (re-opens the invalid-`Tags` hole); an exported **`compose!` macro**
(caller-visible ceremony where a tuple would do).

Two citations were repaired before staging, not amended: the two-constructor sentence is at
`projection.rs:152-154`, not the `:47-61` still carried in `.bklg`'s decomposition and design
artifacts; `RUNBOOK.md`'s AC-013 row is at `:525`, not `:524`. Neither changes the admitted set of
implementations.
