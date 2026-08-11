---
id: kb-decision-0009
title: Error stays unbounded, and the strength goes in a marker
kind: decision
status: accepted
authority_tier: decision
adr_id: ADR-0009
reversibility: low
phase: 2
supersedes: null
superseded_by: null
summary: >-
  Error keeps exactly core::error::Error + 'static on both ports and both flavours, and the
  stronger property moves downstream into a blanket-implemented marker trait —
  ThreadSafeEventStore over SendEventStore<Error: Send + Sync> — that generic code opts into. Four
  compiled findings decide it: a workspace-wide check with + Send + Sync fails only
  happenstance-cloudflare's Rc<str> error; ES-6's premise about JsValue is half wrong, since
  JsValue is Send + Sync on non-atomics wasm32 and Rc is the hazard; the conflict signal never
  travels in Self::Error because the contract lifts it to AppendError::ConditionViolated; and the
  derived Send flavour does not imply a Send error, so ES-6's rule could not have been written
  against today's port for any adapter. Rejected: widening Error directly, which breaks the
  wasm32 target the two-flavour design exists for; leaving ES-6 deferred, which is abandonment
  behind an unwritable rule; a second SendError associated type, which trait_variant would copy
  into the !Send flavour too; and shipping the marker in happenstance-core now. The same answer
  covers ProjectionStore and PS-35, and nothing about the port changes.
depends_on:
  - kb-decision-0008
related:
  - kb-reference-port-traits-compiled-findings-001
source_paths:
  - .kb/_intake/0009-error-send-sync.md
  - references/adr/0009-error-send-sync.md
  - crates/happenstance-core/src/store.rs
  - crates/happenstance-core/src/error.rs
  - crates/happenstance-cloudflare/src/lib.rs
  - spec/SPECIFICATION.md
last_reviewed: 2026-08-10
---

# Error stays unbounded, and the strength goes in a marker

## Context

Both ports declare `type Error: core::error::Error + 'static`, with no `Send` or `Sync`. ES-6
asks whether that should change, and it is the highest blast-radius open question in the
workspace because the associated type cannot be varied between the bare and derived flavours —
whatever is decided is semver-visible and must be settled before publish. Two prior planning
documents had argued it in opposite directions from the same file, and neither in-tree "adapter"
could have falsified either argument: `MemoryStoreError` is uninhabited and the SQLite adapter's
error was a single placeholder variant. `happenstance-cloudflare`, whose error holds a genuinely
`!Send` `Rc<str>`, is the instrument that finally can.

## What four compilations settled

Adding `+ Send + Sync` to `Error` and running the workspace check fails exactly one crate:
`happenstance-cloudflare`, on its `Rc<str>` — the `wasm32` target the two-flavour design exists to
serve. ES-6's stated premise, that an adapter error holding a `JsValue` is the hazard, is half
wrong: `wasm-bindgen` marks `JsValue` `Send + Sync` on non-`atomics` `wasm32`, so `JsValue` costs
nothing and `Rc` is the actual cost. The conflict signal a caller cares about never travels in
`Self::Error` on any adapter regardless of this decision, because `happenstance-core` already
lifts it into `AppendError::ConditionViolated` before `Self::Error` is even constructed. And the
decisive finding: the derived `Send` flavour does not imply a `Send` error — a store can be
`SendEventStore` with `Send` store, stream, and future, and still carry a `!Send` `Error`, and it
compiles. So ES-6's named rule, `store_error_crosses_a_join_handle`, could not have been written
against today's port for any adapter; a rule that only checks the future is `Send` passes against
a `!Send` error and asserts nothing.

## Decision

`Error` keeps its bound exactly as written, on both ports and both flavours. The stronger property
becomes a separate marker trait that generic code opts into:

```rust
pub trait ThreadSafeEventStore: SendEventStore<Error: Send + Sync> {}
impl<S> ThreadSafeEventStore for S where S: SendEventStore<Error: Send + Sync> {}
```

Compiled and confirmed to work — `MemoryEventStore` satisfies it and a rule spawning `append` and
awaiting the error type across a `JoinHandle` passes — and to bite: a store meeting every `Send`
obligation of the derived flavour but carrying `happenstance-cloudflare`'s `Rc<str>` error is
rejected with `E0277`. It does not belong in `happenstance-core`: a local trait with a blanket
impl over a foreign one is ordinary coherence, so the contract crate need not grow anything, and
any downstream application can declare the same marker itself. The identical answer covers
`ProjectionStore` and PS-35, since the marker is a supertrait bound with a blanket impl rather
than a provided method, so it does not meet the prior decision's GAT obstruction.

## Consequences and alternatives rejected

Good: the `wasm32` target keeps a `!Send` error and nothing pays for a native concern; ES-6 moves
from deferred to settled, with its rule now writable and a failing implementation already in the
tree; no breaking change. Bad: generic code gains a third bound to choose between alongside
`EventStore` and `SendEventStore`, and choosing wrong stays silent until someone spawns.

Rejected: adding `Send + Sync` to `Error` directly, which fails the one crate the two-flavour
design exists for with no way to scope the bound to the native flavour; leaving ES-6 deferred,
which defers behind a rule that cannot be written at all rather than one merely unwritten; a
second `SendError` associated type, which `trait_variant` would copy verbatim into the `!Send`
flavour too; and shipping the marker in `happenstance-core` now, which is a surface question for a
later phase rather than a capability question this decision needed to answer.
