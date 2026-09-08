---
id: kb-decision-0048
title: "Op::Read is non-exhaustive, in the release that already spent the break"
kind: decision
status: accepted
authority_tier: decision
adr_id: ADR-0048
reversibility: medium
phase: 12
supersedes: null
superseded_by: null
summary: >-
  Variant-level non_exhaustive is applied to Op::Read in the same release as the to field,
  because that field's addition already spent the one breaking release the attribute could
  otherwise ride for free — deferring pays the same break twice. Scoped to Op::Read alone:
  Op::Append and Op::AppendConditional are documented as complete and marking them would be a
  reflexive RS-13-5 violation. No constructor is added, because nobody has been shown to build
  an Op by hand. Landed at 9000f35.
depends_on: []
related:
  - kb-open-question-model-family-rule-no-clause-001
source_paths:
  - .kb/_intake/remediation-2026-09-04-briefs/op-read-non-exhaustive.md
  - .kb/_intake/ratifications-2026-09-06-pre-publication.md
  - .kb/_intake/2026-09-07-ratifications-discharged-and-what-execution-changed.md
last_reviewed: 2026-09-07
---

# Op::Read is non-exhaustive, in the release that already spent the break

## Decision

`Op::Read` (`crates/happenstance-testkit/src/model.rs`, `pub enum Op`, reachable behind the
`proptest` feature and therefore part of the crate's published surface at `--all-features`)
carries variant-level `#[non_exhaustive]` as of the same release that added its `to: Anchor`
field. `Op::Append` and `Op::AppendConditional` do not carry the attribute. No constructor was
added for `Op::Read`. Landed at `9000f35`, exactly as ratified: Option A, not A plus a builder.

## Why the field addition forced the timing

Adding `to` to a struct-form variant of a `pub enum` with no `#[non_exhaustive]` at either level
is a compile-breaking change: every downstream `match` on that variant without a trailing `..`,
and every downstream construction, stops compiling. That break was already necessary and already
taken in this release — `Op::Read` mirrors `ReadOptions`, which is itself `#[non_exhaustive]`
upstream in `happenstance-core` and had already grown a `to` field, which is how this hole
arrived: the options struct absorbed a new field without a break, and the model's mirror of it
could not. Applying `#[non_exhaustive]` inside the same breaking release costs the marginal
consumer nothing beyond what the `to` field already cost them, and it is the last release at
which that is true. Deferring it to a later phase — the audit's original routing, to whoever owns
`cargo-semver-checks` at phase 12 — pays the identical breaking cost a second time, against a
population of pinned adapters that by then exists, for a benefit no different from paying it now.
The two field-shaped changes were "both free once" only until one of them landed; once `to`
shipped, that symmetry was gone.

## Why the attribute is not reflexive, and RS-13-5 is not violated

`standards/rust/13-sealing-and-exhaustiveness.md`'s RS-13-5 says not to put
`#[non_exhaustive]` on an enum designed not to grow, because it costs downstream a `_ =>` arm
forever and permanently stops the compiler reporting a forgotten variant. `Op`'s own
documentation states its three variants — an unconditional append, a conditional one, a read —
are the whole port surface by design, which is a correct application of RS-13-5 at the **enum**
level and is left untouched. The distinction this decision turns on is that RS-13-5's mechanism
targets the enum, not one struct-form variant: variant-level `#[non_exhaustive]` costs a
trailing `..` in a struct pattern, not a `_ =>` arm, and matching remains exhaustive over the
three-variant enum. `Op::Read` is a mirror of a struct (`ReadOptions`) that is itself
`#[non_exhaustive]` and has already grown once; `Op::Append` and `Op::AppendConditional` mirror
nothing that grows and are documented as complete, so marking them would be exactly the
reflexive, uncosted application of the attribute RS-13-5 forbids. The asymmetry across the three
variants is the evidence that the attribute was costed rather than applied by habit.

## What the removed ability actually costs

The real price is that a consumer can no longer *construct* an `Op::Read` from outside the
crate — only match one with a trailing `..`. That price was weighed against the crate's own
failure renderer, which prints a minimised `Vec<Op>` on a model-conformance failure and invites a
reader to think about replaying that exact sequence; forbidding construction cuts against that
use. The decision proceeds anyway because the ability was never usable in practice: reconstructing
an `Op::Read` already requires building a `Query`, and `Query::Items` is already
`#[non_exhaustive]` and buildable only through `Query::from_items` — so a consumer replaying a
printed sequence is already doing translation work that `Op::Read`'s own constructor was never
the barrier to. A builder or constructor (Option D) is available additively at any later
version at identical cost, so nothing is foreclosed by leaving it out now; the reverse path —
adding the attribute after this release — would have been strictly more expensive, which is the
asymmetry that decided the recommendation.

## What this does not decide

Whether `Op` should eventually become a sealed, opaque type behind a builder rather than a
`pub enum` at all is unaddressed; if the crate's public-surface owner intends that at phase 12,
this attribute is wasted motion and the right move is the larger change, once. Nothing here
supplies evidence either way. Whether the model-family rule this atom's variant belongs to needs
its own specification clause is `.kb/open-questions/model-family-rule-has-no-clause.md`'s
question, and this decision adds `ES-16` to the set of clauses that open question's atom must be
corrected to enumerate — a correction left to that atom's owner rather than made here.
