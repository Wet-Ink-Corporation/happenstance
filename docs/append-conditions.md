# Appending under a condition

> **Answers:** `explanation` — Why does a write re-read what it decided on?

An append condition is checked against the same boundary the query read, so a
writer that saw a consistent view cannot be overtaken between reading and
appending (ES-40).

It answers the one question its declaration names and no second one, because
[`RP-00-1`](../standards/pages/00-one-need.md) is the rule that shaped it.

```rust
use happenstance_core::MemoryEventStore;

let store = MemoryEventStore::new();
assert_eq!(store.len(), 0);
```

`memory` is a private module (`crates/happenstance-core/src/lib.rs:103`); the type is
re-exported at `:122`, so `happenstance_core::MemoryEventStore` is the resolving path.

## Per-adapter notes

### happenstance-postgres

Positions are assigned outside the transaction, which is the axis this
workspace keeps an adapter at the other end of.

### happenstance-sqlite

One writer at a time; positions are assigned under the store's lock.
