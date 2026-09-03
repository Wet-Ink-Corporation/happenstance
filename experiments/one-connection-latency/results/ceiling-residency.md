# One page at the declared data ceiling — R-1

Written by hand from [`raw/ceiling-residency.txt`](raw/ceiling-residency.txt).
Conditions are in [`../README.md`](../README.md#conditions).

> **The question.** `crates/happenstance-sqlite/src/event_store.rs:245` declares
> `MAX_EVENT_DATA_LEN = 1_048_576`, and `:1293` sizes a page by row count alone —
> `let budget = self.remaining.map_or(PAGE_SIZE, |left| left.min(PAGE_SIZE))`,
> with no byte budget anywhere. How much memory is one page, when the rows are as
> large as the adapter says it will accept?

## The construction

520 events, each carrying exactly `MAX_EVENT_DATA_LEN` bytes of `data`, appended
through the real append path in batches of 64. Then **one** page is read at the
shipped `PAGE_SIZE = 512`, from a cold cursor, with the counting global allocator
opening a residency region around the guard's life.

520 is `PAGE_SIZE + 8`, so the first page is **full** and the figure is a whole
page rather than whatever happened to be there. Batches of 64 rather than the
declared `MAX_EVENTS_PER_BATCH` of 256, because at this payload a 256-event batch
is a 256 MiB transaction and the seed would be measuring the write-ahead log
rather than filling the file.

**One page, not a replay.** The claim is about a single hop's residency. A second
page would measure the allocator's reuse of the first page's freed buffers, which
is a property of the allocator rather than of the adapter.

The read is issued with `ReadOptions::new()` — `limit` is `None`, which is
exactly what `run_projection` uses for a rebuild.

## The number

```
PAGE_SIZE = 512 (shipped), MAX_EVENT_DATA_LEN = 1048576 B
seeding 520 events at the ceiling, 64 per batch
seeded 520 MiB in 5.9 s

first page: rows=512  peak=537036800 B (512.2 MiB)  hold=583.2 ms
```

| | |
| --- | --- |
| rows in the page | 512 — a full page |
| **peak live bytes** | **537,036,800 B = 512.2 MiB** |
| per row | 1,048,900 B — the payload, and 324 bytes of everything else |
| **mutex hold** | **583.2 ms** for the one page |
| seed | 520 MiB written through the real append path in 5.9 s |

**R-1's arithmetic was right to within 0.04%.** The finding predicted *"up to
~512 MiB in one `Vec` before a single row reaches the caller"* from
`512 × 1_048_576`. Measured: 512.2 MiB. Every byte of the payload is resident
simultaneously inside the `spawn_blocking` closure, in the merge buffer, before
`poll_next` yields row one.

And it costs 583.2 ms of held connection mutex to build. On a shared handle that
is 583 ms during which no `append`, no `head` and no other stream's ceiling
sample can proceed — a single page, at the adapter's own declared limits, with no
concurrency at all in the picture.

## What makes this the adapter's number and not a synthetic one

Nothing here is a hypothetical maximum. Every quantity is the shipped crate's own:

* `MAX_EVENT_DATA_LEN = 1_048_576` is read off the **real**
  `SqliteEventStore` (`event_store.rs:245`), and `check_ceilings` accepts every
  event seeded here.
* `PAGE_SIZE = 512` is asserted to still be the shipped declaration by
  `tests/the_copy_has_not_drifted.rs`.
* The seed goes through the replica's `append`, which is
  `crates/happenstance-sqlite/src/event_store.rs`'s append transcribed, and the
  page is fetched by `query_sql.rs` — **byte for byte** the shipped module — and
  decoded by `row.rs`, the shipped module under one rename.

An application that stores 1 MiB documents as event payloads is doing something
the adapter documents as supported, and this is what the first hop of a rebuild
costs it.

## And 512 MiB is a floor, not a bound

Two reasons the worst case is worse than this page.

**`metadata` is not ceilinged.** `check_ceilings`
(`crates/happenstance-sqlite/src/event_store.rs:474-496`) bounds `data` and
`tags`. It does not bound `metadata`, so one row's resident bytes are bounded by
SQLite's ~1 GB blob limit rather than by `MAX_EVENT_DATA_LEN`. This experiment
seeded no metadata at all.

**A wide query multiplies it.** [`page-lock-hold.md`](page-lock-hold.md) measures
the merge buffer at `ceil(arms / 400) × PAGE_SIZE` rows before truncation — 1,536
rows at page 512 for a 1,200-item query. The same ceiling-sized events behind a
1,200-item query would be about 1.5 GiB in one buffer, of which two thirds would
be truncated away and re-fetched on the next page. That arm was not run; it is
the two measured facts multiplied, and it is stated as such.

## Why this matters to `PAGE_SIZE`

[`page-lock-hold.md`](page-lock-hold.md) finds that raising `PAGE_SIZE` from 512
to 2,048 removes 187 s to 1,028 s of held mutex from a 10^6-event replay, because
the per-page hold does not scale with the page and the page *count* does. This
page is why that cannot simply be done: the quantity that scales linearly with
`PAGE_SIZE` is the one with no ceiling. At 2,048 rows this page would be about
2 GiB.

The measured answer to both findings together is a **byte budget beside the row
budget** — R-1's remediation. With one, `PAGE_SIZE` becomes a number that can be
raised on the strength of the lock-hold table. Without one, it is a number that
cannot safely be moved in either direction.

## What this page does not show

1. **One page, one configuration, one run.** No repetition, no variance. The
   512.2 MiB figure is one observation; what makes it credible is that it agrees
   with the arithmetic `512 × 1_048_576` to 0.04%, not that it was measured many
   times.
2. **Live bytes, not resident set size.** The counter sums `layout.size()` over
   the allocations live at the worst instant of the region. The system allocator
   rounds every request up to a size class and keeps freed pages mapped, so
   actual process RSS is **higher**. SQLite's own page cache is `malloc`'d
   through the same allocator and *is* included. This is a lower bound on the
   memory the page costs the process, and it is quoted as one.
3. **No metadata, one tag, one query item.** The most favourable shape at this
   payload size. See above for the two directions in which it gets worse.
4. **The hold of 583.2 ms is uncontended.** Nothing else was running on the
   handle. What a second caller would have waited was not measured on this arm.
5. **Nothing here says what a real deployment stores.** 1 MiB payloads are the
   adapter's declared ceiling, not a typical event. The finding is about what the
   adapter *permits* with no byte budget, and that is what this measures.
6. **This is an experiment, never a gate step** (CF-34). The test's only
   assertions are that the first page filled and that at least one page was
   recorded — instrument checks, not thresholds.
