# `experiments/append-condition`

Three candidate append-condition strategies and three tag storages for a SQLite
event store, measured **before the adapter exists**, so that ADR-0022 can quote
a figure instead of a preference.

This is not a crate anybody depends on. It is **not a workspace member** (its
`Cargo.toml` carries an empty `[workspace]` table, the same trick
`experiments/wire-format/` uses), it appears in no `verify:` command and no
`cargo xtask ci` step, and it adds no dependency to any workspace manifest —
`Cargo.lock` at the repository root is untouched. The precedent for the whole
shape is `experiments/position-visibility/`, which measured the Postgres
position-visibility question one phase before any Postgres adapter existed.

## Why it exists

`SqliteEventStore::append` is `todo!()` and stays that way until three stories
after ADR-0022 lands: AC-013 puts the *record* before the implementation. So
there is nothing in the workspace to measure, and the number has to come from
somewhere that is not the adapter.

## Conditions

Every figure below was produced under these, and every figure is printed beside
them by the code that produced it — `Durability::conditions()` reads them back
off the live connection rather than trusting the `PRAGMA` that was issued,
because SQLite silently ignores a `journal_mode` it cannot honour.

| | |
| --- | --- |
| Machine | 13th Gen Intel Core i9-13905H, 20 logical cores, 32 GB RAM |
| OS | Windows 11 (10.0.26200) |
| Filesystem | NTFS, local NVMe |
| Toolchain | `rustc 1.97.1 (8bab26f4f 2026-07-14)`, `x86_64-pc-windows-msvc` |
| Build | `--release` for every timed run |
| SQLite | 3.53.2, `rusqlite` 0.40 `bundled` |
| `journal_mode` | `wal` |
| `synchronous` | `normal` |
| `busy_timeout` | 5,000 ms — **finite and generous** |
| Command | `./run.sh` |
| Wall clock | about five minutes: 1.7 s conformance, 13 s harness, 48 s tag storage, 220 s contention |

**`synchronous` is this experiment's `fsync`.** `PRAGMA synchronous = OFF` is
named **by name** in `spec/SPECIFICATION.md:7481-7484` as a wrong implementation
that CF-14's reopen rule exists to reject, so a figure produced under it is not
merely dishonest — it is a figure for a store that fails conformance. The runner
reads the setting back and **aborts rather than emit a number** under it;
`tests/durability_settings_are_enforced.rs` forces the refusal, because a control
that cannot fire is decorative.

**The busy timeout is finite on purpose.** With sixty-four connections on one
file, `BEGIN IMMEDIATE` on a busy database returns `SQLITE_BUSY` immediately
unless a handler is configured — and an *unbounded* handler converts a livelock
into a hung run naming no rule, because there is no watchdog anywhere in the
conformance suite (CF-33). Five seconds is the value; at 64 contenders it was
never reached (`busy=0` in every row below).

## What is varied, and what is held fixed

One schema, one read path, one identity story, in `src/candidate.rs`. Two axes move:

* **Strategy** (`src/strategy.rs`) — `BeginImmediateProbe`, `ConditionalInsert`,
  `MonotonicGuard`. Exactly the three
  `crates/happenstance-sqlite/src/lib.rs:56-62` names. No fourth was invented.
* **Tag storage** (`src/tags.rs`) — `JoinTable`, `CanonicalBlob`, `Json1`.
  Exactly the three at `lib.rs:63-65`. `Tags` is canonically sorted *precisely
  so* the blob arm stays open, so it was measured rather than dismissed.

A fourth tag arm, `JoinTableGrouped`, is present and is **not a candidate**: it
is the negative control for the single-tag fast path (see *Findings*).

**Which crosses were run.** Nine combinations exist. Five were run for
conformance — all three strategies at the join table, and all three tag storages
at the recommended strategy — so that every arm on both axes is covered by at
least one conformant run. The four remaining crosses vary two things at once,
and a run that varies two things says less than either of the runs that vary
one. The two caller-side controls hold the *other* axis fixed for the same
reason.

## Conformance first, measurement second

**A wrong arm is always the fastest.** `tests/candidates_are_conformant.rs`
points `happenstance_testkit::event_store_conformance!` at each arm — 445 tests,
89 rules × 5 arms — and they all pass. An arm that did not would have its figure
discarded and the failure recorded as what that arm costs.

## The three harness scenarios, verbatim

`tests/measure.rs` drives `happenstance_testkit::event_store_benchmarks!` at
`n = 512`, `k = 64`, `N = 5000`, so that a later
`event_store_benchmarks!(SqliteFixture::new())` re-derives the same workloads
against the real adapter. The emitter — the clock, the warm-up and the median —
is this crate's, because CF-33 forbids the testkit from reading a clock and
CF-23 makes the wrapper a parameter.

**Two honest limitations of those figures, stated because they change how they
read.**

1. **The timed region includes fixture construction.** Each scenario builds its
   own fixture, so creating the file, running migration 1 and opening every
   handle are inside the timer — at `k = 64` that is sixty-five `connect()`
   calls. The cost is common to all arms, so they stay comparable; what the
   figure is not is a throughput number for the adapter.
2. **One test per arm per scenario means one time slot per arm.** On a shared
   developer host, two runs an hour apart disagreed by up to 45%, and one arm's
   *unconditional* append — a path the strategy plays no part in — varied 4x
   between slots. That is a property of measuring on this host, not of the
   harness, and the remedy is the emitter's: interleave the arms.

Both caller-side controls below do interleave, and they are what the decision
rests on. They are labelled as controls everywhere they are quoted and they
replace no harness figure.

## Findings

Full tables in `results/`, every figure a row in `results/raw/` from one
`./run.sh`. In one paragraph each:

**Strategy: the monotonic-position guard wins, and it reverses the prediction it
was written under.** The three arms are indistinguishable on the commit path
(guard cost 67, 79 and 74 µs, moving by more than that between runs) and under
contention at 8 and 64 (ranges overlapping almost completely). They separate
reproducibly on the **rejection** path — the one a DCB loop takes every time it
loses a race — where the guard costs **23 µs against 32 and 45** at 5,000 events
and **213 µs against 311 and 306** at 50,000, and 1.56x less than either on a
two-tag boundary at both. `max(position)` was expected to lose by walking where
`EXISTS` short-circuits; with `event_tag` keyed `(tag, position)` it is a seek to
the end of a range instead, and it answers with the conflicting position rather
than with a boolean, so the rejection path needs no second query at all.

**Tag storage: the join table wins, on the read path, decisively.** A selective
read — 516 matching events out of 50,050, the shape a consistency boundary
actually has — costs **10.7 ms against the canonical blob's 34.0 ms (3.16x) and
JSON1's 49.8 ms (4.63x)**, against a noise floor of ±7% measured as the
unfiltered read that touches no tag storage at all. It pays about **1.5x to 2.1x
on the write path** (an unconditional single-event append at 862 µs against 414
and 590) for two extra index rows and two cardinality upserts per event. On a
*broad* read the advantage disappears into the floor, which is recorded because
it names the regime the win does not apply to.

**A measured schema correction the published sketch does not have.**
`GROUP BY … HAVING COUNT(DISTINCT tag)` — the general superset test — is an
optimisation barrier: SQLite cannot push the enclosing `position > ?` boundary
through an aggregate, so the probe materialises every matching position and
discards the ones below it. With exactly one tag the aggregate says nothing, and
dropping it cut the probe from **1,093 µs to 556 µs** measured against its own
negative control in one interleaved run. The general form is worse than slow: a
**two-tag** boundary over a 50,000-event log costs 42-66 ms per guard evaluation
against 0.2-0.3 ms for a single-tag one, roughly **200x**, which is what makes
`tag_cardinality` and most-selective-tag-first probing a requirement rather than
a tuning knob.

**64 contenders on one file is supportable, and it is not free.** Sixty-four
`rusqlite::Connection`s opened without hitting any platform ceiling; every race
elected exactly one winner with `busy = 0` and `failed = 0`, so the finite
5,000 ms timeout absorbed the contention entirely. The cost is wall time: a race
goes from about **130 ms at 8 contenders to 1.4-2.7 s at 64**, roughly 11 to 20
times. `concurrency-family-and-contender-count` owns whether to raise the
constant; this is the number it inherits, and this story does not touch
`crates/happenstance-testkit/**`.

## Layout

```
src/lib.rs          what this is and what it deliberately is not
src/candidate.rs    migration 1, the read path, identity — everything held fixed
src/strategy.rs     the three append-condition candidates
src/tags.rs         the three tag storages, plus the fast-path control
src/durability.rs   read the pragmas back; refuse to measure under a wrong one
tests/support/      CandidateFixture — in tests/, exactly where an adapter's goes
tests/candidates_are_conformant.rs   445 conformance tests, 5 arms
tests/durability_settings_are_enforced.rs  the control, forced to fire
tests/measure.rs                     event_store_benchmarks!, verbatim
tests/tag_storage_probe.rs           caller-side control, tag storages
tests/contention_at_64.rs            caller-side control: the strategies on the
                                     rejection path at two log sizes, and at 8
                                     and 64 connections
results/                             the tables, and the raw rows they came from
run.sh                               re-derives all of it
```
