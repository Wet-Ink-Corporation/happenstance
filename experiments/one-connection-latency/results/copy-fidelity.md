# The copy has not drifted — the control under every other page

Written by hand from [`raw/drift.txt`](raw/drift.txt) and
[`raw/conformance.txt`](raw/conformance.txt). Conditions are in
[`../README.md`](../README.md#conditions).

> **A copy that has drifted from the original measures nothing.** Three of this
> crate's modules are copies of shipped ones. Every figure in `results/` is a
> figure about `happenstance-sqlite` only for as long as those copies still are
> `happenstance-sqlite`. This page is what says they are.

Both checks run **before any clock starts** — steps 2 and 3 of `run.sh`, ahead of
all three timed targets — so a drifted copy fails the run rather than producing
figures nobody can trace.

## Why there are copies at all

The question instrument (b) asks — *how long does a read page hold the connection
mutex* — cannot be asked of the shipped adapter from outside it:

| what has to be observed | why it cannot be | at |
| --- | --- | --- |
| `PAGE_SIZE` | private `const` | `crates/happenstance-sqlite/src/event_store.rs:141` |
| `fetch_page` | private method on a private type | `event_store.rs:1276` |
| the connection mutex | private field | `event_store.rs` |

Nothing public observes any of the three, and this experiment is not permitted to
edit anything under `crates/`. So the page fetch is reproduced in
`src/replica.rs`, and two of its dependencies are copied verbatim rather than
re-implemented — because a hand-written translation of the same SQL is a
different experiment wearing the same name.

## Check 1 — the two verbatim copies, by re-derivation

`tests/the_copy_has_not_drifted.rs` reads the originals from
`crates/happenstance-sqlite/src/` **at the live tree**, applies the documented
substitution, and compares. It does not compare against a stored hash; it
compares against the file that ships.

| test | what it re-derives | the substitution |
| --- | --- | --- |
| `query_sql_is_byte_for_byte_the_shipped_module` | `src/query_sql.rs` from `crates/happenstance-sqlite/src/query_sql.rs` | none — byte for byte |
| `row_is_the_shipped_module_under_one_rename` | `src/row.rs` from `crates/happenstance-sqlite/src/row.rs` | `use crate::event_store::SqliteEventStoreError;` → `use crate::replica::ReplicaError;`, then `SqliteEventStoreError` → `ReplicaError` throughout |

`query_sql.rs` can be copied with no substitution at all because nothing in it
names the event-store error type or any other crate-local item. That is what
makes a verbatim copy possible, and it is why *this file* — rather than a
transcription — is what the measured page fetch calls. The tag encoding, the
`COLUMNS` list and `to_event`'s three decisions in `row.rs` are the measurement's
subject, not its scaffolding, so nothing beyond the one rename is allowed to
differ.

CRLF/LF is normalised before comparison. Git may check the originals out with
CRLF on Windows while this crate's copies were written with LF, or the reverse;
line endings are not drift, and normalising them is what keeps the test about the
code.

The failure message names the consequence rather than the diff:

> `src/query_sql.rs` has drifted from … Re-copy it and re-run `./run.sh`; every
> figure in `results/` was produced by the version that matched.

## Check 2 — the one private constant the copy must restate

`PAGE_SIZE` is private, so no compiler check can tie the copy to the original.
`the_shipped_page_size_is_still_the_one_this_crate_calls_shipped` substitutes for
one: it asserts that `event_store.rs`'s source text still contains the literal
declaration

```rust
const PAGE_SIZE: usize = 512;
```

because every table in `results/` labels a column **"shipped"** on the strength
of it. If someone tunes the shipped value — which is precisely what J-5 asks for
— this test fails, and the "shipped" column in every table is known stale rather
than quietly wrong.

## Check 3 — the constants that are borrowed, not copied

`the_public_constants_are_the_shipped_ones` asserts four values read off the
**real** `SqliteEventStore`:

| constant | value |
| --- | ---: |
| `MAX_QUERY_ARMS_PER_STATEMENT` | 400 |
| `MAX_EVENTS_PER_BATCH` | 256 |
| `MAX_TAGS_PER_EVENT` | 128 |
| `MAX_EVENT_DATA_LEN` | 1,048,576 |

The type system already defends these — `src/replica.rs` names them, so a change
to any of them is a compile error here. This test does not add safety; it
**records** that they are borrowed, so that a reader of `results/` can tell a
number this crate transcribed from a number it took off the real type. The 400
that produces `ceil(arms/400) = 3` in [`page-lock-hold.md`](page-lock-hold.md)
and the 1,048,576 that produces 512.2 MiB in
[`ceiling-residency.md`](ceiling-residency.md) are both in the second category.

## The result

```
running 4 tests
test the_public_constants_are_the_shipped_ones ... ok
test row_is_the_shipped_module_under_one_rename ... ok
test query_sql_is_byte_for_byte_the_shipped_module ... ok
test the_shipped_page_size_is_still_the_one_this_crate_calls_shipped ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

`test` profile, not `--release` — it compiles and compares text, and there is
nothing here to time.

## What no textual check can cover, and what stands in its place

`src/replica.rs` was **deliberately modified**, so comparing it to
`event_store.rs` would fail by design. Four differences, each of them the
measurement:

1. `const PAGE_SIZE: usize = 512` becomes the const generic parameter `PAGE`.
   `Replica<512>` is the shipped value.
2. `fetch_page` times the `connection.lock()` call and the life of the guard, and
   reports both.
3. `fetch_page` optionally opens a residency region around the guard's life and
   reports the merge buffer's length **before** `truncate(budget)` — which is
   where the `ceil(arms / 400) × PAGE_SIZE` multiplier is either visible or
   invented.
4. The error enum drops variants no path here can reach.

Everything else — migration 1, the ceilings check, `BEGIN IMMEDIATE`, the
`SELECT max(position)` guard, the chunked plan, the ceiling sample on the polling
thread, the deferred `spawn_blocking`, the merge, the dedup, the `exhausted`
computation before truncation — is the original's, transcribed.

**CONTROL 2 is what holds it honest**, and it is a stronger check than a diff
would have been. `tests/replica_is_conformant.rs` points
`happenstance_testkit::event_store_conformance!` at `Replica<PAGE>` for every
page size that is later timed:

| page size | rules | result |
| ---: | ---: | --- |
| 64 | 89 | pass |
| 128 | 89 | pass |
| **512** (shipped) | 89 | pass |
| 2048 | 89 | pass |
| | **356** | **`356 passed; 0 failed`** |

This is not ceremony. **A wrong page size is always the fastest.** `PAGE_SIZE`
sizes the `LIMIT` on every per-chunk statement, seeds `budget`, and is the number
`exhausted = merged.len() < budget` is computed against — so a wrong value does
not produce a slow read, it produces a read that stops early or repeats a row at
a page boundary. The shipped adapter's own `ReadCursor::resume_from` doc
(`event_store.rs:1196-1207`) records exactly that bug having happened once
already, in the shipped adapter, at the page boundary.

The suite's multi-page criteria seed past `2 × PAGE_SIZE` rows, so at `PAGE = 64`
these runs cross a page boundary many times over and at `PAGE = 2048` most do not
cross one at all. That asymmetry is the point: between them the four columns
cover both regimes.

### A fourth control, unplanned

[`reactor-stall.md`](reactor-stall.md) arms 5 and 7 drive a `read()` to its first
row under identical construction — arm 5 through the **real**
`SqliteEventStore`, arm 7 through `Replica<512>`. They measure **639.290 ms** and
**635.056 ms**, 0.7% apart. That is a behavioural agreement between the copy and
the original that no textual check could produce, and it was a by-product of
measuring the residual rather than something the run set out to obtain.

## What this page does not show

1. **Text equality is not behavioural equality for `replica.rs`.** 89 rules × 4
   page sizes is a strong bar and it is not a proof. A difference the conformance
   suite cannot see — a difference in *timing* rather than in results — would
   pass every check on this page. The 0.7% agreement above is one spot check
   against that, at one call, on one arm.
2. **The drift tests compare against the working tree, not against a commit.**
   They check that the copy matches whatever `crates/happenstance-sqlite/src/`
   contains *now*. Re-running `./run.sh` on a tree with uncommitted adapter
   changes checks those changes, which is the intended behaviour and is worth
   knowing.
3. **`src/seam.rs` is checked by nothing here**, because it is not a copy of
   anything that ships. It is the *proposed* remediation, transcribed from
   `projection_store.rs:342-361` onto the event store's bodies. Its fidelity
   claim is "this is what the fix would look like", and the only thing that
   settles it is reading it.
4. **This is an experiment, never a gate step** (CF-34). These four tests and the
   356 conformance tests run from `run.sh` alone; nothing in `cargo xtask ci`
   reaches them.
