# 21 — Send obligations are not inherited

> **Load when:** a generic runner will not compile · `S::Error` is not `Send` ·
> spawning over a store or projection-store type parameter · `error[E0277]` on an
> associated type · rustc's suggested bound gives `error[E0637]`
> **See also:** 20 (flavours) · 22 (RPITIT shapes) · 25 (what removes `Send`) ·
> 61 (asserting it)

---

## RS-21-1. Write the associated-type `Send` bounds by hand, higher-ranked over the GAT.

**Why.** `#[trait_variant::make(X: Send)]` bounds return types only; an
associated type is delegated verbatim — `type Error = <Self as X>::Error` — so
`S::Error` and `S::Batch<'a>` carry nothing from the attribute. Because `Batch`
is a GAT the bound has to be higher-ranked, `for<'a> S::Batch<'a>: Send`, and
that is the one spelling rustc will not print for you.

**Do**

```rust
use std::sync::Arc;

use happenstance_core::{ProjectionId, SendProjectionStore, SequencePosition};

fn spawn_a_batch<S>(store: Arc<S>, id: ProjectionId)
where
    S: SendProjectionStore<Error: Send> + Send + Sync + 'static,
    for<'a> S::Batch<'a>: Send,
{
    drop(tokio::spawn(async move {
        let batch = store.begin().await?;
        // The batch is live across a suspension point, which is what a runner
        // does between applying an event and deciding to commit.
        tokio::task::yield_now().await;
        store.commit(batch, &id, SequencePosition::FIRST).await
    }));
}
# fn main() {}
```

**Not** — rustc's own printed suggestion, which does not compile. A where clause
is not an elision context, so there is nothing for `'_` to resolve to:
`error[E0637]`.

```rust,compile_fail,E0637
# use happenstance_core::SendProjectionStore;
fn spawn_a_batch<S: SendProjectionStore>(_store: S)
where
    <S as SendProjectionStore>::Batch<'_>: Send,
{
}
# fn main() {}
```

**Rejects.** A runner author who takes the printed suggestion, hits `E0637`,
concludes the bound cannot be expressed, and binds a concrete
`SqliteProjectionStore` instead of the port. The runner compiles and its tests
pass; the projection layer has silently stopped being storage-agnostic, no
conformance rule covers it because conformance is about adapters, and the
regression appears in review as a shorter signature.

**Evidence.** `crates/happenstance-ladybug/tests/port_shape.rs:61 (for<'a> S::Batch<'a>: Send)` ·
`crates/happenstance-ladybug/tests/port_shape.rs:57 (cannot be used here)` ·
[SPECIFICATION PS-36](../../spec/SPECIFICATION.md) *(`[FROZEN]`: the `Send`
flavour transitively requires `Batch: Send`, and why no gate can pin it)* ·
[SPECIFICATION ES-5](../../spec/SPECIFICATION.md) *(why the bound cannot be
put on one flavour instead)* ·
[ADR-0008](../../.kb/decisions/0008-one-derivation-for-both-ports.md) ·
[adapter-shapes §2.2](../../references/adapter-shapes.md)

---

## RS-21-2. Write the `Send` assertion as `where F::Output: Send`, never as `F: Send`.

**Why.** `F: Send` and `F::Output: Send` are unrelated obligations and the
compiler discharges whichever one you typed — SPECIFICATION ES-6 gives the reason
a `Send` future promises nothing about its error. What discriminates them is the
`where` clause on a helper that takes the future by value; the failure is
`error[E0277]` naming the offending field's type, not the future's.

**Do**

```rust
use core::future::Future;
use core::pin::Pin;
use core::task::{Context, Poll};

use happenstance_core::{AppendError, MemoryStoreError, SequencePosition};

/// The live pair is `assert_future_is_send` / `assert_output_is_send` in
/// `happenstance-cloudflare`; the failing half is a `compile_fail,E0277`
/// doctest, so `cargo test --doc` re-proves it on every run.
fn output_is_send<F: Future>(future: F) -> F
where
    F::Output: Send,
{
    future
}

/// Field-less on purpose: a coroutine's auto traits are *inferred*, and an
/// inferred property is a bad instrument. This is `Send` by construction, so the
/// only thing left to disagree about is the `Output`.
struct AppendFutureShape;

impl Future for AppendFutureShape {
    type Output = Result<SequencePosition, AppendError<MemoryStoreError>>;
    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        Poll::Pending
    }
}

fn main() {
    let _checked = output_is_send(AppendFutureShape);
}
```

**Not** — the same shape with an `Rc<str>` in the error, asserted the weak way.
It compiles, and that is the defect; what catches it is `assert_output_is_send`'s
`compile_fail,E0277` doctest at `send_shape.rs:75`, not this bound.

```rust
use core::future::Future;
use core::pin::Pin;
use core::task::{Context, Poll};
use std::rc::Rc;

use happenstance_core::{AppendError, SequencePosition};

#[derive(Debug)]
struct LocalError(Rc<str>);

struct AppendFutureShape;

impl Future for AppendFutureShape {
    type Output = Result<SequencePosition, AppendError<LocalError>>;
    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        Poll::Pending
    }
}

fn future_is_send<F: Future + Send>(future: F) -> F {
    future
}

fn main() {
    // Compiles. The future is `Send`; its error can never cross a `JoinHandle`,
    // and nothing on this line notices.
    let _unchecked = future_is_send(AppendFutureShape);
}
```

**Rejects.** A maintainer who finds `assert_output_is_send`'s `compile_fail`
doctest broken by an unrelated edit and repairs it by weakening the bound to
`F: Send`. Both doctests then pass, `cargo xtask ci` stays green, and the only
instrument in the workspace that can fail `SendStoreWithLocalError` has been
quietly retired — so the loss surfaces at the first application `JoinHandle` that
has to carry an adapter's error, in a crate that never touched this file.

**Evidence.** `crates/happenstance-cloudflare/src/send_shape.rs:75 (compile_fail,E0277)` ·
`crates/happenstance-cloudflare/src/send_shape.rs:68 (assert_future_is_send)` ·
`crates/happenstance-cloudflare/src/send_shape.rs:123 (SendStoreWithLocalError)` ·
[SPECIFICATION ES-6](../../spec/SPECIFICATION.md) *(`[FROZEN]`: the
obligation, and why it is asserted on the `Output`)* ·
[ADR-0009](../../.kb/decisions/0009-error-send-sync.md)

---

## RS-21-3. Keep `Send + Sync` off the port's `Error`; ask for it at the point that spawns.

**Why.** `trait_variant` copies an associated type's declaration verbatim, so
`type Error: … + Send + Sync` lands on the bare flavour too and no spelling
scopes it to one flavour (ES-5). Asking at the use site costs the contract crate
nothing: a marker trait declared in *your* crate with a blanket impl over a
foreign one is ordinary coherence. The marker is sugar for the inline bound, not
a stronger requirement — there is no compiled difference between the two.

**Do**

```rust
use std::sync::Arc;

use happenstance_core::{
    AppendError, Event, MemoryEventStore, SendEventStore, SequencePosition,
};

/// The requirement, inline. The error type itself crosses the `JoinHandle`,
/// which is the obligation; `tokio::spawn` demands `F::Output: Send`.
fn append_in_a_task<S>(
    store: Arc<S>,
    events: Vec<Event>,
) -> tokio::task::JoinHandle<Result<SequencePosition, AppendError<S::Error>>>
where
    S: SendEventStore<Error: Send + Sync> + Send + Sync + 'static,
{
    tokio::spawn(async move { SendEventStore::append(&*store, &events, None).await })
}

/// ADR-0009's marker: the same bound, named, for when it repeats. Declared
/// downstream — `happenstance-core` grows nothing.
trait ThreadSafeEventStore: SendEventStore<Error: Send + Sync> {}
impl<S: SendEventStore<Error: Send + Sync>> ThreadSafeEventStore for S {}

fn takes_the_marker<S: ThreadSafeEventStore>(_store: &S) {}

fn main() {
    let _ = append_in_a_task::<MemoryEventStore>;
    takes_the_marker(&MemoryEventStore::new());
}
```

**Not** — the port edit, modelled: the bound moved onto the declaration. It is
the *bare* flavour that fails, which is the crate the two-trait design exists to
serve. `error[E0277]`:

```rust,compile_fail,E0277
use std::rc::Rc;

#[trait_variant::make(SendPort: Send)]
trait Port {
    // `transform_item` returns non-`Fn` items unchanged, so this reaches both
    // flavours or neither.
    type Error: core::error::Error + Send + Sync + 'static;
    async fn head(&self) -> Result<u64, Self::Error>;
}

/// The `wasm32` shape: a JS handle whose payload is an `Rc<str>`.
#[derive(Debug)]
struct EdgeError(Rc<str>);
impl core::error::Error for EdgeError {}

struct EdgeStore;

// error[E0277]: `Rc<str>` cannot be sent between threads safely
impl Port for EdgeStore {
    type Error = EdgeError;
    async fn head(&self) -> Result<u64, Self::Error> {
        Ok(0)
    }
}
# impl core::fmt::Display for EdgeError {
#     fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result { f.write_str("edge") }
# }
# fn main() {}
```

**Rejects.** The one-line alternative ADR-0009 measured and refused: adding
`+ Send + Sync` to `store.rs`'s `type Error`. `cargo check --workspace
--all-features` then passes for every crate except `happenstance-cloudflare`,
which fails with four `error[E0277]` on `Rc<str>` — a change that looks free in
every other crate, is semver-visible the moment it is published, and is found by
whoever next tries to build for Workers.

**Evidence.** `crates/happenstance-core/src/store.rs:101 (type Error: core::error::Error + 'static)` ·
`crates/happenstance-cloudflare/src/send_shape.rs:88 (SendEventStore)` ·
[SPECIFICATION ES-5](../../spec/SPECIFICATION.md) *(`[FROZEN]`: bounds are
identical on both flavours by construction)* ·
[SPECIFICATION ES-6](../../spec/SPECIFICATION.md) *(`[FROZEN]`: the bound
stays off, and the strength is a marker)* ·
[ADR-0009](../../.kb/decisions/0009-error-send-sync.md) ·
[adapter-shapes §2.1](../../references/adapter-shapes.md)
