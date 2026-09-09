# `benchmarks/`

What happenstance costs — measured under stated conditions, above a floor
nobody can dispute, and recorded so a regression is visible before a user finds
it.

## The short answer

On the machine below, writing a batch of eight events through
`happenstance-sqlite` costs **1.7×** what the same eight rows cost through
hand-written SQL on the same file under the same schema, and **4.5×** what they
cost through the single-table event log an engineer writes on day one. Reading
2,000 events back costs **7.1×** the raw floor.

A single-tag DCB guard is **flat in log length** — 9 µs against 1,000 events and
10 µs against 10,000. A **two-tag intersection** is not: 270 µs at 1,000 and
**3.2 ms at 10,000**, evaluated inside `BEGIN IMMEDIATE` with every other writer
queued behind it. That is the one shape in the library that does not scale, and
[`results/GRADES.md`](results/GRADES.md) §4 has it beside the two findings it
independently reproduces.

An `Event::clone()` at the specification's own 64-tag floor costs **66 heap
operations** — and **1** if the tags came from `Tag::from_static`. That 66×
difference is the axis every table here carries a column for, and the reason is
in [The regime trap](#the-regime-trap-and-why-every-table-has-a-regime-column).

`MemoryEventStore::read` with `limit(1)` costs **50,014 heap operations and
3.5 MB of peak live bytes** against `limit(None)`'s 50,026 — asking for one
event out of ten thousand costs 72% of what asking for all of them costs.
`head()` costs **zero allocations and 13.6 ns**. `SqliteEventStore` does honour
`limit`, at 0.16% of the unlimited read.

Every one of those is a ratio between arms of one interleaved run. None of them
should be quoted to a third significant figure. See
[What none of this shows](#what-none-of-this-shows), and
[`results/GRADES.md`](results/GRADES.md) for the whole set with its
provenance.

## This is not a gate step and must never become one

It is **not a workspace member** — `Cargo.toml` opens with a bare `[workspace]`
table, the same trick every crate under `experiments/` uses — it appears in no
`verify:` command and no `cargo xtask ci` step, and it adds no dependency to any
workspace manifest. `Cargo.lock` at the repository root is untouched.

CF-34 (`spec/SPECIFICATION.md:8747`) is why:

> Performance MUST be measured by a separate harness, and that harness MUST NOT
> be part of the conformance bar. An adapter that is slow is conformant.
>
> Rejects: a benchmark result gating a merge. A threshold nobody can justify
> becomes a threshold everybody raises, and the number stops meaning anything
> the second time it is moved. Benchmarks are published per adapter and compared
> against that adapter's own history; they decide nothing about conformance.

There is **no threshold anywhere in this crate, on any budget**. The only
assertions are in `tests/`, and every one of them fires when the *instrument*
has stopped working — never when a number is large.

Here the empty `[workspace]` table is also doing a second job that is not about
policy. `src/bin/allocations.rs` installs a counting `#[global_allocator]`,
which needs `unsafe impl GlobalAlloc`; the workspace root sets
`unsafe_code = "forbid"`, which cannot be overridden from inside a crate at all.
The instrument physically cannot live under `crates/`.

## Why `benchmarks/` and not `experiments/`

An experiment answers one question and is finished: its README is written once
and its `results/` are a closed record. This is a **standing** instrument — the
same arms are re-run over time and `results/history/` accumulates, because
CF-34's own model is that benchmarks are compared against an adapter's own
history. Both trees sit outside the workspace and neither gates anything; what
separates them is whether the numbers are expected to be taken again.

## Why it exists

`README.md` at the repository root positions this library on correctness and
makes **no performance claim at all** — a grep for
`fast|scale|throughput|latency|efficient|overhead|benchmark` across `docs/` and
`README.md` returns one unrelated hit. Three of the fourteen reviews record that
the stated goal's first two words were *"fast, efficient"* and that nothing
supported them (`references/evaluation/ARCHITECTURAL-EVALUATION.md:830`,
`review-docs-adr.md:369`, `review-packaging-semver.md:342`).

That gap costs decisions, not just positioning:

* **Four `[PROVISIONAL]` clauses name the absent measurement as their blocker** —
  ES-17 (`spec/SPECIFICATION.md:3345`, the borrowed batch), ES-32 (`:4087`, the
  polling seam), PS-30 (`:5464`), and the append-condition guard collapse
  (`:1821`).
* **The repository publishes figures it cannot reproduce.**
  `ARCHITECTURAL-EVALUATION.md:827-828` carries a 970× penalty and a 650× swing,
  stamped `[PLAUSIBLE — figures unverified in-repo]`, and they are load-bearing
  for the shape of the tag index.
* **Two doc comments a consumer reads are measured false.**
  `crates/happenstance-core/src/memory.rs:30-31` prices a snapshot as *"bumps
  refcounts rather than copying data"*;
  `crates/happenstance-sqlite/src/event_store.rs:91-96` requires
  most-selective-tag-first probing, which finding CN-1 measured as inverted by
  38–44× on the shipped shape.

`references/seeds/measured-not-claimed.md` states the whole problem and is where
most of this crate's design came from.

## Conditions

Every figure in `results/` was produced under these. Fill the machine rows in
from your own host if you re-run it; the toolchain and SQLite rows are printed
at the top of `results/raw/conditions.txt` and read back off the live process by
`src/report.rs`, so they are checkable rather than transcribed.

**There are two hosts now, and both tables stay.** Host A produced every figure
currently under `results/`. Host B is the dedicated measurement machine
(`ops/host/`), provisioned because the 40% between-run movement in
[What none of this shows](#what-none-of-this-shows) is a property of a busy
laptop rather than of this library. Deleting Host A's table while Host A's
numbers are still in `results/` would leave a conditions table that does not
describe its own results, which is worse than carrying two.

### Host A — Windows 11 laptop (every figure in `results/` through 2026-09-08)

| | |
| --- | --- |
| Machine | 13th Gen Intel Core i9-13905H, 14 cores / 20 logical, 31.7 GiB RAM |
| OS | Windows 11 Home, 10.0.26200 |
| Filesystem | NTFS. **Every database file lives under `%TEMP%` on `C:`** (`std::env::temp_dir()`), which is the internal NVMe — not the drive the repository is checked out on |
| Toolchain | `rustc 1.97.1`, `x86_64-pc-windows-msvc` (`results/raw/conditions.txt`) |
| Build | `--release` and `--bench` for every timed run. Both profiles are **stated** in `Cargo.toml` rather than defaulted, because `[profile.bench]` does not inherit `[profile.release]`: `opt-level = 3`, `debug = false`, `lto = false`, `codegen-units = 16` |
| Warnings | `-D warnings`, ambient from the repository root's `.cargo/config.toml` — cargo config discovery walks up from here and nothing overrides it |
| SQLite | 3.53.2, `rusqlite` 0.40 `bundled` — the same driver at the same version the workspace names |
| `journal_mode` / `synchronous` / `busy_timeout` | `wal` / `1` (NORMAL) / 5,000 ms, taken from `happenstance-sqlite`'s own constants and **read back off the live connection**, because SQLite silently ignores a `journal_mode` it cannot honour |
| Statement cache | **None, on either side.** The workspace pins `rusqlite` with `default-features = false`, which drops its `cache` feature, so `prepare_cached` does not exist in this graph and both the adapter and the floor re-prepare |
| Timer floor | Measured per run and printed with it — around 50 ns per sample for the `Instant::now()` pair that brackets each one. Any arm within 10× of it is labelled `TIMER-DOMINATED` |
| Command | `./run.sh` (about 35–45 minutes), or `./run.sh --fast` (about four) |

### Host B — `britton-ai`, the dedicated measurement host

Provisioned and tuned by [`ops/host/`](../ops/host/README.md); every row below is
declared in `ops/host/host.env` and asserted by `ops/host/preflight.sh` before a
run starts, so these are checkable rather than transcribed — the same property
the toolchain and SQLite rows above already have.

| | |
| --- | --- |
| Machine | Lenovo Legion 5 15ACH6 — a **laptop**. AMD Ryzen 7 5800H, 8 cores / 16 logical, 15 GiB RAM. Slower and smaller than Host A, which is the right trade: every figure here is a ratio between arms of one run, and a repeatable clock beats a fast one |
| OS | Ubuntu 24.04.4 LTS, kernel 6.8.0-134 |
| Filesystem | ext4 on LVM on NVMe. Database files under `std::env::temp_dir()` = `/tmp`, asserted **not** to be a tmpfs — a tmpfs would make the SQLite arms measure RAM and report it in a table headed by a filesystem |
| Toolchain | `rustc 1.97.1`, `x86_64-unknown-linux-gnu` (`results/raw/conditions.txt`) |
| CPU regime | `performance` governor **and** `energy_performance_preference` on all 16 CPUs, applied by `happenstance-bench-tuning.service` so it survives a reboot and `systemctl is-active` is a question the preflight can ask. Frequency **uncapped** and SMT **on**, both deliberately left at their as-found settings until the flakiness is measured |
| Clocksource | **`hpet`**, and that is a finding rather than a default. The kernel marks this part's TSC unstable at boot, and `tsc_adjust` — the MSR Linux would use to correct per-CPU offsets — is absent, so no kernel parameter can fix it. `clock_gettime` therefore costs a measured **1,390 ns** against 19 ns on tsc. `ops/host/probes/tsc-migrate.c` is the instrument that settled it; [`ops/host/README.md`](../ops/host/README.md) carries the numbers and the two wrong answers that came first |
| Timer floor | ~1,390 ns, about **28× Host A's**. At `paired.rs`'s `TIMER_HEADROOM` of 10, arms below roughly 14 µs are `TIMER-DOMINATED`. Of the seven arms in `results/raw/overhead.log` exactly one is — `memory`, at a 5.8 µs median. The other six sit between 143 µs and 5.5 ms, where 1.4 µs is under 1%; the criterion targets batch iterations and amortise it; the allocation counts read no clock at all |
| Quiet | The sole purpose of the machine. `apt-daily`, `unattended-upgrades`, `man-db`, `fstrim` and `motd-news` **masked** — not merely disabled, because `apt-get install` re-enables a disabled timer — no cron, sleep and lid handling masked, and one container running, an idle tunnel |
| Command | `ops/host/bench.sh [--fast]` — preflight, then `taskset` to one thread per physical core, then this `run.sh`. **See the limitation below: `run.sh` cannot complete here.** |

#### `run.sh` does not finish on Host B, and the reason is a control doing its job

`run.sh` is `set -euo pipefail` and CONTROL 2 fails, so the run aborts before it
reaches the criterion sweeps. The failing test is
`tests/instruments_work.rs`'s `the_paired_sampler_sees_a_difference_it_was_given`:
handed two arms with a known difference between them, the paired sampler reports
both at an identical 4,679 ns median and labels both `TIMER-DOMINATED`. It cannot
see a difference it was given.

That is the `hpet` clocksource, at a measured 1,390 ns per read against 19 ns on
`tsc` — and `ops/host/README.md` records why `tsc` is not available on this part
and cannot be made available. **The abort is correct behaviour.** A harness that
completed and published paired ratios from a sampler in that state would be the
worse outcome by far.

What still works here, and what does not:

| | on Host B | why |
| --- | --- | --- |
| CONTROL 1, `conformance_first` | ✅ | no clock |
| `controls_fire` | ✅ | no clock |
| `instruments_work` | ❌ 1 of 9 | the paired sampler, above |
| `allocations` | ✅ | reads no clock at all |
| the seven criterion sweeps | ✅ | criterion batches iterations and amortises the clock |
| `overhead` — the paired ratios | ❌ | same cause as `instruments_work` |

So a Host B run is `run.sh` **minus the paired runner**, and every figure it
produces is a criterion absolute or an exact allocation count. The ratios in
`results/GRADES.md` §1 have no Host B equivalent taken by the same instrument;
deriving them from criterion arms instead is a **substitution of instrument**,
and `results/flakiness/FLAKINESS.md` is the measurement that says the
substitution is sound *on a quiet host* — sequential criterion drift, which is
the paired runner's whole justification, is absent here.

## The history detects a regression, and that was checked

`results/history/` is only worth committing if a change would show up in it. It
was verified the way anything else here is — by making the failure happen:

```console
cargo bench --bench store_query_shapes -- 'query/construction/query-all' --save-baseline pre
# then, with the arm deliberately doing five times the work:
cargo bench --bench store_query_shapes -- 'query/construction/query-all' --baseline pre
```

> ```
> query/construction/query-all
>                         time:   [4.5667 ns 4.7688 ns 4.9895 ns]
>                         change: [+5.2517% +10.153% +14.981%] (p = 0.00 < 0.05)
>                         Performance has regressed.
> ```
>
> The pessimisation was reverted immediately; the point is that a 10% change on
> a four-nanosecond arm is detectable at all.

That is criterion's own comparison, which lives under `target/` and is the right
tool within one afternoon on one machine. `src/bin/collect.rs` flattens the same
estimates into `results/history/`, which is the part that survives into the
repository and can be diffed a year later.

## Conformance first, measurement second

**A wrong arm is always the fastest.** A store that skips the append condition,
drops rows at a page boundary, or hands out a refcount clone where the adapter
opens a connection will post better figures than the real thing, and no timing
can tell the difference.

So `run.sh`'s first step is `tests/conformance_first.rs`, which runs the full
`event_store_conformance!` suite against **both** timed arms — 178 rules, 89
each. `experiments/append-condition/results/append-condition.md:9-11` states the
house rule the same way: *"An arm that had not would have had its figure
discarded."*

Two more control files run before anything is timed:

* `tests/instruments_work.rs` — the allocator counts a known allocation, the
  peak sees a buffer freed before the region closed, the processor clock tells
  working from waiting, the paired sampler sees a difference it was given **and**
  reports none where there is none, and the timer-domination guard fires on an
  arm that does nothing.
* `tests/controls_fire.rs` — both floors write rows and read them back, both are
  configured exactly as the adapter is, the corpus refuses a workload whose
  figures would be meaningless, and CF-34's own worked query selects a proper
  subset.

Three of those found real defects while being written, which is the argument for
having them: a region guard that leaked on unwind and cascaded into four
unrelated tests, a spin loop the compiler constant-folded to 2 ns, and a
histogram that silently clamped **every** sample to 2 because
`Histogram::new(3)` starts with a recordable range of 1 to 2.

## The regime trap, and why every table has a regime column

`references/evaluation/review-pre-publication-2026-09-03.md:2724` measured one
`Event::clone()` at **66 heap operations** at VT-22's 64-tag conformance floor,
against **1** when the tags came from `Tag::from_static` — and the interned arm
is *flat in tag count*, so it does not respond to the axis at all. This crate
reproduces both figures exactly.

That is not a curiosity. ES-17 keeps `append`'s batch borrowed and its
`[PROVISIONAL]` marker is falsified by *"a measurement showing the per-event
clone is a material fraction of append cost"*. The interned arm is the one a
benchmark author writes **without choosing to**, because `from_static` constants
are what a fixture naturally holds — so a suite built that way measures a regime
with a 66× cheaper clone, reports the clone immaterial, and lifts a marker on
evidence that could not have gone the other way. The review's own words: *"a
falsifier that can be satisfied by construction."*

So `corpus::Regime` is a required parameter with no default,
`Corpus::distinct` refuses the interned regime by name, and every row carries
`representative: false` when it came from the interned arm.

**Nothing here lifts ES-17's marker.**
`.kb/open-questions/es-17-two-adapter-measurement-is-unscheduled.md` records what
the falsifier actually asks for — two builds of one adapter differing only in
`append`'s ownership, on one harness — and this crate is not that. What it
supplies is the numerator, and the warning that comes with it.

## Absolutes from criterion, ratios from the paired runner

criterion runs the members of a group sequentially, each to completion, so
anything that drifts across a run lands entirely on whichever arm was running at
the time. This repository has measured that drift twice and both times it was
**larger than the effect being measured**:

* `RUNBOOK.md:1628-1634` — the position-visibility experiment's sequential
  design *failed*; the baseline moved **2.7× at one client and 3.0× at 64**. The
  discarded pass is kept as evidence rather than deleted, and the instruction
  left behind is *"any later benchmark in this repository should assume the same
  instability."*
* `references/adr/0022-append-condition-strategy.md:120-144` — two runs an hour
  apart disagreed by up to **45%**, and one arm's *unconditional* append, a path
  no strategy participates in, varied by **4×** between slots.

So: absolute figures come from criterion, every **ratio** comes from
`src/paired.rs`, which interleaves the arms round-robin in one process, and the
two are never mixed in one table. The paired runner also splits each arm's
samples in half by round and reports the second half's median over the first's,
so drift is **reported** rather than assumed away.

## Layout

```
Cargo.toml            bare [workspace] table; both profiles stated
run.sh                the whole of what produced results/raw/; --fast skips the sweeps
src/
  corpus.rs           the workloads, and the regime axis every table carries
  domain.rs           the typed layer's workload: one event, one model, two projections
  fixtures/           memory, sqlite, and the two raw-SQL floors
  runtime.rs          one executor for every arm, so it is not a confound
  paired.rs           the round-robin sampler every ratio comes from
  counting.rs         the allocator — installed by two binaries, never by the benches
  cpu.rs              processor time, and the ratio that exposes a blocking call
  report.rs           conditions, rows, and the record a run commits
  bin/overhead.rs     the headline: happenstance above raw SQL, interleaved
  bin/allocations.rs  the reproducible half, in counts rather than nanoseconds
  bin/collect.rs      criterion's estimates flattened into results/history/
benches/              seven criterion targets, harness = false
tests/                conformance first, then the instrument and workload controls
results/
  GRADES.md           the reader-facing table
  raw/                tee'd command output, committed
  history/            one JSON per run, named for its date and commit
```

## What none of this shows

1. **One machine, one run per cell.** Every claim is a **ratio between arms of
   one run**, or an order of magnitude. Nothing here should be quoted as
   "287 µs of `append`", and no figure should be read to a third significant
   figure. The wall-clock medians on this host moved by up to 40% between runs
   while the allocation counts stayed identical to the digit — which is why the
   allocation columns are the ones to quote.
2. **The floors are deliberately cheaper than a correct store.** Neither writes
   an `EventId`, a `RecordedAt`, a `store_meta` incarnation or a
   `tag_cardinality` upsert; neither evaluates an append condition; both encode
   the tags column more cheaply than the adapter does. That is the point — a
   floor is only useful if nothing could beat it, because then the ratio above
   it is an *upper* bound. It also means the ratios are **not** a claim that a
   hand-rolled store would be correct.
3. **No comparison against a peer library, and that is declined rather than
   deferred.** `references/seeds/measured-not-claimed.md:215-222`:
   *"cross-language, cross-design throughput comparisons are contested by
   default and the credibility cost outweighs the positioning. Own history,
   cross-adapter within the workspace, and overhead above the bare database are
   the comparisons worth standing behind. If that judgement is wrong it should
   be overturned deliberately, not drifted into."*
4. **Contention here is interleaved on one thread, not OS-thread contention.**
   The testkit's scenario builds *k* futures before polling any of them, which is
   what keeps it usable by the `!Send` flavour the two-trait port design pays
   for. An adapter that wants thread contention supplies it from its own
   emitter.
5. **Nothing here re-measures what an experiment already answered.** Reactor
   stall and `PAGE_SIZE` are `experiments/one-connection-latency/`; poll cost and
   fan-out amplification are `experiments/polling-cost/`; the append-condition
   SQL is `experiments/append-condition/` and
   `experiments/shipped-append-condition-sql/`; the busy-timeout margin is
   `experiments/busy-timeout-margin/`. They are cited, not repeated, and their
   log lengths go far beyond what a 40-minute run can afford.
6. **The CPU column is process-wide.** `cpu-time`'s `ProcessTime` reads the whole
   process's user and kernel time, so any other live thread is counted. Every arm
   that reports a ratio runs alone and `run.sh` passes `--test-threads=1`, but
   `SqliteEventStore::read` hops each page to `spawn_blocking`, so its counts
   include the pool thread's work.
7. **No wasm32 arm.** RS-52-1: on that target clocks, threads and
   `spawn_blocking` all type-check and then panic. What a benchmark of a `!Send`
   store on `wasm32` even means is an open question, not an omission.
8. **This crate has never gated anything and must not start.** Every timed
   target's assertions are deliberately weak — they fire only if the instrument
   stopped working. A threshold on any figure here would make this a gate step by
   the back door.
