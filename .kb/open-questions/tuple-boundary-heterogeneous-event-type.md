---
id: kb-open-question-tuple-boundary-event-type-001
title: Every tuple Boundary is bound to its first member's event type, and no signed-off record says so
kind: open_question
status: accepted
authority_tier: note
summary: >-
  crates/happenstance/src/composition.rs's macro-generated tuple impls bind every member after the
  first to the first member's Event associated type, forcing a caller composing over two different
  domain enums into either merging their enums (which widens every member's derived query and
  breaks fold exhaustiveness) or dropping to happenstance-core. Neither ADR-0020's accepted decision
  text nor the signed-off design for the same phase names Boundary::Event at all, and the two
  signed-off artifacts contradict each other about it; the constraint was resolved in code by a
  later commit and never recorded anywhere. A companion defect is settled and not part of this
  question: boundary.rs's doc comment states the sealed-trait growth rule backwards, claiming a
  required addition to a sealed trait is breaking when sealing is exactly what makes it free — that
  correction is owed under every option and is not contested. Four options were evaluated
  (keep-and-document, relax the bound, split the trait, or add an unconstrained wrapper type) and a
  recommendation was reached at medium confidence, but the brief was explicit that it was not
  ratified: the underlying product question — whether one DCB boundary may honestly span two bounded
  contexts — has never been tested by anything in this workspace, and taking the recommended option
  requires deciding that deliberately rather than by default.
depends_on: []
related:
  - kb-decision-0020
  - kb-concept-sealed-trait-growth-001
source_paths:
  - .kb/_intake/remediation-2026-09-04-briefs/tuple-boundary-event-type.md
  - crates/happenstance/src/composition.rs
  - crates/happenstance/src/boundary.rs
  - references/adr/0020-fold-query-agreement.md
last_reviewed: 2026-09-07
---

# Every tuple Boundary is bound to its first member's event type, and no signed-off record says so

## What is true today

`crates/happenstance/src/composition.rs` (`:29-31`, identically `:39-41`) constrains every tuple
member after the first to `Boundary<Event = <$first as $crate::Boundary>::Event>` — a composed
boundary's members must all fold the same domain event enum. The module's doc comment states only
the arity ceiling (eight, a rendering decision) and never uses the word `Event`; its worked example
composes two models over one shared enum, so the constraint has never been exercised, let alone
documented, anywhere in the tree.

Neither of the two artifacts that should carry this decision does. `references/adr/0020-fold-query-
agreement.md`'s own Decision text declares `Boundary` with `query` and `absorb` only — no
`type Event`. The signed-off design for the same phase (`_design.md:421-435`) repeats that shape and
then, thirty lines later, specifies the command loop using `B::Event` on the same trait it just
declared without one. The two signed-off artifacts contradict each other, and a later commit
(`996853f`) resolved the contradiction by adding `type Event: DomainEvent` to `Boundary` plus the
homogeneity bound above — silently, as an implementation detail rather than a decision anyone took.

The two escapes the (missing) documentation would recommend both cost something real. Merging two
bounded contexts into one enum widens every member's derived query to the union of both contexts'
event types — because a member's query is derived from its event type's *whole* declared set — which
widens the composed append condition and destroys the exhaustiveness pressure `DecisionModel::apply`
is built around; the cost is already visible inside a single context in the canonical example, where
one arm's match falls back to `{}`. Dropping to `happenstance-core` forgoes the typed layer entirely.

A separate, uncontested finding rides along: `boundary.rs`'s doc comment states the sealed-trait
evolution rule backwards, framing "present from birth" as necessary because growing a sealed
trait's required items would be breaking — when sealing a trait for a crate's own blanket
implementation is exactly what makes adding a required item *free*, since no downstream crate holds
an `impl` for the compiler to reject. This correction is owed under every option below and is not
part of the contested question.

## What is not decided

Four options were laid out: **(A)** keep the homogeneity bound, document it, and fence it with a
`compile_fail` doctest — free, forever, forecloses only the relaxation itself becoming non-breaking
after `0.2.0`. **(B)** relax the where-clause so a composed boundary's `Event` is simply the first
member's — a breaking change whose caller cost is a silent, order-dependent meaning with no name in
the signature. **(C)** split `Boundary` into a read-only trait and a second sealed trait carrying
`Event`, bound only where `commit` needs it — breaking, and still leaves a heterogeneous tuple unable
to commit, so it pays a break without finishing the capability it targets. **(D)** add a new,
unconstrained wrapper type with a caller-chosen `Event`, delegating to untouched tuple members —
semver-additive at any date, but a second caller-visible composition syntax beside the bare tuple,
which ADR-0020's own text licenses out of existence ("exactly one path is licensed... no second
composition syntax"); taking D therefore costs a superseding atom against `kb-decision-0020`, not a
`related` link, regardless of when it is taken.

Option A was recommended at medium confidence, on the ground that nothing in this workspace has ever
needed heterogeneous composition and that D remains available afterward at its own governance price
rather than being foreclosed. The recommendation was explicit that it must be taken deliberately: the
real question underneath is a product one — whether a single DCB consistency boundary may honestly
span two bounded contexts — and nothing in the tree has tested it either way.

## What forces it

A caller who actually needs to compose two different domain enums into one boundary. Options A and
D are free before and after `0.2.0`; B and C are free now and become a major-version-equivalent
break once the `0.2.0-alpha.1` movement licence is spent (`RUNBOOK.md`'s phase 12 publishes stable
`0.2.0`). The one item free at any date and worth doing regardless of which option is eventually
taken is the doc-comment correction on `boundary.rs`'s sealed-trait rationale, because it appreciates
in cost the longer it stands uncorrected and could be cited to block a genuinely free change.
