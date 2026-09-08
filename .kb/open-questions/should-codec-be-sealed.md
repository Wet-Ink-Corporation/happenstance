---
id: kb-open-question-seal-the-codec-001
title: Codec stays unsealed after 0.2.0, and the window in which sealing was free has now closed
kind: open_question
status: accepted
authority_tier: note
summary: >-
  kb-decision-0049 (Codec::reads_tag) landed Option A of codec-foreign-tag-resolution.md and left
  Option C — sealing Codec — explicitly open, calling it "the cheaper, truer answer if no fourth
  codec ever appears in the wild" and noting it is breaking, so free only before 0.2.0. Cargo.toml
  now reads version = "0.2.0" and SECURITY.md states 0.2.0 is the first stable release with the
  frozen EventStore clauses semver-binding from here — so the brief's own deadline has passed while
  the question it gated stayed open, and sealing Codec today is a breaking change to a published,
  stable trait rather than a cost-free edit to a pre-release alpha. Nobody has implemented Codec
  outside this workspace, which is the only evidence that would justify paying that cost. Two
  sub-questions ride along, both named in the same brief and left undone on purpose: whether
  CodecError::UnknownTag should split into two #[non_exhaustive] variants distinguishing "known tag,
  feature off" from "tag nothing in this build has ever heard of", and whether Boundary::absorb
  remains the only public reading door reads_tag works through, or whether sealing would change
  what that door needs to guarantee.
depends_on: []
related:
  - kb-decision-0049
  - kb-decision-0021
  - kb-decision-0032
source_paths:
  - .kb/_intake/remediation-2026-09-04-briefs/codec-foreign-tag-resolution.md
  - .kb/_intake/2026-09-07-ratifications-discharged-and-what-execution-changed.md
  - Cargo.toml
  - crates/happenstance/src/codec.rs
last_reviewed: 2026-09-07
---

# Codec stays unsealed after 0.2.0, and the window in which sealing was free has now closed

## What is true today

`Codec`'s own rustdoc still reads "The trait is **not sealed**: a codec of your
own is a legitimate thing to write" (`crates/happenstance/src/codec.rs`), and
`reads_tag` — the defaulted method `kb-decision-0049` landed as Option A — is
a per-codec seam, not a registry. The orphan rule makes that boundary hard:
nobody outside `happenstance` can override `reads_tag` for `Json`, `Postcard`
or `Cbor`, so the migration the trait's page originally used as its
cautionary story — an application that wrote under its own codec for a year,
then adds `Json`, and expects its whole history to still decode — is not
served. `ADR-0032`'s property (a build with only `postcard` enabled must
still read a tag a `json`-only build wrote) holds exactly across the three
built-ins and holds for a third-party codec only insofar as that codec
chooses to recognise the old tag itself.

Option C of the same brief — seal `Codec`, delete the "not sealed" sentence,
make the property exactly true with no new API — was priced as "free only
before `0.2.0`" and "the cheaper, truer answer if no fourth codec ever
appears in the wild." That pricing has changed underfoot rather than been
revisited: `Cargo.toml`'s `version` key now reads `"0.2.0"`, and its own
comment states the release is "stable, as of this release, and that costs
something on purpose," with the frozen `EventStore` clauses semver-binding
from here. `SECURITY.md` independently confirms `0.2.0` is "the first stable
release." The brief's own deadline for the cheap version of this choice has
therefore passed. What is not known is whether anyone noticed it pass, or
whether it was simply not revisited because `kb-decision-0049` closed the
half of the question that had a landed answer and left the rest exactly as
found.

## What is not decided

Whether `Codec` is ever sealed, now that doing so is a breaking change to a
published, stable trait rather than an edit to a pre-release alpha — and,
separately, whether the fact that nobody has implemented `Codec` outside this
workspace in the time since `0.2.0`-shipped is evidence the trait should stay
open (nobody is inconvenienced by leaving it so) or evidence it can be sealed
cheaply relative to its cost at any *later* release (nobody would be broken
by sealing it now either). The brief did not have the evidence to settle
this when it was written, and the passage of the `0.2.0` deadline has not
supplied any.

## What forces it

A real third-party `Codec` implementation appearing — at which point sealing
becomes actively hostile to a real user rather than a hypothetical one, and
the decision tilts hard toward staying open. Absent that, nothing currently
forces a choice; the question sits at a cost asymmetry that only gets worse
with time, since every release after `0.2.0` makes sealing a larger breaking
change than the one before it.

## Ordered sub-questions

1. Does `CodecError::UnknownTag` split into two `#[non_exhaustive]` variants —
   "a known tag whose codec's feature is off" versus "a tag no build of this
   crate has ever heard of" — independent of whether `Codec` is ever sealed,
   since the split is additive either way and was left undone in
   `kb-decision-0049` "rather than presumed"?
2. Does sealing `Codec`, if it happens, change what `Boundary::absorb`
   promises as the one public reading door, or does `reads_tag` continue to
   work through it unchanged regardless of the trait's openness?
3. If no fourth codec ever appears, is "codec stays unsealed" itself the
   answer that should be written down as a decision atom — closing this
   question in the negative — rather than left permanently open on the
   argument that closing it costs nothing either way?
