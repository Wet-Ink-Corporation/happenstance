---
item: HS-S0039
stage: discover
created: 2026-08-12T13:02:02.707Z
updated: 2026-08-12T13:02:02.707Z
template_sig: 86ce4036
rendered_sig: 9c97029f
---

# Discover — A wide Query is chunked and merged, never refused

## Signal Ledger

Evidence gathered: prior art, constraints, and the problem framing.

| Signal | Source | Implication |
| ------ | ------ | ----------- |
| Slice one-line: decompose a wide `Query` into `ceil(arms/N)` prepared statements **privately inside `happenstance-sqlite`** and k-way-merge their cursors, de-duplicating on position and applying the page budget and `ReadOptions::limit` to the merged output. | `_storymap.md`, *Slices* table, row `durable-event-store` / `wide-query-chunked-not-refused` | "Privately" is a decision, not a location detail — see the `index_arms()` row below. And the merge, not the SQL, is where the query rules actually get decided. |
| **AC-008** — a wide query is chunked, not refused: a `Query` whose arms exceed SQLite's pushdown limit is served by merged cursors over `ceil(arms/400)` statements, VT-23's 128-item minimum passes, and "the named wrong implementation is one that returns an error at the limit." | `project.md`, AC-008 | The AC names one mutant; this story's job includes finding the ones it does not name, because a store that *refuses* is loud and a store that merges wrongly is silent. |
| `dependsOn: lazy-read-with-snapshot-ceiling` (HS-S0038) — it supplies the paged cursor, the snapshot ceiling and the inclusive `resume_from` that each chunk statement must share. | `_storymap.md`, *Merge order* item 2 | "Each chunk statement must be ordered by position and bounded by the same ceiling and `resume_from`" (`_decomposition.md`, *Architecture brief*, §5). Chunks that do not share one ceiling are `k` different snapshots. |
| **`Query::index_arms()` does not exist.** `crates/happenstance-core/src/query.rs` has `Query::items() -> Option<&[QueryItem]>` (`:204`), `QueryItem::types()` (`:102`) and `QueryItem::tags()` (`:107`), and no `index_arms`, `arm_count` or `IndexArm`. The name is inherited from an evaluation *proposal*. | `_decomposition.md`, *Architecture brief*, §5; `_grounding.md`, *Tensions*; `references/evaluation/ARCHITECTURAL-EVALUATION.md:827`; `RUNBOOK.md:4203-4206` | Both the runbook's work item and the project AC name an API that was never minted. ADR-0022 must record the rejection explicitly "because the runbook's own work item names the API and a reader will otherwise think it was forgotten." |
| Decision already taken: option (b), decompose privately inside `happenstance-sqlite`. Adding public surface to the contract crate for the benefit of *one* adapter is what `CLAUDE.md` warns about — "one implementor is not a spread." The re-open trigger is named: if `postgres-and-neon-stores` needs the same decomposition, that is two unlike shapes agreeing, and *that* mints the API as its own ADR. | `_decomposition.md`, *Architecture brief*, §5; `CLAUDE.md`, *The rule that matters* | AC-A04 makes it binding: no new public API in `happenstance-core` for this adapter's convenience without an ADR that says so. |
| VT-23 `[PROVISIONAL]`: no `MAX_QUERY_ITEMS` constant, `Query::from_items` must not enforce a count, every store must evaluate at least 128 items. It explicitly rejects "an adapter that generates one SQL parameter per item and silently fails past a driver limit, and an adapter that emits one statement per item without an upper bound on the round trips that implies." | `spec/SPECIFICATION.md:1535-1552`; `crates/happenstance-core/src/limits.rs:32` | Two named wrong implementations already, and neither is "returns an error at the limit" — the clause is watching for *silent* failure and for unbounded round trips. |
| The rule is `store_evaluates_a_query_at_the_guaranteed_minimum_item_count`. | `crates/happenstance-testkit/src/registry.rs:182`; `crates/happenstance-testkit/src/suite.rs:3880` | 128 items is the only width the suite exercises. `RUNBOOK.md:4204` proposes chunking at `N = 400` — so **the suite never crosses the chunk boundary**. That is this story's central problem. |
| The merge must de-duplicate on position and apply the budget and `ReadOptions::limit` to the **merged** output, never per chunk. | `_decomposition.md`, *Architecture brief*, §5 | Named rules: `duplicate_items_do_not_duplicate_events` (`suite.rs:636`), `query_items_are_or` and `query_union_is_item_concatenation` (`registry.rs:116`, `:121`), `limit_applies_across_items_not_per_item` (`suite.rs:1402`), `read_limit_applies_after_filtering` (`registry.rs:130`). |
| The real ceiling is SQLite's compiled-in compound-select and bound-parameter limits; `rusqlite` is pulled `default-features = false, features = ["bundled"]`, so a limits API may need a feature added — weighed against documenting the bundled defaults in ADR-0022. | `_decomposition.md`, *Architecture brief*, §5; `crates/happenstance-sqlite/Cargo.toml` | A workspace manifest change is visible to `cargo deny`, which the gate runs. Not free. |
| `Query::All` is one arm (a bare scan over `event`); an item's arm count is a function of its type set and tag set, after the most-selective-tag choice. | `_decomposition.md`, *Architecture brief*, §5 | The `tag_cardinality` table from `schema-migration-and-identity` is what makes "most selective" answerable — `ANALYZE` stores only an average. |
| No conformance rule is added by this story: AC-008 "requires the *behaviour* … and never the public name", and "no unit test of `index_arms()` exists to write". | `_decomposition.md`, *Architecture brief*, §5; *Testing brief*, §2, AC-008 row | The literal-position bar is vacuous here — but the merge is exactly where a `+1` position assumption would be introduced, and `AUTOINCREMENT` guarantees gaps. |

## Questions

Open questions to resolve before specifying.

1. **What is `N`?** Deferred to `spec`, and it is deferred *with a method*:
   `RUNBOOK.md:4204` proposes 400, but the real bound is SQLite's compiled-in
   compound-select and bound-parameter limits. The spec chooses between probing
   them at open (possibly a `rusqlite` feature addition, and therefore a workspace
   manifest change `cargo deny` sees) and documenting the bundled defaults in
   ADR-0022. What discover fixes: `N` must be a named constant with its provenance
   in a comment, not a literal buried in a query builder.
2. **What exactly is an "arm"?** Answered as shape: one prepared-statement branch
   derived from a `QueryItem` after the most-selective-tag choice, with
   `Query::All` counting as one. The exact function from `(types, tags)` to arm
   count is `spec`'s.
3. **Is the merge streaming or buffered?** Deferred to `spec`, with one hard
   constraint from ADR-0001/ADR-0008: no `Statement`, `Rows` or `Transaction` may
   live in a field across a `poll_next` boundary
   (`crates/happenstance-sqlite/src/event_store.rs:248-271`), and
   `crates/happenstance-sqlite/tests/shapes.rs` is the guard. A k-way merge that
   holds `k` open cursors is the obvious design and is exactly the one that
   breaks it.
4. **Does `index_arms()` get minted in `happenstance-core`?** Answered: **no**,
   per architecture brief §5 and AC-A04, with the re-open trigger recorded rather
   than left to memory. `EventStore` is `[FROZEN]`
   (`spec/SPECIFICATION.md:371`); `Query` is a different item and the addition
   would be purely additive, so the reason for declining is the spread argument,
   not the freeze — and saying which reason applies is part of the record.
5. **Does a chunked read change the meaning of the snapshot ceiling?** Answered:
   no, and that is a requirement rather than an observation. All chunks share one
   ceiling captured at the first poll of the *stream*, not one per chunk.
6. **The append-condition SQL strategy.** Not this story's. It shares the tag
   index with this read path, so ADR-0022's tag-layout choice reaches here, but
   the strategy is not decided or amended by this slice.

## Decision

`Query` bounds nothing by design and a hard cap in the contract would be wrong,
so the adapter has to absorb a width that its driver cannot push down in one
statement. This slice makes a wide query a *plan* rather than an error:
`ceil(arms / N)` prepared statements, each ordered by position and bounded by the
same snapshot ceiling and inclusive `resume_from` the previous story established,
merged in Rust. The spec will cover: the private arm decomposition built on the
already-public `Query::items()` / `QueryItem::types()` / `QueryItem::tags()`
accessors, with **no new public API in `happenstance-core`** and ADR-0022
recording that rejection and its named re-open trigger; the chunk width `N` as a
constant carrying its provenance; the most-selective-tag choice served by
`tag_cardinality`; and — the part that actually decides the query rules — a k-way
merge that de-duplicates on position and applies the page budget and
`ReadOptions::limit` to the **merged** output. This story adds no conformance
rule, so the literal-position bar is vacuous for it; the merge nevertheless must
compare positions the store assigned rather than assume contiguity, because
`AUTOINCREMENT` leaves gaps. No `[FROZEN]` clause is amended — the frozen
`EventStore` cell is a reason this story stays adapter-private, not something it
changes.

## The wrong implementation

**The mutant: chunking that applies `ReadOptions::limit` and the page budget
per chunk instead of to the merged output.** Each of the `k` statements gets
`LIMIT n`, the results are merged, and the caller gets up to `k × n` events. It is
the obvious implementation, because pushing the limit into each statement is the
thing that makes chunking fast, and it is the one an optimiser reaches for.

Every check passes. `limit_applies_across_items_not_per_item` (`suite.rs:1402`)
and `read_limit_applies_after_filtering` (`registry.rs:130`) are exactly the rules
written to catch this — and they cannot, because with `N = 400` and VT-23's floor
at 128 items (`spec/SPECIFICATION.md:1535-1539`), **every query the suite builds
is one chunk**, and with one chunk per-chunk and merged are the same number. The
adapter is conformant on paper and returns three times the requested events the
first time a real caller writes a decision model wider than 400 arms. VT-23 itself
already names the shape of this failure class — "silently fails past a driver
limit" — which is the clause telling us in advance that the danger is silence, not
refusal.

**The second mutant, sharper because it needs no limit at all: a merge that
concatenates chunk cursors rather than merging them, or merges without
de-duplicating on position.** An event carrying two tags that land in two
different chunks is yielded twice, and events arrive grouped by chunk rather than
in position order. `duplicate_items_do_not_duplicate_events` (`suite.rs:636`),
`query_items_are_or` and `query_union_is_item_concatenation`
(`registry.rs:116`, `:121`) are the rules that discriminate, and again all three
run at one chunk, where concatenation and merging are indistinguishable.

**Where the control must live.** Not in the testkit's `mutation_coverage/`: those
mutants are `Rc`/`RefCell` stores holding their log in a `Vec`
(`crates/happenstance-testkit/tests/mutation_coverage/harness.rs:24-30`) and have
no notion of a driver pushdown limit, so a chunk-boundary defect has no expression
there. And it must **not** be fixed by raising VT-23's floor to force the suite
past 400 — VT-23 is a `[PROVISIONAL]` clause about what every store must evaluate,
not a lever for testing one adapter, and moving it would impose 400+ item queries
on every adapter in the workspace. The control belongs to this story, in
`crates/happenstance-sqlite/tests/`: adapter-owned tests that construct a query
with strictly more arms than `N` — trivial once `N` is a named constant the test
can read — and assert (a) the merged output equals the single-statement result
for the same query at a smaller `N`, (b) a `limit` is honoured across the whole
merge, and (c) an event matching arms in two different chunks appears exactly
once. All three compare against positions the store actually assigned. If `N` is
not test-visible, the tests cannot be written, which is the real argument for
making it a constant rather than a literal.

## Gate: Discover

Tick each box once the sections above satisfy it. `redkiln advance` reads these
against `.redkiln/templates/gates/discover.md` — where each box's rationale is
written — and will not leave `discover` until every one is ticked. Keep each box on
one line; the parser matches line by line.

- [x] The problem is framed in one paragraph.
- [x] Prior art and constraints are recorded in the signal ledger.
- [x] Open questions are either answered or explicitly deferred.
- [x] The next stage (spec) has a clear starting point.
- [x] The wrong implementation this work rejects is named.
- [x] No conformance rule added here asserts a literal position value.
- [x] Any `[FROZEN]` clause this touches is changed by a new ADR, written first.
- [x] If a rule here seems wrong, it is fixed and the reason given in the same change.
