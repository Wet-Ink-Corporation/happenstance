---
id: kb-decision-0032
title: The serde-encoded framing region is rejected on two grounds, and ADR-0003 was never one of them
kind: decision
status: accepted
authority_tier: decision
adr_id: ADR-0032
reversibility: low
phase: 7
supersedes:
  - kb-decision-0021
superseded_by: null
summary: >-
  A repair of ADR-0021, not an amendment: the set of implementations admitted is unchanged, and all
  three of ADR-0021's decisions carry over intact — the codec tag lives in Event::metadata inside a
  versioned framing region the typed layer owns, EventType carries no version suffix and matching is
  exact equality, and upcasting happens at decode with no read-path hook added to the frozen
  EventStore, with an untagged event decoding under the codec already in hand rather than as
  UnknownTag. What is withdrawn is one justification for one rejected alternative. ADR-0021 rejected
  a serde-encoded framing region on three grounds and the third stated a binding constraint
  backwards: also barred by ADR-0003, which forbids serde in happenstance-core's default features.
  ADR-0003 constrains happenstance-core only and positively assigns encoding and decoding to
  happenstance, the layer above; the framing region is written by happenstance, one crate above the
  port, so ADR-0003 does not bar serde there, and CLAUDE.md's binding constraint 2 warns about this
  exact reading after ADR-0006's rename. Read as written, an implementer would conclude serde is
  barred from the crate whose entire job is encoding — forbidding the thing the ADR-0006 split
  exists to allow. The rejection stands on its two sound grounds, which never depended on ADR-0003:
  reading the framing region is how the payload codec is discovered, so the region cannot be encoded
  by a Codec and must not depend on any codec feature, since a build with only postcard enabled must
  still read a tag written by a build with only json; and the region must be readable with no default
  features and on wasm32, so it is not serde-encoded and is not JSON. The boundary is stated here
  once in the right direction so the atom that supersedes this one cannot re-acquire the inversion:
  ADR-0003 constrains happenstance-core, whose serde feature covers envelope types only; happenstance
  is the typed layer whose job is encoding, and serde there is the split working; payloads stay Bytes
  at the port. Provenance: the story's staged deliverable gave only the two sound reasons and stated
  the boundary correctly, and the ADR-0003 clause was introduced during distillation in the
  2026-08-15 wave, which makes this an ingest defect rather than a story defect.
depends_on:
  - kb-decision-0021
related:
  - kb-decision-0003
  - kb-decision-0006
  - kb-decision-0016
  - kb-decision-0020
source_paths:
  - .kb/_intake/0031-adr-0021-serde-attribution-correction.md
  - references/adr/0021-payload-evolution-and-codec-tag.md
  - .kb/decisions/0003-opaque-payloads.md
  - crates/happenstance-core/src/event.rs
  - xtask/src/main.rs
last_reviewed: 2026-08-17
---

# The serde-encoded framing region is rejected on two grounds, and ADR-0003 was never one of them

## What this repairs, and what it leaves alone

[kb-decision-0021](0021-payload-evolution-and-codec-tag.md) rejected a `serde`-encoded framing
region on three grounds, and the third states a binding constraint backwards: *"also barred by
ADR-0003, which forbids `serde` in `happenstance-core`'s default features"*
(`.kb/decisions/0021-payload-evolution-and-codec-tag.md:141-144`).

The framing region is written by `happenstance`, the typed layer — one crate above the port.
[kb-decision-0003](0003-opaque-payloads.md) constrains `happenstance-core` only, and does not merely
permit `serde` above it: it *positively assigns* the work, *"Encoding and decoding — a `Codec`, a
`DomainEvent` mapping — belong to `happenstance`, the layer above"*
(`.kb/decisions/0003-opaque-payloads.md:17-18`). So the citation inverts the constraint it cites.
Read as written, an M3 implementer would conclude that `serde` is barred from the crate whose entire
job is encoding — forbidding the thing [kb-decision-0006](0006-bare-name-to-the-typed-layer.md)'s
split exists to allow. `CLAUDE.md`'s binding constraint 2 warns about exactly this reading, in terms,
because after the rename the crate name in the old constraint says the opposite of what it used to.

This is a **repair** on `.kb/decisions/README.md:22-25`'s mechanical test — the set of
implementations the decision admits is unchanged — so all three of ADR-0021's decisions carry over
intact and unqualified: the codec tag lives in `Event::metadata` inside a versioned framing region
the typed layer owns and no store parses; `EventType` carries no version suffix and matching is exact
equality; and upcasting happens at decode, with no read-path hook added to the `[FROZEN]`
`EventStore`. So do its falsifier, its `low` reversibility, and its untagged-event rule — an event
whose metadata carries no framing region decodes with the codec already in hand, not as
`CodecError::UnknownTag`. Only one justification for one rejected alternative is withdrawn.

## The rejection, on the two grounds that were always sufficient

A `serde`-encoded framing region stays rejected, for these reasons and these alone:

- **Codec-feature independence.** Reading the framing region is *how* the payload codec is
  discovered, so the region cannot be encoded by a `Codec` and must not depend on any codec feature:
  a build with only `postcard` enabled must still read a tag written by a build with only `json`.
- **No added dependency.** The region must be readable with no default features and on `wasm32`, so
  it is not `serde`-encoded and is not JSON.

Neither ever depended on ADR-0003. The ADR-0003 attribution is **dropped**, not corrected: ADR-0003
has nothing to say about this alternative in either direction, and a corrected-but-present citation
would keep inviting the same misreading. The relationship is carried by `related` instead.

## The boundary, stated once in the right direction

So that the atom which supersedes *this* one cannot re-acquire the inversion: ADR-0003's prohibition
attaches to **`happenstance-core`**, whose `serde` feature covers envelope types only, off by
default, for replication. `happenstance` is the typed layer whose job *is* encoding, and `serde`
arriving there is the split working rather than a violation. Payloads remain `Bytes` at the port and
`Codec` operates strictly above it; encoding never routes through `happenstance-core/serde`. The
framing region is a stricter case again — it is not `serde`-encoded *at all*, for the two reasons
above, which is why the contract crate's `--no-default-features` doc build and the `wasm32` steps
stay honest about it.

One instrument note that belongs with the second ground, because the obvious reading is wrong: both
`--no-default-features` steps in `cargo xtask ci` are scoped `-p happenstance-core`
(`xtask/src/main.rs:216-223` and `:532-544`), and the framing region is `happenstance`'s, so neither
step ever builds it. The no-dependency property holds through the workspace-wide
`cargo hack check --workspace --feature-powerset --no-dev-deps` step (`:605-615`) and the `wasm32`
powerset (`:623-652`), both behind a `cargo hack` probe.

## Why a supersession rather than an edit, and why full

An accepted decision's body is immutable and `redkiln validate --kb` checks each one against `HEAD`,
so rewording ADR-0021 in place fails the gate by design. The defect is in the atom's **own body**
rather than in an allocation it made, which is what earns the full supersession spelling with a
metadata flip on `kb-decision-0021` — unlike the partial spelling
[kb-decision-0031](0031-the-runner-collapses-upward.md) took, where three still-standing decisions
would have been retired along with the one that changed. ADR-0021's body stays verbatim: the older
commits' reasoning only makes sense with it, and the record of what was written wrongly is the point.

**Provenance — an ingest defect, not a story defect.** The story's staged deliverable gave only the
two sound reasons and stated the boundary correctly in its own section; the ADR-0003 clause was
introduced during distillation in the 2026-08-15 wave. The long-form record
(`references/adr/0021-payload-evolution-and-codec-tag.md:309` and its *"The `serde` boundary, in the
right direction"* section at `:353-365`) never carried the inversion and needs no repair.
