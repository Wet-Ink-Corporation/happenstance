---
id: kb-decision-0055
title: append keeps its borrowed batch at 0.2.0, and ES-17's falsifier names the wrong adapter
kind: decision
status: accepted
authority_tier: decision
adr_id: ADR-0055
reversibility: medium
phase: 12
supersedes: null
superseded_by: null
summary: >-
  EventStore::append's ownership does not change for 0.2.0. Four corpus sites stating the clone cost as roughly two allocations are corrected to the measured t + 2, and ES-17's falsifier is restated to name the adapter shape measured, the tag regime and the tag count — because as written it names the SQLite multi-row insert, which is exactly the shape ADR-0012 item 4 says cannot benefit. happenstance-cloudflare, not MemoryEventStore, is item 4's benefiting shape, and has been since phase 9. The two-build measurement is scheduled against it and is not yet taken.
depends_on:
  - kb-decision-0012
related:
  - kb-open-question-es-17-two-adapter-measurement-001
  - kb-reference-event-clone-allocations-001
  - kb-open-question-references-adr-correction-policy-001
source_paths:
  - .kb/_intake/remediation-2026-09-04-briefs/append-batch-ownership.md
  - .kb/_intake/ratifications-2026-09-06-pre-publication.md
last_reviewed: 2026-09-07
---

# append keeps its borrowed batch at 0.2.0, and ES-17's falsifier names the wrong adapter

## Decision

`EventStore::append` keeps `events: &[Event]` for the `0.2.0` release. ES-17
`[PROVISIONAL]` (`spec/SPECIFICATION.md:3345-3353`) is not lifted, and `kb-decision-0012`
— accepted and immutable — is not superseded: this decision rests on it rather than
correcting it, in the same posture ADR-0029 uses to amend ADR-0004 without superseding it.

**The documentation is wrong and is corrected.** Four sites in the corpus state a clone's
cost as "one `Box<str>` and one boxed tag slice" — roughly two allocations —
(`crates/happenstance-core/src/event.rs:409-412`, `spec/SPECIFICATION.md:3370-3373`,
`references/adr/0012-append-shape-and-preconditions.md:172-174`, and, without a count,
`crates/happenstance-core/src/memory.rs:30-31`). Both the type and the count are stale:
`EventType` and `Tag` back onto `Cow<'static, str>` since ADR-0015, and `Tags` is
`Box<[Tag]>`, each owned `Tag` its own allocation. Measured against
`experiments/event-clone-allocations/results/raw/clone.txt` on `rustc 1.97.1`: the owned
arm is **`t + 2`** heap operations for every `t >= 1` (one for `EventType`'s `String`, one
for the boxed tag slice, one per tag), and **1** at `t = 0`; the borrowed arm is flat at
**1** regardless of tag count. At VT-22's 64-tag floor that is 66 heap operations against
1 — a 66x ratio the corpus's own "two allocations" framing understates by 64x. All four
sites are corrected to the measured figures and the current field types.

**ES-17's falsifier is restated, not fired.** As written it names "the SQLite adapter's
multi-row insert benchmark" as the measurement that would lift the marker
(`spec/SPECIFICATION.md:3350-3351`). But ADR-0012 item 4 already says only a store that
*moves the payload into an owned row it keeps* can benefit from owning the batch — a SQL
adapter that binds parameters from a borrow, which is exactly what
`crates/happenstance-sqlite/src/event_store.rs:901-922`'s `write_batch` does, cannot. The
falsifier as written names the one shape guaranteed to produce a null result. The restated
falsifier must name the adapter shape measured, the tag regime the fixtures run in (the
built harness already exercises the expensive `Cow::Owned` regime via `Tags::from_pairs`),
and the tag count (the harness is hard-coded at one tag, a 22x understatement of VT-22's
floor).

**The census of which adapter could benefit is corrected.** ADR-0012's own text names
`MemoryEventStore` as the only store that clones and disqualifies it as the least
representative shape. That was true at phase 4 and has been stale since phase 9:
`happenstance-cloudflare` has no `todo!()`, runs `event_store_conformance!`, and is
published — and its `write_rows` (`crates/happenstance-cloudflare/src/event_store.rs:699-728`)
copies the event type, payload and metadata into owned `SqlValue`s **per event**, because
the Workers SQL binding cannot take a borrow. That is item 4's benefiting shape, in an
adapter that is neither the reference store nor SQLite. A two-build measurement — the same
adapter, differing only in `append`'s ownership, both driven through
`happenstance_testkit::event_store_benchmarks!` from an out-of-workspace experiment crate
on the `experiments/append-condition/` pattern — is scheduled against `happenstance-cloudflare`
rather than SQLite. That measurement is not yet taken.

## Alternatives rejected

**Changing `append` to `Vec<Event>` now** was rejected: no measurement exists on any
adapter, `ConditionViolated`'s obligation that a caller keep its events across a retry has
no cheap-copy design anywhere in the tree (item 5 of ADR-0012's falsifier), and the
signature is inside a published crate (`happenstance-core` at `0.2.0-alpha.1` since
2026-08-16), so the change is breaking today rather than free. **Lifting ES-17 to
`[FROZEN]` on the structural argument alone** was rejected on `kb-decision-0012`'s own
standing objection: moving a maturity marker because a phase wants it moved, with no new
measurement, is the one thing a marker must never do. The clone-cost correction is new
evidence about a clone's *price*; it is not evidence about `append`'s *cost*, which is the
thing the marker is provisional on.

## What this does not settle

Whether the two-build measurement, once taken against `happenstance-cloudflare`, moves the
marker either way; who owns and schedules it (`kb-open-question-es-17-two-adapter-measurement-001`
remains open); whether `references/adr/` records may be corrected in place for the same
stale figures `spec-trace` cites into by line range; and whether the benchmark harness
gains a tag-count parameter, which both the restated falsifier and the eventual measurement
need.
