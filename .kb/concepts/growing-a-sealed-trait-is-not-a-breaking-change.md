---
id: kb-concept-sealed-trait-growth-001
title: Growing a sealed trait cannot produce E0046 downstream
kind: concept
status: accepted
authority_tier: note
summary: >-
  Adding a required item to a sealed trait cannot break a downstream crate with
  E0046, because no downstream crate contains an impl for the item to be missing
  from — every implementor reaches the trait through this crate's blanket impl,
  which supplies the new item with the rest. RS-40-1's E0046 claim is true, and
  true only of an unsealed port a stranger implements. Two residual costs
  survive: a new method name can create resolution ambiguity at a call site, and
  a new associated type becomes nameable in signatures that must then keep it.
depends_on:
  - kb-decision-0020
related:
  - kb-open-question-tuple-boundary-event-type-001
  - kb-decision-0044
source_paths:
  - .kb/_intake/remediation-2026-09-04-briefs/tuple-boundary-event-type.md
  - standards/rust/40-public-surface-and-evolution.md
last_reviewed: 2026-09-07
---

# Growing a sealed trait cannot produce E0046 downstream

## The claim

`crates/happenstance/src/boundary.rs:70-75` states, as the reason `Boundary`
carries `type Event: DomainEvent` from birth rather than adding it later: "the
trait is sealed and the crate publishes from it, and growing a sealed trait a
required item is a breaking change no downstream crate could have prepared for."
That sentence has the direction backwards. Sealing is precisely what makes
growing the trait *free*, not what makes it costly — the comment spends the
seal's dividend as though it were the seal's price.

## Why the direction is backwards

`RS-40-1` (`standards/rust/40-public-surface-and-evolution.md:14-18`) states the
real hazard correctly for the case it governs: "a new required method is
`error[E0046]` in every adapter's crate, on a minor bump" — true of a port like
`EventStore`, which a stranger implements directly. `E0046` fires because an
existing `impl Trait for MyType` in a downstream crate is now missing the newly
required item, and the compiler has no way to supply it.

That failure mode requires a downstream `impl` to exist in the first place. A
sealed trait has none. `Boundary`'s seal (`crates/happenstance/src/sealed.rs`) is
a `pub` trait inside a private module — nameable as a supertrait bound from
inside the crate, unnameable from outside it — and every `impl Boundary` in
existence lives inside `happenstance` itself: the blanket impl for every
`DecisionModel` and the seven macro-generated tuple impls. A downstream type
never writes `impl Boundary for MyType` directly; it reaches `Boundary` only
through the crate's own blanket impl, and that blanket impl is what the crate
updates when the trait grows. Adding `type Event` to `Boundary` after the fact
would have meant editing one blanket impl and eight macro-generated ones, in this
crate, in one commit — no downstream crate contains an `impl` for the new item to
be absent from, so `E0046` cannot occur there. This is the exact inverse of
`RS-40-1`'s case, not a restatement of it.

## What still costs something

Two residuals are real and worth naming precisely because they are easy to
conflate with `E0046`:

- **A new method name** can create a method-resolution ambiguity at a downstream
  call site that already has another trait of the same method name in scope —
  not a compile error at the trait boundary, but a possible one at a call site
  that becomes ambiguous between two traits.
- **A new associated type** becomes nameable the moment it exists (`B::Event`),
  and any signature written against it afterward must keep naming it — the type
  does not retract once code depends on it.

Neither is `E0046`, and neither is what the doc comment claims. The distinction
matters operationally: a maintainer who applies the doc comment's stated rule
after a stability boundary declines an addition that is in fact free, and a
reviewer citing the comment blocks a relaxation that costs nothing to make.

## The general lesson

Before pricing a change to a trait as breaking, ask who holds the `impl`. If
every implementor is inside the crate that defines the trait — which sealing
guarantees — growing the trait with a required item is the crate's own
refactor, not a promise made to a stranger. The place to look for the real
residual cost is not "does this break `impl`s" but "what does the new item make
newly nameable, and does anything downstream now have to keep naming it."
