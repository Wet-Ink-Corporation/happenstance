# `experiments/correlated-exists-guard`

Does rewriting the append-condition chain's membership test from an
**uncorrelated** `position IN (SELECT …)` into a **correlated** `EXISTS (…)`
remove the multi-tag guard's cost class?

## The short answer

**Yes. Both fixes shipped, and the read path turned out to need a third that did
not.**

`crates/happenstance-sqlite/src/query_sql.rs` now emits a **correlated**
intersection chain with the guard's boundary bound into the seed arm. On a
10^6-event store, a two-tag guard inside `BEGIN IMMEDIATE`, median of 15
round-robin rounds:

| scenario | before | `chain-exists` | **+ bounded seed (ships)** |
| --- | ---: | ---: | ---: |
| accepted, boundary at head | 579,883 µs | 129 µs | **33 µs** |
| rejected, boundary mid-log | 580,703 µs | 151 µs | **37 µs** |
| rejected, **no boundary** | 549,047 µs | 169 µs | **65 µs** |

Four things, and the last two are the ones a reader should carry away:

1. `chain-exists` beats the old chain in **9 of 9 cells** — 189x at 50,000
   events, 3,248x–4,495x at 10^6 — and is near-flat in log length where every
   other shape grows.
2. **The boundary push alone is a conditional repair.** Bound into every arm of
   the *uncorrelated* chain it is excellent at a head boundary (137 µs) and
   worth nothing unanchored (561,838 µs). Bound into the *correlated* chain it
   composes cleanly and adds 2x–4x. The two fixes are not alternatives.
3. **The documented tag ordering was inverted, and is now correct.**
   `event_store.rs`'s public requirement to probe most-selective-first cost
   **36x–43x** on the old shape; it earns **2.0x–2.2x** on this one.
4. **The read path keeps far less of the win, because it has a second
   materialisation this does not touch.** 9.4x–15.7x on a selective query,
   1.7x on a broad one. That second wrapper is finding I-3, it is now the read
   path's floor, and [`results/read-path.md`](results/read-path.md) prices three
   candidate repairs and proposes an approach without implementing one.

## What shipped, and what did not

**Shipped** — `crates/happenstance-sqlite`, gate green, 89 conformance rules
plus the adapter's own 200 tests:

* the chained membership test rewritten from an uncorrelated
  `position IN (SELECT position FROM event_tag WHERE tag = ?)` to a correlated
  `EXISTS (… WHERE tag = ? AND position = seed.position)`;
* the guard's boundary bound into the seed arm — `chunks` gained a `bound`, and
  `evaluate` keeps its Rust-side `highest > boundary` comparison, which the bound
  makes trivially true rather than load-bearing;
* the two public doc blocks that described the old shape, corrected, with the
  measurement cited and a note saying what must change with them if the chain is
  ever returned to `IN (…)`.

* the outer `position IN (<matched>)` wrapper **omitted for `Query::all`**,
  where it is a tautology over the whole table and no estimate is needed to know
  it. At 500,000 events a page 90% of the way through a replay went from
  56,310 µs to 182 µs, and a full replay from quadratic in the log's length to
  linear ([`results/all-query-wrapper.md`](results/all-query-wrapper.md)) — for
  a reason that is **not** the one the change was proposed on.

**Not shipped, deliberately:**

* any repair of that wrapper for a **tagged** query. The best candidate is
  **1,089x** better on a broad query and **2.3x worse** on a selective one, so
  the choice is cardinality-conditional — a decision about when the adapter
  changes its query plan on data it samples. That is ADR-0022's, not an
  experiment's.
* any ADR. ADR-0022 §8 fixes the multi-tag test as a `GROUP BY` aggregate that
  has never been what runs, and its §16 falsifier names that aggregate and so
  cannot fire. The shipped shape has now moved again. That gap is recorded in
  `references/seeds/adr-0022-shipped-shape-drift.md` for the runbook's ADR pass
  to settle.

## This is not a crate anybody depends on

It is **not a workspace member** — `Cargo.toml` carries an empty `[workspace]`
table, the same trick every crate under `experiments/` uses — it appears in no
`verify:` command and no `cargo xtask ci` step, and it adds no dependency to any
workspace manifest. `Cargo.lock` at the repository root is untouched. CF-34 is
why: a benchmark that can turn a merge red teaches people to re-run until green.

**Nothing here changes `happenstance-sqlite`.** The shape under test lives in
this crate's transcription of the adapter's SQL builder. Whether the adapter
adopts it is ADR-0022's to decide, and §"What this does not settle" below says
what that decision still needs.

## Why it exists

`experiments/shipped-append-condition-sql/` established three things about the
guard the adapter emits:

* it is an intersection chain (`query_sql.rs:198-235`), not the
  `GROUP BY … HAVING COUNT(DISTINCT tag) = n` aggregate ADR-0022 §8 decided on
  and measured;
* the chained `position IN (SELECT …)` is **uncorrelated**, so SQLite
  materialises it once and in full — `LIST SUBQUERY 1` over every row carrying
  the second tag — and drives from that list, demoting the seed arm to the probe
  side;
* consequently the documented most-selective-first ordering puts the *unselective*
  tag into the materialisation, and costs 39x–44x.

That experiment measured four shapes and closed. It named binding the boundary
into every arm as the best available repair, and left one shape unbuilt: making
the subquery **correlated**, so it cannot be hoisted at all. This crate builds
it.

The prediction, written before the run and preserved in
[`src/chain.rs`](src/chain.rs)'s `exists_sql` documentation, was that the cost
would fall from `|chained tag|` to `|seed tag| x log(|chained tag|)`. It was
explicitly flagged as *not obviously right*: SQLite might flatten the `EXISTS`
into a join and materialise anyway, and a correlated subquery pays a fresh
b-tree descent per outer row where a materialised list pays one hash probe.

## The one edit

```diff
-AND position IN (SELECT position FROM event_tag WHERE tag = ?)
+AND EXISTS (SELECT 1 FROM event_tag AS m0 WHERE m0.tag = ? AND m0.position = seed.position)
```

`EXPLAIN QUERY PLAN` is the whole mechanism, and both plans come from the same
run at the same boundary on the same 10^6-event store
([`results/query-plans.md`](results/query-plans.md)):

```
chain-as-shipped                          chain-exists
  SEARCH event_tag USING PRIMARY KEY        SEARCH seed USING PRIMARY KEY (tag=?)
    (tag=? AND position=?)                  SEARCH m0 EXISTS USING PRIMARY KEY
  LIST SUBQUERY 1                             (tag=? AND position=?)
    SEARCH event_tag USING PRIMARY KEY
      (tag=?)
```

**The `LIST SUBQUERY` is gone.** `(tag, position)` is the primary key of a
`WITHOUT ROWID` table, so the correlated `EXISTS` is answered by a point seek,
and the seed arm becomes the outer loop rather than the probe side. That is why
the seed's selectivity starts mattering, which is why the ordering policy flips
sign.

How many of those seeks actually happen is a separate question, and the
adversarial corpus answers it *against* the obvious reading — see item 2 of
"What this does not show".

## Conformance first, measurement second

**A shape that is fast and wrong wins every benchmark**, and this shape has an
unusually quiet way to be wrong. The correlation is spelled with a table alias;
without one, `position = event_tag.position` resolves to the *inner* table and
the predicate becomes the tautology `position = position`. No compile error, no
runtime error — a two-tag guard silently behaving as a one-tag guard, returning
a superset, and posting the best number in every cell.

So `run.sh` step 1 runs the full conformance suite against all six shapes before
any clock starts: **534 tests, 6 shapes x 89 rules, all passing**
([`results/raw/conformance.txt`](results/raw/conformance.txt)). Step 3 re-checks
that this crate's transcription of the *shipped* chain is still what the adapter
emits, off a `sqlite3_trace_v2` callback — the baseline every ratio is taken
against.

## Conditions

Every figure was produced under these, and the pragma row is **read back off the
live connection** by `happenstance_sqlite::connection::ConnectionSettings::read_back`
rather than trusted from the `PRAGMA` that issued it — SQLite silently ignores a
`journal_mode` it cannot honour. Every printed row in `results/raw/` carries the
settings string beside it.

| | |
| --- | --- |
| Machine | 13th Gen Intel Core i9-13905H, 14 cores / 20 logical, 31.7 GiB RAM |
| OS | Windows 11 Home, 10.0.26200 |
| Filesystem | NTFS; every database under `%TEMP%` on the internal NVMe |
| Toolchain | `rustc 1.97.1`, `x86_64-pc-windows-msvc`, `--release` for every timed step |
| SQLite | 3.53.2, `rusqlite` 0.40 `bundled` — the driver the workspace names |
| Pragmas | `journal_mode=wal synchronous=1 busy_timeout_ms=5000`, read back |
| Page cache | SQLite's default (`cache_size = -2000`, 2 MB) — the adapter sets none, and it is smaller than every index scanned |
| Corpus | Two. `row:rN` (one position in 97) with `shard:cold` (every position) — the sibling experiment's shape, unchanged — and `all:yes` with `shard:cold`, both matching every position, which is `unselective-pair.md`'s |
| Command | `./run.sh`, about twenty-five minutes |
| Run | 2026-09-05 — **the one run every figure comes from.** Nothing in `results/raw/` is carried across runs |

## Findings

Full tables under [`results/`](results/README.md).

1. [`guard-cost.md`](results/guard-cost.md) — six shapes x four scenarios x three
   log sizes, round-robin within each round so no shape is permanently first.
2. [`query-plans.md`](results/query-plans.md) — the plan for each shape, which is
   the mechanism rather than the timing.
3. [`seed-ordering.md`](results/seed-ordering.md) — the ordering policy on both
   shapes, alternating which ordering runs first every round.
4. [`unselective-pair.md`](results/unselective-pair.md) — **the adversarial
   corpus**: no tag selective, so the seed arm has nothing to work with. Written
   to be able to overturn the other three.
5. [`read-path.md`](results/read-path.md) — **the read path**, which enumerates
   instead of taking a maximum. Where most of the guard's margin turns out to
   have come from, where I-3's second materialisation lives, and the three
   candidate repairs for it, priced.
6. [`all-query-wrapper.md`](results/all-query-wrapper.md) — the one of those
   three that shipped, why its stated mechanism was wrong, and what the real one
   costs a replay.

## What this does not show

1. **One machine, one run per cell.** Every claim is a ratio between shapes of
   one run, or an order of magnitude. Nothing here should be quoted to a third
   significant figure.
2. **The predicted scaling law is refuted, and nothing replaces it.**
   `exists_sql` predicted `|seed| x log(|chained|)`. At 500,000 events the
   selective corpus has a 5,155-row seed and costs 130 µs; the unselective corpus
   has a **500,000**-row seed and costs 128 µs. The seed grew 97x and the cost
   did not move, so the cost is not one seek per seed row. The likely mechanism
   is the `max()` the adapter wraps every guard in
   (`event_store.rs:658-672`) letting SQLite walk the position-ordered seed
   descending and stop at the first satisfying row — but that is a hypothesis
   consistent with the numbers, not a measured fact.
3. **The read path is measured, and it keeps far less of the win.** 9.4x–15.7x
   on a selective query, 1.7x on a broad one, against the guard's 1,617x–1,681x
   at the same store size. Item 2's early-exit reading is therefore confirmed:
   most of the guard's margin is `max()` stopping early.
   [`read-path.md`](results/read-path.md) also shows the read path carries a
   **second** materialisation the rewrite does not touch — I-3's outer
   `position IN (<matched>)` wrapper — which sets that path's floor. Candidate
   repairs are **measured**; none is **built**, and the crossover between them
   is characterised by two points with nothing in between.
4. **Two tags throughout.** Three or more chain additional `EXISTS` clauses and
   are not measured. Two corpora are: 97:1 and 1:1. A middle case — two
   moderately selective tags whose intersection is smaller than either — is
   between them and is measured by neither.
5. **No append is performed, and no full drain.** The guard is evaluated inside
   `BEGIN IMMEDIATE` and rolled back, exactly as the sibling experiment does;
   the read path is measured one page at a time. A real replay would also pay
   `ReadCursor`'s per-hop `spawn_blocking` and connection mutex, which
   `experiments/one-connection-latency/` measures and this does not.
6. **This is not a patch to `happenstance-sqlite`.** The shape lives here. What a
   real adoption also has to answer: I-3's outer wrapper (item 3), whether the
   `EXISTS` form composes with statement chunking at
   `MAX_QUERY_ARMS_PER_STATEMENT = 400`, whether `Selectivity::read_for` is
   still worth its own statement now that ordering buys 2.2x instead of costing
   40x — I-5 measured that lookup at 40x a `BTreeSet` at VT-23's item floor,
   *inside the write lock* — and whether older SQLite versions plan it the same
   way. None of those is measured here.
7. **The `GROUP BY` form is still ADR-0022's decision.** This crate shows the
   chain can be repaired; it does not show the chain is the right shape. Both
   remediated chain forms beat `grouped-adr0022` in all nine cells, which is
   evidence for the chain — but ADR-0022 §16's falsifier names the aggregate,
   and re-opening that record is the runbook's to schedule, not this experiment's
   to pre-empt.
