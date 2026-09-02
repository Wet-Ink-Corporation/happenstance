---
item: HS-S0038
stage: spec
created: 2026-08-12T13:46:36.717Z
updated: 2026-08-12T13:46:36.717Z
template_sig: 87bbf1d0
rendered_sig: c48c46e4
---

# Spec — The real read stream, lazy, paged, and snapshot-bounded

## Scope lock

| Layer | Path |
| --- | --- |
| Initiative (gold source) | [`.bklg/from-contract-to-published-library/initiative.md`](../../initiative.md) |
| Initiative decomposition | [`.bklg/from-contract-to-published-library/_decomposition.md`](../../_decomposition.md) |
| Project | [`.bklg/from-contract-to-published-library/sqlite-durable-store/project.md`](../project.md) |
| This spec | `.bklg/from-contract-to-published-library/sqlite-durable-store/lazy-read-with-snapshot-ceiling/spec.md` |
| Key briefs | [`../_decomposition.md`](../_decomposition.md) — **architecture brief §4** (the read path: the ceiling, its composition with `to`, `resume_from`'s inclusivity, "lazy means lazy"), **§2** (the Accepted atoms that bind), **§3** (the runtime seam this story is the first caller of), **§5** (where chunking stops being this story's), **§11** (what is deliberately left to the implementer, including `PAGE_SIZE`); **testing brief §2** (the AC-to-tier matrix), **§3** (what may not be doubled) |
| Signed-off design | [`../_design.md`](../_design.md) — records **N/A, no user-facing surface** for this whole project, approved 2026-08-12. It binds this story to no surface; what survives it is the obligation that every public Rust item this story touches carries real rustdoc, `# Errors` naming conditions rather than types |
| Story map row | [`../_storymap.md`](../_storymap.md) — slice `durable-event-store`, archetype `capability`, `depends_on: schema-migration-and-identity`, `traces_to: AC-001`, and the coverage note that this story *carries the largest rule family* |
| Grounding | [`../_grounding.md`](../_grounding.md) — the Accepted atoms and existing code patterns this project was grounded against |
| Roadmap pointer | `RUNBOOK.md:4166-4237` (phase 8 in full); `crates/happenstance-sqlite/src/event_store.rs:11-32` (why the stream is a hand-written state machine, in the crate's own words) |

## One-line PR slice

Replace the read path's `todo!()`s with a still-lazy paged `SqliteReadStream` that captures ADR-0011's position ceiling no later than the first `poll_next`, composes it with `ReadOptions::to` and `backwards`, and keeps `resume_from` inclusive.

## Executive summary

`ReadCursor::fetch_page` is `todo!("SQLite event store: page query …")`
(`crates/happenstance-sqlite/src/event_store.rs:330-343`) and `head` is
`todo!("SQLite event store: head")` (`:226-235`). Everything around them is
already written and already right: the state machine
(`:371-426`), the direction-aware `advance()` (`:345-368`), the deferred spawn
(`:388-398`) and the doc comment explaining why no borrowing `rusqlite` handle
may become a field (`:248-271`). This PR fills the two holes and adds **the one
field the skeleton is missing** — a position ceiling on `ReadCursor` — which
ADR-0011 requires of any adapter that issues more than one statement per `read`,
and which at `PAGE_SIZE = 512` (`:81`) is every log over 512 events.

**Delta, not restatement.** `project.md` says the real `read` is in scope and the
architecture brief §4 says what it must hold. Neither says what a *reviewer looks
at*. This story's answer is a second real `cargo test` target,
`crates/happenstance-sqlite/tests/read.rs`, that seeds a real file with real SQL,
drives `SendEventStore::read` under a real tokio runtime across **more than two
pages**, and races a second `rusqlite::Connection` against a half-drained stream
to prove the ceiling is a ceiling. It is the first place in the workspace where a
paged read is exercised at all: `MemoryEventStore` snapshots under the lock and
issues one statement, so ES-11's multi-statement clause has never been executed
by anything but a mutant.

Three obligations this story adds that the briefs imply and no artifact yet
states. **`head` lands here**, because the ceiling *is* `SELECT max(position)` and
two stories writing that query is two spellings of one fact — and because
`head_is_the_highest_visible_position` is a read-side observability rule.
**The empty store is a named trap, not an edge case**: `head()` on an empty store
is `None` (ES-30), and a ceiling computed as arithmetic on that bound errors or
panics on a store whose only fault is being new — `NullHeadPagingStore`
(`crates/happenstance-testkit/tests/mutation_coverage.rs:769`) is the registered
adapter that does exactly this, so the failure is already falsifiable and merely
needs not to be committed. And **the multi-page proof may not buy itself a
test-only page-size knob**: the paging path the conformance suite will run has to
be the one this story's tests run, so the tests seed past `2 × PAGE_SIZE` rows
rather than shrinking the constant behind a `cfg`.

## Context pack

Read this section and you can start. Everything below the Integration contract is
either a boundary or a signposted anchor.

### The decision this story discharges, and it is not a design choice

**ADR-0011 is Accepted and this story is its discharge.**
`.kb/decisions/0011-read-laziness-and-isolation.md` states it in one sentence:
*"an adapter issuing more than one statement per `read` must capture a position
ceiling no later than the first poll and bound every later statement by it."*
`ReadCursor` (`crates/happenstance-sqlite/src/event_store.rs:307-328`) has no
such field. Adding one is required, not discretionary, and the architecture brief
§4 says so in the same words.

The ceiling **is one field and one extra `WHERE` term** — not a rewrite of
`advance()`, whose budget and direction arithmetic is already correct
(`:345-368`). Capture it on the *first* `fetch_page`, under the same connection
lock acquisition that selects the first page's rows, before those rows are
selected. Every subsequent page statement carries it as a bound.

**Why it is a ceiling at all** is ES-10 / ADR-0013
(`.kb/decisions/0013-position-assignment-and-visibility.md`): position order is
visibility order, so nothing that commits after *H* is captured can ever land at
or below *H*. That is what makes one cheap `max(position)` equivalent to a
snapshot, and it is why ES-11 records that **ES-11 and ES-12 reduce to ES-10 plus
a ceiling** (`spec/SPECIFICATION.md:2955-2957`). A store that re-samples per page
instead grows under the caller's feet, and the consequence is not a wrong read —
it is an **accepted append that should have been rejected**, because the caller
derives its `AppendCondition`'s boundary from the maximum position the read
observed, and that maximum sits above an event the read silently missed
(`crates/happenstance-testkit/src/suite.rs:5592`, the rule's own assertion
message).

**ES-12 costs nothing extra.** All items of one `Query` share one snapshot
because they share the same ceiling predicate — ADR-0011's finding is that ES-12
is discharged by ES-11's ceiling rather than by a second mechanism
(`spec/SPECIFICATION.md:3017-3024`). Do not build a second mechanism for it.

### Laziness is load-bearing, and it is already in the code

`read` executes nothing today and must still execute nothing after this PR
(`crates/happenstance-sqlite/src/event_store.rs:203-215`). This is not a
performance nicety: `EventStore::read` is deliberately **not** `async` so the
`Send` flavour can mark the *stream* `Send` rather than the future that produces
it (ADR-0001, ADR-0008; `CLAUDE.md` constraint 3), which means `read` runs on
whatever thread called it, possibly outside any runtime — and `spawn_blocking`
**panics** there. The spawn is therefore deferred into `poll_next`, which by
definition runs under an executor. Laziness is what makes the two constraints
compatible at all, and the module doc at `:21-25` says so.

Two consequences the implementer inherits rather than decides:

- **The ceiling is captured at the first poll, not at `read`.** ES-11 permits the
  sample to be taken *at or before* the first poll and a caller may not depend on
  which (`spec/SPECIFICATION.md:2941-2944`); here it cannot be taken before,
  because `read` may not touch SQLite. `read_result_is_stable_under_concurrent_append`
  polls once *before* it appends precisely so that a deferring adapter and a
  buffering one are both conformant (`crates/happenstance-testkit/src/suite.rs:5592`).
- **Failures arrive as `Err` items on the stream, not out of `read`.** That is
  what a lazy stream's contract already promises, and it is why
  `SqliteEventStoreError::NoRuntime` exists rather than a panic
  (`crates/happenstance-sqlite/src/event_store.rs:27-32`, `:172-178`).

### The runtime seam — consume ADR-0022's answer, do not re-open it

Architecture brief §3 is the largest architectural risk in this project and it is
**settled by ADR-0022 before this story starts**: where does `spawn_blocking` get
its runtime when the caller is a bare OS thread? The brief recommends **(a)**
capture a `tokio::runtime::Handle` at construction, prefer it, keep
`Handle::try_current()` as the fallback — and records **(b)** run inline on the
calling thread when no runtime is found — as the alternative that lost, with the
consequence that (b) obliges deleting `NoRuntime` and rewriting the module-doc
paragraph in the same change.

This story is the **first caller** of that seam: `poll_next`'s
`Handle::try_current()` at `:390-393` is the exact line the decision governs.
Take ADR-0022's answer verbatim. If ADR-0022 chose (b), the `NoRuntime`
error-condition row in this spec is void and the variant dies here rather than
being left as documentation of a state that cannot occur. **If ADR-0022 has not
landed, stop and say so** — inventing this answer here is how the concurrency
family gets discovered in a red run three stories later.

### The composition rules — where a read gets its bounds wrong

Four bounds are live at once and they are not interchangeable. `ReadOptions`
carries `from`, `to`, `backwards` and `limit`
(`crates/happenstance-core/src/query.rs:267-289`).

- **The ceiling composes with `to`; it does not replace it.** Forwards, the
  effective upper bound is the *tighter* of the caller's `to` and the ceiling.
  Under `backwards`, `from` stays the starting (higher) bound and `to` the
  stopping (lower) one — they swap roles in position order, not in meaning
  (`query.rs:279-282`) — and the **ceiling bounds the starting end**, so a
  backwards read begins at the tighter of `from` and *H*. ES-16 is `[FROZEN]`
  (`spec/SPECIFICATION.md:3231-3277`) and names the shape that gets this wrong:
  `WHERE position <= ?` copied verbatim into the descending branch, which is
  *correct* forwards, passes two of the three rules, and returns the oldest
  events where the newest were asked for.
- **`to` is inclusive in both directions.** `read_to_is_inclusive`
  (`crates/happenstance-testkit/src/suite.rs:1190`),
  `read_from_and_to_bound_a_closed_window` (`:1227`) and
  `read_to_under_backwards_bounds_the_older_end` (`:1271`) are the three rules
  that notice.
- **`from` is a threshold, not a seek.** It need not name an event that exists;
  a read from an unoccupied position yields the next matching event above it (or
  below, backwards) rather than erroring or coming back empty (`query.rs:270-277`,
  ES-9 at `spec/SPECIFICATION.md:2748`). `AUTOINCREMENT` guarantees gaps exist
  after a delete, so this is reachable, not theoretical
  (`read_from_a_gap_position`, `suite.rs:1490`).
- **`limit` is a whole-read budget applied after filtering, never per page and
  never per query item.** `Some(0)` yields nothing — a deliberate divergence from
  the DCB reference implementation's JavaScript falsiness (`query.rs:333-346`).
  `read_limit_applies_after_filtering` (`suite.rs:1052`),
  `read_backwards_limit_applies_after_filtering` (`:1082`),
  `limit_applies_across_items_not_per_item` (`:1402`) and
  `read_limit_zero_yields_nothing` (`:1327`) are the four rules.

### `resume_from` stays inclusive, and the field name records a real bug

`ReadCursor::resume_from` is **inclusive**, the same sense as `ReadOptions::from`,
which is what seeds it (`crates/happenstance-sqlite/src/event_store.rs:313-324`).
It was once `resume_after`, seeded from an inclusive `from` and advanced to
`last.position`: two senses in one field, so page two re-read the last row of page
one, once per page boundary. `AppendCondition::after` is the exclusive one in this
contract and the two sit two types apart. **Do not tidy them into one sense.**
`advance()` already steps strictly past the last row in a direction-aware way and
already treats "no position left in this direction" as spent rather than wrapped
(`:345-368`); the ceiling does not change that arithmetic.

The observable failure this protects against is precise and quiet: a duplicated
row at every 512-row boundary, or — with the opposite mistake — a dropped one.
Neither is visible in a store with fewer than `PAGE_SIZE` events, which is every
test anyone writes casually.

### The empty store, and the trap ES-11 names by name

`head()` on an empty store is `None` (ES-30, `spec/SPECIFICATION.md:3941`). A
ceiling computed as *arithmetic* on that bound errors or panics on a store whose
only fault is being new — the state every adapter is in on its first run. ES-11
records this as its one already-documented failure mode
(`spec/SPECIFICATION.md:2958-2965`); `reading_an_empty_store_yields_nothing`
(`crates/happenstance-testkit/src/suite.rs:1002`) is the rule and
`NullHeadPagingStore` (`crates/happenstance-testkit/tests/mutation_coverage.rs:769`)
is the registered adapter that commits it. A ceiling of `None` means the read is
spent, not that it failed.

### `head` lands in this story, and it is never a cached field

`head` is `todo!()` (`crates/happenstance-sqlite/src/event_store.rs:226-235`) and
the schema story explicitly left it to "the slice-mates". It belongs here: the
ceiling is the same `SELECT max(position)` query, and writing it twice in one
slice is two spellings of one fact. Its own comment already carries ADR-0013's
corollary and it is binding — **`head` must never become a field this store caches
and `append` updates**, because one file backs several handles and a second
connection would then report a head that predates the first connection's commit,
which is the stale head ES-30 exists to reject
(`.kb/decisions/0013-position-assignment-and-visibility.md`). `NULL` on an empty
table is the `None` arm, not an error.

Two distinctions worth holding: the ceiling is captured **inside the cursor's own
lock acquisition**, not by calling `head()` back through the port — that would be
a second lock and a second sample. And `head_advances_across_two_handles`
(`suite.rs:1855`) is a rule about the *fresh* query, so a memoised head passes
every single-handle test and fails only once the fixture arrives, two stories
later.

### Where this story stops and the next one starts

`wide-query-chunked-not-refused` (HS-S0039) owns the decomposition of a wide
`Query` into `ceil(arms / N)` prepared statements and the k-way merge over their
cursors (architecture brief §5). This story owns **the read that a single chunk
performs**: the page SQL, the ceiling, the bounds, the budget and the decode. Two
things follow, and both are stated so the seam is deliberate rather than
discovered:

- Write the page query so a *merge* can be layered over it without rewriting it:
  each statement ordered by position in the read's direction, bounded by the same
  ceiling and the same `resume_from`. That is exactly what the brief says the
  chunked statements must be.
- Do **not** apply the page budget or `ReadOptions::limit` inside a per-arm
  statement in a way the next story cannot lift to the merged output. Applying
  a budget per chunk is the defect `limit_applies_across_items_not_per_item`
  catches.

The brief's §5 decision also binds here: **no new public API lands in
`happenstance-core` for this adapter's convenience.** `Query::index_arms()`, which
`RUNBOOK.md:4203-4206` names as a work item, **does not exist** — `query.rs` has
`Query::items()` (`:204`), `QueryItem::types()` and `QueryItem::tags()`, and
nothing mints the rest. Decompose privately or not at all.

### The other parallel slice-mate, and the one thing you will share with it

`append-atomicity-and-store-limits` (HS-S0037) lands in parallel and writes the
rows this story reads. The encoding of a stored row — event type, canonical tags,
payload, metadata, the origin pair and `recorded_at` — is **one codec used by both
directions**, not two. Whichever story lands first authors it as a crate-private
module; the second adopts it rather than writing a second spelling. A decode that
disagrees with the encode by one byte produces a store that passes
`append_preserves_event_payload` and fails
`append_preserves_event_type_and_tags_byte_for_byte`, and the diagnosis costs a
day.

This story's test target seeds through that codec on a real connection. It must
**not** depend on `append` having landed — the two stories are independent in the
merge order (`../_storymap.md`, *Merge order* 2) — and it must **not** hand-roll a
second encoding in the test file, which would be exactly the double the testing
brief §3 forbids.

### The persona-journey slice this realizes

Two journeys, and this story is the seam between them
(`.bklg/from-contract-to-published-library/initiative.md:245-252`). *"Learn when
you are finished"* is the adapter author's loop, and this story hands the
conformance suite the largest single family of rules it has been unable to run
against a real database — query semantics, read options, positions and head. But
the fear this story actually retires belongs to the **application author**, whose
stated fear is *"discovering a contract defect in production — particularly
through a second, independent reader building a wrong answer from a torn or
gapped log"* (`initiative.md:203-208`). A paged read with no ceiling **is** that
torn log, and until this PR the workspace has had no adapter that could produce
one.

### Boundaries the implementer will be tempted to cross

- **Do not make `read` do work.** A `head()` call, a prepared-statement cache
  warm-up, a connection lock — any of them turns `read` into a function that
  panics outside a runtime and breaks ADR-0001's whole reason for the shape.
- **Do not put a `Statement`, `Rows` or `Transaction` in a field.** All three
  borrow the connection and all three are `!Send`; the state machine's doc
  comment at `crates/happenstance-sqlite/src/event_store.rs:248-271` explains
  that confining them to the inside of the blocking closure is not a style
  choice. `tests/shapes.rs::read_stream_is_send_and_unpin` is the guard.
- **Do not shrink `PAGE_SIZE` behind a `cfg(test)` to make multi-page tests
  cheap.** Its final value is deliberately the implementer's (architecture brief
  §11) and may change; what may not happen is the tested paging path differing
  from the shipped one.
- **Do not assert on literal positions** (`[1, 2, 3]`). The specification permits
  gaps and `AUTOINCREMENT` produces them; compare against positions the store
  actually assigned (`CLAUDE.md`, *The rule that matters*).
- **Do not add a timeout, watchdog or retry anywhere.** CF-33 — there is none in
  the suite by design (`crates/happenstance-testkit/src/concurrency.rs:24-43`),
  and a concurrent-read test that hangs is a finding about the busy timeout, not
  something to paper over.
- **No clause marker moves and no ADR authoring.** ES-11 and ES-12 stay
  `[PROVISIONAL]`; their falsifier is the transport axis, which
  `postgres-and-neon-stores` owns, not this adapter. Verdicts on ES-35, CF-14 and
  CF-17 are `reopen-negative-control-and-durability-verdicts`'.
- **`publish = false` stays**, `PUBLISHABLE` in `xtask/src/package.rs` stays, and
  `#![allow(clippy::todo)]` stays — DR-01 puts its deletion in the same change as
  the *last* `todo!()`, which is in the projection store.

## Integration contract

- **Archetype**: `capability` — a user-observable slice through every layer of
  this crate: the port method, the SQL, the decode, and a real test target that
  executes all three against a file.
- **Slice / milestone**: `durable-event-store`. Slice-mates, implemented in one
  context and mounted as one integrated surface:
  `schema-migration-and-identity` (predecessor),
  `append-atomicity-and-store-limits` (parallel),
  `wide-query-chunked-not-refused` (successor),
  `sqlite-fixture-and-whole-suite` (which closes the slice).
- **Mount point**: `crates/happenstance-sqlite/tests/read.rs` — a **new, real
  `cargo test` target**, run by `cargo test -p happenstance-sqlite` inside
  `cargo xtask affected --base main` and `cargo xtask ci --fast`. It is this
  story's composition root because the slice's ultimate root,
  `crates/happenstance-sqlite/tests/conformance.rs`, cannot exist yet:
  `event_store_conformance!` drives `append` as well as `read`, and needs the
  `SqliteFixture` that `sqlite-fixture-and-whole-suite` builds. This target is
  **not** retired when `conformance.rs` arrives — it asks three questions the
  conformance suite cannot: whether a read crosses more than two *pages*
  correctly, whether the ceiling survives a write from a genuinely separate
  `rusqlite::Connection` mid-drain, and whether `read` itself is inert outside a
  runtime. Architecture brief AC-A01 is honoured in substance: reachable from
  `tests/`, never from a `mod` only `cargo check` sees.
- **Wires into**:
  - `crates/happenstance-core/src/store.rs` — `SendEventStore::read` / `head`,
    the port surface this story implements. Bind `EventStore` in any generic
    helper, never `SendEventStore` (`CLAUDE.md` constraint 4).
  - `crates/happenstance-core/src/query.rs:190-230, 267-346` — `Query::items()`,
    `QueryItem::types()`/`tags()`/`matches()`, and the four `ReadOptions` fields
    with their inclusivity documented on each. The page SQL is derived from
    these and from nothing invented.
  - `crates/happenstance-sqlite/src/event_store.rs:198-216, 226-235, 307-343,
    345-368, 371-426` — `read`, `head`, `ReadCursor` and `fetch_page`,
    `advance()`, and the `poll_next` state machine. The two `todo!()`s this story
    fills and the one field it adds.
  - `crates/happenstance-sqlite/tests/shapes.rs` — the existing type-level guard.
    It must still pass, unchanged in intent, once `ReadCursor` carries a ceiling
    and (per ADR-0022) possibly a `tokio::runtime::Handle`.
  - `crates/happenstance-core/src/memory.rs:296-336` — the reference read: it
    filters, orders and truncates under the read lock, which is the *other* legal
    shape. Read it for the semantics, not for the mechanism; the mechanism here
    is the opposite one.
  - The crate-private row codec shared with `append-atomicity-and-store-limits`
    (see the Context pack) — one module, two directions.
- **Renders surfaces**: **none.** `../_design.md` records N/A — no user-facing
  surface — for this entire project, approved at the design sign-off gate, and
  `design.capture` is deliberately absent from `.redkiln/config.yaml`, which makes
  the perceptual review a declared skip rather than a silent pass. The public Rust
  items this story completes (`SendEventStore::read`, `head`, `SqliteReadStream`)
  are library surface inside that determination and each carries rustdoc with an
  `# Errors` section naming conditions rather than types
  (`standards/rust/70-rustdoc-obligations.md`).
- **Conformance rule(s)**: this story adds **no** conformance rule and changes no
  port. What it makes runnable — and what will observe it once
  `sqlite-fixture-and-whole-suite` mounts the fixture — is the largest family in
  the registry (`crates/happenstance-testkit/src/registry.rs:108-146, 212-217`):
  the twelve **Query semantics** rules (`query_all_matches_every_event`,
  `suite.rs:375`; `query_item_tags_match_supersets`, `:439`;
  `untagged_events_match_query_all`, `:605`;
  `duplicate_items_do_not_duplicate_events`, `:636`; and their siblings), the
  fifteen **Read options** rules (`read_from_is_inclusive`, `:883`;
  `read_backwards_reverses_order`, `:912`;
  `reading_an_empty_store_yields_nothing`, `:1002`;
  `read_limit_applies_after_filtering`, `:1052`;
  `read_backwards_limit_applies_after_filtering`, `:1082`;
  `read_to_is_inclusive`, `:1190`; `read_from_and_to_bound_a_closed_window`,
  `:1227`; `read_to_under_backwards_bounds_the_older_end`, `:1271`;
  `read_limit_zero_yields_nothing`, `:1327`;
  `limit_applies_across_items_not_per_item`, `:1402`; `read_from_a_gap_position`,
  `:1490`), the three **Head** rules (`:1731`, `:1798`, `:1855`), and the two
  **Read isolation** rules that are this story's headline —
  `read_result_is_stable_under_concurrent_append` (`:5592`) and
  `query_items_share_one_snapshot` (`:5696`) — plus
  `a_live_read_stream_does_not_block_an_append` (`:5504`) and
  `nothing_below_an_observed_position_appears_later` (`:5880`). Because the
  fixture is a slice-mate's, this story proves the same behaviours in
  `tests/read.rs` against a real file; the rules are what will keep them true.
- **Clause(s)**: discharges **ES-11** (`spec/SPECIFICATION.md:2926-2996`,
  `[PROVISIONAL]`) and **ES-12** (`:2998-3036`, `[PROVISIONAL]`) *on this
  adapter* — this is the first multi-statement `read` in the workspace, so it is
  the first implementation that could fail them. It honours **ES-16**
  (`:3231-3277`, `[FROZEN]`), **ES-9** (`:2748`), **ES-10** (`:2816`),
  **ES-13**, **ES-14** and **ES-30** (`:3941`). **No `[FROZEN]` clause is
  amended, no marker is edited and no ADR is authored in this PR** — ES-11's and
  ES-12's provisional markers are held open by the *transport* axis, whose far
  end is `postgres-and-neon-stores`', not this adapter's.
- **Advances DoD scenario**: initiative **DoD 3** — *"The durable store passes the
  suite for real"* (`.bklg/from-contract-to-published-library/initiative.md:365-368`).
  This story is the half of DoD 3 that makes the read side runnable at all;
  project **AC-001** (*the suite runs, whole*) is the row it traces to.

## PR boundary

```
crates/happenstance-sqlite/src/**
crates/happenstance-sqlite/tests/read.rs
.bklg/from-contract-to-published-library/sqlite-durable-store/lazy-read-with-snapshot-ceiling/**
```

**In this PR**

- A real `ReadCursor::fetch_page`: one page of rows selected under the connection
  lock, ordered by position in the read's direction, bounded by the ceiling, by
  `resume_from` and by `ReadOptions::to`, and decoded into `SequencedEvent`s.
- **The ceiling**: one new field on `ReadCursor`, captured on the first
  `fetch_page` under the same lock acquisition as the first page, and carried as a
  bound on every subsequent page statement.
- A real `SendEventStore::head`, queried fresh every call, `None` on an empty
  table, never memoised.
- The crate-private row codec's **decode** half — event type, canonical tags,
  payload, metadata, origin pair and `recorded_at` back out of migration 1's
  columns — shared with `append-atomicity-and-store-limits` rather than duplicated.
- Consumption of ADR-0022's runtime-seam answer at
  `crates/happenstance-sqlite/src/event_store.rs:388-398`: a captured `Handle` if
  the ADR chose (a), or the inline path plus the deletion of `NoRuntime` and its
  module-doc paragraph if it chose (b). Not a re-decision.
- `crates/happenstance-sqlite/tests/read.rs` — the new target that executes all of
  the above against a real temporary file: multi-page drains past
  `2 × PAGE_SIZE` rows, both directions, a concurrent append from a second
  connection mid-drain, a multi-item query, an empty store, a gap position, and a
  `read` built outside any runtime.
- Documentation corrections that this PR makes true or false: the `PAGE_SIZE` doc
  comment (`:76-81`) if the value moves, and the `read`/`head` rustdoc where it
  describes behaviour that is now real.

**Explicitly not in this PR**

- `append`, the store limits and the encode half of the row codec —
  `append-atomicity-and-store-limits` (HS-S0037).
- Wide-query decomposition into `ceil(arms / N)` statements and the k-way merge —
  `wide-query-chunked-not-refused` (HS-S0039). This story serves the single-chunk
  read and leaves the merge a clean layer above it.
- `SqliteFixture`, `tests/conformance.rs`, and any testkit macro invocation —
  `sqlite-fixture-and-whole-suite`.
- `contains_event_id`, migration SQL, pragmas, the `StoreId` — landed by
  `schema-migration-and-identity` (HS-S0036) and consumed here unchanged.
- The concurrency family, `CONTENDERS`, and the mutant registry —
  `concurrency-family-and-contender-count`, `model-family-and-mutant-pass-column`.
- Deleting `#![allow(clippy::todo)]` or rewriting `lib.rs`'s status banner —
  `instrument-markers-removed-and-gate-green` (DR-01).
- Any `spec/SPECIFICATION.md` edit, any `[PROVISIONAL]` marker move, any ADR
  authoring, any `Cargo.toml` change and any new dependency.

The implementer **may** also touch the wiring named in the Integration contract —
`crates/happenstance-sqlite/tests/shapes.rs` if a new field needs a new assertion,
and the shared row-codec module — to mount this slice. That is not scope drift.

**Merge DoD one-liner** — `cargo xtask affected --base main` green with
`crates/happenstance-sqlite/tests/read.rs` draining a real multi-page read from a
real file, the ceiling proven by a second connection's append landing outside a
half-drained stream, `tests/shapes.rs` still passing unchanged in intent, and no
`todo!()` removed outside `fetch_page` and `head`.

## Behavior and interfaces

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| **`read` still executes nothing** | No SQL, no lock, no `head()` call, no runtime lookup at call time. Building a stream outside any runtime returns a stream; the absence of a runtime surfaces on the first poll, as an `Err` item, never a panic. | `crates/happenstance-sqlite/src/event_store.rs:203-215`, `:21-32`; ADR-0001 / ADR-0008 via `CLAUDE.md` constraint 3 |
| **A position ceiling is captured no later than the first poll** | One new field on `ReadCursor`, set during the first `fetch_page` under the same connection lock acquisition that selects the first page, before those rows are selected. Never re-captured. | `.kb/decisions/0011-read-laziness-and-isolation.md`; `../_decomposition.md` architecture brief §4; against `crates/happenstance-sqlite/src/event_store.rs:307-328`, which has no such field |
| **Every statement after the first is bounded by it** | `position <= H` forwards, `>= H` under `backwards`. One extra `WHERE` term; `advance()`'s arithmetic is unchanged. | `spec/SPECIFICATION.md:2946-2952`; `crates/happenstance-sqlite/src/event_store.rs:345-368` |
| **The ceiling is sound because position order is visibility order** | Nothing committing after *H* can land at or below *H*, which is why one `max(position)` is equivalent to a snapshot. ES-11 and ES-12 reduce to ES-10 plus a ceiling. | `.kb/decisions/0013-position-assignment-and-visibility.md`; `spec/SPECIFICATION.md:2955-2957` |
| **All items of one `Query` share that one ceiling** | ES-12 is discharged by the same predicate, not by a second mechanism. A per-item statement evaluated at a later poll picks up a later event and tears. | `spec/SPECIFICATION.md:3017-3024`; `crates/happenstance-testkit/src/suite.rs:5696` |
| **The ceiling composes with `to`, and does not replace it** | Forwards: effective upper bound is the tighter of `to` and *H*. Backwards: `from` stays the higher starting bound, `to` the lower stopping bound, and *H* bounds the **starting** end. Both inclusive. | `crates/happenstance-core/src/query.rs:279-282, 318-330`; ES-16 `[FROZEN]`, `spec/SPECIFICATION.md:3231-3277` |
| **`resume_from` stays inclusive** | Same sense as `ReadOptions::from`, which seeds it. `advance()` steps strictly past the last row, direction-aware, and treats "no position left this way" as spent rather than wrapping. Page boundaries neither repeat nor drop a row. | `crates/happenstance-sqlite/src/event_store.rs:313-324, 345-368` |
| **`from` is a threshold, not a seek** | A read from a position nothing occupies yields the next matching event above (or below) it. `AUTOINCREMENT` makes gaps real. | `crates/happenstance-core/src/query.rs:270-277`; ES-9, `spec/SPECIFICATION.md:2748`; `crates/happenstance-testkit/src/suite.rs:1490` |
| **`limit` is a whole-read budget applied after filtering** | Never per page and never per query item; `Some(0)` yields nothing; the page budget (`min(remaining, PAGE_SIZE)`) is an implementation detail beneath it. | `crates/happenstance-core/src/query.rs:333-346`; `crates/happenstance-testkit/src/suite.rs:1052, 1082, 1327, 1402` |
| **An empty store yields nothing, and does not error** | `head()` is `None` on an empty table (ES-30); a ceiling computed as arithmetic on that bound is the registered defect. `None` ceiling means the read is spent. | `spec/SPECIFICATION.md:2958-2965`, `:3941`; `crates/happenstance-testkit/tests/mutation_coverage.rs:769` (`NullHeadPagingStore`) |
| **Query semantics are served by the page SQL** | Types within an item are OR; tags within an item are AND with superset matching; items are OR; duplicate items do not duplicate events; `Query::All` matches untagged events. Derived from `Query::items()` / `QueryItem::types()` / `tags()` — no new `happenstance-core` API. | `crates/happenstance-core/src/query.rs:190-230`; `crates/happenstance-testkit/src/suite.rs:375, 439, 605, 636`; `../_decomposition.md` architecture brief §5 (option (b), adapter-private) |
| **`head` is a fresh query, every time** | `SELECT max(position)`; `NULL` is the `None` arm, not an error. **Never** a cached field `append` updates — one file backs several handles, and a stale head is what ES-30 rejects. | `crates/happenstance-sqlite/src/event_store.rs:226-235`; `.kb/decisions/0013-position-assignment-and-visibility.md`; `crates/happenstance-testkit/src/suite.rs:1731, 1798, 1855` |
| **Failures arrive as `Err` items and the stream is then terminal** | A poison, a driver error, an invalid stored position or a missing runtime becomes one `Err` item; the state machine already replaces state with `Done` on the error paths. No panic escapes a poll. | `crates/happenstance-sqlite/src/event_store.rs:371-426`, `:144-193` |
| **The runtime seam is ADR-0022's answer, consumed** | Either a captured `Handle` preferred over `Handle::try_current()`, or the inline path **with** `NoRuntime` and its module-doc paragraph deleted in the same change. Not both, and not a fresh decision. | `../_decomposition.md` architecture brief §3; `crates/happenstance-sqlite/src/event_store.rs:388-398` |
| **The shape guard still holds** | `SqliteReadStream: Send + Unpin`, `SqliteEventStore: Send + Sync`, `SqliteEventStoreError: Error + Send + Sync + 'static` after the cursor gains fields. No `Statement`, `Rows` or `Transaction` in any field — all three borrow the connection and all three are `!Send`. | `crates/happenstance-sqlite/tests/shapes.rs`; `crates/happenstance-sqlite/src/event_store.rs:248-271` |
| **Both flavours still type-check** | The bare `EventStore` and the `trait_variant`-derived `SendEventStore` both, with `read` returning the stream at the top level and not nested in a future. Generic helpers bind `EventStore`. | `CLAUDE.md` constraints 3 and 4; `crates/happenstance-sqlite/tests/shapes.rs` (`send_impl_satisfies_the_bare_bound`) |
| **The multi-page path tested is the shipped one** | Multi-page criteria seed strictly more than `2 × PAGE_SIZE` rows so at least three page statements run. No `cfg(test)` page-size seam, because a knob makes the shipped paging path untested. | `crates/happenstance-sqlite/src/event_store.rs:76-81`; `../_decomposition.md` architecture brief §11 (`PAGE_SIZE`'s value is the implementer's) |
| **No literal positions anywhere** | Assertions compare against positions the store actually assigned. The specification permits gaps and `AUTOINCREMENT` produces them after a delete. | `CLAUDE.md`, *The rule that matters*; `crates/happenstance-sqlite/src/event_store.rs:56-58` |
| **Nothing is doubled** | The test target seeds through the shared row codec on a real `rusqlite::Connection` against a real file. No fake driver, no second hand-rolled encoding, no in-memory stand-in for the paging path. | `../_decomposition.md` testing brief §3 |

## Data and migrations

**N/A — this story creates, alters and drops nothing.** Migration 1 is
`schema-migration-and-identity`'s (HS-S0036) and lands before this story starts;
this PR is the first *reader* of what it created. What follows is the read-side
contract on that schema, recorded here because a column this story reads by a
name it invents is a defect nothing else would catch.

**Columns this story reads, and what it owes each.**

| Object | What the read path takes from it | The wrong shape it must not adopt |
| --- | --- | --- |
| `event.position` | The ordering key, the ceiling's `max()`, and every bound — `from`, `to`, `resume_from`, *H* | Treating position arithmetic as a count, or assuming `+1` between adjacent rows. `AUTOINCREMENT` leaves gaps and the specification permits them |
| `event.event_type`, `event.tags` | The filter, and the decode back into `EventType` / `Tags`. Both are validated types: a stored value that no longer validates is `StoredEventType` / `StoredTag`, not a panic | Re-normalising or re-sorting tags on read. `Tags` is canonically sorted at write; two tags differing only by Unicode normalisation are distinct and must stay so |
| `event.data`, `event.metadata` | Opaque `Bytes` returned byte-for-byte; `metadata` distinguishes absent from empty | Coercing `NULL` metadata to an empty blob, which collapses two states the contract keeps apart |
| `event.origin_store`, `event.origin_position` | The `EventId` the decode reconstructs via `EventId::new` | Reconstructing an id from the local `StoreId` rather than the stored origin pair — correct for locally-appended events, wrong for every ingested one |
| `event.recorded_at` | Returned **as stored**, never re-stamped on read | A read that stamps `now()` — the defect `recorded_time_survives_a_reopen` exists to reject |
| `event_tag` (key `(tag, position)`, covering `event_type`) | Tag matching, answerable without leaving the index; the range stays sorted by position | Joining back to `event` to satisfy a type constraint. That is the amendment's entire purpose, and undoing it here re-creates the wrong plan the schema story fixed |
| `tag_cardinality` | Choosing the most-selective tag first for a multi-tag arm | Ignoring it and probing an arbitrary tag, which is correct and slow |

**Migration consequence.** A column rename or a key-order change after this PR is
a **migration 2**, not an edit: the encode half, the decode half and the page SQL
would all move together. Name what you read once, here, against the names
migration 1 actually created — read them out of `sqlite_master`, not out of the
module-doc block, if the two ever disagree.

**Reversibility.** Nothing to reverse: no schema object is added, no stored byte
is rewritten, and the crate is `publish = false`.

## Acceptance criteria

Ten criteria. Each is a *persona goal crossing the whole crate* — port method,
SQL, decode, and a target that executes all three against a real file — not a
capability restated. The two personas are the initiative's own
(`.bklg/from-contract-to-published-library/initiative.md:203-214`,
`_discovery/distillation/personas-and-journeys.md`): the **application author**,
whose stated fear is *a second, independent reader building a wrong answer from a
torn or gapped log*, and the **adapter author**, whose journey is *"learn when you
are finished"*. Every verification names a real test path; where a conformance
rule will later observe the same behaviour through `SqliteFixture`
(`sqlite-fixture-and-whole-suite`), it is named too — it is the rule that keeps
the behaviour true, not this story's proof of it.

| id | criterion | verification |
| --- | --- | --- |
| AC-001 | **Given** an application author holding a store handle on a bare OS thread — the shape ADR-0001 exists to keep possible — **when** they call `read` and have not yet polled the returned stream, **then** no SQL has run, no connection lock has been taken and no runtime has been looked up; **and when** they poll that stream with no tokio runtime present, the absence arrives as exactly one `Err` item on the stream and nothing panics. | `crates/happenstance-sqlite/tests/read.rs::read_executes_nothing_until_polled` (a stream built while a sentinel connection holds the lock; the build completes) and `::read_polled_outside_a_runtime_yields_an_error_item` — or, if ADR-0022 chose the inline path, `::read_polled_outside_a_runtime_runs_inline`, with `NoRuntime` deleted in the same change. Type-level backstop: `crates/happenstance-sqlite/tests/shapes.rs::send_impl_satisfies_the_bare_bound`. |
| AC-002 | **Given** an application author replaying a log far longer than one page, **when** they drain a `read` over a store seeded with strictly more than `2 × PAGE_SIZE` events so at least three page statements run, **then** the stream yields every matching event exactly once in ascending position order — no row repeated at a page boundary and none dropped — and the drain never buffers the whole log. | `crates/happenstance-sqlite/tests/read.rs::a_multi_page_drain_repeats_and_drops_nothing` — seeds through the shared row codec on a real file, compares against the positions the store actually assigned (never literals), and asserts the drained sequence equals the seeded one under both `backwards: false` and `true`. No `cfg(test)` page-size knob: the paging path tested is the shipped one. |
| AC-003 | **Given** the application author's stated fear of a torn log, **when** a genuinely separate `rusqlite::Connection` appends to the same file while their stream is half-drained, **then** none of those events appear in that stream, every position it yields is at or below the ceiling captured no later than its first poll, and a *fresh* `read` issued after the drain does see them. | `crates/happenstance-sqlite/tests/read.rs::a_concurrent_append_mid_drain_is_not_observed` — drain one page, append from a second connection, drain the rest, assert the maximum observed position is below the appended ones and that a subsequent read observes them. Kept true afterwards by `read_result_is_stable_under_concurrent_append` (`crates/happenstance-testkit/src/suite.rs:5592`) and `nothing_below_an_observed_position_appears_later` (`:5880`), which this story makes runnable rather than runs. |
| AC-004 | **Given** an adapter author with a multi-item `Query` — the shape a real decision model produces — **when** the read is drained across pages while a matching event lands concurrently, **then** every item is answered against the *same* ceiling, so no item observes an event another item could not, and the store implements no second isolation mechanism to achieve it. | `crates/happenstance-sqlite/tests/read.rs::query_items_share_the_one_ceiling` — a two-item query drained across a page boundary against a concurrent append matching only one item; the read is all-or-nothing on the new event. Code review checks a single ceiling predicate, not a per-item sample. Kept true by `query_items_share_one_snapshot` (`crates/happenstance-testkit/src/suite.rs:5696`). |
| AC-005 | **Given** an application author asking for *the newest events before position P*, **when** they set `backwards` with `from` and `to`, **then** they get the newest-first window they asked for: `from` is the higher starting bound and `to` the lower stopping one, both inclusive, the ceiling bounds the **starting** end rather than replacing `to`, and a closed forward window `[from, to]` returns exactly its endpoints and everything between. | `crates/happenstance-sqlite/tests/read.rs::bounds_compose_in_both_directions` — four assertions: forward closed window inclusive at both ends; backwards window inclusive at both ends and newest-first; backwards start clamped to the tighter of `from` and the ceiling; a `to` above the ceiling narrowed to the ceiling rather than ignored. The wrong shape it must reject is ES-16's named one (`spec/SPECIFICATION.md:3231-3277`): `WHERE position <= ?` copied into the descending branch. Kept true by `read_to_under_backwards_bounds_the_older_end` (`suite.rs:1271`), `read_backwards_reverses_order` (`:912`), `read_from_and_to_bound_a_closed_window` (`:1227`). |
| AC-006 | **Given** an adapter author on their **first** run — the store is empty, which is the state every adapter is in before it works — **when** they read it, **then** they get an empty stream and no error; **and given** a log with gaps, which `AUTOINCREMENT` guarantees after a delete, **when** they read `from` a position nothing occupies, **then** they get the next matching event above it (or below, backwards) rather than an error or an empty result. | `crates/happenstance-sqlite/tests/read.rs::an_empty_store_yields_nothing_and_does_not_error` and `::read_from_a_gap_position_yields_the_next_event` — the second deletes a row to make a real gap. The registered defect this rejects is `NullHeadPagingStore` (`crates/happenstance-testkit/tests/mutation_coverage/mutants.rs:1125`), a ceiling computed as arithmetic on a `None` head. Kept true by `reading_an_empty_store_yields_nothing` (`suite.rs:1002`) and `read_from_a_gap_position` (`:1490`). |
| AC-007 | **Given** an application author rebuilding a decision model who wants only the first *n* matching events, **when** they set `limit`, **then** they get at most *n* events counted **after** filtering and **across** all query items — never *n* per item and never *n* per page — and `Some(0)` yields nothing at all rather than everything. | `crates/happenstance-sqlite/tests/read.rs::limit_is_a_whole_read_budget` — a limit that spans a page boundary, a limit smaller than one page, a limit over a multi-item query, and `Some(0)`, each against a store seeded past `2 × PAGE_SIZE` so the per-page budget is genuinely exercised. Kept true by `read_limit_applies_after_filtering` (`suite.rs:1052`), `read_backwards_limit_applies_after_filtering` (`:1082`), `limit_applies_across_items_not_per_item` (`:1402`), `read_limit_zero_yields_nothing` (`:1327`). |
| AC-008 | **Given** an application author deriving an `AppendCondition`'s boundary from what the store says it has, **when** they call `head` — on an empty store, on a populated one, and from a *second* handle onto the same file after the first handle appended — **then** they get `None`, the highest visible position, and the position the other handle just wrote, respectively; the value is queried fresh every call and is never a field the store caches. | `crates/happenstance-sqlite/tests/read.rs::head_is_a_fresh_query_across_two_handles` — `None` on a new file, the assigned maximum after a seed, and the updated value from a second `SqliteEventStore::open` on the same path. Code review confirms no memoised field. Kept true by `head_is_the_highest_visible_position` (`suite.rs:1798`) and `head_advances_across_two_handles` (`:1855`). |
| AC-009 | **Given** an adapter author who wrote an event and now reads it back, **when** the page SQL filters and the codec decodes, **then** they get *their* event: types within an item OR'd, tags within an item AND'd with superset matching, items OR'd, duplicate items not duplicating events, `Query::All` matching untagged events — and the payload, metadata (absent distinguished from empty), canonically-ordered tags, `EventId` reconstructed from the **stored origin pair**, and `recorded_at` **as stored** all coming back byte-for-byte. | `crates/happenstance-sqlite/tests/read.rs::query_semantics_are_served_by_the_page_sql` and `::a_decoded_row_matches_what_was_written_byte_for_byte` — both seeded through the row codec shared with `append-atomicity-and-store-limits`, never a second hand-rolled encoding. Reference semantics: `crates/happenstance-core/src/memory.rs:296-336`. Kept true by the twelve Query-semantics rules (`crates/happenstance-testkit/src/registry.rs:108-146`), e.g. `query_all_matches_every_event` (`suite.rs:375`), `query_item_tags_match_supersets` (`:439`), `untagged_events_match_query_all` (`:605`), `duplicate_items_do_not_duplicate_events` (`:636`). |
| AC-010 | **Given** an adapter author reading this crate as the worked example of the port — the "learn when you are finished" journey — **when** they open the completed read path, **then** the type-level obligations still hold after the cursor gained a ceiling (`SqliteReadStream: Send + Unpin`, `SqliteEventStore: Send + Sync`, no `Statement`/`Rows`/`Transaction` in any field, both port flavours type-checking with the stream at the top level) **and** every public item this story completes carries real composed rustdoc — a summary, the laziness and ceiling contract stated where a caller meets it, and an `# Errors` section naming the *conditions* rather than the error types. | `crates/happenstance-sqlite/tests/shapes.rs` passing unchanged in intent — `read_stream_is_send_and_unpin`, `store_is_send_and_sync`, `send_impl_satisfies_the_bare_bound`, `send_flavour_stream_is_send_in_generic_code`, `read_returns_the_named_stream_type` — plus `cargo doc -p happenstance-sqlite` and `cargo clippy -p happenstance-sqlite -- -D warnings` clean under the crate's `missing_docs` posture, reviewed against `standards/rust/70-rustdoc-obligations.md`. Both run inside `cargo xtask affected --base main`. |

Every criterion above serves project **AC-001** (*the suite runs, whole*) — the
single row this story traces to (`../_storymap.md`, Coverage: *"read carries the
largest rule family"*). No other project AC is claimed here.

## Interaction quality

This project renders **no user-facing surface**: `../_design.md` records N/A for
the whole project, approved 2026-08-12, and `design.capture` is deliberately
absent from `.redkiln/config.yaml:75-83`, which makes the perceptual review a
declared skip rather than a silent pass. The invariants below are therefore the
library-medium reading of RFC §6.7/D6 — the same two families, against the
surface a `cargo add` user actually meets. **Every one of them is already an
`AC-###` row in the table above**; this section only says which row carries which
invariant, and how it is verified.

**STATE invariants.**

| Invariant (library reading) | Carried by | How it is verified |
| --- | --- | --- |
| **In-place, not a context jump** — building a stream does not move the caller onto a runtime, take a lock, or run a query. `read` is where the caller is, not somewhere else. | **AC-001** | `read.rs::read_executes_nothing_until_polled` |
| **Non-occlusion** — a failure never occludes the caller's control flow as a panic; it arrives as one `Err` item on the stream the caller is already holding. | **AC-001** (surfacing), **EC-001 – EC-006** (each condition) | `read.rs::read_polled_outside_a_runtime_yields_an_error_item`; the state machine's error arms (`crates/happenstance-sqlite/src/event_store.rs:400-414`) |
| **Preserved position across a boundary** — the reader's place in the log survives a page boundary exactly: `resume_from` stays inclusive, `advance()` steps strictly past the last row, and nothing repeats or drops. This is the library analogue of preserved scroll/selection and it is the invariant a casual test cannot see. | **AC-002** | `read.rs::a_multi_page_drain_repeats_and_drops_nothing` |
| **Stability under concurrent change** — the view does not grow under the reader's feet mid-drain; the ceiling is captured once and never re-sampled. | **AC-003**, **AC-004** | `read.rs::a_concurrent_append_mid_drain_is_not_observed`, `::query_items_share_the_one_ceiling` |
| **Reversibility / no side effects** — a `read` that is built and dropped un-polled leaves the database untouched, and a stream that errored is terminal rather than half-consumed. | **AC-001**, **EC-002 – EC-004** | `read.rs::read_executes_nothing_until_polled`; state replaced with `Done` on every error arm |
| **Reachability** — every behaviour above is reachable from a real `cargo test` target a reviewer can run, never from a `mod` only `cargo check` sees (architecture brief AC-A01). | **AC-001 – AC-010** | `cargo test -p happenstance-sqlite --test read` inside `cargo xtask affected --base main` |

**COMPOSITION invariants.** `../_design.md` declares no surfaces, so there is no
composition, transience, density or hierarchy budget to inherit — and inventing
one here would contradict a signed-off determination. What the sign-off explicitly
leaves standing is the documentation obligation, and it is the exact analogue of
"presentation exists at all": an API that type-checks and is undocumented is the
unstyled render.

| Invariant | Carried by | How it is verified |
| --- | --- | --- |
| **Presentation exists at all** — every public item this story completes (`SendEventStore::read`, `head`, `SqliteReadStream`, the new cursor state) carries real rustdoc, not a restated signature; `# Errors` names the *conditions* a caller can act on, never the error type list. | **AC-010** | `cargo doc -p happenstance-sqlite`, `cargo clippy -- -D warnings`, reviewed against `standards/rust/70-rustdoc-obligations.md` |
| **Hierarchy** — the laziness contract and the ceiling are stated where a caller meets them (on `read`, on the stream type), not only in the module doc; the module doc keeps the *why*, the item docs carry the *what*. | **AC-010** | Review against `crates/happenstance-sqlite/src/event_store.rs:11-32` (module doc) and the item docs this PR writes |
| **Named anti-patterns hold** — the shape the crate documents as forbidden stays forbidden: no borrowing `rusqlite` handle in a field, no cached `head`, no `cfg(test)` page-size knob, no literal position assertions. | **AC-002**, **AC-008**, **AC-010** | `tests/shapes.rs` unchanged in intent; review against `event_store.rs:248-271`; `CLAUDE.md`, *The rule that matters* |
| **Documentation that became false is corrected in the same change** — `PAGE_SIZE`'s "placeholder until it is measured" comment (`:76-81`) and any `read`/`head` doc describing behaviour that is now real. | **AC-010** | Diff review; `cargo xtask lints` |

## Error conditions

| id | condition | required behaviour | evidence |
| --- | --- | --- | --- |
| EC-001 | The stream is first polled with no tokio runtime available. | Per ADR-0022. If it chose (a): one `Err(SqliteEventStoreError::NoRuntime)` item, then the stream is terminal; never the panic `spawn_blocking` would raise. If it chose (b): the page runs inline on the polling thread, **and** `NoRuntime` and its module-doc paragraph are deleted in this same change — this row is then void rather than left as documentation of an unreachable state. | `crates/happenstance-sqlite/src/event_store.rs:172-178`, `:388-398`; `../_decomposition.md` architecture brief §3 |
| EC-002 | The connection mutex is poisoned by a panicking thread. | `Err(SqliteEventStoreError::ConnectionPoisoned)` as one stream item; no `unwrap()` on the guard anywhere on the read path; the stream is terminal afterwards. | `crates/happenstance-sqlite/src/event_store.rs:158-166`, `:333-336` |
| EC-003 | `rusqlite` fails mid-page — I/O, `SQLITE_BUSY`, a malformed statement. | `Err(SqliteEventStoreError::Sqlite(_))` as one stream item; the state machine has already replaced state with `Done`, so no partial page is emitted after it. **No retry loop and no busy-timeout added here** — the timeout is migration 1's and ADR-0022's. | `event_store.rs:154-156`, `:400-414`; testing brief §4 |
| EC-004 | The `spawn_blocking` task panics or is cancelled. | `Err(SqliteEventStoreError::Worker(JoinError))`; nothing is silently treated as an empty page, which would present a truncated log as a complete one. | `event_store.rs:168-170`, `:405` |
| EC-005 | A stored `event_type` or tag no longer validates against the current rules. | The dedicated stored-value variants (`StoredEventType` / `StoredTag`, already declared in this module) as an `Err` item — never a panic and never a silent skip, because skipping a row turns a validation change into a torn log. | `event_store.rs` (`SqliteEventStoreError` variants); ADR-0015 via `.kb/decisions/0015-validated-identifiers-and-store-limits.md` |
| EC-006 | A stored `position` is not a valid `SequencePosition` (zero, negative, or out of range). | An `Err` item, not a panic and not a `saturating` coercion. `SequencePosition` is `NonZeroU64`; a `0` in the column is corruption, and reporting it as position 1 is worse than failing. | `event_store.rs:56-58`, `:357` |
| EC-007 | The store is empty, so the ceiling query returns `NULL`. | **Not an error.** A `None` ceiling means the read is spent: the stream ends immediately. Arithmetic on the `None` head — the `NullHeadPagingStore` defect — is what this row exists to forbid. | `spec/SPECIFICATION.md:2958-2965`, `:3941`; `crates/happenstance-testkit/tests/mutation_coverage/mutants.rs:1125` |
| EC-008 | `ReadOptions` name a window that cannot contain anything (`from` above `to` forwards, or below it backwards), or a `from` above the ceiling. | An empty stream, not an error and not an unbounded scan. An impossible window is a legitimate question with the answer "nothing". | `crates/happenstance-core/src/query.rs:267-289` |

## Non-functional

| id | requirement | why it is here, and how it is checked |
| --- | --- | --- |
| NF-001 | **Bounded memory.** A drain of a log of any size holds at most one page of decoded rows plus the cursor. No `Vec` accumulates across pages. | The entire reason `read` returns a stream rather than a `Vec` (`event_store.rs:76-81`). Checked by review of the state machine plus AC-002's `> 2 × PAGE_SIZE` drain, which would be indistinguishable from buffering only if the implementation buffered. |
| NF-002 | **A live read stream does not hold the connection lock between pages.** The lock is acquired inside `fetch_page` and released when it returns; an `append` from the same process must be able to make progress while a stream is parked in `Draining`. | This adapter serialises writers by construction (`event_store.rs:85-91`); holding the lock across the whole drain would turn "serialises writers" into "a long read blocks all writers". Kept true by `a_live_read_stream_does_not_block_an_append` (`crates/happenstance-testkit/src/suite.rs:5504`) once the fixture lands. |
| NF-003 | **No new dependency and no `Cargo.toml` change.** The test target uses a process-local temp path and `Drop` cleanup, per architecture brief §11 — not a new crate. | `git diff` over `crates/happenstance-sqlite/Cargo.toml` is empty; `cargo deny` (whole gate) unchanged. |
| NF-004 | **No timeout, watchdog or retry anywhere on this path**, in the code or in the tests. CF-33 forbids a rule reading a clock, and a hang here is a finding about the busy timeout, not something to paper over. | `crates/happenstance-testkit/src/concurrency.rs:24-43`; testing brief §4. Checked by review of the diff for `Duration`, `timeout`, `sleep` and retry loops. |
| NF-005 | **Tag matching is answered from the covering index.** The page SQL uses `event_tag` keyed `(tag, position)` with `event_type` as a covering column and consults `tag_cardinality` to probe the most selective tag of a multi-tag arm first; it does not join back to `event` to satisfy a *type* constraint. | That covering column is the entire point of ADR-0022's schema amendment, and undoing it here re-creates the plan the schema story fixed. Reviewed against migration 1 as `schema-migration-and-identity` landed it. |
| NF-006 | **The gate the project is held to stays green**, including the four wasm32 steps and the two doc builds, unaffected by this PR. | `cargo xtask ci --fast` (`.redkiln/config.yaml:55`) — the project's integration grain, not this story's bar, but it must not be broken by this story. |
| NF-007 | **The tested paging path is the shipped one.** No `cfg(test)` or feature-gated `PAGE_SIZE`. Its final value stays the implementer's (architecture brief §11); what may not differ is which constant the tests exercise. | Review of `event_store.rs:76-81` for `cfg` attributes; AC-002 seeds past the real constant. |

## Implementation notes (non-prescriptive)

These are observations, not instructions. The architecture brief §11 leaves the
mechanism to the implementer and this section does not take it back.

- **The ceiling is one `Option<SequencePosition>` field and one `WHERE` term.**
  A shape that works: `ceiling: Option<SequencePosition>` on `ReadCursor`,
  `None` meaning *not yet captured*; the first `fetch_page` takes the lock, runs
  `SELECT max(position) FROM event`, stores the result, and if it is `NULL`
  returns an exhausted empty page. Every later page adds `AND position <= ?H`
  (or `>= ?H` backwards). Distinguishing *not yet captured* from *captured as
  empty* matters: conflating them re-samples on every page, which is the defect.
- **Consider capturing it inside the same statement batch as the first page**
  rather than as a separate round trip. Both are one lock acquisition, which is
  what ES-11 requires; the second is simpler to read and the difference is a
  measurement nobody has taken. Do not optimise it on a guess.
- **`advance()` needs no change.** Its direction-aware step and its
  "no position left this way means spent" arm already compose with a ceiling. If
  you find yourself editing it, check first whether the ceiling belongs in the
  `WHERE` clause instead.
- **The page budget and `ReadOptions::limit` are two different numbers.**
  `min(remaining, PAGE_SIZE)` is already computed at `:337`; keep `limit` as the
  cursor's `remaining` and let the page budget be derived from it, never the
  reverse. Applying `limit` inside a per-arm statement is what
  `limit_applies_across_items_not_per_item` catches, and it is also what would
  make `wide-query-chunked-not-refused`'s merge impossible to layer on.
- **Write the page query so a merge can sit above it.** One statement, ordered by
  position in the read's direction, bounded by the ceiling and `resume_from`. The
  next story wraps `ceil(arms / N)` of exactly this in a k-way merge; if that
  requires rewriting the statement, this story chose wrong.
- **Decode is half of one codec.** Whichever of this story and
  `append-atomicity-and-store-limits` lands first authors the crate-private
  module; the second adopts it. A round-trip test inside that module is cheaper
  than diagnosing a one-byte disagreement through a conformance failure.
- **Seeding past `2 × PAGE_SIZE`** — currently 1,025 rows — is fast on a real
  file inside one transaction. Seed through the codec, in one `BEGIN`, and reuse
  the helper across every multi-page criterion rather than re-seeding per test.
- **`head` and the ceiling share a query, not a call.** The cursor computes its
  ceiling inside its own lock acquisition; it does not call `head()` back through
  the port, which would be a second lock and a second sample. Factor the SQL, not
  the call.
- **If ADR-0022 has not landed, stop and say so.** The runtime seam at
  `:388-398` is its decision, and inventing an answer here is how the concurrency
  family gets discovered in a red run three stories later.

## Tests and CI (merge gate)

Tiers are the testing brief's (§1) and the commands are the repository's own,
wired from `.redkiln/config.yaml` rather than typed fresh
(testing brief AC-T02). This story's bar is the **story grain**; the project
grain (`cargo xtask ci --fast`) belongs to the slice, and the whole gate to
`closeout-and-durable-audience`.

| Tier | Command / path | Proves |
| --- | --- | --- |
| Static | `cargo xtask lints && cargo xtask spec-trace` (`reachability_static`, `.redkiln/config.yaml:48`) | No clause marker moved and no citation count fell; the five file-reading lints still pass. This story amends no clause, so the citation count must be **unchanged**, not merely non-falling. |
| Static | `cargo fmt --check`, `cargo clippy -p happenstance-sqlite -- -D warnings` (inside `affected`) | The rustdoc obligation (AC-010) and the crate's lint posture, including that `#![allow(clippy::todo)]` is still scoped and still needed — two `todo!()`s leave this crate, several remain. |
| Unit / type-level | `cargo test -p happenstance-sqlite --test shapes` | AC-010's type obligations after `ReadCursor` gains fields: `SqliteReadStream: Send + Unpin`, the store `Send + Sync`, both flavours, the stream at the top level. The guard that catches a `Statement` or `Rows` added to a field. |
| Story integration (this story's mount) | `cargo test -p happenstance-sqlite --test read` — `crates/happenstance-sqlite/tests/read.rs` | AC-001 – AC-009 against a real temporary file through a real `rusqlite::Connection`: multi-page drains past `2 × PAGE_SIZE`, both directions, a concurrent append from a second connection mid-drain, a multi-item query, an empty store, a gap position, limits, `head` across two handles, and a `read` built outside any runtime. Nothing doubled (testing brief §3). |
| Story merge gate | `cargo xtask affected --base main` (`affected_gate`, `:40`) | The whole of the above for every package this diff touches plus their dependents, from staged, unstaged **and** untracked files. This is the command that must be green before the story advances. |
| Doc build | `cargo doc -p happenstance-sqlite` (inside `affected` / `ci --fast`) | AC-010: the rustdoc compiles, intra-doc links resolve, and any doctest written on `read` or `head` runs. |
| Conformance (deferred, named so it is not mistaken for a gap) | `cargo test -p happenstance-sqlite --test conformance` — `crates/happenstance-sqlite/tests/conformance.rs` | Owned by `sqlite-fixture-and-whole-suite`. It is where the ~32 Query-semantics / Read-options / Head / Read-isolation rules this story makes runnable are actually run. This story does **not** invoke `event_store_conformance!` and must not stub a fixture to fake one. |
| Mutation coverage (not this story's) | `cargo xtask ci --fast` → `mutation_coverage::every_rule_has_a_mutant`, `::conformant_variants_pass_everything` (`xtask/src/proof.rs:86-90`) | That the rules relied on above *can* fail. `NullHeadPagingStore` already registers this story's headline defect; no registry row is added here. |
| Project integration (not this story's bar) | `cargo xtask ci --fast` (`integration_scoped`, `:55`) | The slice's bar. Must not be broken by this story; is not claimed green by it. |

**Ledger obligation.** `require_ledger: true` (`.redkiln/config.yaml:67`): every
AC-001 – AC-010 row in `_ledger.md` carries cited evidence — a `file:line` and/or
the verifying test id — before this story advances. A green gate is a
precondition for reading the criteria, never a substitute
(`RUNBOOK.md:38-42`, testing brief AC-T05).

## Risks and coupling (PR-scoped)

| Risk | Blast radius | Mitigation, in this PR |
| --- | --- | --- |
| **ADR-0022 has not landed, or landed without the runtime-seam paragraph.** The seam at `:388-398` is unanswered and the implementer invents one. | The whole `race-model-and-durability` slice: `NoRuntime` from every contender is a red concurrency family two stories later, and the fix is a rewrite of the spawn path. | Named as a **stop condition** in the Context pack, not a judgement call. If the atom does not answer it, halt and report rather than choose. |
| **The row codec is written twice**, once here for decode and once in `append-atomicity-and-store-limits` for encode. | A one-byte disagreement that passes `append_preserves_event_payload` and fails `append_preserves_event_type_and_tags_byte_for_byte` — a day to diagnose, in a story that did not cause it. | One crate-private module, authored by whichever story lands first and adopted by the second. AC-009 seeds *through* it, so a divergence fails here rather than in the fixture story. |
| **The ceiling is re-sampled per page** because "it is only a `max(position)`". | The exact defect ES-11 exists to reject, and it is invisible to every sequential test: a torn read that leads a caller to derive an `AppendCondition` boundary above an event it never saw — an accepted append that should have been rejected. | AC-003 and AC-004 both require a genuinely separate connection appending mid-drain. A single-connection test cannot fail this way, which is why the test target must open a second `rusqlite::Connection`. |
| **The multi-page path is never actually exercised**, because every test seeds fewer than 512 events. | Page-boundary duplication or loss ships and is found by an application author replaying a real log — the persona failure this story exists to prevent. | AC-002 and AC-007 both require `> 2 × PAGE_SIZE`; NF-007 forbids shrinking the constant to make that cheap. |
| **`backwards` gets `WHERE position <= ?` copied from the forward branch.** | ES-16 is `[FROZEN]`; the store returns the oldest events where the newest were asked for while passing two of the three read-options rules. | AC-005 asserts all four compositions explicitly, including a `to` above the ceiling under `backwards`. |
| **Scope creep into the merge.** The wide-query decomposition is tempting once the page SQL is in hand. | `wide-query-chunked-not-refused` loses its slice and its test target; the merge order in `../_storymap.md` breaks. | The PR boundary lists it under *Explicitly not in this PR*, and the implementation notes state the seam positively: write the statement so a merge can sit above it, and stop. |
| **`head` memoised as a field** because it is queried on every page. | Passes every single-handle test and fails only once `SqliteFixture` arrives, two stories later, against `head_advances_across_two_handles`. | AC-008 requires a second `SqliteEventStore::open` on the same path in this story's own target, so the failure is local. |
| **A hang under a busy connection is papered over with a timeout.** | CF-33 violated, and the busy-timeout evidence ADR-0022 needs is destroyed. | NF-004; the diff is reviewed for `Duration`, `sleep`, `timeout` and retry loops. |

**Coupling, stated.** Upstream: migration 1's column names and index shape
(`schema-migration-and-identity`) — read them out of `sqlite_master` if the
module doc and the schema ever disagree. Sideways: the row codec
(`append-atomicity-and-store-limits`). Downstream: the page statement's shape
(`wide-query-chunked-not-refused`) and the fixture that runs the real suite
against it (`sqlite-fixture-and-whole-suite`). Nothing outside
`crates/happenstance-sqlite/` changes.

## Dependencies

**Blocks on**

- `schema-migration-and-identity` (HS-S0036) — hard. This story is the first
  *reader* of migration 1: the `event` columns, the `event_tag` index keyed
  `(tag, position)` with its covering `event_type`, `tag_cardinality`, the
  origin pair and `recorded_at`. Without it there is no table to page over and
  no `StoreId` to reconstruct an `EventId` against.

Transitively (through that story, not re-declared here):
`adr-0022-append-condition-strategy` → `benchmark-harness`. ADR-0022 is
nonetheless a **direct stop condition** for this story's runtime seam, as the
Context pack states — it is a dependency of the *content*, satisfied by the
predecessor's own dependency edge.

**Runs in parallel with**

- `append-atomicity-and-store-limits` (HS-S0037) — independent in the merge order
  (`../_storymap.md`, *Merge order* 2). Shares exactly one artifact: the
  crate-private row codec. This story must not wait for it and must not
  hand-roll a second encoding in its test file.

**Unlocks**

- `wide-query-chunked-not-refused` (HS-S0039) — declares this story as its
  `depends_on`; layers `ceil(arms / N)` statements and a k-way merge over the
  single-chunk read this story defines.
- `sqlite-fixture-and-whole-suite` (HS-S0040) — needs `read` and `head` real
  before `event_store_conformance!` can be green whole; this story supplies the
  largest rule family it will run.

## Anchors (progressive disclosure)

Load-bearing depth, deferred rather than pasted. Open each at the moment named —
not before, and not "if it seems relevant". Every path was confirmed present in
this worktree.

| Anchor | Why it is load-bearing | When to open | Serves |
| --- | --- | --- | --- |
| `.kb/decisions/0011-read-laziness-and-isolation.md` | The Accepted decision this story discharges: the ceiling is required of any adapter issuing more than one statement per `read`, and it names *at or before the first poll* as the permitted window. Its rejected alternatives are why re-sampling per page is not a cheaper equivalent. | **Before writing `fetch_page`** — first thing, ahead of any SQL. | AC-003, AC-004 |
| `.kb/decisions/0013-position-assignment-and-visibility.md` | Why one `max(position)` is equivalent to a snapshot at all: position order **is** visibility order, so nothing committing after *H* can land at or below it. Also the source of the "`head` is never a cached field" corollary. | **Before deciding where the ceiling is captured**, and again before writing `head`. | AC-003, AC-008 |
| `spec/SPECIFICATION.md` — ES-9 (`:2748`), ES-10 (`:2816`), ES-11 (`:2926-2996`), ES-12 (`:2998-3036`), ES-16 (`:3231-3277`, `[FROZEN]`), ES-30 (`:3941`) | The normative text, each clause carrying its maturity marker, the conformance rule that checks it and the wrong implementation it forbids. ES-16's forbidden shape is the exact copy-paste this story is most likely to commit; ES-11's is the empty-store arithmetic. | **ES-11/ES-12 before the ceiling; ES-16 before the `backwards` branch; ES-9/ES-30 before the empty-store and gap criteria.** | AC-003, AC-005, AC-006 |
| `crates/happenstance-testkit/src/suite.rs` — `:1002`, `:1052`, `:1082`, `:1190`, `:1227`, `:1271`, `:1327`, `:1402`, `:1490`, `:1731`, `:1798`, `:1855`, `:5504`, `:5592`, `:5696`, `:5880` | The rules that will observe this behaviour once the fixture lands, with their assertion messages — `read_result_is_stable_under_concurrent_append` (`:5592`) polls once *before* it appends, which is precisely what makes a deferring adapter conformant. Read the assertion text: it states the consequence in the caller's terms. | **When writing each `tests/read.rs` criterion**, to mirror the rule's shape rather than invent a weaker one. | AC-003, AC-005, AC-006, AC-007, AC-008, AC-009 |
| `crates/happenstance-core/src/query.rs` — `:190-230`, `:267-289`, `:318-346` | The four `ReadOptions` fields with their inclusivity documented on each, the role-swap under `backwards`, `Some(0)`'s deliberate divergence from the DCB reference implementation, and the only `Query` accessors that exist. `Query::index_arms()` does **not** exist and nothing here mints it. | **Before writing the `WHERE` clause and the budget arithmetic.** | AC-005, AC-007, AC-009 |
| `crates/happenstance-sqlite/src/event_store.rs` — `:11-32`, `:76-81`, `:198-235`, `:248-271`, `:307-343`, `:345-368`, `:371-426` | The code being completed, and the reasoning already written into it: why the stream is a hand-written state machine, why no borrowing handle may become a field, why `resume_from` is inclusive and what bug the rename fixed, and where the two `todo!()`s are. | **First, and continuously** — it is the file this PR edits. | AC-001, AC-002, AC-010 |
| `.bklg/from-contract-to-published-library/sqlite-durable-store/_decomposition.md` — architecture brief §3, §4, §5, §11; testing brief §2, §3, §4 | §3 is the runtime seam and its stop condition; §4 is the read path in the brief's own words; §5 is where chunking stops being this story's and why no `happenstance-core` API is added; §11 is what is deliberately the implementer's. Testing §3 names what may not be doubled. | **§3 and §4 before starting; §5 when the page SQL is in hand and the merge looks tempting; §11 before touching `PAGE_SIZE`.** | AC-001, AC-002, AC-009 |
| `crates/happenstance-testkit/tests/mutation_coverage/mutants.rs:1125` (`NullHeadPagingStore`; module doc at `:170`) | The registered adapter that commits this story's named trap — decoding `max(position)` into a shape that mishandles the empty-store `NULL`. It is a *working* wrong implementation, which is more instructive than the clause. | **Before writing the empty-store criterion**, to see the defect concretely. | AC-006 |
| `crates/happenstance-core/src/memory.rs:296-336` | The reference read: filter, order, truncate, all under the read lock. The **other** legal shape — read it for the semantics (what "matches", what order, how `limit` interacts) and explicitly *not* for the mechanism, which is the opposite of this one's. | **When resolving a semantic question** (does this event match? in what order?) rather than a mechanical one. | AC-007, AC-009 |
| `crates/happenstance-sqlite/tests/shapes.rs` | The existing type-level guard, with each assertion's reasoning in its doc comment — including why `SendEventStore` is named by full path rather than imported. It must still pass once the cursor carries a ceiling and possibly a `Handle`. | **After adding any field to `ReadCursor` or `SqliteReadStream`.** | AC-010 |
| `standards/rust/70-rustdoc-obligations.md` | The house rule for what rustdoc must contain — `# Errors` naming conditions rather than error types — which is the only composition obligation surviving `_design.md`'s no-surface determination. | **Before writing the docs on `read`, `head` and the stream type.** | AC-010 |
| `crates/happenstance-testkit/src/registry.rs:108-146, 212-217` | The rule families this story makes runnable, enumerated in one place: Query semantics, Read options, Head, Read isolation. Useful for checking that a behaviour you are about to invent already has a rule that defines it. | **When scoping `tests/read.rs`** — to cover the families, not to duplicate the suite. | AC-009 |
| `crates/happenstance-testkit/src/concurrency.rs:24-43` | CF-33 in its own words: no rule reads a clock, and there is no watchdog in the suite by design. The rationale is what stops a hang being papered over during this story's concurrent-append criterion. | **If a test hangs** — before adding anything that looks like a timeout. | AC-003 |
| `crates/happenstance-core/src/store.rs` | The port surface being implemented: `read` returning the stream at the top level and not `async`, `head`'s contract, and the `trait_variant` derivation that produces `SendEventStore`. Bind `EventStore` in generic helpers, never `SendEventStore`. | **Before writing any generic helper** in the test target. | AC-001, AC-010 |

## Clarifications resolved during spec

1. **The AC set is exactly the ten the front half enumerated** — AC-001 through
   AC-010, none added and none dropped. They partition as: the laziness contract
   (AC-001), paging correctness (AC-002), the ceiling and its two isolation
   consequences (AC-003, AC-004), bound composition (AC-005), the two "ordinary
   states that look like errors" (AC-006), the budget (AC-007), `head` (AC-008),
   query semantics and decode fidelity (AC-009), and the shape-plus-rustdoc
   surface (AC-010).
2. **`head` belongs to this story, not to the fixture story.** The story map's
   coverage row assigns AC-001 to three stories and leaves `head`'s home implicit;
   the schema story deferred it to "the slice-mates". Resolved here because the
   ceiling *is* `SELECT max(position)` and two stories writing that query is two
   spellings of one fact. Recorded so the fixture story does not write it again.
3. **This story adds no conformance rule and invokes no testkit macro.** It would
   be natural to reach for `event_store_conformance!` as soon as `read` works, but
   the macro drives `append` too and needs `SqliteFixture`, which
   `sqlite-fixture-and-whole-suite` owns. The mount is a plain `cargo test` target
   instead — and it survives the fixture's arrival, because it asks three
   questions the suite cannot: more than two *pages*, a second `rusqlite::Connection`
   mid-drain, and `read` built outside a runtime.
4. **ES-11 and ES-12 keep their `[PROVISIONAL]` markers.** This story is the first
   implementation in the workspace that could fail them, which is tempting grounds
   for freezing. It is not sufficient: what holds them open is the *transport*
   axis, whose far end is `postgres-and-neon-stores`'. No marker moves and no ADR
   is authored here.
5. **The runtime seam is consumed, not decided.** ADR-0022 owns it. If the atom
   chose the inline path, EC-001 is void and `NoRuntime` plus its module-doc
   paragraph die in this same change rather than lingering as documentation of an
   unreachable state. If ADR-0022 has not landed, this story **halts** — that is a
   stated stop condition, not a judgement call.
6. **Interaction-quality invariants are carried as AC rows, not prose.** The
   project's `_design.md` records N/A for all surfaces, so RFC §6.7/D6's
   composition family is read in the library medium: the rustdoc obligation is the
   "presentation exists at all" invariant, and it is AC-010's, in the table, where
   `redkiln verify` can extract it. The state family maps onto AC-001, AC-002,
   AC-003 and AC-004. Nothing that gates lives only in the Interaction quality
   section.
7. **`PAGE_SIZE` stays a single constant with no test-only override.** The
   temptation is real — 1,025 seeded rows per multi-page criterion — but a knob
   means the shipped paging path is untested, which is the failure mode this whole
   project exists to retire. Its *value* remains the implementer's
   (architecture brief §11); its *singularity* does not.
8. **The row codec's ownership is "first to land authors it".** Neither this story
   nor `append-atomicity-and-store-limits` can be sequenced after the other — they
   are parallel in the merge order — so ownership is stated as a rule rather than
   assigned to a slug. The second story adopts; it does not write a second
   spelling.
