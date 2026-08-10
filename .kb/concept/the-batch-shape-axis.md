---
id: concept-the-batch-shape-axis
title: The batch-shape axis, and why a lifetime does not close the foreign-batch hole
kind: concept
status: accepted
authority_tier: note
summary: >-
  A projection batch can BORROW its store (rusqlite's `Transaction<'_>`) or OWN its
  connection (sqlx's `Transaction<'static, Postgres>`). The first compiles on the bare
  flavour and fails on the `Send` one for two independent reasons. Tying the batch to a
  lifetime does NOT make a foreign batch unrepresentable — that was compiled and refuted.
depends_on: []
related:
  - playbook-freezing-a-port
  - question-projection-store-port-is-provisional
  - reference-experiment-rustc-ice-gat-foreign-trait
source_paths:
  - docs/adapter-shapes.md
  - crates/happenstance-core/src/projection.rs
last_reviewed: 2026-08-09
---

# The batch-shape axis

## The two ends

**Borrowed.** `type Batch<'a> = rusqlite::Transaction<'a>` compiles on the **bare** flavour
and fails on `SendProjectionStore` for two independent reasons: `Connection` is `Send` but
not `Sync`, so `&Self` is not `Send`; and `Transaction<'_>` is not `Send`, so the `commit`
future cannot be.

**Owned.** `sqlx::Pool::begin()` yields `Transaction<'static, Postgres>`, which owns its
`PoolConnection`, carries no borrow of the store, and *is* `Send`.

So the GAT's stated justification is unearned on the flavour every native adapter will
implement.

## The refuted claim

It is tempting to say that dropping to an owned batch makes the foreign-batch hazard —
`let b = a.begin(); other.commit(b);` — unrepresentable. It does not, and this was compiled
rather than argued: **a lifetime names a region, not an instance**, and two `&Store`
references unify to a common region. Even tying the batch to the receiver's lifetime accepts
the hazard. Only a generative brand rejects it, which is why the clause records it as
provisional instead of pretending the owned shape closed it.

## Why the axis matters more than the answer

Four of this workspace's implementations serialise their writers and assign positions under
a lock. Freezing the port against them freezes it against one storage shape. Postgres and
Neon exist to sit at the other end.
