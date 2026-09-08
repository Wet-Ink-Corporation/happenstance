# 23 — Streams and hand-written state machines

> **Load when:** writing `poll_next` by hand · E0507 moving a cursor out of a
> state enum · E0597/E0515 holding a driver handle beside its connection · rustc
> asks for pin projection · `async fn drop` · the stream type has no name to
> assert about
> **See also:** 24 (the blocking bridge) · 25 (what removes `Send`) · 22 (what
> RPITIT captures) · 61 (asserting `Send`)

---

## RS-23-1. Make every field of a hand-written `Stream` `Unpin`.

**Why.** `unsafe_code = "forbid"` removes pin projection, so the only route from
`Pin<&mut Self>` into the struct is `Pin::get_mut` / `Pin::into_inner`, both
bounded `Self: Unpin`. A struct is `Unpin` exactly when every field is, so the
obligation lands on the fields rather than on the impl. The port asks for no
`Unpin` at all (ES-42) — this one is the implementor's, and it is discharged
inside the adapter or not at all.

**Do**

```rust
# use core::pin::Pin;
# use core::task::{Context, Poll};
use futures_core::Stream;

// Every field Unpin: an owned iterator, a Pin<Box<_>>, a shared reference.
struct ReadStream(std::vec::IntoIter<u32>);

impl Stream for ReadStream {
    type Item = u32;
    fn poll_next(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<u32>> {
        Poll::Ready(self.get_mut().0.next())
    }
}
```

**Not** — a `!Unpin` field, so `get_mut` is `error[E0277]`. `PhantomPinned`
stands in for what actually arrives here: the anonymous future of an inline
`async` block stored directly in a state variant.

```rust,compile_fail,E0277
# use core::pin::Pin;
use core::marker::PhantomPinned;

struct ReadStream {
    rows: std::vec::IntoIter<u32>,
    step: PhantomPinned,
}

fn next(stream: Pin<&mut ReadStream>) -> Option<u32> {
    stream.get_mut().rows.next()
}
```

**Rejects.** The author who cannot get past E0277 and boxes the *result set*
instead of the step future: first poll collects every row into a `Vec`, which is
`Unpin`, so `poll_next` compiles and every conformance rule passes — they all run
against logs of tens of events. The buffering is invisible until the first
production backfill loads a million-event replay into memory, which is the
outcome returning a stream at all exists to prevent.

**Evidence.** `crates/happenstance-sqlite/src/event_store.rs:1514 (needs no pin)` ·
`crates/happenstance-postgres/src/read_stream.rs:318 (the design above, and it is what makes every field)` ·
`crates/happenstance-neon/src/event_store.rs:1051 (Every field is)` ·
`crates/happenstance-testkit/src/registry.rs:306 (the alternative — hand-writing a)` ·
[ES-42](../../spec/SPECIFICATION.md)

## RS-23-2. Take the state by value with `mem::replace`, never match it through `&mut`.

**Why.** `poll_next` holds `&mut Self`, and a cursor owned by a state variant
cannot be moved out from behind that borrow into a `'static` closure or a boxed
step future — `error[E0507]`. Replacing the state with the terminal variant moves
it out legally and leaves the enum inhabited; every arm either restores a live
state or is genuinely terminal.

**Do**

```rust
struct Cursor { fetched: u32 }
enum State { Idle(Box<Cursor>), Done }

fn step(state: &mut State) -> u32 {
    match core::mem::replace(state, State::Done) {
        State::Idle(mut cursor) => {
            cursor.fetched += 1;
            let fetched = cursor.fetched;
            *state = State::Idle(cursor);
            fetched
        }
        State::Done => 0,
    }
}

let mut state = State::Idle(Box::new(Cursor { fetched: 0 }));
assert_eq!(step(&mut state), 1);
assert_eq!(step(&mut state), 2);
```

**Not** — `error[E0507]`, cannot move out of `*cursor` behind a mutable
reference:

```rust,compile_fail,E0507
# struct Cursor { fetched: u32 }
# enum State { Idle(Box<Cursor>), Done }
fn consume(_cursor: Box<Cursor>) -> u32 { 0 }

fn step(state: &mut State) -> u32 {
    match state {
        State::Idle(cursor) => consume(*cursor),
        State::Done => 0,
    }
}
```

**Rejects.** The author who meets E0507 and reaches for `Option<Box<Cursor>>`
plus `take()`. The discriminant and the payload can now disagree, so
`Fetching(None)` is representable, and a spurious wake — which any executor may
deliver — ends the stream mid-replay. A truncated read is indistinguishable from
a legitimately short one, so the consumer rebuilds a decision model from half a
log and appends against it.

**Evidence.** `crates/happenstance-sqlite/src/event_store.rs:1983 (std::mem::replace(&mut this.state)` ·
`crates/happenstance-postgres/src/read_stream.rs:323 (Taking the state by value)` ·
[ES-11](../../spec/SPECIFICATION.md) *(no visible event may be omitted)*

## RS-23-3. Hold the owner and re-borrow it per step; store nothing that borrows a connection.

**Why.** A driver's `fetch` borrows its executor, and `read` is not `async`, so at
construction time there is no connection to borrow. A value holding both the
connection and something borrowed from it is self-referential and the borrow
checker refuses it — `error[E0597]` at the shipped `sqlx` call site. Moving the
owner into each step future and taking it back with the rows keeps every borrow
inside one `await`, which is a borrow the compiler can see the end of.

**Do** — stand-in for `sqlx::Transaction<'static, Postgres>`, since `sqlx` is not
a dependency of the crate these examples compile in:

```rust
struct Cursor { rows: u32 }
struct Rows<'a>(&'a mut Cursor);

fn fetch(cursor: &mut Cursor) -> Rows<'_> { Rows(cursor) }

// By value in, by value out: the `&mut` begins and ends inside the call.
async fn step(mut cursor: Box<Cursor>) -> (Box<Cursor>, u32) {
    let fetched = fetch(&mut cursor).0.rows;
    (cursor, fetched)
}

let (cursor, fetched) = happenstance_testkit::block_on(step(Box::new(Cursor { rows: 7 })));
assert_eq!(fetched, 7);
assert_eq!(cursor.rows, 7);
```

**Not** — `error[E0597]`, the shape the module docs quote verbatim: the borrow
outlives the connection it came from.

```rust,compile_fail,E0597
# struct Pool;
# struct Conn;
# struct Rows<'a>(&'a mut Conn);
# impl Pool { fn acquire(&self) -> Conn { Conn } }
# fn fetch(conn: &mut Conn) -> Rows<'_> { Rows(conn) }
fn read(pool: &Pool) -> usize {
    let rows;
    {
        let mut conn = pool.acquire();
        rows = fetch(&mut conn);
    }
    let _ = rows;
    0
}
```

**Rejects.** The fix that compiles and is wrong: abandon the cursor and re-issue
`WHERE position > $last ORDER BY position LIMIT n` per chunk on a freshly
borrowed connection. Nothing outlives anything and no bound objects, so the type
checker signs it off — and the read now self-paginates across independent
snapshots, so appends landing between chunks appear mid-stream and a replay its
consumer believes is one state of the store silently is not. The compiler is the
wrong instrument here and the suite is the right one: this shape is the
registered mutant `RefetchingPagedStore`, and it fails
`read_result_is_stable_under_concurrent_append` and
`query_items_share_one_snapshot`. ES-11 is what it violates — capture a position
ceiling no later than the first poll and bound every later statement by it.

**Evidence.** `crates/happenstance-postgres/src/read_stream.rs:26 (does not live long enough)` ·
`crates/happenstance-postgres/src/read_stream.rs:458 (taking the cursor by value and handing it back)` ·
`crates/happenstance-testkit/tests/mutation_coverage.rs:1849 (name: "RefetchingPagedStore")` ·
[ES-11](../../spec/SPECIFICATION.md) ·
[ADR-0011](../../.kb/decisions/0011-read-laziness-and-isolation.md) ·
[adapter-shapes §2.1](../../references/adapter-shapes.md)

## RS-23-4. Give the stream a name; do not build it from `unfold` or an `async` block.

**Why.** `futures_util::stream::unfold` returns `Unfold<T, F, Fut>`, parameterised
by an unnameable closure type, and a coroutine's auto traits are *inferred* from
what it captures. Both make `Send` a property of the body rather than of the
declaration, so there is no type at which to write the obligation down.

**Do**

```rust
# use core::pin::Pin;
# use core::task::{Context, Poll};
use futures_core::Stream;

pub struct ReadStream(std::vec::IntoIter<u32>);

impl Stream for ReadStream {
    type Item = u32;
    fn poll_next(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<u32>> {
        Poll::Ready(self.get_mut().0.next())
    }
}

fn assert_send<T: Send>() {}
assert_send::<ReadStream>();
```

**Not** — this compiles, and that is the problem. No assertion in the fence
catches it; the test that would, `read_stream_is_send`, cannot be written at all,
because there is no type to name.

```rust
use futures_core::Stream;
use futures_util::stream;

fn rows() -> impl Stream<Item = u32> {
    stream::unfold(0u32, |n| async move { (n < 3).then(|| (n, n + 1)) })
}

fn assert_send<T: Send>(_: &T) {}
assert_send(&rows());
```

**Rejects.** An adapter whose read stream is an anonymous `impl Stream` carries
no `read_stream_is_send` test, so its `Send` claim rests on today's captures. The
first capture that is not `Send` — an `Rc<Query>` cloned into the closure to
avoid a lifetime — moves the diagnostic to the `impl SendEventStore for` block
tens of lines away, names a hidden type nobody chose, and offers no field to
point at; the plausible-looking fix is to drop the adapter to the bare flavour,
which removes it from every consumer that spawns.

**Evidence.** `crates/happenstance-postgres/src/read_stream.rs:54 (unnameable)` ·
`crates/happenstance-postgres/src/read_stream.rs:659 (read_stream_is_send)` ·
`crates/happenstance-cloudflare/src/send_shape.rs:34 (a coroutine's auto traits are inferred)` ·
[ES-2](../../spec/SPECIFICATION.md)

## RS-23-5. There is no async `Drop`; compensation must be done before the future can be dropped.

**Why.** `Drop::drop` is `fn drop(&mut self)`; writing it `async` changes the
return type to a future and rustc rejects the impl with `error[E0053]`. A
destructor's entire budget is therefore synchronous, and a future destroyed at a
suspension point spends none of it — whatever rows the state machine already
wrote stay written, which is the partial batch ES-22 forbids, so atomicity is
bought at the statement that writes them, never at cleanup.

**Do** — synchronous release in `Drop`, and the round trip as an awaited method
the caller drives while it is still polling:

```rust
struct Cursor { open: bool }

impl Drop for Cursor {
    // Return a pooled connection, drop a refcount, roll back by dropping the
    // transaction. No round trip, because none can be awaited here.
    fn drop(&mut self) { self.open = false; }
}

impl Cursor {
    async fn close(mut self) -> bool {
        self.open = false;
        true
    }
}

assert!(happenstance_testkit::block_on(Cursor { open: true }.close()));
```

**Not** — first `error[E0053]`: `drop` has an incompatible type for the trait,
`expected ()`, `found future`.

```rust,compile_fail,E0053
struct Cursor;

impl Drop for Cursor {
    async fn drop(&mut self) {}
}
```

Then the shape that ships, because it compiles: the destructor builds the closing
future and drops it unpolled, so the body never runs. The assertion at the foot of
the fence is what catches it.

```rust
use core::cell::Cell;
use std::rc::Rc;

struct Cursor { closed: Rc<Cell<bool>> }

impl Cursor {
    async fn close(&self) { self.closed.set(true); }
}

impl Drop for Cursor {
    fn drop(&mut self) { let _ = self.close(); }
}

let closed = Rc::new(Cell::new(false));
drop(Cursor { closed: Rc::clone(&closed) });
assert!(!closed.get(), "the future was built and dropped; the body never ran");
```

**Rejects.** `YieldingRowAtATimeStore` is the registered mutant this shape ends
at: one `INSERT` per row, awaited, the `BEGIN` forgotten, and nothing a destructor
could compensate with, because the rollback needs an await it cannot have. At the
edge a dropped future is the *normal*
termination — a client disconnect, a CPU limit, a Durable Object eviction — so the
rows already written stay written and no `Result` exists for anyone to read.
`dropped_append_future_leaves_no_partial_batch` is the only thing that reports it,
and only for an adapter that has run the suite.

**Evidence.** `crates/happenstance-testkit/tests/mutation_coverage/mutants.rs:1895 (YieldingRowAtATimeStore)` ·
`crates/happenstance-postgres/src/read_stream.rs:257 (Dropping the cursor drops the transaction)` ·
[ES-22](../../spec/SPECIFICATION.md) ·
[E0053](https://doc.rust-lang.org/error_codes/E0053.html) *(checked 2026-08-09, rustc 1.97.1)*
