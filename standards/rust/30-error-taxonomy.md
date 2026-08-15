# 30 — Error taxonomy

> **Load when:** designing an adapter's `Error` · a retry loop must tell
> contention from failure · choosing between a `String` field and a type
> parameter for a wrapped error · `E0425: cannot find value` inside an
> `#[error(…)]` message · an adapter error will not satisfy
> `core::error::Error + 'static` · an operator's log line is missing a field the
> error carries
> **See also:** 12 (manual impls) · 13 (`#[non_exhaustive]` and wildcard arms) ·
> 21 (`Send` on an error is a separate obligation) · 51 (`no_std` and the
> `thiserror/std` feature) · 91 (adapter recipe)

---

## RS-30-1. Keep the retryable outcome in the contract's enum, and the adapter's error a type parameter.

**Why.** `AppendError<E>` is generic over the adapter's error, so a caller
holding *any* store can ask `is_condition_violated()` and get a real answer. Put
the same outcome inside `E` and the question becomes "downcast to which type?",
which generic code cannot ask.

**Do**

```rust
use happenstance_core::{AppendError, ConditionViolated};

// One retry policy, every adapter, no downcasting.
fn should_retry<E>(err: &AppendError<E>) -> bool {
    err.is_condition_violated()
}

let violated: AppendError<std::io::Error> = ConditionViolated::unspecified().into();
assert!(should_retry(&violated));

// The enum is `#[non_exhaustive]`, so naming every variant is still not
// exhaustive — the wildcard arm is compulsory (atom 13).
match violated {
    AppendError::ConditionViolated(_) => {}
    _ => panic!("expected the concurrency signal"),
}
```

**Not** — this compiles, and the assertion at the bottom is the whole problem:

```rust
use happenstance_core::AppendError;

#[derive(Debug, thiserror::Error)]
enum MyStoreError {
    #[error("the append condition was violated")]
    ConditionViolated,
}

let err = AppendError::Store(MyStoreError::ConditionViolated);
assert!(!err.is_condition_violated());
```

**Rejects.** An adapter that reports contention through `AppendError::Store`. It
passes its own unit tests, which match on `MyStoreError`. The failure surfaces in
somebody else's crate: a generic retry loop written against `EventStore` sees
`is_condition_violated() == false`, classifies routine contention as a fault, and
either aborts the command or — in a sync runner — parks an event that would have
succeeded on the next attempt.

**Evidence.** `crates/happenstance-core/src/error.rs:188 (lifted out of the adapter's error type)` · `crates/happenstance-core/src/error.rs:253 (is_condition_violated)` · `crates/happenstance-core/src/error.rs:261 (pub fn map_store<F, T>)` · `crates/happenstance-sqlite/src/event_store.rs:147 (Append-condition violations are)` ·
[SPECIFICATION ES-25](../../spec/SPECIFICATION.md) ·
[ADR-0009](../../.kb/decisions/0009-error-send-sync.md)

---

## RS-30-2. Carry a foreign error as a type parameter with `#[source]`, never as a `String`.

**Why.** `#[source]` is what populates `Error::source()`, and a `dyn Error`
source can be downcast back to its concrete type. `format!`-ing the error into a
`String` field terminates the chain at that variant; the information is still
printable and no longer *actionable*.

**Do**

```rust
use core::error::Error;

// The adapter does not know whether the transport is `reqwest`, a `JsValue`
// stringified by `wasm-bindgen`, or a test double — so it does not decide.
#[derive(Debug, thiserror::Error)]
enum StoreError<E> {
    #[error("the SQL-over-HTTP round trip did not complete")]
    Transport(#[source] E),
}

#[derive(Debug, thiserror::Error)]
#[error("connection reset")]
struct Reset;

let err = StoreError::Transport(Reset);
assert!(matches!(err.source(), Some(s) if s.downcast_ref::<Reset>().is_some()));
```

**Not** — also compiles; the source chain is simply gone:

```rust
# use core::error::Error;
# #[derive(Debug, thiserror::Error)]
# #[error("connection reset")]
# struct Reset;
#[derive(Debug, thiserror::Error)]
enum StoreError {
    #[error("the SQL-over-HTTP round trip did not complete: {0}")]
    Transport(String),
}

let err = StoreError::Transport(Reset.to_string());
assert!(err.source().is_none());
```

**Rejects.** A transport-generic adapter whose author flattens the wrapped error
"because the message is all anyone reads". Six months later a caller needs to
distinguish a timeout from a rejected TLS handshake in order to decide whether a
retry is safe without an idempotency key. The type that knew is gone, the only
remedy is substring matching on a message the transport is free to reword, and
the fix is a breaking change to the adapter's public error enum.

**Evidence.** `crates/happenstance-neon/src/error.rs:60 (survives intact rather than being flattened into a string)` · `crates/happenstance-neon/src/error.rs:72 (Transport(#[source] E))` · `crates/happenstance-neon/src/error.rs:47 (is_serialization_failure)` — the same crate derives `Deserialize` *and*
`thiserror::Error` on `NeonSqlError`, so a parsed wire body is the error type ·
[thiserror 2.0.19](https://docs.rs/thiserror/2.0.19/thiserror/)
*(checked 2026-08-09, rustc 1.97.1)*

---

## RS-30-3. Give an impl that cannot fail an uninhabited error enum.

**Why.** An enum with no variants has no values, so `match never {}` is
exhaustive with zero arms and the compiler *knows* the `Err` arm is dead. A
one-variant placeholder is inhabited, and every caller must invent a value for a
case that never happens.

**Do**

```rust
#[derive(Debug, thiserror::Error)]
#[error("unreachable: the in-memory event store cannot fail")]
enum MemoryStoreError {}

fn value_of(result: Result<u64, MemoryStoreError>) -> u64 {
    match result {
        Ok(position) => position,
        // Zero arms, and the match is complete.
        Err(never) => match never {},
    }
}

assert_eq!(value_of(Ok(7)), 7);
```

**Not** — compiles, and pushes an unreachable arm onto every caller forever:

```rust
#[derive(Debug, thiserror::Error)]
enum MemoryStoreError {
    #[error("unreachable")]
    Never,
}

fn value_of(result: Result<u64, MemoryStoreError>) -> u64 {
    match result {
        Ok(position) => position,
        // Nothing constructs this. The type system cannot say so, so the
        // caller invents a value — usually `unreachable!()`, i.e. a panic.
        Err(MemoryStoreError::Never) => 0,
    }
}

assert_eq!(value_of(Ok(7)), 7);
```

**Rejects.** A reference store whose error is a never-constructed placeholder
variant. It reads as harmless, and it silently weakens the contract: the store
that exists to prove the port does not *require* a fallible read path now claims
it does, every conformance fixture and every doctest written against it grows a
dead `Err` arm, and the first one written as `unreachable!()` turns a proof into
a latent panic.

**Evidence.** `crates/happenstance-core/src/memory.rs:286 (An uninhabited error is worth having)` · `crates/happenstance-core/src/memory.rs:291 (pub enum MemoryStoreError {})` · `crates/happenstance-core/src/error.rs:84 (match never {})` — atom 12 owns the `From<Infallible>` spelling of the same mechanism

---

## RS-30-4. Hand-write `Display` when the message has two shapes, or a carried field will not be rendered.

**Why.** `#[error("…")]` is one format string per variant. A field it does not
name is carried and never shown, and neither `thiserror` nor `rustc` warns —
there is no unused-field lint for a `pub` field. A manual `impl` must then also
carry `impl core::error::Error` by hand; write `core::`, not `std::`, or the
contract crate stops building `--no-default-features`.

**Do**

```rust
use happenstance_core::{ConditionViolated, SequencePosition};

let located = ConditionViolated::at(SequencePosition::FIRST);
assert!(located.to_string().contains("at position 1"));

// The other shape: no position, and no dangling "at position" fragment.
let unspecified = ConditionViolated::unspecified();
assert!(!unspecified.to_string().contains("at position"));
```

**Not** — the derive compiles, the field is populated, and it never reaches a
log:

```rust
#[derive(Debug, thiserror::Error)]
#[error("append condition violated: the store already contains a matching event")]
struct Violated {
    conflicting_position: Option<u64>,
}

let err = Violated { conflicting_position: Some(41) };
assert!(!err.to_string().contains("41"));
```

**Rejects.** Exactly the state this repository shipped and then fixed: adapters
populated `conflicting_position` and the derived message dropped it, so an
operator reading production logs got "the store already contains a matching
event" with no way to find which event, on the one error class that is expected
to occur routinely under contention. Nothing failed; the field was simply
invisible until somebody diffed the struct against the message.

**Evidence.** `crates/happenstance-core/src/error.rs:171 (Hand-written rather than a)` · `crates/happenstance-core/src/error.rs:184 (impl core::error::Error for ConditionViolated)` ·
[core::error stabilised in 1.81](https://blog.rust-lang.org/2024/09/05/Rust-1.81.0/)
*(checked 2026-08-09, rustc 1.97.1)*

---

## RS-30-5. Bind a limit constant into the message as an explicit named format argument.

**Why.** `thiserror` only special-cases names that match a field; everything else
falls through to `format!`'s implicit capture and is resolved from the
surrounding scope. So a bare `{max}` with no such field is `E0425: cannot find
value`, and a path inside braces — `{crate::MAX_TAG_LEN}` — is not a
`thiserror` limitation but a hard format-string syntax error. A trailing
`max = PATH` argument is what binds the name; a positional `{}` with the path
after the string also compiles, but then the message no longer says what it is
printing.

**Do**

```rust
use happenstance_core::{MAX_TAG_LEN, Tag};

// The message is generated from the same constant the check uses, so the two
// cannot drift.
let oversized = "t".repeat(MAX_TAG_LEN + 1);
let message = match Tag::new(oversized) {
    Ok(_) => String::new(),
    Err(err) => err.to_string(),
};
assert!(message.contains(&MAX_TAG_LEN.to_string()));
```

**Not** — `error[E0425]`, and rustc's only suggestion is `use std::cmp::max`,
a false lead — the name it cannot find is a format argument, not a missing
import:

```rust,compile_fail,E0425
const MAX_TAG_LEN: usize = 255;

#[derive(Debug, thiserror::Error)]
enum InvalidTag {
    // No field is named `max`, so this is implicit capture from a scope that
    // has no `max` in it.
    #[error("a tag must be at most {max} bytes, got {len}")]
    TooLong { len: usize },
}
```

**Rejects.** A validation error whose message hard-codes the limit — `"a tag must
be at most 255 bytes"` — while the check reads the constant. Raising
`MAX_TAG_LEN` is a one-line edit that passes the whole gate, because no test
asserts on the sentence; the API now accepts 512-byte tags and tells every caller
who exceeds *that* limit that the maximum is 255.

**Evidence.** `crates/happenstance-core/src/error.rs:24 (max = crate::MAX_TAG_LEN)` · `crates/happenstance-core/src/tag.rs:16 (pub const MAX_TAG_LEN)` · `crates/happenstance-core/src/error.rs:58 (max = crate::MAX_EVENT_TYPE_LEN)`

---

## RS-30-6. Make a variant a unit when the type it would wrap cannot be `'static`.

**Why.** Both ports declare `type Error: core::error::Error + 'static`.
`PoisonError<MutexGuard<'_, Connection>>` borrows the connection, so wrapping it
forces a lifetime parameter onto the error enum — and an associated type has no
lifetime to bind it to, which is `error[E0637]`. (`#[from]` does not even get
that far: `thiserror` refuses a non-`'static` source itself, because
`Error::source` returns `dyn Error + 'static`.) `'static` is the binding
constraint here; `Send` is *not* — it is a separate opt-in marker (ADR-0009,
atom 21).

**Do** — `rusqlite` is not available to a doctest, so `Mutex<u32>` stands in for
`Mutex<Connection>`; the real declaration is the first **Evidence** citation.

```rust
use std::sync::{Mutex, MutexGuard};

# trait Port { type Error: core::error::Error + 'static; }
#[derive(Debug, thiserror::Error)]
enum SqliteEventStoreError {
    // The guard is not carried. The remedy for a poisoned mutex does not depend
    // on which guard was poisoned, so nothing actionable is lost.
    #[error("the SQLite connection mutex was poisoned by a panicking thread")]
    ConnectionPoisoned,
}

fn lock(connection: &Mutex<u32>) -> Result<MutexGuard<'_, u32>, SqliteEventStoreError> {
    connection
        .lock()
        .map_err(|_| SqliteEventStoreError::ConnectionPoisoned)
}

struct SqliteEventStore;
impl Port for SqliteEventStore {
    type Error = SqliteEventStoreError;
}
```

**Not** — `error[E0637]: '_ cannot be used here`. There is no lifetime in scope
at an associated type, and no honest one to invent:

```rust,compile_fail,E0637
use std::sync::{MutexGuard, PoisonError};

# trait Port { type Error: core::error::Error + 'static; }
#[derive(Debug, thiserror::Error)]
enum SqliteEventStoreError<'a> {
    #[error("the SQLite connection mutex was poisoned")]
    ConnectionPoisoned(PoisonError<MutexGuard<'a, u32>>),
}

struct SqliteEventStore;
impl Port for SqliteEventStore {
    type Error = SqliteEventStoreError<'_>;
}
```

**Rejects.** An adapter author who wraps `PoisonError<_>` because `?` is
convenient, then adds `<'a>` to the error enum when the compiler asks for a
lifetime, then threads that parameter through every method signature in the
adapter. The wall arrives last, at `type Error = …`, where there is nothing to
write — by then the error enum, every `?` site and every test have been rewritten
around a parameter that has to come back out.

**Evidence.** `crates/happenstance-sqlite/src/event_store.rs:160 (Carried as a unit variant rather than wrapping)` ·
`crates/happenstance-sqlite/src/event_store.rs:163 (it is neither)` ·
`crates/happenstance-core/src/store.rs:101 (type Error: core::error::Error + 'static)` · `crates/happenstance-core/src/projection.rs:421 (type Error)` ·
[ADR-0009](../../.kb/decisions/0009-error-send-sync.md)
