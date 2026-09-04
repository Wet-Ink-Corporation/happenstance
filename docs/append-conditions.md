# Appending under a condition

> **Answers:** `explanation` — Why does a write re-read what it decided on?

A decision and the write that records it are two moments, and the store is the
only party present for both. Re-reading is how you hand it the boundary your
decision rested on, so it can refuse a write that boundary no longer supports.

It answers the one question its declaration names and no second one, because
[`RP-00-1`](../standards/pages/00-one-need.md) is the rule that shaped it.

## Why the store re-reads, and not you

You could run the query again yourself, immediately before appending, and it
would buy you nothing. Whatever you learn is already stale by the time the
append leaves your process, because the interval you are trying to close is the
one between your last look and the store's write — and that interval is inside
the store. Handing the condition over moves the check to the only place where
"nothing has moved since" is a sentence anybody is in a position to say.

That is also why a condition is not a lock. Nothing is held between the read
and the append: you say *afterwards* which events the decision depended on, and
the store checks that set has not grown underneath you. A lock pays for a
collision on every command; a condition pays only on the commands that lose,
and it can be built that way only because the check happens where the writes
are already ordered.

## What the store re-runs

The condition carries the query you read and the position that read reached. A
matching event above that position is what refuses the append
([ES-25](../spec/SPECIFICATION.md#es-25--condition-semantics)) — above, and not
*at*: the event sitting exactly on the boundary is one you saw and decided
with, so it cannot invalidate the decision it informed
([ES-26](../spec/SPECIFICATION.md#es-26--the-ac3-boundary-after-is-exclusive-from-is-inclusive)).

```rust
use happenstance_core::MemoryEventStore;

let store = MemoryEventStore::new();
assert_eq!(store.len(), 0);
```

`memory` is a private module (`crates/happenstance-core/src/lib.rs:134`); the
type is re-exported at `crates/happenstance-core/src/lib.rs:173`, so
`happenstance_core::MemoryEventStore` is the resolving path.

## What the condition does not claim

It is a claim about the log the evaluating store holds, and never about the
world. Where the events a condition's query ranges over are no longer in that
store there is nothing left to match, so the append is admitted — and admitted
silently, because a store that has been pruned and a store that is young are
the same value at every seam the port exposes
([ES-40](../spec/SPECIFICATION.md#es-40--a-conditional-append-is-sound-only-over-a-complete-store)).

The refusal above is therefore worth exactly what the completeness of the store
underneath it is worth. On an application's own store that is ordinarily the
whole log, and the caveat costs nothing to know; on a store fed by ingest from
somewhere else, or one that has had history removed by any means outside this
port, it is the first thing to establish rather than the last.

## Per-adapter notes

### happenstance-postgres

Positions are assigned outside the transaction, which is the axis this
workspace keeps an adapter at the other end of.

### happenstance-sqlite

One writer at a time; positions are assigned under the store's lock.
