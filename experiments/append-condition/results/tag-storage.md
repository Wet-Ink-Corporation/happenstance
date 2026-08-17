# The tag storage, measured

Every figure here is a row in [`raw/tag-storage.txt`](raw/tag-storage.txt),
produced by one `./run.sh` under `journal_mode=wal`, `synchronous=normal`,
`busy_timeout_ms=5000`, SQLite 3.53.2, `--release`, on the machine
[`../README.md`](../README.md) names.

All three arms passed `event_store_conformance!` first, at the recommended
strategy — 89 rules each. The blob arm is measured rather than dismissed on
taste: `Tags` is canonically sorted **precisely so** that a single-column
encoding stays viable, and discarding it without a number would contradict the
reason that sorting exists.

Strategy is held at `BEGIN IMMEDIATE` + `EXISTS` probe throughout, so the only
thing varying is the tag storage.

## The run

50,000 seeded events, 25 rounds, **round-robin inside one process** — one
operation against each arm per round, so a host that gets busy partway through
affects all of them equally. Three sequential tests were written first and
thrown away: two runs an hour apart disagreed about the *ordering* of the
filtered read, because the machine slowed down and the arm that happened to run
late wore it.

Every store is checkpointed (`PRAGMA wal_checkpoint(TRUNCATE)`) and `ANALYZE`d
after seeding and before anything is timed, so a read figure is not a function of
how long a write-ahead log happens to be.

Two tags per event: `shard:hot|cold`, one in three hot, and `row:rN` for N in
0..97. Three reads are timed:

* **selective** — `Seeded` tagged `row:r7`, **516 of 50,050**. The shape a
  consistency boundary actually has: a handful of events out of a log, where the
  cost is dominated by *finding* them.
* **broad** — `Seeded` tagged `shard:hot`, **16,684 of 50,050**, where
  materialising `SequencedEvent`s dominates and the tag storage barely shows.
* **unfiltered** — `Query::all`, 50,050 events, touching no tag storage at all.
  This is the **noise floor**: whatever it varies by between arms is what the
  instrument cannot resolve.

## Results

Medians, microseconds.

| arm | conditional append | unconditional append | probe | **selective read** | broad read | unfiltered read |
| --- | --- | --- | --- | --- | --- | --- |
| **join table** (single-tag fast path) | 1,418 | 862 | 556 | **10,744** | 75,599 | 131,611 |
| join table, no fast path *(control)* | 1,773 | 680 | 1,093 | 10,334 | 77,128 | 126,147 |
| canonical blob | 1,037 | 414 | 623 | 33,992 | 71,351 | 116,036 |
| JSON1 | 1,024 | 590 | 434 | 49,766 | 99,329 | 126,410 |

**The noise floor.** The unfiltered read spans 116,036 to 131,611 across four
arms executing identical SQL over identical rows — about **±7%**. Nothing
narrower than that is a result.

## What the table says

**The join table wins the read path, and it is not close.** On the selective
read it is **3.16x** the canonical blob and **4.63x** JSON1, against a ±7% floor.
The reason is structural rather than incidental: `event_tag` is keyed
`(tag, position)`, so "the positions carrying this tag" is a contiguous range
scan already sorted by position, while both other arms have no index over tags at
all and must scan every candidate row applying `instr` or `json_each`.

**On the broad read the advantage disappears, and that is not a contradiction.**
At 16,684 matching events out of 50,050, materialising a third of the log
dominates and the tag lookup is a rounding error: 75,599 against 71,351 and
99,329, which is the floor plus a little. It is recorded because it names the
regime the join table's advantage does *not* apply to, and because a reader who
measured only broad reads would reach the opposite conclusion.

**It pays for the win on the write path.** An unconditional single-event append
costs 862 µs against the blob's 414 µs and JSON1's 590 µs — about **1.5x to
2.1x** — for two extra `event_tag` rows and two `tag_cardinality` upserts per
event. That is the real cost of the recommendation and the record states it: the
join table trades roughly a factor of two on writes for a factor of three to five
on the reads a DCB command loop performs **before every one of those writes**.

**`event_type` earns its place as a covering column.** All four arms carry it, so
this table does not measure it; what it means is that a query item constraining
both type and tags never leaves the `event_tag` index — the join back to `event`
that the published sketch at
`crates/happenstance-sqlite/src/event_store.rs:36-54` forces, and forces *under
the write lock*.

## The single-tag fast path, and its negative control

The `join table, no fast path` row is **not a fourth candidate**. It is the same
arm with one change: it always emits the general superset test,
`GROUP BY position HAVING COUNT(DISTINCT tag) = n`, where the shipped arm skips
the aggregate when the item carries exactly one tag.

**The aggregate is an optimisation barrier.** SQLite cannot push the enclosing
`position > ?` predicate — the boundary an append condition's guard always
carries — through a `GROUP BY`, so the probe materialises *every* matching
position in the log and then discards the ones at or below the boundary. With one
tag the aggregate asserts nothing: "carries at least this one tag" is membership,
and the `(tag, position)` key answers it with a seek.

Measured side by side in one interleaved run: **1,093 µs → 556 µs**, a factor of
**1.97**, with the read columns unchanged as expected — a read has no
`position > ?` to push. A single-tag query item is the overwhelmingly common
shape of a consistency boundary, so this is the common case rather than a corner.

## What the multi-tag path costs, and what follows from it

The general form is not merely slower, it is a different order of magnitude. On
the rejection-path control in [`append-condition.md`](append-condition.md), a
**two-tag** boundary over a 50,000-event log costs 42 ms to 66 ms per guard
evaluation against 0.2 ms to 0.3 ms for a single-tag one — roughly **200x** — on
every strategy.

Two things follow, and both belong in migration 1 rather than in a later
optimisation pass:

1. **`tag_cardinality` is a requirement.** Probing most-selective-tag-first is
   what keeps the union the aggregate has to materialise small, and SQLite cannot
   supply per-value cardinality on its own: `ANALYZE` stores an average, and an
   average is exactly wrong for a tag set where one value matches a third of the
   log and another matches one percent.
2. **The 200x is measured with `event_type` already covering.** It is what the
   aggregate costs *after* the schema correction, not before it — so it is a
   floor on the multi-tag path rather than an argument that the correction did
   not work.
