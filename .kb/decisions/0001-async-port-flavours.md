---
id: kb-decision-0001
title: Async ports in two flavours, Send and !Send
kind: decision
status: accepted
authority_tier: decision
adr_id: ADR-0001
reversibility: low
phase: 0
supersedes: null
superseded_by: null
summary: >-
  Each async storage port is defined once with no Send bound, and the Send flavour is derived by
  #[trait_variant::make(SendEventStore: Send)], with SendEventStore implying EventStore through a
  blanket impl so generic code binds the weaker one. read returns its Stream at the top level of
  the return type rather than from an async fn, so the stream itself carries the derived
  Send-ness; two tests hold that shape and it takes both. Authored provisional on 2026-08-05 with
  no !Send implementer in existence; the marker lifted 2026-08-06 when LocalMemoryEventStore
  passed the suite natively and on wasm32, and ADR-0008 records the lift. Rejected: #[async_trait],
  which injects + Send and kills the Workers target; a !Send-only port; a sync core with an async
  wrapper, which a Durable Object's async SqlStorage cannot implement; and two hand-written traits
  per port. The naming departs from trait_variant's LocalEventStore convention because Local
  already means the on-device store. Its stated consequence that dynosaur erases the port is
  wrong, and ADR-0011 corrects it.
depends_on: []
related:
  - kb-reference-port-traits-compiled-findings-001
source_paths:
  - .kb/_intake/0001-async-port-flavours.md
  - references/adr/0001-async-port-flavours.md
  - crates/happenstance-core/src/store.rs
  - crates/happenstance-core/src/memory.rs
  - CLAUDE.md
last_reviewed: 2026-08-10
---

# Async ports in two flavours, Send and !Send

## Context

The storage ports are async, and Rust fixes an `async fn` in a trait's `Send`-ness at the trait
definition rather than at the implementation. Two targets disagree about what that fixed answer
should be: native/tokio requires `Send` futures for `tokio::spawn`, while
`wasm32-unknown-unknown` on Cloudflare Workers is single-threaded and its futures are `!Send` — a
`Send` bound cannot be satisfied there at all. The motivating deployment needs both at once: a
local-first native application syncing with SQLite inside a Durable Object behind a Rust Worker.
`#[async_trait]` injects `+ Send` unconditionally and so cannot serve the Workers target; the
closest Rust prior art (`umadb-dcb`) ships two hand-written parallel traits.

## Decision

Define each port — `EventStore`, and later `ProjectionStore` under the same scheme — once, with
no `Send` bound, and derive the `Send` flavour with `#[trait_variant::make(SendEventStore:
Send)]`. The macro emits a blanket impl, so `SendEventStore` implies `EventStore`: adapters
implement whichever flavour they can honour, and all generic code binds `EventStore`, the weaker
requirement, accepting both.

`read` returns its `Stream` at the **top level** of its return type and is not itself `async`.
This is load-bearing rather than stylistic: `trait_variant` attaches `+ Send` only to the
outermost item of the return type. Had `read` been `async fn read(..) -> Result<impl Stream, E>`,
the *future* would be `Send` and the *stream* would not, defeating the entire reason the `Send`
flavour exists — a caller could not hold a read across an await inside `tokio::spawn`. Two tests
assert this and both are required: a generic assertion that the stream is `Send`
(`send_flavour_stream_is_send_in_generic_code`), which is necessary but not sufficient because it
is also satisfied by the wrong `async fn read` refactor through auto-trait leakage at the future
rather than the stream; and `spawns_from_generic`, which holds a read across an await inside a
real `tokio::spawn` and is the one that actually rejects that refactor. Laziness follows as a
consequence: the query executes on first poll and failures arrive as `Err` items rather than
up front.

Naming departs from `trait_variant`'s own convention, which would call the `!Send` flavour
`LocalEventStore`. "Local" already names something specific in this project — a local-first
application's on-device store — and the collision would be a permanent source of confusion.

## Consequences

One definition serves both targets: native adapters get `Send` futures and `Send` streams, and
the Cloudflare path stays open before any Cloudflare code exists, with `happenstance-core`
building for `wasm32-unknown-unknown` on every commit. The cost is that importing both trait
names into one module makes method-call syntax ambiguous (`error[E0034]`) — callers must import
one or disambiguate — and that `async fn` in traits is not dyn-compatible, so there is no
`dyn EventStore` without an erasure wrapper.

**Provisional-to-accepted history.** This decision was authored provisional on 2026-08-05,
alongside the initial scaffold, with its entire evidence base a spike and a `cargo check` for
`wasm32` — a compile of the *trait*, with no `!Send` implementation anywhere, not even a
reference one. The stated lift condition was a genuine `!Send` implementer passing the
conformance suite. `LocalMemoryEventStore` — `Rc<RefCell<Vec<SequencedEvent>>>`, implementing
bare `EventStore` in `happenstance-testkit`'s own `tests/` — met that condition on 2026-08-06,
passing all twenty-seven conformance rules under three harnesses and under `wasm-bindgen-test` on
`wasm32-unknown-unknown` in CI, with no coherence conflict against the blanket impl. ADR-0008
records the lift. The full proof remains the Cloudflare adapter (phase 9): what is settled is
that the design admits a `!Send` implementer and that the suite can drive one on the target, not
that a real platform SDK (`worker::Error`, a `JsValue` in an error payload, `SqlStorage`'s async
shape) fits without friction.

**A stated consequence of this decision was later found wrong.** The original text claimed that
if erasure were needed, `dynosaur` would generate a `dyn`-compatible wrapper. It does not —
`error[E0277]: dyn Stream cannot be unpinned`, because `dynosaur`'s generated wrapper produces
`Box<dyn Stream>` and `Box<T>` is only `Stream` when `T: Unpin`. ADR-0011 corrects this: a
hand-written object-safe wrapper trait, with a blanket impl over `EventStore` boxing and pinning
at both methods, erases the port today with no `unsafe`. This decision's own text is not edited
for the correction, per this repository's immutability rule; the compiled fact is kept at
`kb-reference-port-traits-compiled-findings-001`.

## Alternatives rejected

`#[async_trait]` injects `+ Send`, making the Workers target impossible. A `!Send`-only port
compiles everywhere but taxes every native caller, who cannot use `tokio::spawn` without
workarounds. A sync core with an async wrapper cannot be implemented against a Durable Object's
`SqlStorage`, which is async in `workers-rs`. Two hand-written traits per port (the `umadb-dcb`
shape) double the trait surface, the conformance suite, and every adapter's maintenance burden.
