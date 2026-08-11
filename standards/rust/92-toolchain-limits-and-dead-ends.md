# 92 — Toolchain limits and dead ends

> **Load when:** `thread 'rustc' panicked` on an adapter impl · a store that
> carries a lifetime will not implement a port · `error[E0658]: return type
> notation is experimental` · designing around a feature that "will land soon"
> **See also:** 21 (`Send` bounds by hand) · 22 (`E0195` and RPITIT) · 62
> (doctest annotations, and why an error code is not a pin) · 90 (skeletons) ·
> 01 (the standard of evidence)

Every rule here is a dead end plus the check that says when it has opened up.
Without that check the atom becomes folklore, which is the failure it exists to
prevent.

---

## RS-92-1. A type implementing a port with a batch GAT MUST be `'static`; a lifetime on it ICEs rustc 1.97.1.

**Why.** Five ingredients, each independently necessary: the trait in a **foreign
crate**, a GAT with `where Self: 'a`, an **RPITIT** return, `Self::Batch<'_>` in
that method's signature, and a **non-`'static`** impl self type. `ProjectionStore`
supplies the first four — it is the only port in the workspace that does, and
`impl EventStore for Store<'db>` compiles, because that port has no GAT — so on
the projection side the store's own lifetime is the only ingredient you control.
rustc finds a genuine region error and then crashes in the path that explains a
bound for an item in another crate.

**Do** — hold the database by refcount, not by reference:

```rust
use std::sync::Arc;

use happenstance_core::{ProjectionId, SendProjectionStore, SequencePosition};
# #[derive(Debug)]
# struct GraphError;
# impl core::fmt::Display for GraphError {
#     fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result { f.write_str("graph") }
# }
# impl core::error::Error for GraphError {}
struct Database;

/// `'static`, because the connection owns an `Arc` instead of borrowing a
/// `&'db Database`. That refcount is the difference between an impl and an ICE.
struct GraphStore { database: Arc<Database> }

struct WriteSet { statements: Vec<String> }

impl SendProjectionStore for GraphStore {
    type Error = GraphError;
    type Batch<'a> = WriteSet where Self: 'a;

    async fn checkpoint(
        &self,
        _id: &ProjectionId,
    ) -> Result<Option<SequencePosition>, Self::Error> {
        Ok(None)
    }
    async fn begin(&self) -> Result<Self::Batch<'_>, Self::Error> {
        let _ = Arc::clone(&self.database);
        Ok(WriteSet { statements: Vec::new() })
    }
    async fn commit(
        &self,
        mut batch: Self::Batch<'_>,
        id: &ProjectionId,
        position: SequencePosition,
    ) -> Result<(), Self::Error> {
        batch.statements.push(format!("MERGE {id} = {}", position.get()));
        Ok(())
    }
    async fn rollback(&self, _batch: Self::Batch<'_>) -> Result<(), Self::Error> { Ok(()) }
}

fn main() { let _ = GraphStore { database: Arc::new(Database) }; }
```

**Not** — the obvious layout for a driver whose connection borrows its database.
This cannot be a compiled fence and never will be: a file that crashes the
compiler fails the gate, so the reproduction lives in `experiments/`.

<!-- ignore: this source ICEs rustc 1.97.1; a fence that crashes the compiler fails the gate -->
```rust,ignore
struct GraphStore<'db> { connection: Connection<'db> }

impl SendProjectionStore for GraphStore<'_> {
    type Batch<'a> = WriteSet where Self: 'a;
    async fn commit(&self, batch: Self::Batch<'_>, /* … */) -> Result<(), Self::Error> { /* … */ }
}

// thread 'rustc' panicked at
//   compiler\rustc_trait_selection\src\errors\note_and_explain.rs:27:22:
// DefId::expect_local: `DefId(… SendProjectionStore::commit)` isn't local
// #0 [compare_impl_item] checking assoc item `<impl …>::commit::{anon_assoc#0}`
```

**What would open it up.** `rust-lang/rust#158983` closing, or PS-5 landing —
`type Batch;` deletes ingredient two and the whole exposure with it, and the
clause is provisional today. Re-check with
`experiments/rustc-ice-gat-foreign-trait/bisect.sh`; the recorded answer is
that it reproduces on 1.85.1, 1.97.1 and 1.99.0-nightly, on editions 2018, 2021
and 2024, so this is neither a regression nor fixed on nightly.

**Rejects.** An adapter for an embedded database whose `Connection<'db>` borrows
an open `Database` — the natural layout, and the one LadybugDB's API invites. The
author writes `struct Store<'db>`, gets a compiler crash with no line of their
own code named, and reads it as a broken toolchain or a corrupt incremental
cache. The fix is a refcount in a field, and nothing in the diagnostic points
there.

**Evidence.** `crates/happenstance-ladybug/src/live_handle.rs:38 (store crashes the compiler)` ·
`crates/happenstance-ladybug/src/live_handle.rs:63 (only by stores that outlive every batch)` ·
`references/adapter-shapes.md:346 (Five ingredients)` ·
[SPECIFICATION PS-5](../../spec/SPECIFICATION.md) ·
[adapter-shapes §6](../../references/adapter-shapes.md) ·
[rust-lang/rust#158983](https://github.com/rust-lang/rust/issues/158983) *(checked 2026-08-09, rustc 1.97.1)*

---

## RS-92-2. When rustc ICEs on a port impl, copy the trait locally to read the diagnostic.

**Why.** The crash is in the *error-reporting* path, not in the check: rustc has
already found the region error and panics while explaining a bound whose item
lives in another crate. Move the trait declaration into your own crate and the
same code reports `error[E0477]` with the offending lifetime named.

**Do** — a local paste of the four ingredients, which is the whole diagnostic:

```rust,compile_fail,E0477
trait Store {
    type Batch<'a> where Self: 'a;
    fn commit(&self, batch: Self::Batch<'_>) -> impl Sized;
}

struct Borrowing<'a>(&'a ());

impl Store for Borrowing<'_> {
    type Batch<'a> = () where Self: 'a;
    fn commit(&self, _batch: Self::Batch<'_>) -> impl Sized {}
}
# fn main() {}
```

**Not** — "fixing" the reproduction. Drop `where Self: 'a` from your local copy
and it compiles, which feels like a diagnosis and is not one: the bound belongs
to the port, in another crate, and is still there. The check that catches it is
rebuilding the real impl, which ICEs again:

```rust
trait Store {
    // The port's clause, dropped. This is now a different trait.
    type Batch<'a>;
    fn commit(&self, batch: Self::Batch<'_>) -> impl Sized;
}

struct Borrowing<'a>(&'a ());

impl Store for Borrowing<'_> {
    type Batch<'a> = ();
    fn commit(&self, _batch: Self::Batch<'_>) -> impl Sized {}
}

fn main() { let owned = (); let _ = Borrowing(&owned); }
```

**What would open it up.** The same issue closing. The check is mechanical: put
the borrowing impl back against the real port and rebuild. An ICE means the dead
end is still there; `error[E0477]` means rustc can now explain a foreign item's
bound and this technique is no longer needed.

**Rejects.** An hour spent bisecting toolchains, clearing `target/`, and
reinstalling rustup against a panic that names only `compare_impl_item` and a
`DefId` — because nothing in the message says the error is in the impl's self
type. The local paste turns that into one line of `E0477` naming the lifetime,
and it is the difference between a five-minute diagnosis and a lost afternoon.

**Evidence.** `references/adapter-shapes.md:325 (rustc crashes **while diagnosing a region error**)` ·
`crates/happenstance-testkit/src/contract.rs:97 (owned associated type and not a GAT)` ·
`crates/happenstance-testkit/src/contract.rs:110 (refcount instead of borrowing a lifetime)`

---

## RS-92-3. Do not design around return-type notation.

**Why.** `S: Port<head(..): Send>` is `error[E0658]: return type notation is
experimental` on every stable toolchain, and rustc's own note points at tracking
issue #109417. The two-flavour derivation plus hand-written associated-type
bounds is not a workaround waiting to be replaced; it is the design.

**Do** — the bound RTN would have abbreviated, written out (atom 21 owns the
shape):

```rust
use happenstance_core::{MemoryEventStore, SendEventStore};

fn runner<S>(_store: S)
where
    S: SendEventStore + Send + Sync + 'static,
    S::Error: Send,
{
}

fn main() { let _ = runner::<MemoryEventStore>; }
```

**Not** — the RTN spelling. `error[E0658]`:

```rust,compile_fail,E0658
trait Port {
    async fn head(&self) -> u64;
}

fn spawnable<P>(_port: P)
where
    P: Port<head(..): Send>,
{
}
# fn main() {}
```

**What would open it up.** Compile the fence above on the pinned toolchain: when
it stops failing, RTN has stabilised. Do not infer that from a blog post or a
project-goals update. This repository's own research is dated evidence, not a
status: at 2026-08-05 it recorded the stabilisation PR closed unmerged and no
successor, which is a reason to write the bounds by hand today and not a
prediction about next year.

**Rejects.** A port redesign deferred — "we will collapse the two flavours once
RTN lands" — so `SendEventStore` is left undocumented as provisional, adapters
are written against whichever flavour is convenient, and the collapse never
arrives. Every month of that is a month of impls that have to be split into two
types later (RS-20-4), and the deferral is invisible in the code because there is
nothing in the tree that names it. ES-1 freezes the scheme besides, so the
collapse was never a refactor waiting on a toolchain — it is a new ADR.

**Evidence.** `references/evaluation/research-rust-api-guidelines.md:47 (Do not design around it)` ·
`crates/happenstance-core/src/store.rs:93 (trait_variant::make)` ·
[SPECIFICATION ES-1](../../spec/SPECIFICATION.md) ·
[ADR-0001](../../.kb/decisions/0001-async-port-flavours.md) ·
[rust-lang/rust#109417](https://github.com/rust-lang/rust/issues/109417), which
is the issue rustc's own note names *(checked 2026-08-09, rustc 1.97.1)*

