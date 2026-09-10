# 10 — Newtypes and niches

> **Load when:** choosing between `type X = u64` and `struct X(u64)` · adding a
> store-assigned scalar to the contract · an `Option<T>` grew a word · about to
> subtract two positions · deciding `[u8; 16]` against `u128`
> **See also:** 11 (the `const` constructor these types get) · 12 (the impls a
> newtype then owes) · 13 (private fields) · 61 (asserting layout)

---

## RS-10-1. Newtype every value a wrong one is constructible for; never a type alias.

**Why.** An alias is a second name for one type, so every value and every
function of the underlying type is accepted at every alias-typed position. Only a
distinct nominal type gives you a constructor to validate in, an impl surface,
and a diagnostic at the mismatch.

**Do**

```rust
use happenstance_core::SequencePosition;
# fn column_value() -> u64 { 0 }
// The only way in is the constructor, and it refuses the forbidden value.
assert!(SequencePosition::new(column_value()).is_none());
```

**Not** — compiles; the two assertions are the values a newtype would have
refused:

```rust
type SequencePosition = u64;
fn assign(raw: u64) -> SequencePosition {
    raw
}

let tags_len: usize = 3;
assert_eq!(assign(0), 0, "zero is the one value a position may never take");
assert_eq!(assign(tags_len as u64), 3, "and a tag count is now a position");
```

**Rejects.** An adapter whose read path maps a nullable `INTEGER` column straight
into the position field: `NULL` arrives as `0`, every event in the batch sorts
below the first real position, and what anyone notices is a projection replaying
from the top of the log on every restart. The conformance suite never fed that
adapter a null column, so it found out in production, months after the suite went
green.

**Evidence.** `crates/happenstance-core/src/event.rs:240 (pub struct SequencePosition(NonZeroU64))` ·
[SPECIFICATION VT-11](../../spec/SPECIFICATION.md) ·
[API guidelines C-NEWTYPE](https://rust-lang.github.io/api-guidelines/type-safety.html) *(checked 2026-08-09, rustc 1.97.1)*

## RS-10-2. Spend a niche on the forbidden value rather than checking for it.

**Why.** `NonZeroU64` makes zero unrepresentable, and std *documents* — rather
than merely achieving — that `Option<NonZeroU64>` has the same size and alignment
as `NonZeroU64`. That guaranteed list is `num::NonZero*`, `&`/`&mut`, `Box`, `fn`,
`NonNull`, and a `#[repr(transparent)]` struct around one of them — a plain
newtype like `SequencePosition` is on none of them, so its niche is real but
**unguaranteed**, and `option_position_is_niche_optimised` is what holds it. The
niche is one level deep either way: `Option<Option<_>>` pays a word again.

**Do**

```rust
use happenstance_core::SequencePosition;

assert!(SequencePosition::new(0).is_none());
// `Option<SequencePosition>` sits in every `Guard` and every `ReadOptions`.
assert_eq!(
    size_of::<Option<SequencePosition>>(),
    size_of::<SequencePosition>(),
);
// The niche is spent; a second layer is not free.
assert_eq!(size_of::<Option<Option<SequencePosition>>>(), 16);
```

**Not** — compiles, refuses the same value, and costs a word per option:

```rust
#[derive(Debug, Default)]
struct Watermark(u64);

impl Watermark {
    fn new(value: u64) -> Option<Self> {
        (value != 0).then_some(Self(value))
    }
}

assert!(Watermark::new(0).is_none(), "the constructor is not the only door");
assert_eq!(Watermark::default().0, 0, "and `Default` walked through the other one");
assert_eq!(size_of::<Option<Watermark>>(), 2 * size_of::<Watermark>());
```

**Rejects.** A `Watermark(u64)` whose zero-check lives in `new` and whose
`#[derive(Default)]` does not go through `new`. `Watermark::default()` is then the
zero a position may not take, a sync runner reads it as "processed up to 0"
instead of "nothing processed yet", and the first replication into a fresh peer
skips the origin store's first event — silently, because every position it then
sees is greater than the watermark it started from.

**Evidence.** `crates/happenstance-core/src/event.rs:219 (makes position zero unrepresentable)` ·
`crates/happenstance-core/src/event.rs:1037 (option_position_is_niche_optimised)` ·
[SPECIFICATION VT-13](../../spec/SPECIFICATION.md) ·
[std `Option` representation](https://doc.rust-lang.org/std/option/index.html#representation) *(checked 2026-08-09, rustc 1.97.1)*

## RS-10-3. Give an ordering-key newtype `next()` and `Ord`, and no `Sub` or `Add` impl.

**Why.** Operator traits are opt-in, so the *absence* of `impl Sub` is the
instrument: `a - b` on `SequencePosition` is `error[E0369]`, raised in the crate
that wanted a count rather than in the one that owns the key. Adding the impl
publishes an infallible-looking arithmetic surface no constructor guards, cannot
be withdrawn without a breaking change, and demotes `get()` from the one audited
door out of the newtype to one of two.

**Do**

```rust
use happenstance_core::SequencePosition;
# fn pos(v: u64) -> SequencePosition {
#     match SequencePosition::new(v) { Some(p) => p, None => SequencePosition::FIRST }
# }
let seen = [pos(1), pos(4), pos(9)];

// Count what the read yielded; advance by asking the type, not by adding one.
assert_eq!(seen.len(), 3);
assert_eq!(seen[2].next(), SequencePosition::new(10));
assert!(seen[0] < seen[2], "`Ord` is the whole of what an ordering key owes");
```

**Not** — compiles: the same newtype with the operator its author wanted, and the
last assertion is what catches it:

```rust
use core::ops::Sub;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Position(u64);

impl Sub for Position {
    type Output = u64;
    fn sub(self, rhs: Self) -> u64 {
        self.0 - rhs.0
    }
}

let seen = [Position(1), Position(4), Position(9)];
let lag = seen[2] - seen[0];

assert_eq!(lag, 8);
assert_ne!(lag as usize, seen.len(), "the store held three events, not eight");
```

**Rejects.** A projection-lag gauge computed as `head - checkpoint`. It is
correct against `MemoryEventStore`, whose allocator leaves no gaps, and against
SQLite; against any store that gaps on rollback it reads orders of magnitude
high, so the first operator to see the dashboard pages someone about a
replication stall that is not happening, and the runbook entry they write says to
ignore the gauge.

**Evidence.** `crates/happenstance-core/src/event.rs:216 (an opaque ordering key)` ·
`crates/happenstance-core/src/event.rs:263 (is a threshold rather than a)` ·
[SPECIFICATION VT-11](../../spec/SPECIFICATION.md)

## RS-10-4. Where the return type *is* the overflow signal, use `checked_add`.

**Why.** `saturating_add` clamps to a value the newtype still accepts, so the
constructor wraps it back into `Some` and the signal is gone. `NonZeroU64`'s
`checked_add` is `const`, so the correct spelling costs neither `const`-ness nor a
byte.

**Do**

```rust
use happenstance_core::SequencePosition;
# fn pos(v: u64) -> SequencePosition {
#     match SequencePosition::new(v) { Some(p) => p, None => SequencePosition::FIRST }
# }
assert_eq!(pos(u64::MAX).next(), None, "there is no position above the last one");
assert_eq!(SequencePosition::FIRST.next(), SequencePosition::new(2));
```

**Not** — compiles, and the assertion states the bug rather than catching it:

```rust
use core::num::NonZeroU64;

#[derive(Debug, PartialEq)]
struct Position(NonZeroU64);

impl Position {
    fn new(value: u64) -> Option<Self> {
        NonZeroU64::new(value).map(Self)
    }
    fn next(&self) -> Option<Self> {
        Self::new(self.0.get().saturating_add(1))
    }
}

# let last = match Position::new(u64::MAX) { Some(p) => p, None => return };
assert_eq!(
    last.next(),
    Position::new(u64::MAX),
    "the resume idiom now re-reads one event forever",
);
```

**Rejects.** The one method whose documented purpose is signalling overflow
becomes incapable of it. Nothing shows until a store actually reaches the top of
the key space, and then a consumer's `checkpoint.next()` hands back the
checkpoint: the runner reprocesses the same event on every poll, at full rate,
while every metric it publishes says it is caught up and the store reports no
error at all.

**Evidence.** `crates/happenstance-core/src/event.rs:273 (Saturating made the one method)` ·
`crates/happenstance-core/src/event.rs:909 (position_next_signals_overflow)` ·
[SPECIFICATION VT-13](../../spec/SPECIFICATION.md)

## RS-10-5. Use `[u8; N]`, not an integer, for an opaque identifier a database will order.

**Why.** An array has no byte order to choose, and its `Ord` is lexicographic
over exactly the bytes an adapter stores, which is the order a `BLOB`/`BYTEA`
collation already applies. An integer's `Ord` is numeric, agrees with the stored
bytes under one endianness only, and invites arithmetic on a value that is opaque
by construction.

**Do**

```rust
use happenstance_core::StoreId;

let low = StoreId::from_bytes([0x00; 16]);
let mut first_byte = [0x00; 16];
first_byte[0] = 0x01;
let mut last_byte = [0x00; 16];
last_byte[15] = 0x01;

// Lexicographic over the stored bytes, so the database agrees with `Ord`.
assert!(low < StoreId::from_bytes(last_byte));
assert!(StoreId::from_bytes(last_byte) < StoreId::from_bytes(first_byte));
```

**Not** — compiles; the two assertions are the same pair of values in two
different orders:

```rust
let a: u128 = 1;
let b: u128 = 1 << 120;

assert!(a < b, "numeric order, which `Ord` on `u128` gives you");
assert!(b.to_le_bytes() < a.to_le_bytes(), "the order the BYTEA column sorts by");
```

**Rejects.** Two adapters persisting the same identifier, one reaching for
`to_be_bytes` and one for `to_le_bytes`. Both round-trip perfectly and both pass
every in-process rule, because neither ever reads the other's bytes. The
disagreement appears only when a replication hub compares identities as stored
bytes and concludes that one incarnation of one store is two different stores,
after which deduplication stops working in one direction and nothing raises an
error.

**Evidence.** `crates/happenstance-core/src/identity.rs:34 (has a byte order and an array does not)` ·
`crates/happenstance-core/src/identity.rs:392 (store_id_orders_lexicographically_over_its_bytes)` ·
[SPECIFICATION VT-6](../../spec/SPECIFICATION.md)
