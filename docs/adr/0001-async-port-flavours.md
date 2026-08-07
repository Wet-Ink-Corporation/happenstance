# ADR-0001: Async ports in two flavours, `Send` and `!Send`

- **Status:** accepted
- **Date:** 2026-08-05
- **Provisional marker lifted:** 2026-08-06, at
  [phase 1](../RUNBOOK.md#phase-1--the-send-proof-and-the-derivation-decision)
- **Extended by:** [ADR-0008](0008-one-derivation-for-both-ports.md), which
  covers `ProjectionStore` under the same scheme and states what a provided body
  owes both flavours

> **This was provisional until 2026-08-06, and the reason it no longer is.** It
> was authored alongside the initial scaffold, before any of the code it
> constrains existed, and its whole evidence base was design reasoning, a spike
> and a `cargo check` for `wasm32` — a compile of the *trait*, with **no `!Send`
> implementation of these ports anywhere**, not even a reference one. The stated
> lift condition was a genuine `!Send` implementer passing the conformance suite.
>
> `LocalMemoryEventStore` — `Rc<RefCell<Vec<SequencedEvent>>>`, implementing the
> bare `EventStore` and nothing else, in `happenstance-testkit`'s own `tests/` —
> now passes all twenty-seven rules under three harnesses natively, and under
> `wasm-bindgen-test` on `wasm32-unknown-unknown` in CI. The direct impl sits
> beside the blanket `impl<T: SendEventStore> EventStore for T` in a genuinely
> downstream crate without `error[E0119]`, which is the coherence claim at
> "Consequences of the shape" below, compiled outside the crate that declares it
> for the first time.
>
> **The full proof is still the Cloudflare adapter**
> ([phase 9](../RUNBOOK.md#phase-9--cloudflare-durable-object)). What is settled
> is that the design admits a `!Send` implementer and that the suite can drive
> one on the target. Whether a real platform SDK fits — `worker::Error`, a
> `JsValue` in an error payload, `SqlStorage`'s async shape — is phase 9's, and
> ES-6 is deferred to it.

## Context

The storage ports are async. In Rust, an `async fn` in a trait produces an
anonymous future whose `Send`-ness is fixed by the *trait definition*, not by
the implementation. Two of happenstance's targets disagree about what that should be:

- **native / tokio** requires `Send` futures. Without it, `tokio::spawn` rejects
  the future and a store cannot be used from a multi-threaded runtime.
- **`wasm32-unknown-unknown` on Cloudflare Workers** is single-threaded, and its
  futures are `!Send`. A `Send` bound cannot be satisfied there at all.

The motivating deployment needs both: a local-first native application syncing
with SQLite inside a Durable Object behind a Rust Worker.

The obvious choice, `#[async_trait]`, injects `+ Send` unconditionally and
therefore cannot serve the Workers target. `umadb-dcb`, the closest Rust prior
art, ships two hand-written parallel traits.

## Decision

Define each port once, without any `Send` requirement, and derive the `Send`
flavour with [`trait_variant`](https://docs.rs/trait-variant):

```rust
#[trait_variant::make(SendEventStore: Send)]
pub trait EventStore {
    type Error: core::error::Error + 'static;

    fn read(&self, query: &Query, options: ReadOptions)
        -> impl Stream<Item = Result<SequencedEvent, Self::Error>>;

    async fn append(&self, events: &[Event], condition: Option<&AppendCondition>)
        -> Result<SequencePosition, AppendError<Self::Error>>;
}
```

Consequences of the shape, each verified by a spike before adoption:

- `trait_variant` emits a blanket impl, so **`SendEventStore` implies
  `EventStore`**. Adapters implement whichever they can honour; all generic code
  binds `EventStore`, the weaker requirement, and accepts both.
- A downstream crate can implement `EventStore` directly for its own type
  without hitting a coherence conflict with that blanket impl.
- `read` returns the stream at the **top level** of its return type and is not
  `async`. This is not a style preference. `trait_variant` attaches `+ Send` to
  the outermost item in the return type only; had `read` been
  `async fn read(..) -> Result<impl Stream, E>`, the *future* would have been
  `Send` and the *stream* would not — so a caller could not hold a read across
  an await inside `tokio::spawn`, which is the entire reason the `Send` flavour
  exists. Two tests assert this, and it takes two: a *generic* assertion that the
  stream is `Send` (`send_flavour_stream_is_send_in_generic_code`), and
  `spawns_from_generic`, which holds a read across an await inside a real
  `tokio::spawn`. Only the second rejects the `async fn read` refactor — after
  it, the outermost item is the future, the future is `Send`, and an assertion
  on the call's result is satisfied by the wrong thing. See
  [ADR-0008](0008-one-derivation-for-both-ports.md)'s amendments section.
- Laziness follows: the query executes on first poll and failures arrive as
  `Err` items rather than up front.

Naming departs from `trait_variant`'s convention, which would call the `!Send`
flavour `LocalEventStore`. "Local" already means something specific in this
project — a local-first application's on-device store — and the collision would
be a permanent source of confusion.

## Consequences

**Good.** One definition, both targets. Native adapters get `Send` futures *and*
`Send` streams. The Cloudflare path is open before any Cloudflare code exists,
and CI builds `happenstance-core` for `wasm32-unknown-unknown` on every commit
so it stays open.

**Bad.** Importing both trait names into one module makes method-call syntax
ambiguous (`error[E0034]`), because a type satisfying `SendEventStore` satisfies
both. Callers must import one, or disambiguate with fully-qualified syntax. This
is documented on the `store` module.

**Bad.** `async fn` in traits is not dyn-compatible, so there is no
`dyn EventStore`. If erasure is needed, [`dynosaur`](https://docs.rs/dynosaur)
generates the wrapper; it is not a dependency until something needs it.

## Alternatives rejected

- **`#[async_trait]`** — injects `+ Send`, so the Workers target is impossible.
- **`!Send` only** — compiles everywhere, but native callers cannot use
  `tokio::spawn` without workarounds. An unacceptable tax on server-side use.
- **Sync core with an async wrapper** — the Durable Object `SqlStorage` API is
  async in `workers-rs`, so a blocking port cannot be implemented there.
- **Two hand-written traits (the `umadb-dcb` approach)** — doubles the trait
  surface, the conformance suite, and every adapter's maintenance burden.
