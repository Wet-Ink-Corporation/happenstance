---
id: kb-decision-0049
title: A codec declares the tags it reads, and that is narrower than it sounds
kind: decision
status: accepted
authority_tier: decision
adr_id: ADR-0049
reversibility: medium
phase: 12
supersedes: null
superseded_by: null
summary: >-
  Codec gains a defaulted, additive reads_tag method so decode_by_tag's closed built-in chain can
  be extended by a third-party codec. The orphan rule makes it strictly per-codec: it serves "my
  codec also reads the tag I used to write", and not the migration Codec's own page used as its
  cautionary story, where an app that wrote under its own codec for a year and adopts Json still
  gets UnknownTag. A registry would answer that and costs global mutable state; sealing the trait
  stays open. Landed at 2d37fc4.
depends_on:
  - kb-decision-0032
related:
  - kb-decision-0021
  - kb-open-question-seal-the-codec-001
source_paths:
  - .kb/_intake/remediation-2026-09-04-briefs/codec-foreign-tag-resolution.md
  - .kb/_intake/ratifications-2026-09-06-pre-publication.md
  - .kb/_intake/2026-09-07-ratifications-discharged-and-what-execution-changed.md
last_reviewed: 2026-09-07
---

# A codec declares the tags it reads, and that is narrower than it sounds

## Decision

`Codec` gains a defaulted method, consulted before the built-in tag chain:

```rust
fn reads_tag(&self, tag: &str) -> bool {
    false
}
```

A codec that answers `true` for a tag is offered the bytes for that tag ahead of
`decode_by_tag`'s closed `#[cfg]`-gated match over `Json`, `Postcard` and `Cbor`.
The default refuses, so every existing implementor is unaffected — the method is
additive on a trait that is not yet frozen, and stays additive after `0.2.0`
because a defaulted method breaks no implementor. Landed at `2d37fc4`.

This is Option A from `codec-foreign-tag-resolution.md`, ratified on the brief's
own recommendation. Two alternatives were named and rejected: a codec-*set* bound
generalising `C: Codec` across three public signatures (`commit_with`,
`Boundary::absorb`, the projection runner's decode path), refuted earlier because
it forces a blanket impl that overlaps any user set type unless coherence can
prove otherwise; and a separate registry threaded through the read path, which
delivers the property in full but reintroduces the same coherence problem one
level up and costs global mutable state, an initialisation order, and a second
read path.

## Why the repair is narrower than the brief implied

`Codec`'s own page had used one migration as its motivating story: an application
adopts its own codec, runs for a year, then adds `Json`, and every historical
event should still decode. `reads_tag` does **not** deliver that migration. The
orphan rule forbids anyone outside `happenstance` from overriding the method for
`Json`, `Postcard` or `Cbor` — so a build holding plain `Json` and nothing else
still returns `UnknownTag` for a tag it never wrote. What the method actually
serves is the narrower case: *my codec also reads the tag I used to write* — a
rename, or one codec that is really a small dispatcher over several encodings it
already knows about. A registry would have closed the wider gap; it was not
built, because the coherence and state cost was judged larger than the evidence
for it, which is a single unreported failure mode rather than an observed one.

A test in the tree, `a_codec_of_your_own_writes_a_tag_no_other_codec_can_read`,
had documented itself as failing "under every option on the table" — a
resolution seam on `Codec`, a registry, or sealing the trait. That claim is true
of a registry and false of a per-codec method, and the test was corrected in
place rather than deleted, because the correction is itself the record of what
landed versus what was proposed.

## What stays open

Sealing `Codec` — Option C — is the cheaper, truer answer if no fourth codec ever
appears in the wild: it would make ADR-0032's property (a build with only
`postcard` enabled must still read a tag written by a build with only `json`)
exactly true rather than true-of-a-subset, with no new API. It is deferred rather
than taken because it is breaking, and breaking is free only before `0.2.0`. `A`
and `C` are not mutually exclusive in the wrong order: `A` then `C` is a coherent
sequence (add a defaulted method, then seal), just a wasteful one if `C` was
always going to be the answer. The open question is whether anyone has actually
asked for a fourth codec — this decision does not have that evidence, and says
so rather than manufacturing it.

## What this does not decide

Whether `Boundary::absorb` is the right public door for reading at all — it is
the only one today, and `reads_tag` works through it unchanged. Whether
`CodecError::UnknownTag` should split into two variants distinguishing "known
tag, feature off" from "tag nothing in this build has ever heard of" — it is
`#[non_exhaustive]`, so that split stays additive and was deliberately left
undone here rather than presumed. What a projection runner does with a foreign
tag: the same `decode_event` serves it, so the same `reads_tag` consultation
applies, but the runner sits behind `unstable-projection` and is exempt from
semver, so it does not constrain this choice either way.
