# 12 — Manual impls and derive traps

> **Load when:** adding a field to a type that derives `Eq`/`Hash` · a `HashMap`
> lookup returns `None` for a key it holds · writing `Borrow<str>` · E0117 or
> E0271 on a conversion bound · a payload appeared in a log
> **See also:** 10 (the newtype these impls hang off) · 13 (sealing) · 30 (error
> taxonomy) · 40 (public surface and evolution)

---

## RS-12-1. Hand-write every impl a `Borrow<str>` probe travels through: `Eq`, `Hash` and `Ord`.

**Why.** `HashMap::get(&str)` hashes the `str` and then compares through
`Borrow`; `BTreeMap::get(&str)` walks a tree ordered by `K: Ord` while comparing
with `str`'s. One `Borrow<str>` impl therefore signs the type up for three impls
that must each agree with the borrowed form, and a derive computes over *every*
field — so it discharges all three on a one-field newtype and stops discharging
them the moment a second field lands.

**Do**

```rust
use happenstance_core::EventType;
use std::collections::{BTreeMap, HashMap};

let mut hashed = HashMap::new();
hashed.insert(EventType::from_static("CourseDefined"), "decode");
let mut ordered = BTreeMap::new();
ordered.insert(EventType::from_static("CourseDefined"), "decode");

// One `&str` probe, two containers, three impls; no allocation, no re-validation.
assert_eq!(hashed.get("CourseDefined"), Some(&"decode"));
assert_eq!(ordered.get("CourseDefined"), Some(&"decode"));
```

**Not** — compiles; the assertions are the bug, and they are the *only* place it
is visible:

```rust
use std::borrow::Borrow;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

// The derive spans `arity` *and* `name`; `borrow` yields only `name`.
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Kind {
    arity: u8,
    name: String,
}

impl Borrow<str> for Kind {
    fn borrow(&self) -> &str {
        &self.name
    }
}

// What `Hash` is *fed*, rather than what it returns. The disagreement between the
// owned key and its borrowed probe is the bug, and this is the one face of it no
// hash seed can flip.
#[derive(Default)]
struct Fed(Vec<u8>);

impl Hasher for Fed {
    fn write(&mut self, bytes: &[u8]) {
        self.0.extend_from_slice(bytes);
    }
    fn finish(&self) -> u64 {
        0
    }
}

fn fed<T: Hash + ?Sized>(value: &T) -> Vec<u8> {
    let mut hasher = Fed::default();
    value.hash(&mut hasher);
    hasher.0
}

// rs_12_1_counterexample_must_hold_for_every_seed: `RandomState` is drawn afresh
// per `HashMap`, so one map is one sample and a counterexample asserted on one
// sample is a coin toss, not evidence. `registry.get("CourseDefined") == None` is
// exactly that coin toss — `get` picks a bucket by hash and then compares by
// equality *within* it, so on the seeds where the borrowed probe lands in the
// owned key's bucket `borrow` returns `"CourseDefined"`, the comparison succeeds,
// and the entry is found: 1,619 of 200,000 seeds measured, one gate run in 126.
for _ in 0..4096 {
    let mut registry: HashMap<Kind, &str> = HashMap::new();
    registry.insert(Kind { arity: 3, name: "CourseDefined".into() }, "decode");

    let owned = Kind { arity: 3, name: "CourseDefined".into() };
    assert_eq!(registry.get(&owned), Some(&"decode"), "the entry is present");
    assert_ne!(
        fed(&owned),
        fed(<Kind as Borrow<str>>::borrow(&owned)),
        "and unreachable: `Borrow` requires the owned and borrowed forms to hash alike"
    );
}

// `Ord` breaks the same way, and that is the ordering a `BTreeMap` searches by.
let (a, b) = (Kind { arity: 2, name: "aa".into() }, Kind { arity: 1, name: "zz".into() });
assert!(a > b, "the derive orders by `arity` first");
assert!(<Kind as Borrow<str>>::borrow(&a) < <Kind as Borrow<str>>::borrow(&b));
```

**Rejects.** A codec registry keyed by event type that answers `None` for a type
it is holding. The decode path reads the miss as "unknown event type" and skips
the event, so a projection completes cleanly having ignored an entire class of
history. No test written the obvious way can see it, because inserting and
reading with the same key *shape* always agrees with itself — catching it needs
an owned key and a borrowed probe, which is why the counterexample above is
written that way and not as a round trip.

**Evidence.** `crates/happenstance-core/src/event.rs:136 (promises the borrowed form hashes)` ·
`crates/happenstance-core/src/event.rs:1007 (a_map_keyed_by_event_type_is_probed_by_str)` ·
[SPECIFICATION VT-32](../../spec/SPECIFICATION.md) ·
[SPECIFICATION VT-33](../../spec/SPECIFICATION.md) ·
[std `Borrow`](https://doc.rust-lang.org/std/borrow/trait.Borrow.html) *(checked 2026-08-09, rustc 1.97.1)*

## RS-12-2. Canonicalise in the constructor, then the derived `PartialEq`/`Hash` mean set equality.

**Why.** A derive over a `Box<[T]>` is positional. `Tags` sorts and deduplicates
in `FromIterator`, so no unsorted value can exist, and the positional derive
*is* set equality for every value that can — plus `contains_all` becomes one
`O(n + m)` merge scan instead of a nested loop.

**Do**

```rust
use happenstance_core::Tags;

let a = Tags::from_pairs([("student", "s1"), ("course", "c1")])?;
let b = Tags::from_pairs([("course", "c1"), ("student", "s1")])?;

assert_eq!(a, b, "one set, one value, whatever order it was built in");
assert!(a.contains_all(&Tags::from_pairs([("course", "c1")])?));
# Ok::<(), happenstance_core::InvalidTag>(())
```

**Not** — compiles; the assertion is what an uncanonicalised set gets you:

```rust
use happenstance_core::Tag;

#[derive(Debug, PartialEq, Eq, Hash)]
struct Labels(Vec<Tag>);

let a = Labels(vec![Tag::from_static("student:s1"), Tag::from_static("course:c1")]);
let b = Labels(vec![Tag::from_static("course:c1"), Tag::from_static("student:s1")]);

assert_ne!(a, b, "one set, two values — and `Hash` disagrees the same way");
```

**Rejects.** Any later set-shaped value — a peer's declared origin set, a
projection's tag filter — built as a plain `Vec` wrapper with a derive. Two
writers assembling the same set in different orders produce values that compare
unequal, so a replication hub's deduplication keeps both and re-evaluates a
condition it already refused, and any `HashMap` keyed on the type misses on a
differently-ordered probe. A single-writer test produces one order and never
sees it.

**Evidence.** `crates/happenstance-core/src/tag.rs:248 (Canonicalisation is enforced at construction)` ·
`crates/happenstance-core/src/tag.rs:481 (fn from_iter)` ·
[SPECIFICATION VT-16](../../spec/SPECIFICATION.md)

## RS-12-3. Grow a store's capability with a new trait in the crate that needs it.

**Why.** The orphan rule admits an impl from the crate defining the trait or the
crate defining the type, and from nobody else. Read backwards that is a growth
mechanism: a downstream crate can give a foreign store a new capability without
the port crate changing a line — and a *third* crate cannot supply that impl on
an adapter's behalf, so no blanket impl in a conformance suite can make it free.

**Do**

```rust
use happenstance_core::Event;

// Local trait, foreign type: the half of the orphan rule a port crate grows by.
trait Replicable {
    fn origin_hint(&self) -> Option<&str>;
}

impl Replicable for Event {
    fn origin_hint(&self) -> Option<&str> {
        None
    }
}

assert!(Event::new("A", &b""[..])?.origin_hint().is_none());
# Ok::<(), happenstance_core::InvalidEventType>(())
```

**Not** — foreign trait, foreign type, a third crate: `error[E0117]`.

```rust,compile_fail,E0117
impl Default for happenstance_core::StoreLimit {
    fn default() -> Self {
        happenstance_core::StoreLimit::EventDataLen
    }
}
```

**Rejects.** A plan to put `impl<S: EventStore> IngestStore for S` in
`happenstance-sync-testkit` — a crate that will define neither the trait nor any
store — so that every adapter acquires replication for nothing. It reviews as a
one-line convenience and no crate in the workspace can compile it, so the
`error[E0117]` arrives against a design already committed to; a blanket impl that
*could* be placed also forecloses every adapter's own, because coherence forbids
the overlap and there is no opt-out.

**Evidence.** `crates/happenstance-sync/src/ingest.rs:14 (The escape is **coherence**)` ·
`crates/happenstance-sync/src/ingest.rs:104 (compile_fail,E0117)` ·
[SPECIFICATION VT-10](../../spec/SPECIFICATION.md) ·
[reference orphan rules](https://doc.rust-lang.org/reference/items/implementations.html#orphan-rules) *(checked 2026-08-09, rustc 1.97.1)*

## RS-12-4. Take `T: TryInto<X>` plus `E: From<T::Error>`, never `TryInto<X, Error = E>`.

**Why.** The `Error = E` form is an equality constraint, and the identity
conversion `X: TryInto<X>` has `Error = Infallible` — so passing an
already-built `X` is `error[E0271]`, expected `E`, found `Infallible`. The
looser pair accepts both, paid for by an `impl From<Infallible> for E` whose
body is `match never {}`: an empty match on an uninhabited type is exhaustive.

**Do**

```rust
use happenstance_core::{Event, EventType};

const COURSE_DEFINED: EventType = EventType::from_static("CourseDefined");

// One bound, two callers: a literal validated here, and an interned
// `EventType` that is not validated a second time.
let from_str = Event::new("CourseDefined", &b"{}"[..])?;
let from_const = Event::new(COURSE_DEFINED, &b"{}"[..])?;

assert_eq!(from_str.event_type(), from_const.event_type());
# Ok::<(), happenstance_core::InvalidEventType>(())
```

**Not** — `error[E0271]` at the interned call site, and only there:

```rust,compile_fail,E0271
use happenstance_core::{EventType, InvalidEventType};

fn make<T>(value: T) -> Result<EventType, InvalidEventType>
where
    T: TryInto<EventType, Error = InvalidEventType>,
{
    value.try_into()
}

const COURSE_DEFINED: EventType = EventType::from_static("CourseDefined");
let _ = make(COURSE_DEFINED);
```

**Rejects.** A codec registry that interns one `EventType` per domain event at
start-up and then cannot hand it to the constructor. `E0271` is reported at the
*call site*, naming `Infallible` against the caller's own error type, so it reads
as "this `const` is the wrong type" rather than as a bound that is one character
too strong — and the fix everyone reaches for is passing the `&str` again, which
compiles, reviews as an ordinary call, and re-runs the whole validating byte walk
on every append for a value that cannot have changed.

**Evidence.** `crates/happenstance-core/src/event.rs:353 (Deliberately not)` ·
`crates/happenstance-core/src/error.rs:81 (accepts an empty match on an uninhabited)` ·
`crates/happenstance-core/src/error.rs:85 (match never {})` ·
[SPECIFICATION VT-18](../../spec/SPECIFICATION.md)

## RS-12-5. Treat `Debug` as API: render a payload's size, never its bytes.

**Why.** `missing_debug_implementations` is a workspace warning and the gate runs
`-D warnings`, so every public type acquires a `Debug` whether or not anyone
chose one — and the derived impl over an opaque payload prints every byte into
every assertion failure and every log line. A hand-written impl with a private
renderer keeps the field and drops the contents.

**Do**

```rust
use happenstance_core::Event;

let event = Event::new("SeatMapPublished", &b"authorization: hunter2"[..])?;
let rendered = format!("{event:?}");

assert!(rendered.contains("<22 bytes>"), "{rendered}");
assert!(!rendered.contains("hunter2"));
# Ok::<(), happenstance_core::InvalidEventType>(())
```

**Not** — compiles; the assertion is the payload, now in the log:

```rust
#[derive(Debug)]
struct Envelope {
    data: Vec<u8>,
}

let rendered = format!("{:?}", Envelope { data: b"hunter2".to_vec() });
assert!(rendered.contains("104, 117, 110"), "{rendered}");
```

**Rejects.** A `SequencedEvent` printed by a conformance-rule failure or carried
as a `tracing` field. The assertion message becomes a page of decimal integers
that nobody reads, and any payload holding a bearer token or personal data is now
in the CI log of every adapter that runs the suite — retained for as long as the
CI provider keeps logs, in a repository whose whole design premise is that the
contract layer never looks inside a payload.

**Evidence.** `crates/happenstance-core/src/event.rs:447 (Payloads are frequently large and rarely UTF-8)` ·
`crates/happenstance-core/src/event.rs:450 (struct ByteLen)` ·
`Cargo.toml:162 (missing_debug_implementations)`
