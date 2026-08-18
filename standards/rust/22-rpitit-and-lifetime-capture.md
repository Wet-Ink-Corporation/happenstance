# 22 — RPITIT, GATs and lifetime capture

> **Load when:** declaring a method on a port · an `async_fn_in_trait` warning
> fails the gate · `error[E0597]`/`E0716`/`E0515` returning a read stream · the
> `Send` flavour hands back a stream that is not `Send`
> **See also:** 20 (flavours) · 21 (`Send` obligations) · 23 (streams) ·
> 92 (toolchain limits)

---

## RS-22-1. In a public trait *not* under `#[trait_variant::make]`, spell `-> impl Future<Output = …>`.

**Why.** `async_fn_in_trait` is warn-by-default on publicly reachable traits and
the gate runs `-D warnings`. It never fires inside a `trait_variant` trait,
because the macro has already rewritten every `async fn` into `-> impl Future` —
which is why `EventStore` may write `async fn append` and `Fixture` may not.
Writing the desugaring by hand also puts the *absence* of `+ Send` at the
declaration, where a reader can see it.

**Do**

```rust
use core::future::Future;

pub trait SqlTransport {
    type Error: core::error::Error + 'static;

    /// No `+ Send`. On `wasm32` this future wraps a `JsFuture`, which cannot
    /// have it, and the bound could not be relaxed later without a breaking
    /// change.
    fn round_trip(&self, sql: &str) -> impl Future<Output = Result<String, Self::Error>>;
}
# fn main() {}
```

**Not** — this compiles here and fails in CI. Doctests receive neither the
workspace lint table nor clippy, so the fence below cannot fail; `cargo xtask ci`
is what rejects it.

```rust
pub trait Fixture {
    // warning: use of `async fn` in public traits is discouraged as auto trait
    // bounds cannot be specified
    async fn connect(&self) -> u64;
}
# fn main() {}
```

**Rejects.** An adapter author adding a hand-written port — a transport, a
fixture trait, a peer trait — with `async fn`, whose own crate builds clean
locally because the lint is a warning. It lands as a workspace-wide `-D warnings`
failure in CI, and the fix that first suggests itself is
`#[allow(async_fn_in_trait)]`, which silences the one line where a reader could
have seen that the future has no `Send` bound and never will have one.

**Evidence.** `crates/happenstance-testkit/src/contract.rs:97 (Why the methods are spelled)` ·
`crates/happenstance-neon/src/transport.rs:40 (async_fn_in_trait)` ·
[SPECIFICATION CF-20](../../spec/SPECIFICATION.md) *(why `Fixture` is
hand-written and un-derived, and so is subject to the lint)* ·
[ADR-0001](../../.kb/decisions/0001-async-port-flavours.md) ·
[rustc lint listing](https://doc.rust-lang.org/rustc/lints/listing/warn-by-default.html#async-fn-in-trait) *(checked 2026-08-09, rustc 1.97.1)*

---

## RS-22-2. Put a returned stream at the top level of the return type; never nest it inside a future.

**Why.** `trait_variant` appends its bound list to the **outermost** item of the
return type and to nothing else: `async fn` becomes `-> impl Future<..> + Send`,
and a non-`async` `-> impl Trait` takes the bound on the returned value itself. A
future's `Output` is a different type and an associated type is merely delegated,
so whichever item you put outermost is the only one the derived flavour promises
anything about.

**Do**

```rust
use futures_core::Stream;

#[trait_variant::make(SendPort: Send)]
trait Port {
    type Error;
    fn read(&self) -> impl Stream<Item = Result<u8, Self::Error>>;
}

/// Bound at the *definition*, not on a concrete stream: only the trait's own
/// promise can discharge this (SPECIFICATION ES-2 names both tests it takes).
fn stream_is_send<P: SendPort>(port: &P) {
    fn is_send<T: Send>(_value: &T) {}
    is_send(&SendPort::read(port));
}
# fn main() {}
```

**Not** — the nested shape. Modelled with an associated type because the real
spelling's diagnostic carries **no error code at all**, so no `compile_fail`
fence can pin it (adapter-shapes §2.1); the mechanism is identical and here it is
`error[E0277]`.

```rust,compile_fail,E0277
#[trait_variant::make(SendPort: Send)]
trait Port {
    type Stream;
    async fn read(&self) -> Self::Stream;
}

fn stream_is_send<P: SendPort>() {
    fn is_send<T: Send>() {}
    is_send::<<P as SendPort>::Stream>();
}
# fn main() {}
```

**Rejects.** The same shape in a port ES-2 does not cover and no suite stands
behind — `happenstance-sync`'s `pull`, drafted as `async fn pull(..) ->
Result<impl Stream<..>, Self::Error>`. Writing `+ Send` on the stream in the
one-shot HTTP impl is refused as `refining_impl_trait`, and the fix that
suggests itself is the `#[allow]` rustc prints: it compiles, deletes the only
signal, and hands every native caller a stream it cannot hold across an await —
in a crate whose conformance suite does not exist yet, so nothing else asks.

**Evidence.** `crates/happenstance-core/src/store.rs:108 (putting the stream at the)` ·
`crates/happenstance-core/src/memory.rs:614 (send_flavour_stream_is_send_in_generic_code)` ·
`crates/happenstance-sync/tests/cursor_shape_probe.rs:43 (Nesting the stream inside the future)` ·
`crates/happenstance-sync/tests/cursor_shape_probe.rs:54 (refining_impl_trait)` ·
[SPECIFICATION ES-2](../../spec/SPECIFICATION.md) *(the `read` case,
`[FROZEN]`, and the two tests it takes)* ·
[ADR-0008](../../.kb/decisions/0008-one-derivation-for-both-ports.md)

---

## RS-22-3. Take the query as a parameter; never build it locally and return the stream.

**Why.** In edition 2024 an `impl Trait` return captures every in-scope lifetime
parameter, and RPITIT always did. `read(&self, query: &Query, ..) -> impl Stream`
therefore captures the query's lifetime even though the hidden type owns
everything, and the opaque captures the `&self` borrow too — so rustc requires
`&query` to be borrowed for *that* lifetime (`argument requires that 'query' is
borrowed for '1`), which a local declared inside the function cannot survive. A
*parameter* outlives the stream by construction; a local cannot.

**Do**

```rust
use futures_core::Stream;
use happenstance_core::{EventStore, MemoryEventStore, Query, ReadOptions, SequencedEvent};

fn replay<'a, S: EventStore>(
    store: &'a S,
    query: &'a Query,
) -> impl Stream<Item = Result<SequencedEvent, S::Error>> + 'a {
    store.read(query, ReadOptions::new())
}

fn main() {
    let store = MemoryEventStore::new();
    let query = Query::all();
    let _stream = replay(&store, &query);
}
```

**Not** — one defect reported from two ends. Returned bare, as here, it is
`error[E0597]`; bound to a `let` it is `E0716`; returned with `+ '_` it is
`E0515`.

```rust,compile_fail,E0597
# use futures_core::Stream;
# use happenstance_core::{EventStore, Query, ReadOptions, SequencedEvent};
fn escapes<S: EventStore>(store: &S) -> impl Stream<Item = Result<SequencedEvent, S::Error>> {
    let query = Query::all();
    store.read(&query, ReadOptions::new())
}
# fn main() {}
```

**Rejects.** A provided or extension method that builds its own `Query` and hands
back the stream — ADR-0008's only escape from `where Self: Sync` on a provided
body, and exactly the shape this capture rule forbids, because the stream cannot
be built before the `async move` block. It does not compile, so the author
reaches instead for the edit ES-13 already refuses: `read` by value. That one
compiles everywhere, changes a `[FROZEN]` clause with no ADR, and reaches review
as a signature that got simpler.

**Evidence.** `crates/happenstance-core/tests/frozen_signatures.rs:62 (fn replay<'a, S: EventStore>)` ·
`crates/happenstance-core/tests/frozen_signatures.rs:210 (error[E0716])` ·
`crates/happenstance-core/src/memory.rs:672 (Inlining is E0716)` ·
[SPECIFICATION ES-13](../../spec/SPECIFICATION.md) *(`[FROZEN]`: `read`
takes `&Query`, and what by-value costs)* ·
[ADR-0008](../../.kb/decisions/0008-one-derivation-for-both-ports.md) ·
[edition guide, RPIT lifetime capture](https://doc.rust-lang.org/edition-guide/rust-2024/rpit-lifetime-capture.html) *(checked 2026-08-09, rustc 1.97.1)*

---

## Retired

**RS-22-4 — "Write the literal `Self::Batch<'_>` in every impl, even when the
batch is owned."** Retired 2026-08-13, at the trigger the rule itself stated: it
carried `[PROVISIONAL — settles at SPECIFICATION PS-5, which retires the GAT and
with it this trap]`, and PS-5 landed with ADR-0017. `ProjectionStore::Batch` is
now `type Batch;` with no lifetime, so `commit` declares no lifetime generics
for an impl to mismatch and `error[E0195]` is unreachable through this port. The
rule stopped rejecting any implementation an author could write, which is the
decorative shape RS-01-1 exists to forbid, and its `Do` example — binding
`type Batch<'a>` and spelling `Self::Batch<'_>` in four signatures — no longer
compiles. The id is spent and is never reused.

The transcript is kept where it happened rather than restated here:
`crates/happenstance-sqlite/src/projection_store.rs:534 (transcript this line used to carry)`
holds the error verbatim beside the impl that paid it, and
`experiments/live-handle-projection-batch/live_handle.rs:68 (exactly where PS-5 says it is)`
holds the same finding from the borrowed end.
