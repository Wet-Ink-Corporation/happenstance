# ADR-0001: Async ports in two flavours, `Send` and `!Send`

- **Status:** accepted — **provisional**
- **Date:** 2026-08-05

> **Provisional.** Authored on 2026-08-05 alongside the initial scaffold, before
> any of the code this decision constrains existed. Its entire evidence base is
> design reasoning, a spike, and a `cargo check` for `wasm32` — **no `!Send`
> implementation of these ports exists anywhere**, not even a reference one. So
> this is a recorded intention, not settled precedent: work that contradicts it
> still needs a superseding ADR, but it does not owe deference to a decision the
> code has not yet voted on.
>
> **Lifts when** a genuine `!Send` implementer passes the conformance suite. The
> cheapest such proof is a `RefCell`-backed reference store in the testkit; the
> full proof is the Cloudflare adapter (phase 5).

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
  exists. A unit test asserts the stream is `Send`.
- Laziness follows: the query executes on first poll and failures arrive as
  `Err` items rather than up front.

Naming departs from `trait_variant`'s convention, which would call the `!Send`
flavour `LocalEventStore`. "Local" already means something specific in this
project — a local-first application's on-device store — and the collision would
be a permanent source of confusion.

## Consequences

**Good.** One definition, both targets. Native adapters get `Send` futures *and*
`Send` streams. The Cloudflare path is open before any Cloudflare code exists,
and CI builds `happenstance` for `wasm32-unknown-unknown` on every commit so it
stays open.

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
