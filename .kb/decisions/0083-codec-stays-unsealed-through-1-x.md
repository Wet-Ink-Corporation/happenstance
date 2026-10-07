---
id: kb-decision-0083
title: Codec stays unsealed through 1.x, and CodecError::UnknownTag is not split
kind: decision
status: proposed
authority_tier: decision
adr_id: ADR-0083
reversibility: low
phase: 17
supersedes: null
superseded_by: null
summary: >-
  Answers kb-open-question-seal-the-codec-001 in the negative and closes it once the record is
  accepted. happenstance's Codec trait stays unsealed for every 1.x release. The two directions
  are not symmetric. Sealing a published trait means adding a private supertrait, which breaks
  every downstream impl Codec with E0277, so it is a major. Removing a seal is additive. So "stays
  unsealed" is the one-way choice: it gives up sealing until 2.0, and in 1.x Codec can grow only
  defaulted items (RS-40-1). The tree is built around foreign codecs. commit_with is ungated and
  its page is titled "with a codec of your own". reads_tag exists only to serve one. Five external
  implementors in the crate's own tests and doctests would stop compiling. A build with no codec
  feature would have zero implementors, and commit_with, Boundary::absorb and DomainEvent::decode
  would be uncallable in it. crates.io shows no reverse dependency, so nobody is visibly broken
  either way, but nobody is inconvenienced by staying open. If ADR-0049's Option C were ever
  taken, it could not reuse crate::sealed::Sealed, whose blanket impl over DecisionModel would let
  any model implement Codec. Sub-question 1 is declined: UnknownTag keeps all three of its
  conditions (unreadable framing, a built-in tag whose feature is off, a foreign tag). Moving an
  input out of an existing variant is a behaviour break even on a non_exhaustive enum. The only
  repair for the feature-off case is a Cargo.toml edit, which no runtime branch can make, so the
  rustdoc is fixed to name all three. Sub-question 2: Boundary::absorb is unaffected either way.
  Codec's rustdoc stated the asymmetry backwards ("additive to take later"), and that is
  corrected. ADR-0049's atom shows reads_tag defaulting to false, while the code defaults to
  tag == Self::TAG, and the record notes this. The cost accepted is an unpoliced tag namespace.
depends_on:
  - kb-decision-0049
  - kb-decision-0066
related:
  - kb-decision-0049
  - kb-decision-0066
  - kb-decision-0020
  - kb-decision-0021
  - kb-decision-0032
  - kb-concept-sealed-trait-growth-001
  - kb-open-question-seal-the-codec-001
source_paths:
  - crates/happenstance/src/codec.rs
  - crates/happenstance/src/sealed.rs
  - crates/happenstance/src/boundary.rs
  - crates/happenstance/src/command.rs
  - crates/happenstance/src/lib.rs
  - crates/happenstance/Cargo.toml
  - crates/happenstance/tests/codec_extension_point.rs
  - crates/happenstance/tests/composition.rs
  - standards/rust/40-public-surface-and-evolution.md
  - references/adr/0083-codec-stays-unsealed-through-1-x.md
last_reviewed: 2026-10-07
---

# Codec stays unsealed through 1.x, and CodecError::UnknownTag is not split

The full record, with the Rust mechanics of sealing, the census of implementors and the
alternatives, is
[`references/adr/0083-codec-stays-unsealed-through-1-x.md`](../../references/adr/0083-codec-stays-unsealed-through-1-x.md).

## The question

ADR-0049 gave `Codec` a defaulted `reads_tag` and left Option C, sealing the trait, open. It
called sealing "the cheaper, truer answer if no fourth codec ever appears"
(`.kb/decisions/0049-a-codec-declares-the-tags-it-reads.md:82-91`). `0.2.0` shipped with the
trait open. ADR-0066 §10 put the question in phase 17's breaking window
(`.kb/decisions/0066-what-1-0-promises.md:347-351`), because after 1.0 sealing a public trait is a
major (`.kb/open-questions/should-codec-be-sealed.md:101-108`).

## Decision

1. **`Codec` stays unsealed for every `1.x` release.** Nothing in the trait's code changes
   (`crates/happenstance/src/codec.rs:80-188`).
2. **The asymmetry is what makes this the decision.** Sealing is a new private supertrait, and an
   existing `impl Codec for Mine` downstream then fails with `E0277`. Removing a seal breaks
   nobody. So "unsealed" is the answer that forecloses: it gives up sealing until 2.0, and in 1.x
   the trait can only grow by defaulted items, which is RS-40-1
   (`standards/rust/40-public-surface-and-evolution.md:12-17`).
3. **Sub-question 1, splitting `UnknownTag`, is declined.** `UnknownTag` covers three
   conditions, not two:
   - unreadable framing (`codec.rs:438-445`);
   - a built-in tag whose feature is off (`codec.rs:501-503`);
   - a foreign tag.

   Moving an existing input into a new variant changes which arm a caller's `match` takes. That
   is a break in effect even though `CodecError` is `#[non_exhaustive]`. So the split is `0.4.0`
   or never in 1.x, and the case it serves is repaired by editing `Cargo.toml`, which no runtime
   branch can do. The rustdoc names the third condition instead.
4. **Sub-question 2.** `Boundary::absorb` promises the same thing whichever way the trait goes.
   It reaches the codec only through `decode_event` (`crates/happenstance/src/boundary.rs:146`,
   `codec.rs:458-473`), and that path consults `reads_tag` the same way for any `C: Codec`.
5. **Two rustdoc corrections, both two-way.** ADR-0066 §5 exempts rustdoc prose from semver
   (`0066-what-1-0-promises.md:262-265`).
   - `codec.rs:61-67` says sealing "is additive to take later and impossible to undo", which is
     backwards. It is rewritten to cite this record.
   - `codec.rs:207-211` says "two conditions", which undercounts. It is rewritten to name three.

   Both keep their line counts. The strings `tests/codec_extension_point.rs` pins stay where they
   are: `not sealed`, the heading `# Reading a tag this build did not write`, and `no build`.

## Why

- **The published surface invites foreign codecs.**
  - `commit_with` is ungated (`crates/happenstance/src/lib.rs:232`), and its page is the one for
    a codec of your own (`tests/codec_extension_point.rs:175-185`).
  - `reads_tag` exists only to serve a foreign codec (`CHANGELOG.md:1419-1432`).
- **Sealing would leave a build that compiles but cannot be used.** `default-features = false`
  with no codec feature is a valid build (`crates/happenstance/Cargo.toml:105-127`). Sealed, it
  would have no implementors at all.
- **Five implementors outside the crate would stop compiling.**
  - the doctest at `codec.rs:130-159`;
  - `Runic` (`tests/codec_extension_point.rs:211`), `Elder` (`:234`) and `Forgetful` (`:437`);
  - `Json` in `tests/composition.rs:93`.
- **The one invariant the crate needs from a foreign codec is already enforced.** That is a
  non-empty tag, checked at compile time per codec type (`codec.rs:372-384`).
- **There is no evidence that sealing would harm nobody.** crates.io showed 0 reverse dependencies
  for `happenstance` on 2026-10-07. Private users are invisible, so that does not prove sealing
  harms nobody. Staying open costs nobody anything.

## If Option C is ever taken

It must not reuse `crate::sealed::Sealed`. That trait has `impl<M: DecisionModel> Sealed for M`
(`crates/happenstance/src/sealed.rs:19`), so any downstream decision model would satisfy the
supertrait and could implement `Codec`. The seal would be no seal. Option C needs its own marker,
implemented only for `Json`, `Postcard` and `Cbor` under their `cfg`s, with a `compile_fail`
doctest that implements `Codec` for a downstream `DecisionModel` type.

## A discrepancy, recorded so nobody re-derives it

ADR-0049's atom shows `reads_tag`'s default as `false`
(`0049-a-codec-declares-the-tags-it-reads.md:37-41`). The code's default is
`tag == Self::TAG` (`codec.rs:185-187`), and `decode_event` checks `tag == C::TAG` before asking
(`codec.rs:471`). The code is what shipped and is what `0.3.2` promises. The atom is immutable and
is not edited.

## Accepted costs

- **The tag namespace is unpoliced.** A foreign codec may choose `TAG = "json"`. A reader holding
  `Json` would then decode that codec's bytes as JSON. That ends in a `Decode` error or a misread.
  Sealing would have prevented it. A rustdoc warning on `Codec::TAG` would be additive and can
  land in any phase. This record does not owe one.
- **No required item can be added to `Codec` in 1.x.** Only defaulted methods and defaulted
  associated consts can be added.

## Falsifiers

- A downstream codec's `reads_tag`, or a tag collision, produces a silent misread that the
  documentation could not have prevented. That reopens Option C, as a `2.0` question.
- Someone deletes one of the external implementors, or the "not sealed" test, on the grounds that
  the trait "should be sealed". That is this record being overturned without a superseding one.
