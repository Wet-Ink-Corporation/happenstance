# Is the conformance suite actually a bar?

Review lens: `happenstance-testkit` as the instrument that makes "storage agnostic" checkable.
Date: 2026-08-05. Reviewed at commit `9fd2337`, working tree as found.

---

## Method — this is not a reading of the code, it is a measurement of it

Claims about a test suite are worth exactly as much as the test of the test suite. So rather than
reason about the 27 rules, I built six deliberately non-conformant event stores, each carrying
exactly one bug that a real SQLite adapter would plausibly ship, and ran the full suite against each.

The harness lives at
`C:\Users\ryanm\AppData\Local\Temp\claude\D--repos-happenstance\559ba08d-08bc-4f98-b677-6b98e7a98078\scratchpad\downstream\`
(a crate *outside* the workspace, consuming `happenstance-testkit` by path — which also answers
question 5 empirically).

### Result

| Injected bug | Rules that caught it | Verdict |
|---|---|---|
| `LIMIT` applied **before** the query filter (`SELECT * FROM event LIMIT ?` then filter in Rust) | **0 / 27** | undetected |
| `from` implemented as "the row whose position **equals** `from`", on a store with position gaps | **0 / 27** | undetected |
| An event matching **two query items** returned **twice** (join without `DISTINCT`) | **0 / 27** | undetected |
| `Some(Bytes::new())` metadata collapsed to `None` (nullable BLOB column) | **0 / 27** | undetected |
| Positions allocated in **descending per-batch blocks** (HiLo / per-writer ID block) | 2 / 27 | caught |
| Append condition evaluated **after** the insert (batch self-conflict) | 1 / 27 | caught, but by the wrong rule with a misleading message |

Baseline (a correct store) passes 27/27, and `MemoryEventStore` passes 27/27, so the suite is not
simply broken. It is *narrow*.

**Four of six realistic adapter bugs pass the entire conformance suite.** The two that are caught
are caught by append-condition rules, not by the rules nominally responsible — `PerBatchPositions`
sails past `positions_are_strictly_monotonic` and `positions_are_unique`, and
`ConditionAfterInsert` is caught only by `racing_conditional_appends_elect_one_winner`, whose
failure message reads *"the first handler to commit must succeed"* — which tells the adapter author
nothing about what they actually did wrong.

Two of the four undetected bugs are ones the project's **own intended schema invites**.
`crates/happenstance-sqlite/src/event_store.rs:21-25` specifies

```sql
CREATE TABLE event_tag (
    position INTEGER NOT NULL REFERENCES event(position),
    tag      TEXT    NOT NULL,
    PRIMARY KEY (tag, position)
) WITHOUT ROWID;
```

A query of two OR-ed items joined against that table without `DISTINCT`/`EXISTS`/`GROUP BY` returns
the doubly-matching event twice. And line 15, `metadata BLOB` (nullable), is where
`Some(Bytes::new())` goes to die. Phase 1 is scheduled to write exactly this adapter against exactly
this suite.

---

## 1. Rule-by-rule: the weakest implementation that still passes

Grouped by how much load each rule actually bears. Line numbers are
`crates/happenstance-testkit/src/suite.rs`.

### Rules that hold real weight

| Rule | Line | Weakest passing implementation | Note |
|---|---|---|---|
| `query_item_combines_types_and_tags_with_and` | 167 | correct AND of type and tags | properly anchored against `positions_of(&all[..1])` — the model for how every rule should assert |
| `query_items_are_or` | 200 | correct OR, **no dedup required** | anchored, but no event in the fixture matches two items |
| `query_item_rejects_partial_tag_overlap` | 148 | correct AND | tight |
| `condition_after_ignores_events_at_the_boundary` | 477 | `position <= after` skip | uses `append`'s returned position as the anchor — the right cross-check |
| `condition_after_rejects_events_beyond_the_boundary` | 494 | correct `>` comparison | the rule that caught `PerBatchPositions` |
| `condition_rejection_is_reported_as_condition_violated` | 549 | correct error variant | tight, and the message explains why |
| `racing_conditional_appends_elect_one_winner` | 596 | correct pre-insert condition evaluation | the strongest rule in the suite; also the only thing standing between an adapter and the self-conflict bug |

### Rules that are weaker than they look

| Rule | Line | Weakest passing implementation | What slips through |
|---|---|---|---|
| `read_limit_truncates` | 292 | `ORDER BY position LIMIT n` **applied before any filtering** | query is `Query::all()`, so filter/limit ordering is unobservable. **The single cheapest high-value fix in this document.** |
| `read_backwards_from_with_limit` | 306 | same | also `Query::all()` |
| `positions_are_strictly_monotonic` | 355 | *any* store that sorts its read output by position | proven: `PerBatchPositions` passes. It never checks that position order agrees with **append** order across two `append` calls. Only `query_all_matches_every_event` checks that, and only *within* one batch. |
| `positions_are_unique` | 336 | a monotone counter | reads back through `read`, so a store that de-dups on read hides duplicates |
| `read_defaults_to_ascending_order` | 249 | any sorted output | asserts positions ascend; never asserts the *events* are in append order |
| `query_item_tags_are_and` | 96 | correct AND returning **any one** event | asserts `positions_of(&found).len() == 1` — the length only. A store returning the *wrong* single event passes. |
| `query_item_tags_match_supersets` | 123 | `found.len() == 1` | same shape |
| `read_from_is_inclusive` | 262 | `WHERE position >= ?` **or** "index into the vec" | the anchor is always an existing, densely-allocated position; on a gapped store an exact-match implementation is indistinguishable (proven) |
| `append_returns_last_written_position` | 372 | `SELECT max(position) FROM event` after commit | racy under concurrency, indistinguishable sequentially |
| `query_all_matches_every_event` | 70 | return everything in insertion order | genuinely useful: it is the *only* rule pinning intra-batch order |
| `query_item_types_are_or` | 83 | `WHERE type IN (…)` | no query type absent from the store, no duplicate type in the item |

### A rule that does not test what it is named for

`append_is_atomic` (line 385) sets up a condition that is *already* violated, appends three events,
and asserts nothing was written. That is not atomicity — that is "a rejected append writes nothing",
which is the same claim `condition_rejection_leaves_store_unchanged` (line 529) makes twelve rules
later. Genuine atomicity is "a batch that fails **part-way through** leaves nothing", and no adapter
failure can be induced through the port today. Two of the 27 assert the same property, so the real
count is 26.

---

## 2. Missing rules

### 2a. The laziness contract is documented, false in the oracle, and untested

`crates/happenstance/src/store.rs:103-105`:

> The returned stream is **lazy**: nothing is executed until it is first polled, and failures
> surface as `Err` items rather than up front.

`crates/happenstance/src/memory.rs:150-182` does the entire filter, order and truncate *inside the
call*, under the lock, and returns an already-materialised `Snapshot(Vec::into_iter)`. I verified it:

```
events observed by a stream created before an append: 1
=> LAZY would be 2, EAGER/snapshot is 1
```

So the reference implementation — the crate's own oracle, and the thing every adapter author will
read as the worked example — contradicts the contract sentence on the very method the contract is
built around.

This matters far beyond a doc typo, because it is entangled with a second thing the contract does
not say at all: **read isolation**. Does a stream held across a concurrent `append` see the new
events?

- `MemoryEventStore`: no (snapshot at call time).
- A SQLite adapter with a live cursor in WAL mode: no (snapshot isolation) — but the snapshot is
  taken at *first poll*, not at call.
- A SQLite adapter paging with `LIMIT/OFFSET`: yes, and it will **skip and duplicate rows**.

All three are "conformant" today. The suite never holds a stream across anything: every rule calls
`collect(store.read(…))` inline. This is the most important semantic hole in the contract, and it is
invisible to the suite by construction.

**Decide and write it down.** My recommendation: specify *snapshot at first poll* — laziness plus a
stable view — because it is the only choice that lets a million-event replay stream without
buffering *and* gives the caller a coherent answer. Then delete the "eager" behaviour from
`MemoryEventStore` (wrap the `Vec` clone in a `poll_next`-triggered `OnceCell`, or make the stream
hold the `Arc` and index into it) and add rules for both halves.

### 2b. Batch self-conflict is unspecified, and the oracle silently answers it

`memory.rs:197-224` checks the condition, then checks emptiness, then extends. So the condition is
evaluated against the **pre-append** state and a batch cannot conflict with itself. Nothing in
`store.rs`, `append.rs` or the spec text says this, and the DCB spec's wording ("fail if the Event
Store **contains** at least one Event matching the Append Condition") is exactly ambiguous about
whether "contains" is evaluated before or after the insert.

Every adapter that implements the check as `INSERT …; SELECT EXISTS(…); ROLLBACK` — or with an
`AFTER INSERT` trigger — gets the other answer. I built that store: it fails exactly one rule, with
the message `the first handler to commit must succeed`. An adapter author will read that as
"my append is broken" and not as "I am evaluating my condition against my own writes".

This is genuinely subtle, it is the classic SQL-adapter bug, and it needs (a) a sentence in
`AppendCondition`'s docs and (b) its own rule with its own message.

### 2c. Value-edge coverage is absent

`append_preserves_event_payload` (line 417) round-trips one event with a JSON payload, one tag and
non-empty metadata. Untested, all of which a real column mapping gets wrong:

- `Bytes::new()` payload (zero-length BLOB vs NULL vs "" in a TEXT column)
- `metadata: Some(Bytes::new())` vs `metadata: None` — **distinct values in the type system**
  (`Option<Bytes>`, `event.rs:187`), routinely collapsed by a nullable column. Proven undetected.
- an event with zero tags, and an event with many tags
- a tag at exactly `MAX_TAG_LEN`, an event type at exactly `MAX_EVENT_TYPE_LEN`
- non-ASCII event types and tags (both validators use `char::is_control` and byte-length limits, so
  `"Kurs–Definiert"` is legal — does the adapter's `TEXT` column and its `IN (…)` binding agree?)
- a large payload (1 MiB) — SQLite's default `SQLITE_MAX_LENGTH` is fine, but a chunked adapter is not
- reading an **empty store** at all (`MIN`/`MAX` returning NULL is the classic crash)

### 2d. Missing read-option cases

- `from` beyond the head → empty, not an error (verified correct on the oracle; unpinned)
- `from` + `backwards` where `from` precedes every event → empty
- `from` at a position **inside a gap** — since gaps are legal, `from` is a *threshold*, not an
  identity. Proven undetected.
- `limit` greater than the match count; `limit` exactly equal to it
- `limit` combined with a **filtering** query, forwards and backwards
- reading twice with the same query returns the same answer (idempotence)

### 2e. Missing append-condition cases

- a condition whose query is `Query::all()` — on an empty store it must succeed; afterwards every
  append must fail. A SQL adapter that special-cases `Query::All` into "no `WHERE` clause" has a
  distinct code path here and nothing exercises it.
- a condition on an **empty store** (an `EXISTS` probe or a `max(position)` guard over an empty
  table)
- `after` pointing beyond the head → append succeeds
- `after` pointing at a never-assigned position inside a gap
- the retry loop closing: after a rejection, the loser re-reads and *succeeds*. That is the loop
  every application actually writes and no rule demonstrates it end-to-end.

### 2f. No durability rule, and the macro's shape forbids one

`event_store_conformance!` takes `$factory:expr` and the rules call it as `|| $factory` — a `Fn() -> S`
producing a **fresh, empty** store per test. There is therefore no way to express "close the store,
reopen it, the log is unchanged, and the next position continues past the old head". That is
precisely the guarantee `AUTOINCREMENT` was chosen for
(`happenstance-sqlite/src/event_store.rs:30-32`: *"it guarantees positions are never reused after a
delete"*), and nothing will check it.

Fix: a second, opt-in macro taking a factory that can hand out a **second handle to the same
store** — `Fn(&Path) -> S`, or `Fn() -> S` plus `Fn(&S) -> S`. `MemoryEventStore` opts out; SQLite
and the Durable Object opt in.

### 2g. No genuinely parallel rule — and the stated reason does not hold

`suite.rs:589-595`:

> It is written sequentially on purpose. A genuinely parallel version would need `Send + Sync +
> 'static` bounds that `EventStore` does not carry […] and would risk a flaky conformance suite,
> which is worse than none.

The first half is right. The second half is not, and I have a running counter-example: the
assertion in a proper DCB race is an **invariant**, not a timing observation, so it cannot flake.
32 tasks on an 8-thread runtime running read→decide→append→retry against a capacity-3 course:

```
committed=3 final_len=3
test capacity_invariant_holds_under_real_contention ... ok
```

Exactly `CAPACITY` commit, every time, regardless of interleaving. If the store's condition check
and write are not one atomic step, `final_len` exceeds `CAPACITY` and the test fails deterministically
in the sense that matters: it fails *eventually and loudly*, never spuriously. That is the rule that
would have caught a `SELECT EXISTS` followed by a separate `INSERT` outside a transaction —
which is the bug RUNBOOK phase 1 predicts will hurt, and which the current sequential rule
**cannot** catch, because sequentially the two statements are indistinguishable from one.

---

## 3. Recommended new rules

Each line is the assertion and the adapter bug it catches. Ordered by value. The first four are
the ones I proved slip through today.

### Tier 1 — proven gaps

| # | Rule | Assertion | Bug caught |
|---|---|---|---|
| 1 | `read_limit_applies_after_filtering` | append `[A,B,A,B,A]`; `read(types=[A], limit=2)` returns the **first two A's**, not "the A's among the first two rows" | `SELECT … LIMIT ?` before the `WHERE`, or `LIMIT` inside a subquery. **Proven undetected.** |
| 2 | `read_limit_applies_after_filtering_backwards` | same, with `.backwards()` | the same bug on the descending path, which is usually a separate SQL string |
| 3 | `query_returns_an_event_matching_two_items_once` | two items, one event matching **both**; `found.len() == 1` | `JOIN event_tag` / `OR`-ed `EXISTS` without `DISTINCT`. **Proven undetected**, and the intended schema invites it. |
| 4 | `read_from_is_a_threshold_not_an_identity` | append 3 events; `read(from = P₂.next())` returns the events at positions **> P₂**, i.e. `{P₃}` even when `P₂+1` was never assigned | `WHERE rowid >= (SELECT rowid WHERE position = ?)`. **Proven undetected** on a gapped store. |
| 5 | `metadata_none_and_empty_are_distinct` | append two events, one `metadata: None`, one `Some(Bytes::new())`; both round-trip distinctly | nullable BLOB collapsing `X''` to `NULL`. **Proven undetected.** |
| 6 | `positions_agree_with_append_order_across_batches` | append `[A,B]` then `[C,D]`; `types_of(read_all()) == [A,B,C,D]` **and** positions strictly increase | per-writer ID blocks, HiLo allocators, snowflake IDs with clock skew. **`positions_are_strictly_monotonic` passes today** on such a store. |
| 7 | `condition_is_evaluated_before_the_batch_is_written` | append `[X]` with a condition matching type `X` and `after = head`; must **succeed** | condition probed post-insert (self-conflict). Currently caught only by `racing_…` with a misleading message. |

### Tier 2 — cheap, and each one is a distinct SQL code path

| # | Rule | Assertion | Bug caught |
|---|---|---|---|
| 8 | `read_on_an_empty_store_is_empty` | fresh store; `read(Query::all())` yields `[]` and does not error | `MIN`/`MAX` returning NULL; an unwrapped `Option` in the head lookup |
| 9 | `read_from_beyond_the_head_is_empty` | `read(from = head.next())` yields `[]` | off-by-one in the range predicate |
| 10 | `read_backwards_from_before_the_first_event_is_empty` | `read(from = FIRST-ish, backwards)` on a store whose first position is > that | reversed comparison on the descending path |
| 11 | `read_limit_larger_than_the_match_count_returns_all` | `limit(100)` over 3 matches returns 3 | a `LIMIT`/`OFFSET` pager that loops forever or pads |
| 12 | `read_is_idempotent` | the same query twice returns the identical `Vec` | cursor state leaking between reads; a `LAST_INSERT_ROWID`-style shared statement |
| 13 | `condition_on_an_empty_store_allows_the_append` | fresh store; append with any condition succeeds | `EXISTS` probe / `max(position)` guard over an empty table |
| 14 | `condition_matching_all_events_rejects_after_the_first_append` | condition = `Query::all()`, no `after`; first append OK, second `ConditionViolated` | the `Query::All` special case compiling to an always-false predicate |
| 15 | `condition_after_beyond_the_head_allows_the_append` | `after = head.next()`; append succeeds | unsigned underflow, or `after` treated as inclusive |
| 16 | `condition_after_a_gapped_position_still_rejects_later_matches` | with gaps, `after = P₂` and a match at `P₃` rejects | the exact-match `from`/`after` bug on the write path |
| 17 | `a_rejected_handler_can_retry_and_commit` | after `ConditionViolated`, re-read, rebuild the condition, append again → `Ok` | a store that leaves a poisoned transaction or a stale statement after a rejection |
| 18 | `empty_payload_round_trips` | `Bytes::new()` payload survives | zero-length BLOB → NULL → `NOT NULL` violation, or `""`→NULL in TEXT |
| 19 | `event_with_no_tags_round_trips_and_matches_query_all` | zero tags survives, and is returned by `Query::all()` | an `INNER JOIN event_tag` that silently drops untagged events — a *very* likely bug given the schema |
| 20 | `max_length_tag_and_event_type_round_trip` | tag at `MAX_TAG_LEN`, type at `MAX_EVENT_TYPE_LEN` | `VARCHAR(n)` truncation, index key-length limits |
| 21 | `non_ascii_event_types_and_tags_round_trip_and_match` | `"Kurs–Definiert"`, `"student:ß1"` filter correctly | collation / `NOCASE` / byte-vs-char indexing |
| 22 | `many_tags_on_one_event_round_trip` | 64 tags survive and all match | fixed-width tag encoding; parameter-count limits (`SQLITE_MAX_VARIABLE_NUMBER`) |
| 23 | `large_payload_round_trips` | 1 MiB payload survives byte-for-byte | chunking, `SQLITE_MAX_LENGTH`, a `TEXT` column mangling non-UTF-8 |

### Tier 3 — needs a contract decision first

| # | Rule | Assertion | Blocked on |
|---|---|---|---|
| 24 | `read_does_no_work_until_polled` | build a stream, do not poll it, append; poll — behaviour matches the specified isolation model | §2a: decide lazy-vs-eager and snapshot-vs-live |
| 25 | `a_held_stream_is_not_invalidated_by_a_concurrent_append` | hold a half-drained stream across an append; the remaining items are well-defined (no skips, no duplicates) | §2a, and the lifetime issue in §5 |
| 26 | `empty_batch_is_rejected_even_when_a_condition_would_also_fail` | `append(&[], Some(violated_condition))` returns `NoEvents` | §2b: pin the ordering. The oracle currently returns `ConditionViolated` (verified). |
| 27 | `append_is_atomic_under_a_mid_batch_failure` | a fault-injecting wrapper store fails on event 2 of 3; nothing is written | needs a `FaultyStore<S>` decorator in the testkit |
| 28 | `reopening_the_store_preserves_the_log_and_the_position_watermark` | close, reopen, read → identical; next append lands **above** the old head | needs the durability factory (§2f) |

---

## 4. The macro's shape — build the registry now

### The two defects

**(a) The rule list is duplicated.** The 27 names appear in `suite.rs` as `pub async fn` items and
again in `lib.rs:93-129` as `conformance_test!(…)` arms. They are in sync today (I diffed them), but
nothing enforces it. Adding a rule and forgetting the second list ships a rule nobody runs — in the
crate whose entire purpose is running rules. The failure is silent and permanent.

**(b) `#[tokio::test]` is hardcoded** at `lib.rs:85`, and `tokio` is resolved in the *caller's*
namespace, so a downstream crate that renames the dependency breaks, and one that uses
`async-std`/`smol`/`wasm-bindgen-test` cannot use the suite at all.

RUNBOOK phase 5 defers this. That is the wrong phase. **Phases 1, 2 and 4 all write adapters and
their conformance harnesses before phase 5 runs**, so deferring means three adapters, one worked
example and a projection suite will be built against a shape that is already known to be wrong. The
restructure is a day's work now and a migration later.

### The design

The Rust-specific constraint that shapes this: `macro_rules!` cannot build a list programmatically,
cannot paste an identifier from a string, and cannot recurse over "all functions in a module" —
there is no reflection at macro-expansion time. The idiomatic workaround is the **callback macro**
(sometimes "x-macro"): one macro owns the list and takes the *name of another macro* to hand it to.

I verified this compiles on 1.97.1 / edition 2024, including invoking a macro through a `$…:path`
metavariable:

```rust
// happenstance-testkit/src/lib.rs — the single source of truth.
#[doc(hidden)]
#[macro_export]
macro_rules! for_each_rule {
    ($callback:path $(, $($extra:tt)*)?) => {
        $callback! {
            $($($extra)*,)?
            rules = [
                query_all_matches_every_event,
                query_item_types_are_or,
                // … every rule, listed exactly once in the whole workspace
            ]
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __emit_tokio {
    (factory = $f:expr, rules = [$($name:ident),* $(,)?]) => {
        $(
            #[tokio::test]
            async fn $name() { $crate::rules::$name(|| $f).await; }
        )*
    };
}

/// The default, tokio-flavoured suite. Unchanged for every existing caller.
#[macro_export]
macro_rules! event_store_conformance {
    ($factory:expr) => {
        $crate::event_store_conformance!(mod_name = dcb_conformance, factory = $factory);
    };
    (mod_name = $m:ident, factory = $f:expr) => {
        mod $m {
            #![allow(clippy::unwrap_used, unused_imports)]
            use super::*;
            $crate::for_each_rule!($crate::__emit_tokio, factory = $f);
        }
    };
}
```

Three notes that are easy to get wrong:

- The callback macro must be `#[macro_export]`ed even though it is `#[doc(hidden)]`.
  `macro_rules!` items live in a flat, crate-root, textual namespace — they are not resolved like
  normal items — so a private helper is not reachable from the expansion site downstream.
- Prefix the callback with `$crate::` in the invocation. Without it, resolution happens in the
  *caller's* scope, which is exactly the bug you are fixing.
- `rules = [ … ]` as a named section (rather than positional `tt`s) is what lets a second emitter add
  its own leading arguments without ambiguity.

Then each runtime is one ~10-line emitter, and none of them touch the rule list:

```rust
// wasm / Cloudflare Workers — phase 5
macro_rules! __emit_wasm { (factory = $f:expr, rules = [$($n:ident),*$(,)?]) => { $(
    #[::wasm_bindgen_test::wasm_bindgen_test]
    async fn $n() { $crate::rules::$n(|| $f).await; }
)* }; }

// no async runtime at all — works everywhere, zero runtime dependency
macro_rules! __emit_blocking { (factory = $f:expr, rules = [$($n:ident),*$(,)?]) => { $(
    #[test]
    fn $n() { ::futures::executor::block_on($crate::rules::$n(|| $f)); }
)* }; }
```

`futures::executor::block_on` deserves consideration as the **default** rather than tokio. Every
rule is pure `async` with no timers, no I/O and no `spawn`; the only thing tokio contributes is a
dependency the adapter author must add and a runtime the rules never use. `block_on` removes the
downstream dev-dependency entirely and works identically on native and (with a caveat) wasm. Adapters
that genuinely need a tokio context — a `sqlx` pool, say — opt into `__emit_tokio`.

`libtest-mimic` is the wrong tool here. It buys you a custom test harness with runtime-registered
cases, which matters when the case list is data (file-driven golden tests). Your case list is
statically known, and you *want* one `#[test]` per rule so `cargo test <rule_name>` and the IDE
gutter work. Stay with generated `#[test]` items.

### The parallel suite

Genuine concurrency needs bounds `EventStore` deliberately does not carry, so it must be a second,
opt-in macro — which is fine, because it is also the only one that can express the rules that
matter most:

```rust
/// Opt-in. Requires `S: SendEventStore + Send + Sync + 'static` and a
/// multi-thread runtime. Every native adapter should invoke BOTH macros.
#[macro_export]
macro_rules! send_event_store_conformance { … }
```

It must assert, over `Arc<S>` with N tasks on a multi-threaded runtime:

1. **The capacity invariant.** N contenders, a capacity-K decision model, read→decide→append→retry
   on `ConditionViolated`. Exactly K commit and the final log holds exactly K matching events —
   verified working against `MemoryEventStore` (`committed=3 final_len=3`, 32 contenders, 8 threads).
   Catches a non-atomic probe-then-insert, which the sequential rule cannot.
2. **Uniqueness under contention.** After N concurrent unconditional appends, positions are unique
   and total — catches a `max(position)+1` allocator.
3. **`append`'s return value is the caller's own last event**, not the global head — catches
   `SELECT max(position)` after commit, which is *only* observable in parallel.
4. **Concurrent reads during writes never observe a partial batch** — a reader loop asserting the
   count of a marker type is always a multiple of the batch size. This is the atomicity rule the
   sequential suite cannot write.
5. **No deadlock / no starvation**: every task terminates within a generous timeout. Catches a
   SQLite adapter with `busy_timeout` unset or a write lock held across an `await`.

---

## 5. The property tests

The eight properties in `tests/properties.rs` are good and correctly scoped — they pin `Tags`
canonicalisation, `contains_all` against a naive definition, and `Query` matching as a lattice
(order-insensitive, monotone under item addition, `all` is top). `contains_all` is a hand-written
merge-scan (`tag.rs:231-245`) and `contains_all_agrees_with_elementwise_containment` is exactly the
right guard for it.

But they test only **pure functions**. Not one of them touches an `EventStore`. The generators
are also narrower than they look: `any_tag` samples from `["a","b","c","d","e"]` and `any_event_type`
from `["A","B","C"]`, so nothing ever generates a `key:value` tag, a max-length value, or a
non-ASCII one — the shapes real events carry.

### The missing test: a stateful model

I built it as a prototype and it is the single highest-value addition to this crate. A random
program of appends / conditional appends / reads, run against the adapter and against a model built
from the contract's own pure matcher, asserting they agree after every step.

Against the six stores:

```
baseline_agrees_with_the_model ....... ok (400 random programs)
catches_limit_before_filter .......... CAUGHT -> step 2: read returned 0 events, model says 1
catches_gapped_from_exact ............ CAUGHT -> step 1: read returned 0 events, model says 1
catches_duplicate_on_or .............. CAUGHT -> step 1: read returned 2 events, model says 1
catches_per_batch_positions .......... CAUGHT -> step 1: append returned SequencePosition(200), log head is Some(SequencePosition(300))
catches_condition_after_insert ....... CAUGHT -> step 0: append REJECTED but nothing matched the condition
```

**All five behavioural bugs, including the three that the entire 27-rule suite misses**, in a few
hundred lines that will never need updating when a new read option is added.

Design notes that make it work:

- **Positions are learned, never assumed.** The generator emits *symbolic* anchors —
  `None | First | Middle | Head | BeyondHead` — resolved at execution time against the positions the
  store actually assigned. This is the generalisation of CLAUDE.md's rule about never asserting on
  literal positions, and it is what lets the same test run against a dense store and a gapped one.
- **The model reuses `Query::matches` and `AppendCondition::is_violated_by`.** That is not circular:
  those are pure functions already property-tested against naive definitions, and the adapter's job
  is precisely to agree with them. It makes the model ~30 lines instead of ~300.
- **The conditional-append outcome is *predicted*, not observed.** Before calling `append`, the model
  computes whether the condition is already violated, and asserts the store agrees in both
  directions. `append SUCCEEDED but the condition was already violated` is the message that catches
  the class of bug DCB exists to prevent.
- **Cross-check `append`'s return against the log head** after every append. That is what caught the
  per-batch position allocator.
- Hand-rolled `Vec<Op>` + `prop::collection::vec` rather than `proptest-state-machine`: no new
  dependency, and proptest shrinks a `Vec<Op>` perfectly well. The shrunk counterexamples above are
  1–3 steps long.

Two extensions before shipping it: generate metadata (`None` / `Some(empty)` / `Some(bytes)`) and
payloads (`empty` / small / large) so it also covers the value-edge class, and export it from the
testkit as `event_store_model_conformance!(…)` so **adapters** get it, not just this repo.

---

## 6. Is the suite runnable by a third party today? Yes.

Traced empirically with a crate outside the workspace:

```toml
[dependencies]
happenstance = { path = "…/crates/happenstance" }
[dev-dependencies]
happenstance-testkit = { path = "…/crates/happenstance-testkit" }
tokio = { version = "1", features = ["macros", "rt"] }
```

```
running 27 tests … test result: ok. 27 passed; 0 failed
```

The feature wiring is correct and, I think, deliberately so:

- `Cargo.toml:24` sets `default-features = false` on the internal `happenstance` workspace
  dependency, with a comment explaining exactly why (a member cannot *remove* a workspace default).
  That is right and it is the non-obvious half of the workspace-dependency pattern.
- `happenstance-testkit/Cargo.toml:14` therefore gets `happenstance` with **only** `std` for its
  library — no `memory`. The rules are generic over `S: EventStore`, so `memory` is genuinely a
  dev-only concern, and line 17 adds it there. A downstream adapter that wants a `no-memory` build
  is not forced into one by the testkit; it only inherits `std`, which the testkit really does need.
- The testkit does not declare `tokio` at all, which is correct — the `#[tokio::test]` lands in the
  *caller's* crate. It is documented at `lib.rs:23-24`. (It is also the coupling that the registry in
  §4 removes.)

Two smaller problems in this area:

**The headline doctest is inert.** `lib.rs:12-15`:

```rust
//! ```
//! # #[cfg(feature = "doctest-only")]
//! happenstance_testkit::event_store_conformance!(MemoryEventStore::new());
//! ```
```

`doctest-only` is not a declared feature of this crate — there is no `[features]` section in its
`Cargo.toml` — so the cfg is permanently false and the macro is never expanded. The second example
(`lib.rs:64-71`) is wrapped in `macro_rules! ignore`, so it is also never compiled. Both "pass"
(`2 passed`), and neither compiles a single line of the API they document. Given CLAUDE.md's own
rule — *"Doctests are documentation that cannot rot — prefer a runnable example to a described
one"* — this is a self-inflicted violation in the crate whose whole job is checking claims. It is
also trivially fixable: `happenstance` (with `memory`) and `tokio` are already dev-dependencies, so

```rust
//! ```
//! # #[tokio::main(flavor = "current_thread")] async fn main() {
//! use happenstance::MemoryEventStore;
//! happenstance_testkit::rules::query_all_matches_every_event(MemoryEventStore::new).await;
//! # }
//! ```
```

compiles, runs, and would catch a change to a rule's signature.

**`read`'s opaque return type captures the `&Query` lifetime.** Not a testkit bug, but it directly
blocks rules 24 and 25. This does not compile:

```rust
let stream = store.read(&Query::all(), ReadOptions::new());
//                       ^^^^^^^^^^^^ E0716: temporary value dropped while borrowed
```

Under RPITIT, the opaque type captures *every* in-scope lifetime by default, so the stream borrows
the `Query` as well as the store — even though `MemoryEventStore`'s concrete `Snapshot` borrows
nothing. Every rule in the suite calls `collect(store.read(&q, opts))` inline, so this is invisible
to all 27.

The obvious Rust fix is unavailable: precise capturing (`+ use<'_>`) **cannot be used in an RPITIT**
that has `Self` in scope. I verified both failure modes on 1.97.1 —

```
error[E0799]: `Self` can't be captured in `use<...>` precise captures list, since it is an alias
error: `impl Trait` must mention all type parameters in scope in `use<...>`
```

— which is a genuine rustc limitation with no workaround today. The two options that do work:

1. **Take the query by value**: `fn read(&self, query: Query, options: ReadOptions) -> impl Stream<…>`.
   Verified to free the borrow. `Query` is a `Box<[QueryItem]>`; one clone per read is nothing next
   to the I/O, and every adapter translates it to SQL immediately anyway.
2. **Name the stream as a GAT**: `type Stream<'a>: Stream<Item = …> where Self: 'a;` plus
   `fn read(&self, …) -> Self::Stream<'_>`. Full control, and adapters can name their type — but
   `trait_variant` cannot add `+ Send` to an associated type, so the two flavours would have to be
   written by hand, which cuts against ADR-0001's "defined once".

I would take option 1 before publishing. It is free now and it is what makes
`tokio::spawn(async move { … store.read(query, opts) … })` write naturally.

---

## What is right, and should not be relitigated

- **The suite exists at all, is a public crate, and is invoked by the reference store.**
  `tests/memory_conformance.rs` running the suite against `MemoryEventStore` is the right structure:
  it validates the oracle and the suite simultaneously, and the file says so.
- **Rules are `pub async fn` taking a factory, separate from the macro that wraps them.** That is
  what makes the registry in §4 a ten-line change rather than a rewrite, and it lets an adapter
  author call one rule directly while debugging. Good instinct, already half-executed.
- **"Compare against positions the store actually assigned, never against literal 1/2/3"**
  (`suite.rs:189-191`, CLAUDE.md). This is the single most important design rule in the suite, it is
  observed everywhere it matters, and my model-test design is just its generalisation.
- **`AppendError::ConditionViolated` lifted out of the adapter's error type**, with a rule asserting
  it (`condition_rejection_is_reported_as_condition_violated`). The distinction between "retry" and
  "something broke" belongs in the type system and it is there.
- **The `after`-boundary matrix.** Four rules covering exclusive-at-boundary / rejects-beyond /
  ignores-non-matching / rejects-any-without-`after` is the right decomposition, and
  `condition_after_ignores_events_at_the_boundary` correctly anchors on `append`'s returned position.
- **The suite binds `EventStore`, not `SendEventStore`.** I built a genuinely `!Send` store
  (`Rc<RefCell<…>>`, `!Send` stream) and all 27 rules pass under `#[tokio::test]`, whose default
  `current_thread` flavour does not require `Send`. The claim in `lib.rs:42-48` is true and now
  measured.
- **The feature and workspace-dependency wiring** (`Cargo.toml:22-25`, testkit `Cargo.toml:13-19`).
  `default-features = false` on the internal dependency, with the comment explaining that a member
  cannot remove a workspace default, is the correct and non-obvious answer.
- **`futures-core` only, `collect` hand-written** (`store.rs:162-184`) rather than pulling
  `futures-util` into the contract crate.
- **The stub `Pending` stream in `happenstance-sqlite`** (`event_store.rs:82-96`) so `read`'s
  signature is type-checked against the trait before the body exists. Small, and exactly right.
