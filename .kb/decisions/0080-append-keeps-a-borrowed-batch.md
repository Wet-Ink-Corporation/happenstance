---
id: kb-decision-0080
title: append keeps its borrowed batch, and ES-17 is frozen on the two-build measurement
kind: decision
status: accepted
authority_tier: decision
adr_id: ADR-0080
reversibility: low
phase: 17
supersedes: null
superseded_by: null
summary: >-
  EventStore::append keeps events: &[Event] and ES-17 moves from PROVISIONAL to FROZEN, on the
  two-build measurement ADR-0012's falsifier asked for and ADR-0055 scheduled against
  happenstance-cloudflare (experiments/append-batch-ownership/, 2026-10-07). Four arms on wasm32
  against the node:sqlite Durable Object host: the real adapter; B0, a replica of its write path
  verified to allocate identically at all 72 sweep points; B1, the same statements without the
  binding clone (borrowed, one Rust copy per payload, and not the strongest borrowed shape); and
  O1, an owned Vec<Event> moving data and metadata into the binding. Under a
  rule written before the run (Weigh-In wi-95d2b2; self-attested) — O1 more than 10% faster than B1 with its
  median below B1's first quartile, in any of nine cells at batch 128, tags 1/8/64, payload up to
  16 KiB — 0 of 9 cells fired. Owning the batch saves exactly 2 heap operations per event (12.5%,
  3.9%, 0.6% of B1's at 1, 8, 64 tags); the time is the statements. A raw caller resending one
  batch under by-value pays +90-95% heap operations in a k=8 contended run; a typed-layer caller,
  which rebuilds per attempt, saves 2 per committed event. The record states what ADR-0012 and
  ADR-0055 cannot: event_store_benchmarks! is compiled out on wasm32, so replica arms were used
  (wi-8b2786); Postgres copies too; only two of ADR-0055's four doc sites were corrected; and
  ES-17's third ground binds only raw-port callers. Binding Cloudflare's rows from the borrow
  through worker's SqlStorage::exec_raw (no Rust copy at all, fewer than O1), or failing that
  removing its binding clone (27-29% of its heap operations), is named as a follow-up, not
  decided; neither changes an API. No API change.
depends_on:
  - kb-decision-0012
  - kb-decision-0055
related:
  - kb-decision-0012
  - kb-decision-0055
  - kb-decision-0066
  - kb-decision-0072
  - kb-open-question-es-17-two-adapter-measurement-001
  - kb-reference-event-clone-allocations-001
  - kb-open-question-references-adr-correction-policy-001
source_paths:
  - experiments/append-batch-ownership/README.md
  - experiments/append-batch-ownership/results/realistic-point.md
  - experiments/append-batch-ownership/results/allocations.md
  - experiments/append-batch-ownership/results/caller-side.md
  - experiments/append-batch-ownership/results/raw/cloudflare-sweep.txt
  - experiments/append-batch-ownership/results/raw/cloudflare-contention.txt
  - experiments/append-batch-ownership/results/raw/host.txt
  - spec/SPECIFICATION.md
  - references/adr/0080-append-keeps-a-borrowed-batch.md
last_reviewed: 2026-10-07
---

# append keeps its borrowed batch, and ES-17 is frozen on the two-build measurement

The full record, with the tables, the Rust background, the controls and the alternatives, is
[`references/adr/0080-append-keeps-a-borrowed-batch.md`](../../references/adr/0080-append-keeps-a-borrowed-batch.md).

## The question

ES-17 was `[PROVISIONAL]` on one measurement: a real adapter showing the per-event clone is a
material fraction of append cost. ADR-0012's falsifier (`references/adr/0012-append-shape-and-preconditions.md:244-266`)
fixed its shape in five items, and ADR-0055 named its subject, `happenstance-cloudflare`, the one
shipped adapter that copies the payload into an owned row. Phase 16 made the clause
`freeze-by-17`, because the other answer breaks `append` on seven crates and only `0.4.0` can
carry it. `experiments/append-batch-ownership/` took the measurement on 2026-10-07.

## Why the shape matters

`&[Event]` lends the batch: the adapter reads it and keeps nothing without cloning, and the
caller still owns it afterwards. `Vec<Event>` gives it away: the adapter may take each event apart
with `Event::into_parts` and, when a payload's `Bytes` is unique and `Vec`-backed, recover its
buffer with `Vec::from(Bytes)` at no cost. If anyone cloned the `Bytes` first, the refcount is
above one and that recovery copies. `ConditionViolated` returns no batch, so a by-value caller
that wants to resend must clone before every attempt, which is exactly that case. `EventType` and
`Tag` are not refcounted and core cannot turn them into `String`s, so no shape moves them.
ADR-0012 (`:93-160`) already foreclosed the other two shapes: an `EventBatch` newtype by VT-24, and
`impl IntoIterator` because a generic method has no single vtable entry and cannot be erased.

## Decision

1. **`append` keeps `events: &[Event]`, and ES-17 is `[FROZEN]`.** From `1.0.0` changing it is a
   major on `happenstance-core` and every adapter.
2. **The rule fixed before the run did not fire.** At batch 128, tags 1, 8 and 64, payload 64 B,
   1 KiB and 16 KiB, `Vec`-backed: O1 would have had to beat B1 by more than 10% with its median
   below B1's first quartile in one cell. **0 of 9 did.** The largest apparent saving, 11.5%, sat
   inside B1's interquartile range. The clock's noise rests on the bimodal 64-tag samples and on a
   re-run moving which out-of-region cells fire; the static regime (equal allocation counts, not
   identical work) moved medians by up to 19.6%, consistent with that but not proof of it.
   B1 is borrowed with one Rust copy per payload, not the best a borrowed batch can do:
   `worker` 0.8.5's `SqlStorage::exec_raw` (`sql.rs:220`) binds from the borrow with no Rust copy,
   fewer than O1. O1 failed against a weaker borrowed arm than exists, so the freeze is the more
   robust for it. The rule's precedence over the run is self-attested; its earliest records are
   the lane brief's suggested rule and `wi-95d2b2`, both 27 minutes before the first raw output.
3. **The deterministic numbers agree.** Owning the batch saves exactly **2 heap operations per
   event** (data and metadata) — 256 of 2,056, 6,536 and 42,376 per 128-event append at 1, 8 and
   64 tags — and the payload bytes. An append costs `batch × (1 + t) + 1` statements, each a JS
   crossing, and that is where the time goes.
4. **Item 5 points the other way.** In a k = 8 contended run (36 attempts, 8 commits), a raw
   caller resending one batch under `Vec<Event>` pays **+90–95% heap operations** over `&[Event]`,
   and its clone turns O1's one saving back into a copy. A typed-layer caller saves 2,048
   operations a run and nothing on a refusal. The 0.2–3.8% is that difference's share of a
   simplified run (a fixed batch and a one-statement fence); the real command loop also reads,
   decodes, decides and encodes per attempt, so it is an upper bound on the share.
5. **ES-17's text is corrected line for line**: the falsifier naming the SQLite multi-row insert
   benchmark is replaced by this outcome and the reopening conditions; the clone cost is `t + 2`;
   the cited clone is `memory.rs:402-414` and the early return `:386-398`.
6. **ES-7 is not frozen here.** It is `freeze-by-17b` since ADR-0072, on the `trait-variant`
   caret question, which this measurement does not touch.

## What ADR-0012 and ADR-0055 cannot say

- **(a)** ADR-0055 scheduled the builds through `event_store_benchmarks!`, which is compiled out
  on wasm32 (`crates/happenstance-testkit/src/lib.rs:438`). The measurement used calibrated
  replica arms instead (Weigh-In `wi-8b2786`): B0 equals the real adapter, heap operations and
  bytes, at all 72 points, and nothing under `crates/` changed.
- **(b)** Postgres copies too: `insert_batch` takes `.to_owned()` and `.to_vec()` per event
  (`crates/happenstance-postgres/src/event_store.rs:1025-1030`). Not measured.
- **(c)** ADR-0055's four corrected doc sites were two (`experiments/event-clone-allocations/README.md:19-26`).
  This record fixes the specification's; the ADR-0012 long form waits on the correction-policy
  question.
- **(d)** The typed command loop rebuilds its batch every attempt (`crates/happenstance/src/command.rs:468`,
  used once at `:473`), so ES-17's third ground binds only raw-port callers resending one batch.

## Not measured

`workerd` or a deployed object; Postgres; SQLite and Neon (null by construction); the adapter's
own `evaluate`; the compensating discard; effects the clock does not resolve (bimodal samples, a
re-run moving which cells fire; ~20% or less); an `exec_raw`
arm. Two of the 63 cells outside the decision region fired — batch 128, 8 tags, 256 KiB at 14.9%
(neighbours −8.1% and −15.9%), and batch 1, 8 tags, 64 B at 13.8% — and a reviewer's re-run fired
at different ones (batch 1, 256 KiB, at 1 and 8 tags), which marks them as noise.

## Falsifiers

A `workerd` or deployed-object run of the four arms firing the same rule at a realistic point; a
domain whose ordinary payload is 256 KiB or more, with that cell firing again on repetition; a
shipped adapter whose write path is owned memory rather than statements; `ConditionViolated`
handing the batch back, which removes item 5's cost; core gaining `EventType` and `Tag` into
`String`, which lets an owned batch move more than two buffers.

## Follow-up, not decided here

Binding `happenstance-cloudflare`'s rows through `SqlStorage::exec_raw` (`worker-0.8.5/src/sql.rs:220`),
with `Uint8Array::from(&[u8])` and `JsValue::from_str(&str)` built from the borrowed batch, removes
every Rust copy of a bound value with no signature change. Failing that, removing
`SqlValue::to_binding`'s clone (B0 → B1) saves 27–29% of the adapter's heap operations and 25–50% of
its requested bytes, also with no signature change and no measurable time. Either is internal and
left to a lane that changes the adapter, with the equivalent sqlx check for Postgres.
