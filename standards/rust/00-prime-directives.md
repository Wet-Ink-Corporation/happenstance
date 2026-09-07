# 00 — Prime directives

> **Load when:** starting any change in this workspace · reaching for a trait
> attribute, a pin projection or a new dependency · `error[E0034]` on a store
> method call · a `!Send` adapter stopped compiling · wanting a language feature
> and unsure whether the floor allows it
> **See also:** 01 (what makes a rule legitimate) · 20 (two-flavour ports) ·
> 22 (RPITIT and lifetime capture) · 23 (streams) · 25 (what removes `Send`) ·
> 50 (dependency hygiene)

---

Five things no task trades. Each is a binding constraint in
[`CLAUDE.md`](../../CLAUDE.md), and the first four are governed by a clause of
[`SPECIFICATION.md`](../../spec/SPECIFICATION.md) cited under the rule: the
clause is the authority, and what this atom adds is the spelling and the
diagnostic. Changing one means writing an ADR, not writing code around it.

## RS-00-1. Make every `Stream` you write `Unpin` in all of its fields.

**Why.** `unsafe_code = "forbid"` is set at the workspace root and opted into by
every member, so no pin projection can be hand-written. `Pin<&mut Self>` reaches
`&mut Self` only through `DerefMut`, whose impl is bounded on `Self: Unpin`, so a
stream with one `!Unpin` field cannot touch its own state in `poll_next` —
`error[E0596]: cannot borrow data in dereference of `Pin<&mut Rows>` as
mutable` — and the only fixes are a projection dependency or a `Pin<Box<…>>`
per item. ES-42 forbids buying this at the port instead, so keeping the field
set `Unpin` is each adapter's own obligation and nothing in the gate states it.

**Do**

```rust
# fn main() {
#     fn assert_unpin<T: Unpin>() {}
#     assert_unpin::<Rows>();
# }
use core::pin::Pin;
use core::task::{Context, Poll};

use futures_core::Stream;

/// Every field `Unpin`, so `poll_next` needs no projection and no `unsafe`.
struct Rows(std::vec::IntoIter<u64>);

impl Stream for Rows {
    type Item = u64;

    fn poll_next(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<u64>> {
        Poll::Ready(self.0.next())
    }
}
```

**Not** — one `!Unpin` field, and `self.0` is unreachable. The diagnostic is
`error[E0596]` rather than a bound error, because the missing `Unpin` is on
`impl DerefMut for Pin<&mut T>` and what rustc reports is the failed mutable
borrow:

```rust,compile_fail,E0596
use core::marker::PhantomPinned;
use core::pin::Pin;
use core::task::{Context, Poll};

use futures_core::Stream;

struct Rows(std::vec::IntoIter<u64>, PhantomPinned);

impl Stream for Rows {
    type Item = u64;

    fn poll_next(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<u64>> {
        Poll::Ready(self.0.next())
    }
}
# fn main() {}
```

**Rejects.** An adapter whose read stream stores an inline driver future — the
natural shape when a driver's cursor API is async — compiles until `poll_next` is
written, at which point the author's cheapest exit is `Pin<Box<…>>` per polled
item. That is a heap allocation on every event read, chosen under deadline, in a
crate whose stream type is already named in a public signature and in the
adapter's own `shapes.rs` assertions.

**Evidence.** `Cargo.toml:213 (unsafe_code = "forbid")` ·
`crates/happenstance-sqlite/tests/shapes.rs:23 (needs no pin projection)` ·
`crates/happenstance-testkit/src/registry.rs:305 (std::task::Wake)` ·
`crates/happenstance-cloudflare/src/js.rs:33 (The escape hatch is an)` ·
[SPECIFICATION ES-42](../../spec/SPECIFICATION.md#es-42--reads-return-type-carries-no-unpin-bound)

## RS-00-2. Never write `#[async_trait]`, and never hard-code `+ Send` on a port's future.

**Why.** `async_trait` rewrites every method to
`Pin<Box<dyn Future + Send + 'async_trait>>`: an allocation per call, and a
`Send` bound that `wasm32-unknown-unknown` cannot satisfy at all. What ES-1
mandates instead is not a subtrait — `trait_variant` emits an independent trait
plus a blanket impl that runs one way only, so `SendEventStore` implies
`EventStore` and never the reverse.

**Do**

```rust
# fn main() {}
#[trait_variant::make(SendCounter: Send)]
trait Counter {
    async fn count(&self) -> usize;
}

// `trait_variant`'s blanket impl is one-directional, which is the whole reason
// generic code may bind the weaker flavour and still accept both.
fn takes_either<C: Counter>(_counter: &C) {}
fn given_the_send_flavour<C: SendCounter>(counter: &C) {
    takes_either(counter);
}
```

**Not** — the bound `async_trait` bakes in, written on the return type by hand.
The trait compiles; the `wasm32`-shaped implementer is rejected with
`error[E0277]: `Rc<usize>` cannot be sent between threads safely`, and there is
nothing the adapter can do about it from its own crate:

```rust,compile_fail,E0277
# use core::pin::Pin;
# use core::task::{Context, Poll};
use core::future::Future;
use std::rc::Rc;

trait Counter {
    fn count(&self) -> impl Future<Output = usize> + Send;
}

/// A future holding an `Rc`, which is what a `wasm32` handle amounts to.
struct CountFuture(Rc<usize>);
# impl Future for CountFuture {
#     type Output = usize;
#     fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<usize> {
#         Poll::Ready(*self.0)
#     }
# }

struct LocalCounter(Rc<usize>);

impl Counter for LocalCounter {
    fn count(&self) -> CountFuture {
        CountFuture(Rc::clone(&self.0))
    }
}
# fn main() {}
```

**Rejects.** A port method added in a hurry with `+ Send` spelled on its return
type, which passes the whole native gate — every adapter in the tree today is
`Send` — and is found only when the Cloudflare adapter's `Rc`-backed handle
reaches it, in phase 9, in a crate that cannot change the trait it is
implementing.

**Evidence.** `crates/happenstance-core/src/store.rs:13 (from it. The two)` ·
[SPECIFICATION ES-1](../../spec/SPECIFICATION.md#es-1--one-definition-two-flavours-and-generic-code-binds-the-weaker-one) ·
[ADR-0001](../../.kb/decisions/0001-async-port-flavours.md) ·
[trait-variant 0.1.3](https://docs.rs/trait-variant/0.1.3/) *(checked 2026-08-09, rustc 1.97.1)*

## RS-00-3. Return `read`'s stream at the top level, and never make `read` `async`.

**Why.** ES-2 is the rule; what it does not carry is the compiler's answer.
`trait_variant` bounds the **outermost** item of a return type, so under
`async fn read` that item is the future, the stream's `Send`-ness is never
stated, and the *trait* still compiles. The obligation resurfaces as
`error[E0277]` on `Self::Stream` in the first caller that holds a read across an
await — in that caller's crate, not the port's.

**Do**

```rust
use happenstance_core::{Query, ReadOptions, SendEventStore};

// The bound is written at the definition, so the obligation is discharged
// before monomorphisation rather than by auto-trait leakage from one store.
fn assert_stream_is_send<S: SendEventStore>(store: &S, query: &Query) {
    fn is_send<T: Send>(_: &T) {}
    is_send(&SendEventStore::read(store, query, ReadOptions::new()));
}
```

**Not** — the `async fn read` shape, on a stand-in port. It compiles; the caller
does not, with `error[E0277]: `<R as SendReader>::Stream` cannot be sent between
threads safely`:

```rust,compile_fail,E0277
use futures_core::Stream;

#[trait_variant::make(SendReader: Send)]
trait Reader {
    type Stream: Stream<Item = u64>;

    async fn read(&self) -> Self::Stream;
}

async fn hold_across_await<R: SendReader>(reader: &R) {
    fn is_send<T: Send>(_: &T) {}
    is_send(&SendReader::read(reader).await);
}
# fn main() {}
```

**Rejects.** The forbidden shape one layer up, where the clause does not reach: a
helper or extension trait in an adapter or application crate spelled
`async fn events(&self) -> impl Stream<…>`, written that way because the body
wants to `await` a connection checkout before building the cursor. The port is
untouched, so both tests in `memory.rs` still pass and the gate stays green,
while every caller of that helper loses the one composition the `Send` flavour
was invented to buy — holding a read across an await inside `tokio::spawn`.

**Evidence.**
`crates/happenstance-core/src/memory.rs:628 (send_flavour_stream_is_send_in_generic_code)` ·
`crates/happenstance-core/src/memory.rs:657 (async fn spawns_from_generic)` ·
[SPECIFICATION ES-2](../../spec/SPECIFICATION.md#es-2--read-returns-the-stream-at-the-top-level-and-is-not-async) ·
[ADR-0001](../../.kb/decisions/0001-async-port-flavours.md) ·
[ADR-0008](../../.kb/decisions/0008-one-derivation-for-both-ports.md)

## RS-00-4. Bind `EventStore` in generic code, and import exactly one flavour per module.

**Why.** ES-1 states both halves; the second half's mechanism is the blanket
impl. Every `SendEventStore` is also an `EventStore`, so with both names in scope
method-call syntax has two candidates and rustc rejects the *call* rather than
the import: `error[E0034]: multiple applicable items in scope`.

**Do**

```rust
use happenstance_core::{EventStore, MemoryEventStore};

fn takes_any_store<S: EventStore>(_store: &S) {}

// `MemoryEventStore` implements the `Send` flavour; the weaker bound accepts it.
takes_any_store(&MemoryEventStore::new());
```

**Not** — both names imported, so the method call is `error[E0034]`. Fully
qualified syntax (`SendEventStore::read(&store, …)`) is the escape hatch when a
module genuinely needs both:

```rust,compile_fail,E0034
use happenstance_core::{EventStore, MemoryEventStore, Query, ReadOptions, SendEventStore};

let store = MemoryEventStore::new();
let _stream = store.read(&Query::all(), ReadOptions::new());
```

**Rejects.** A projection runner bounded on `SendEventStore` because that is the
name an editor auto-imported first. It compiles, passes the whole suite, and
ships — and then the Cloudflare store, which implements only the bare flavour,
cannot be handed to the one piece of code that reads events for a projection, on
the target the bare flavour was invented for.

**Evidence.** `crates/happenstance-core/src/store.rs:37 (error[E0034])` ·
`crates/happenstance-core/tests/frozen_signatures.rs:43 (multiple applicable items in scope)` ·
[SPECIFICATION ES-1](../../spec/SPECIFICATION.md#es-1--one-definition-two-flavours-and-generic-code-binds-the-weaker-one) ·
[ADR-0001](../../.kb/decisions/0001-async-port-flavours.md)

## RS-00-5. Use the language 1.97.1 gives you; move the floor only in an ADR.

**Why.** `rust-version` and `rust-toolchain.toml` both say 1.97.1 and are two
different facts: the pin is what contributors compile with, the floor is what
consumers may. Nothing is published, so the floor is a preference rather than a
promise — but `cargo hack check --rust-version` and `resolver = "3"` both read
`package.rust-version` metadata, and a dependency that declares none is invisible
to both.

**Do** — let-chains stabilised in 1.88 and are available, because ADR-0029 raised
the floor deliberately:

```rust
use happenstance_core::SequencePosition;

fn resume_from(checkpoint: Option<SequencePosition>, ceiling: u64) -> Option<SequencePosition> {
    if let Some(position) = checkpoint
        && position.get() < ceiling
    {
        position.next()
    } else {
        None
    }
}

assert_eq!(resume_from(SequencePosition::new(4), 10), SequencePosition::new(5));
assert_eq!(resume_from(SequencePosition::new(40), 10), None);
```

**Not** — a manifest whose floor is believed to be checked. Neither
`libsqlite3-sys`, `rusqlite`, `sqlx`, `sqlx-core` nor `sqlx-postgres` declares a
`rust-version`, so five packages out of five are exempt from the mechanism that
is supposed to protect the number:

```toml
[workspace.package]
rust-version = "1.85"          # protected by `cargo hack check --rust-version`…

[workspace.dependencies]
rusqlite = { version = "0.40", features = ["bundled"] }   # …which cannot see this
```

**Rejects.** Exactly what happened at phase 2: `libsqlite3-sys 0.38.1`'s *build
script* uses `cfg_select!`, the msrv CI job stayed green because the crate
declares no floor of its own, and the break surfaced only when someone ran a 1.85
compiler by hand. A floor nobody runs is a number in a manifest, and the consumer
who finds out is the first one who is not on the pinned toolchain.

**Evidence.** `Cargo.toml:26 (rust-version = "1.97.1")` ·
`crates/happenstance-testkit/tests/mutation_coverage/correct.rs:429 (stored, condition)` ·
[ADR-0029](../../.kb/decisions/0029-msrv-raised-to-1-97-1.md) ·
[ADR-0004](../../.kb/decisions/0004-edition-and-msrv.md)
