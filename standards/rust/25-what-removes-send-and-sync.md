# 25 — What removes Send, and what removes Sync

> **Load when:** "cannot be sent between threads safely" · "cannot be shared
> between threads safely" · a store will not satisfy `SendEventStore` · a future
> is `!Send` and nothing in it looks it · E0277 naming `Rc`, `Cell` or `RefCell` ·
> choosing the type for a deliberately `!Send` instrument
> **See also:** 21 (`Send` is not inherited) · 24 (the `Mutex` that buys it) ·
> 20 (flavours) · 61 (observing an absence)

---

## RS-25-1. `Rc` removes `Send`; `RefCell` removes `Sync` and keeps `Send`.

**Why.** std carries `impl<T: Send + ?Sized> Send for RefCell<T>` and an explicit
negative `impl<T: ?Sized> !Sync for RefCell<T>` — the absence is written down, not
merely unimplemented. `Rc` carries negative impls for both, for every `T`, so no
choice of payload recovers either.

**Do**

```rust
use core::cell::RefCell;

fn assert_send<T: Send>() {}
assert_send::<RefCell<Vec<u32>>>();
```

**Not** — `error[E0277]`, and the word in the message is *shared*, not *sent*:

```rust,compile_fail,E0277
use core::cell::RefCell;

fn assert_sync<T: Sync>() {}
assert_sync::<RefCell<Vec<u32>>>();
```

**Rejects.** The agent told to build the `!Send` witness CF-28 requires reaches
for `RefCell`, writes `assert_send::<Store>()` to check its work, and the probe
compiles — so the check reads as green while establishing the opposite of what it
was written for. Nothing downstream distinguishes the two stores: the suite
passes either way, and only an *absence* probe (61), or the `Rc` the reference
store actually holds, would have reported it.

**Evidence.** `crates/happenstance-testkit/tests/local_conformance.rs:77 (RefCell<Vec<SequencedEvent>>)` ·
`crates/happenstance-cloudflare/src/js.rs:46 (is what surrenders)` ·
[CF-28](../../spec/SPECIFICATION.md) ·
[std::cell::RefCell](https://doc.rust-lang.org/std/cell/struct.RefCell.html) *(checked 2026-08-09, rustc 1.97.1)*

## RS-25-2. `&T: Send` requires `T: Sync` — that is the real bound on a captured `&self`.

**Why.** std: `impl<T: Sync + ?Sized> Send for &T`, against `impl<T: Send +
?Sized> Send for &mut T`. A shared reference needs `Sync`; a mutable one needs
only `Send`. ES-3 is where that lands on the port, and it lands on `&self` rather
than on any value a method body holds.

**Do**

```rust
use core::cell::Cell;
use std::sync::Mutex;

fn assert_send<T: Send>() {}
assert_send::<&Mutex<Cell<u64>>>();
```

**Not** — `error[E0277]`: the reference is rejected because the referent is not
`Sync`.

```rust,compile_fail,E0277
use core::cell::Cell;

fn assert_send<T: Send>() {}
assert_send::<&Cell<u64>>();
```

**Rejects.** The author reading "future cannot be sent between threads safely" on
`checkpoint` — a method that touches no batch, no cursor and no connection —
searches the body for the offending value and does not find one, because the
value is the `&self` the signature captured. Phase 2 produced four such
diagnostics at once against a store owning a `rusqlite::Connection` directly, one
per method of the trait, and the fix was a single `Mutex` at the field rather
than anything in any body.

**Evidence.** `crates/happenstance-core/src/memory.rs:657 (across an await, which needs)` ·
`crates/happenstance-sqlite/src/event_store.rs:147 (flavour captures)` ·
[ES-3](../../spec/SPECIFICATION.md) ·
[adapter-shapes §2.1](../../references/adapter-shapes.md) ·
[std::marker::Send](https://doc.rust-lang.org/std/marker/trait.Send.html) *(checked 2026-08-09, rustc 1.97.1)*

## RS-25-3. `Arc<T>: Send` requires `T: Send + Sync`, so `Arc` around a `!Sync` value buys nothing.

**Why.** std: `impl<T: Sync + Send + ?Sized, A: Allocator + Send> Send for
Arc<T, A>`. Both halves are load-bearing — `Arc` hands `&T` to other threads, and
the last clone drops `T` on whichever thread it lands on.

**Do**

```rust
use core::cell::RefCell;
use std::sync::{Arc, Mutex};

fn assert_send<T: Send>() {}
assert_send::<Arc<Mutex<RefCell<u32>>>>();
```

**Not** — `error[E0277]`: the failure is the `Sync` half, and the name in the
message is `RefCell`, not `Arc`.

```rust,compile_fail,E0277
use core::cell::RefCell;
use std::sync::Arc;

fn assert_send<T: Send>() {}
assert_send::<Arc<RefCell<u32>>>();
```

**Rejects.** `Arc` is the right reflex when a stream must outlive the `&self`
that produced it — `SqliteReadStream` holds `Arc<Mutex<Connection>>` for exactly
that. Reaching for `Arc<RefCell<_>>` because the store needs interior mutability
and `Arc` is "the thread-safe one" produces a type that is neither `Send` nor
`Sync`, and the shortest escape from the diagnostic is to implement the bare
`EventStore` instead. That compiles, passes the whole suite, and ships an adapter
no consumer can spawn.

**Evidence.** `crates/happenstance-sqlite/src/event_store.rs:153 (is not for sharing the store; it is so that a)` ·
`crates/happenstance-sqlite/src/event_store.rs:1116 (is what makes the whole)` ·
[ES-3](../../spec/SPECIFICATION.md) ·
[std::marker::Send](https://doc.rust-lang.org/std/marker/trait.Send.html) *(checked 2026-08-09, rustc 1.97.1)*

## RS-25-4. Collapse a value with no `Send` bound before the next `await`.

**Why.** A future's auto traits are decided by what is alive across its
suspension points, not by what it returns. ES-6 leaves `EventStore::Error`
unbounded, so a `Result<_, S::Error>` still in scope at the next `await` puts a
possibly-`!Send` type in the witness set and `tokio::spawn` rejects the whole
future.

**Do**

```rust
use std::rc::Rc;

async fn read_len() -> Result<usize, Rc<str>> { Ok(1) }
async fn append() {}

async fn run() -> usize {
    // Collapsed in one statement: the Result is dead before the await below.
    let seen = read_len().await.map_or(0, |len| len);
    append().await;
    seen
}

fn spawn_it() -> tokio::task::JoinHandle<usize> { tokio::spawn(run()) }
let _ = spawn_it;
```

**Not** — `error: future cannot be sent between threads safely`, blamed on
`tokio::spawn` rather than on the binding; the `help` line names `Rc<str>` and a
`note` names the await that `seen` is held across. This whole diagnostic family
carries **no error code** — `--message-format=json` reports `code: None` — so
this is the one fence in the corpus a `compile_fail,E####` tag cannot pin, and
the compiling `Do` above is the only positive control it has.

```rust,compile_fail
# use std::rc::Rc;
# async fn read_len() -> Result<usize, Rc<str>> { Ok(1) }
# async fn append() {}
async fn run() -> usize {
    let seen = read_len().await;
    append().await;
    seen.map_or(0, |len| len)
}

fn spawn_it() -> tokio::task::JoinHandle<usize> { tokio::spawn(run()) }
```

**Rejects.** The read-then-append runner compiles today only because
`collect(stream).await` is folded to a `usize` before the second await. Rebind it
— `let events = collect(stream).await?` — and the runner stops compiling against
*every* adapter simultaneously, in the runner's own crate, with a diagnostic
pointing at `tokio::spawn` and naming `S::Error`, a type the runner's author
never chose and cannot change. The fix is at the bind site; the search starts at
the adapters.

**Evidence.** `crates/happenstance-core/src/memory.rs:679 (held across the *next* await would make)` ·
`crates/happenstance-core/src/memory.rs:684 (a second await against the same borrow)` ·
`crates/happenstance-cloudflare/src/send_shape.rs:7 (on that future's)` ·
[ES-6](../../spec/SPECIFICATION.md) ·
[adapter-shapes §2.1](../../references/adapter-shapes.md) *(these six diagnostics carry no error code)*

## RS-25-5. Build a `!Send` instrument out of `Rc`, never out of an inherited `unsafe impl Send`.

**Why.** ES-6 settles which type the `!Send` instrument holds, and why the one
`unsafe impl Send` this workspace could have inherited was not it. What is left
to the code is the consequence: an auto trait acquired under a `cfg` states a
fact about the target, so only a type carrying a negative impl on *every* target
— `Rc`, here — answers the same on host and on `wasm32`.

**Do** — `Rc<str>`, unconditional on every target the gate builds. Observing an
*absence* needs autoref specialisation (61); the positive control is what stops
the probe passing vacuously.

```rust
use core::marker::PhantomData;
use std::rc::Rc;

struct Probe<T>(PhantomData<T>);
trait NotSend { fn is_send(&self) -> bool { false } }
impl<T> NotSend for Probe<T> {}
impl<T: Send> Probe<T> { fn is_send(&self) -> bool { true } }

struct JsHandle { repr: Rc<str> }

assert!(Probe::<u32>(PhantomData).is_send(), "positive control");
assert!(!Probe::<JsHandle>(PhantomData).is_send());
let _ = JsHandle { repr: Rc::from("[object SqlStorage]") };
```

**Not** — compiles, and the last assertion says what it actually established. The
`cfg` models what holding a real `JsValue` buys, without the `unsafe impl` the
workspace forbids:

```rust
#[cfg(not(target_feature = "atomics"))]
type Inherited = u32;
#[cfg(target_feature = "atomics")]
type Inherited = std::rc::Rc<u32>;

struct Handle(Inherited);

fn assert_send<T: Send>() {}
assert_send::<Handle>();
assert!(
    cfg!(not(target_feature = "atomics")),
    "this instrument reported a fact about the target, not about the type"
);
```

**Rejects.** A `cfg`-conditional instrument is green on both targets the gate
touches and means opposite things on them: the assertion only *runs* on the host,
where the type is `!Send`, while `cargo xtask wasm` merely *builds* the target
where it is `Send`, and no step compares the two. So the workspace holds a `!Send`
witness that is `Send` on the one target the two-flavour design exists for, and
learns it on the day Workers enables threads — after the bound it was cited for is
frozen.

**Evidence.** `crates/happenstance-cloudflare/src/js.rs:25 (unsafe impl Send for JsValue)` ·
`crates/happenstance-cloudflare/src/js.rs:38 (anything. What it costs is stated rather than hidden — see the crate)` ·
`crates/happenstance-cloudflare/src/lib.rs:60 (can only ever *inherit*)` ·
[ES-6](../../spec/SPECIFICATION.md) ·
[ADR-0009](../../.kb/decisions/0009-error-send-sync.md)
