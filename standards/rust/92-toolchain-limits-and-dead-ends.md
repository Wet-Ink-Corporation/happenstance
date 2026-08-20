# 92 — Toolchain limits and dead ends

> **Load when:** `thread 'rustc' panicked` on an adapter impl · a panic naming
> `compare_impl_item` and a `DefId` you did not write · `error[E0658]: return
> type notation is experimental` · designing around a feature that "will land
> soon"
> **See also:** 21 (`Send` bounds by hand) · 22 (RPITIT and lifetime capture) ·
> 62 (doctest annotations, and why an error code is not a pin) · 90 (skeletons) ·
> 01 (the standard of evidence)

Every rule here is a dead end plus the check that says when it has opened up.
Without that check the atom becomes folklore, which is the failure it exists to
prevent. The check has now fired once — see **Retired**, at the foot of this
atom — which is the evidence that the discipline is load-bearing rather than
ceremonial.

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
`crates/happenstance-testkit/src/contract.rs:108 (owned associated type and not a GAT)` ·
`crates/happenstance-testkit/src/contract.rs:121 (refcount instead of borrowing a lifetime)`

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
`crates/happenstance-core/src/store.rs:141 (trait_variant::make)` ·
[SPECIFICATION ES-1](../../spec/SPECIFICATION.md) ·
[ADR-0001](../../.kb/decisions/0001-async-port-flavours.md) ·
[rust-lang/rust#109417](https://github.com/rust-lang/rust/issues/109417), which
is the issue rustc's own note names *(checked 2026-08-09, rustc 1.97.1)*

---

## Retired

**RS-92-1 — "A type implementing a port with a batch GAT MUST be `'static`; a
lifetime on it ICEs rustc 1.97.1."** Retired 2026-08-13 by its own opening
condition, which read: *"`rust-lang/rust#158983` closing, or PS-5 landing —
`type Batch;` deletes ingredient two and the whole exposure with it."* PS-5
landed with ADR-0017. The ICE needed five ingredients together — a trait in a
foreign crate, a GAT with `where Self: 'a`, an RPITIT return, `Self::Batch<'_>`
in that method's signature, and a non-`'static` impl self type — and
`ProjectionStore` was the only port in the workspace supplying the first four.
It now declares `type Batch;`, so no port here can supply ingredient two and no
store in this workspace can reach the crash by writing a lifetime.

**The upstream bug is not fixed, and this is not a claim that it is.**
`rust-lang/rust#158983` is open; `experiments/rustc-ice-gat-foreign-trait/` and
its `bisect.sh` still reproduce on 1.85.1, 1.97.1 and 1.99.0-nightly across
editions 2018, 2021 and 2024. What changed is the *reach*: the workspace no
longer has a port that can be hit this way, so the rule constrains nothing an
author of this repository could write, and RS-01-1 is what retires it. RS-92-2
above stays, because reading an ICE by pasting the trait locally is technique
that outlives this particular crash.

If a GAT ever returns to a port here, this rule comes back with it — as a new
id, not this one. The id is spent.
