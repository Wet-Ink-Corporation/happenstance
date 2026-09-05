# 61 — Compile-time assertions, and how they go vacuous

> **Load when:** asserting a type is `Send` · asserting a type is **not** `Send` ·
> proving a GAT does not borrow · a test that passed unchanged after you deleted
> the thing it checks · `error: lifetime may not live long enough` · a skeleton
> whose every body is `todo!()` and cannot be called
> **See also:** 21 (Send is not inherited) · 22 (RPITIT and lifetime capture) ·
> 25 (what removes Send) · 60 (what a test must prove)

---

## RS-61-1. Write an auto-trait assertion inside a generic function bounded by the trait.

**Why.** Inside a generic function the compiler knows only what the trait
promises, so the obligation is discharged before monomorphisation and only the
trait can satisfy it. Against a concrete store the assertion is answered by
auto-trait leakage from the hidden type, which holds whatever the trait says.

**Do**

```rust
use happenstance_core::{MemoryEventStore, Query, ReadOptions, SendEventStore};

/// The bound is at the *definition*. `is_send` is nested so the assertion cannot
/// drift away from the function whose parameter carries the promise.
fn assert_stream_is_send<S: SendEventStore>(store: &S, query: &Query) {
    fn is_send<T: Send>(_: &T) {}
    is_send(&SendEventStore::read(store, query, ReadOptions::new()));
}

fn main() {
    assert_stream_is_send(&MemoryEventStore::new(), &Query::all());
}
```

**Not** — the same assertion against a concrete store. It compiles, it passes,
and it passes just as well against a port that promises nothing at all, which is
what this fence shows.

```rust
use futures_core::Stream;

/// No `Send` anywhere in the trait — stand-in for `Send` being struck from the
/// `trait_variant` attribute.
trait Port {
    fn read(&self) -> impl Stream<Item = u8>;
}

struct Store;
impl Port for Store {
    fn read(&self) -> impl Stream<Item = u8> {
        futures_util::stream::iter([1u8, 2])
    }
}

fn is_send<T: Send>(_: &T) {}

fn main() {
    // Passes. The hidden type happens to be `Send` and the auto trait leaks out
    // of it; nothing here consults `Port`.
    is_send(&Store.read());
}
```

**Rejects.** Precisely this test's predecessor, measured: it asserted `Send` on
`MemoryEventStore`'s concrete stream and **passed unchanged with `Send` struck
from the `trait_variant` attribute** — the port's whole two-flavour design could
have been deleted with the suite green. Note also that the generic form is
necessary and not sufficient: after an `async fn read(..) -> Result<impl Stream,
E>` refactor the outermost item is the future, `trait_variant` marks *that*
`Send`, and this is satisfied by the wrong thing. `spawns_from_generic` is the
second test, and it takes both.

**Evidence.** `crates/happenstance-core/src/memory.rs:628 (fn send_flavour_stream_is_send_in_generic_code)` ·
`crates/happenstance-core/src/memory.rs:635 (on a concrete store)` ·
`crates/happenstance-core/src/memory.rs:657 (async fn spawns_from_generic)` ·
[SPECIFICATION ES-2](../../spec/SPECIFICATION.md) ·
[ADR-0008](../../.kb/decisions/0008-one-derivation-for-both-ports.md)

---

## RS-61-2. `'_` in argument position is universally quantified; in a turbofish it is merely inferred.

**Why.** The same two characters mean different things in the two positions. In a
parameter type `'_` elides to a fresh universally-quantified lifetime, so the
signature holds only if the type genuinely does not mention it. In a turbofish it
is an inference variable, and the compiler is free to pick `'static`.

**Do**

```rust
trait ProjectionStore {
    type Batch<'a>
    where
        Self: 'a;
}

struct Owned;
struct OwnedBatch(Vec<u8>);
impl ProjectionStore for Owned {
    type Batch<'a> = OwnedBatch;
}

/// The assertion is the signature: returning the batch as a type that names no
/// lifetime is what fails if the batch ever starts borrowing the store.
fn the_batch_does_not_borrow_the_store(
    batch: <Owned as ProjectionStore>::Batch<'_>,
) -> OwnedBatch {
    batch
}

fn main() {
    let _ = the_batch_does_not_borrow_the_store;
}
```

Swap the binding to a borrowing batch — sqlx's `Transaction<'a, DB>` is the real
shape — and it fails. The diagnostic carries **no error code**; its first line is
`error: lifetime may not live long enough`.

```rust,compile_fail
# trait ProjectionStore { type Batch<'a> where Self: 'a; }
struct Borrowing;
struct Tx<'a>(&'a Borrowing);
impl ProjectionStore for Borrowing {
    type Batch<'a> = Tx<'a>;
}

fn the_batch_does_not_borrow_the_store(
    batch: <Borrowing as ProjectionStore>::Batch<'_>,
) -> Tx<'static> {
    batch
}
# fn main() { let _ = the_batch_does_not_borrow_the_store; }
```

**Not** — the obvious spelling. It compiles against the *borrowing* shape, so it
certifies exactly what it was written to reject.

```rust
# trait ProjectionStore { type Batch<'a> where Self: 'a; }
struct Borrowing;
struct Tx<'a>(&'a Borrowing);
impl ProjectionStore for Borrowing {
    type Batch<'a> = Tx<'a>;
}

fn assert_static<T: 'static>() {}

fn main() {
    // `'_` is inferred here, so the compiler picks `'static` and this holds for
    // a batch that borrows the store.
    assert_static::<<Borrowing as ProjectionStore>::Batch<'_>>();
}
```

**Rejects.** The Postgres projection store's owned-batch test as it was first
written — `assert_static::<Batch<'_>>()`. PS-5's whole open question is whether
`Batch` can stay owned once a real `sqlx` transaction is behind it, and that test
was the instrument for it: it would have reported "still owned" the day the
binding became `Transaction<'a, Postgres>`, and the finding would have been the
compile error in some downstream runner months later.

**Evidence.** `crates/happenstance-postgres/src/projection_store.rs:161 (proved nothing)` ·
`crates/happenstance-postgres/src/projection_store.rs:166 (universally-quantified lifetime)` ·
`crates/happenstance-postgres/src/projection_store.rs:182 (fn the_batch_does_not_borrow_the_store)` ·
[SPECIFICATION PS-5](../../spec/SPECIFICATION.md)

---

## RS-61-3. Observe the *absence* of an auto trait with autoref specialisation, and never without a positive control.

**Why.** `T: !Send` is not expressible, and there is no stable `impls!`. Method
resolution tries inherent candidates before trait ones and discards an inherent
candidate whose bounds do not hold, so `is_send()` reaches the inherent method
when `T: Send` and falls through to the trait's default when it does not — a
compile-time decision reported as a runtime `bool`. `&self` is the mechanism: an
associated function is resolved by path and never falls through.

**Do**

```rust
use core::marker::PhantomData;
use std::rc::Rc;

struct Probe<T>(PhantomData<T>);

trait NotSend {
    fn is_send(&self) -> bool {
        false
    }
}
impl<T> NotSend for Probe<T> {}

impl<T: Send> Probe<T> {
    fn is_send(&self) -> bool {
        true
    }
}

fn main() {
    // The positive control, and it comes first because it is what makes the
    // line below mean anything.
    assert!(Probe::<u32>(PhantomData).is_send());
    assert!(!Probe::<Rc<u8>>(PhantomData).is_send());
}
```

**Not** — the same module with the inherent impl gone, which is what a refactor
that turns `is_send(&self)` into an associated function amounts to. Every
`!Send` assertion still passes; the second line is what catches it.

```rust
use core::marker::PhantomData;
use std::rc::Rc;

struct Probe<T>(PhantomData<T>);

trait NotSend {
    fn is_send(&self) -> bool {
        false
    }
}
impl<T> NotSend for Probe<T> {}

fn main() {
    // Still "passes", and now means nothing.
    assert!(!Probe::<Rc<u8>>(PhantomData).is_send());
    // Because so does this. `u32` is `Send`.
    assert!(!Probe::<u32>(PhantomData).is_send());
}
```

**Rejects.** A broken probe in `happenstance-cloudflare`, whose entire reason to
exist is being the workspace's only `!Send` store and the sole instrument for
ES-6. Every `assert_not_send!` there keeps passing, the crate keeps advertising
that its error type cannot cross a thread, and the claim is discovered to be
untested when someone adds a `Send` bound to `EventStore::Error` and nothing
fails.

**Evidence.** `crates/happenstance-cloudflare/src/lib.rs:529 (struct Probe)` ·
`crates/happenstance-cloudflare/src/lib.rs:585 (fn the_probe_is_not_vacuous)` ·
`crates/happenstance-cloudflare/src/lib.rs:540 (is the entire mechanism)` ·
`crates/happenstance-testkit/tests/local_conformance.rs:426 (is the entire mechanism)` ·
[SPECIFICATION ES-6](../../spec/SPECIFICATION.md)

---

## RS-61-4. Put the claim in a `const` item, so the compiler is the thing that checks it.

**Why.** A generic function's obligations are discharged when it is
*instantiated*, and `const _: () = { let _ = f::<T>; };` instantiates without
calling — which is the only option against a skeleton whose bodies are `todo!()`.
The same item form is what forces a `const fn` to be evaluated at compile time; a
`let` may call the identical function at run time and proves nothing about it.

**Do**

```rust
trait ProjectionStore {
    type Batch<'a>
    where
        Self: 'a;
}

/// A runner, generic. Instantiating it is what asks the compiler whether every
/// batch shape satisfies it.
fn advance<P: ProjectionStore>(_store: &P) {}

struct Skeleton;
impl ProjectionStore for Skeleton {
    type Batch<'a> = &'a [u8];
}

// Binds no name, runs no body, and is checked during `cargo check`.
const _: () = {
    let _ = advance::<Skeleton>;
};

/// One validator, shared by the fallible constructor and the `const fn` one.
const fn accepted(value: &str) -> bool {
    !value.is_empty() && value.len() <= 128
}

fn main() {
    // Asserting on a *constant* is the point: the claim is that the check ran
    // before the program did, and the only way to state it is to make the
    // compiler run it. In-tree this carries `#[expect(clippy::
    // assertions_on_constants, reason = …)]` — `expect` rather than `allow`, so
    // it fails when the assertion stops being about a constant.
    const VERDICT: bool = accepted("CourseDefined");
    assert!(VERDICT);
}
```

**Not** — the shape that reads identical in a test report. Both lines pass, and
neither compiles anything the other spelling would not.

```rust
# trait ProjectionStore { type Batch<'a> where Self: 'a; }
# fn advance<P: ProjectionStore>(_store: &P) {}
# struct Skeleton;
# impl ProjectionStore for Skeleton { type Batch<'a> = &'a [u8]; }
const fn accepted(value: &str) -> bool {
    !value.is_empty()
}

fn both_batch_shapes_satisfy_the_same_generic_code() {
    // Names the claim; instantiates nothing. `cargo test` prints it as passing.
}

fn main() {
    both_batch_shapes_satisfy_the_same_generic_code();
    // A `const fn` may be called at run time, so this line is unchanged the day
    // `accepted` stops being `const`.
    let verdict = accepted("CourseDefined");
    assert!(verdict);
}
```

**Rejects.** `port_shape.rs` with its `const _` block dropped — a plausible edit,
because the block binds no name and the `#[test]` beside it looks like the real
artefact. Nothing is then instantiated at either of the two batch shapes, the
skeleton's `todo!()` bodies mean no call site exists anywhere else, and a
signature that holds for the owned batch and not the borrowing one ships as a
green test named `both_batch_shapes_satisfy_the_same_generic_code`.

**Evidence.** `crates/happenstance-ladybug/tests/port_shape.rs:112 (const _: () = {)` ·
`crates/happenstance-ladybug/tests/port_shape.rs:109 (Instantiating a generic function)` ·
`crates/happenstance-core/src/validate.rs:179 (const VERDICT: bool)` ·
`crates/happenstance-core/src/validate.rs:174 (clippy::assertions_on_constants)`
