# 90 — Skeletons, and what todo!() does not prove

> **Load when:** starting an adapter crate before its driver exists · deciding
> what a `todo!()` body may stand in for · `clippy::todo` fired and you are about
> to silence it · `dead_code` on an item nothing constructs yet · a crate
> compiles and you are about to call it an adapter
> **See also:** 91 (the adapter recipe) · 92 (the ICE these types are shaped
> around) · 20 (which flavour) · 81 (checks that cannot be types)

---

## RS-90-1. Declare the associated types for real; `todo!()` only the bodies.

**Why.** `todo!()` has type `!`, which coerces to every return type, so no body
can contradict a signature. The associated types are the only part of a bodiless
impl a type checker can disagree with, and stubbing one does not defer the
disagreement — it deletes it.

**Do** — bind the batch type the driver actually offers, here an owned buffer:

```rust
#[trait_variant::make(SendPort: Send)]
trait Port {
    type Batch<'a> where Self: 'a;
    async fn begin(&self) -> Self::Batch<'_>;
    async fn commit(&self, batch: Self::Batch<'_>);
}

/// What `sqlx::Transaction<'static, Postgres>`, a Cypher statement list and a
/// buffered `(SQL, params)` vector all reduce to.
struct WriteSet(Vec<String>);

struct Store;

impl SendPort for Store {
    // The GAT lifetime is satisfiable and unused. That is a recorded result,
    // not a smell — see the postgres citation below.
    type Batch<'a> = WriteSet where Self: 'a;
    async fn begin(&self) -> Self::Batch<'_> { todo!("phase 8 fills this in") }
    async fn commit(&self, _batch: Self::Batch<'_>) { todo!("phase 8") }
}

fn assert_send_flavour<P: SendPort>() {}

fn main() { assert_send_flavour::<Store>(); }
```

**Not** — the live borrowed handle `rusqlite` forces, bound to the same GAT. The
`Send` flavour rejects it, and the diagnostic **carries no error code**:
`error: future cannot be sent between threads safely`. Declare
`type Batch<'a> = ();` instead and this compiles, in silence:

```rust,compile_fail
use std::rc::Rc;
# #[trait_variant::make(SendPort: Send)]
# trait Port {
#     type Batch<'a> where Self: 'a;
#     async fn begin(&self) -> Self::Batch<'_>;
#     async fn commit(&self, batch: Self::Batch<'_>);
# }
/// Stands in for `rusqlite::Transaction<'a>`: a live handle on a `!Sync`
/// connection. The crate is deliberately not a dependency of `xtask`.
struct LiveHandle<'a>(Rc<str>, &'a String);

struct Store(String);

impl SendPort for Store {
    type Batch<'a> = LiveHandle<'a> where Self: 'a;
    async fn begin(&self) -> Self::Batch<'_> { LiveHandle(Rc::from("tx"), &self.0) }
    async fn commit(&self, _batch: Self::Batch<'_>) {}
}
# fn main() {}
```

**Rejects.** A skeleton that writes `type Error = std::io::Error;` and
`type Batch<'a> = ();` "until the driver lands", is listed in the repository map
as an instrument, and is cited in a freeze as evidence that the port fits its
storage shape. It fits nothing: the impl is six `todo!()`s over two placeholders,
and the driver's real types are met by whoever writes the bodies — one phase
after the port was frozen against them.

**Evidence.** `crates/happenstance-postgres/src/projection_store.rs:100 (Transaction<'static, Postgres>)` ·
`crates/happenstance-sqlite/src/projection_store.rs:216 (rustc accepts it present or absent)` ·
`docs/adapter-shapes.md:11 (stubbed the only part)` ·
`docs/adapter-shapes.md:98 (code: None)` ·
[ADR-0008](../adr/0008-one-derivation-for-both-ports.md)

---

## RS-90-2. Where the skeleton makes a claim, write real bodies.

**Why.** `!` satisfies the type checker without satisfying anything else, so an
impl of `todo!()`s proves a signature is *nameable*, not that it can be
*inhabited*. Only a body that runs distinguishes the two.

**Do** — the claim is "a borrowed live handle works on the `Send` flavour", so
the three methods that carry it are written out and driven:

```rust
use core::sync::atomic::{AtomicUsize, Ordering};

use happenstance_testkit::block_on;
# #[trait_variant::make(SendPort: Send)]
# trait Port {
#     type Batch<'a> where Self: 'a;
#     async fn begin(&self) -> Self::Batch<'_>;
#     async fn commit(&self, batch: Self::Batch<'_>) -> usize;
# }

struct Db { committed: AtomicUsize }
struct Handle<'a> { db: &'a Db, rows: usize }
struct Store { db: Db }

impl SendPort for Store {
    type Batch<'a> = Handle<'a> where Self: 'a;
    async fn begin(&self) -> Self::Batch<'_> { Handle { db: &self.db, rows: 0 } }
    async fn commit(&self, batch: Self::Batch<'_>) -> usize {
        batch.db.committed.fetch_add(batch.rows, Ordering::SeqCst) + batch.rows
    }
}

fn main() {
    let store = Store { db: Db { committed: AtomicUsize::new(0) } };
    // Fully qualified: both flavour names are in scope here, so method syntax
    // is `error[E0034]` (RS-20-3).
    let total = block_on(async {
        let mut batch = SendPort::begin(&store).await;
        batch.rows = 3;
        SendPort::commit(&store, batch).await
    });
    assert_eq!(total, 3);
}
```

**Not** — the same impl with `todo!()` bodies. It compiles, and the fence ends
with the assertion that says what the compile was worth:

```rust
use core::sync::atomic::AtomicUsize;
use std::panic::{AssertUnwindSafe, catch_unwind};

use happenstance_testkit::block_on;
# #[trait_variant::make(SendPort: Send)]
# trait Port {
#     type Batch<'a> where Self: 'a;
#     async fn begin(&self) -> Self::Batch<'_>;
#     async fn commit(&self, batch: Self::Batch<'_>) -> usize;
# }
# struct Db { committed: AtomicUsize }
# struct Handle<'a> { db: &'a Db, rows: usize }
# struct Store { db: Db }

impl SendPort for Store {
    type Batch<'a> = Handle<'a> where Self: 'a;
    async fn begin(&self) -> Self::Batch<'_> { todo!("phase 11") }
    async fn commit(&self, _batch: Self::Batch<'_>) -> usize { todo!("phase 11") }
}

fn main() {
    std::panic::set_hook(Box::new(|_| {}));
    let store = Store { db: Db { committed: AtomicUsize::new(0) } };
    let outcome = catch_unwind(AssertUnwindSafe(|| block_on(SendPort::begin(&store))));
    assert!(outcome.is_err(), "`!` type-checked; nothing else happened");
}
```

**Rejects.** An ADR that cites a skeleton as the counter-example to "the borrowed
GAT batch cannot be implemented on the `Send` flavour" when every body in it is
`todo!()`. The citation survives review because the crate compiles and the impl
is right there; it collapses at the phase that writes the bodies and finds the
handle cannot be constructed at all, by which time the clause it supported is
frozen.

**Evidence.** `crates/happenstance-ladybug/src/live_handle.rs:188 (Real bodies from here down)` ·
`crates/happenstance-ladybug/src/live_handle.rs:26 (with real bodies, not)` ·
`crates/happenstance-sync/src/lib.rs:9 (checker is the instrument)` ·
[adapter-shapes §2.2](../adapter-shapes.md)

---

## RS-90-3. Scope every skeleton suppression to the crate, and give it an expiry.

**Why.** `clippy::todo` is `deny` in the workspace manifest precisely so a
skeleton has to opt out in its own root, where review sees it. `#[expect]` goes
further than `#[allow]`: rustc reports `this lint expectation is unfulfilled`
when the lint stops firing, so the suppression fails the build on the day it
becomes false rather than outliving the code it covered.

**Do**

```rust
/// Both variants are matched and both payloads read — as neon's own `append`
/// does — so the only `dead_code` left is *never constructed*, and that is the
/// one that stops firing on the day the decoder lands. Leave a payload unread
/// and `dead_code` goes on firing about the field, which keeps the expectation
/// fulfilled and the suppression alive forever.
#[expect(
    dead_code,
    reason = "constructed by decode_append_response, which is todo!() until phase 10"
)]
enum AppendOutcome {
    Appended(u64),
    Conflict(u64),
}

fn position(outcome: &AppendOutcome) -> u64 {
    match outcome {
        AppendOutcome::Appended(at) | AppendOutcome::Conflict(at) => *at,
    }
}

fn decode() -> AppendOutcome { todo!("phase 10") }

fn main() {
    let _ = decode as fn() -> AppendOutcome;
    let _ = position as fn(&AppendOutcome) -> u64;
}
```

**Not** — `#[allow]`, which is silent in both directions:

```rust
#[allow(dead_code)]
enum AppendOutcome {
    Appended(u64),
    Conflict(u64),
}

fn position(outcome: &AppendOutcome) -> u64 {
    match outcome {
        AppendOutcome::Appended(at) | AppendOutcome::Conflict(at) => *at,
    }
}

fn main() {
    // The decoder landed. `#[expect]` here reports itself unfulfilled; the
    // attribute above is now a lie and nothing says so.
    let _ = position(&AppendOutcome::Appended(1));
    let _ = position(&AppendOutcome::Conflict(2));
}
```

No fence above can show what catches it, and that is a property of the harness
rather than of the rule: rustdoc compiles doctests under `allow(unused)` and
clippy does not lint them at all (RS-01-4). The catcher is `cargo xtask ci`'s
clippy step under `-D warnings`, which promotes this warn-by-default lint to an
error. The diagnostic carries no error code, and rustc echoes the `reason`
string back:

```text
error: this lint expectation is unfulfilled
   --> crates/happenstance-neon/src/event_store.rs:239:5
    |
239 |     dead_code,
    |     ^^^^^^^^^
    |
    = note: constructed by decode_append_response, which is todo!() until phase 10
    = note: `#[warn(unfulfilled_lint_expectations)]` on by default
```

**Rejects.** A workspace-wide `todo = "allow"` added for "intentionally
unimplemented stub crates". This repository shipped one; a grep found no
`todo!()` anywhere in the tree, so the allow protected nothing and stood ready to
accept the first real one silently. The same shape one level down is an
`#[allow(dead_code)]` on a variant a later phase starts constructing — the
attribute then hides the *next* dead variant, and nobody learns that until a
reviewer reads the enum.

**Evidence.** `./Cargo.toml:125 (the allow protected nothing)` ·
`crates/happenstance-ladybug/src/lib.rs:71 (Phase 11 removes both the bodies)` ·
`crates/happenstance-neon/src/event_store.rs:239 (constructed by decode_append_response)`

---

## RS-90-4. Record a capability limit beside the type that has it; no diagnostic will.

**Why.** Nothing in `append`'s signature constrains how many round trips it
takes, so a probe-then-write implementation type-checks exactly as readily as an
atomic one. A shape table assembled from `error[E….]` rows therefore ranks the
least capable adapter in the workspace as the most compatible.

**Do** — one statement, one snapshot, one round trip; and the residual limit
written into the crate's own rustdoc as a table, because the type cannot carry
it:

```rust
#[trait_variant::make(SendTransport: Send)]
trait Transport {
    /// One request, one response. No cursor, no interactive transaction, and
    /// nothing held open between calls.
    async fn round_trip(&self, sql: &str) -> Option<u64>;
}

struct Store<T>(T);

impl<T: Transport> Store<T> {
    /// A CTE computes the probe and the insert on one snapshot and projects
    /// both, so the verdict and the write cannot be separated.
    async fn append(&self) -> Option<u64> {
        self.0.round_trip("WITH probe AS (…), ins AS (…) SELECT …").await
    }
}

fn main() { let _ = Store::<Never>::append; }
# struct Never;
# impl Transport for Never {
#     async fn round_trip(&self, _sql: &str) -> Option<u64> { None }
# }
```

**Not** — probe, then write. Two round trips, two implicit transactions, a
network-latency-wide window between them, and no compiler complaint. What
rejects it is `happenstance_testkit::event_store_concurrency_conformance!` — a
second caller on a real thread — not a type:

```rust
# #[trait_variant::make(SendTransport: Send)]
# trait Transport {
#     async fn round_trip(&self, sql: &str) -> Option<u64>;
# }
# struct Store<T>(T);
impl<T: Transport> Store<T> {
    async fn append(&self) -> Result<u64, u64> {
        // Round trip one. Its implicit transaction has committed and dropped
        // its snapshot by the time this await resolves.
        if let Some(conflict) = self.0.round_trip("SELECT min(position) …").await {
            return Err(conflict);
        }
        // Round trip two. Nothing connects it to round trip one.
        self.0.round_trip("INSERT …").await.ok_or(0)
    }
}
# fn main() { let _ = Store::<Never>::append; }
# struct Never;
# impl Transport for Never {
#     async fn round_trip(&self, _sql: &str) -> Option<u64> { None }
# }
```

**Rejects.** A portfolio table built by running `cargo check` over six skeletons
and recording the errors. `happenstance-neon` — no connection, no interactive
transaction, no cursor, one round trip per operation, a hard 64 MiB response cap
— produces none, so the table shows it fully compatible and a freeze cites it as
the far end of the transport axis. The limits are found by whoever writes the
bodies, against a port already frozen on the strength of their absence.

**Evidence.** `crates/happenstance-neon/src/event_store.rs:390 (It compiles, and that is the finding)` ·
`crates/happenstance-neon/src/lib.rs:42 (would therefore rank this crate)` ·
`docs/adapter-shapes.md:207 (limits that are not type errors)` ·
[adapter-shapes §3](../adapter-shapes.md)
