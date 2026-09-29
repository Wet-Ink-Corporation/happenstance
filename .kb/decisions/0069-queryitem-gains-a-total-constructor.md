---
id: kb-decision-0069
title: QueryItem gains a total constructor, and the outward face of D-1 is closed as intended
kind: decision
status: accepted
authority_tier: decision
adr_id: ADR-0069
reversibility: medium
phase: 16
supersedes: null
superseded_by: null
summary: >-
  The record ADR-0020 owed for its defect candidate D-1 (not the runbook's D-1, which puts sync
  inside 1.0). happenstance-core gains an infallible QueryItem constructor that takes at least one
  EventType by value — a first type as its own parameter, any further types, and a Tags — and
  returns QueryItem rather than Result. It is total by construction: QueryItem::new has exactly one
  failure left once its inputs are already-built EventTypes, UnconstrainedItem when types and tags
  are both empty, and a constructor whose signature cannot receive zero types cannot reach it. It
  enforces the same rule as new, not a different one, which is what answers ADR-0020's refusal of
  "a second constructor enforcing different rules"; it deduplicates and sorts exactly as new does, so
  the two produce equal items from equal inputs. Additive: it can ride 0.4.0 and is built in phase
  17. The outward face — DomainEvent::tags is total while string to Tag is fallible — is closed as
  working as intended: Tags: FromIterator<Tag> is already infallible and Tag::from_static exists, so
  the only fallible step left is validating a runtime string, which is validation rather than a
  missing path, and the worked example's validate-at-construction newtype is the intended shape.
  Because no infallible string-to-Tags path is added, ADR-0033's reopen condition does not fire and
  AC-013's measurement is not re-taken. Rejected: an unchecked or token-guarded constructor (ADR-0020
  refused it by name), a NonEmpty wrapper type (a new public type to buy what a first parameter buys
  free), and documenting the Result as unreachable (it leaves every caller writing an arm that
  cannot run). Closes kb-open-question-d-1-no-total-path-001.
depends_on:
  - kb-decision-0020
related:
  - kb-decision-0066
  - kb-decision-0033
  - kb-decision-0015
  - kb-open-question-d-1-no-total-path-001
  - kb-reference-macros-ceremony-measurement-001
source_paths:
  - references/adr/0020-fold-query-agreement.md
  - crates/happenstance-core/src/query.rs
  - crates/happenstance-core/src/tag.rs
  - crates/happenstance-core/src/event.rs
  - crates/happenstance/src/boundary.rs
  - examples/course-subscriptions/src/main.rs
  - spec/SPECIFICATION.md
last_reviewed: 2026-09-29
---

# QueryItem gains a total constructor, and the outward face of D-1 is closed as intended

## Which D-1

This is ADR-0020's **defect candidate D-1**
(`references/adr/0020-fold-query-agreement.md:268-293`, and "D-1's resolution" among the things
that record did not decide, `:370`). It is not the runbook's D-1, which is the owner's decision that
`happenstance-sync` is inside 1.0. The two share a label by accident, and this record spells the
first out in full wherever it names it.

## Decision

**Inward face: a total constructor.** `happenstance-core` gains a `QueryItem` constructor that takes
**at least one `EventType` by value** — the first type as a parameter of its own, any further types
as an iterator, and a `Tags` — and returns `QueryItem`, not `Result<QueryItem, InvalidQuery>`. The
shape is decided here. The spelling is phase 17's to choose when it compiles it, and one candidate
is:

```rust,ignore
impl QueryItem {
    pub fn of(first: EventType, rest: impl IntoIterator<Item = EventType>, tags: Tags) -> Self;
}
```

**Why it is total, read against the code.** `QueryItem::new`
(`crates/happenstance-core/src/query.rs:56-76`) can fail two ways. One is a type string that does
not validate (`InvalidQuery::EventType`), and that failure is already `Infallible` when the caller
passes built `EventType`s: `EventType -> EventType` is the reflexive `From<T> for T`, which the
blanket `TryFrom` impl turns into a conversion whose error is `Infallible`. The other is
`InvalidQuery::UnconstrainedItem`, when types and tags are both empty (`:68-70`). A signature with
a separate `first: EventType` parameter cannot be called with zero types, so that branch is
unreachable for every call that compiles. Nothing is asserted at run time. The non-emptiness is in
the signature.

The Rust point is worth stating, because the alternatives look similar. A `&[EventType]` or an
`impl IntoIterator` parameter says nothing about length, so a total function over one would have to
panic, return a `Result`, or accept an empty item. Splitting off the head is the standard way to
make "at least one" a type-level fact without introducing a type. `NonEmpty<T>` crates are the same
idea wrapped in a struct.

**Why this is not the constructor ADR-0020 refused.** ADR-0020 refused an infallible constructor
as "a second constructor enforcing different rules" (`:287-289`). This one enforces the **same**
rule: an item must constrain something. It enforces it through the signature rather than a branch,
and it canonicalises (sort and dedup) exactly as `new` does, so equal inputs produce `==` items
through either door. ADR-0020 also refused it as "an unrecorded change to a frozen contract". This
record is the recording.

**VT-18 is not edited.** VT-18 (`spec/SPECIFICATION.md:1426-1432`, `[FROZEN]`) requires
constructors to accept values the caller already holds and requires their errors to compose. The
new constructor extends that courtesy to `QueryItem`. It adds no MUST and moves no marker.

**Outward face: closed as working as intended, with no API added.** `DomainEvent::tags` returns
`Tags` totally. The atom's worry was that every route into `Tags` is fallible, but at HEAD
`Tags: FromIterator<Tag>` is infallible (`crates/happenstance-core/src/tag.rs:479-487`) and
`Tag::from_static` is a `const fn` for literals (`:112`). The one fallible step left is turning a
**runtime string** into a `Tag`. That step is validation, and no total path should exist for it,
because a total path would be the unvalidated value the type exists to exclude. The worked example
holds the validated form in a newtype and pays the `?` once, at construction
(`examples/course-subscriptions/src/main.rs:85-88`; the comment's first sentence, at `:83-84`, that
`Tags` has no infallible constructor, predates this record and is stale, and phase 17 corrects it).
That is the intended shape,
and it is the same resolution `DecisionModel::scope` makes for a model's own tags.

## What it touches, and what it does not

- **Additive.** A new inherent method on a public type is a minor change. It can ship in `0.4.0`,
  it is built in **phase 17** alongside that window's other `happenstance-core` work, and it does
  not belong to phase 17's breaking list.
- **`Boundary::query` keeps its `Result`.** The typed layer's `derive_query`
  (`crates/happenstance/src/boundary.rs:172-175`) is only reached after `AtLeastOneType::CHECKED`
  has asserted at compile time that `EVENT_TYPES` is non-empty (`:159-163`). It could move to the
  new constructor via `split_first`, but the slice it receives is still a slice. Dropping the
  `Result` from `Boundary::query`'s published signature would be a break in `happenstance`, and it
  is not decided here.
- **ADR-0033 does not reopen.** Its condition is "if and only if D-1 is settled with an infallible
  `Tags` path" (`.kb/decisions/0033-happenstance-macros-out-of-scope-for-0-1.md:120-121`). This
  record deliberately does not add an infallible string-to-`Tags` path, so AC-013 stays closed and
  `kb-reference-macros-ceremony-measurement-001` is not re-taken.

## Alternatives rejected

- **An unchecked constructor (`new_unchecked`) or a `Validated` token.** ADR-0020 refused this by
  name. It would enforce a different rule, or none, and the item it lets someone write is an
  unconstrained `QueryItem` that silently reads as `Query::all()`.
- **A `NonEmpty`-style wrapper for both `QueryItem` and `Query`.** A new public type is permanent
  surface. It would buy what a separate first parameter buys for free, and callers would first
  have to build the wrapper, fallibly.
- **Do nothing and document the `Result` as unreachable.** This leaves every caller holding
  validated inputs writing an error arm that cannot run. `Boundary` is sealed, so no downstream
  test could ever exercise that arm, which is the strongest form of the complaint in the
  open question.
- **An infallible string-to-`Tags` path for the outward face.** This is the one thing the outward
  face could have asked for, and it would admit the unvalidated tag.

## Falsifier

Reopen if a caller that holds validated inputs still cannot build a `QueryItem` totally, for
example a boundary whose constraint is **tags only**. The new constructor does not cover that case
(`QueryItem::tagged` stays fallible on an empty `Tags`), and if a real model needs it, a `Tags`
non-emptiness guarantee is the next record's subject. Reopen the outward face if a domain turns up
whose tag values cannot be validated at the point the event is constructed.
