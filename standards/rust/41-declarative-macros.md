# 41 — Declarative macros

> **Load when:** writing a `macro_rules!` other crates will invoke · "cannot find
> macro … in this scope" from an adapter · "macro expansion ignores `!`" ·
> E0423 / E0308 from an expansion · a macro arm matched and the wrong one
> expanded · the expansion must name a type only the caller knows
> **See also:** 40 (an exported macro is semver surface) · 62 (doctests and
> harnesses) · 91 (adapter authoring) · 81 (checks that cannot be types)

---

## RS-41-1. `$crate::`-qualify every path your expansion emits, and require callers to qualify anything they hand you.

**Why.** `macro_rules!` hygiene covers local variables and labels, not item or
macro paths: those are substituted verbatim and resolved where the expansion
lands, which is the *caller's* crate. `$crate` is the single fragment that
expands to the defining crate, so it is the only way an expansion can name its
own items.

**Do**

```rust
# fn main() {
let names = happenstance_testkit::for_each_event_store_rule!(
    happenstance_testkit::__emit_rule_names
);
assert!(names.contains(&"append_is_atomic"));
# }
```

**Not** — the same emitter named bare, as it is written inside the testkit. There
is no error code; the first line is
``error: cannot find macro `__emit_rule_names` in this scope``:

```rust,compile_fail
# fn main() {
let names = happenstance_testkit::for_each_event_store_rule!(__emit_rule_names);
# let _ = names;
# }
```

**Rejects.** An emitter path baked into the macro as a bare name. Every harness
in this workspace still compiles, because the testkit's own `tests/` are in the
defining crate and resolve it — the failure only appears in the first adapter
crate outside the workspace, months later, and reads as though the adapter did
something wrong.

**Evidence.** `crates/happenstance-testkit/src/lib.rs:666 (for_each_event_store_rule!($emit))` ·
`crates/happenstance-testkit/src/registry.rs:82 (bare name resolves in *your* scope)` ·
[SPECIFICATION CF-23](../../spec/SPECIFICATION.md) *(why the emitter is a
parameter at all)*

## RS-41-2. Capture a callback macro as `$($cb:tt)+`, never as `$cb:path`.

**Why.** A `path` fragment is a parsed AST node, and a parsed path cannot sit in
callee position of a macro call inside an *expression* — rustc emits
``macro expansion ignores `!` and any tokens following`` and then
`error[E0423]`. Raw token trees are re-emitted unparsed, so `$($cb)+ ! { … }`
works in expression and item position alike.

**Do**

```rust
macro_rules! names { ($($n:ident),* $(,)?) => { [ $( stringify!($n) ),* ] }; }
macro_rules! for_each { ($($cb:tt)+) => { $($cb)+ ! { alpha, beta } }; }

# fn main() {
let list = for_each!(names);
assert_eq!(list, ["alpha", "beta"]);
# }
```

**Not**

```rust,compile_fail,E0423
macro_rules! names { ($($n:ident),* $(,)?) => { [ $( stringify!($n) ),* ] }; }
macro_rules! for_each { ($cb:path) => { $cb! { alpha, beta } }; }

# fn main() {
let list = for_each!(names);
# let _ = list;
# }
```

**Rejects.** A registry macro that is fine everywhere it is first used — every
harness invokes it in item position — and then refuses the one call the meta-test
needs, `let registered = for_each_event_store_rule!(__emit_rule_names);`. The
author reads the diagnostic as a problem with the *callback*, rewrites the
emitter, and arrives at "a macro cannot return a value", which is false.

**Evidence.** `crates/happenstance-testkit/src/registry.rs:95 (captured as raw token trees)` ·
`crates/happenstance-testkit/src/registry.rs:101 (meta-test below needs)` ·
`crates/happenstance-testkit/src/registry.rs:421 (for_each_event_store_rule!(crate::__emit_rule_names))`

## RS-41-3. Order arms most-literal-first; the first arm that matches is the one that expands.

**Why.** The matcher tries arms top to bottom and takes the first that consumes
the *whole* invocation; an arm whose fragment parses but leaves tokens over is
abandoned and the next is tried, which is what rescues
`(fixture = $f:expr, threads = $n:literal)` from a bare `($f:expr)` above it.
Ordering therefore decides which of two arms that *both* consume everything
wins, and `$x:expr` consumes nearly any token sequence — `fixture = 7` included,
because it is an assignment expression. A fragment that fails to parse *after*
consuming tokens is fatal rather than a miss: `($e:expr)` above `($i:ident +)`
makes `m!(x +)` `error: expected expression, found end of macro arguments`, and
the second arm is never reached.

**Do**

```rust
macro_rules! conformance {
    (mod_name = $m:ident, fixture = $f:expr) => { "explicit" };
    ($f:expr) => { "defaulted" };
}

# fn main() {
assert_eq!(conformance!(mod_name = alpha, fixture = 7), "explicit");
assert_eq!(conformance!(7), "defaulted");
# }
```

**Not** — this compiles, and the assertion is what catches it:

```rust
macro_rules! shadowed {
    ($f:expr) => { "defaulted" };
    ($a:ident + $b:ident) => { "sum" };
}

# fn main() {
// The second arm is unreachable: `a + b` is a perfectly good expression.
assert_eq!(shadowed!(a + b), "defaulted");
# }
```

**Rejects.** A single-keyword convenience arm added below the general one —
`(fixture = $f:expr)` under a bare `($f:expr)`. It is dead on arrival, because
`fixture = MemoryFixture::new()` is a complete assignment expression that the
general arm consumes whole; the adapter author gets either silence, where the
general arm's expansion happens to type-check anyway, or an
``error[E0425]: cannot find value `fixture` in this scope`` raised inside a
macro expansion they did not write, naming a binding they never declared. Nothing
anywhere points at the arm order that caused it.

**Evidence.** `crates/happenstance-testkit/src/lib.rs:616 (macro_rules! event_store_conformance)`
*(the comment there attributes the order to never having to back out of
`fixture = $fixture:expr`; measured on 1.97.1 that arm backs out either way —
the order is load-bearing for the reason above, not that one)* ·
`crates/happenstance-testkit/src/lib.rs:678 (mod_name = dcb_conformance,)` ·
[Reference — macros by example, transcription](https://doc.rust-lang.org/reference/macros-by-example.html)
*(checked 2026-08-09, rustc 1.97.1)*

## RS-41-4. Hoist a caller-supplied expression behind a function returning `impl Trait`.

**Why.** A macro is handed an *expression*, never a type, so any expansion that
writes a concrete return type is guessing — `error[E0308]` for every caller who
guesses differently. `impl Trait` in return position names the bound instead of
the type; and because hygiene does not apply to items, one macro's expansion can
define the function and another's can call it.

**Do**

```rust
trait Fixture { fn label(&self) -> &'static str; }
# struct Sqlite;
# impl Fixture for Sqlite { fn label(&self) -> &'static str { "sqlite" } }

macro_rules! harness {
    ($fixture:expr) => {
        fn __conformance_fixture() -> impl Fixture { $fixture }
    };
}
harness!(Sqlite);
# fn main() { assert_eq!(__conformance_fixture().label(), "sqlite"); }
```

**Not**

```rust,compile_fail,E0308
# struct Memory;
# struct Sqlite;
macro_rules! harness {
    ($fixture:expr) => { fn __conformance_fixture() -> Memory { $fixture } };
}
harness!(Sqlite);
# fn main() { let _ = __conformance_fixture(); }
```

**Rejects.** An expansion that binds the fixture once, as a value the emitters
share, instead of behind a re-evaluated function. Every rule then drives the same
backing store, so `two_fixture_instances_observe_none_of_each_others_appends`
cannot be written at all — and the isolation defect it exists to catch becomes
untestable rather than undetected.

**Evidence.** `crates/happenstance-testkit/src/lib.rs:637 (__conformance_fixture() -> impl $crate::__private::Fixture)` ·
`crates/happenstance-testkit/src/registry.rs:40 (which it could not know)` ·
`crates/happenstance-testkit/src/concurrency.rs:1244 (impl $crate::__private::ConcurrentFixture)`

## RS-41-5. Enforce a caller-side obligation with `#[must_use = "…"]`, and write the consequence into the message.

**Why.** When the emitter is a caller-supplied parameter, nothing in the defining
crate can force it to report anything. `#[must_use]` on the *returned type* makes
dropping it an `unused_must_use` warning at the caller, which `-D warnings` turns
into a build failure; the attribute's string is what the diagnostic's `note:`
line prints, so it is the only place the reason can be stated.

**Do**

```rust
use happenstance_testkit::RuleOutcome;

# fn main() {
let outcome = RuleOutcome::Skipped { capability: "REOPEN", reason: "a volatile store" };
assert_eq!(
    outcome.skip_line("acknowledged_writes_survive_a_reopen").as_deref(),
    Some("SKIP acknowledged_writes_survive_a_reopen: fixture declines `REOPEN` — a volatile store"),
);
# }
```

**Not** — an emitter that drops the outcome. Lints carry no error code; the first
line is ``error: unused `RuleOutcome` that must be used``:

```rust,compile_fail
#![deny(unused_must_use)]
# use happenstance_testkit::RuleOutcome;
fn rule() -> RuleOutcome { RuleOutcome::Ran }

# fn main() {
rule();
# }
```

**Rejects.** A hand-written emitter for a runtime the testkit has never heard of
— an embedded executor, a `LocalSet` harness — that awaits each rule and discards
the result. Every capability-gated rule then passes silently, and the adapter's
CI reports a full green suite in which `acknowledged_writes_survive_a_reopen`
never ran and nothing anywhere says so.

**Evidence.** `crates/happenstance-testkit/src/contract.rs:1192 (a rule's outcome must be reported)` ·
`crates/happenstance-testkit/src/registry.rs:51 (It is not asked politely)` ·
[SPECIFICATION CF-18](../../spec/SPECIFICATION.md) *(the reporting
obligation this attribute mechanises)*
