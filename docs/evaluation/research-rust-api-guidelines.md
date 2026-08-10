# happenstance — idiomatic Rust API audit

**Lens:** Rust API Guidelines checklist + 2024-edition idiom
**Scope read:** every file under `crates/happenstance/src/` and `crates/happenstance-testkit/src/`,
plus `.kb/decision/0001`, `docs/RUNBOOK.md`, `examples/course-subscriptions/src/main.rs`, all manifests.
**Date:** 2026-08-05. **Everything below was compiled**, not reasoned about — see "Probe log".

---

## Headline

This is a well-built contract crate. The parts most libraries get wrong — error taxonomy,
newtype validation, niche optimisation, `#[non_exhaustive]` placement, doc quality — are right,
and several decisions (`Query` as an enum, `SequencePosition` as `NonZeroU64`, `AppendError<E>`
lifting the concurrency signal out of the adapter error) are better than the closest Rust prior
art. The findings are concentrated in three places:

1. **The `append` signature makes every adapter clone** and makes `Event::into_parts` — a method
   that exists specifically to prevent that clone — unreachable through the port.
2. **The port has no extensibility strategy.** Every future convenience is a breaking change to
   every adapter, because there is no extension trait and no provided methods.
3. **`Tag`/`EventType` being `Box<str>` forecloses const construction**, so every string literal
   in every event allocates and returns a `Result` the caller must `?`. Five `?` before a
   single-event append in the worked example.

And one verified-negative research result: **RTN will not save you.** Details in §1.

---

## 1. The two-trait `trait_variant` design — verdict: keep it, and add the test that guards it

**Is RTN a way out? No, and not soon.**

- Tracking issue [rust-lang/rust#109417](https://github.com/rust-lang/rust/issues/109417) is still
  `S-tracking-impl-incomplete`.
- The stabilisation PR [rust-lang/rust#138424](https://github.com/rust-lang/rust/pull/138424)
  (opened 2025-03-12, would have stabilised `where T: Trait<method(..): Send>` in where-clause and
  item-bound position) was **closed unmerged on 2025-12-27**. The stated reason is not technical —
  the author wrote "I'm not working on Rust anymore." The lang team had already resolved the one
  substantive blocker (accidental partial-TAIT stabilisation) *in favour* of shipping, on
  2025-04-02. So the feature is design-approved, implemented, and orphaned.
- RTN still appears in the [April 2026 project-goals
  update](https://blog.rust-lang.org/2026/05/18/project-goals-2026-04/) only as an unstabilised
  blocker for Rust-for-Linux. No successor stabilisation PR.

There is no plausible path to RTN on a 1.85 MSRV, and none on *any* stable toolchain within this
project's 0.1 horizon. **Do not design around it.**

`trait-variant` itself is healthy and is the right dependency: it lives in the `rust-lang` GitHub
org (`rust-lang/impl-trait-utils`, alongside `dynosaur`), and **0.1.3 shipped 2026-07-22** — two
weeks ago, after a two-year gap on 0.1.2. C-STABLE is satisfied: a rust-lang-org crate is as stable
a public dependency as a non-std crate gets.

**Verified property that the design depends on, and that nothing currently tests.** The claim in
ADR-0001 is that binding the weak trait costs nothing. I checked what that actually means:

| Call site | Result |
|---|---|
| `fn f<S: EventStore + Send + Sync + 'static>(s: Arc<S>)` that `tokio::spawn`s `read_decision_model` | **E0277** — `impl Stream<Item = Result<SequencedEvent, <S as EventStore>::Error>>` is not `Send` |
| same, but `S: SendEventStore` | **compiles** — and it still calls the `EventStore`-bound `read_decision_model` |
| concrete `MemoryEventStore`, no generics | compiles either way (auto-trait leakage) |

So the guidance is correct *and* under-explained. What actually happens: `trait_variant`'s blanket
impl `impl<T: SendEventStore> EventStore for T` forwards `read` to the `Send` variant, and the
RPITIT's hidden type leaks its `Send`-ness back through. That is why the `EventStore`-bound free
functions stay usable from `tokio::spawn` when the caller's own bound is `SendEventStore`.

That leakage is load-bearing and fragile — it is a property of how `trait_variant` writes its
blanket impl, not something the language guarantees you. And the one test that claims to guard the
design, `memory.rs:328` `read_stream_is_send`, tests a **concrete** store, which passes by
auto-trait leakage even if the generic property were broken. See finding `send-composition-untested`.

**The E0034 tax.** Real but small, and it is mitigable without RTN: ship a `happenstance::prelude`
that re-exports exactly one flavour (`EventStore`) plus the free functions, so the ambiguity is
only reachable by someone who deliberately imports both. Right now `lib.rs:97` re-exports both at
the root, so `use happenstance::*` is the ambiguous case.

**Alternatives rejected:** hand-writing two traits (doubles the surface — the `umadb-dcb` approach,
which also pays `#[async_trait]`'s `Pin<Box<dyn Future>>` allocation on every call); `#[async_trait]`
(kills wasm32); erasing to `Pin<Box<dyn Stream + Send>>` in the port (allocates per read, and the
`!Send` flavour can't use it).

---

## 2. `append` takes `&[Event]` — the highest-leverage signature change

```rust
// store.rs:141
async fn append(
    &self,
    events: &[Event],
    condition: Option<&AppendCondition>,
) -> Result<SequencePosition, AppendError<Self::Error>>;
```

```rust
// memory.rs:219 — the consequence, in the reference implementation
stored.extend(events.iter().enumerate().map(|(offset, event)| {
    SequencedEvent::new(position_at(first_index + offset), event.clone())
}));
```

`Event::clone()` is not cheap. `Bytes` is a refcount bump, but `EventType(Box<str>)` is one
allocation and `Tags(Box<[Tag]>)` is `n + 1` allocations because each `Tag` is its own `Box<str>`.
A three-tag event costs **five allocations per event, per append**, in every adapter, forever.

The crate already knows this is wrong. `event.rs:243`:

```rust
/// Decomposes the event, avoiding a clone in adapter write paths.
pub fn into_parts(self) -> (EventType, Bytes, Tags, Option<Bytes>) {
```

`into_parts` takes `self` by value. Through the port, an adapter never has an owned `Event`.
The method is unreachable from the only place it was written for.

**Recommendation: `events: Vec<Event>`.**

Alternatives, and why they lose:

- **`impl IntoIterator<Item = Event>`** — the most flexible, and my first instinct. Rejected on a
  specific ground: it makes `append` a *generic method*, and `dynosaur` — which ADR-0001 names as
  the erasure escape hatch for the missing `dyn EventStore` — cannot erase generic methods. Taking
  `impl IntoIterator` closes the only door ADR-0001 left open. It also gives every adapter author a
  harder signature to write.
- **`&mut dyn Iterator<Item = Event>`** — dyn-compatible and avoids the clone, but it is hostile to
  write and to call, and it prevents an adapter from knowing the batch length up front (which a SQL
  adapter wants for a multi-row `INSERT`).
- **Keep `&[Event]`** — the argument for it is that a `ConditionViolated` retry can re-submit the
  same slice. That argument is wrong on DCB grounds: after a violation you *must* re-read and
  re-decide, so re-submitting the identical batch is the bug, not the feature. Consuming the `Vec`
  makes the correct thing the easy thing.
- Prior art agrees: `umadb-dcb`'s `DcbEventStoreAsync::append` takes `events: Vec<DcbEvent>`.

Take `condition: Option<AppendCondition>` by value at the same time, for symmetry and because
`AppendCondition` is small and the caller almost always builds it at the call site. Lower stakes —
`&AppendCondition` costs nothing today because adapters only read it. Judgment call; I would change
it for consistency.

---

## 3. `AppendError::NoEvents` — an illegal state that is representable, and inconsistently reported

Two findings in one place.

**(a) It contradicts the crate's own stated design principle.** `lib.rs:35` says "Illegal states are
unrepresentable," and justifies `Query` being an enum on exactly that ground. But `error.rs:161`
carries `NoEvents`, and `store.rs:143` accepts `&[]`. The empty batch is representable, so every
adapter must check for it, the conformance suite must have a rule for it, and the failure is
runtime rather than compile-time — the one case where the crate does the opposite of what it says.

**(b) The precedence is unspecified and the reference store already disagrees with what a SQL
adapter will naturally do.** `memory.rs:197-216` checks the condition *first* and the empty batch
*second*. Verified:

```
empty batch + violated condition -> ConditionViolated(ConditionViolated { conflicting_position: Some(1) })
```

A SQLite adapter that early-returns on `events.is_empty()` before opening `BEGIN IMMEDIATE` — the
obvious implementation — returns `NoEvents`. Both pass the suite, because `suite.rs:408` only tests
`store.append(&[], None)` with no condition. Two conformant adapters, two different errors.

**Recommendation.** Introduce a non-empty newtype and delete the variant:

```rust
/// A non-empty batch. The specification defines `Events` as a non-empty
/// collection; this makes that a type, not a runtime check.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EventBatch(Vec<Event>);

impl EventBatch {
    pub fn new(events: Vec<Event>) -> Result<Self, EmptyBatch>;
    pub fn of(event: Event) -> Self;            // infallible, the common case
    pub fn into_vec(self) -> Vec<Event>;
}
impl From<Event> for EventBatch { .. }
impl TryFrom<Vec<Event>> for EventBatch { type Error = EmptyBatch; .. }
impl<'a> IntoIterator for &'a EventBatch { .. }
impl IntoIterator for EventBatch { .. }

async fn append(&self, events: EventBatch, condition: Option<AppendCondition>)
    -> Result<SequencePosition, AppendError<Self::Error>>;
```

`AppendError` then has exactly two variants, both of which are genuine *outcomes* rather than
caller bugs — which is what `AppendError` is documented to be (`error.rs:122-129`). The conformance
rule `append_rejects_empty_batch` is deleted, because it becomes a compile error. The precedence
question evaporates.

**Cost, stated honestly:** one more type to learn, and `store.append(vec![e], None)` becomes
`store.append(e.into(), None)` or `store.append(EventBatch::of(e), None)`. That is the entire
downside. Given the crate's own philosophy, it is worth it.

**Alternative rejected:** keep `Vec<Event>` and `NoEvents`, and just add a conformance rule pinning
the precedence (empty-check first). That is the cheap fix and it is *sufficient* — if you don't
want the newtype, do at least this, or the two adapters really will disagree.

---

## 4. `AppendError<E>` generic vs boxed — the generic is right, keep it

Asked directly, so answered directly. Keep `AppendError<E>`.

- It preserves the adapter's concrete error, so `match err { AppendError::Store(rusqlite::Error::SqliteFailure(..)) => ..}`
  works without `downcast_ref`.
- It costs no allocation on a path (`SQLITE_BUSY` under contention) that is *not* rare.
- `map_store` (`error.rs:178`) already gives the erasure escape hatch for a layer that wants one.
- Boxing to `Box<dyn Error + Send + Sync>` would erase the type, allocate, and force a `Send + Sync`
  bound that the wasm32 flavour cannot always satisfy.

One gap: `EventStore::Error` is bounded `core::error::Error + 'static` (`store.rs:99`) with no
`Send + Sync`. That is correct for the `!Send` flavour, but it means `AppendError<S::Error>` is not
convertible into `anyhow::Error` (which needs `Error + Send + Sync + 'static`) for an arbitrary
native adapter — the worked example only gets away with it because `MemoryStoreError` happens to be
`Send + Sync`. Document the expectation ("native adapters SHOULD make `Error: Send + Sync +
'static`") and add a `happenstance_testkit::assert_send_store!(MyStore)` macro so it is checked.
Do **not** put `Send + Sync` on the associated type — that would re-break wasm32.

---

## 5. Naming and conventions (C-CASE, C-CONV, C-GETTER, C-ITER)

**`Event::data() -> &Bytes` is right.** C-GETTER (no `get_` prefix) is satisfied. `&Bytes` rather
than `&[u8]` is the correct call: it lets a caller `.clone()` in O(1) to hand the payload onward,
which is exactly what a replication path does, and `Bytes: AsRef<[u8]>` means `&[u8]` is one method
call away. Returning `Bytes` by value would force a refcount bump on callers who only wanted to
peek. The one thing owed here is a docs statement that **`bytes` is a public dependency**: a
`bytes` 2.0 is a breaking change for `happenstance`. The re-export at `lib.rs:106` shows this was
thought about; say it out loud in the crate docs.

**`collect` is a poor free-function name.** `store.rs:162`. Every Rust programmer's first
association with `collect` is `Iterator::collect`, and this function is neither a method nor
inherent to event stores — its signature is `S: Stream<Item = Result<T, E>>` with nothing
happenstance-specific in it. The exact prior art is `futures::TryStreamExt::try_collect`, which has
identical semantics (stop at the first `Err`, discard what was collected). **Rename to
`try_collect`.** It is more accurate *and* it tells the reader about the error behaviour, which
`collect` actively hides.

**`read_decision_model` names a thing it does not return.** It returns
`(Vec<SequencedEvent>, Option<SequencePosition>)` — raw events, not a model. The tuple is a
C-CUSTOM-TYPE smell: two positional values a caller can transpose with no help from the compiler,
where the second is derivable from the first.

Replace with a named type that carries the one operation callers actually want next:

```rust
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct DecisionSlice {
    pub events: Vec<SequencedEvent>,
    pub last_position: Option<SequencePosition>,
}

impl DecisionSlice {
    /// The append condition that closes the boundary this slice was read under.
    pub fn close(&self, query: Query) -> AppendCondition {
        AppendCondition::new(query).after_opt(self.last_position)
    }
}
```

Then the worked example's `commit(store, &[event], &query, last_seen)` (which currently threads
three arguments to rebuild the condition by hand) collapses to `slice.close(query)`. That is the
DCB loop expressed once, in the library, instead of once per application.

**C-ITER:** `Tags::iter` exists and returns `core::slice::Iter<'_, Tag>` — correct, and returning
the concrete std type rather than `impl Iterator` is the right call for a stable crate (it is
nameable, `Clone`, `DoubleEnded`, `ExactSize`). No `iter_mut` and no owned `into_iter` — see §7.

---

## 6. Type safety: const-constructible newtypes (the biggest ergonomics win available)

Count the `?` for the simplest possible operation, from the real example
(`examples/course-subscriptions/src/main.rs:86-104`):

```rust
let query = Query::from_item(QueryItem::new(
    [COURSE_DEFINED],
    Tags::from_pairs([("course", course)])?,   // 1
)?)?;                                          // 2, 3

let event = Event::new(
    COURSE_DEFINED,
    format!("{{\"capacity\":{capacity}}}").into_bytes(),
)?                                             // 4
.with_tags(Tags::from_pairs([("course", course)])?);   // 5
```

Five `?` and three distinct error types (`InvalidTag`, `InvalidQuery`, `InvalidEventType`) to
append one event. The example escapes with `anyhow`, which library crates are not allowed to do —
so a downstream library author must write a three-arm `From` impl or use `Box<dyn Error>` before
they can write their first command handler.

The root cause is a type choice: `EventType(Box<str>)` (`event.rs:29`) and `Tag(Box<str>)`
(`tag.rs:39`) cannot be constructed in a const context, so validation *must* be deferred to runtime
and *must* return `Result`, even for the string literals that make up ~100% of real event types.

**Recommendation: `Cow<'static, str>` plus a `const fn from_static`.** Verified to compile on this
toolchain:

```rust
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EventType(Cow<'static, str>);

impl EventType {
    /// Validated at compile time. An invalid literal is a compile error, not a `Result`.
    pub const fn from_static(value: &'static str) -> Self {
        let b = value.as_bytes();
        assert!(!b.is_empty(), "event type must not be empty");
        assert!(b.len() <= MAX_EVENT_TYPE_LEN, "event type too long");
        let mut i = 0;
        while i < b.len() {
            assert!(b[i] >= 0x20 && b[i] != 0x7f, "control character in event type");
            i += 1;
        }
        Self(Cow::Borrowed(value))
    }
}

const COURSE_DEFINED: EventType = EventType::from_static("CourseDefined");
//  EventType::from_static("")  =>  error: evaluation of `BAD` failed inside this call
```

Why this is the right shape, for someone new to Rust:

- **`Cow<'static, str>` is a two-state enum**: `Borrowed(&'static str)` for literals (zero
  allocation, and it can be built in a `const`), `Owned(Box<str>)`/`String` for runtime strings. It
  keeps `EventType::new(String)` working exactly as today.
- **All the derives stay correct.** `Cow`'s `PartialEq`/`Ord`/`Hash` impls delegate to the target
  (`&**self`), so `Borrowed("A") == Owned("A")` and both hash identically. Verified.
- **Cost:** `size_of::<EventType>()` goes 16 → 24 bytes. `Event` is heap-indirected everywhere it
  matters; this is not a hot-loop concern.
- `const { EventType::from_static("X") }` works in expression position too, so it is not restricted
  to `const` items.

This wins less for `Tag`, because a tag's *value* half is usually runtime data —
`Tag::key_value("course", course)` cannot be const. That is fine; say so. The win is for
`EventType` and for the `Query`/`QueryItem` construction that consumes event types, which is where
three of the five `?` above come from.

**Alternatives rejected:**
- **A proc macro `event_type!("X")`** — validates at expansion time but still allocates at runtime,
  and adds a proc-macro dependency to the contract crate. Strictly worse than `const fn`, which
  needs no dependency at all.
- **`&'static str` only** — forbids runtime-derived event types, which replication ingest needs.
- **`new_unchecked`** — the workspace `forbid(unsafe_code)` makes this pointless; it would just be
  a footgun with no performance story.

---

## 7. Interoperability gaps (C-COMMON-TRAITS, C-CONV-TRAITS, C-COLLECT)

All verified by compilation, not by reading.

| Missing | Why it matters | Effort |
|---|---|---|
| `Borrow<str>` on `Tag`, `EventType` | `HashMap<EventType, Handler>::get("CourseDefined")` fails (E0308). A codec/handler registry keyed by event type is the *core data structure* of the typed layer ADR-0006 is about to build. `Hash`/`Eq`/`Ord` already agree with `str`'s, so the impl is sound. | trivial |
| `FromStr` on `Tag`, `EventType` | `"course:c1".parse::<Tag>()` fails. Needed for config files, `clap`, `serde_with`, CLI tools. `TryFrom<&str>` exists but `parse()` is what people type. | trivial |
| `IntoIterator for Tags` (owned) | `for t in tags` fails. Worse: `tags.into_iter()` **compiles and silently yields `&Tag`** by autoref to the `&Tags` impl — verified. That is the array-`into_iter` footgun, reproduced. | trivial |
| `Extend<Tag> for Tags` | C-COLLECT explicitly pairs it with `FromIterator`, which is implemented. Re-canonicalise on each call; document that it is O(n log n) per call so nobody loops it. | trivial |
| `Hash` on `Event`, `SequencedEvent`, `Query`, `QueryItem`, `AppendCondition` | `HashSet<Event>` fails (E0599). Replication's idempotent-ingest problem (runbook phase 6) is a dedup problem. `Tag`, `Tags`, `EventType`, `SequencePosition` all have it; these five are the inconsistency. | trivial |
| `AsRef<str>`, `From<&str>`, `From<String>`, `FromStr`, `Borrow<str>`, serde on `ProjectionId` | `ProjectionId` has none of what its five sibling newtypes have. It also has **no validation at all** (`projection.rs:46` is infallible) — `ProjectionId::new("")` is legal, and it becomes a primary key in a checkpoint table. | small |
| `PartialEq<str>` / `PartialEq<&str>` on `Tag`, `EventType` | `assert_eq!(tag, "course:c1")` in adapter tests. Convention in string newtypes (`camino::Utf8Path`). Nice-to-have. | trivial |

**Deliberately not recommended:** `Deref<Target = str>` on `Tag`/`EventType`. C-DEREF says only
smart pointers implement `Deref`, and it would make every `str` inherent method appear on the
newtype, which defeats the point of having a newtype (`tag.len()` silently meaning byte length, and
`tag.to_uppercase()` silently producing a `String` that is no longer a validated tag).
`AsRef<str>` + `Borrow<str>` + `as_str()` covers every legitimate use.

**`Query: Default` should be removed.** `query.rs:143-150` makes `Query::default() == Query::All`,
i.e. "read the entire log". C-COMMON-TRAITS asks for `Default` *where there is a sensible default*;
here the default is the most expensive and most dangerous value, and it will be reached silently by
`#[derive(Default)]` on any struct that embeds a `Query`. `Query::all()` is three more characters
and says what it means.

---

## 8. `ReadOptions` — public fields are fine; `limit(0)` is not

**On the public-fields-plus-`#[non_exhaustive]`-plus-builder question:** the combination is
coherent and I would keep it, but it has an undocumented consequence.

- `#[non_exhaustive]` blocks struct-literal construction downstream — verified:
  `ReadOptions { backwards: true, ..ReadOptions::new() }` is `E0639`. **Functional-update syntax is
  blocked too**, and that is the idiom every Rust programmer reaches for with an options struct.
  Nothing in the docs warns about this.
- Public fields are still readable (the doctest at `query.rs:221` does `assert!(options.backwards)`)
  and writable field-by-field (`opts.backwards = true;` compiles). So there are two blessed ways to
  set an option and one obvious way that fails.
- The fields need to be public: an adapter's `read` must destructure them to build SQL, and
  `#[non_exhaustive]` already prevents it from matching exhaustively. Making them private would buy
  three accessor methods and nothing else. **Keep as is, and add a doc note** explaining that
  `..Default::default()` will not compile and why.

**The real defect is `limit`:**

```rust
// query.rs:262
/// Reads at most `limit` events. A `limit` of zero is ignored, since
/// requesting nothing is never what the caller meant.
pub const fn limit(mut self, limit: usize) -> Self {
    self.limit = NonZeroUsize::new(limit);   // 0 -> None -> UNLIMITED
}
```

"Requesting nothing is never what the caller meant" is true for a literal `0` and false for a
computed one. A paging loop that writes `.limit(budget - fetched)` and reaches parity does not read
zero events — it reads **the entire log**. That is a silent unbounded read produced by an argument
the API chose to discard. C-VALIDATE says functions validate their arguments; silently reinterpreting
one as its opposite is the anti-pattern.

**Recommendation:** change the field to `limit: Option<usize>` and let `0` mean "return nothing",
matching SQL `LIMIT 0` and every other paging API in existence. Cost: `ReadOptions` grows 8 bytes
(it is `Copy` and passed by value; irrelevant). Add one conformance rule that `limit(0)` yields an
empty stream.

**Alternative rejected:** `limit(NonZeroUsize)`. It is type-safe but forces
`.limit(NonZeroUsize::new(50).expect("nonzero"))` at every call site, or a `nonzero!` macro, for a
value that is almost always a literal. The `NonZeroUsize` niche buys nothing here — unlike
`SequencePosition`, where `Option<SequencePosition>` appears in every `AppendCondition` and the
niche is load-bearing.

---

## 9. Validation: `char::is_control` is not "ASCII control characters"

`event.rs:46` and `tag.rs:56`:

```rust
if value.chars().any(char::is_control) {
```

`char::is_control` tests the Unicode `Cc` category: `U+0000–U+001F` **and `U+007F–U+009F`**. The doc
comments (`event.rs:37`, `tag.rs:47`) and both error variants (`error.rs:29`, `error.rs:53`) say
"ASCII control character". `U+0085` (NEXT LINE) and `U+0080–U+009F` are rejected and are not ASCII.
A one-word doc fix — but this is the doc comment on a *validation* function of a public type, which
is a contract, and the DCB specification imposes no character constraint at all, so this rule is
entirely happenstance's own and its exact shape is the only thing a caller has to go on.

**The larger question the code invites.** `Cc` is not obviously the right rejection set for a value
whose equality *is* the consistency mechanism. Not rejected today:

- Leading/trailing whitespace — `"course:c1"` and `"course:c1 "` are two different tags that render
  identically in a log.
- `Cf` format characters — `U+200B` zero-width space, `U+200D` ZWJ, `U+202E` right-to-left override.
  Two visually identical tags, one consistency boundary silently split in two. `U+202E` in an event
  type is also a log-spoofing vector.
- **Unicode normalisation.** `"café"` in NFC (`U+00E9`) and NFD (`U+0065 U+0301`) are different byte
  sequences, therefore different `Tag`s, therefore different consistency boundaries — and they are
  indistinguishable on screen. A Mac client (NFD-ish) and a Linux client (NFC) writing the "same"
  tag would silently fail to conflict.

I am **not** recommending pulling in `unicode-normalization` — that is a heavy dependency for a
contract crate and it makes the tag no longer byte-identical to what the caller passed, which
breaks replication's byte-for-byte forwarding promise. I am recommending you **decide and state**:

1. Fix the doc to say "code points in Unicode category `Cc`".
2. Extend the check to `Cf` as well (one extra predicate, no dependency) and rename the variant to
   something like `InvalidTag::ControlOrFormatCharacter`, or document why `Cf` is allowed.
3. Add one sentence to `Tag`'s docs: *"Tags are compared as byte sequences. No Unicode
   normalisation is performed; callers that accept tag values from user input must normalise
   before constructing a `Tag`."* That is honest, costs nothing, and is the sentence whose absence
   will one day cost someone a day of debugging.

---

## 10. `MAX_TAG_LEN` / `MAX_EVENT_TYPE_LEN` — defensible, but unenforced

The DCB specification imposes **no** length limit on tags or event types (confirmed against
<https://dcb.events/specification/>), so 255 is happenstance's own contract-level constraint.

Is that the right layer? **Yes**, and the doc gives the right reason (`tag.rs:11-13`): a bound is
what lets an adapter declare a fixed-width indexed column without risking silent truncation. Without
a contract-level bound, every adapter invents its own limit and the "storage agnostic" claim is
false the first time a caller moves a store. Raising a limit later is non-breaking; lowering it is
breaking — so 255 errs in the safe direction. Keep it.

**But nothing enforces the other half of the bargain.** There is no conformance rule requiring an
adapter to *accept* a 255-byte tag or a 255-byte event type. `MAX_TAG_LEN` is currently a promise
the contract makes on adapters' behalf and never checks. A SQLite adapter with `tag VARCHAR(64)`
passes all 27 rules today.

**Fix (trivial, high value):** one rule.

```rust
pub async fn accepts_maximum_length_identifiers<S: EventStore, F: Fn() -> S>(factory: F) {
    let store = factory();
    let long_type = "T".repeat(happenstance::MAX_EVENT_TYPE_LEN);
    let long_tag  = Tag::new("x".repeat(happenstance::MAX_TAG_LEN)).expect("at the limit");
    // append, read back, assert byte-identical round trip
}
```

---

## 11. `read` laziness and snapshot semantics are unspecified — and the oracle contradicts the docs

`store.rs:103`:

> The returned stream is **lazy**: nothing is executed until it is first polled, and failures
> surface as `Err` items rather than up front.

`memory.rs:150-182`: `read` takes the lock, filters, sorts, truncates and clones **eagerly**, then
returns a `Snapshot` over an already-materialised `Vec`. The reference implementation — the oracle
the entire suite is validated against — does the opposite of what the port documents.

That is not merely cosmetic, because it changes an observable:

```rust
let stream = store.read(&q, ReadOptions::new());   // not polled yet
store.append(batch, None).await?;                  // a write lands
let events = try_collect(stream).await?;           // does it contain the new events?
```

On `MemoryEventStore`: **no** (snapshot taken at call time). On a lazy SQLite adapter that opens its
transaction on first poll: **yes**. A projection runner or a replication pump that holds an unpolled
stream across a write — which is a natural thing to write — gets different answers from different
conformant adapters.

**Recommendation:** pick one and put it in the contract *and* in the suite. I would specify
**"the stream observes the store as of the first poll"** (it is what a real database adapter does
naturally, and it is what "lazy" already claims), then make `MemoryEventStore` honour it by moving
the snapshot into the first `poll_next`, and add a conformance rule asserting it. If you would
rather specify call-time snapshotting, that is defensible too — but then delete the word "lazy"
from `store.rs:103`, because a SQLite adapter cannot honour it without buffering.

---

## 12. Extensibility: there is no way to add anything to the port without breaking every adapter

Asked directly in the brief, so: adding a method to `EventStore` is breaking for every adapter, full
stop — there is no default body, and `#[non_exhaustive]` does not apply to traits. Today the port
has exactly two methods and no strategy for a third.

Three mechanisms, and which to use for what:

**(a) An extension trait with a blanket impl — for pure combinators.** Verified to compile and to
stay `Send`-composable:

```rust
pub trait EventStoreExt: EventStore {
    fn read_to_vec(&self, query: &Query, options: ReadOptions)
        -> impl Future<Output = Result<Vec<SequencedEvent>, Self::Error>>
    { try_collect(self.read(query, options)) }
}
impl<S: EventStore + ?Sized> EventStoreExt for S {}
```

I compiled this against the real crate and spawned it from a generic
`fn f<S: SendEventStore + Send + Sync + 'static>` — it works. This is the `Iterator`/`Itertools`,
`Future`/`FutureExt` pattern. Everything a caller wants that can be *derived* from `read` and
`append` belongs here: `read_to_vec`, `first`, `exists`, `read_decision_model`. Adding to it later
is non-breaking for adapters, because the blanket impl covers them automatically.

**(b) A separate opt-in trait — for capabilities an adapter should push down.** A blanket ext trait
cannot be overridden, so `count` and `head` do **not** belong in (a): a SQLite adapter wants
`SELECT COUNT(*)` and `SELECT max(position)`, not a full scan.

```rust
/// Optional. Adapters that can answer these without scanning should.
pub trait EventStoreStats: EventStore {
    async fn head(&self) -> Result<Option<SequencePosition>, Self::Error>;
    async fn count(&self, query: &Query) -> Result<u64, Self::Error>;
}
```

`head` in particular is a real gap: `umadb-dcb` has it as a first-class method, and the projection
runner that ADR-0006 moves *into* the contract crate needs it to answer "am I caught up?". Today
the only way to ask is `read(Query::all(), backwards().limit(1))`, which is expressible but is a
full index seek an adapter could do better.

**(c) Provided methods on the port itself** — `umadb-dcb` uses this for `read_with_head`, and it
works, but every provided method is still a permanent part of the trait's `Send`-variant expansion
and complicates `trait_variant`. Prefer (a) and (b).

**Do this before 0.1**, because the ext trait's *existence* is what makes the port stable. Shipping
without it means the first convenience anybody asks for is a breaking change.

Related: **there is still no `dyn EventStore`.** ADR-0001 acknowledges it and names `dynosaur`.
That is the right answer (it is a sibling crate to `trait-variant` in `rust-lang/impl-trait-utils`,
so they are designed to compose), but "batteries included" means an application that picks its
store from a config file should not have to discover `dynosaur` on its own. At minimum ship a
documented recipe; better, ship `happenstance::erased::DynEventStore` behind a feature flag.

---

## 13. `#[non_exhaustive]` audit and semver blast radius

| Type | Marked? | Verdict |
|---|---|---|
| `SequencedEvent` (`event.rs:276`) | yes, pub fields | **Right.** Event identity (`EventId`) is landing in phase 3; this is exactly the room it needs. |
| `ConditionViolated` (`error.rs:95`) | yes, pub field | Right. |
| `ReadOptions`, `AppendCondition` | yes, pub fields | Right — see §8 for the undocumented consequence. |
| `InvalidTag`/`InvalidEventType`/`InvalidQuery`/`AppendError` | yes | Right. Adding a variant is non-breaking; the `AppendError` doc even teaches the wildcard arm (`error.rs:146`). |
| `Event`, `QueryItem` | no, but all fields private | Fine — private fields already prevent literal construction and exhaustive matching. Adding `#[non_exhaustive]` would change nothing. |
| **`Query`** (`query.rs:144`) | **no** | **Deliberately right, and undocumented.** An adapter must translate every variant into storage; a `_ =>` arm it cannot implement is worse than a compile error. Adding a variant *should* break every adapter, loudly. **Say this in the type's docs** as a stability promise, or someone will "fix" it later. |
| `MemoryStoreError` (`memory.rs:145`) | no | Should be. It is `pub enum {}` today; adding a variant later is breaking, and a memory store gaining a failure mode (a capacity bound, say) is plausible. One attribute. |

---

## 14. Serde wire format (C-SERDE) — two decisions that become permanent when `sync` ships

**(a) `Query::All` serialises to JSON `null`** (`query.rs:304-311`). Clever, and compact, but it
means the most dangerous value in the protocol — "this condition matches every event in the store"
— is the value a field takes when it is absent, mistyped, or emitted by a peer with a bug.
`AppendCondition`'s wire struct has no `#[serde(default)]` on `fail_if_events_match`, so a *missing*
field errors, which saves it — but an explicit `null` from a buggy peer silently means "all". An
explicitly tagged form (`{"all": true}` / `{"items": [...]}`) costs a few bytes and cannot be
reached by accident.

**(b) `Bytes` in a self-describing format.** `bytes/serde` calls `serialize_bytes`, which
`serde_json` renders as a **JSON array of integers**. A 1 KiB payload becomes ~4 KiB of
`[123,34,115,...]`. If the replication envelope is ever JSON, that is a 4× wire cost and unreadable
logs. Either document that the `serde` feature targets binary formats (postcard/CBOR/MessagePack,
where `serialize_bytes` is compact and correct — and postcard is already the natural fit), or
branch on `serializer.is_human_readable()` and emit base64 for the human-readable case. This is
cheap now and impossible later.

The rest of the serde work is careful and correct — in particular `Tags::deserialize`
re-canonicalising (`tag.rs:318-323`) so a peer cannot smuggle an unsorted `Tags` past the invariant,
and the private `*Wire` mirror structs that keep the format stable while the real fields stay
private. That is the right pattern and most crates do not bother.

---

## 15. Conformance suite gaps (the testkit lens)

The suite is genuinely good — 27 rules, each traced to a MUST, each with a failure message that
explains the rule rather than printing `assertion failed`. The gaps are specific:

**(a) `limit` is only ever tested against `Query::all()`.** `suite.rs:292-302`. The single most
likely bug in a real SQL adapter is applying `LIMIT` **before** the tag filter — `SELECT ... WHERE
type IN (..) LIMIT 2` and then filtering tags in memory returns the wrong two events. The suite
cannot catch it. Add:

```rust
pub async fn read_limit_applies_after_filtering<S: EventStore, F: Fn() -> S>(factory: F) {
    // Interleave matching and non-matching events, then read with limit 2.
    // A store that truncates before filtering returns 1 event (or the wrong ones).
}
```
This is the highest-value rule missing, by a distance.

**(b) No rule that an adapter accepts `MAX_TAG_LEN` / `MAX_EVENT_TYPE_LEN`.** See §10.

**(c) No rule pinning empty-batch vs violated-condition precedence.** See §3.

**(d) `append_is_atomic` (`suite.rs:385`) does not test atomicity.** It appends three events with a
condition that is already violated and asserts nothing landed — which is byte-for-byte what
`condition_rejection_leaves_store_unchanged` (`suite.rs:529`) tests. Genuine partial-write atomicity
(event 2 of 3 fails for a storage reason) is not reachable through the port, so either rename the
rule to `rejected_append_writes_nothing` or add a hook the adapter can use to inject a mid-batch
failure. Naming a rule for a property it does not check is worse than not having it.

**(e) No rules for:** `from` beyond the last position (must be empty, not an error); `limit(0)` (see
§8); the retry loop (violated append → re-read → re-conditioned append succeeds); read/append
interleaving semantics (see §11).

**(f) The rule list is duplicated by hand.** `lib.rs:93-129` lists 27 names; `suite.rs` defines
them. Adding a rule to `suite.rs` and forgetting `lib.rs` is a **silent** loss of coverage for every
adapter. A `rules!` registry macro that generates both is already on the runbook for phase 5 —
pull it forward, it is cheap and it removes a silent failure mode.

**(g) `fixtures.rs` uses `expect` throughout in a *published library* crate.** The house rule is "no
`unwrap`/`expect` in library code; tests may". `happenstance-testkit` is a library that ships to
crates.io, and the workspace lint list sets `clippy::unwrap_used = "warn"` but not
`clippy::expect_used`, so nothing catches it. The `expect`s are defensible here (they are test
fixtures) — the finding is that the lint policy does not say so. Add `expect_used = "warn"` to the
workspace and an explicit `#![allow(clippy::expect_used)]` with a one-line justification at the top
of `fixtures.rs`. That turns an accident into a decision.

---

## 16. Confirmed defect: `SequencePosition::next()` does not detect overflow

```rust
// event.rs:138
/// The next position, or `None` on overflow.
pub const fn next(self) -> Option<Self> {
    Self::new(self.0.get().saturating_add(1))
}
```

`u64::MAX.saturating_add(1) == u64::MAX`, and `SequencePosition::new(u64::MAX)` is `Some`. Verified
at runtime:

```
SequencePosition::new(u64::MAX).next() = Some(SequencePosition(18446744073709551615))
is it Some(MAX)? true
```

The method returns the *same* position instead of `None`. `checked_add(1)` is the one-word fix.
Practically unreachable, but this is a method whose entire purpose is signalling overflow, and it is
the kind of thing a reviewer of a "reliable workhorse" library will find.

Same pattern, same file family: `memory.rs:131` `position_at` uses
`u64::try_from(index).unwrap_or(u64::MAX - 1).saturating_add(1)`. The `unwrap_or` arm is
unreachable on every supported platform (`usize` is 16/32/64-bit), so it is dead code that reads
like a considered decision. Simplify or comment.

---

## What is right, and should not be touched

- **Constraint 3 (`read` returns the stream at the top level and is not `async`).** Correct, and the
  reasoning in ADR-0001 is exactly right. Keep the test.
- **`AppendError<E>` generic over the adapter error, with `ConditionViolated` lifted out.** Better
  than every prior art I looked at. `is_condition_violated()` and `map_store()` are the right two
  helpers and no more.
- **`Query` as an enum rather than `Option<Vec<QueryItem>>`.** `umadb-dcb` uses
  `Option<DcbQuery>`; the enum is strictly better and the reason given in the docs is the right one.
- **`SequencePosition(NonZeroU64)`.** The niche is genuinely load-bearing — `Option<SequencePosition>`
  is in `AppendCondition` and `ReadOptions` — and the doc explicitly warns against treating
  positions as counts, which is the mistake everyone makes.
- **`Tags` canonical at construction**, with the `PartialEq`-means-set-equality consequence spelled
  out, `contains_all` as a merge scan, and a property test pinning it to the naive definition.
- **Opaque `Bytes` payloads (ADR-0003)** and the `bytes` re-export.
- **`MemoryStoreError` as an uninhabited enum**, and the docstring explaining why that is worth
  having. Most crates would have written `struct MemoryStoreError;`.
- **Documentation.** C-CRATE-DOC, C-EXAMPLE, C-QUESTION-MARK (doctests use `?`, not `unwrap`),
  C-FAILURE (`# Errors` everywhere) and C-LINK are all met, and the *why* comments
  (`memory.rs:190-194` on the write lock, `append.rs:101` on the missing let-chain,
  `store.rs:31-45` on E0034) are the best thing in the codebase.
- **The workspace lint policy and single-source dependency table.** `pedantic`, `missing_docs`,
  `unreachable_pub`, `forbid(unsafe_code)`, `broken_intra_doc_links = "deny"` — this is stricter
  than most published crates and the `priority = -1` trick is used correctly.

---

## Probe log

Everything asserted above was compiled against `D:/repos/happenstance/crates/happenstance` from a
scratch crate at
`C:\Users\ryanm\AppData\Local\Temp\claude\D--repos-happenstance\559ba08d-08bc-4f98-b677-6b98e7a98078\scratchpad\probe`.

| Probe | Result |
|---|---|
| `SequencePosition::new(u64::MAX).next()` | `Some(MAX)` — doc says `None` |
| generic `fn f<S: EventStore + Send + Sync>` + `tokio::spawn(read_decision_model)` | **E0277**, stream not `Send` |
| same with `S: SendEventStore` | compiles |
| `read_stream_is_send` equivalent on a concrete store | compiles either way (leakage) |
| `owned_tags.into_iter()` | compiles, yields `&Tag` |
| `for t in tags` | E0277 |
| `HashMap<EventType,_>::get("literal")` | E0308 (`Borrow<str>` missing) |
| `HashSet<Event>`, `HashSet<Query>` | E0599 (`Hash` missing) |
| `"X".parse::<EventType>()`, `"a:b".parse::<Tag>()` | E0277 (`FromStr` missing) |
| `ReadOptions { backwards: true, ..ReadOptions::new() }` | E0639 |
| `ProjectionId::from("x")`, `ProjectionId::as_ref()` | E0277 / E0599 |
| `append(&[], Some(&violated_condition))` on `MemoryEventStore` | `ConditionViolated`, not `NoEvents` |
| `EventStoreExt` blanket-impl ext trait, spawned from a generic `SendEventStore` fn | compiles and runs |
| `const fn from_static` on `EventType(Cow<'static, str>)`, incl. `Cow` Ord/Hash delegation | compiles; invalid literal is a compile error; 16 → 24 bytes |

## Sources

- <https://rust-lang.github.io/api-guidelines/checklist.html>
- <https://github.com/rust-lang/rust/issues/109417> (RTN tracking issue)
- <https://github.com/rust-lang/rust/pull/138424> (RTN stabilisation PR — closed unmerged 2025-12-27)
- <https://blog.rust-lang.org/2026/05/18/project-goals-2026-04/>
- <https://crates.io/api/v1/crates/trait-variant> (0.1.3, 2026-07-22; repo `rust-lang/impl-trait-utils`)
- <https://docs.rs/trait-variant/latest/trait_variant/attr.make.html>
- <https://dcb.events/specification/>
- <https://docs.rs/umadb-dcb/latest/umadb_dcb/trait.DcbEventStoreAsync.html>
