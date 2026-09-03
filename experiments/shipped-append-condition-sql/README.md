# `experiments/shipped-append-condition-sql`

What the append-condition SQL `happenstance-sqlite` **actually emits** costs, and
whether binding the guard's boundary into its seed arm changes the cost class.

This is not a crate anybody depends on. It is **not a workspace member** (its
`Cargo.toml` carries an empty `[workspace]` table, the same trick
`experiments/append-condition/` and `experiments/wire-format/` use), it appears
in no `verify:` command and no `cargo xtask ci` step, and it adds no dependency
to any workspace manifest — `Cargo.lock` at the repository root is untouched.
CF-34 is why: a benchmark that can turn a merge red teaches people to re-run
until green.

## Why it exists

ADR-0022 §8 fixes the general multi-tag superset test as
`GROUP BY position HAVING COUNT(DISTINCT tag) = n`, and derives two shipped
requirements from measurements **of that form**: the single-tag fast path
(1,093 µs → 556 µs, 1.97x) and `tag_cardinality` (a two-tag boundary at
*"42 ms to 66 ms per guard evaluation"* over a 50,000-event log, *"roughly
200x"* a single-tag one). `crates/happenstance-sqlite/src/query_sql.rs:43-54`
and `event_store.rs:91-96` repeat those numbers in the crate's own **public**
module documentation.

The adapter emits neither form. `query_sql.rs:195-233` emits a correlated
intersection chain seeded by the most selective tag, with no aggregate anywhere.
That shape was never built as an arm in `experiments/append-condition/`, never
conformance-tested there, and never timed. ADR-0022 §16's falsifier for §8 —
*"re-open it if a future SQLite pushes predicates through an aggregate, at which
point the special case stops earning its branch"* — therefore cannot fire, because
the aggregate it names is not what runs.

Separately, `event_store.rs:658-672` wraps each chunk in
`SELECT max(position) FROM (…)` and compares `highest > boundary` in **Rust**. No
boundary predicate reaches SQLite on either caller, so the pushdown the module
doc credits the chain with (*"keeps the boundary pushable"*) is a mechanism the
shipped strategy never exercises.

## Conditions

Every figure below was produced under these, and the pragma row is **read back
off the live connection** by `happenstance_sqlite::connection::ConnectionSettings::read_back`
rather than trusted from the `PRAGMA` that issued it — SQLite silently ignores a
`journal_mode` it cannot honour. Every printed row in `results/raw/` carries the
settings string beside it.

| | |
| --- | --- |
| Machine | 13th Gen Intel Core i9-13905H, 14 cores / 20 logical, 31.7 GB RAM |
| OS | Windows 11 Home, 10.0.26200 |
| Filesystem | NTFS. **The databases live under `%TEMP%` on `C:`, which is the internal SK Hynix HFS001TEJ9X115N NVMe** — not the external USB SSD the repository is checked out on. |
| Toolchain | `rustc 1.97.1 (8bab26f4f 2026-07-14)`, `cargo 1.97.1`, `x86_64-pc-windows-msvc` |
| Build | `--release` for every timed run (`[profile.release]` in `Cargo.toml`: `debug = false`, `lto = false`) |
| SQLite | 3.53.2, `rusqlite` 0.40.2 `bundled` |
| `journal_mode` | `wal` |
| `synchronous` | `1` (`NORMAL`) |
| `busy_timeout` | 5,000 ms |
| Page cache | SQLite's default (`cache_size = -2000`, i.e. 2 MB). **The adapter sets no `cache_size`,** so this is the shipped value and it is smaller than every index scanned below. |
| Chunk width | `SqliteEventStore::MAX_QUERY_ARMS_PER_STATEMENT` = 400 |
| Page size | `event_store.rs:141`'s `PAGE_SIZE` = 512 rows |
| Command | `./run.sh` |
| Run | 2026-09-03, 06:14–06:27 local — **the one run every figure below comes from.** `results/raw/seed-ordering-replication.txt` is a later re-run of step 6 alone, and is the only file in `raw/` that is not from it |
| Wall clock | thirteen minutes. The harness-reported durations sum to 730 s: conformance 1.92 s, conditions 0.00 s, emitted SQL 0.06 s, guard cost 557.12 s, query plans 111.30 s, seed ordering 49.48 s, selectivity 10.53 s |

Nothing here sets `synchronous = OFF`. `spec/SPECIFICATION.md:7481-7484` names it
by name as a wrong implementation CF-14's reopen rule rejects, and the settings
are printed rather than assumed so that a reader can check.

## What is varied, and what is held fixed

**One axis: the string between the parentheses of `SELECT max(position) FROM (…)`.**

Migration 1 is applied by calling the *shipped adapter's own* `SqliteEventStore::open`
on the file; the connection comes from its own `connection::open_configured`; the
chunk width is its own public constant; the row codec is its own `0x1F`-delimited
canonical tag column. Everything that is not the guard is borrowed rather than
transcribed, which is why the four shapes read identical bytes out of one shared
database file per log size.

| shape | what it is |
| --- | --- |
| `chain-as-shipped` | `query_sql.rs:195-233` verbatim: seed arm `tag = ?`, most selective first, then one `AND position IN (SELECT position FROM event_tag WHERE tag = ?)` per remaining tag |
| `chain-bounded-seed` | the same chain with `AND position > ?` — the guard's own boundary — bound into the **seed** arm |
| `chain-bounded-all-arms` | the boundary bound into the seed **and** every chained membership subquery |
| `grouped-adr0022` | ADR-0022 §8's `GROUP BY position HAVING COUNT(DISTINCT tag) = n`, single-tag fast path included — `experiments/append-condition/src/tags.rs`'s `JoinTable::item_sql` |

The fourth shape was added **after** the third was measured and found to change
nothing. Its reason is in `results/query-plans.md`: the chained subquery is
uncorrelated, so SQLite materialises it once in full, and a boundary the seed
carries cannot reach that materialisation.

The seeded log is `experiments/append-condition/`'s: events of type `Seeded`,
each tagged `shard:cold` (matching all of them) and `row:r{n mod 97}` (matching
about one in ninety-seven). That contrast is the shape `tag_cardinality` exists
to order, and reusing it is what makes the 50,000-event figures comparable with
§1's recorded ones.

## The transcription is checked, not asserted

`crates/happenstance-sqlite/src/query_sql.rs` is a private module, so its
builders cannot be called from outside the crate. They can be **watched**:
`tests/emitted_sql.rs` installs a `sqlite3_trace_v2` callback on the connection
it hands a real `SqliteEventStore`, drives a real conditional append and a real
paged `Query::all` replay, prints the statements verbatim, and asserts **byte for
byte** that this crate's `chain-as-shipped` builder emits what the adapter
emitted. `results/raw/emitted-sql.txt` is that output untouched. If
`query_sql.rs` changes, that test fails and every figure here is known stale
rather than quietly wrong.

## Conformance first, measurement second

**A wrong arm is always the fastest**, and the hazard here is sharper than usual:
two of the four shapes answer the guard question by asking SQLite a *narrower*
one — "is anything above the boundary" rather than "what is the highest match,
anywhere". An arm that answers a boundary question faster by answering it less
completely wins every timing and is worthless.

So `tests/arms_are_conformant.rs` points `happenstance_testkit::event_store_conformance!`
at each shape — **356 tests, 89 rules × 4 shapes** — and `run.sh` runs it first.
Beyond that:

* every timed round asserts each scenario actually took the path it names
  (accepted guards return `None`, rejected ones return a position);
* the first and last round of every size assert all four shapes returned the
  **same** answer;
* `tests/seed_ordering.rs` asserts both tag orderings return the same position
  before either time is recorded;
* `seed::verify` checks the `event` count, the `event_tag` count, that no row is
  left unstamped, and that `tag_cardinality` agrees with the rows written —
  before any clock starts, because `tag_cardinality` is what orders the chain's
  seed arm and a wrong count there would silently measure a different shape.

## Findings

Full tables in [`results/`](results/README.md), and a verdict against each
reviewed finding in [`results/README.md`](results/README.md). Every figure below
is a row in `results/raw/` from the one `./run.sh` the conditions table names —
with a single stated exception, the replication figures for the seed-ordering
result, which come from `results/raw/seed-ordering-replication.txt`. Nothing here
is carried across runs except where it says so. In one paragraph each.

**The shipped chain is slower than the aggregate it replaced, at every size.** A
two-tag guard anchored at head — the path an application takes on almost every
conditional write — costs **29.8 ms** at 50,000 events, **296 ms** at 500,000 and
**560 ms** at 10^6, against the `GROUP BY` form's **19.2 ms**, **185 ms** and
**362 ms**. Across all three two-tag scenarios and all three sizes that is nine
cells at 1.54x to 1.86x, all in the same direction, reproducibly, interleaved, on
one shared file. `query_sql.rs:34-54` justifies the chain by reasoning from the
`GROUP BY` form's 1.97x single-tag measurement; the chain was never measured
against it, and when it is, it loses.

**Binding `position > ?` into the seed arm changes nothing — not the number and
not the cost class.** At 10^6 events the two-tag guard costs 560 ms as it ships
and 571 ms with the boundary in the seed (+2.0%, against a p10→max band of
485–4,685 ms and 517–741 ms respectively); at the mid-log boundary, where half the
log is skippable, 593 ms against 549 ms. Nine cells, no consistent sign, none of
them outside its own spread. `EXPLAIN QUERY PLAN` says why, and it is the
*opposite* of what the finding predicted: the seed is not the driving table.
SQLite materialises the chained subquery as a `LIST SUBQUERY` over the whole of
the **least** selective tag and drives from that, seeking
`(tag = <seed>, position = ?)` once per entry. The two plans are identical line
for line. A range restriction on the seed therefore restricts nothing that runs.

**Bind it into every arm and the cost class does change — by 3,126x.** The same
accepted-at-head guard at 10^6 events costs **179 µs** instead of 560 ms, because
the materialisation is what shrinks. The control that makes that credible rather
than suspicious is the other two boundaries: at mid-log, where exactly half the
log survives the predicate, it costs 254 ms against 593 ms — half, as the
mechanism predicts — and at a boundary of zero, where nothing can be discarded, it
costs 534 ms against 546 ms, which is nothing at all. Three predictions, three
sizes, nine agreements. This is a shape nothing in the repository has ever built,
and it is the only one of the four that makes the crate's own *"keeps the boundary
pushable"* sentence true.

**`tag_cardinality`'s most-selective-first ordering costs the shipped chain 40x.**
`event_store.rs:91-96` states in the crate's public documentation that multi-tag
items *"must be probed most-selective-tag-first"*, and ADR-0022 §8 makes it a
migration-1 requirement. Swapping the two tags — the same builder, the same
statement text, the same rows, the same connection, alternating which ordering
goes first, asserting an identical answer every round — moves the 500,000-event
two-tag guard from **511 ms to 11.8 ms**. A fresh replication run on a new store
moved it from 646 ms to 16.4 ms. Six cells, two runs, 38x to 44x, one direction.
The mechanism is the plan above: the ordering decides which tag SQLite
materialises, and the stated policy always picks the larger one. The sentence was
measured on the `GROUP BY` form, where `tag IN (?,?)` is order-independent and the
question does not arise, and inherited by a shape where the planner picks the
driving side itself.

**The paged read carries a tautology that costs 846x.** `fetch_page` emits
`… FROM event WHERE position IN (SELECT position FROM event) AND position >= ?
AND position <= ? ORDER BY position ASC LIMIT ?` for `Query::all` — 1,954 times
over a 10^6-event log. At page 1,000 that statement takes **60.1 ms**; the same
statement with the `position IN (…)` clause deleted takes **0.071 ms** and
returns the identical 512 rows. The plan says why: with the clause, SQLite drives
the query from the IN-operator (`USING ROWID SEARCH ON TABLE event FOR
IN-OPERATOR`) instead of from the `position` range it already has. A full
`Query::all` replay of 10^6 events through the real adapter took **97.2 s** —
49.8 ms per page for everything the adapter does, against 0.071 ms for the
statement without the clause.

**`Selectivity::read_for`'s deduplication is 40x its own replacement at the
specification's own floor.** A 128-item query carrying 128 distinct tags each —
VT-23's `MIN_SUPPORTED_QUERY_ITEMS` with nothing bounding tags per item —
presents 16,384 tags to the `Vec::contains` accumulation, which takes **257.7 ms**
against a `BTreeSet`'s **6.4 ms**, output asserted identical. It runs once per
read page and once per append guard. `SqliteEventStore::planned_statement_count`,
which is public and reaches only the *other* quadratic (`distinct_tags`), takes
4.6 ms on the same query.

**And that query is one item wider than the adapter can prepare.** 400 items —
`MAX_QUERY_ARMS_PER_STATEMENT` exactly — carrying 128 tags each binds **51,600**
parameters against SQLite's limit of 32,766, in a 3.2 MB statement string.
`chunks` reports one chunk and the public `planned_statement_count` agrees the
plan is one statement; `prepare` returns `too many SQL variables`.
`Selectivity::read_for` fails first, at 300 items, and on the append path it fails
inside `BEGIN IMMEDIATE` with the write lock held.

## Two honest limitations, and they change how the tables read

**1. The 50,000-event calibration did not reproduce ADR-0022 §1's figure.** §1
recorded 42,399 µs for a two-tag rejection at 50,000 events on the `GROUP BY`
form; the same SQL shape here costs 14.1 ms on the matching cell
(`rejected-2tag-unbounded`) and 18.0 ms on the mid-log one, about 3.0x less. The most likely
reason is page-cache state between measurements rather than a disagreement about
the SQL: `experiments/append-condition/tests/contention_at_64.rs`'s `sequential`
performs two full `append`s between every rejection timing, and that experiment's
`stamp_identity` is `UPDATE event … WHERE origin_position IS NULL` with **no**
positional bound (`candidate.rs:296-303`), which scans the whole `event` table
and evicts `event_tag` from a 2 MB cache every round. This experiment's rounds
leave the index warm. Both are honest; they are different operating conditions,
and that experiment's own README records 45% run-to-run variance on this host
besides. **So the absolute numbers here are warm-cache numbers.** The
cold/warm spread is visible in `results/query-plans.md`, where the first
execution of the shipped chain at 10^6 rows cost seconds rather than the
hundreds of milliseconds the interleaved medians report.

**2. Every comparison here is between shapes, on one file, in one interleaved
run.** That is what the verdict rests on, and it is the strongest claim the host
supports. It is *not* a throughput number for the adapter, and it is not a claim
about any machine but this one.

Four smaller ones, stated because a reader will otherwise assume them away:

* The seeded log has **two tags per event and one event type**. A guard crossing
  three or more tags adds chain links this experiment did not measure — and the
  seed-ordering result in particular falsifies the stated policy for two tags
  without supplying the replacement policy for three.
* The seed writes rows in position order with explicit positions, so `event_tag`
  is built by the same insertion pattern an append-driven fill would produce but
  in far larger transactions. Physical page layout may differ from a
  long-lived store's. `row:r7` is also spread uniformly through the log; a
  clustered selective tag would change how many pages each seek touches.
* `planned_statement_count` passes `Selectivity::default()`
  (`event_store.rs:288-291`), so the public entry point never reaches
  `read_for`. The 257.7 ms figure is from a line-for-line transcription of a
  crate-private function, not from the crate.
* **One row of one table is anomalous and carries no claim.** The `APPEND`
  instrument's `rejected-2tag-midlog` cell at 500,000 events reports medians five
  to ten times the `GUARD` instrument's figure for the same cell, with `us_p10` an
  order of magnitude below the median — a bimodal sample, unexplained, and not
  reproduced at either other size. It is printed in `results/guard-cost.md` §3
  rather than dropped, and nothing rests on it.

## Layout

```
src/lib.rs             what this is and what it deliberately is not
src/chain.rs           the four guard shapes; Selectivity, transcribed with its quadratic intact
src/store.rs           a conformant store whose only variable is the shape
src/seed.rs            bulk seeding to 10^6, and the check that runs before any clock
src/conditions.rs      the pragma read-back, and the refusal that can stop a run
tests/support/         ProbeFixture — in tests/, exactly where an adapter's goes
tests/arms_are_conformant.rs   356 conformance tests, 4 shapes
tests/conditions_are_enforced.rs  the durability control, forced to fire
tests/emitted_sql.rs           what the adapter emits, off a trace callback, and the
                               byte-for-byte check on this crate's transcription
tests/guard_cost.rs            4 shapes x 5 scenarios x 3 log sizes, interleaved
tests/query_plan.rs            EXPLAIN QUERY PLAN, and the paged read at 10^6 rows
tests/seed_ordering.rs         does most-selective-first buy the chain anything?
tests/selectivity_cost.rs      the two query-planning quadratics and the parameter
                               limit, no database
results/README.md              the index, and one verdict per finding
results/guard-cost.md          the four shapes, five scenarios, three log sizes
results/query-plans.md         the plans, the cold/warm gap, the paged read
results/seed-ordering.md       most-selective-first, and its replication
results/selectivity.md         the dedup quadratic and the parameter limit
results/raw/                   the rows those tables were written from, untouched
run.sh                         re-derives all of it
```

`results/*.md` are written **by hand** from `results/raw/`, which is why they can
say when a row is anomalous and a generator could not. Every figure in them is a
row of a file in `raw/`, and every row in `raw/` carries the pragma settings it
was produced under, read back off the live connection.
