# 62 — Doctests and test harnesses

> **Load when:** writing a `# Examples` block · an example that uses `?` or
> `.await` · `error[E0277]` on a `?` in an example · a `compile_fail` fence ·
> showing usage that must not compile ·
> reading a green `#[tokio::test]` as evidence about `Send`
> **See also:** 60 (what a test must prove) · 61 (compile-time assertions) ·
> 70 (rustdoc obligations) · 81 (checks that cannot be types)

---

## RS-62-1. Pair every `compile_fail` fence with a compiling one, and do not trust its error code.

**Why.** rustdoc 1.97.1 compares the error code, finds no match, and reports the
fence as **passing anyway** — a fence tagged `compile_fail,E0080` is green when
the code is `E0425`, so the annotation is advisory and cannot be the check. What
is left is a compiling fence over the same items: it is the one thing in the file
that goes red when the subject is renamed or moved behind a feature.

**Do** — the control first, then the refusal it makes legible.

```rust
use happenstance_core::EventType;

/// A *free* `const` — the call site VT-32's rule names, for the reason it gives
/// there. This compiling fence is what says the fence below fails because the
/// value is rejected, rather than because `EventType` was renamed.
const COURSE_DEFINED: EventType = EventType::from_static("CourseDefined");

fn main() {
    assert_eq!(COURSE_DEFINED.as_str(), "CourseDefined");
}
```

```rust,compile_fail
use happenstance_core::EventType;

const EMPTY: EventType = EventType::from_static("");
# fn main() { let _ = EMPTY; }
```

That diagnostic *is* `error[E0080]: evaluation panicked: an event type must not be
empty`, and the fence declines to claim the code on purpose — the same choice
`event.rs` records at the identical snippet, for the reason the next fence
demonstrates.

**Not** — and this fence is the demonstration: it claims `E0080` and fails with
`error[E0425]: cannot find value`. The doctest step reports it as **passing**.

```rust,compile_fail,E0080
use happenstance_core::EventType;

const EMPTY: EventType = EventType::from_static(no_such_constant);
# fn main() { let _ = EMPTY; }
```

**Rejects.** A lone `compile_fail` fence in a published doc comment, with no
compiling neighbour over the same items — and, beside it, the review comment
"tag the code" and the commit that obeys it. Neither instrument now reports
anything: nothing goes red when `from_static` moves behind a feature, and the
annotation is the one artefact in the file that looks like it pins the
diagnostic. The reader is an adapter author copying the pattern into their own
crate's documentation, and they find out when their own invalid constant reaches
run time rather than `cargo check`.

**Evidence.** `crates/happenstance-core/src/event.rs:103 (silently ignores an error-code annotation)` ·
`xtask/src/lint_constitution.rs:524 (silently ignores an unmatched error code)` ·
[SPECIFICATION PS-36](../../spec/SPECIFICATION.md) ·
[SPECIFICATION WF-12](../../spec/SPECIFICATION.md) ·
[SPECIFICATION VT-32](../../spec/SPECIFICATION.md) ·
[ADR-0015 §trybuild](../../.kb/decisions/0015-validated-identifiers-and-store-limits.md)

---

## RS-62-2. Close a `?`-using example with `# Ok::<(), E>(())`; give an `.await` one a hidden runtime.

**Why.** rustdoc wraps a fence that contains no `fn main` in one, so a bare `?`
is `error[E0277]: the ? operator can only be used in a function that returns
Result`. A trailing `Ok::<(), E>(())` is what makes rustdoc generate a
`Result`-returning wrapper instead, so it must be the final expression. An
`.await` needs an executor, which means writing the `main` yourself — hidden, so
the example still reads as the three lines that matter.

**Do**

```rust
use happenstance_core::EventType;

let event_type = EventType::new("CourseDefined")?;
assert_eq!(event_type.as_str(), "CourseDefined");
# Ok::<(), happenstance_core::InvalidEventType>(())
```

```rust
# use happenstance_core::{Event, EventStore, MemoryEventStore};
# #[tokio::main(flavor = "current_thread")]
# async fn main() -> Result<(), Box<dyn std::error::Error>> {
let store = MemoryEventStore::new();
let position = store
    .append(&[Event::new("CourseDefined", &b"{}"[..])?], None)
    .await?;
assert_eq!(store.head().await?, Some(position));
# Ok(())
# }
```

**Not** — the same example with the closer dropped. `error[E0277]`.

```rust,compile_fail,E0277
use happenstance_core::EventType;

let event_type = EventType::new("CourseDefined")?;
assert_eq!(event_type.as_str(), "CourseDefined");
```

**Rejects.** The repository README's own Quick start, which is the first code a
visitor reads: it used `?` and `.await` at the top level of a `rust` block, so it
had never compiled and could not have, and nothing was compiling it to say so.
The reader copies it, hits `E0277` on line one of their first attempt, and
concludes the library's documented entry point does not work.

**Evidence.** `xtask/src/lib.rs:5 (it had never compiled and could not have)` ·
`crates/happenstance-testkit/src/lib.rs:436 (macro_rules! ignore)` ·
`xtask/src/lib.rs:16 (inside the package and stays correct after publication)`

---

## RS-62-3. A green `#[tokio::test]` says nothing about `Send`.

**Why.** The attribute expands to `Runtime::block_on`, which drives the future on
the calling thread and requires no `Send` — on a *multi-threaded* runtime as well
as on the `current_thread` one `#[tokio::test]` in fact defaults to, unlike
`#[tokio::main]`. `tokio::spawn` is the only thing that requires `Send`, because
it moves the future into the executor. So an `Rc`-holding store passes the
default attribute unchanged.

**Do**

```rust
use std::rc::Rc;
use tokio::runtime::Runtime;

fn main() {
    // Strictly stronger than a plain `#[tokio::test]`: `Runtime::new()` is
    // multi-threaded, and the attribute's own default is `current_thread`.
    let runtime = Runtime::new().unwrap();
    let held = runtime.block_on(async {
        // `Rc` live across a suspension point: the future is `!Send`, and
        // `block_on` drives it regardless.
        let store = Rc::new(vec![1u8, 2]);
        tokio::task::yield_now().await;
        store.len()
    });
    assert_eq!(held, 2);
}
```

**Not** — the one construct that does require it. The diagnostic carries **no
error code**; its first line is `error: future cannot be sent between threads
safely`.

```rust,compile_fail
use std::rc::Rc;

# fn main() {
let _task = tokio::spawn(async {
    let store = Rc::new(vec![1u8, 2]);
    tokio::task::yield_now().await;
    store.len()
});
# }
```

**Rejects.** The runbook's plan for phase 1, which assumed the shipped
`#[tokio::test]` emitter could not drive a `!Send` store and that a bespoke
single-threaded harness therefore had to land before the `RefCell` reference
store was reachable at all. Taken further, the same belief produces a `Fixture`
trait carrying `Self: Send` — which excludes precisely the adapters the
two-flavour port exists for, and is discovered only when the Workers adapter
cannot implement it.

**Evidence.** `crates/happenstance-testkit/tests/local_conformance.rs:41 (Every rule passes against this store)` ·
`crates/happenstance-testkit/tests/local_conformance.rs:476 (local_tokio_default)` ·
[SPECIFICATION CF-20](../../spec/SPECIFICATION.md) ·
[ADR-0010](../../.kb/decisions/0010-the-suite-must-prove-itself.md)

---

## RS-62-4. Show usage that must not compile with a discarding macro, not an `ignore` fence.

**Why.** An `ignore` fence is never handed to the compiler, so it is
indistinguishable from an example that stopped compiling. A `macro_rules!` arm
matching `$($t:tt)*` and expanding to nothing discards its body *after*
tokenising it, so the example is still parsed and still rendered while nothing in
it is name-resolved, type-checked or run.

**Do**

```rust
# macro_rules! ignore { ($($t:tt)*) => {} }
# ignore! {
use happenstance_testkit::fixtures::MemoryFixture;

happenstance_testkit::event_store_conformance!(MemoryFixture::new());
# }
# fn main() {}
```

**Not** — the same example as an `ignore` fence. Nothing checks it, including
that it is still Rust.

<!-- ignore: this fence is the counterexample; an `ignore` fence is exactly what the rule forbids -->
```rust,ignore
happenstance_testkit::event_store_conformance!(factory = MemoryStore::new(),,);
```

**Rejects.** That fence, verbatim. `factory =` was renamed to `fixture =` at
phase 3 with no deprecated arm, and it took a store expression where the current
keyword takes a `Fixture` — so an `ignore` fence would still be printing the old
spelling, unparseable and wrong in two ways, as the first thing every adapter
author copies out of the macro's documentation.

**Evidence.** `crates/happenstance-testkit/src/lib.rs:436 (macro_rules! ignore)` ·
`crates/happenstance-testkit/src/lib.rs:458 (Migrating from)` ·
`xtask/src/lint_constitution.rs:640 (ignore: <reason>)` ·
[SPECIFICATION CF-15](../../spec/SPECIFICATION.md)

---

## RS-62-5. Compile an out-of-package file's examples from a `publish = false` crate.

**Why.** `include_str!` is resolved by the compiler against the file tree,
relative to the file that writes it — not against the crate root, and not at run
time. A path that leaves the package is a path the packaged `.crate` does not
contain, so the attribute compiles here and fails `cargo test` for everyone who
depends on the published crate.

**Do**

```rust
// Anchored on the package root, which is what makes "inside this package" a
// checkable claim rather than a hope about relative depth.
const MANIFEST: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/Cargo.toml"));

fn main() {
    assert!(MANIFEST.contains("[package]"));
}
```

**Not** — a path that leaves the package, which is what a consumer's unpacked
`.crate` looks like. The diagnostic carries no error code; its first line is
`error: couldn't read <path>: <the platform's own not-found message>` — `os error
2` on Linux, `os error 3` on Windows, which is why only the prefix is quotable.

```rust,compile_fail
const README: &str = include_str!("../../../../outside-this-package/README.md");
# fn main() { let _ = README; }
```

**Rejects.** `#![cfg_attr(doctest, doc = include_str!("../../README.md"))]` moved
onto `happenstance` — the obvious home, since the README is about that crate. It
passes every local check, because in a git checkout the path resolves; it fails
`cargo test` for every downstream user of the published crate, and the first
report arrives from a stranger. The same argument is why this constitution's
atoms hang off `xtask` and why each crate README is compiled by its own crate.

**Evidence.** `xtask/src/lib.rs:10 (does not exist inside a packaged)` ·
`xtask/src/constitution.rs:7 (publish = false)` ·
`xtask/src/lib.rs:16 (inside the package and stays correct after publication)`
