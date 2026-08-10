---
id: adr-0015-validated-identifiers-and-store-limits
title: "ADR-0015: Validated identifiers, byte equality, and the two kinds of bound"
kind: decision
status: accepted
authority_tier: decision
summary: >-
  How a validated identifier is constructed, that `Tag` equality is byte equality, and
  the difference between a bound the contract sets and a limit a store happens to have.
  Discharges VT-14 through VT-25.
depends_on: []
related: []
source_paths:
  - docs/architecture/SPECIFICATION.md
last_reviewed: 2026-08-09
adr_id: ADR-0015
phase: 4
supersedes: []
superseded_by: null
---

# ADR-0015: Validated identifiers, byte equality, and the two kinds of bound

- **Status:** accepted
- **Date:** 2026-08-08
- **Settles:** VT-14 (both halves, neither of which phase 3 wrote), VT-15 – VT-20
  and VT-25 by transcription and implementation, and the constants VT-21 – VT-24
  name but the crate does not contain
- **Leaves provisional:** VT-14, VT-21 – VT-24, each with a restated falsifier and
  a named owning phase; two of the four have falsifiers **no scheduled phase can
  reach**, and this ADR says which and what that marker therefore means
- **Adds:** VT-32 (const-constructible identifiers) and VT-33 (the standard-library
  trait surface), because both ship here and neither is claimed by any clause
  today; and **CF-40**, the fixture's numeric-limit declaration, placed here at
  sign-off (see decision 8) because without it VT-25 `[FROZEN]` ships implemented
  and checked by nothing
- **Extends:** [ADR-0003](0003-opaque-payloads.md), whose byte-for-byte forwarding
  promise is what makes normalisation unavailable rather than merely unwanted

## The one question, and the two questions it is not

The queue's question ([`RUNBOOK.md:288`](../RUNBOOK.md)) is *how is a validated
identifier constructed, and is `Tag` equality byte equality?* Phase 4's body adds
a third face: *and what capacity must every store accept?* They are one question
asked at three moments in the life of the same three values — `EventType`, `Tag`
and `Tags`. Construction decides what may exist; equality decides what two of them
mean when they meet; a store limit decides which of the survivors a particular
engine will actually hold. Answering any one without the other two produces a
value that is valid, comparable and unstorable, which is the state the crate is in
today.

Two questions in the neighbourhood are **not** answered here, and both are named
rather than absorbed:

- **`ProjectionId`'s validation** belongs to the port phase 6 freezes, for reasons
  set out in decision 10. Phase 4 fixes only its documentation.
- **The four-way error union** E2E-51 describes in prose — the two validation
  errors plus `S::Error` plus `AppendError<S::Error>` — cannot be closed in
  `happenstance-core` at all, and is phase 7's. Decision 5 says why, and says
  precisely which half of E2E-51 phase 4 does close.

And one question this ADR surfaced without answering, which was different from
declining it: **where the fixture's numeric-limit declaration lands.** VT-25's
rule cannot be written against a store with no ceiling, so it needs `Fixture` to
state its actual limits, and no CF clause let a fixture declare a number rather
than a boolean. **Settled at sign-off: it lands here, as CF-40.** Decision 8
carries the clause and the reason the competing home lost. The draft's refusal to
choose is kept there rather than deleted, because the argument it makes — that
parking a blocker on an ADR which has not accepted it is indistinguishable, in a
work queue, from having placed it — is the reason this was worth a human's
attention rather than a coin toss.

## Context

### What was already decided, and what phase 3 left undone

Seven of the twelve clauses in scope are `[FROZEN]`: VT-15, VT-16, VT-17, VT-18,
VT-19, VT-20 and VT-25. For those, phase 4's job is transcription and
implementation, and the temptation to mistake execution for design is the main
hazard in this ADR. In particular, the RUNBOOK asks this phase to "take a position
on `Cf` format characters" and "on normalisation" ([`RUNBOOK.md:2940-2949`](../RUNBOOK.md)).
**Both positions are already taken.** VT-14 forbids blanket `Cf` rejection in
terms; VT-15 is `[FROZEN]` on byte equality and on the contract normalising
nothing. Neither is reopened here. What is owed is transcription into `Tag`'s
documentation, which is a real obligation and the cheapest one in the phase.

Three things in scope are genuinely undone rather than merely unimplemented.

**VT-14's bidirectional half does not exist.** `grep -rn "202A\|202E\|2066\|bidi"
crates/happenstance-core/src/` matches nothing. `char::is_control` is the only
character test either constructor runs (`event.rs:46`, `tag.rs:56`), and the
clause's `Rule:` line read "extended" for a whole stage — a word a reader takes as
done. `cargo xtask spec-trace` structurally cannot notice, because it reconciles
*suite rule* names and VT-14 cites unit tests.

**D9: four docstrings describe a validator the crate does not have.**
`char::is_control` is Unicode general category `Cc`, which includes the C1 range
U+0080–U+009F. `tag.rs:47`, `event.rs:37`, `error.rs:29` and `error.rs:53` all say
"ASCII control characters". The behaviour is right and the prose is wrong, which
is the more dangerous direction: `Latin1IdentifierStore`'s provenance in the
mutant registry (`mutation_coverage.rs:1102`) cites those four sites as what
invites an ASCII-only column. The fix is two words at four sites and it lands with
the bidirectional work on purpose — landing it alone would make three provenance
paragraphs false in the same change while the MUST they explain stayed unwritten.

**Four constants that four conformance rules assert against do not exist.**
`suite.rs:132-142` hardcodes 65,536 / 64 / 128 / 128 as private suite-local
consts, with a comment saying exactly why (`suite.rs:113-130`) and recording that
the RUNBOOK's own prose says "1 MiB" where VT-21 says 65,536. The clause wins;
`suite.rs:128-130` already says so.

And one `[FROZEN]` clause is simply unimplemented: VT-25 requires
`AppendError::ExceedsStoreLimit`, and `error.rs:150-166` has exactly three
variants. A frozen marker binds the design; it does not assert the code exists.

### Why a const-constructible identifier is the load-bearing question

`EventType` is `Box<str>` (`event.rs:29`). Every construction allocates and
re-validates. A typed layer that interns one `EventType` per `DomainEvent` — which
is the entire point of interning — has nowhere to put it: `const` cannot call
`EventType::new`, because `new` allocates and heap allocation is not available at
const-evaluation time, and `LazyLock` is `std` while this crate is `no_std` +
`alloc`.

And even if it had somewhere to put it, it could not use it. `Event::new`'s bound
is `impl TryInto<EventType, Error = InvalidEventType>` (`event.rs:197-200`), an
*equality* constraint on the associated error type. For a Rust newcomer this is
the whole trap in one line, so it is worth spelling out. The standard library
carries a blanket `impl<T, U: From<T>> TryFrom<T> for U`, so every type converts
into itself infallibly, with `Error = Infallible` — the uninhabited type, the one
with no values at all. An equality constraint written `Error = InvalidEventType`
therefore admits `&str` and `String` and excludes the identity conversion, which
is the *only* conversion that cannot fail. Passing an already-built `EventType`
produces `error[E0271]` at `event.rs:198`. This is D4, VT-18 rejects the current
signature by name, and the crate already applied the fix two files over:
`QueryItem::new` writes `T: TryInto<EventType>, InvalidQuery: From<T::Error>`
(`query.rs:57-61`) with `impl From<Infallible> for InvalidQuery` (`error.rs:77-84`).

So the const constructor and D4's widened bound are one change with two halves,
and either alone is inert: a `const EventType` you cannot pass to `Event::new` is
decoration, and a widened bound with nothing infallible to pass through it is a
signature nobody exercises.

### The objection that would have sunk it, and what happened to it

Experiment E6 (dossier, *What the compiler said*) recorded a divergence: `chars()`
is not a `const fn`, so const validation must walk `as_bytes()`, and E6 concluded a
byte walk "cannot see U+0080–U+009F — exactly the range `char::is_control` rejects
today". If true, that is fatal in a quiet way. `from_static` and `new` would
enforce two different rules, so `EventType::from_static("a\u{0085}b")` would
compile while `EventType::new("a\u{0085}b")` returned `Err`, and the crate's one
real invariant — *an `EventType` value is always validated* — would become *always
validated, by one of two validators, and which one is invisible at the use site*.
The dossier offered the escape of narrowing the rule to ASCII C0, which VT-14's
`Rejects:` line forbids in terms: *"a validator that bans only ASCII C0, which is
what four doc comments in the crate currently claim happens."*

**This ADR compiled the question rather than taking the escape.** The finding is
decision 1 and it goes the other way.

**A note on experiment numbers, because they have already collided once.** E1 –
E9 are the dossier's (`docs/evaluation/phase-4-reconciliation.md`, *What the
compiler said*). **E10 is ADR-0011's** — "do the escaping cases compile under
`&Query` today?" — recorded in that ADR rather than in the dossier. This ADR's
compiles are therefore **E11**, and they are recorded here, in decisions 1, 3 and
4, because there is nowhere else at this point in the pass to put them. Anyone
adding a twelfth should check both ADRs before reaching for a number.

## Decision

### 1. One validator, walked over bytes, shared by both constructors — and it is not weaker

`EventType::new` and `Tag::new` delegate character validation to a single
`const fn` that walks `&[u8]`. `from_static` calls the same function. There is
one rule, not two.

E6's divergence does not survive compilation, and the reason is arithmetic rather
than clever. UTF-8 is self-synchronising: a byte walk is not restricted to seeing
one byte at a time, only to seeing bytes. The C1 controls U+0080–U+009F encode as
the two-byte sequences `C2 80` … `C2 9F`, so a walk that looks at the lead byte
*and its successor* sees every one of them. The seven explicit bidirectional
formatting controls are the same shape one byte longer: U+202A–U+202E is
`E2 80 AA`…`E2 80 AE` and U+2066–U+2069 is `E2 81 A6`…`E2 81 A9`.

```rust
const fn validate(bytes: &[u8]) -> u8 {
    // … empty and length checks …
    let mut i = 0;
    while i < bytes.len() {
        let b = bytes[i];
        if b < 0x20 || b == 0x7F { return CONTROL; }               // C0 and DEL
        if b == 0xC2 && i + 1 < bytes.len() {
            let n = bytes[i + 1];
            if n >= 0x80 && n <= 0x9F { return CONTROL; }          // C1
        }
        if b == 0xE2 && i + 2 < bytes.len() {
            let (m, n) = (bytes[i + 1], bytes[i + 2]);
            if m == 0x80 && n >= 0xAA && n <= 0xAE { return BIDI; }
            if m == 0x81 && n >= 0xA6 && n <= 0xA9 { return BIDI; }
        }
        i += 1;
    }
    OK
}
```

**Compiled, and executed, on rustc 1.97.1** (experiment E11, this pass; the
reproduction is a standalone crate, so what is proved is the shape, not that
`happenstance-core` will accept it unchanged). An exhaustive test walks all
1,112,064 Unicode scalar values, encodes each, and compares the byte walk's verdict
against `char::is_control` plus a `matches!` over the seven-codepoint closed list —
once with the codepoint alone and once with it embedded between two ASCII letters,
because a lead byte at index 0 and a lead byte mid-string are different positions
in the walk:

```text
test byte_walk_agrees_with_char_walk_on_every_scalar_value ... ok
test byte_walk_agrees_when_the_offender_is_embedded ... ok
test c1_controls_are_rejected_by_the_byte_walk ... ok
test the_seven_bidi_controls_are_rejected_and_zwj_zwnj_are_not ... ok
```

`checked 1112064 scalar values, 0 disagreements`, both ways round. So:

- **VT-14's `Cc` MUST stands unchanged.** No narrowing to ASCII C0, and the
  `Rejects:` line that forbids narrowing keeps its force.
- **`from_static` is exactly as strong as `new`**, not "strictly weaker and it must
  say so". The clause that ships this (VT-32) states the equivalence as a MUST,
  because it is the property the whole construct rests on and it is invisible at
  every call site.
- **The escape hatch is not taken, and the four docstrings are still fixed** — by
  correcting the prose to match the code, which is decision 2, rather than by
  weakening the code to match the prose.

The bidirectional check is a `matches!` over a closed list with no dependency, and
that is why it is affordable where a general-category predicate is not: `core`
exposes no `Cf` predicate, and pulling in a Unicode tables crate to get one would
be the largest dependency in the contract crate by an order of magnitude. The
list is closed on purpose. U+202E in an event type is a log-spoofing vector — a
type that renders as one thing in every console and matches another in every
query — and the seven override and isolate characters are the only `Cf` codepoints
no script needs.

The residual hazard is named rather than fixed: U+200B and the other invisible
`Cf` characters stay legal, so two visually identical tags can be two different
consistency boundaries. That is the price of not banning `Cf`, VT-14 already
records it, and the mitigation belongs to the typed layer's validating
constructors and to a lint, not to the contract.

### 2. The four docstrings say "control characters (Unicode `Cc`)"

`tag.rs:47`, `event.rs:37`, `error.rs:29` and `error.rs:53` lose the word "ASCII".
Two words at four sites, landing in the same change as the bidirectional
`matches!` and the new `InvalidTag`/`InvalidEventType` variant, so that
`Latin1IdentifierStore`'s provenance is true before and after rather than only
before.

Both error enums gain a `BidirectionalControl` variant. Both are already
`#[non_exhaustive]` (`error.rs:18`, `error.rs:40`), so this is additive, and a
separate variant rather than a reuse of `ControlCharacter` because the two
refusals have different remedies: a control character in an identifier is almost
always an encoding bug, and a bidirectional override is almost always an attack.

### 3. `EventType` and `Tag` become const-constructible

Both change their backing field from `Box<str>` to `Cow<'static, str>` and gain

```rust
pub const fn from_static(value: &'static str) -> Self
```

which validates with `assert!` in a `const` context.

**Why `Cow` and not something else.** `Cow<'static, str>` is an enum with two
arms — `Borrowed(&'static str)` and `Owned(String)` — so one type expresses both
"this identifier was written in the source and is baked into the binary" and "this
identifier arrived from a peer at runtime and had to be allocated". Ingest needs
the second; interning wants the first; a newtype over `&'static str` alone can
only express the first and would make a runtime-derived event type
unrepresentable. That is the whole reason a plain `&'static str` loses.

**Compiled, E11.** `Borrowed("A")` and `Owned("A")` compare equal, hash equal, and
are one `HashMap` key; a lookup by `&str` finds a key inserted as `Borrowed`.
`size_of::<Cow<'static, str>>()` is 24 against `Box<str>`'s 16 — eight bytes on a
heap-indirected type, paid on every `Event`, in exchange for removing an
allocation from every event construction on the hot path. The construct compiles
under `#![no_std]` with `extern crate alloc`, which was not obvious and is
checked, because `Cow` lives in `alloc::borrow` and its `Owned` arm needs
`alloc::string::String`. (`happenstance-core` is `no_std` *conditionally* —
`#![cfg_attr(not(feature = "std"), no_std)]` at `lib.rs:84`, with `extern crate
alloc` unconditional at `:87` — and the gate's `--no-default-features` doc build
is what keeps that arm honest. A `std`-only construct would not fail the default
build; it would fail that one.)

**What "intern one `EventType` per `DomainEvent`" actually buys, precisely.** A
`const` in Rust is not a static: it is inlined at each mention and
re-materialised as a fresh value, so `Event::new(TY, …)` twice does not move `TY`
the first time and does not need a `clone` the second — compiled, E11,
`a_const_is_inlined_not_moved`. What is shared is not the `EventType` value but
the `&'static str` its `Borrowed` arm points at, which is in `.rodata`. So the
gain is *zero allocations and zero validations per construction*, not *one object*.
That is the whole gain worth claiming, and claiming the other one would be wrong.

**`Eq`, `Hash` and `Ord` are hand-written, not derived.** A derive on a
single-field tuple struct happens to produce the right answer today, and would
silently produce a different one the moment a second field is added — and phase 4
is adding fields to neighbouring types for exactly that reason (VT-4). Writing
them by hand also makes VT-33's `Borrow<str>` obligation explicit at the place it
is discharged: `Borrow` is not a convenience trait, it is a promise that the
borrowed form hashes and compares *identically* to the owner, and a `HashMap`
whose key type breaks that promise loses entries with no diagnostic anywhere.

The cost of hand-writing is worth saying out loud, because it is the mirror image
of the benefit. A `derive` cannot make `Eq` and `Hash` disagree with each other;
a hand-written pair can, and a `HashMap` whose key type has an `Eq` its `Hash`
does not respect is silently broken in a way no lint sees. Both derives here
would in fact be correct today — `Cow`'s own `PartialEq`, `Hash` and `Ord` all
delegate to `str`, which is why `Borrowed("A")` and `Owned("A")` are one key
either way — so what is being bought is not correctness now but a place to stand
when a second field lands. That trade is only sound if the agreement with `str`
is *asserted* rather than asserted-about, which is why VT-33 makes the test a
requirement rather than a suggestion.

**The hazard, which is sharper than E6 recorded and than the crate documents.**
`assert!` in a `const fn` is the mechanism that turns invalid input into a compile
error, but only where the compiler is obliged to evaluate the constant, and Rust's
rules about *when* that happens are not intuitive. E11 measured all four cases,
each one run rather than argued:

| Call site | `cargo check` | `cargo clippy` | `cargo build` / `cargo test` |
|---|---|---|---|
| free `const X: EventType = …from_static("")` | **E0080** | E0080 | E0080 |
| associated `const`, **read** somewhere | passes | passes | **E0080** |
| associated `const`, never read | passes | passes | **passes** |
| `let x = EventType::from_static("")` | passes | passes | passes, panics at run time |

```text
error[E0080]: evaluation panicked: an event type must not be empty
  = note: evaluation of `probe::BAD` failed inside this call
```

Rows one, two and four reproduce what `Capability::declined` already documents at
`contract.rs:325-330`, which is the same construct one crate over and the reason
this repository has met the diagnostic before. **Row three is new and is worse
than the documented case**: an associated const is evaluated lazily, only when
monomorphised code reads it, so a `pub const TY: EventType` declared in a codec
registry and never referenced holds an invalid value through `check`, `clippy`,
`build` *and* `test`. Nothing in the gate sees it. That is stated here so nobody
credits `from_static` with a guarantee it does not carry, and it is a MUST in
VT-32's documentation obligation.

Two consequences follow, and both are costs this decision accepts rather than
hides:

- Phase 4's exit criterion 7 — "`EventType::from_static("")` is a compile error,
  demonstrated by a `trybuild` case" — is **only checkable at a free `const` call
  site**. A `let` binding compiles and panics at run time; an associated const may
  never fail at all. The `trybuild` case must be written in the free-const form,
  and the criterion's wording is owed a correction.
- Satisfying it pulls **phase 6's `trybuild` dependency decision forward**
  (`adapter-shapes.md:97-102`). A bare `compile_fail` doctest is the cheaper
  instrument and is what `contract.rs` uses today, but it pins nothing beyond
  "something refused to compile", and the reason is worth getting right because
  the obvious reason is wrong. **This diagnostic does carry a code:** it is
  `error[E0080]`, and `--message-format=json` reports
  `"code":{"code":"E0080",…}` for it (E11). That distinguishes it from the six
  diagnostics `adapter-shapes.md:97-102` generalises PS-36 over, which genuinely
  report `code: None`; this family is **not** one of them, and reading
  `adapter-shapes.md` as though it covered every compile-time refusal in the
  crate is the mistake to avoid here. The annotation is worthless for a
  different reason, which is the one `contract.rs:320-323` already gives:
  **rustdoc on stable 1.97.1 does not enforce a doctest's error code at all.**
  Compiled, E11: a fence spelled `compile_fail,E0999` — a code no rustc emits —
  over a plain `let x: u32 = "not a number";` passes, and so does the same bogus
  code over the `from_static` case. So `compile_fail,E0080` is not a stronger
  check than bare `compile_fail`; it is the same check wearing a claim. A
  `compile_fail` doctest ships now as documentation; whether the *gate* gets a
  `trybuild` snapshot is a dependency question, and this ADR asks phase 4 to
  answer it early rather than pretending a doctest discharges the criterion.

**No clause requires any of this**, which is why VT-32 is owed. Its `Rejects:`
line has real content: it rejects a `from_static` that skips validation, or a
`new_unchecked`, or any pair of constructors enforcing different rules — the
divergence decision 1 refuted and which would otherwise be reintroduced by the
first person who finds the byte walk inconvenient. It also rejects a `Cow`-backed
type with derived `Eq`/`Hash` after a second field lands.

### 4. D4's fix ships in the same change

```rust
pub fn new<T>(event_type: T, data: impl Into<Bytes>) -> Result<Self, InvalidEventType>
where
    T: TryInto<EventType>,
    InvalidEventType: From<T::Error>,
```

plus

```rust
impl From<core::convert::Infallible> for InvalidEventType {
    fn from(never: core::convert::Infallible) -> Self { match never {} }
}
```

The `match` has no arms because `Infallible` has no values; the compiler accepts
an empty match on an uninhabited type as exhaustive, which is what lets an
infallible conversion satisfy a fallible bound. This is the shape `error.rs:77-84`
already uses for `InvalidQuery`, and it is a strict widening: E8 compiled it, and
E11 re-compiled it against the `Cow`-backed type, accepting a `&str`, an
already-built const `EventType`, and correctly rejecting a `&str` carrying U+202E.
Nothing that compiles today stops compiling.

### 5. The errors compose; the "umbrella" was never one type

VT-18 is `[FROZEN]` and mandates composition:

```rust
pub enum InvalidQuery {
    // … NoItems, UnconstrainedItem, EventType(InvalidEventType) …
    #[error(transparent)]
    Tag(#[from] InvalidTag),
}
```

The RUNBOOK item is titled "**One umbrella validation error**"
([`RUNBOOK.md:2923`](../RUNBOOK.md)). Read as a single collapsed `InvalidInput`,
that is forbidden by VT-18's own body (`SPECIFICATION.md:1231-1234`) on three
grounds: the three enums are returned by three different constructors and matching
on which one failed is worth keeping, `#[non_exhaustive]` enums are cheap, and the
conversion direction is unambiguous because a query can contain tags and a tag
cannot contain a query. **"Umbrella" is read here as composition**, and no frozen
clause is overturned.

**`AppendError<E>`'s seam is explicitly left open, and it is phase 7's.** VT-18
does not mention it, and the omission is not an oversight in the clause — it is a
thing the contract crate cannot fix. `S::Error` is an associated type of a generic
parameter, so `happenstance-core` cannot write a `From` impl into it or out of it;
only the adapter or the caller can. `impl<E> From<InvalidQuery> for AppendError<E>`
*is* writable — `AppendError` is local — and is rejected anyway, because it would
put a caller's programming error into an enum whose entire purpose (VT-25, and
`error.rs:122-134`) is to let a caller distinguish store outcomes from each other.
A validation failure is not an append outcome, and a variant saying it is would
make `AppendError`'s match arms mean two incompatible things.

So, precisely:

- **E2E-51 as written closes at phase 4.** Its GIVEN/WHEN is a handler calling
  `Tags::from_pairs`, `QueryItem::new` and `Query::from_items` and propagating with
  `?`; after `impl From<InvalidTag> for InvalidQuery`, that compiles with
  `InvalidQuery` as the sole error type and no bespoke enum.
- **The four-way union in E2E-51's *Falsifies* prose does not**, and cannot in this
  crate. A handler that also reads and appends still needs its own error type or a
  `Box<dyn Error>`. That is the typed layer's ergonomic surface and belongs to
  [phase 7](../RUNBOOK.md#phase-7--the-typed-layer-and-the-worked-example), which
  is where the worked example that reaches for `anyhow` actually lives. The
  RUNBOOK's observation that the library "ships nothing to copy instead" is true
  and stays true until phase 7 ships it.

### 6. Two kinds of bound, and why a capacity limit must never reach `Deserialize`

VT-19 is `[FROZEN]` and is transcribed rather than decided, but the reasoning is
restated here because it is the part everyone gets wrong and the code has to
embody it in two places at once.

A **validity invariant** is enforced by the constructor and re-enforced on
deserialisation; a violating value is unrepresentable. `MAX_EVENT_TYPE_LEN` and
`MAX_TAG_LEN` are these, at 255 bytes (VT-20), because they are the two values an
adapter declares a column width against — no conformant store would accept a
longer one, so deferring the refusal to the write buys nothing.

A **capacity limit** is enforced at the store boundary and nowhere else. It MUST
NOT be enforced by a constructor and MUST NOT be enforced in `Deserialize`. That
prohibition is the counter-intuitive half, and the reason is the quarantine path.
"Validate on the way in" is the natural instinct, and applied to a capacity limit
it destroys the one thing a peer needs: a peer running a tighter bound than its
neighbour that refuses at *decode* cannot decode the event at all, so it cannot
identify it, cannot report which event was refused, and cannot park it for a human.
E2E-42 then bites — an event a peer never appended has no local position, is never
forwarded, and disappears permanently from a third peer's view. A limit enforced
at the store boundary produces a value the peer can hold, name and quarantine; a
limit enforced at the decoder produces a byte range the peer can only drop.

This also answers the objection that every new bound is a wire-compatibility
break. It is not a break at all: the bytes are unchanged and only the acceptance
set moves.

### 7. The four minima become public constants in `happenstance-core`

```rust
pub const MIN_SUPPORTED_EVENT_DATA_LEN:   usize = 65_536;
pub const MIN_SUPPORTED_TAGS_PER_EVENT:   usize = 64;
pub const MIN_SUPPORTED_QUERY_ITEMS:      usize = 128;
pub const MIN_SUPPORTED_EVENTS_PER_BATCH: usize = 128;
```

`suite.rs:132-142`'s four private consts become `use happenstance_core::{…}` and
the comment block above them (`suite.rs:113-130`), which exists precisely to
record that the constants did not exist yet, is replaced by a one-line citation.
The four rules keep their names and their numbers.

**A floor, never a ceiling, and the reason is a peer set.** A single
`MAX_EVENT_DATA_LEN` is either a straitjacket on Postgres — which will happily
hold Turnstile's 340 KB seat map — or a lie on a KV-backed peer that cannot hold
it at all, and it would freeze at 0.1 with everything else so the number could
never be revised. A floor tells an application what it may write and still expect
to replicate, and gives the sync layer a value to compare a peer's declared limit
against *before* it starts pushing.

**The "1 MiB" discrepancy is already resolved, and the stale text is now in the
testkit rather than the RUNBOOK.** `suite.rs:128-130` says *"Note the discrepancy
the runbook's own prose carries: its value-edge item says '1 MiB' and VT-21 says
65,536 bytes. The clause wins."* — present tense, and no longer true: phase 3
struck the RUNBOOK line through and corrected it in place
([`RUNBOOK.md:2051`](../RUNBOOK.md), recorded at `:2364`). The comment is right
about which document wins and wrong about the state of the other one, which is
exactly the failure mode the comment was written to prevent. It is replaced in the
same change that turns the four consts into imports.

**The two floors multiply, and nobody has done the multiplication.** VT-22
justifies 64 tags by keeping a multi-row tag insert "for a hundred-event batch"
inside `SQLITE_MAX_VARIABLE_NUMBER` = 32,766 at three parameters per tag:
100 × 64 × 3 = 19,200. But VT-24 mandates **128** events, not 100. At the floor
both clauses actually state, the tag insert alone is 128 × 64 × 3 = **24,576**
parameters, leaving 8,190 before the event rows' own parameters are counted — call
it four per event, 512, for 25,088 against 32,766. The pair still fits. It fits
with 23% headroom rather than the 41% VT-22's arithmetic implies, and an adapter
that binds one extra parameter per tag, or that adds a metadata column to the tag
insert, is inside a factor of 1.3 of the ceiling. Phase 8 must verify this against
a real driver rather than inherit it; VT-22's arithmetic is owed a restatement at
128.

**All four markers stay `[PROVISIONAL]`, and this ADR is explicit that two of the
four falsifiers no scheduled phase can reach.** CF-38 forbids a provisional marker
with an *empty* falsifier; it does not forbid a slow one, but a reader is entitled
to know which kind they are looking at.

| Clause | Falsifier | Who can observe it | Verdict |
|---|---|---|---|
| **VT-21** (65,536 bytes) | a named target that cannot honour 64 KiB | **Phase 9.** The Cloudflare Durable Object is the only target in the plan with a per-value cap; the legacy KV-backed form clears 64 KiB with room for the envelope | Reachable. Stays provisional until phase 9 runs the rule against a real Durable Object |
| **VT-22** (64 tags) | a domain event legitimately carrying more than 64 tags | **Nobody in the plan.** The six scenarios are closed and their richest event is Wattline's `SessionStarted` at eight. The store half is testable; the *domain* half needs an application nobody has written | Adoption-gated. Earliest observer is phase 12's first external user |
| **VT-23** (128 query items) | a decision model legitimately needing more than 128 items | **Split.** The store half — an adapter that generates one SQL parameter per item and silently fails past a driver limit — is reachable at phases 8 and 10. The domain half is adoption-gated, largest in the six scenarios being four | Half reachable. The provisional marker is doing two jobs and the clause should say which |
| **VT-24** (128 events) | a domain whose smallest indivisible unit of work exceeds 128 events | **Phase 8** for the parameter-ceiling half, per the multiplication above. Kestrel Cold Chain's largest conditioned group is under forty | Reachable at phase 8 for the mechanism, adoption-gated for the domain |

"Adoption-gated" is the honest name for VT-22's and half of VT-23's marker: it
means *revisable on field evidence*, not *an experiment is scheduled*. Recording
it that way is the difference between a live provisional and CF-38's failure
mode — a marker indistinguishable from a decision nobody wanted to make.

**VT-24 is about batch *size* and ES-17 is about batch *ownership*.** They are
different questions on the same parameter, and ADR-0012 owns the second. Neither
ADR may lift the other's marker in passing.

### 8. `AppendError::ExceedsStoreLimit`, and the fixture seam it needs

VT-25 is `[FROZEN]` and unimplemented. It lands as:

```rust
#[non_exhaustive]
pub enum StoreLimit {
    EventDataLen,
    TagsPerEvent,
    EventsPerBatch,
}

// in AppendError<E>:
ExceedsStoreLimit { limit: StoreLimit, len: usize },
```

`AppendError` is already `#[non_exhaustive]` and its documentation already tells
callers a wildcard arm is required (`error.rs:146-147`), so the variant is
additive.

**Three variants, not four, and that is a decision.** VT-21, VT-22 and VT-24 each
say a store "MUST refuse beyond it with `AppendError::ExceedsStoreLimit`". VT-23
deliberately does not: a query-item refusal is not an append outcome, and VT-23
routes an over-large query to the ingest policy seam (§4) rather than to an error
variant. Putting `QueryItems` in `StoreLimit` would create a variant that no
`append` can ever produce, which is the decorative-variant version of the
decorative-rule failure.

Without this variant a caller cannot tell *"this event will never be accepted
here, park it and tell a human"* from *"the disk is full, retry"*, and the sync
runner — which must make exactly that distinction to avoid E2E-42's
disappearance — has nothing to switch on.

**The rule that checks it cannot be written against a store with no limit, and
that is a testkit contract change this ADR names but does not make.**
`append_reports_exceeded_store_limits` needs an adapter that *has* a ceiling and
declares it; `MemoryEventStore` has neither. `Fixture` (`contract.rs`) declares
three `Capability` constants — `SECOND_HANDLE`, `REOPEN`, `MID_BATCH_FAULT` — and
none of them can carry a number. The rule therefore needs the fixture to state its
actual limits, which is the machine-readable form of the "MUST document its actual
limit" that VT-21, VT-22 and VT-24 already require, in the shape
`const MAX_EVENT_DATA_LEN: Option<usize>` and two siblings, with `None` reported
through the same declined-capability path that already prints a reason into the
adapter's CI log.

That is a change to the fixture contract and therefore a CF-clause question —
**and this ADR is deliberate that the question has no owner yet, because the
reading that gives it one is wrong and is easy to reach.** That reading is
"CF-18 assigns fixture capabilities to ADR-0012, so this goes there." It does
not. What `SPECIFICATION.md:6794-6804` assigns is the **`MID_BATCH_FAULT`**
clause, it assigns it to **phase 4** by name, and it names no ADR. ADR-0012 has
taken that slot as **CF-39**, and CF-39 is about a *boolean* capability: whether
an injected fault is honoured when armed. Nothing in CF-16 – CF-19, in CF-39, or
anywhere else in the tree says a fixture may declare a **number**. Parking this
blocker on ADR-0012 would therefore park it on an ADR that has not accepted it,
which is indistinguishable in a work queue from having placed it.

So the question the draft surfaced was: `append_reports_exceeded_store_limits`
needs a fixture-contract extension that no clause and no phase-4 ADR owned, and
a human had to place it before the rule could be written. **Placed here at
sign-off, 2026-08-08, as CF-40.**

The reason it lands here rather than beside CF-39 is that **ownership follows the
obligation, not the file.** The rule exists to check VT-25 and VT-19, both of
which are this ADR's; both mutants owed with it are value-type mutants; and the
competing argument — keeping the CF numbering in one ADR — buys tidiness at the
price of answering *"may a fixture declare a number?"* inside a document about
`append`'s preconditions, which is a lookup nobody makes. CF numbers are already
spread across §6 by subject, and ADR-0012 has taken exactly one.

> **CF-40.** A `Fixture` MUST be able to declare the numeric limits its store
> actually enforces, as associated constants
> `MAX_EVENT_DATA_LEN: Option<usize>`, `MAX_TAGS_PER_EVENT: Option<usize>` and
> `MAX_EVENTS_PER_BATCH: Option<usize>`, each defaulting to `None`. `None` means
> the store enforces no ceiling on that dimension and MUST be reported through
> the same declined-capability path as a `Capability`, with a stated reason, so
> that a skipped limit rule is distinguishable in CI output from a passing one. A
> fixture declaring `Some(n)` asserts that its store refuses at `n + 1` with
> `AppendError::ExceedsStoreLimit` and accepts at `n`.
> `[PROVISIONAL — falsified by an adapter whose real ceiling is not a constant:
> a Postgres row limited by a shared `SQLITE_MAX_VARIABLE_NUMBER`-style budget
> across tags *and* rows together, or a Durable Object whose value cap moves with
> its storage backend, would make `Option<usize>` the wrong shape and the
> declaration a function of the batch rather than of the store. The instruments
> are `happenstance-sqlite` at phase 8 and `happenstance-postgres` at phase 10;
> neither has a body, and `MemoryEventStore` has no ceiling to declare, so this
> clause is written against no passing implementation and says so.]`
> `Rule:` new `append_reports_exceeded_store_limits`, which is VT-25's and is
> unwritable without this.
> `Cases:` E2E-35, E2E-42.
> `Rejects:` a fixture that declares a limit it does not enforce — the rule then
> asserts a refusal that never comes and fails, which is the correct direction —
> and, more importantly, the **absence** this clause repairs: with no way to
> declare a number, `append_reports_exceeded_store_limits` cannot be written at
> all, and VT-25 ships `[FROZEN]`, implemented, and checked by nothing.

**Why `Option<usize>` and not a `Capability`.** `Capability` (`contract.rs:288`)
is an opaque newtype over `Option<&'static str>` whose whole design forbids
representing a declined state with no reason. It is the right shape for a
yes/no whose "no" needs explaining, and the wrong shape here: a store with no
ceiling is not declining to cooperate, it is reporting a fact about itself. Using
`Option<usize>` keeps the two ideas apart, and the `None` arm still routes
through `RuleOutcome::Skipped` so the CI line is identical.

Two mutants are owed with it: a conformant store that declares a limit and
refuses correctly, and one that reports the same refusal through
`AppendError::Store` — which is what **every** adapter does today, because there
is no other variant, and which `PayloadCeilingStore` (`mutation_coverage.rs:1116`)
already models one clause over.

### 9. `Tags::value_of` is declined; `Tags::values_of` ships instead

VT-17 is `[FROZEN]`: "`Tags` MUST NOT offer a `get(key)` accessor", because
*"an accessor that returns one value where two may exist would make the ambiguity
invisible rather than resolving it"* (`SPECIFICATION.md:1188-1191`). Repeated keys
need no decision — VT-17 settles them legal, and `Tags::from_pairs([("tenant",
"a"), ("tenant", "b")])` yields a two-element set today, because deduplication is
on the whole `key:value` string.

The RUNBOOK asks for `Tags::value_of(key)` as a `partition_point` on the `"key:"`
prefix ([`RUNBOOK.md:2985-2988`](../RUNBOOK.md)), notices the conflict, and does
not resolve it. **The single-value form is declined.** What ships instead is

```rust
pub fn values_of(&self, key: &str) -> impl Iterator<Item = &str> + '_
```

which returns **every** value under `key`, in canonical order, and is empty when
there is none. It keeps the O(log n) the RUNBOOK wanted and keeps the ambiguity
visible, which is the property VT-17's prohibition protects. This is not a
loophole around a frozen MUST: VT-17 names `get(key)` specifically and gives the
reason, and a form that returns all matches does not have the defect the reason
describes.

The mechanism is worth explaining, because it is where VT-16's canonicalisation
gets paid back. `Tags` is sorted by the byte ordering of the *whole* tag string
(`tag.rs:137-145`), and every string beginning `"key:"` therefore occupies one
contiguous run of that ordering — lexicographic order does not interleave a
prefix's members with anything else. `slice::partition_point` is a binary search
that returns the index where a predicate stops holding, so two calls bound the run
and the iterator is a slice range with a `strip_prefix` on each element. Both
predicates are monotone, which is what makes two binary searches sufficient
rather than one plus a linear walk: `t < "key:"` is true then false across the
sorted slice, and so is `t < "key:" || t.starts_with("key:")`. No allocation, no
scan, and the existing sort is the only thing that makes it possible.

The worked example writes that scan by hand **once**, at O(n)
(`examples/course-subscriptions/src/main.rs:136-138`).
[`RUNBOOK.md:2986`](../RUNBOOK.md) says "twice"; it is one site, and the
correction is owed with the others in item 15. One site is still the argument,
because it is the only place in the workspace that has ever wanted this and it
reached straight past the type for `iter().any()`.

Two things stay unchanged and are stated so nobody re-derives them: the contract
still does not require a colon, `Tag::key` may still return `None`, and
`values_of` on a key nobody used returns an empty iterator rather than an error.

### 10. `ProjectionId` stays infallible, and its docstring stops teaching the wrong lesson

`projection.rs:41-56` is `pub fn new(value: impl Into<String>) -> Self` — no
validation at all — while both its siblings return `Result`. `ProjectionId::new("")`
succeeds and becomes the primary key of a checkpoint row. No VT clause covers it.

The RUNBOOK offers two exits: validate it the way `EventType` is validated, or
"state in the docstring that it is a deliberately opaque operator-chosen key"
([`RUNBOOK.md:2976-2980`](../RUNBOOK.md)). **Phase 4 takes neither, and this is a
result rather than a dodge.**

Validating it now is out of scope in a way that matters: `ProjectionId` belongs to
`ProjectionStore`, which is `[PROVISIONAL]`, has **no conformance suite at all**,
and is frozen at phase 6. Phase 4 has no instrument that could fail a
`ProjectionId` rule, and CLAUDE.md's standing corollary — a rule that no adapter
can fail is decorative — applies to a validating constructor with nothing to check
it just as it applies to a rule.

Calling it "deliberately opaque" is worse, because it is false. There is no
decision behind the current signature; there is an omission, and blessing it in a
docstring would freeze an accident at exactly the moment the reader is most likely
to believe the document.

A half-measure is worse than either whole. Adding a fallible `ProjectionId::parse`
beside the infallible `new` reproduces the D2 mistake VT-16 names: an infallible
constructor that can produce a value the fallible one rejects, which is the defect
VT-26 closes `Query::Items` against.

So the decision is: **`new` is unchanged, and the docstring says the validation
question is open, names phase 6 as its owner, and names the empty-string
primary-key hazard.** A reader who currently learns "validation here is optional"
learns instead "validation here is unresolved", which is both true and actionable.

### 11. Normalisation: a transcription into `Tag`'s docs, not a decision

VT-15 is `[FROZEN]`. Equality is byte equality over the UTF-8 encoding, and the
contract — and every adapter — applies no normalisation, case folding or trimming.
Whitespace is significant everywhere, including leading and trailing.

`Tag`'s documentation gains the position in terms, with both worked failures,
because the exit criterion asks for a *stated position* and a position nobody can
find is not one:

- `"café"` in NFC (U+00E9) and in NFD (`e` + U+0301) are two different `Tag`s. A
  macOS client and a Linux client writing the "same" tag silently fail to
  conflict — two consistency boundaries where the application intended one, with
  no visible cue at all.
- Kestrel Rotor's `turbine:HW2-A14 ` with a trailing space: `Tag::new` accepts it,
  `Tags` sorts it adjacent to the real tag, and `contains_all` is a strict
  merge-scan on equality (`tag.rs:231-245`) that does not match it. A lot-recall
  query silently missed a turbine.

Both are the application's to prevent, and the contract's obligation is to say so
loudly rather than to guess which normal form the caller meant.

The RUNBOOK adds a third reason for not normalising —"adds a dependency the
`no_std` build cannot take" — which is **stronger than the evidence supports** and
is corrected below. The objections that hold are two: normalising at construction
would rewrite a caller's data so the tag read back is not the tag written, which
breaks [ADR-0003](0003-opaque-payloads.md)'s byte-for-byte forwarding promise and
makes the store's index disagree with any external system holding the original
string; and Unicode tables would be, as VT-14 already says for the `Cf` predicate,
the largest dependency in the contract crate by an order of magnitude. Neither is
"`no_std` cannot", and overstating an argument that is already sufficient invites
someone to knock it down and take the conclusion with it.

### 12. The standard-library trait surface

Five impls ship together as VT-33, on the argument that they are ecosystem
conformance rather than new semantics — they make these types behave the way a
Rust programmer already expects, and their absence is felt at every call site
without ever producing a diagnostic that names the cause.

- **`Borrow<str>` for `Tag` and `EventType`.** A `HashMap<EventType, Handler>` is
  the core data structure of a codec registry. `HashMap::get` takes `&Q where K:
  Borrow<Q>`, so without this impl every lookup must construct an `EventType` —
  an allocation and a re-validation — to probe a map. `Borrow` is the trait with a
  documented extra obligation: the borrowed form must hash and compare exactly as
  the owner does. Decision 3's hand-written `Eq`/`Hash`/`Ord` discharge it, and
  VT-33 requires a test that asserts it rather than a comment claiming it, because
  a violation loses map entries with no diagnostic anywhere.
- **`FromStr` for `Tag` and `EventType`**, with `Err = InvalidTag` /
  `InvalidEventType`. This is what makes `"course:c1".parse::<Tag>()?` work and is
  the spelling every configuration parser and CLI reaches for. It is a second
  entry point to the same validator, not a second validator.
- **Owned `IntoIterator for Tags`.** `&Tags` already iterates (`tag.rs:268-275`)
  and `From<Tags> for Vec<Tag>` already exists, so the owned form is the missing
  third corner and it lets a caller drain a set without a clone.
- **`Extend<Tag> for Tags`**, re-canonicalising per call. The cost is stated
  rather than hidden: because canonicality is an invariant of the type (VT-16),
  `extend` must sort and dedup the merged result, so it is O(n log n) per call and
  **not** the amortised O(1) `Extend` usually implies. VT-33 requires the
  docstring to say so; a caller building a large set in a loop should collect and
  `FromIterator` once.
- **`Tags::values_of`**, per decision 9.

**`Hash` on `Event` is declined, and the reason is that phase 13 asked for the
wrong instrument.** The RUNBOOK wants "`Hash` on the wire types, which phase 13's
dedup needs" ([`RUNBOOK.md:2983`](../RUNBOOK.md)). Content hashing is not how a
replicated event should be deduplicated: VT-4 and VT-5 give `SequencedEvent` a
store-assigned `EventId`, and two legitimately distinct events with identical
type, data and tags are ordinary — a `SeatReleased` for the same seat twice is not
a duplicate. A `Hash` impl on `Event` would make the wrong dedup one line away and
the right one no easier. The dedup key is **ADR-0014's** (`EventId`) and the dedup
policy is **phase 13's**.

**And the prohibition is not free, so it is priced here rather than waved
through.** VT-33 lands `[FROZEN]`, so "`Event` MUST NOT implement `Hash`" costs a
*superseding ADR* to undo — phase 13 cannot simply add the impl if it later finds
a job for a content hash that `EventId` cannot do. That is the intended cost: the
impl is one line, the wrong use of it is one line, and neither is visible in
review, so the thing worth making expensive is the addition rather than the
misuse. But a frozen MUST NOT written on phase 13's behalf, on phase 4's
evidence, is exactly the "deciding early on no evidence" this runbook is
organised against, and it is the one place in this ADR where a human should push
back if they disagree. The alternative offered and not taken was to leave the
prohibition non-normative — in VT-33's `Rejects:` line only, where it explains a
wrong implementation without forbidding a right one — which would cost phase 13 a
paragraph instead of an ADR and would let the impl arrive without anyone
noticing. Freezing it is a judgement that the second failure is worse.

## Consequences

**Good.** The crate acquires one validator instead of two, and it is checked
against every Unicode scalar value rather than against the cases someone thought
of. VT-14's bidirectional half exists for the first time, and the four docstrings
stop describing a validator the crate does not have — together, so
`Latin1IdentifierStore`'s provenance is never momentarily false.

**Good.** An `EventType` can be a `const`, and can be passed to `Event::new`. The
typed layer can intern one per `DomainEvent`, on `no_std`, with no allocation on
the write path. Neither half would have been worth shipping alone.

**Good.** The four minima stop being a number the suite believes and the contract
does not state. `store_accepts_the_guaranteed_minimum_payload` and its three
siblings cite the constant they enforce, so a future revision moves the rule with
the number instead of leaving them to drift apart.

**Bad, and measured.** `EventType` and `Tag` grow from 16 to 24 bytes. Every
`Event` carries one `EventType`, and every `Tags` carries N `Tag`s, so a
64-tag event pays 520 bytes it did not pay before. The compensating removal is one
allocation per identifier construction on the hot path; the trade is favourable
for a write-heavy store and unfavourable for a read path that materialises many
events and never constructs an identifier. Nothing in the workspace measures
either today, and phase 8's harness is the first thing that could.

**Bad, and the sharpest thing in this ADR.** `from_static`'s compile-time
guarantee has a hole the gate cannot see: an associated `const` that is never read
is never evaluated, so an invalid one survives `check`, `clippy`, `build` and
`test`. This is documented in VT-32 and on `from_static` itself. It is not fixed,
because Rust offers no mechanism to force evaluation of a constant nobody uses.

**Bad, and it needs a human.** VT-25's rule is blocked on a fixture-contract
change — a fixture declaring its *actual numeric* limits — that **no clause and
no phase-4 ADR owns**. So `AppendError::ExceedsStoreLimit` ships before the rule
that proves any adapter reports through it. That is a frozen clause implemented
ahead of its check, which is the ordering this repository normally refuses. It is
accepted here because the variant is what unblocks the rule rather than the other
way round. What is *not* settled, and must be settled before the rule is written,
is which ADR's clause set the fixture constants land in: this one, because the
rule serves VT-25 and VT-19, or ADR-0012's, because it is already amending the
fixture contract at CF-39. Decision 8 sets out both and declines to choose.

**Bad.** Two of the four minima carry provisional markers no scheduled phase can
lift. Calling that "adoption-gated" is honest but it is still a marker that will
sit unchanged through publication, and phase 12 must decide whether shipping 0.1
with two adoption-gated floors is acceptable or whether they are frozen on the
evidence available then.

**Neutral.** The census in `SPECIFICATION.md` §1.3 moves by two for this ADR alone,
and ADR-0013's ES-10 lift moves the maturity split independently. That count is
the one number in the document a human computes by reading and `spec-trace` checks
rather than generates, so it must be recomputed **once, by hand, after all five
phase-4 ADRs land** — not five times, and not by whoever merges last.

## Alternatives rejected

- **Narrow the character rule to ASCII C0**, which E6's divergence made look
  necessary and which would have made the four "ASCII" docstrings true for free.
  Rejected because decision 1's exhaustive compile shows the divergence does not
  exist: a byte walk sees the C1 range through the `C2` lead byte. Taking the
  escape would have weakened a `[PROVISIONAL]` MUST to work around a limitation
  that is not there, and VT-14's `Rejects:` line names an ASCII-only validator as
  a wrong implementation.

- **Reject all of Unicode `Cf`.** The natural over-correction, and it bans U+200C
  and U+200D, which Persian and Hindi orthography and every emoji ZWJ sequence
  require. VT-14 rejects it in terms. The narrow seven-codepoint list is what
  remains affordable without a Unicode tables dependency.

- **Normalise at construction (NFC).** It would make the macOS/Linux `"café"`
  failure go away, which is a real failure. It loses on two independent grounds:
  it rewrites a caller's data so the tag read back is not the tag written, which
  breaks ADR-0003's byte-for-byte forwarding and makes the store's index disagree
  with any external system holding the original string; and VT-15 is `[FROZEN]`
  against it, so it is a superseding-ADR question rather than a design one.

- **A proc macro, `event_type!("CourseDefined")`.** Validates at expansion, which
  is the same guarantee `from_static` gives at a free `const` site, and it still
  allocates at run time unless it also produces a `Cow::Borrowed` — at which point
  it is `from_static` with a dependency in front of it. `proc-macro2`, `quote` and
  `syn` in the contract crate is a large cost for a spelling.

- **`&'static str` only, with no `Owned` arm.** Half the size and simpler, and it
  cannot express a runtime-derived event type, which is exactly what ingest
  produces. The type would then need a sibling for the runtime case and the crate
  would have two identifier types, which is worse than eight bytes.

- **`EventType::new_unchecked`.** The usual escape from validation cost. There is
  no performance story for it here — the validator is a byte walk over at most 255
  bytes and `from_static` runs it at compile time — and under this workspace's
  `unsafe_code = "forbid"` it could not even be `unsafe`, so it would be a safe
  function that silently breaks the type's only invariant.

- **Deriving `Eq`, `Hash` and `Ord` on the `Cow`-backed newtypes.** Correct today,
  and it becomes silently wrong the first time a second field is added — which
  phase 4 is doing to neighbouring types right now. Hand-writing them costs
  fifteen lines and puts `Borrow`'s consistency obligation at the site that owes it.

- **A single `InvalidInput` the three validation enums collapse into.** VT-18
  rejects it by name and gives three reasons. The `From` composition delivers what
  E2E-51 actually asks for.

- **`impl<E> From<InvalidQuery> for AppendError<E>`,** to make the worked
  example's four error types into three. Writable — `AppendError` is local, so
  coherence permits it — and rejected because it puts a caller's programming error
  into the enum whose job is distinguishing store outcomes, and it still leaves
  `S::Error` from the read outside. Three types instead of four is not "one error
  type"; it is the same problem with a worse `AppendError`.

- **`Tags::value_of(key) -> Option<&str>`,** which is what the RUNBOOK asks for.
  VT-17 `[FROZEN]` forbids exactly the property it has: returning one value where
  two may legally exist, making the ambiguity invisible. `values_of` gives the same
  O(log n) and keeps the ambiguity.

- **Enforcing a tag *count* in `Tags`,** or an event-data ceiling in `Event`.
  VT-19 and VT-22 forbid it, and the reason is the quarantine path in decision 6.
  It also re-creates D2: a `FromIterator` that cannot fail beside a `new` that can.

- **Making `ProjectionId::new` fallible now.** The change is right in the
  abstract and cannot be checked by anything phase 4 owns, because
  `ProjectionStore` has no conformance suite. Deferred to phase 6 with the hazard
  documented, which is decision 10.

- **A `MAX_EVENT_DATA_LEN` ceiling** instead of a floor, and an `EventBatch`
  newtype to carry a batch ceiling. Both are rejected in terms by VT-21 and VT-24
  respectively; the batch newtype additionally constrains ADR-0012 and is recorded
  here so that ADR does not reach for it.

## Amendments this decision owes the specification

Eighteen items. Recorded rather than applied, because six of them touch
`[FROZEN]` clauses — items 2, 3, 4 and 9 amend existing ones, and items 10 and 11
add two more — and this repository's rule is that a frozen clause changes by ADR
and not by edit. Item 12 adds a clause too, but `[PROVISIONAL]`, so it is not in
that six. **Where a clause below is marked frozen, this ADR is the
authority for the change.** None of them reverses a normative MUST; three add
one (item 2's documentation MUST on `Tag`, and the two new clauses).

**No `[FROZEN]` clause is overturned by this ADR.** VT-15, VT-16, VT-17, VT-18,
VT-19, VT-20 and VT-25 are all implemented as written.

### Clauses amended

1. **VT-14** — `[PROVISIONAL]`, not frozen. Its `Rule:` line says the unit tests
   "today cover the `Cc` half only, because **phase 4 owns the bidirectional
   extension and phase 3 wrote none of it**". It must name the landed tests and
   drop the ownership note. Its body must additionally record decision 1's
   compiled finding — that a `const fn` byte walk sees the C1 range and the seven
   bidirectional controls, so one validator serves both constructors — because the
   clause's own text implies a general-category predicate is the only way to reach
   `Cc` and that shaped E6's conclusion. **The marker stays**, with its falsifier
   unchanged: a legitimate event type or tag that requires an explicit
   bidirectional formatting control.

2. **VT-15** — `[FROZEN]`; **this ADR is the authority**, and this adds a MUST.
   The clause records the normalisation position but places no obligation on the
   documentation. It gains one, in the shape VT-17 already uses for `Tags`:
   *"`Tag`'s documentation MUST state that no normalisation is applied and MUST
   name the NFC/NFD case."* Without it, phase 4's exit criterion "a stated position
   on Unicode normalisation exists in `Tag`'s docs" is checked by nothing.

3. **VT-17** (`:1167`) — `[FROZEN]`; **this ADR is the authority**. Two edits.
   Its `Cases:` line (`:1175-1178`) currently reads *"none. This clause comes from
   Wattline D3 … and no E2E case covers it"*, and §7.5 (`:8066`) calls that a
   genuine hole; it becomes the new E2E case in item 14. And the closing
   paragraph (`:1186-1192`), which today reads *"The absence of `Tags::get` is the
   same decision seen from the read side: an accessor that returns one value
   where two may exist would make the ambiguity invisible rather than resolving
   it"*, gains a following sentence: *"An accessor returning **every** value under
   a key — `Tags::values_of(key)` — is permitted and is what the contract ships,
   because it does not have the defect this paragraph describes; the MUST NOT is
   on `get(key)`'s shape, not on prefix lookup."* Without it the next reader
   deletes `values_of` as a violation.

4. **VT-18** — `[FROZEN]`; **this ADR is the authority**. Its `Rule:` names two
   compile tests in `crates/happenstance-core/tests/`, a directory that does not
   exist and which this work creates. Its `Cases:` line must record that E2E-51 as
   *written* closes here and that the four-way union in E2E-51's *Falsifies* prose
   does not and is phase 7's — otherwise the case is reported green while the
   observation that produced it stands.

5. **VT-21** — `[PROVISIONAL]`. Its `Rule:` line says the rule was written
   "against **this clause's number rather than against a constant**, because
   `MIN_SUPPORTED_EVENT_DATA_LEN` does not exist until phase 4 introduces it". It
   does now; the sentence must say the rule cites the constant. Marker stays;
   falsifier's owning phase named as **9**.

   **Nothing inside the square brackets changes.** The owning phase is recorded
   in the clause body, not in the marker: `spec-trace` reads the bracketed
   falsifier and a marker edited into "provisional, phase 9" would pass its
   ≥12-character check while having lost the falsifier it is there to carry.
   The same applies to items 6, 7 and 8.

6. **VT-22** (`:1328`) — `[PROVISIONAL]`. Its closing sentence (`:1346-1348`)
   reads *"Sixty-four is eight times the observed maximum and keeps a multi-row
   tag insert for a hundred-event batch inside SQLite's
   `SQLITE_MAX_VARIABLE_NUMBER` of 32,766 at three parameters per tag."* The
   hundred-event batch is not a floor anyone states; VT-24 mandates 128. Restate
   at the floor both clauses actually state: 128 × 64 × 3 = 24,576 parameters
   against 32,766, before the event rows' own parameters, which is 23% headroom
   and not the 41% the current arithmetic implies. The marker's bracketed text —
   *"falsified by a domain event legitimately carrying more than 64 tags; the
   richest event in the six scenarios is Wattline's `SessionStarted` at eight"* —
   is **kept verbatim**; what is added, in the body, is that no scheduled phase
   can observe it, that it is therefore adoption-gated, and that the earliest
   observer is phase 12.

7. **VT-23** (`:1350`) — `[PROVISIONAL]`. Bracketed falsifier kept verbatim. The
   body gains the split: the *store* half — an adapter that generates one SQL
   parameter per item and silently fails past a driver limit, which the clause's
   own `Rejects:` line already names — is reachable at phases 8 and 10; the
   *domain* half, a decision model legitimately needing more than 128 items, is
   adoption-gated. One marker is presently doing both jobs and a reader cannot
   tell which they are looking at.

8. **VT-24** (`:1371`) — `[PROVISIONAL]`. Bracketed falsifier kept verbatim; the
   body names **phase 8** as the owner of the parameter-ceiling half, per item 6's
   arithmetic, and records that VT-24 governs batch *size* while ES-17 governs
   batch *ownership*, so ADR-0012 does not lift this marker in passing.

9. **VT-25** (`:1395`) — `[FROZEN]`; **this ADR is the authority**. Two additions.
   `StoreLimit` carries three variants and not four, and the clause must name
   which limits (VT-21, VT-22, VT-24) and why VT-23 is excluded — a query-item
   refusal is not an append outcome, so a `QueryItems` variant would be one no
   `append` could ever produce. And its `Rule:` line, which currently reads
   `new append_reports_exceeded_store_limits` and nothing else, must record that
   the rule depends on a fixture able to declare its **actual numeric** limits
   and **cite CF-40** as the clause that supplies them. It must **not** cite
   CF-39; that is `MID_BATCH_FAULT`, a boolean, and ADR-0012's.

### Clauses added

10. **VT-32 — identifiers are const-constructible, and both constructors enforce
    one rule.** New, in §2.4. `[FROZEN]`. `EventType` and `Tag` MUST be backed by
    `Cow<'static, str>` and MUST offer `pub const fn from_static(&'static str)`;
    `from_static` and `new` MUST accept exactly the same set of values; `Eq`,
    `Hash` and `Ord` MUST be hand-written and MUST agree with `str`'s; the
    documentation MUST state that an associated `const` which is never read is
    never evaluated. `Rule:` a unit test asserting `from_static` and `new` agree
    across the control, C1 and bidirectional boundaries; a `compile_fail` doctest
    at a free-`const` site, with a `trybuild` snapshot if phase 4 takes the
    dependency. `Cases:` E2E-40, E2E-51. `Rejects:` a `from_static` that skips
    validation or a `new_unchecked`; any pair of constructors enforcing different
    rules, which is the divergence a byte walk appears to force and does not; and
    a derived `Eq`/`Hash` that silently changes meaning when a second field lands.

11. **VT-33 — the standard-library trait surface.** New, in §2.4. `[FROZEN]`.
    `Tag` and `EventType` MUST implement `Borrow<str>` and `FromStr`; `Tags` MUST
    implement owned `IntoIterator` and `Extend<Tag>`, and MUST document that
    `extend` re-canonicalises and is therefore not amortised O(1); `Tags` MUST
    offer `values_of(key)` returning every match. `Event` MUST NOT implement
    `Hash`. `Rejects:` a codec registry whose every `HashMap` probe allocates an
    `EventType`; a `Borrow<str>` whose hash disagrees with `str`'s, which loses map
    entries silently; and a content-hash dedup on `Event`, which treats two
    legitimately identical domain events as one and whose correct instrument is
    `EventId` (ADR-0014).

12. **CF-40 — the fixture declares its numeric limits.** New, in §6.3, beside
    CF-39. `[PROVISIONAL]`. The clause, its marker, its `Rule:`, `Cases:` and
    `Rejects:` are given verbatim in
    [decision 8](#8-appenderrorexceedsstorelimit-and-the-fixture-seam-it-needs)
    and are
    transcribed from there rather than restated here, so the two cannot drift.
    It is placed in this ADR rather than in ADR-0012 by the sign-off of
    2026-08-08.

13. **§1.3's census** — 193 clause IDs / 191 normative / **135 `[FROZEN]`** /
    46 `[PROVISIONAL]` becomes **196 / 194 / 137 / 47** for this ADR's three
    additions (VT-32 and VT-33 frozen, CF-40 provisional). The other phase-4
    ADRs move it further and in both columns: ADR-0012 adds CF-39
    (`[PROVISIONAL]`), ADR-0014 adds ES-41 (`[PROVISIONAL]`), and ADR-0013 lifts
    ES-10 from `[PROVISIONAL]` to `[FROZEN]`, which moves two counts without
    moving the total. **Recompute once, by hand, after all five phase-4 ADRs
    land** — this is the one count in the document a human computes by reading
    and `spec-trace` checks rather than generates, so a per-ADR recount produces
    a sequence of wrong numbers and a red gate at each.

14. **§7.5** (`:8058`) — VT-17's row (`:8066`, "Defect: the case is missing")
    closes, and the closing sentence at `:8074`, "two genuine holes (VT-17,
    WF-12)", becomes one (WF-12, which is phase 5's). **And §7.6** (`:8077`):
    *"None. All 56 cases are claimed by at least one clause"* becomes 57. That
    sentence lives in `SPECIFICATION.md`, not in `E2E-CASES.md`, which is why it
    is here rather than in item 14.

### Owed outside `SPECIFICATION.md`

15. **`docs/scenarios/E2E-CASES.md` gains E2E-57**, VT-17's missing case:
    construct a `Tag` with no colon and one with two, assert both are accepted and
    that neither acquires structure — `key()` is `None` for the first and `Some`
    of the text before the *first* colon for the second, with the remainder,
    colon included, as the value.

    **Where it goes is not a free choice, and the arithmetic has to be done
    before the edit rather than during it.** The index table is at `:37-45`, and
    its groups are contiguous numeric ranges: A is E2E-01 … E2E-14, D is
    E2E-46 … E2E-51, and **E is E2E-52 … E2E-56**. A new E2E-**57** therefore
    extends **group E**, not group D — the case cannot be numbered 57 and filed
    in D without renumbering all five of E's cases and every citation to them.
    The house pattern is already to append numerically and let the topical fit
    be approximate: E2E-56 is an epoch/concurrency case sitting in "Edge and
    `!Send`" for exactly this reason. So: append as E2E-57, extend group E's
    range in the index, and say in the case body that it serves VT-17.
    Renumbering into group A is the alternative and it is not worth the
    citation churn; if a human prefers it, that is the moment to say so.

    Note that `spec_trace::check_citations` reads only `SPECIFICATION.md`, so
    nothing checks this file's cross-references; that is a live item in phase 4's
    own body.

16. **`docs/RUNBOOK.md`**, five edits, all in phase 4's body or its ledger. (The
    "1 MiB" discrepancy is **not** among them: phase 3 already corrected it at
    `:2051` and recorded the correction at `:2364`. The stale copy is
    `suite.rs:128-130`'s comment, which is item 17.)
    - `:2947-2948`, "normalising at construction … adds a dependency the `no_std`
      build cannot take" overstates the objection (decision 11); the objections
      that hold are ADR-0003's byte-for-byte promise and dependency weight;
    - `:2985-2988`, the `Tags::value_of(key)` item is superseded by `values_of`
      (decision 9);
    - `:2986`, "the O(n) scan the worked example writes by hand **twice**" — it is
      written once, at `examples/course-subscriptions/src/main.rs:136-138`. The
      count is wrong, not the argument;
    - `:2976-2980`, the `ProjectionId` item's "deliberately opaque operator-chosen
      key" wording is refused (decision 10); the docstring records an open
      question, not a decision;
    - `:3040-3041`, exit criterion 7 must specify a **free `const`** call site,
      since a `let` binding compiles and panics at run time and an associated
      const may never be evaluated at all (decision 3).

17. **`RUNBOOK.md:447-451` and `:553-566`** assign VT-14 and VT-21 – VT-24 to
    "phase 5", which is now the wire format. The specification names phase 4 in
    terms at VT-14 (`:1085-1086`) and VT-21 (`:1308-1310`), and under RUNBOOK
    rule 5 the clauses win. Only the rows for this ADR's clauses are corrected
    here; the neighbouring rot is the other four ADRs' to fix for their own
    clauses, and `:558`'s assignment of VT-10 to phase 13 is **correct** and must
    not be swept up.

18. **`crates/happenstance-testkit/src/suite.rs:113-142`** — the four private
    consts become imports; the comment block that explains their absence becomes a
    one-line citation; and `:128-130`'s claim that the RUNBOOK "carries" the 1 MiB
    discrepancy goes, because it has not been true since phase 3 corrected it.
    Two new mutants are owed with VT-25's rule
    (decision 8), each with a `Declared` row and a provenance paragraph, and
    `CHANGELOG` entries for the rule and the variant.
