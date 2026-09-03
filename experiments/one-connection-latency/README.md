# `experiments/one-connection-latency`

What `happenstance-sqlite`'s one-connection-behind-one-mutex shape costs: what a
contended `append` does to the reactor, what a read page costs everybody else
sharing the handle, and what `PAGE_SIZE = 512` actually controls.

This is not a crate anybody depends on. It is **not a workspace member** — its
`Cargo.toml` carries an empty `[workspace]` table, the same trick
`experiments/append-condition/`, `experiments/wire-format/` and
`experiments/shipped-append-condition-sql/` use — it appears in no `verify:`
command and no `cargo xtask ci` step, and it adds no dependency to any workspace
manifest. `Cargo.lock` at the repository root is untouched. CF-34 is why: a
benchmark that can turn a merge red teaches people to re-run until green.

Here the empty `[workspace]` table is doing a second job that is not about
policy. This crate installs a counting `#[global_allocator]`, which is a
per-binary singleton and needs `unsafe impl GlobalAlloc`; the workspace root sets
`unsafe_code = "forbid"`, which cannot be overridden from inside a crate at all.
The instrument physically cannot live under `crates/`.

## Why it exists

Five findings in the pre-publication review say this shape costs something
specific, and all five say it **without a number**.

* **J-2 / F2-1 / I-4** — `append` (`crates/happenstance-sqlite/src/event_store.rs:1018`),
  `head` (`:1069`) and `contains_event_id` (`:1090`) take the connection `Mutex`
  and run rusqlite synchronously on whatever task polled them. Only `read` hops
  to `spawn_blocking`, and even its first poll takes the same mutex on the
  polling thread (`ReadCursor::sample_ceiling`, `:1239`). The sibling module in
  the same crate routes *every* SQL-touching body through
  `SqliteProjectionStore::in_blocking_task` (`projection_store.rs:342`) and its
  module doc names the asymmetry as a defect in advance. The magnitude decides
  whether this is a release blocker or a documentation correction.
* **J-5** — `PAGE_SIZE = 512` (`event_store.rs:141`) says of itself *"The value
  is a placeholder until it is measured"*, and the finding calls it the only knob
  sizing how long `fetch_page` (`:1276`) holds the connection mutex.
* **R-1** — a page's memory is bounded by row count and not by bytes
  (`budget = min(remaining, PAGE_SIZE)`, `:1293`), while one event's `data` may
  be `MAX_EVENT_DATA_LEN = 1_048_576` bytes (`:245`). The finding's arithmetic
  says a full page of ceiling-sized rows is about 512 MiB in one `Vec`.

## Conditions

Every figure in `results/` was produced under these. The pragma rows are **read
back off the live connection** by
`happenstance_sqlite::connection::ConnectionSettings::read_back` rather than
trusted from the `PRAGMA` that issued them — SQLite silently ignores a
`journal_mode` it cannot honour — and the string is printed at the top of
`results/raw/reactor-stall.txt` beside the figures it governs.

| | |
| --- | --- |
| Machine | 13th Gen Intel Core i9-13905H, 14 cores / 20 logical, 31.7 GiB RAM |
| OS | Windows 11 Home, 10.0.26200 (build 26200) |
| Filesystem | NTFS. **Every database file lives under `%TEMP%` on `C:`** (`std::env::temp_dir()`, `src/workload.rs:65`), which is the internal SK Hynix HFS001TEJ9X115N NVMe — not the external USB SSD the repository is checked out on |
| Toolchain | `rustc 1.97.1 (8bab26f4f 2026-07-14)`, `cargo 1.97.1 (c980f4866)`, `x86_64-pc-windows-msvc`, LLVM 22.1.6 (`results/raw/conditions.txt`) |
| Build | `--release` for every timed run. `[profile.release]` is stated in `Cargo.toml` rather than defaulted: `opt-level = 3`, `debug = false`, `lto = false`, `codegen-units = 16`, `panic = "unwind"` |
| Warnings | `-D warnings`, ambient from the repository root's `.cargo/config.toml` — cargo config discovery walks up from this directory and nothing here overrides it |
| SQLite | 3.53.2, `rusqlite` 0.40.2 `bundled` (`libsqlite3-sys` 0.38.2) — the same driver at the same version the workspace names |
| `journal_mode` | `wal` |
| `synchronous` | `1` (`NORMAL`) |
| `busy_timeout` | 5,000 ms (`connection.rs:62`) |
| Chunk width | `SqliteEventStore::MAX_QUERY_ARMS_PER_STATEMENT` = 400 |
| Page size | `event_store.rs:141`'s `PAGE_SIZE` = 512 rows, plus 64, 128 and 2048 as the varied axis |
| Command | `./run.sh` |
| Run | 2026-09-03, 07:42:15–07:47:27 local — **the one run every figure comes from.** No file in `results/raw/` is carried across runs |
| Wall clock | five minutes twelve seconds. The harness-reported durations sum to 305 s: drift 0.00 s, conformance 1.17 s, reactor stall 7.29 s, ceiling residency 6.60 s, page lock-hold 289.57 s (85.6 s of it seeding a million events) |

**The Windows timer resolution is part of the conditions, not part of the
result.** The default system timer resolution is 15.6 ms unless something in the
process has called `timeBeginPeriod`, so a 1 ms `tokio::time::interval` does not
produce 1 ms gaps on an *idle* reactor either. Every reactor-stall table
therefore carries an `idle` row taken under exactly the same construction — it
measures **16.177 ms** — and no arm's number means anything except against it.
A stall figure quoted without its floor has silently attributed the operating
system's timer granularity to the adapter.

Nothing here sets `synchronous = OFF`. `spec/SPECIFICATION.md:7481-7484` names it
by name as a wrong implementation CF-14's reopen rule rejects, and the settings
are printed rather than assumed so that a reader can check.

## Real code and faithful copy

This crate is not permitted to instrument `crates/happenstance-sqlite` in place,
so it does both, and which is which is the first thing a reader is owed.

### Measured through the real crate, by path dependency

| what | where the code is |
| --- | --- |
| Instrument (a), arms 1, 4 and 5 | the **real** `SqliteEventStore` — `append` (`crates/happenstance-sqlite/src/event_store.rs:1018`), `head` (`:1069`), `read` (`:955`) |
| CONTROL 1, arm 2 | the **real** `SqliteProjectionStore::commit` (`crates/happenstance-sqlite/src/projection_store.rs:592`) |
| The second connection's `BEGIN IMMEDIATE` | the **real** `happenstance_sqlite::connection::open_configured` (`crates/happenstance-sqlite/src/connection.rs:93`), so the holder runs under the same WAL, the same `synchronous` and the same 5,000 ms busy timeout as the first |
| The four store ceilings and the 400-arm chunk width | the **real** public associated constants on `SqliteEventStore` — `MAX_QUERY_ARMS_PER_STATEMENT`, `MAX_EVENTS_PER_BATCH`, `MAX_TAGS_PER_EVENT`, `MAX_EVENT_DATA_LEN` (`event_store.rs:245` and neighbours), read off the real type rather than restated |

### Copied, and what the copy is checked against

`PAGE_SIZE` is a private `const`, `fetch_page` is a private method on a private
type, and the connection mutex is a private field. Nothing public observes any
of the three, so the page fetch is reproduced here.

| this crate | copied from | how it differs | what holds it honest |
| --- | --- | --- | --- |
| `src/query_sql.rs` | `crates/happenstance-sqlite/src/query_sql.rs` | **byte for byte**, nothing at all | `tests/the_copy_has_not_drifted.rs::query_sql_is_byte_for_byte_the_shipped_module` |
| `src/row.rs` | `crates/happenstance-sqlite/src/row.rs` | exactly one rename: `SqliteEventStoreError` → `ReplicaError`, and its import moves from `crate::event_store` to `crate::replica` | `tests/the_copy_has_not_drifted.rs::row_is_the_shipped_module_under_one_rename`, which re-derives the original *through that substitution* and compares |
| `src/replica.rs` | `crates/happenstance-sqlite/src/event_store.rs` | four deliberate differences, each of them the measurement: `const PAGE_SIZE: usize = 512` (`:141`) becomes the const generic `PAGE`; `fetch_page` times the `lock()` and the guard's life; `fetch_page` optionally opens a residency region and reports the merge buffer's length **before** `truncate(budget)`; the error enum drops unreachable variants | **CONTROL 2** — no textual check can compare a copy that was deliberately modified, so 89 conformance rules run against it at every page size that is later timed |
| `src/seam.rs` | `crates/happenstance-sqlite/src/projection_store.rs:342-361` | the `in_blocking_task` seam transcribed onto the event store's three synchronous bodies — the fix J-2, F2-1 and I-4 all propose | it is not a copy of anything shipped on the event store; it is the *proposed* code, built so the residual after it can be measured rather than argued about |

The one private constant the copy has to restate is `PAGE_SIZE`, and
`the_shipped_page_size_is_still_the_one_this_crate_calls_shipped` asserts the
original's source text still declares `const PAGE_SIZE: usize = 512;` — because
every table in `results/` labels a column "shipped" on the strength of it.

**A copy that has drifted from the original measures nothing.** That is what
`results/raw/drift.txt` is: four tests, run before any clock starts, that
re-derive the two verbatim copies from `crates/happenstance-sqlite/src/` at the
live tree and fail on any difference. If someone changes the shipped
`query_sql.rs` and these figures are re-quoted afterwards, that test is what says
so — loudly, rather than the figures quietly becoming wrong. The full account is
[`results/copy-fidelity.md`](results/copy-fidelity.md).

## Conformance first, measurement second

**A wrong page size is always the fastest.** `PAGE_SIZE` is not a tuning knob
that can only be slow: it sizes the `LIMIT` on every per-chunk statement, seeds
`budget`, and is the number `exhausted = merged.len() < budget` is computed
against. A wrong value does not produce a slow read, it produces a read that
stops early or repeats a row at a page boundary — and the shipped adapter's own
`ReadCursor::resume_from` doc (`event_store.rs:1196-1207`) records exactly that
bug having happened once already.

So `tests/replica_is_conformant.rs` points
`happenstance_testkit::event_store_conformance!` at `Replica<PAGE>` for
`PAGE ∈ {64, 128, 512, 2048}` — **356 tests, 89 rules × 4 page sizes** — and
`run.sh` runs it before any timed target. `results/raw/conformance.txt` ends
`356 passed; 0 failed`. A figure taken at a page size whose column there is red
is discarded.

Beyond that, every assertion in the timed targets is deliberately **weak**: it
fires only if the instrument stopped working, never on a threshold. The contended
append arm asserts only that it exceeded the idle floor; the ceiling arm asserts
only that the first page filled. A threshold would be this crate quietly becoming
a gate step.

## The instruments

| | what it does | raw |
| --- | --- | --- |
| **(a)** reactor stall | one `current_thread` runtime; a task ticks a 1 ms `tokio::time::interval` and records every gap while a bare OS thread holds `BEGIN IMMEDIATE` on a second connection for 750 ms. The largest gap is how long the executor was unavailable | `raw/reactor-stall.txt` |
| **(b)** page lock-hold | 10^6 events on one file, one handle, a replay driven page by page while a second task appends **through the same handle**. `PAGE_SIZE ∈ {64, 128, 512, 2048}` × query width ∈ {1, 400, 1200} items | `raw/page-lock-hold.txt` |
| **(c)** the seam | arms 3, 6 and 7 of instrument (a): the same calls through `src/seam.rs`, so the residual after the proposed fix is measured rather than argued about | `raw/reactor-stall.txt` |
| **R-1** ceiling residency | 520 events at exactly `MAX_EVENT_DATA_LEN`, and **one** page read at the shipped `PAGE_SIZE = 512`, with a counting global allocator open around the guard's life | `raw/ceiling-residency.txt` |
| **CONTROL 1** | the real `SqliteProjectionStore::commit` under the same contention — the conformant arm, and the floor the event store should reach | `raw/reactor-stall.txt` row 2 |
| **CONTROL 2** | 89 conformance rules at every timed page size | `raw/conformance.txt` |
| **drift** | the copy-fidelity control | `raw/drift.txt` |

Two contentions, not one, because WAL makes them different. A writer does not
block readers under WAL, so the file **write lock** stalls `append` (which opens
`BEGIN IMMEDIATE`) and nothing else, while the connection **mutex** stalls
`head`, `contains_event_id` and `sample_ceiling` — all of them plain `SELECT`s
that WAL would happily serve concurrently. They wait because the *adapter*
serialises them, not because SQLite does. Measuring only the first would have
produced two rows of zeroes and the wrong conclusion about `head` and about the
read's first poll.

## Findings

Full tables in [`results/`](results/README.md), and a verdict against each
reviewed finding in [`results/README.md`](results/README.md). In one paragraph
each.

**A contended `append` stalls the whole reactor for the whole of the call.** On a
`current_thread` runtime with a second connection holding `BEGIN IMMEDIATE` for
750 ms, the shipped `append` leaves a maximum tick gap of **885.761 ms** against
an idle floor of **16.177 ms** — 54.8x — and the 1 ms ticker fires **5** times
where it fired 54 with nobody contending. The same contention through the
projection store's already-conformant `commit` costs **27.525 ms**, and through
an `in_blocking_task` seam on the event store's own body, **34.812 ms**. The
call takes just as long either way; what changes is whether anything else on the
runtime can run while it does.

**`PAGE_SIZE` does not control what the finding says it controls, and does
control something else.** Over a 32x range — 64 to 2048 — the per-page lock hold
does not move: 187/142/147/203 ms at query width 1, inside its own run-to-run
spread. What moves it is query width: 147 ms → 203 ms → 684 ms at the shipped
512 for widths 1, 400 and 1,200, because `ceil(arms / 400)` statements per page
is 1, 1 and 3, and the merge buffer measurably holds `3 × PAGE` rows. But
`PAGE_SIZE` does control the *number* of holds, so over a whole 10^6-event
replay the aggregate held-mutex time scales with 1/`PAGE`: 286 s at 512 against
99 s at 2048 and 2,929 s at 64.

**The caller sharing the handle pays at the tail, not at the median.** A quiet
appender is p50 0.118 ms / p99 7.727 ms. Under a concurrent replay at the shipped
`PAGE_SIZE = 512`, its p50 is unchanged at 0.103–0.148 ms while its p99 rises to
186 ms (width 1), 259 ms (width 400) and **799 ms** (width 1,200) — 103x the
quiet p99, with a worst observed append of 945 ms. The cost is asymmetric: the
page itself waits about 1 ms.

**R-1's arithmetic is right to within 0.04%.** One page of 512 events at exactly
`MAX_EVENT_DATA_LEN` peaks at **537,036,800 live bytes — 512.2 MiB** — inside a
single `spawn_blocking` hop, before one row reaches the caller, and holds the
connection mutex for 583.2 ms while it does.

**The seam fixes three of the four, and the fourth is not a seam problem.**
Routing `append` and `head` through `in_blocking_task` takes their stalls from
885.761 ms to 34.812 ms (25.4x) and 717.545 ms to 41.573 ms (17.3x). A `read()`'s
first poll goes from 639.290 ms to 635.056 ms — **0.7%, which is nothing** —
because ES-11 requires the ceiling to be sampled no later than the first poll,
`sample_ceiling` takes the same mutex from inside `poll_next`, and no seam on the
write side can move a lock acquisition that happens on the polling thread by
requirement.

## Running it

```console
./run.sh
```

About five and a half minutes, of which instrument (b) is 290 s (85 s of it
seeding a million events). Disk: the ceiling arm writes about 1 GB into `%TEMP%`
(520 MiB of events plus its write-ahead log) and removes all three files when it
ends; instrument (b) writes about 500 MB for the same span.

`run.sh` is **not a gate step and must never become one.** Nothing in
`.redkiln/config.yaml` or `xtask/src/main.rs` invokes it, the root manifest's
`members = ["crates/*", "examples/*", "xtask"]` does not reach `experiments/`,
and the empty `[workspace]` table means cargo cannot adopt this crate from the
root manifest even by accident. NF-003 — it terminates unattended — holds by
construction: every replay is bounded by a page count *and* a wall-clock budget,
both constants in the source, and every busy timeout is the adapter's own finite
5,000 ms. There is no watchdog anywhere, and nothing here needs one.
