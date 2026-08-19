---
item: HS-S0039
stage: spec
created: 2026-08-12T13:46:37.634Z
updated: 2026-08-12T13:46:37.634Z
template_sig: 87bbf1d0
rendered_sig: 5c27ebd6
---

# Spec — A wide Query is chunked and merged, never refused

## Scope lock

| Layer | Path |
| --- | --- |
| Initiative (gold source) | [`.bklg/from-contract-to-published-library/initiative.md`](../../initiative.md) |
| Initiative decomposition | [`.bklg/from-contract-to-published-library/_decomposition.md`](../../_decomposition.md) |
| Project | [`.bklg/from-contract-to-published-library/sqlite-durable-store/project.md`](../project.md) |
| This spec | `.bklg/from-contract-to-published-library/sqlite-durable-store/wide-query-chunked-not-refused/spec.md` |
| Key briefs | [`../_decomposition.md`](../_decomposition.md) — **architecture brief §5** (query pushdown, chunking, and the `index_arms()` question — the decision this story executes), **§4** (the read path, the snapshot ceiling, `resume_from`'s inclusive sense, `advance()`), **§1** (the mount table), **§7** (`event_tag`'s covering column and `tag_cardinality`, which is what an arm is planned against); **testing brief §2** (AC-008's row: the conformance rule *plus* a query wide enough to force `ceil(arms/N) > 1`), **§3** (a fixture is not a mock) |
| Signed-off design | [`../_design.md`](../_design.md) — records **N/A, no user-facing surface** for this whole project, approved 2026-08-12. It binds this story to no surface; what it does not license is skipping the rustdoc on anything this story makes public, and this story deliberately makes nothing public |
| Story map row | [`../_storymap.md`](../_storymap.md) — slice `durable-event-store`, archetype **capability**, `depends_on: lazy-read-with-snapshot-ceiling`, `traces_to: AC-008` |
| Grounding | [`../_grounding.md`](../_grounding.md) — `_grounding.md:193-213`, the finding that `Query::index_arms()` **does not exist** and the two options for what to do about it |
| Roadmap pointer | `RUNBOOK.md:4203-4206` — the work item, verbatim, including the `ceil(arms/400)` proposal and *"`Query` bounds nothing by design; the limit is adapter-specific and a hard cap in the contract would be wrong"* |

## One-line PR slice

Decompose a wide `Query` into `ceil(arms/N)` prepared statements **privately inside `happenstance-sqlite`** and k-way-merge their cursors, de-duplicating on position and applying the page budget and `ReadOptions::limit` to the merged output.

## Executive summary

This PR is the second half of `ReadCursor::fetch_page`
(`crates/happenstance-sqlite/src/event_store.rs:332-345`). The slice-mate
`lazy-read-with-snapshot-ceiling` makes one page statement real and bounds it by
ADR-0011's ceiling; **this story is what happens when one statement is not enough**
— when a `Query` decomposes into more index arms than SQLite will accept in a
single compiled statement. The runbook's answer is chunk and merge rather than
refuse, and this PR implements exactly that.

**Delta, not restatement.** `project.md` AC-008 states the behaviour and the
architecture brief §5 has already taken the decision that matters (decompose
*privately*, mint no public API in the frozen contract crate). What neither states
is the thing an implementer gets wrong: **the conformance suite cannot reach the
chunk boundary on its own.** VT-23's floor is 128 items, and at the suite's grain —
one tag per item — 128 arms is nowhere near SQLite's 32,766-parameter ceiling
(`crates/happenstance-core/src/limits.rs:34-44`; the same arithmetic is written into
`ChunkedQueryStore`'s own comment at
`crates/happenstance-testkit/tests/mutation_coverage/mutants.rs:2591-2601`). Ship
this with `N = 400` and every rule in the suite runs one chunk, the merge is dead
code, and the crate acquires a green suite over an unexercised path — the exact
failure mode this initiative exists to retire. So this PR owes a **new real test
target**, `crates/happenstance-sqlite/tests/wide_query.rs`, that crosses the
boundary deliberately and observes that it was crossed, and a chunk width that is
**test-visible** rather than a private literal.

The second delta is the one the specification cares about: making a read
multi-statement is precisely the shape ES-12 names as its rejected implementation
(`spec/SPECIFICATION.md:3033-3037`). This PR is therefore the story that could
break the snapshot guarantee the story before it just established — and the answer
is not a second mechanism, it is that **every chunk statement carries the same
ceiling** (`spec/SPECIFICATION.md:3020-3025`).

## Context pack

Read this section and you can start. Everything below the Integration contract is a
boundary, a table, or a signposted anchor.

### The decision this story executes, already taken

**Query decomposition is adapter-private. No new public API lands in
`happenstance-core`.** The runbook's work item names `Query::index_arms()`
(`RUNBOOK.md:4203`) and so does the evaluation
(`references/evaluation/ARCHITECTURAL-EVALUATION.md:827`) — **that API does not
exist**, and no decision atom mints it. `happenstance-core` offers
`Query::items() -> Option<&[QueryItem]>` (`crates/happenstance-core/src/query.rs:204`),
`QueryItem::types()` (`:102`) and `QueryItem::tags()` (`:107`), and those are
sufficient. The architecture brief §5 chose option (b), and its reasoning is the
repository's own rule: adding public surface to a contract crate for the benefit of
*one* adapter is one implementor, and *"a port is only as well-designed as the
spread of what implements it"* (`CLAUDE.md`). **The re-open trigger is named rather
than left to memory:** if `postgres-and-neon-stores` independently needs the same
decomposition, that is two unlike storage shapes agreeing, and *that* is the
evidence to mint `index_arms()` in `happenstance-core` — as its own ADR, not as a
side effect of this one. Do not re-litigate this here; ADR-0022 records it.

### What an arm is, and where the chunk boundary comes from

An **arm** is one prepared-statement branch derived from a `QueryItem` after the
most-selective-tag choice. `Query::All` is one arm — a bare scan over `event`. An
item's arm count is a function of its type set and its tag set, and the
most-selective-tag choice is what `tag_cardinality` (landed by
`schema-migration-and-identity`) exists to answer: SQLite cannot, because `ANALYZE`
stores only an average (`references/evaluation/ARCHITECTURAL-EVALUATION.md:827`,
where the probe-order swing is measured at 650×).

`RUNBOOK.md:4204` proposes `N = 400`. **That number is a proposal, not a
measurement.** The real ceiling is SQLite's compiled-in compound-select and
bound-parameter limits, and `rusqlite` is pulled `default-features = false,
features = ["bundled"]` (`Cargo.toml:47`), so a limits API may need a feature
added — weigh that against documenting the bundled defaults, and take whichever
ADR-0022 recorded. What is **not** available is inventing a different number here
in silence, and what is **required** is that whatever N is, this crate's own tests
can force `ceil(arms/N) > 1` without appending a hundred thousand events.

### The shape of the emitted SQL is the whole game

The evaluation measured the obvious translation — a materialised
`position IN (… UNION …)` set — at **970× slower**, and, worse for this port,
*"buffers the entire result before the first row"*
(`references/evaluation/ARCHITECTURAL-EVALUATION.md:827`). Buffering the whole
result is not a performance opinion here; it is a breach of the promise
`EventStore::read` makes (`crates/happenstance-sqlite/src/event_store.rs:76-81`) and
of the laziness ADR-0011 requires. The fast plan is bare top-level compound arms
ordered by position, merged as they stream — which is also the plan that keeps
`event_tag`'s `(tag, position)` key order alive
(`references/evaluation/ARCHITECTURAL-EVALUATION.md:828`).

### The merge is where the query rules actually get decided, not the SQL

Six named rules already in the suite fail an adapter that chunks and merges
carelessly, and they fail it for six different reasons:

- **Union, not first-chunk.** `store_evaluates_a_query_at_the_guaranteed_minimum_item_count`
  (`crates/happenstance-testkit/src/suite.rs:3880`) puts the one matching item
  **last** in a 128-item list precisely so a store that evaluates a prefix returns
  nothing. `query_items_are_or` (`:553`) and `query_union_is_item_concatenation`
  (`:780`) are the same property at the algebra's own grain.
- **Exactly once.** Arms may overlap, so the merge **must** de-duplicate on
  position — `duplicate_items_do_not_duplicate_events` (`:636`).
- **Order-free.** `query_item_order_does_not_change_the_result_set` (`:710`): the
  partition into chunks must not leak into the result order. Output order is
  position order, always (ES-8, `spec/SPECIFICATION.md:2732`).
- **Limit on the merged output, never per chunk.**
  `limit_applies_across_items_not_per_item` (`:1402`) and
  `read_limit_applies_after_filtering` (`:1052`) with its backwards mirror. ES-14
  is `[FROZEN]` and names this exact wrong adapter: *"one statement per item with
  `LIMIT n` on each"* (`spec/SPECIFICATION.md:3111-3145`).
- **One snapshot across every statement.** `query_items_share_one_snapshot`
  (`:5696`, registered at `crates/happenstance-testkit/src/registry.rs:214`).

The compiled falsifier for the headline behaviour already exists and is in the
registry: **`ChunkedQueryStore`**
(`crates/happenstance-testkit/tests/mutation_coverage/mutants.rs:2580-2617`,
registered at `crates/happenstance-testkit/tests/mutation_coverage.rs:1981`) —
*"chunk, then forget to union the chunks"*, which answers **wrong** rather than
erroring. This story adds **no** new mutant row and needs none: the rule it must
pass already has a live wrong implementation, which is what `CLAUDE.md`'s
"a rule that no adapter can fail is decorative" asks for.

### One snapshot, and why this story is the risk to it

ES-12 (`spec/SPECIFICATION.md:2998-3037`, `[PROVISIONAL]`) requires every item of
one `Query` to be evaluated against the same snapshot, and its `Rejects:` is
literally *"a one-round-trip adapter that emits one SQL statement per `QueryItem`
and unions the results client-side"*. Chunking is that shape. The discharge is
stated in the clause and repeated in the rule's own docs: **ES-12 is discharged by
ES-11's ceiling rather than by a second mechanism**
(`spec/SPECIFICATION.md:3020-3025`; `crates/happenstance-testkit/src/suite.rs:5691-5695`).
Concretely: the ceiling captured no later than the first `poll_next` by
`lazy-read-with-snapshot-ceiling` is carried into **every chunk statement of every
hop**, identically, and so is `ReadOptions::to`, `from`/`resume_from` and the
direction. An event that lands between chunk 1's statement and chunk 4's has a
position above the ceiling and is excluded by the same predicate. A chunk statement
that omits the ceiling because "it is only a filter" reintroduces the tear.

### The Rust constraint that shapes the merge state

`SqliteReadStream` is `Send + Unpin` and must stay so
(`crates/happenstance-sqlite/tests/shapes.rs`;
`crates/happenstance-sqlite/src/event_store.rs:248-271`). `rusqlite::Statement`,
`Rows` and `Transaction` are all `!Send`, so **no per-chunk cursor may become a
field**: the k live statements exist only inside the `spawn_blocking` closure, and
what crosses the `poll_next` boundary is owned data — a per-chunk resume position,
plus already-materialised `SequencedEvent`s. That is the same rule the type's
existing docs state, and it is why the merge is "k positions and a buffer", not "k
open cursors". `advance()` (`:351-368`) is already written and already correct: it
folds the **merged** page into `resume_from` and `remaining`, and per-chunk
arithmetic must never touch either.

### Refusal is not an option the contract offers

VT-23 (`spec/SPECIFICATION.md:1535-1552`) is explicit: there MUST NOT be a
`MAX_QUERY_ITEMS` constant, `Query::from_items` MUST NOT enforce a count, and every
store MUST evaluate at least 128 items. `StoreLimit` has three variants and not
four, and `crates/happenstance-core/src/limits.rs:46-52` says why in terms: *"a
query-item refusal is not an append outcome"*. So there is no error variant to
reach for, no ceiling to declare on the fixture, and the store that returns an
error at its pushdown limit is AC-008's named wrong implementation
(`../project.md` AC-008). A store MAY refuse something *larger* than the floor —
but this adapter does not, because chunking removes the reason to.

### The persona-journey slice this realizes

*"Learn when you are finished"* — the adapter author's loop, from a signature that
type-checks to a suite that says pass or fail and names why
(`.bklg/from-contract-to-published-library/initiative.md:245-246`). This story is
the one place in the project where the suite's verdict is **not** sufficient on its
own: a green VT-23 rule tells the adapter author that 128 items work, and tells them
nothing about the code path that runs at 4,000. The deliverable therefore includes
the evidence that the path was executed, which is what the testing brief §2 means by
*"plus a query wide enough to force the chunk boundary"*.

### Boundaries the implementer will be tempted to cross

- **Do not add `index_arms()`, `arm_count()` or `IndexArm` to `happenstance-core`**
  (architecture brief AC-A04). The evaluation proposes them; nothing accepted mints
  them.
- **Do not introduce `MAX_QUERY_ITEMS`, a `StoreLimit::QueryItems`, or a fixture
  ceiling for query width.** VT-23 forbids the first, `limits.rs:46-52` forbids the
  second, and CF-40's clause home is an open question this project must not settle
  in passing (`.kb/open-questions/cf-40-fixture-limits-ownership.md`).
- **Do not make `read` execute anything.** `crates/happenstance-sqlite/src/event_store.rs:203-215`
  is load-bearing: planning the arms is cheap and pure, but the *statements* are
  prepared on the blocking thread, inside `fetch_page`.
- **Do not change `advance()`, and do not "tidy" `resume_from` into an exclusive
  sense** (`:309-330` records the historical bug by name).
- **Do not reorder or deduplicate a query's *items*.** VT-31 permits an adapter to
  reorder or dedup *provided the match set is preserved*
  (`spec/SPECIFICATION.md:1860-1889`, `[FROZEN]`) — partitioning arms into chunks is
  inside that licence; collapsing two items because their type lists look alike is
  not, and `ItemDedupByTypeStore` already exists to catch it.

## Integration contract

- **Archetype**: `capability` — a user-observable slice through the whole adapter:
  a caller issues a wide `Query` through the public `EventStore::read` and gets the
  right events back instead of an error or a truncated answer.
- **Slice / milestone**: `durable-event-store`. Slice-mates, implemented in one
  context and mounted as one integrated surface: `schema-migration-and-identity`,
  `append-atomicity-and-store-limits`, `lazy-read-with-snapshot-ceiling`,
  `sqlite-fixture-and-whole-suite`.
- **Mount point**: `crates/happenstance-sqlite/tests/wide_query.rs` — a **new, real
  `cargo test` target**, run by `cargo test -p happenstance-sqlite` inside
  `cargo xtask affected --base main` and `cargo xtask ci --fast`. The production
  seam it drives is `ReadCursor::fetch_page`
  (`crates/happenstance-sqlite/src/event_store.rs:332-345`) reached through the
  public `SendEventStore::read` (`:198-215`) — never through a `mod` only
  `cargo check` sees (architecture brief AC-A01). This target is **not** made
  redundant by `crates/happenstance-sqlite/tests/conformance.rs`, the slice's
  ultimate root owned by `sqlite-fixture-and-whole-suite`: the conformance suite
  runs this code at one chunk and cannot tell you so, and this target is where the
  boundary is crossed and observed.
- **Wires into**:
  - `crates/happenstance-core/src/query.rs:150-230` — `Query::items`,
    `QueryItem::types`, `QueryItem::tags`, `Query::matches`. The only accessors the
    decomposition is allowed to use, and they are enough.
  - `crates/happenstance-core/src/limits.rs:32` — `MIN_SUPPORTED_QUERY_ITEMS`, the
    floor the wide-query test anchors on rather than a literal `128`.
  - `crates/happenstance-sqlite/src/event_store.rs:302-368` — `Page`, `ReadCursor`,
    `fetch_page`, `advance`: the exact seam this story completes, on top of the
    ceiling field `lazy-read-with-snapshot-ceiling` adds.
  - `crates/happenstance-sqlite/src/event_store.rs:34-58` + migration 1's
    `event_tag` covering column and `tag_cardinality`
    (`schema-migration-and-identity`) — what an arm is planned against.
  - `crates/happenstance-testkit/src/suite.rs:3880` and the query/limit rules at
    `:553`, `:636`, `:710`, `:780`, `:1052`, `:1402`, `:5696` — the sibling contract
    that judges the merge.
  - `crates/happenstance-sqlite/tests/shapes.rs` — the standing type-level guard;
    it must still pass unchanged in intent after `ReadCursor` gains merge state.
- **Renders surfaces**: **none.** `../_design.md` records N/A — no user-facing
  surface — for this entire project, approved 2026-08-12. This story adds **no**
  public item at all: the decomposition, the chunk width and the merge are
  `pub(crate)` or private, and that is itself an acceptance criterion (AC-007).
- **Conformance rule(s)**: no rule is added and no port changes. The rules that
  **observe** this story, by name and path, are
  `store_evaluates_a_query_at_the_guaranteed_minimum_item_count`
  (`crates/happenstance-testkit/src/suite.rs:3880`, registered at
  `crates/happenstance-testkit/src/registry.rs:182`),
  `query_items_are_or` (`:553`), `duplicate_items_do_not_duplicate_events` (`:636`),
  `query_item_order_does_not_change_the_result_set` (`:710`),
  `query_union_is_item_concatenation` (`:780`),
  `read_limit_applies_after_filtering` (`:1052`) and its backwards mirror,
  `limit_applies_across_items_not_per_item` (`:1402`), and
  `query_items_share_one_snapshot` (`:5696`). They run green only once
  `sqlite-fixture-and-whole-suite` mounts the fixture; until then this story's own
  target is what executes the path. The live wrong implementation is
  `ChunkedQueryStore` (`crates/happenstance-testkit/tests/mutation_coverage/mutants.rs:2589`),
  and no new registry row is owed.
- **Clause(s)**: discharges **VT-23** (`spec/SPECIFICATION.md:1535-1552`,
  `[PROVISIONAL]`) for this adapter and supplies adapter evidence for **ES-12**
  (`:2998-3037`, `[PROVISIONAL]`). Honours without amending the `[FROZEN]`
  **VT-31** (`:1860-1889`), **ES-14** (`:3111-3145`) and **ES-15** (`:3146-3175`),
  and consumes **ES-11**'s ceiling (`:2926-2940`). **No marker is edited and no
  `[FROZEN]` clause is touched**; the durability/marker verdicts belong to
  `reopen-negative-control-and-durability-verdicts` and the clause reconciliation to
  `spec-and-code-reconciliation`.
- **Advances DoD scenario**: initiative **DoD 3** — *"The durable store passes the
  suite for real"* (`.bklg/from-contract-to-published-library/initiative.md:366-368`).
  A suite that skipped or failed VT-23's rule would leave DoD 3 unmet; project
  **AC-008** is the row this story traces to.

## PR boundary

```
crates/happenstance-sqlite/src/**
crates/happenstance-sqlite/tests/wide_query.rs
.bklg/from-contract-to-published-library/sqlite-durable-store/wide-query-chunked-not-refused/**
```

**In this PR**

- The adapter-private decomposition of a `Query` into index arms, built only from
  `Query::items` / `QueryItem::types` / `QueryItem::tags`, with `Query::All` as one
  unconstrained arm and multi-tag arms planned most-selective-tag-first against
  `tag_cardinality`.
- The chunking of arms into `ceil(arms / N)` prepared statements at the width
  ADR-0022 records, with N reachable from this crate's own tests so the boundary can
  be crossed deliberately.
- The k-way merge inside `fetch_page`: ordered by position in the read's direction,
  de-duplicating on position, bounded by the *same* snapshot ceiling, `to`,
  `resume_from` and direction on every chunk statement, and yielding one `Page`
  whose length respects the page budget and what remains of `ReadOptions::limit`.
- `crates/happenstance-sqlite/tests/wide_query.rs`: the new target that drives all
  of the above against a real temporary file through the public `read`, including
  the VT-23 floor, a forced multi-chunk query, an overlap case, a limit case in both
  directions, and a snapshot case with a concurrent append between hops.
- The module documentation in `crates/happenstance-sqlite/src/event_store.rs`
  gaining a short, accurate paragraph on how a wide query is served — the crate's
  docs currently say nothing about it, and after this PR they would otherwise
  describe a read path that no longer exists.

**Explicitly not in this PR**

- The ceiling field, the first real page statement, and `ReadOptions`
  composition — `lazy-read-with-snapshot-ceiling` (the hard predecessor; if it has
  not landed, stop and say so rather than inventing a ceiling here).
- `append`, `head`, `contains_event_id` — `append-atomicity-and-store-limits`.
- `SqliteFixture`, `tests/conformance.rs`, any testkit macro invocation, any
  capability or limit declaration — `sqlite-fixture-and-whole-suite`.
- Any change to `happenstance-core`: no `index_arms()`, no `arm_count()`, no
  `MAX_QUERY_ITEMS`, no fourth `StoreLimit` variant.
- Any change to `happenstance-testkit`: no new rule, no new mutant row, no change to
  `registry.rs`.
- Deleting `#![allow(clippy::todo)]` or any `todo!()` outside the read path — DR-01
  binds the allow's deletion to the *last* `todo!()`
  (`instrument-markers-removed-and-gate-green`).
- `spec/SPECIFICATION.md` edits, marker moves, ADR authoring, and benchmark
  numbers — ADR-0022 and `benchmark-harness` own those.

**Merge DoD one-liner** — `cargo xtask affected --base main`
(`.redkiln/config.yaml:40`) green with `crates/happenstance-sqlite/tests/wide_query.rs`
executing a real multi-chunk read against a real file, the crossing of the chunk
boundary observed rather than assumed, `cargo xtask lints && cargo xtask spec-trace`
(`:48`) clean, and `git diff --stat crates/happenstance-core crates/happenstance-testkit`
empty.

## Behavior and interfaces

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| **A wide query is served, never refused** | No pushdown limit produces an error, an empty answer or a truncated answer. There is no query-width refusal path anywhere in the crate. | `spec/SPECIFICATION.md:1535-1552` (VT-23); `crates/happenstance-core/src/limits.rs:46-52` (*"a query-item refusal is not an append outcome"*); `RUNBOOK.md:4203-4206` |
| **Arms are derived from the public accessors only** | `Query::items()`, `QueryItem::types()`, `QueryItem::tags()`. `Query::All` is one unconstrained arm. Nothing is added to `happenstance-core`. | `crates/happenstance-core/src/query.rs:150-230`; `../_decomposition.md` architecture brief §5; `../_grounding.md:193-213` |
| **Multi-tag arms are probed most-selective-tag-first** | The cardinality comes from `tag_cardinality`, not from `ANALYZE`, which stores only an average. The measured swing is 650×. | `references/evaluation/ARCHITECTURAL-EVALUATION.md:827`; migration 1 via `schema-migration-and-identity` |
| **Arms are chunked at `ceil(arms / N)`** | N is ADR-0022's — the runbook's `400` is a proposal, and the real ceiling is SQLite's compiled-in compound-select and bound-parameter limits under `features = ["bundled"]`. N is not a private literal: this crate's tests can force `ceil(arms/N) > 1`. | `RUNBOOK.md:4204`; `Cargo.toml:47`; `../_decomposition.md` architecture brief §5 |
| **Each chunk statement streams in position order** | Bare top-level compound arms ordered by position, not a materialised `position IN (… UNION …)` set — which was measured 970× slower and *buffers the entire result before the first row*. | `references/evaluation/ARCHITECTURAL-EVALUATION.md:827-828` |
| **Every chunk carries the same bounds** | The snapshot ceiling, `ReadOptions::to`, `resume_from` and the direction are identical across all `ceil(arms/N)` statements of every hop. This is how ES-12 is discharged — by ES-11's ceiling, not by a second mechanism. | `spec/SPECIFICATION.md:2998-3037`, esp. `:3020-3025`; `crates/happenstance-testkit/src/suite.rs:5691-5715`; `.kb/decisions/0011-read-laziness-and-isolation.md` |
| **The merge de-duplicates on position** | Arms may overlap by construction — two items can match one event — and the merged output yields it exactly once. | `crates/happenstance-testkit/src/suite.rs:636`; `spec/SPECIFICATION.md:3146-3175` (ES-15, `[FROZEN]`) |
| **Item order never reaches the result** | The partition into chunks is an implementation detail; output order is position order in the read's direction. | `crates/happenstance-testkit/src/suite.rs:710`; `spec/SPECIFICATION.md:2732` (ES-8) |
| **Items are not collapsed or rewritten** | VT-31 licenses reorder/dedup *only* where the match set is preserved. Chunking is inside that licence; collapsing items with similar type lists is not. | `spec/SPECIFICATION.md:1860-1889` (`[FROZEN]`); `ItemDedupByTypeStore` in `crates/happenstance-testkit/tests/mutation_coverage/mutants.rs` |
| **The page budget and `limit` apply to the merged output** | `min(PAGE_SIZE, remaining)` is spent on merged rows, never per chunk. A chunk may over-fetch; the surplus is carried as owned rows or re-read, and either way `advance()` sees only the merged page. | `crates/happenstance-sqlite/src/event_store.rs:81, 332-368`; `spec/SPECIFICATION.md:3111-3145` (ES-14, `[FROZEN]`); `crates/happenstance-testkit/src/suite.rs:1402` |
| **`limit` is applied after filtering, in both directions** | `LIMIT` pushed into a descending scan takes the highest rows, so a per-chunk `LIMIT` passes forwards and fails backwards. Both directions are exercised. | `crates/happenstance-testkit/src/suite.rs:1052`; `read_backwards_limit_applies_after_filtering` via `crates/happenstance-testkit/src/registry.rs:130-131` |
| **`resume_from` stays inclusive and `advance()` is untouched** | The cursor's arithmetic is already correct for budget and direction; chunking adds a per-chunk resume position *inside* the hop and changes neither field's sense. | `crates/happenstance-sqlite/src/event_store.rs:309-330, 351-368`; `../_decomposition.md` architecture brief §4 |
| **`read` still executes nothing** | Arm planning may happen eagerly (it is pure and cheap) or lazily; statement preparation happens on the blocking thread inside `fetch_page`. A `read` that prepares a statement is a `read` that panics with no runtime in scope. | `crates/happenstance-sqlite/src/event_store.rs:11-32, 203-215`; ADR-0001, ADR-0008 via `CLAUDE.md` constraint 3 |
| **No `!Send` handle becomes a field** | `Statement`, `Rows` and `Transaction` live only inside the `spawn_blocking` closure. Merge state that crosses `poll_next` is owned data: per-chunk positions and materialised `SequencedEvent`s. | `crates/happenstance-sqlite/src/event_store.rs:248-271`; `crates/happenstance-sqlite/tests/shapes.rs` |
| **Both flavours still hold** | `SqliteEventStore` implements `SendEventStore`; the bare flavour is derived by `trait_variant` and neither is served differently by this story. Nothing here adds a bound. | `crates/happenstance-sqlite/src/event_store.rs:195-215`; `CLAUDE.md` constraints 1 and 4 |
| **Nothing becomes public** | The decomposition, the chunk width and the merge are crate-private. The only surface change is documentation. | `../_design.md` (N/A — no user-facing surface); this story's own diff |
| **The boundary is observed, not assumed** | The new target proves `ceil(arms/N) > 1` was actually taken — by a crate-visible counter, a test-visible width, or an equivalent in-crate observation. A merge that never ran is dead code behind a green suite. | `../_decomposition.md` testing brief §2, AC-008 row; `CLAUDE.md` — *the rule that matters* |
| **The falsifier already exists** | `ChunkedQueryStore` — chunk, then forget to union — is registered and passes nothing. No new mutant row is owed by this story. | `crates/happenstance-testkit/tests/mutation_coverage/mutants.rs:2580-2617`; `crates/happenstance-testkit/tests/mutation_coverage.rs:1981` |

## Data and migrations

**N/A — this story writes no schema and no data.** Migration 1 is landed whole by
`schema-migration-and-identity`, and this story is a **reader**: it plans arms
against the tables that story creates and adds no column, no index and no table of
its own.

Two consequences worth stating so they are not mistaken for gaps.

**If the merge wants an index that does not exist, that is a finding, not a
migration.** The two amendments migration 1 carries — `event_type` as a covering
column on `event_tag` with the key left `(tag, position)`, and `tag_cardinality` —
were chosen precisely so a typed, tagged arm is answerable from the index alone
(`RUNBOOK.md:4178-4187`; `references/evaluation/ARCHITECTURAL-EVALUATION.md:828`).
If chunked arms turn out to need something else, the correct move is to raise it
against `schema-migration-and-identity` and ADR-0022, not to add a `CREATE INDEX`
inside this PR's read path — a schema change smuggled into a reader is invisible to
the story that owns the schema and to the migration version it wrote.

**`tag_cardinality` is read here, and this story does not maintain it.** Keeping it
current is the write path's obligation (`append-atomicity-and-store-limits`). A
stale or empty cardinality table must degrade the *plan* — an arbitrary but valid
probe order — and never the *answer*; the merge's correctness may not depend on it.

## Acceptance criteria

Eight criteria, `AC-001` – `AC-008`. Each is written from the goal of a real
persona in `_discovery/distillation/personas-and-journeys.md` — **the application
author** (persona 1, `:42`), whose DCB decision model is what makes a query wide,
and **the adapter author** (persona 2, `:114`), whose journey *"Learn when you are
finished"* is the one this project exists to close. Every criterion crosses the
whole stack: a real temporary SQLite file, a real `rusqlite::Connection`, through
the public `SendEventStore::read`. Nothing here is satisfied by a unit test of a
private function.

`crates/happenstance-sqlite/tests/wide_query.rs` is a **new target this story
creates**; the test names below are the ones it is expected to carry, and an
implementer who renames one must rename it in `_ledger.md` too.

| id | criterion | verification |
| --- | --- | --- |
| **AC-001** | **GIVEN** an application author whose DCB decision model has grown to the 128 query items every store must evaluate (`crates/happenstance-core/src/limits.rs:32`) and beyond, **WHEN** they call `read` on `SqliteEventStore` with that `Query`, **THEN** they receive every matching event — not a `SqliteEventStoreError`, not an empty stream, and not a silently truncated prefix — and there is no query-width refusal path anywhere in the crate to reach: no `MAX_QUERY_ITEMS`, no fourth `StoreLimit` variant, no early `return Err` keyed on item or arm count. | `crates/happenstance-sqlite/tests/wide_query.rs::a_query_at_the_guaranteed_minimum_item_count_is_served` — builds `MIN_SUPPORTED_QUERY_ITEMS` items (imported, never the literal `128`) with the single matching item **last**, and asserts the matched event comes back; plus `::a_query_far_above_the_minimum_is_served`. Negative guard: `rg -n "MAX_QUERY_ITEMS\|QueryItems\|too many (query )?items" crates/happenstance-sqlite/src` returns nothing. Sibling proof once `sqlite-fixture-and-whole-suite` mounts the fixture: `store_evaluates_a_query_at_the_guaranteed_minimum_item_count` (`crates/happenstance-testkit/src/suite.rs:3880`). |
| **AC-002** | **GIVEN** an adapter author who needs to know the merge *ran* rather than that the suite was green, **WHEN** they run this crate's tests, **THEN** at least one test issues a `Query` wide enough that `ceil(arms / N) > 1` and **observes in-crate that the boundary was crossed** — the chunk width is reachable from the test rather than an unreadable private literal, and the test fails if the read were served by a single statement. A merge that never executes is dead code behind a green suite, which is the failure mode this initiative exists to retire. | `crates/happenstance-sqlite/tests/wide_query.rs::a_wide_query_actually_crosses_the_chunk_boundary` — asserts the observed chunk count (`> 1`) via whatever crate-visible observation the implementer chooses (a `pub(crate)` planner function the integration target reaches through a `#[doc(hidden)]`/`cfg(test)`-free seam, a counter, or a `#[cfg(feature = "…")]`-free `pub(crate) const` plus an arm-count assertion), and would fail at `== 1`. Run by `cargo test -p happenstance-sqlite --test wide_query` inside `cargo xtask affected --base main` (`.redkiln/config.yaml:40`). |
| **AC-003** | **GIVEN** an application author whose wide query has two items that both match the same event, and whose items were written in whatever order the domain suggested, **WHEN** they read, **THEN** they get the **union** of every item's matches — never only the first chunk's — with each event yielded **exactly once**, in ascending position order regardless of how items were partitioned into chunks or ordered in the `Query`, and with no two items collapsed because their type lists looked alike (VT-31 licenses reorder/dedup only where the match set is preserved). | `crates/happenstance-sqlite/tests/wide_query.rs::overlapping_arms_yield_each_event_once`, `::item_order_does_not_change_the_result_set` (same `Query` items shuffled, identical output vector), `::the_union_is_every_chunk_not_the_first`, `::items_with_similar_types_but_different_tags_are_not_collapsed`. Sibling proof: `query_items_are_or` (`suite.rs:553`), `duplicate_items_do_not_duplicate_events` (`:636`), `query_item_order_does_not_change_the_result_set` (`:710`), `query_union_is_item_concatenation` (`:780`). Live falsifier already registered: `ChunkedQueryStore` (`crates/happenstance-testkit/tests/mutation_coverage/mutants.rs:2589`). |
| **AC-004** | **GIVEN** an application author who asked for `ReadOptions::limit = n` over a wide query, in either direction, **WHEN** they read, **THEN** they receive exactly `min(n, matches)` events counted **after** filtering and **across** the merged output — never `n` per chunk and never `n` per item — and the backwards read returns the *highest* `n` matching positions rather than whatever a per-chunk `LIMIT` happened to keep. ES-14 is `[FROZEN]` and names the per-statement-`LIMIT` adapter as its rejected implementation (`spec/SPECIFICATION.md:3111-3145`). | `crates/happenstance-sqlite/tests/wide_query.rs::limit_applies_across_chunks_not_per_chunk`, `::limit_applies_after_filtering_forwards`, `::limit_applies_after_filtering_backwards`. Sibling proof: `limit_applies_across_items_not_per_item` (`suite.rs:1402`), `read_limit_applies_after_filtering` (`:1052`) and its backwards mirror (`crates/happenstance-testkit/src/registry.rs:130-131`). |
| **AC-005** | **GIVEN** an application author replaying a wide query while another writer is appending, **WHEN** the read spans several chunk statements and several `spawn_blocking` hops, **THEN** every one of those statements is bounded by the **same** snapshot ceiling, `ReadOptions::to`, direction and `resume_from` — so an event appended between chunk 1 and chunk k, or between hop 1 and hop 2, is above the ceiling and absent from the whole read. ES-12 is discharged by ES-11's ceiling and by no second mechanism (`spec/SPECIFICATION.md:3020-3025`). | `crates/happenstance-sqlite/tests/wide_query.rs::a_concurrent_append_between_chunks_is_not_seen` (wide query, real append committed on a second connection mid-read, asserted absent) and `::a_concurrent_append_between_pages_is_not_seen`. Sibling proof: `query_items_share_one_snapshot` (`suite.rs:5696`, registered `registry.rs:214`). |
| **AC-006** | **GIVEN** an application author paging a wide replay whose page boundary lands in the middle of a chunk's rows, **WHEN** the stream fetches the next page, **THEN** no row is repeated and none is dropped: `resume_from` keeps its **inclusive** sense, per-chunk resume state stays *inside* one hop, and `advance()` folds only the **merged** page into `resume_from` and `remaining`. The historical `resume_after` bug — an inclusive seed advanced by an exclusive step, re-reading one row per page boundary — must not return in per-chunk form (`crates/happenstance-sqlite/src/event_store.rs:313-330`). | `crates/happenstance-sqlite/tests/wide_query.rs::paging_a_wide_query_repeats_no_row_and_drops_none` — more than `PAGE_SIZE` (512) matching events across a multi-chunk query, asserting the collected positions equal the positions the appends actually assigned, with no duplicates and no gaps of our own making. Plus `::paging_backwards_repeats_no_row_and_drops_none`. Guard: `git diff crates/happenstance-sqlite/src/event_store.rs` shows `advance()` unchanged. |
| **AC-007** | **GIVEN** an adapter author or an evaluator reading `happenstance-core`'s public surface after this PR, **WHEN** they diff it, **THEN** it is byte-identical: no `index_arms()`, no `arm_count()`, no `IndexArm`, no `MAX_QUERY_ITEMS`, no fourth `StoreLimit` variant — and `happenstance-testkit` gains no rule, no registry entry and no mutant row either. Everything this story builds (arm planning, the chunk width, the merge) is `pub(crate)` or private inside `happenstance-sqlite`; the crate's only surface change is documentation. The re-open trigger is recorded, not forgotten: two unlike storage shapes needing the same decomposition is what earns a public `index_arms()`, in its own ADR. | `git diff --stat crates/happenstance-core crates/happenstance-testkit` empty on this PR's branch; `cargo public-api`-free check by review of the diff, plus `cargo doc -p happenstance-sqlite --no-deps` showing no new public item. Enforced continuously by `crates/happenstance-sqlite/tests/wide_query.rs` compiling as an **integration** target — it can only reach public API, so anything it needs that is not public would fail to compile, and the boundary observation of AC-002 must therefore be designed, not smuggled. |
| **AC-008** | **GIVEN** a constrained caller — a bare OS thread with no tokio runtime in scope, and a `tokio::spawn`ed task that holds the stream across an await — **WHEN** they call `read` with a wide query and then poll it, **THEN** `read` still executes no SQL and prepares no statement (it cannot: `spawn_blocking` panics with no runtime, which is why the absent-runtime case is `SqliteEventStoreError::NoRuntime` and not a panic), the returned `SqliteReadStream` is still `Send + Unpin`, no `rusqlite::Statement`, `Rows` or `Transaction` has become a field, and the first page arrives without the whole result being buffered — the materialised `position IN (… UNION …)` plan is rejected because it buffers before the first row, not merely because it measured 970× slower. | `crates/happenstance-sqlite/tests/shapes.rs` still green unchanged in intent (`cargo test -p happenstance-sqlite --test shapes`); `crates/happenstance-sqlite/tests/wide_query.rs::read_of_a_wide_query_executes_nothing_off_runtime` (construct the stream on a non-runtime thread, assert no panic and no I/O) and `::a_wide_read_streams_from_a_tokio_spawn` (stream held across an await inside a real `tokio::spawn`). Static tier: `cargo xtask lints` and `cargo clippy -- -D warnings` inside `cargo xtask affected --base main`. |

## Interaction quality

RFC §6.7/D6. Both families are declared, and one of them is declared **N/A on the
authority of a signed-off artifact** rather than skipped.

### Composition family — N/A, and here is the artifact that says so

[`../_design.md`](../_design.md) records **N/A, no user-facing surface** for this
entire project, approved 2026-08-12. This story renders nothing: no screen, no CLI
output, no log line, no new public item (AC-007). There is therefore no
composition, transience policy, density budget, hierarchy or anti-pattern list to
bind to, and inventing one would contradict a signed-off decision.

What the N/A does **not** license is a documentation regression. The one surface
this story touches is the module documentation of
`crates/happenstance-sqlite/src/event_store.rs`, which today describes a read path
that this PR changes and says nothing about how a wide query is served. That
obligation is carried as **NF-004**, not as a composition AC, because
`standards/rust/70-rustdoc-obligations.md` — not `_design.md` — is the artifact
that governs it.

### State family — carried by AC rows, not by prose here

Every applicable state invariant is an `AC-###` row in the table above, because
`redkiln verify` extracts ACs from that table and a bullet here would be gated by
nothing. The library analogues, and which row carries each:

| State invariant (library analogue) | Carried by | How it is verified |
| --- | --- | --- |
| **In-place, not a context jump** — a wide query is served by the same `read` call the narrow one uses; the caller changes nothing, learns nothing about chunking, and is never asked to re-issue | **AC-001**, **AC-007** | `wide_query.rs::a_query_at_the_guaranteed_minimum_item_count_is_served`; empty `git diff --stat` against `happenstance-core` |
| **Non-occlusion** — widening a query never hides a matching event behind the chunk partition; the answer is the union, not the visible chunk | **AC-003** | `::the_union_is_every_chunk_not_the_first`, `::overlapping_arms_yield_each_event_once` |
| **Preserved position** (the paging analogue of preserved scroll/selection) — a page boundary that coincides with a chunk boundary repeats no row and drops none; `resume_from` keeps one sense | **AC-006** | `::paging_a_wide_query_repeats_no_row_and_drops_none` and its backwards twin |
| **Stable ordering under re-issue** — the same `Query` with items shuffled yields the identical sequence; chunk membership never leaks into output order | **AC-003** | `::item_order_does_not_change_the_result_set` |
| **Reversibility** — every guarantee above holds in the backwards direction, where the naive `LIMIT` pushdown silently returns the wrong rows | **AC-004**, **AC-006** | `::limit_applies_after_filtering_backwards`, `::paging_backwards_repeats_no_row_and_drops_none` |
| **A consistent view under concurrent change** — the reader's world does not shift under it mid-read, however many statements the read costs | **AC-005** | `::a_concurrent_append_between_chunks_is_not_seen` |
| **Reachability** (the keyboard-reachability analogue: reachable through the public contract alone) — everything asserted is asserted through `SendEventStore::read` from an integration target, never through a private hook a `mod` test could reach | **AC-002**, **AC-007**, **AC-008** | `wide_query.rs` is an integration target by construction; `shapes.rs` green |
| **No silent failure mode** — a store that answers *wrongly* rather than erroring is the worst outcome here, and a live wrong implementation exists to prove the rules can catch it | **AC-003** | `ChunkedQueryStore` (`mutation_coverage/mutants.rs:2589`), already registered |

## Error conditions

| id | condition | required behaviour |
| --- | --- | --- |
| **EC-001** | A chunk statement fails mid-merge — `SQLITE_BUSY`, I/O, a malformed statement — after earlier chunks have already produced rows for this page. | The hop returns `Err(SqliteEventStoreError::Sqlite(_))`, the stream yields that error **once** and transitions to `ReadState::Done` (`crates/happenstance-sqlite/src/event_store.rs:289-291`). A partially merged page MUST NOT be yielded as a successful page: a short page is indistinguishable from a correct one to the caller, and that is a wrong answer wearing a green stream. |
| **EC-002** | The connection mutex is poisoned by a panicking thread while chunked statements are in flight. | `SqliteEventStoreError::ConnectionPoisoned`, exactly as today (`:75`). Chunking adds no second lock and no lock ordering: the k statements of one hop are prepared and stepped under the **one** existing guard, which is also what makes the shared snapshot of AC-005 cheap. |
| **EC-003** | A stored row inside one chunk fails to decode — an event type or tag that no longer validates, or a position SQLite accepted that `SequencePosition` cannot represent. | `StoredEventType` / `StoredTag` / `InvalidPosition` (`:79-92`) propagate and fail the whole read. The row MUST NOT be skipped so the merge can continue: dropping it converts a loud decode failure into a silently incomplete answer, which AC-001's "not a silently truncated prefix" forbids. |
| **EC-004** | An individual chunk is still too wide for SQLite — compound-select terms or bound parameters over the compiled-in limit — at whatever `N` ADR-0022 records. | This is a defect in `N`, **not** a caller error, and it MUST NOT be surfaced as a "query too wide" error: VT-23 (`spec/SPECIFICATION.md:1535-1552`) forbids the refusal, and `crates/happenstance-core/src/limits.rs:46-52` says there is no variant for it. `N` is chosen strictly below the ceiling under `rusqlite`'s `features = ["bundled"]` defaults, and AC-001's above-the-minimum test is the standing guard. If the ceiling cannot be established, that is an ADR-0022 finding to raise, not a number to guess. |
| **EC-005** | `tag_cardinality` is empty, stale, or absent for a tag an arm is being planned against. | The **plan** degrades to an arbitrary but valid probe order; the **answer** does not change and no error is produced. The merge's correctness may not depend on cardinality data this story does not maintain (`append-atomicity-and-store-limits` owns keeping it current). |
| **EC-006** | The stream is polled outside a tokio runtime, or the blocking task panics or is cancelled mid-hop. | `NoRuntime` / `Worker(JoinError)` (`:77-88`), unchanged by this story. In particular `read` itself still returns without touching the runtime (AC-008) — a wide query must not become the reason `read` starts doing work eagerly. |
| **EC-007** | `Query::items()` returns `None` (`Query::All`) or an empty item slice. | `Query::All` plans as **one** unconstrained arm over `event` — a single chunk, the ordinary path, no merge overhead. An empty item slice matches nothing and yields an empty stream, not an error. Neither may take a chunking branch that only exists for wide queries. |

## Non-functional

| id | requirement | why, and how it is checked |
| --- | --- | --- |
| **NF-001** | **First row before full materialisation.** A wide read must produce its first page without buffering the whole result set. The rejected shape is explicit: a materialised `position IN (… UNION …)` set, measured 970× slower and, decisively for this port, *"buffers the entire result before the first row"* (`references/evaluation/ARCHITECTURAL-EVALUATION.md:827`). | It is a breach of the promise `EventStore::read` makes (`crates/happenstance-sqlite/src/event_store.rs:76-81`) and of ADR-0011, not a performance preference. Checked by AC-008's streaming test and by review of the emitted SQL shape. |
| **NF-002** | **Index-order preserving arms.** Chunk statements are bare top-level compound arms ordered by position, keeping `event_tag`'s `(tag, position)` key order alive (`references/evaluation/ARCHITECTURAL-EVALUATION.md:828`) so a typed, tagged arm is answerable from the index alone. | The covering column and `tag_cardinality` in migration 1 were chosen for exactly this. If a plan needs an index that does not exist, it is a finding against `schema-migration-and-identity` and ADR-0022 — never a `CREATE INDEX` in a reader (see **Data and migrations** above). |
| **NF-003** | **No numbers are invented in silence.** `N` is whatever ADR-0022 records, with the runbook's `400` treated as a proposal (`RUNBOOK.md:4204`); `PAGE_SIZE = 512` stays the placeholder it declares itself to be (`event_store.rs:77-81`); this PR quotes no benchmark figure. | ADR-0022 owns the measurement (`project.md` AC-013), and `benchmark-harness` owns the harness. A number that appears here without a record is exactly the drift the ADR queue exists to prevent. |
| **NF-004** | **The documentation still describes the code.** `crates/happenstance-sqlite/src/event_store.rs`'s module docs gain a short, accurate paragraph on how a wide query is served — arms, chunking, one shared ceiling — and every `# Errors` / laziness statement already there stays true after the change. | `standards/rust/70-rustdoc-obligations.md`; `cargo doc` runs inside `cargo xtask ci`. Docs that survive a change without being read are the ones that go stale, and this PR changes the paragraph they describe. |
| **NF-005** | **The `Send`/`!Send` two-flavour design costs nothing here.** No `+ Send` bound is added anywhere; `SqliteEventStore` implements `SendEventStore` and the bare flavour is derived by `trait_variant`, unchanged. No `#[async_trait]`, ever. | `CLAUDE.md` constraints 1 and 4; ADR-0001, ADR-0008. Checked by `shapes.rs` and by the wasm32 steps of `cargo xtask ci` (which this crate does not target, but the contract crate does). |
| **NF-006** | **Memory is bounded by the page, not by the query's width.** Merge state that crosses `poll_next` is `k` positions plus at most one page of materialised rows; a wide query must not make peak memory a function of arm count times result size. | `standards/rust/23-streams-and-state-machines.md`, `24-the-blocking-bridge.md`; the same reasoning that keeps `!Send` handles out of fields (AC-008). |

## Implementation notes (non-prescriptive)

The implementer owns every detail below; these are the traps, not the design.

- **Plan, then chunk, then merge — three separable pieces.** Arm planning from
  `Query::items()` / `QueryItem::types()` / `QueryItem::tags()` is pure and cheap
  and may run eagerly in `read`; *statement preparation* may not
  (`crates/happenstance-sqlite/src/event_store.rs:203-215`). Keeping the planner
  a pure function of `&Query` is also what makes AC-002's boundary observation
  cheap to express.
- **The merge is "k positions and a buffer", not "k open cursors".**
  `rusqlite::Statement`, `Rows` and `Transaction` are `!Send`; they live and die
  inside the `spawn_blocking` closure. What survives a hop is owned:
  per-chunk resume positions and materialised `SequencedEvent`s. This is stated
  in the type's own docs at `:248-271` and is the reason `shapes.rs` exists.
- **Two viable shapes for the k-way merge, and both are acceptable.** Either
  (a) step all k statements together under the one connection guard, taking the
  minimum position each round and skipping equal positions — a true k-way merge
  with a binary heap or a linear scan over small k; or (b) fetch a bounded slab
  per chunk, merge in memory, and carry the surplus as owned rows into the next
  hop. (a) keeps memory tightest; (b) is simpler to get right. What is **not**
  acceptable is fetching all chunks to exhaustion first (NF-001).
- **Over-fetch is fine; over-limiting is not.** A chunk statement may be asked
  for more rows than the page budget — the budget applies to merged output. Any
  `LIMIT` pushed into a chunk statement must be a *safe over-approximation* of
  the merged budget, never the budget itself, and must be re-checked in both
  directions before it is trusted (AC-004).
- **De-duplicate on position, not on identity.** Two arms matching the same row
  produce the same `position`; that is the key. `EventId` equality is a different
  question (`contains_event_id`, owned by `append-atomicity-and-store-limits`).
- **Do not touch `advance()`.** It is already correct for budget and direction,
  including the `saturating_sub` that makes "no more positions this way" the same
  fact as "spent" (`:351-368`). Per-chunk arithmetic stays inside `fetch_page`.
- **Anchor the wide test on the constant, not the literal.** Import
  `MIN_SUPPORTED_QUERY_ITEMS` (`crates/happenstance-core/src/limits.rs:32`) so the
  test tracks the floor if it ever moves, and never assert on literal position
  values — the specification permits gaps and `CLAUDE.md` says so in terms.
  Compare against the positions the store actually assigned.
- **Make the boundary observable without making it public.** AC-002 and AC-007
  pull in opposite directions on purpose. Options worth weighing: a `pub(crate)`
  planner plus a `#[cfg(test)]` unit module *in addition to* the integration
  target (the integration target still owns the behavioural assertions); a
  `#[doc(hidden)] pub` seam (rejected here — it is public surface wearing a hat);
  or making the observable a property of the *result* rather than the code, e.g.
  a query whose arm count is arithmetically known from `N` and whose answer can
  only be right if every chunk ran. Prefer the last where it is expressible.
- **`Query::All` is the one-arm case and should stay on the ordinary path.** A
  chunking branch that triggers for `All` would put the common read behind the
  rare one's machinery.

## Tests and CI (merge gate)

Grounded in the testing brief's tier table (`../_decomposition.md` §1, lines
598-605) and its AC-008 row (`:634`). No tier here is a description of intent with
no command behind it.

| tier | command / path | proves |
| --- | --- | --- |
| **Static** | `cargo xtask lints && cargo xtask spec-trace` (`reachability_static`, `.redkiln/config.yaml:48`) | `fmt`, `clippy -D warnings` (so `missing_docs` catches NF-004's gap), the file-reading lints, and that no `spec/SPECIFICATION.md` citation was broken — this story cites VT-23, VT-31, ES-11, ES-12, ES-14, ES-15 and edits none of them. |
| **Static (scoped guard)** | `git diff --stat crates/happenstance-core crates/happenstance-testkit` empty; `rg -n "MAX_QUERY_ITEMS\|index_arms\|arm_count\|IndexArm" crates/` finds nothing new | **AC-007**, **AC-001**'s "no refusal path to reach". A reviewer runs both; they are cheap and they are the only mechanism that catches a convenience API added under deadline. |
| **Unit / type-level** | `cargo test -p happenstance-sqlite --test shapes` (`crates/happenstance-sqlite/tests/shapes.rs`, exists) | **AC-008**, **NF-005**: `SqliteReadStream: Send + Unpin` and `SqliteEventStore: Send + Sync` still hold after `ReadCursor` gains merge state. This is the guard that fires before any SQL runs if a `!Send` handle became a field. |
| **Integration (this story's mount)** | `cargo test -p happenstance-sqlite --test wide_query` (`crates/happenstance-sqlite/tests/wide_query.rs`, **new**) | **AC-001 – AC-006**, **AC-008**: every criterion above, against a real temporary file through a real `rusqlite::Connection` and the public `read`. No `rusqlite`, filesystem or runtime double is permitted here (testing brief AC-T03, `:574-578`). |
| **Story gate (the command that runs the above)** | `cargo xtask affected --base main` (`affected_gate`, `.redkiln/config.yaml:40`) | That the new target is *actually executed* by the merge gate rather than merely existing — the gate maps the diff to `happenstance-sqlite` and runs its whole test set. This is the story-grain bar. |
| **Conformance (deferred to the slice-mate, named here so it is not forgotten)** | `cargo test -p happenstance-sqlite` inside `cargo xtask ci --fast` (`integration_scoped`, `:55`), once `sqlite-fixture-and-whole-suite` mounts `crates/happenstance-sqlite/tests/conformance.rs` | The eight sibling rules that judge this merge — `suite.rs:553`, `:636`, `:710`, `:780`, `:1052`, `:1402`, `:3880`, `:5696` — report `Ran` rather than `Skipped`. **This story does not create that target**; it must leave it green when it arrives, and the slice is implemented in one context so the feedback is not deferred to a later PR. |
| **Mutation coverage (the check on the checker)** | `cargo xtask ci --fast` runs `mutation_coverage::every_rule_has_a_mutant` and `::conformant_variants_pass_everything` (`xtask/src/proof.rs:86-90`) | That the rules judging this merge **can fail**: `ChunkedQueryStore` (`mutation_coverage/mutants.rs:2580-2617`, registered `mutation_coverage.rs:1981`) answers wrong rather than erroring, and `ItemDedupByTypeStore` (`:691`) catches the VT-31 violation. **No new mutant row is owed by this story** — the falsifier already exists, which is the `CLAUDE.md` bar met rather than argued. |
| **Not this story's gate** | `cargo xtask ci` (`e2e`, `:60`), MSRV, feature powersets, `cargo deny`, docsrs | Owned by `closeout-and-durable-audience`; `project.md` DoD names `cargo xtask ci --fast` as this project's terminal command. Benchmarks are never a gate (CF-34). |

**Merge is blocked unless**: `cargo xtask affected --base main` is green with
`wide_query.rs` executing a real multi-chunk read, `cargo xtask lints && cargo
xtask spec-trace` is clean, the two scoped diff guards are empty, and every row of
`_ledger.md` is `satisfied: true` with cited evidence (`require_ledger: true`,
`.redkiln/config.yaml:67`).

## Risks and coupling (PR-scoped)

| risk | why it bites here | containment |
| --- | --- | --- |
| **The predecessor has not landed.** This story consumes the snapshot ceiling `lazy-read-with-snapshot-ceiling` adds to `ReadCursor` and the first real page statement it writes. | Without them there is nothing to chunk, and AC-005 has no ceiling to replicate across statements. Inventing a second ceiling here would give the crate two isolation mechanisms and satisfy ES-12 by accident. | Hard blocker: if the ceiling field is absent, **stop and say so** rather than adding one. Both stories are in the same slice and the same context, so the correct order is available, not a scheduling hope. |
| **Chunking is literally ES-12's rejected shape.** *"A one-round-trip adapter that emits one SQL statement per `QueryItem` and unions the results client-side"* (`spec/SPECIFICATION.md:2998-3037`). | This PR could break the snapshot guarantee the previous PR just established, and the sequential rules would stay green while it did. | AC-005 is the criterion, `query_items_share_one_snapshot` is the sibling rule, and the discharge is stated once: **the same ceiling on every statement**, not a second mechanism (`:3020-3025`). |
| **The suite cannot reach the boundary.** VT-23's floor is 128 items at one tag per item — orders of magnitude below any plausible `N`. | Ship at `N = 400` with no extra target and the merge is dead code behind a green suite: the adapter would acquire conformance over a path that never ran, which is precisely the failure this initiative exists to retire. | AC-002 and the `wide_query.rs` target exist for this and nothing else. The testing brief asked for it in terms (`:634`). |
| **The convenience API creeps back in.** Both `RUNBOOK.md:4203` and the evaluation (`ARCHITECTURAL-EVALUATION.md:827`) write the work item as `Query::index_arms()`, so a reader assumes it is owed. | Adding it would put public surface in a contract crate for one implementor's benefit — one implementor is not a spread (`CLAUDE.md`). | AC-007, the empty-diff guard, and the named re-open trigger (two unlike storage shapes; its own ADR). Do not re-litigate in this PR. |
| **The width `N` is a guess wearing a decimal point.** `400` is a proposal; the real ceiling is SQLite's compiled-in limits, and `rusqlite` is `default-features = false, features = ["bundled"]`. | A silently wrong `N` fails only at scale, i.e. in a user's replay and not in CI. | NF-003: `N` is ADR-0022's, and EC-004 forbids converting a too-wide chunk into a caller-facing error. Probing the limits may need a `rusqlite` feature — that trade is ADR-0022's to record, not this PR's to make. |
| **Coupling to `schema-migration-and-identity`.** Arm planning reads `tag_cardinality` and depends on `event_tag`'s covering column. | A merge that wants a different index would tempt a `CREATE INDEX` inside a reader — invisible to the story that owns the schema and to the migration version it wrote. | EC-005 (degrade the plan, never the answer) plus the explicit rule in **Data and migrations**: raise it against the schema story and ADR-0022. |
| **Coupling to `sqlite-fixture-and-whole-suite`.** The eight sibling rules only run once that story mounts the fixture. | A green `wide_query.rs` with a red conformance run is not partial credit. | Same slice, one context, sequential implementation; this story's target is what executes the path in the interim, and the conformance run is what confirms it. |
| **`clippy::todo` stays.** `append`, `head` and `contains_event_id` are still `todo!()` after this PR. | An implementer may be tempted to delete `#![allow(clippy::todo)]` once the read path is complete. | DR-01 binds the allow's removal to the **last** `todo!()`; that is `instrument-markers-removed-and-gate-green`'s, and removing it early turns the gate red for everyone in the slice. |

## Dependencies

**Blocks on**

- `lazy-read-with-snapshot-ceiling` — **hard**. Supplies the snapshot ceiling on
  `ReadCursor` (architecture brief AC-A03, ADR-0011) and the first real page
  statement that this story generalises from one to `ceil(arms/N)`. Every criterion
  from AC-004 through AC-006 is expressed in terms of state that story creates.
- `schema-migration-and-identity` — **soft, same slice, earlier in it**. Migration 1
  creates `event`, `event_tag` with its covering column, and `tag_cardinality`; arms
  are planned against them. This story reads that schema and writes none of it. Not
  listed in `depends_on` because the story map routes it through
  `lazy-read-with-snapshot-ceiling`, which already requires it
  (`../_storymap.md:54`).

**Unlocks**

- `sqlite-fixture-and-whole-suite` — declares this story a direct dependency
  (`../_storymap.md:55`). The eight query/limit/snapshot rules cannot go green
  against `SqliteFixture` until the merge exists, and that story is the slice's
  ultimate root.
- `benchmark-harness` — the merged read path is what `event_store_benchmarks!`
  measures for ADR-0022's `N` and busy-timeout paragraphs.
- `instrument-markers-removed-and-gate-green` — one of the four `todo!()`s on the
  SQLite path is retired here (the read path's), narrowing what DR-01 waits on.

**Explicitly not a dependency**

- `adr-0022-append-condition-strategy` sequences *before* the implementation for the
  numbers it records (`N`, the busy timeout, the append-condition SQL), but this
  story writes no ADR text and authors no decision atom — `project.md` AC-013 and
  the runbook's ADR queue own that.

## Anchors (progressive disclosure)

Everything above is enough to start. Open these when the row's moment arrives —
each is load-bearing for one criterion, and none needs to be read in advance.

| anchor | why it is load-bearing | when to open | serves |
| --- | --- | --- | --- |
| `.bklg/from-contract-to-published-library/sqlite-durable-store/lazy-read-with-snapshot-ceiling/spec.md` | The predecessor's own spec: where the ceiling field lives, when it is captured, and what the first page statement looks like. This story generalises exactly that statement from one to k. | **Before writing a single line** — if this story is implemented first, stop. | AC-005, AC-006 |
| `.bklg/from-contract-to-published-library/sqlite-durable-store/_decomposition.md` (architecture §5, `:239-288`; testing §2, `:634`) | The decision this story executes — decompose privately, option (b), with the named re-open trigger — plus the non-prescriptive shape of arms and the merge, and the tier/proof row for AC-008. | Before designing the planner, and again when writing the test list. | AC-002, AC-003, AC-007 |
| `spec/SPECIFICATION.md` (VT-23 `:1535-1552`; ES-12 `:2998-3037`; ES-14 `:3111-3145`; VT-31 `:1860-1889`; ES-15 `:3146-3175`; ES-11 `:2926-2940`; ES-8 `:2732`) | The normative clauses, each with its `Rejects:` line naming the wrong adapter. ES-12's rejected shape *is* chunking; ES-14's is per-statement `LIMIT`. Read the `Rejects:` lines, not just the requirement. | When designing the merge, and before deciding where `LIMIT` goes. **No marker is edited.** | AC-001, AC-003, AC-004, AC-005 |
| `crates/happenstance-sqlite/src/event_store.rs` (`:76-81`, `:198-215`, `:248-271`, `:300-368`) | The seam itself: the laziness promise, `read`'s "nothing is executed here on purpose", the `Send` rationale that forbids `!Send` fields, and `Page`/`ReadCursor`/`fetch_page`/`advance` with `resume_from`'s inclusive sense documented at `:313-330`. | Immediately before writing `fetch_page`. | AC-006, AC-008 |
| `crates/happenstance-core/src/query.rs` (`:102`, `:107`, `:204`) | `QueryItem::types`, `QueryItem::tags`, `Query::items` — the **only** accessors arm planning may use, and the evidence that `index_arms()` does not exist. | When writing the planner, and again if tempted to add an accessor. | AC-007 |
| `crates/happenstance-core/src/limits.rs` (`:32`, `:46-52`) | `MIN_SUPPORTED_QUERY_ITEMS` — the constant the wide test anchors on — and the paragraph explaining why `StoreLimit` has three variants and not four. | When writing the AC-001 test, and if any refusal path is ever contemplated. | AC-001 |
| `crates/happenstance-testkit/src/suite.rs` (`:553`, `:636`, `:710`, `:780`, `:1052`, `:1402`, `:3880`, `:5691-5715`) | The eight sibling rules that will judge this merge, each with its own docs saying what it rejects — including `query_items_share_one_snapshot`'s statement that ES-12 is discharged by ES-11's ceiling. | While writing `wide_query.rs`, so the local tests and the suite agree about what is being asserted. | AC-001, AC-003, AC-004, AC-005 |
| `crates/happenstance-testkit/tests/mutation_coverage/mutants.rs` (`:691`, `:2580-2617`) | `ChunkedQueryStore` — "chunk, then forget to union the chunks", with the parameter arithmetic in its own comment — and `ItemDedupByTypeStore`. The already-registered falsifiers, which is why this story owes no new mutant row. | Before concluding a new mutant is needed. | AC-002, AC-003 |
| `references/evaluation/ARCHITECTURAL-EVALUATION.md` (`:827-828`) | The measurements: the 970× penalty and the *buffers-before-first-row* finding that rules out the materialised `IN` set; the 650× probe-order swing that justifies `tag_cardinality`; the `(tag, position)` key order to preserve. | When choosing the emitted SQL shape. | AC-008, NF-001, NF-002 |
| `.kb/decisions/0011-read-laziness-and-isolation.md` | The Accepted decision behind the ceiling and the laziness this story must not break. An accepted atom is immutable — read it, do not amend it. | Before AC-005's concurrent-append test. | AC-005, AC-008 |
| `crates/happenstance-sqlite/tests/shapes.rs` | The standing type-level guard. It must still pass unchanged in intent after `ReadCursor` gains merge state — it is what fires if a `Statement` or `Rows` becomes a field. | Immediately after adding any field to `ReadCursor` or `SqliteReadStream`. | AC-008 |
| `standards/rust/23-streams-and-state-machines.md`, `standards/rust/24-the-blocking-bridge.md` | House rules for the exact two constructs this story touches: a hand-written `Stream` state machine and the `spawn_blocking` bridge. Pull these two atoms only — not the corpus. | Before restructuring `ReadState` or the hop boundary. | AC-006, AC-008, NF-006 |
| `standards/rust/70-rustdoc-obligations.md` | What the module documentation owes once the read path it describes has changed. | When writing NF-004's paragraph. | NF-004 |
| `crates/happenstance-testkit/src/registry.rs` (`:130-131`, `:182`, `:214`) | Where the judging rules are registered, including the backwards `limit` mirror that is generated rather than hand-written — the reason AC-004 tests both directions. | When checking that a rule runs rather than skips. | AC-004 |
| `.kb/open-questions/cf-40-fixture-limits-ownership.md` | The open question about where a fixture's limit declarations belong. Named so it is **not** settled in passing by anyone tempted to declare a query-width ceiling. | Only if the urge to declare a width limit arises. | AC-001, AC-007 |
| `RUNBOOK.md` (`:4178-4187`, `:4203-4206`) | The work item verbatim — migration 1's two amendments, the `ceil(arms/400)` proposal, and *"`Query` bounds nothing by design; the limit is adapter-specific and a hard cap in the contract would be wrong"*. | When `N` is being chosen and when the schema is being read. | AC-001, AC-002 |
| `.bklg/from-contract-to-published-library/_discovery/distillation/personas-and-journeys.md` (`:42`, `:114-181`) | The two personas the criteria are written from, including *"Learn when you are finished"* and why "it compiles" is not evidence. | If an acceptance criterion starts to read like a capability list rather than a goal. | all |

## Clarifications resolved during spec

1. **The AC set is exactly the eight the first pass decided** — `AC-001` –
   `AC-008`, none added and none dropped. `AC-007` is the "nothing becomes public"
   criterion the Integration contract already named by id, and the other seven were
   derived from the **Behavior and interfaces** table so that every row there lands
   in a gated criterion rather than in prose. `_ledger.md` carries exactly these
   eight ids.
2. **`Query::index_arms()` is not owed and not written.** Two authoritative-looking
   sources name it (`RUNBOOK.md:4203`, `ARCHITECTURAL-EVALUATION.md:827`) and it
   does not exist. The architecture brief chose option (b) and this spec executes it;
   the re-open trigger (a second, unlike storage shape needing the same
   decomposition → its own ADR) is written into AC-007 so it survives this PR.
3. **`N` is deliberately left unspecified here.** The runbook's `400` is a
   proposal; ADR-0022 owns the number and the `rusqlite` limits-API-versus-documented-
   defaults trade. What this spec fixes instead is the *property* `N` must have:
   test-visible, and low enough that EC-004 cannot fire.
4. **AC-002 and AC-007 are in deliberate tension, and the tension is the point.**
   The boundary must be observable from a test that can only reach public API, while
   nothing new becomes public. Three candidate resolutions are listed under
   Implementation notes; the implementer picks one and records which in the ledger's
   evidence. Resolving it by adding `#[doc(hidden)] pub` is public surface wearing a
   hat and does not satisfy AC-007.

   **Amendment, 2026-08-17, after implementation and slice review.** The tension is
   not resolvable as written, and it was resolved in the implementation by deviating
   from AC-007's literal text. That is a spec amendment, so it is recorded here
   rather than left as a paragraph in `_ledger.md`'s evidence — an implementer
   flipping a criterion `satisfied` while stating a deviation from it is amending the
   spec in the wrong artifact.

   All three candidate resolutions fail, and one of them cannot be built at all.
   AC-002's own verification cell (`:412`) offers *"a `pub(crate)` planner function
   the integration target reaches"* — **impossible**: an integration target is a
   separate crate, so `pub(crate)` is exactly what it cannot reach. The
   `#[cfg(test)]` unit-module option is buildable but contradicts **Interaction
   quality → Reachability** (`:455`), which requires everything asserted to be
   asserted from an integration target and never through a private hook a `mod` test
   could reach. The "property of the result" option observes that every chunk
   *answered*, which is AC-003's assertion, not AC-002's: it cannot fail at
   `== 1`, because a single-statement plan returns the same union.

   **What was built, and what AC-007 now means.** `happenstance-sqlite` gains exactly
   two documented public items — `SqliteEventStore::MAX_QUERY_ARMS_PER_STATEMENT` and
   `SqliteEventStore::planned_statement_count` — and AC-007's clause *"the crate's
   only surface change is documentation"* is amended to *"…is documentation plus
   exactly those two items, which AC-002 compels"*. **The invariant AC-007 exists to
   protect is unchanged and holds**: `happenstance-core` and `happenstance-testkit`
   byte-identical, no `index_arms()`, no `arm_count()`, no `IndexArm`, no
   `MAX_QUERY_ITEMS`, no fourth `StoreLimit` variant, no new rule, registry entry or
   mutant row. What moved is one adapter-private crate's surface by two items, in the
   crate whose ceilings are already public facts (`MAX_EVENT_DATA_LEN` and its two
   siblings) rather than in the frozen contract.

   Two consequences, stated so they are not rediscovered. First, AC-007's stated
   verification *"`cargo doc -p happenstance-sqlite --no-deps` showing no new public
   item"* is retired by this amendment: it asserts the opposite of the disclosed
   deviation and cannot pass. `git diff --stat` over `crates/happenstance-core` and
   `crates/happenstance-testkit` being empty is the check that carries the invariant,
   and it does hold. Second, the widening is **local to this adapter**: it is not
   licence to publish a planner from `happenstance-core`, which is what ADR-0022 §10
   refuses and what the recorded re-open trigger — two unlike storage shapes needing
   the same decomposition, in its own ADR — still governs.
5. **ES-12 is discharged, not re-argued.** Making a read multi-statement is the
   shape ES-12 rejects, and the temptation is to invent a second isolation mechanism
   (a transaction, a `BEGIN`, a snapshot table). The clause and the rule both say the
   discharge is ES-11's ceiling carried identically onto every statement. AC-005 is
   written that way on purpose.
6. **No new conformance rule and no new mutant row.** `CLAUDE.md` requires a named
   plausible wrong implementation before a rule is added; here the rule exists and so
   does the falsifier (`ChunkedQueryStore`). Adding either would be decoration, and
   `happenstance-testkit` is outside this PR's boundary regardless.
7. **The composition family of RFC §6.7/D6 is N/A on a signed-off artifact**, not
   skipped: `../_design.md` records no user-facing surface for the whole project. The
   state family is fully carried, as `AC-###` table rows rather than as bullets, and
   the mapping is tabulated under **Interaction quality** so `redkiln verify` gates
   every one of them.
8. **`recorded_time_survives_a_reopen`'s missing negative control (testing brief
   AC-T06) is not this story's.** It belongs to
   `reopen-negative-control-and-durability-verdicts`. Named here only so its absence
   from this spec is a decision rather than an oversight.
