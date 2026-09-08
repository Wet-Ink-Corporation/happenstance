---
id: kb-reference-event-clone-allocations-001
title: Cloning an Event costs t + 2 allocations, and Cow<'static, str> is 24 bytes
kind: reference
status: accepted
authority_tier: note
summary: >-
  The phase-12 measurement taken 2026-09-03 from experiments/event-clone-allocations/, on
  x86_64-pc-windows-msvc at rustc 1.97.1, of what it costs to clone an Event and of what the
  contract's small types actually weigh. Tags is Box<[Tag]> and Tag is Cow<'static, str>, so the
  cost depends entirely on which constructor built the tags: Tags::from_pairs goes through
  Tag::key_value to Tag::new and yields Cow::Owned, which clones by allocating, while
  Tag::from_static yields Cow::Borrowed, which clones free. At VT-22's 64-tag floor with a
  Bytes::from_static payload, from_pairs costs 66 allocations and 2,001 bytes against
  from_static's 1 allocation and 1,536 bytes; at 8 tags, 10 against 1; at 0 tags, 1 against 0.
  The from_static arm is flat in tag count and is the arm a fixture author writes without
  choosing to, so a benchmark built the natural way reports the clone immaterial whatever the
  truth is. Separately, Bytes::clone is a refcount bump only from the second clone onward, so a
  Bytes::from(Vec<u8>) payload — the shape every decoded payload has — allocates a shared header
  on its first clone, one extra allocation per payload for any store that clones each appended
  event once. And a correction to a figure that was reasoned rather than measured: Cow<'static,
  str> is 24 bytes and not 32, because String's capacity field carries the discriminant, giving
  Tag and EventType 24, Tags 16, Event 104, SequencedEvent 144 and QueryItem 32. A layout budget
  written from the derived numbers would have failed on its first run. Four sites in the corpus
  state the clone cost as two, one of them ES-17's own rationale; this atom records the
  measurement and rewrites none of them.
depends_on: []
related:
  - kb-decision-0012
  - kb-decision-0015
  - kb-reference-wire-format-measurements-001
  - kb-open-question-es-17-two-adapter-measurement-001
  - kb-decision-0055
source_paths:
  - .kb/_intake/2026-09-03-pre-publication-review.md
  - references/evaluation/review-pre-publication-2026-09-03.md
  - experiments/event-clone-allocations/
  - crates/happenstance-core/src/tag.rs
  - crates/happenstance-core/src/event.rs
last_reviewed: 2026-09-04
---

# Cloning an Event costs t + 2 allocations, and Cow<'static, str> is 24 bytes

## What this is a pointer to

The instrument — a microbenchmark over `Event::clone` at 0, 8 and 64 tags, each tag built two
ways, plus a `std::mem::size_of` census of every small type on the append path — lives in
`experiments/event-clone-allocations/`, outside the workspace and outside the gate. This atom is
the citable summary. Conditions: `x86_64-pc-windows-msvc`, rustc 1.97.1, `Bytes::from_static`
payload unless noted.

## Why the constructor decides the cost, not the type

`Tags` is `Box<[Tag]>` (`crates/happenstance-core/src/tag.rs:281`) and `Tag` is `Cow<'static,
str>` (`:79`). A `Cow` clones its variant, not its content: `Cow::Borrowed` copies a pointer and
a length, `Cow::Owned` clones the `String` it owns, which allocates. `Tags::from_pairs` builds
every tag through `Tag::key_value` → `Tag::new`, which always returns `Cow::Owned` — it has no
way to know the caller's string outlives `'static`, so it copies defensively. `Tag::from_static`
takes a `&'static str` directly and returns `Cow::Borrowed`, which clones for free.

## The measurement

| tags | via `Tags::from_pairs` | via `Tag::from_static` (control) |
| ---: | ---: | ---: |
| 0 | 1 alloc | 0 |
| 8 | 10 allocs | 1 |
| 64 | **66 allocs / 2,001 B** | **1 alloc / 1,536 B** |

The `t + 2` in the title is the `from_pairs` row read as a formula: one allocation per tag, plus
one for the boxed slice, plus one for the event's own payload handling. The `from_static` row is
flat in tag count — it pays exactly one allocation regardless of how many tags are attached —
and it is also the arm a fixture author writes without deciding to, since `from_static` is the
shorter spelling for a tag whose text is a literal. A benchmark that builds its fixture events
the natural way therefore measures the free arm and reports the clone as immaterial, which is
true of the arm it measured and false of `from_pairs`.

## Two things found alongside, neither claimed by any existing finding

`Bytes::clone` is a refcount increment from the *second* clone onward — the first clone of a
`Bytes::from(Vec<u8>)` payload allocates the shared header the refcount lives in. That shape is
what every decoded payload has, so any store that clones each appended event exactly once (to
hold one copy for the write and one for an in-flight subscriber, say) pays one extra allocation
per payload that a benchmark built from `Bytes::from_static` payloads cannot see.

The layout correction: `Cow<'static, str>` is 24 bytes on this target, not 32. `String`'s
capacity field doubles as the enum discriminant, so the `Cow` needs no separate tag byte plus
padding. Measured sizes: `Tag` and `EventType` 24 bytes, `Tags` 16, `Event` 104, `SequencedEvent`
144, `QueryItem` 32. A budget derived from "a `Cow` is a `String` plus a discriminant, rounded up
to alignment" gives 32 and would have been wrong before it was written down.

## What this atom does not do

Four sites in the corpus currently state the clone cost as two allocations, and one of them is
ES-17's own rationale. This atom records what was measured and rewrites none of them —
`kb-open-question-es-17-two-adapter-measurement-001` already owns the thread of reconciling ES-17
against measurement, and an accepted decision's rationale is corrected by superseding it, not by
a reference atom asserting against it in passing.
