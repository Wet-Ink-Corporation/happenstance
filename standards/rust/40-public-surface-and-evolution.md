# 40 — Public surface and evolution

> **Load when:** adding a method to a port · an adapter stops compiling after a
> dependency bump · E0046 / E0119 / E0638 / E0433 / E0080 in a downstream crate ·
> deciding whether a change needs a major version · a fixture must decline an
> operation
> **See also:** 13 (sealing and exhaustiveness) · 41 (an exported macro is
> surface too) · 50 (dependency hygiene) · 91 (adapter authoring)

---

## RS-40-1. Grow a port with a new trait, never with a new required method.

**Why.** Every trait method needs a body somewhere, and `#[non_exhaustive]` does
not apply to traits — so a new required method is `error[E0046]` in every
adapter's crate, on a *minor* bump. A supertrait plus a blanket impl adds nothing
to the port, so it reaches adapters that were published before it existed.

**Do**

```rust
use core::future::Future;
use happenstance_core::{EventStore, MemoryEventStore};

trait EventStoreExt: EventStore {
    fn is_vacant(&self) -> impl Future<Output = Result<bool, Self::Error>> {
        async { Ok(self.head().await?.is_none()) }
    }
}
impl<S: EventStore + ?Sized> EventStoreExt for S {}

# fn main() {
let store = MemoryEventStore::new();
assert!(matches!(happenstance_testkit::block_on(store.is_vacant()), Ok(true)));
# }
```

**Not** — an adapter written against yesterday's port, after a third method
landed on it:

```rust,compile_fail,E0046
# use futures_core::Stream;
# use happenstance_core::{AppendCondition, AppendError, Event, EventStore, MemoryStoreError, Query, ReadOptions, SequencePosition, SequencedEvent};
struct Adapter;

impl EventStore for Adapter {
    type Error = MemoryStoreError;
#     fn read(&self, _q: &Query, _o: ReadOptions)
#         -> impl Stream<Item = Result<SequencedEvent, Self::Error>> { futures_util::stream::empty() }
#     async fn append(&self, _e: &[Event], _c: Option<&AppendCondition>)
#         -> Result<SequencePosition, AppendError<Self::Error>> { Err(AppendError::NoEvents) }
    // `read` and `append` are written. `head` and `contains_event_id` are not:
    // error[E0046]: missing `head`, `contains_event_id`
}
# fn main() {}
```

**Rejects.** A convenience added to `EventStore` in what its author versions as
0.1.1. Every adapter crate outside this workspace stops compiling on the next
`cargo update`, in *their* crate, with a diagnostic that names a method they have
never heard of — and the `semver` CI job did not warn, because it diffs the pull
request against its own base SHA and the break is inside the diff only if
somebody bothered to look at the job's output.

**Evidence.** `crates/happenstance-core/src/store.rs:303 (async fn head)` ·
`.github/workflows/ci.yml:588 (baseline-rev)` ·
[ADR-0008](../../.kb/decisions/0008-one-derivation-for-both-ports.md) ·
[research §12](../../references/evaluation/research-rust-api-guidelines.md) *(dated evidence)* ·
[cargo-semver-checks 0.50](https://github.com/obi1kenobi/cargo-semver-checks) —
does not detect breaking type changes, generic/lifetime changes, or breakage
visible only under a feature subset *(checked 2026-08-09, rustc 1.97.1)*

## RS-40-2. Put a derivable convenience on the blanket ext trait; put anything an adapter could do faster on the port.

**Why.** `impl<S: EventStore + ?Sized> Ext for S {}` covers every type at once, so
a second impl for one adapter is `error[E0119]` — the provided body is not a
default, it is the *only* body. A method whose honest implementation is
`SELECT max(position)` on one store and a scan on another therefore cannot live
there.

**Do**

```rust
use core::future::Future;
use happenstance_core::EventStore;

// Derived from `read` alone, identical for every store: safe to blanket.
trait EventStoreExt: EventStore {
    fn is_vacant(&self) -> impl Future<Output = Result<bool, Self::Error>> {
        async { Ok(self.head().await?.is_none()) }
    }
}
impl<S: EventStore + ?Sized> EventStoreExt for S {}
# fn main() {}
```

**Not** — an adapter trying to install its index behind the same name:

```rust,compile_fail,E0119
# use core::future::Future;
# use happenstance_core::{EventStore, MemoryEventStore};
# trait EventStoreExt: EventStore {
#     fn is_vacant(&self) -> impl Future<Output = Result<bool, Self::Error>> {
#         async { Ok(self.head().await?.is_none()) }
#     }
# }
# impl<S: EventStore + ?Sized> EventStoreExt for S {}
impl EventStoreExt for MemoryEventStore {
    fn is_vacant(&self) -> impl Future<Output = Result<bool, Self::Error>> {
        async { Ok(false) } // error[E0119]: conflicting implementations
    }
}
# fn main() {}
```

**Rejects.** `head` shipped as a blanket `read(&Query::all(), backwards().limit(1))`.
It compiles, it is correct, and every adapter that owns a `max(position)` index is
permanently locked out of using it — the SQLite author discovers this while
profiling a projection runner that is doing a full table scan per poll, and the
only fix is a breaking change to a trait somebody else's crates implement.

**Evidence.** `crates/happenstance-core/src/store.rs:277 (Why this is required rather than provided)` ·
`crates/happenstance-core/src/store.rs:303 (async fn head)` ·
[adapter-shapes §2.2](../../references/adapter-shapes.md) *(the `E0119` row)* ·
[ADR-0008](../../.kb/decisions/0008-one-derivation-for-both-ports.md)

## RS-40-3. Return a `#[non_exhaustive]` struct wherever a tuple would freeze the arity.

**Why.** A tuple's arity is written into every caller's pattern, so a fifth part
is a breaking change with no signature that avoids it. `#[non_exhaustive]` makes
a cross-crate exhaustive pattern `error[E0638]` and a struct literal
`error[E0639]`, which is what makes *adding* a field additive: every existing
pattern already carries `..`.

**Do**

```rust
use happenstance_core::{Event, EventParts};

# fn main() -> Result<(), Box<dyn core::error::Error>> {
let event = Event::new("Enrolled", &b"{}"[..])?;
let EventParts { event_type, data, .. } = event.into_parts();
assert_eq!(event_type.as_str(), "Enrolled");
assert_eq!(data.len(), 2);
# Ok(())
# }
```

**Not** — the pattern a caller writes when nothing stops them:

```rust,compile_fail,E0638
# use happenstance_core::{Event, EventParts};
# fn main() -> Result<(), Box<dyn core::error::Error>> {
let event = Event::new("Enrolled", &b"{}"[..])?;
let EventParts { event_type, data, tags, metadata } = event.into_parts();
# let _ = (event_type, data, tags, metadata);
# Ok(())
# }
```

**Rejects.** `into_parts` returning `(EventType, Bytes, Tags, Option<Bytes>)`.
The day a per-store `ingested_at` or a redaction flag joins the event, every
ingest path in every peer crate breaks at once — and the author of the addition
sees a clean workspace build, because nothing in this repository destructures it.

**Evidence.** `crates/happenstance-core/src/event.rs:416 (not public API)` ·
`crates/happenstance-core/src/event.rs:435 (non_exhaustive)` ·
[SPECIFICATION VT-4](../../spec/SPECIFICATION.md) *(the same attribute, on
`SequencedEvent`, and why `new` is then the whole compatibility surface)*

## RS-40-4. Name a signature's types through the defining crate's own re-export.

**Why.** `happenstance-core` does `pub use bytes;` and `pub use futures_core;` on
purpose: it costs the crate a major bump whenever either takes one, and buys the
guarantee that a caller and an adapter cannot be holding two `Bytes` or two
`Stream`s that look identical. The re-export set is not a courtesy — it is
*exactly* the crates whose types appear in that crate's own public signatures,
which is why each adapter re-exports its **driver** (`happenstance-sqlite` its
`rusqlite`, `happenstance-cloudflare` its `worker`) and the contract
re-exports neither. Reach outside a crate's set and the first symptom is
`error[E0433]`; reach around it, with a copy of your own, and the symptom is
worse.

**Do**

```rust
use happenstance_core::bytes::Bytes;
use happenstance_core::futures_core::Stream;

// `Stream` is at the *top level* of `EventStore::read`'s signature, so an
// adapter cannot implement the port without naming it — through this path, or
// through a second `futures-core` that nothing unifies with this one.
fn readable<S: Stream>(_s: S) {}

# fn main() -> Result<(), Box<dyn core::error::Error>> {
let payload: Bytes = Bytes::from_static(b"{}");
let event = happenstance_core::Event::new("Enrolled", payload)?;
assert_eq!(event.data().len(), 2);
readable(futures_util::stream::empty::<u8>());
# Ok(())
# }
```

**Not** — a re-export lives on the crate whose *signatures* name the type, so a
path through the wrong crate is refused at the path rather than three steps later:

```rust,compile_fail,E0433
// `rusqlite` is in `happenstance-sqlite`'s constructors and error enums and in
// none of the contract's. `happenstance_sqlite::rusqlite` resolves; this is what
// asking the crate one layer down costs.
fn open(_c: happenstance_core::rusqlite::Connection) {}
# fn main() {}
```

**Rejects.** An adapter crate that ignores the re-export, adds
`futures-core = "0.3"` of its own and, one `cargo update` later, resolves a
different major than `happenstance-core` did. The two `Stream` traits print
identically, so `impl EventStore for MyStore` fails with `error[E0277]: the trait
bound … is not satisfied` naming a trait the author can see is implemented — a
diagnostic that sends people to rewrite the adapter rather than to read
`cargo tree -d`. The re-export does not *prevent* the second copy; it makes the
first one nameable, which is the only reason anyone reaches for it.

**What a re-export is not, and this half is load-bearing.** It is a
**type-identity and discoverability** guarantee and nothing else. It does not
forward the *features* a consumer did not enable, and it is not a substitute for
their own dependency line. Say so at the re-export site, because the reader who
needs the sentence arrives at the item, not at this file.

**And when the crate is taken at a partial feature set, that sentence is not
enough — decline the re-export instead.** The rule's arithmetic is a *necessary*
condition, not a sufficient one: a crate whose types appear in your public
signatures is a *candidate* for the set, and it earns its place only if the path
you hand the reader is shorter than the one they would have walked anyway.
`happenstance-sqlite` is the worked case, and it went the other way. Its error
enums carry `tokio::task::JoinError` and `tokio::runtime::TryCurrentError`, so
`tokio` qualifies on the arithmetic — and it *was* re-exported. But the crate
takes `tokio` at `features = ["rt"]`, so `happenstance_sqlite::tokio` was a
partial `tokio`, and a consumer who reached it and then wrote `#[tokio::main]`
met an `error[E0433]` *further* from its cause than the `error[E0308]` the
re-export existed to prevent. The re-export was removed at `0.2.0`; the
consumer writes their own `tokio` line, and cargo unifies it for every
semver-compatible requirement.

Two things follow, and the second is the one people get wrong. **State the
omission where the reader looks for the item**, not only where you decided it —
`crates/happenstance-sqlite/src/lib.rs`'s `reexported_paths` says why `tokio` is
absent, in the same doc that proves the others resolve. And **fence it**: a
`compile_fail,E0433` doctest on the path a reader following the old
documentation would take, so that re-adding the re-export turns a test red
rather than passing unnoticed. Put the fence in the **lib**, never in an
integration-test target — cargo never hands those to a compiler, and a fence
that is never compiled is decoration (F1-04, and
[`80-the-gate.md`](80-the-gate.md)).

**Evidence.** `crates/happenstance-core/src/lib.rs:186 (pub use bytes)` ·
`crates/happenstance-core/src/lib.rs:193 (pub use futures_core)` ·
`crates/happenstance-sqlite/src/lib.rs:143 (pub use rusqlite)` ·
`crates/happenstance-sqlite/src/lib.rs:185 (compile_fail,E0433)` ·
`crates/happenstance-cloudflare/src/lib.rs:578 (pub use {happenstance_core, worker})` ·
`crates/happenstance-core/src/store.rs:178 (impl Stream<Item = Result<SequencedEvent, Self::Error>>)` ·
[ADR-0003](../../.kb/decisions/0003-opaque-payloads.md)

## RS-40-5. Spell an optional capability as an associated `const` whose constructor rejects an empty reason.

**Why.** The obvious `enum Capability { Supported, Declined(&'static str) }` admits
`Declined("")`, so the reason becomes prose. A private field forces every value
through one `const fn`, where an `assert!` is a const-evaluation panic —
`error[E0080]` at compile time for a free `const`, but **at codegen** for an
associated one, which `cargo check` and `cargo clippy` never reach.

**Do**

```rust
use happenstance_testkit::Capability;

const REOPEN: Capability = Capability::declined(
    "this store is volatile: nothing outlives the last handle, so a reopen has \
     no durably committed state to observe",
);

# fn main() {
assert!(!REOPEN.is_supported());
assert!(REOPEN.reason().is_some());
# }
```

**Not**

```rust,compile_fail,E0080
use happenstance_testkit::Capability;

const REOPEN: Capability = Capability::declined("");
# fn main() { let _ = REOPEN; }
```

**Rejects.** An adapter author who meets a red `acknowledged_writes_survive_a_reopen`,
declines `REOPEN` to go green, and — because the constant is *associated* — gets
a clean `cargo clippy` before anyone runs `cargo test`. Written as a bare `bool`
the trade would leave no line in the CI log at all, and the reviewer approving
the pull request would see a green build and thirty-four fewer rules than they
thought they had.

**Evidence.** `crates/happenstance-testkit/src/contract.rs:1006 (pub const fn declined)` ·
`crates/happenstance-testkit/src/contract.rs:999 (Where it does *not* fire)` ·
`crates/happenstance-testkit/src/fixtures.rs:282 (const REOPEN)` ·
[SPECIFICATION CF-18](../../spec/SPECIFICATION.md) *(why a declined
capability still emits a reported test)*
