# 91 — The adapter authoring recipe

> **Load when:** starting a new event store adapter · asked to "wire up" a
> driver · writing a `Fixture` · `event_store_conformance!` will not expand ·
> deciding what a fixture may decline · a conformance rule fails and you want to
> skip it
> **See also:** 20 (which flavour) · 21 (`Send` obligations) · 22 (RPITIT and
> `E0195`) · 23 (the read stream) · 24 (the blocking bridge) · 40 (an associated
> `const`'s `assert!` fires at codegen) · 90 (skeletons) · 92 (dead ends)

Six steps, in order. Steps 2–4 belong to other atoms and are named here only so
the order is on record: **1.** pick the flavour (RS-91-1 below, and 20). **2.** declare the
associated types for real before any body (90), and check the `Send` obligations
a runner will need (21). **3.** decide where the read stream's laziness lives
before you write `read` (23, 24). **4.** write the `Fixture` — one instance is
one isolated backing store, which is
[CF-15](../../spec/SPECIFICATION.md)'s sentence and not this atom's. Steps
5–6 are RS-91-3 and RS-91-4 below.

---

## RS-91-1. Implement the strongest flavour your own types allow.

**Why.** The blanket `impl<T: SendEventStore> EventStore for T` runs one way, so
implementing `SendEventStore` gives you `EventStore` free while implementing the
bare flavour gives you nothing extra. Nothing can add `Send` afterwards, so the
choice is made once, at the impl, and every caller inherits it.

**Do** — the native adapter's shape. Write `+ Send` on the returned stream at the
impl, as `MemoryEventStore` does; that is the bound the whole two-trait design
exists to keep reachable:

```rust
# use core::pin::Pin;
# use core::task::{Context, Poll};
# use futures_core::Stream;
use happenstance_core::{
    AppendCondition, AppendError, Event, EventId, Query, ReadOptions, SendEventStore,
    SequencePosition, SequencedEvent,
};
# #[derive(Debug)]
# struct PgError(String);
# impl core::fmt::Display for PgError {
#     fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result { f.write_str(&self.0) }
# }
# impl core::error::Error for PgError {}
# struct PgStream;
# impl Stream for PgStream {
#     type Item = Result<SequencedEvent, PgError>;
#     fn poll_next(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
#         Poll::Ready(None)
#     }
# }

struct PgStore;

impl SendEventStore for PgStore {
    type Error = PgError;

    fn read(
        &self,
        _query: &Query,
        _options: ReadOptions,
    ) -> impl Stream<Item = Result<SequencedEvent, Self::Error>> + Send {
        PgStream
    }

    async fn append(
        &self,
        _events: &[Event],
        _condition: Option<&AppendCondition>,
    ) -> Result<SequencePosition, AppendError<Self::Error>> {
        todo!("phase 10")
    }
    async fn head(&self) -> Result<Option<SequencePosition>, Self::Error> { todo!("phase 10") }
    async fn contains_event_id(&self, _id: EventId) -> Result<bool, Self::Error> {
        todo!("phase 10")
    }
}

/// The obligation, instantiated. A bound that merely parses proves nothing.
fn spawnable<S: SendEventStore + Send + Sync + 'static>() {}

fn main() { spawnable::<PgStore>(); }
```

**Not** — the same store implementing the bare flavour "because that is the one
generic code binds". `error[E0277]`: nothing promotes `EventStore` to
`SendEventStore`, so every caller that wants `tokio::spawn` is locked out:

```rust,compile_fail,E0277
# use core::pin::Pin;
# use core::task::{Context, Poll};
# use futures_core::Stream;
use happenstance_core::{
    AppendCondition, AppendError, Event, EventId, EventStore, Query, ReadOptions,
    SendEventStore, SequencePosition, SequencedEvent,
};
# #[derive(Debug)]
# struct PgError(String);
# impl core::fmt::Display for PgError {
#     fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result { f.write_str(&self.0) }
# }
# impl core::error::Error for PgError {}
# struct PgStream;
# impl Stream for PgStream {
#     type Item = Result<SequencedEvent, PgError>;
#     fn poll_next(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
#         Poll::Ready(None)
#     }
# }
# struct PgStore;
impl EventStore for PgStore {
    type Error = PgError;
    fn read(
        &self,
        _query: &Query,
        _options: ReadOptions,
    ) -> impl Stream<Item = Result<SequencedEvent, Self::Error>> {
        PgStream
    }
    async fn append(
        &self,
        _events: &[Event],
        _condition: Option<&AppendCondition>,
    ) -> Result<SequencePosition, AppendError<Self::Error>> { todo!() }
    async fn head(&self) -> Result<Option<SequencePosition>, Self::Error> { todo!() }
    async fn contains_event_id(&self, _id: EventId) -> Result<bool, Self::Error> { todo!() }
}

fn spawnable<S: SendEventStore + Send + Sync + 'static>() {}

fn main() { spawnable::<PgStore>(); }
```

**Rejects.** A pooled Postgres adapter written against the bare flavour because
the author read RS-20-2 and applied it to the impl rather than to the bound. It
compiles, passes the whole suite, and is unusable from any projection runner that
spawns — discovered by the first application that runs two projections
concurrently, in a published crate whose only fix is a new impl on a new type.

**Evidence.** `crates/happenstance-core/src/memory.rs:300 (impl Stream<Item = Result<SequencedEvent, Self::Error>> + Send)` ·
`crates/happenstance-testkit/src/fixtures.rs:186 (a handle that quietly weakened)` ·
[SPECIFICATION ES-1](../../spec/SPECIFICATION.md) ·
[ADR-0001](../../.kb/decisions/0001-async-port-flavours.md)

---

## RS-91-3. Write the fixture's constants knowing the type cannot tell a MUST from a trade.

**Why.** `Capability` is one opaque type with one supported value and one
declining constructor, and the identical spelling stands at `SECOND_HANDLE`,
which CF-16 makes a MUST, and at `REOPEN`, which CF-17 makes a trade: nothing at
the impl site separates them, and the suite is what does — `must!` against
`require!`. The three ceilings are a different type for that same reason (CF-40):
`Option<usize>`, never `Capability`. The empty-reason `assert!` on an *associated*
const fires at codegen rather than at `cargo check`, which is RS-40-5's and
applies to every constant below.

**Do**

```rust
use core::future::Future;

use happenstance_testkit::fixtures::MemoryFixture;
use happenstance_testkit::{Capability, Fixture};

/// The inner fixture stands in for a pool; the constants are the point.
struct PgFixture(MemoryFixture);

impl Fixture for PgFixture {
    type Store = <MemoryFixture as Fixture>::Store;

    const SECOND_HANDLE: Capability = Capability::SUPPORTED;

    // The real reason, not "unsupported": it is printed on every run.
    const REOPEN: Capability = Capability::declined(
        "the throwaway schema is dropped when the pool closes, so reopening \
         cannot distinguish durable rows from discarded ones",
    );

    // A fact about the store, not a decline, and a different type for it.
    const MAX_EVENT_DATA_LEN: Option<usize> = Some(1_073_741_824);

    fn connect(&self) -> impl Future<Output = Self::Store> {
        self.0.connect()
    }
}

fn main() { let _ = PgFixture(MemoryFixture::new()); }
```

**Not** — declining `SECOND_HANDLE` to turn a red rule green. It compiles, being
the same three tokens of Rust as the decline above it, and
`two_handles_observe_each_others_appends` **panics** rather than skipping,
quoting the reason back:

```rust
# use core::future::Future;
# use happenstance_testkit::fixtures::MemoryFixture;
use happenstance_testkit::{Capability, Fixture};
# struct PgFixture(MemoryFixture);

impl Fixture for PgFixture {
    type Store = <MemoryFixture as Fixture>::Store;

    // CF-16 makes this a MUST, and a MUST is not skippable.
    const SECOND_HANDLE: Capability = Capability::declined("we hold one connection");
    const REOPEN: Capability = Capability::declined("volatile");

    fn connect(&self) -> impl Future<Output = Self::Store> { self.0.connect() }
}
# fn main() { let _ = PgFixture(MemoryFixture::new()); }
```

**Rejects.** An adapter author whose store *does* survive a reopen, who meets a
red `acknowledged_writes_survive_a_reopen` caused by a bug in their own teardown,
and who declines `REOPEN` rather than fix it. That decline is legal — `REOPEN` is
`require!`d, not `must!`ed, so the rule reports the stated reason and the suite
goes green — and the diff reads as configuration, so the reviewer approves it.
Durability is then unchecked for that adapter until someone loses data and reads
the fixture.

**Evidence.** `crates/happenstance-testkit/src/contract.rs:146 (This one is a MUST)` ·
`crates/happenstance-testkit/src/contract.rs:61 (A limit is a **fact**)` ·
[SPECIFICATION CF-16](../../spec/SPECIFICATION.md) ·
[SPECIFICATION CF-18](../../spec/SPECIFICATION.md) ·
[SPECIFICATION CF-40](../../spec/SPECIFICATION.md)

---

## RS-91-4. It is not an adapter until `event_store_conformance!` has run against it.

**Why.** The macro takes an expression that builds a **`Fixture`** — not a store,
and not the pre-phase-3 `factory =` keyword, which is gone with no deprecated arm
— and expands to one `#[tokio::test]` per rule, so a failure names the rule.
Nothing else in the workspace decides whether an adapter exists.

**Do**

```rust
use happenstance_testkit::fixtures::MemoryFixture;

happenstance_testkit::event_store_conformance!(MemoryFixture::new());

fn main() {}
```

**Not** — the adapter's own integration test, which is what an author reaches for
first. It compiles and it passes, and it catches none of: an append condition
evaluated without consulting tags, positions assigned outside the transaction, an
acknowledged write lost on reopen, or a second handle that cannot see the first's
appends:

```rust
use happenstance_core::{Event, EventStore, MemoryEventStore, Query, ReadOptions, collect};

#[tokio::test]
async fn it_round_trips() {
    let store = MemoryEventStore::new();
    if let Ok(event) = Event::new("Issued", &b"{}"[..]) {
        let _ = store.append(&[event], None).await;
    }
    let seen = collect(store.read(&Query::all(), ReadOptions::new())).await;
    assert_eq!(seen.map(|events| events.len()), Ok(1));
}

fn main() {}
```

It is also written against a *store*, so every defect that lives in the
**fixture** is invisible to it — CF-15's among them. `MemoryFixture::sharing` is
that shape in memory, and only
`two_fixture_instances_observe_none_of_each_others_appends` reports it; the fence
ends with the leak asserted, which is what the rule sees:

```rust
use std::sync::Arc;

use happenstance_core::{Event, EventStore, MemoryEventStore, Query, ReadOptions, collect};
use happenstance_testkit::fixtures::MemoryFixture;
use happenstance_testkit::{Fixture, block_on};

fn main() {
    block_on(async {
        let shared = Arc::new(MemoryEventStore::new());
        let first = MemoryFixture::sharing(Arc::clone(&shared));
        let second = MemoryFixture::sharing(shared);

        let store = first.connect().await;
        if let Ok(event) = Event::new("Issued", &b"{}"[..]) {
            let _ = store.append(&[event], None).await;
        }

        let other = second.connect().await;
        let seen = collect(other.read(&Query::all(), ReadOptions::new())).await;
        assert_eq!(seen.map(|events| events.len()), Ok(1), "the isolation leak");
    });
}
```

**Rejects.** The state all six skeletons in this workspace are in right now: a
crate that compiles, is listed in the repository map, has a rustdoc page claiming
a target, and has never run a conformance rule. Calling one of them "the SQLite
adapter" in a release note is the failure — and if a rule looks wrong once you do
run it, the change is to the rule, in the same commit, with the wrong
implementation it now rejects added to the testkit's own `tests/`.

**Evidence.** `crates/happenstance-testkit/src/lib.rs:13 (not considered to exist)` ·
`crates/happenstance-testkit/src/lib.rs:352 (Migrating from)` ·
`crates/happenstance-testkit/src/fixtures.rs:252 (Creates a fixture over an **existing** store)` ·
`references/adapter-shapes.md:302 (A skeleton falsifies a signature)` ·
[SPECIFICATION CF-1](../../spec/SPECIFICATION.md) ·
[SPECIFICATION CF-15](../../spec/SPECIFICATION.md)
