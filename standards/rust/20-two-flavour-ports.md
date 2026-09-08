# 20 — Two-flavour ports

> **Load when:** writing a new store or projection adapter · deciding which
> flavour to bind in generic code · `error[E0034]` on `store.read(..)` ·
> `error[E0119]` after adding a second impl · `Box<dyn EventStore>` will not
> compile
> **See also:** 21 (`Send` obligations are not inherited) · 22 (RPITIT shapes) ·
> 25 (what removes `Send`) · 91 (adapter recipe)

---

## RS-20-1. Derive the `Send` flavour with `#[trait_variant::make(…)]`; never hand-write the pair.

**Why.** The attribute emits a blanket `impl<T: SendPort> Port for T` that
forwards every method *and* every associated type, which is what collapses the
two flavours' projections into one type. Nothing in either trait's declaration
states that; only an identity function between the two projections does, so that
is the assertion to write.

**Do**

```rust
#[trait_variant::make(SendPort: Send)]
trait Port {
    type Error: core::error::Error + 'static;
    async fn head(&self) -> Result<u64, Self::Error>;
}

// The identity function is the assertion: returning a `SendPort::Error` where a
// `Port::Error` is demanded type-checks only if both projections normalise to
// one type. A bounds check (`where P::Error: Error`) passes trivially and proves
// nothing.
fn one_error_type<P: SendPort>(error: <P as SendPort>::Error) -> <P as Port>::Error {
    error
}
# fn main() { let _ = one_error_type::<Never>; }
# #[derive(Debug)] struct NeverError;
# impl core::fmt::Display for NeverError {
#     fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result { f.write_str("x") }
# }
# impl core::error::Error for NeverError {}
# struct Never;
# impl SendPort for Never {
#     type Error = NeverError;
#     async fn head(&self) -> Result<u64, Self::Error> { Ok(0) }
# }
```

**Not** — two hand-written traits, the `umadb-dcb` shape ADR-0001 rejected. The
same assertion is `error[E0308]`, because the two `Error`s are unrelated
projections:

```rust,compile_fail,E0308
trait Port {
    type Error;
}
trait SendPort: Send {
    type Error: Send;
}

fn one_error_type<P: SendPort + Port>(error: <P as SendPort>::Error) -> <P as Port>::Error {
    error
}
# fn main() {}
```

**Rejects.** An author who finds the macro opaque and writes `SendEventStore` out
by hand: every adapter still compiles and every generic function still compiles,
because each one sees only one of the two traits. The failure surfaces in
application code that moves an error value between a `SendEventStore`-bound layer
and an `EventStore`-bound one, as `E0308` between two types that print with the
same name — and the fix is to delete the hand-written trait, which is now
implemented by every adapter in the workspace.

**Evidence.** `crates/happenstance-core/src/store.rs:563 (fn error_projections_are_one_type)` ·
`crates/happenstance-core/src/store.rs:17 (EventStore comes free)` ·
[SPECIFICATION ES-1](../../spec/SPECIFICATION.md) *(derived, not
hand-written)* · [SPECIFICATION ES-5](../../spec/SPECIFICATION.md) *(the
forwarding, and why a bound cannot differ between flavours)* ·
[ADR-0001](../../.kb/decisions/0001-async-port-flavours.md) ·
[ADR-0008](../../.kb/decisions/0008-one-derivation-for-both-ports.md) ·
[trait-variant 0.1.3](https://docs.rs/trait-variant/0.1.3/) *(checked 2026-08-09, rustc 1.97.1)*

---

## RS-20-2. Bind the bare flavour in generic code.

**Why.** The blanket impl runs one way only: a `SendEventStore` implementer
satisfies `S: EventStore`, and an `EventStore` implementer does **not** satisfy
`S: SendEventStore` — nothing can conjure `Send`. The bare bound is therefore the
weaker requirement and the one that accepts both flavours.

**Do**

```rust
use happenstance_core::{EventStore, MemoryEventStore, Query, ReadOptions, collect};

async fn count_all<S: EventStore>(store: &S) -> usize {
    let query = Query::all();
    collect(store.read(&query, ReadOptions::new()))
        .await
        .map_or(0, |events| events.len())
}

fn main() {
    // `MemoryEventStore` implements the *Send* flavour; the bare bound takes it.
    let store = MemoryEventStore::new();
    let _future = count_all(&store);
}
```

**Not** — the `Send` flavour demanded of an adapter that has only the bare one.
`error[E0277]`:

```rust,compile_fail,E0277
#[trait_variant::make(SendPort: Send)]
trait Port {
    async fn head(&self) -> u64;
}

/// The `wasm32` shape: `Rc` inside, so the bare flavour is all it can offer.
struct EdgeStore(std::rc::Rc<str>);

impl Port for EdgeStore {
    async fn head(&self) -> u64 {
        self.0.len() as u64
    }
}

fn spawnable<P: SendPort>(_port: &P) {}

fn main() {
    spawnable(&EdgeStore(std::rc::Rc::from("do")));
}
```

**Rejects.** A projection runner or application service written
`S: SendEventStore` because the author's own adapter is native. It compiles and
it passes the suite, and no gate step sees it: `cargo xtask wasm` names four
crates — `happenstance-core`, the testkit's own harnesses, and the two `wasm32`
adapters — and a downstream crate's generic helpers are in none of them. The
exclusion is found by whoever ports the application to Workers, in a crate whose
signature they cannot change without a breaking release.

**Evidence.** `crates/happenstance-core/src/store.rs:117 (async fn count_all<S: EventStore>)` ·
`crates/happenstance-core/src/memory.rs:615 (async fn count<S: EventStore>)` ·
`crates/happenstance-ladybug/tests/port_shape.rs:55 (pub(crate) async fn advance<S: ProjectionStore>)` ·
[SPECIFICATION ES-1](../../spec/SPECIFICATION.md) ·
[ADR-0001](../../.kb/decisions/0001-async-port-flavours.md)

---

## RS-20-3. Import one flavour per module; reach the other through its full path.

**Why.** A type implementing `SendEventStore` implements `EventStore` too, so
with both names in scope every `store.read(..)` has two applicable candidates and
neither is more specific: `error[E0034]`. This is not a name collision between
two crates — the two `read`s are distinct trait items — so an `as` rename does
not help. Only dropping an import or fully-qualified syntax does.

**Do**

```rust
use happenstance_core::{EventStore, MemoryEventStore, Query, ReadOptions};

fn main() {
    let store = MemoryEventStore::new();
    // Bound to a local: the stream captures the query's lifetime (RS-22-3).
    let query = Query::all();

    let _bare = store.read(&query, ReadOptions::new());
    // The other flavour, named rather than imported.
    let _send = happenstance_core::SendEventStore::read(&store, &query, ReadOptions::new());
}
```

**Not** — both names imported. `error[E0034]`:

```rust,compile_fail,E0034
use happenstance_core::{EventStore, MemoryEventStore, Query, ReadOptions, SendEventStore};

fn main() {
    let store = MemoryEventStore::new();
    let query = Query::all();
    let _stream = store.read(&query, ReadOptions::new());
}
```

**Rejects.** The reflex fix. `frozen_signatures.rs` hit this on its first compile;
an author who resolves it by deleting the `EventStore` import and binding
`SendEventStore` throughout gets a green build and re-introduces RS-20-2's
exclusion of every `!Send` adapter — in a diff that shows only an import line
changing, which is exactly the shape a reviewer scrolls past.

**Evidence.** `crates/happenstance-core/src/store.rs:37 (error[E0034]: multiple applicable items in scope)` ·
`crates/happenstance-core/tests/frozen_signatures.rs:43 (multiple applicable items in scope)` ·
`crates/happenstance-ladybug/tests/port_shape.rs:40 (makes every method call ambiguous)` ·
[SPECIFICATION ES-1](../../spec/SPECIFICATION.md) *(one name per module)* ·
[ADR-0001](../../.kb/decisions/0001-async-port-flavours.md)

---

## RS-20-4. An adapter that must satisfy both flavours is two types.

**Why.** The blanket `impl<T: SendPort> Port for T` already covers every
`SendPort` implementer, so a hand-written `impl Port for MyStore` beside
`impl SendPort for MyStore` is `error[E0119]`, naming the macro-generated impl as
the conflicting one. The flavours are mutually exclusive per type, not a lattice
a type can sit at two points of.

**Do**

```rust
#[trait_variant::make(SendPort: Send)]
trait Port {
    async fn head(&self) -> u64;
}

/// One source file, the bare flavour, both targets.
struct EdgeStore(std::rc::Rc<str>);
impl Port for EdgeStore {
    async fn head(&self) -> u64 { self.0.len() as u64 }
}

/// A *different* type for the native flavour. It gets `Port` free.
struct NativeStore(std::sync::Arc<str>);
impl SendPort for NativeStore {
    async fn head(&self) -> u64 { self.0.len() as u64 }
}

fn takes_the_bare_flavour<P: Port>(_port: &P) {}

fn main() {
    takes_the_bare_flavour(&EdgeStore(std::rc::Rc::from("do")));
    takes_the_bare_flavour(&NativeStore(std::sync::Arc::from("pg")));
}
```

**Not** — one type, both impls. `error[E0119]`:

```rust,compile_fail,E0119
#[trait_variant::make(SendPort: Send)]
trait Port {
    async fn head(&self) -> u64;
}

struct Store;

impl SendPort for Store {
    async fn head(&self) -> u64 { 0 }
}

// Conflicts with `impl<T: SendPort> Port for T`, which the attribute emitted.
impl Port for Store {
    async fn head(&self) -> u64 { 0 }
}
# fn main() {}
```

**Rejects.** The two-step trap `happenstance-neon` has the transcript for: assert
`MyStore: SendEventStore`, get `error[E0277]`, and "fix" it by adding a second
impl. Both edits read as progress in review. The second is unfixable without
splitting the type, and the author who instead deletes the bare impl passes the
gate on the host and deletes the `wasm32` build that the single source file
existed to serve.

**Evidence.** `crates/happenstance-neon/src/event_store.rs:29 (conflicting implementations of trait EventStore)` ·
`crates/happenstance-neon/src/event_store.rs:19 (compiles *for both targets)` ·
[SPECIFICATION ES-7](../../spec/SPECIFICATION.md) *(the blanket impl a
direct impl must not collide with)* ·
[adapter-shapes §2.2](../../references/adapter-shapes.md) ·
[ADR-0001](../../.kb/decisions/0001-async-port-flavours.md)

---

## RS-20-5. There is no `dyn EventStore`. Erase the stream or the future, never the port.

**Why.** A trait carrying an RPITIT or an `async fn` is not dyn compatible —
`error[E0038]`, *"because method `read` references an `impl Trait` type in its
return type"*. `Stream` and `Future` are dyn compatible, so the erasure that
works is one level down, and writing it without `+ Send` keeps a `JsFuture`
admissible.

**Do**

```rust
use core::pin::Pin;

use futures_core::Stream;
use happenstance_core::{EventStore, MemoryEventStore, Query, ReadOptions, SequencedEvent};

/// Erase the *stream*, not the store. No `+ Send`: that is the bound a
/// `wasm_bindgen_futures::JsFuture` can meet, and `NeonReadStream` relies on it.
fn erased<'a, S: EventStore>(
    store: &'a S,
    query: &'a Query,
) -> Pin<Box<dyn Stream<Item = Result<SequencedEvent, S::Error>> + 'a>> {
    Box::pin(store.read(query, ReadOptions::new()))
}

fn main() {
    let store = MemoryEventStore::new();
    let query = Query::all();
    let _boxed = erased(&store, &query);
}
```

**Not** — erasing the port. `error[E0038]`, and note the associated type must be
named first or `E0191` masks it — which is what makes this look like a fixable
mistake for one edit:

```rust,compile_fail,E0038
use happenstance_core::EventStore;

fn any_store(_store: &dyn EventStore<Error = std::io::Error>) {}
# fn main() {}
```

**Rejects.** A caller who needs runtime store selection and answers it with a
parallel `DynEventStore` returning `Box<dyn Stream<..>>`. It duplicates the port
with no conformance suite behind it, and a boxed stream has to pick `Send` or not
once and for all — so the second port collapses the flavour split that the first
one exists to keep open, and every adapter grows two impls that can drift apart.
If the port itself genuinely must be erased, `dynosaur` generates the wrapper;
its 0.3 spelling is `#[dynosaur::dynosaur(DynStore = dyn(box) EventStore)]`, and
it is not a dependency of this workspace.

**Evidence.** `crates/happenstance-neon/src/event_store.rs:1030 (Pin<Box<dyn Future)` ·
[SPECIFICATION ES-42](../../spec/SPECIFICATION.md) *(the hand-written
`Pin<Box<dyn Stream + 'a>>` wrapper, and why it needs no `Unpin` bound)* ·
[ADR-0001](../../.kb/decisions/0001-async-port-flavours.md) ·
[dynosaur 0.3.1](https://docs.rs/dynosaur/0.3.1/) *(checked 2026-08-09, rustc 1.97.1)*
