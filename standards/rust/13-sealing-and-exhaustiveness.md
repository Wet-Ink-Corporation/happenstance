# 13 — Sealing and exhaustiveness

> **Load when:** adding a public struct or enum that will grow · E0603, E0616,
> E0638, E0639 or E0004 in an adapter crate but not in `happenstance-core` ·
> deciding between a private field and `#[non_exhaustive]` · returning several
> values from one method
> **See also:** 10 (newtypes) · 12 (the impls a sealed type still owes) · 40
> (public surface and evolution) · 30 (error taxonomy)

---

## RS-13-1. When a wrong *value* must be unreachable, make the field private.

**Why.** `#[non_exhaustive]` blocks the struct-literal expression, functional
record update included, and forces `..` in downstream patterns. It does nothing
to `value.field = x` on a value the caller already holds, so an invariant guarded
only by the attribute is not guarded at all.

**Do**

```rust
use happenstance_core::{AppendCondition, Query};

// `guards` is private: an accessor to read it, and no expression that writes it.
let condition = AppendCondition::new(Query::all());
assert_eq!(condition.guards().len(), 1);
```

**Not** — compiles. `Guard` carries `#[non_exhaustive]` and its public field is
assignable anyway; the assertion is the attribute failing to seal:

```rust
use happenstance_core::{AppendCondition, Guard, Query, SequencePosition};

let condition = AppendCondition::new(Query::all());
let mut guard: Guard = condition.guards()[0].clone();
guard.after = SequencePosition::new(9_000);

assert_eq!(guard.after, SequencePosition::new(9_000), "the attribute sealed nothing here");
```

**Rejects.** `AppendCondition.guards` made public on the reasoning that
`#[non_exhaustive]` is already on the struct. Two lines of safe downstream code —
`let mut c = AppendCondition::new(q); c.guards = Box::new([]);` — then build a
condition with no guards, which nothing can ever violate. That is a conditional
append that is silently unconditional: a lost update, with no error at the store,
no error at the caller and nothing in any log to find it by.

**Evidence.** `crates/happenstance-core/src/append.rs:106 (blocks the struct-literal)` ·
`crates/happenstance-core/src/append.rs:118 (guards: Box<[Guard]>)` ·
[SPECIFICATION VT-30](../../spec/SPECIFICATION.md) ·
[reference `non_exhaustive`](https://doc.rust-lang.org/reference/attributes/type_system.html) *(checked 2026-08-09, rustc 1.97.1)*

## RS-13-2. Match a `#[non_exhaustive]` tuple variant as `Variant { .. }`, never `Variant(..)`.

**Why.** Downstream, the variant's tuple constructor is not visible, and rustc
rejects the tuple form in *pattern* position too — `error[E0603]: tuple variant
is private`, with a note pointing at the attribute rather than at any visibility
modifier. The trailing `..` that makes a `#[non_exhaustive]` *struct* pattern
legal does not rescue this one: `Query::Items(items, ..)` is the same error.
Inside the defining crate the attribute is inert, so that crate's own tests
cannot see either.

**Do**

```rust
use happenstance_core::{Query, QueryItem};

let query = Query::from_item(QueryItem::of_types(["CourseDefined"])?);

assert!(matches!(query, Query::Items { .. }));
// Or skip the pattern: the accessor is what the crate actually offers.
assert_eq!(query.items().map(<[_]>::len), Some(1));
# Ok::<(), happenstance_core::InvalidQuery>(())
```

**Not** — `error[E0603]`, and note that this is a read, not a construction:

```rust,compile_fail,E0603
use happenstance_core::Query;
# let query = Query::all();
let count = match query {
    Query::All => 0,
    Query::Items(items, ..) => items.len(),
};
# let _ = count;
```

**Rejects.** An adapter's query push-down written as
`match query { Query::Items(items) => … }`, copied out of `happenstance-core`'s
own unit tests where `#[non_exhaustive]` has no effect. It reads as correct in
review, fails only once the adapter crate is compiled, and the diagnostic says
"private tuple variant" — which sends the author hunting for a missing `pub` in
the contract crate instead of for the attribute that is doing exactly its job.

**Evidence.** `crates/happenstance-core/src/query.rs:156 (so that no downstream crate can build one directly)` ·
`crates/happenstance-core/src/query.rs:194 (pub fn items)` ·
[SPECIFICATION VT-26](../../spec/SPECIFICATION.md)

## RS-13-3. Reach for `#[non_exhaustive]` with public fields when a value must be readable but not fabricable.

**Why.** Downstream, `Guard { query, .. }` in a pattern and `guard.after` as a
read both compile, while `Guard { query, after }` as an expression is
`error[E0639]`. That combination is exactly "anyone may inspect one, nobody
outside may mint one", and no other shape gives both — private fields would cost
the pattern, and no attribute at all would cost the seal.

**Do**

```rust
use happenstance_core::{AppendCondition, Guard, Query, QueryItem, SequencePosition};

let condition = AppendCondition::new(Query::from_item(QueryItem::of_types(["A"])?))
    .after(SequencePosition::FIRST);

// What a replication hub does to a peer-supplied condition before refusing it.
let Guard { query, .. } = &condition.guards()[0];
assert!(!query.is_all());
assert_eq!(condition.guards()[0].after, Some(SequencePosition::FIRST));
# Ok::<(), happenstance_core::InvalidQuery>(())
```

**Not** — `error[E0639]`: the struct expression is precisely what the attribute
removes.

```rust,compile_fail,E0639
use happenstance_core::{Guard, Query};

let guard = Guard { query: Query::all(), after: None };
# let _ = guard;
```

**Rejects.** Dropping `#[non_exhaustive]` from `Guard` so that a peer runner can
build one directly. It then mints a guard whose `after` names a position no read
ever returned; the store finds nothing above that position, accepts the append,
and the write lands on a decision model nothing ever justified. From every log it
is a successful conditional write, which is the one failure mode DCB exists to
make impossible.

**Evidence.** `crates/happenstance-core/src/append.rs:124 (deliberate asymmetry)` ·
`crates/happenstance-core/src/append.rs:128 (as a read both compile)` ·
[SPECIFICATION VT-30](../../spec/SPECIFICATION.md)

## RS-13-4. Return a named `#[non_exhaustive]` struct from a decomposing method, never a tuple.

**Why.** A tuple's arity is public API: every `let (a, b, c) = …` downstream is
`error[E0308]` the day a fourth part exists. A `#[non_exhaustive]` struct makes
`..` mandatory downstream (`error[E0638]` otherwise), which is what makes the
same addition source-compatible.

**Do**

```rust
use happenstance_core::{Event, EventParts};

let parts: EventParts = Event::new("A", &b"{}"[..])?.into_parts();

// `..` is compulsory downstream, and that is what makes a fifth part additive.
let EventParts { event_type, data, .. } = parts;
assert_eq!(event_type.as_str(), "A");
assert_eq!(data.len(), 2);
# Ok::<(), happenstance_core::InvalidEventType>(())
```

**Not** — the tuple, one part later: `error[E0308]` at every destructure that was
correct yesterday.

```rust,compile_fail,E0308
# struct EventType; struct Payload; struct Tags; struct Metadata;
# fn into_parts() -> (EventType, Payload, Tags, Metadata) {
#     (EventType, Payload, Tags, Metadata)
# }
let (event_type, data, tags) = into_parts();
# let _ = (event_type, data, tags);
```

**Rejects.** An adapter write path that destructures the tuple to avoid cloning
the payload. It compiles today; the commit adding a fifth part to `EventParts`
breaks it in every adapter crate at once, none of which is the crate being
changed. Each fix is mechanical and none is the author's, which is exactly the
pressure that gets the new part bolted on as a side-channel `Option` instead of
landing where it belongs.

**Evidence.** `crates/happenstance-core/src/event.rs:406 (the *number* of an event's)` ·
`crates/happenstance-core/src/event.rs:423 (so a later part is additive)` ·
[SPECIFICATION VT-4](../../spec/SPECIFICATION.md)

## RS-13-5. Do not put `#[non_exhaustive]` on an enum designed not to grow.

**Why.** It costs downstream a `_ =>` arm forever (`error[E0004]` without one),
so the compiler permanently stops reporting the variant they forgot. The
attribute is inert inside the defining crate, so that crate's own tests never pay
the cost every consumer does — which is what makes "add it everywhere for
consistency" look free.

**Do**

```rust
use happenstance_testkit::RuleOutcome;

// No `#[non_exhaustive]`, deliberately: the day a variant is added this match
// stops compiling, which is the report the author of that variant wants.
let outcome = RuleOutcome::Ran;
let label = match outcome {
    RuleOutcome::Ran => "ran",
    RuleOutcome::Skipped { .. } => "skipped",
};
assert_eq!(label, "ran");
```

**Not** — `error[E0004]`: every named variant is matched and a wildcard is still
required, because `StoreLimit` is `#[non_exhaustive]` and *will* grow.

```rust,compile_fail,E0004
use happenstance_core::StoreLimit;
# let limit = StoreLimit::EventDataLen;
let name = match limit {
    StoreLimit::EventDataLen => "data",
    StoreLimit::TagsPerEvent => "tags",
    StoreLimit::EventsPerBatch => "batch",
};
# let _ = name;
```

**Rejects.** `RuleOutcome` given `#[non_exhaustive]` to match the rest of the
crate. Every emitter downstream then carries a `_ => {}` arm to compile at all,
and the day a `Failed` variant lands those emitters build unchanged and report
nothing for it — the failure is swallowed by the arm the attribute forced them to
write, and the adapter's CI is green on a suite that found a defect.

**Evidence.** `crates/happenstance-testkit/src/contract.rs:727 (Deliberately exhaustive)` ·
`crates/happenstance-core/src/limits.rs:53 (non_exhaustive)` ·
[SPECIFICATION VT-25](../../spec/SPECIFICATION.md) ·
[SPECIFICATION CF-18](../../spec/SPECIFICATION.md) ·
[cargo SemVer: enum variants](https://doc.rust-lang.org/cargo/reference/semver.html) *(checked 2026-08-09, rustc 1.97.1)*
