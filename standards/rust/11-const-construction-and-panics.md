# 11 — Const construction, and where its panics surface

> **Load when:** writing a `const fn` constructor · a `from_static` with a bad
> literal did not fail the build · E0658 inside a `const fn` · choosing between a
> free `const` and an associated one · a `#[should_panic]` you cannot write
> **See also:** 10 (the newtype being constructed) · 12 (the impls it then owes) ·
> 13 (private fields as the other validating seal) · 62 (what a `compile_fail`
> fence actually proves)

---

## RS-11-1. Route both constructors through one `const fn` validator and no second copy of the rules.

**Why.** The pair is a fallible `new(impl Into<String>) -> Result<Self, E>` and a
panicking `const from_static(&'static str) -> Self`, and they must accept exactly
the same set. A `const fn` cannot call `?`, `.map()` or any iterator adaptor
(RS-11-3), so the tempting repair is a second, simpler check written for the
`const` path — and nothing in the language compares the two.

**Do**

```rust
use happenstance_core::EventType;

// One validator, reached from both doors. The `const` site is checked before
// the program runs; the runtime site returns the refusal as a value.
const COURSE_DEFINED: EventType = EventType::from_static("CourseDefined");
let at_runtime = EventType::new("CourseDefined")?;

assert_eq!(COURSE_DEFINED.as_str(), at_runtime.as_str());
assert!(EventType::new("Order\u{202E}Placed").is_err());
# Ok::<(), happenstance_core::InvalidEventType>(())
```

**Not** — compiles, and the last assertion is what catches it:

```rust
const fn lax(value: &str) -> bool {
    !value.is_empty()
}

fn strict(value: &str) -> bool {
    !value.is_empty() && !value.contains('\u{202E}')
}

const fn from_static(value: &'static str) -> &'static str {
    assert!(lax(value));
    value
}

fn new(value: &str) -> Option<&str> {
    strict(value).then_some(value)
}

const SPOOFED: &str = from_static("Order\u{202E}Placed");
assert!(new(SPOOFED).is_none(), "the const path minted what the runtime path refuses");
```

**Rejects.** A `const` constructor whose byte walk was simplified because the
full check would not compile in a `const` context. Every `const` identifier in
the workspace is then validated against a weaker rule than the one `new`
enforces, so a bidirectional override — an event type that renders as one thing
in a console and matches another in every query — is reachable through a source
literal and unreachable through the API, and no test that exercises one
constructor can see the other's hole.

**Evidence.** `crates/happenstance-core/src/validate.rs:53 (pub(crate) const fn check)` ·
`crates/happenstance-core/src/event.rs:71 (Enforces exactly the rules)` ·
`crates/happenstance-core/src/event.rs:949 (from_static_and_new_agree)` ·
[SPECIFICATION VT-32](../../spec/SPECIFICATION.md)

## RS-11-2. Put a `from_static` you want the compiler to check in a **free** `const`.

**Why.** Const evaluation is lazy, and where the panic lands depends entirely on
the call site: a free `const` is evaluated during `cargo check` and its panic is
`error[E0080]`; an associated `const` that something reads is evaluated at
codegen, so `check` and `clippy` pass and `build` fails; an associated `const`
nothing reads is **never** evaluated; a `let` binding panics at run time.

**Do**

```rust
use happenstance_core::EventType;

// Evaluated by `cargo check`. An invalid literal here is `error[E0080]` before
// anything is built, which is the only one of the four sites that is free.
const COURSE_DEFINED: EventType = EventType::from_static("CourseDefined");
assert_eq!(COURSE_DEFINED.as_str(), "CourseDefined");
```

**Not** — compiles, runs, and passes, with an identifier `EventType::new` refuses
sitting in the program. Nothing catches this one, which is the point:

```rust
use happenstance_core::EventType;

trait Codec {
    const KIND: EventType;
}

struct Broken;

// `""` is refused by both constructors. Nothing reads `Broken::KIND`, so
// nothing evaluates it: `check`, `clippy`, `build` and `test` are all green.
impl Codec for Broken {
    const KIND: EventType = EventType::from_static("");
}

assert_eq!(size_of::<Broken>(), 0);
```

**Rejects.** A fixture declaring `const REOPEN: Capability = Capability::declined("")`
— the empty reason the constructor's `assert!` exists to refuse — in an adapter
whose suite run never touches `reopen`. The associated const is never read, so
the compile-time check the private field was built to buy does not fire, and the
adapter ships declining a capability for no stated reason while its CI log prints
a blank where the account of the trade belongs.

**Evidence.** `crates/happenstance-core/src/event.rs:85 (never read | **never**)` ·
`crates/happenstance-core/src/event.rs:89 (evaluated lazily)` ·
`crates/happenstance-testkit/src/contract.rs:999 (Where it does *not* fire)` ·
`crates/happenstance-testkit/src/contract.rs:1006 (pub const fn declined)`

## RS-11-3. Write a `const fn` body with `match` and `while`, never `?`, `.map()` or an iterator adaptor.

**Why.** On 1.97.1 `Try` and `FromResidual` are not yet const traits and
`Option::map` is not yet a const fn, so both are `error[E0658]` inside a
`const fn` — not a lint, a feature gate. The same gate is why the shared
validator is a byte walk with a `while` loop rather than the `chars().any(..)`
it reads like.

**Do**

```rust
const fn first_byte(value: &str) -> Option<u8> {
    let bytes = value.as_bytes();
    match bytes.len() {
        0 => None,
        _ => Some(bytes[0]),
    }
}

const FIRST: Option<u8> = first_byte("CourseDefined");
assert_eq!(FIRST, Some(b'C'));
```

**Not** — `error[E0658]`: `?` is not allowed on `Option` in a `const fn`,
because `Try` is not yet stable as a const trait.

```rust,compile_fail,E0658
# const fn check(_value: &str) -> Option<()> { Some(()) }
const fn from_static(value: &'static str) -> Option<&'static str> {
    check(value)?;
    Some(value)
}
# let _ = from_static("A");
```

**Rejects.** An author tidying the `const` constructor into `?` or `.map()` has
two exits and both are wrong. Dropping `const` converts every free-`const` call
site in the workspace from a `cargo check` failure into a run-time panic, which
no test notices because the values in those `const`s are all valid; duplicating
the validator is the divergence RS-11-1 forbids. Neither exit fails a test, so
the review that lets it through sees a smaller diff.

**Evidence.** `crates/happenstance-core/src/event.rs:918 (it is the reason the body is a)` ·
`crates/happenstance-core/src/validate.rs:8 (iterator adaptors and)` ·
[const_trait_impl tracking issue #143874](https://github.com/rust-lang/rust/issues/143874) *(checked 2026-08-09, rustc 1.97.1)*
