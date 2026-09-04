# 24 — The blocking bridge

> **Load when:** calling a synchronous driver from async code · `spawn_blocking`
> panicked · "there is no reactor running" · a connection is `Send` but not
> `Sync` · deciding what `read` may do before the first poll · a driver handle
> will not go in a struct field
> **See also:** 23 (the state machine it lives in) · 25 (what removes `Send`) ·
> 30 (why the poison guard becomes a unit variant) · 20 (flavours)

---

## RS-24-1. `read` must not spawn; defer every `spawn_blocking` to the first `poll_next`.

**Why.** `read` is not `async` (ES-2), so it runs on whatever thread called it,
which need not be inside a runtime — and `tokio::task::spawn_blocking` panics
when no runtime is in thread-local scope. `poll_next` by definition runs under an
executor, so the first poll is the earliest line at which the call is legal, and
ES-11 is what makes deferring to it conformant rather than merely convenient.

**Do** — `read` only captures; the spawn is a line in `poll_next`:

```rust
struct Plan { path: String }

enum State {
    Idle(Box<Plan>),
    Fetching(tokio::task::JoinHandle<usize>),
    Done,
}

// Touches no runtime. Callable from a bare thread.
fn read(path: &str) -> State {
    State::Idle(Box::new(Plan { path: path.to_owned() }))
}

fn poll(state: &mut State) {
    if let State::Idle(plan) = core::mem::replace(state, State::Done) {
        // The earliest line at which a runtime is guaranteed.
        *state = State::Fetching(tokio::task::spawn_blocking(move || plan.path.len()));
    }
}

let _ = (read, poll);
```

**Not** — the eager spawn compiles. The assertion that catches it is at the end
of the fence:

```rust
fn read_eagerly() -> tokio::task::JoinHandle<usize> {
    tokio::task::spawn_blocking(|| 0)
}

assert!(tokio::runtime::Handle::try_current().is_err(), "no runtime on this thread");
let panicked = std::panic::catch_unwind(|| { let _ = read_eagerly(); }).is_err();
assert!(panicked, "spawn_blocking outside a runtime panics");
```

**Rejects.** An adapter whose `read` spawns eagerly panics in a caller that
builds its streams before entering the runtime — a projection runner that opens
one stream per subscription during setup and only then calls
`Runtime::block_on`. The message names tokio and a line inside the adapter the
author has no test for, because a rule that only ever calls `read` from inside a
`#[tokio::test]` cannot distinguish the eager spawn from the deferred one.

**Evidence.** `crates/happenstance-sqlite/src/event_store.rs:14 (tokio::task::spawn_blocking)` ·
`crates/happenstance-sqlite/src/event_store.rs:24 (Laziness stops being a nicety)` ·
`crates/happenstance-sqlite/src/event_store.rs:1007 (Nothing is executed here on purpose)` ·
[ES-2](../../spec/SPECIFICATION.md) ·
[ES-11](../../spec/SPECIFICATION.md) ·
[ADR-0001](../../.kb/decisions/0001-async-port-flavours.md) ·
[ADR-0011](../../.kb/decisions/0011-read-laziness-and-isolation.md)

## RS-24-2. Turn a runtime-less poll into a stream item error with `Handle::try_current`.

**Why.** Deferring the spawn leaves one residual risk: the first poll may still
happen under a non-tokio executor, where `spawn_blocking` panics anyway — and
CF-23 obliges the testkit to demonstrate a runtime-free harness, so that executor
is not hypothetical. `try_current` returns a `TryCurrentError` instead of
panicking, and a stream is already allowed to surface failures as `Err` items.

**Do**

```rust
# use core::pin::Pin;
# use core::task::{Context, Poll};
use futures_core::Stream;

struct Reads { polled: bool }

impl Stream for Reads {
    type Item = Result<u32, tokio::runtime::TryCurrentError>;

    fn poll_next(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.get_mut();
        if this.polled {
            return Poll::Ready(None);
        }
        this.polled = true;
        match tokio::runtime::Handle::try_current() {
            Err(err) => Poll::Ready(Some(Err(err))),
            Ok(_runtime) => Poll::Ready(Some(Ok(0))),
        }
    }
}

// The testkit's runtime-free executor: the failure arrives as an item.
let first = happenstance_testkit::block_on(async {
    use futures_util::StreamExt as _;
    Reads { polled: false }.next().await
});
assert!(first.is_some_and(|item| item.is_err()));
```

**Not** — `Handle::current()` compiles and panics inside `poll_next`; the
assertion that catches it drives the same non-tokio executor:

```rust
let panicked = happenstance_testkit::block_on(async {
    std::panic::catch_unwind(|| { let _ = tokio::runtime::Handle::current(); }).is_err()
});
assert!(panicked, "Handle::current panics off-runtime; try_current returns Err");
```

**Rejects.** A stream that panics rather than yielding `Err` converts a
recoverable environment mistake into an aborted task. The caller polling under
`futures::executor::block_on` — or under the conformance suite's own
runtime-free emitter — gets a panic naming tokio rather than the adapter, and no
variant of `Self::Error` to match on, so the retry path the port's error model
exists for is unreachable.

**Evidence.** `crates/happenstance-sqlite/src/event_store.rs:30 (turns that from a panic into an)` ·
`crates/happenstance-sqlite/src/event_store.rs:1529 (Handle::try_current())` ·
`crates/happenstance-sqlite/src/event_store.rs:928 (no tokio runtime is available)` ·
`crates/happenstance-testkit/src/registry.rs:320 (deliberately not bounded on)` ·
[CF-23](../../spec/SPECIFICATION.md)

## RS-24-3. Put a `Send + !Sync` connection behind a `Mutex`.

**Why.** ES-3 puts the `Self: Sync` requirement at the point of use rather than
on the trait, so a store whose futures hold `&self` across an await needs `Sync`
from its own fields; std is what supplies it here, with
`impl<T: ?Sized + Send> Sync for Mutex<T>`. The mutex is therefore what makes the
store usable from the generic code that binds it — and it is simultaneously what
makes the adapter serialise its writers, which is a shape choice worth stating
rather than discovering.

**Do** — stand-in for `rusqlite::Connection`, which is `Send` and not `Sync`;
`rusqlite` is not a dependency of the crate these examples compile in:

```rust
use core::cell::Cell;
use std::sync::{Arc, Mutex};

struct Conn(Cell<u64>);

struct Store { conn: Arc<Mutex<Conn>> }

// `&T: Send` is exactly `T: Sync`, so this is the bound a generic caller asks for.
fn assert_ref_is_send<T: Sync>() {}
assert_ref_is_send::<Store>();
```

**Not** — without the mutex, `&Store` is not `Send`, and the diagnostic is
`error[E0277]` naming the `Cell`, not the store:

```rust,compile_fail,E0277
# use core::cell::Cell;
# use std::sync::Arc;
struct Conn(Cell<u64>);
struct Store { conn: Arc<Conn> }

fn assert_ref_is_send<T: Sync>() {}
assert_ref_is_send::<Store>();
```

**Rejects.** `Sync` is also purchasable for free by opening a connection inside
every method, which compiles, needs no mutex, and reads like an improvement. On a
file it silently gives up the serialisation the mutex was providing, so two
writers can be assigned the same position with nothing in the type system
objecting; on `Connection::open_in_memory()` it is worse, because each call gets
its own empty database and an `append` followed by a `head` on one store handle
disagree.

**Evidence.** `crates/happenstance-sqlite/src/event_store.rs:146 (but **not**)` ·
`crates/happenstance-sqlite/src/event_store.rs:148 (is what makes)` ·
[ES-3](../../spec/SPECIFICATION.md) ·
[VT-11](../../spec/SPECIFICATION.md) ·
[adapter-shapes §2.1](../../references/adapter-shapes.md) ·
[std::marker::Send](https://doc.rust-lang.org/std/marker/trait.Send.html) *(checked 2026-08-09, rustc 1.97.1)*

## RS-24-4. Keep every handle that borrows the connection inside the blocking closure.

**Why.** `Statement`, `Rows` and `Transaction` are `!Send` in the synchronous
drivers. One of them in a state field costs the stream its `Send` and with it the
`SendEventStore` impl. What crosses the closure boundary is owned data: the
cursor, and a `Vec` of decoded rows.

**Do**

```rust
use core::marker::PhantomData;

struct Row(u64);
// !Send, like rusqlite::Statement.
struct Stmt<'a> { conn: &'a mut u64, _local: PhantomData<*const ()> }

struct Cursor { conn: u64 }

impl Cursor {
    // The borrowing handle is created and dropped inside one call.
    fn page(&mut self) -> Vec<Row> {
        let stmt = Stmt { conn: &mut self.conn, _local: PhantomData };
        vec![Row(*stmt.conn)]
    }
}

struct ReadStream { rows: std::vec::IntoIter<Row> }

fn assert_send<T: Send>() {}
assert_send::<ReadStream>();
let _ = Cursor::page;
```

**Not** — the handle kept as a field, so `error[E0277]`: `*const ()` cannot be
sent between threads safely.

```rust,compile_fail,E0277
# use core::marker::PhantomData;
# struct Stmt<'a> { conn: &'a mut u64, _local: PhantomData<*const ()> }
struct ReadStream<'a> { stmt: Stmt<'a> }

fn assert_send<T: Send>() {}
assert_send::<ReadStream<'static>>();
```

**Rejects.** An author caching the prepared statement across pages — a real
optimisation, and the obvious one — gets a `!Send` stream whose diagnostic lands
on the `impl SendEventStore for` block rather than on the field that caused it.
The shortest way to make it compile is to implement the bare `EventStore`
instead, which passes the entire conformance suite and quietly removes the
adapter from every consumer that spawns, which is the one thing the two-flavour
split exists to keep available.

**Evidence.** `crates/happenstance-sqlite/src/event_store.rs:1179 (boundary would)` ·
`crates/happenstance-sqlite/src/event_store.rs:1534 (runtime.spawn_blocking(move ||)` ·
[ES-2](../../spec/SPECIFICATION.md) ·
[adapter-shapes §2.1](../../references/adapter-shapes.md)
