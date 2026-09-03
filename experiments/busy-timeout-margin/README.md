# `experiments/busy-timeout-margin`

What SQLite's busy handler actually costs a contender at `CONTENDERS = 64`, and
how much of `BUSY_TIMEOUT_MS = 5_000` is left when the gate runs it.

**The short answer, and it is not the one the question expected.** In the
configuration `cargo xtask ci` actually runs, the worst per-contender wait is
**3,628 ms of the 5,000 ms budget** measured directly, and **≈5.5 s** under the
busy handler that actually ships — and **one launch in seven turns the
concurrency suite red today, at the contender count that ships today, on the
fastest machine in the corpus.** The cores axis runs the *other way* from the
prediction: cutting the process to one core reduces the worst wait by about
450x, from 3,628 ms to 8 ms.

This is not a crate anybody depends on. It is **not a workspace member** (its
`Cargo.toml` carries an empty `[workspace]` table, the same trick
`experiments/append-condition/` and `experiments/wire-format/` use), it appears
in no `verify:` command and no `cargo xtask ci` step, and it adds no dependency
to any workspace manifest — `Cargo.lock` at the repository root is untouched.
Nothing under `crates/`, `spec/`, `.kb/` or `docs/` is modified by it.

## Why it exists

`crates/happenstance-sqlite/src/connection.rs:62` ships

```rust
pub const BUSY_TIMEOUT_MS: u64 = 5_000;
```

and it is the only liveness bound anywhere in the system. CF-33 is `[FROZEN]`
and forbids the conformance suite from carrying a watchdog
(`crates/happenstance-testkit/src/concurrency.rs:24-44`), so nothing else can
time-bound a contended run. The constant's own documentation says *"Five seconds
was measured to absorb 64-way contention with zero `SQLITE_BUSY` (ADR-0022
§11)"*.

That measurement is real and it is `experiments/append-condition`'s. Its recorded
conditions (`references/adr/0022-append-condition-strategy.md:140-144`) are a
20-logical-core i9, `--release`, and its own test target run alone. The gate's
conditions are not those:

| | ADR-0022 §11's row | `cargo xtask ci` |
| --- | --- | --- |
| build | `--release` | **debug** — `xtask/src/main.rs:179-190` passes no `--release` |
| test parallelism | one target, run alone | libtest's default; three 64-contender rules in one binary overlap |
| `CONTENDERS` | 64, in an experiment | 64, in `concurrency.rs:238`, for every adapter everywhere |

The gap between those two columns has never been measured. Worse, it could not
be: **nothing anywhere in the tree reports a nonzero busy count**, which is why
ADR-0022's own falsifier — *"Re-open the busy timeout if any run ever reports
`busy > 0`, which would mean five seconds stopped being generous"*
(`0022-append-condition-strategy.md:615-616`) — has never been able to fire.
`busy = 0` is a one-bit answer to a continuous question: it cannot tell 40 ms
from 4,999 ms.

Making that falsifier able to fire is the point of this directory. It fired.

## Conditions

Every figure below was produced under these, and every figure is printed beside
them by the code that produced it — the affinity mask is read back off the live
*process*, the pragmas off the live *connection*.

| | |
| --- | --- |
| Machine | 13th Gen Intel Core i9-13905H, 14 physical / 20 logical cores, 32 GB RAM |
| OS | Windows 11 (10.0.26200) |
| Filesystem | NTFS, local NVMe; databases under `%TEMP%` |
| Toolchain | `rustc 1.97.1`, `x86_64-pc-windows-msvc` |
| Build | **debug** for the matrix, the shipped-arm row, the headroom sweep and the rate; `--release` for the two controls and the bridge. Every row says which. |
| SQLite | 3.53.2, `rusqlite` 0.40 `bundled` |
| `journal_mode` | `wal` (read back) |
| `synchronous` | `normal` (read back) |
| busy budget | 5,000 ms, under both handlers |
| Host load | **shared.** Twenty-odd agent processes and a 52 GB `target/` were live throughout. This inflates the absolute times and is the reason the release control lands 1.33-1.36x above ADR-0022's recorded row. |
| Command | `./run.sh` |
| Wall clock | about eighteen minutes |

**`PRAGMA busy_timeout` reads back `0` on every counting-handler row, and that
is correct rather than broken.** `sqlite3_busy_handler()` sets
`db->busyTimeout = 0` as a side effect of installing a handler, so the pragma
stops being where the answer lives. Every row therefore carries
`busy_handler=counting cap_ms=5000` beside the pragma read-back rather than in
place of it. On the `sqlite-default` rows the pragma reads `5000` again, which is
how a reader tells the two apart without the label.

## The core-count method — this is the part that could silently lie

**`taskset` does not exist on this machine.** The constraint is a Windows
processor affinity mask, and three ways of applying one were considered:

* `Start-Process -PassThru` then `$p.ProcessorAffinity = …` — **rejected.** It
  leaves a window in which the child is already running unconstrained. A seeding
  phase that ran on twenty cores before the mask landed produces a "2-core" row
  that is nothing of the kind.
* `cmd /c start /affinity <hex> /b …` — sets the mask at creation and is correct,
  but `start /b` needs a console: invoked from a Git Bash pipeline it produced no
  output and no process at all, silently. **Rejected on that.**
* `SetProcessAffinityMask` through FFI — needs `unsafe`, which this crate forbids
  (`#![forbid(unsafe_code)]`, matching the workspace's `unsafe_code = "forbid"`).
  **Rejected on that.**

What is used is **inheritance**: `affinity-run.ps1` sets the mask on *itself* and
then starts the test binary, which inherits it at creation the way every Windows
child does. There is no window.

**And it is verified rather than assumed, twice, by two independent routes.**

1. **What the operating system says.** The test binary asks Windows for its
   *own* `ProcessorAffinity`, compares it against `HS_EXPECTED_AFFINITY`, and
   **panics rather than emit a figure** if they disagree
   (`src/cores.rs`, `Constraint::require_honest`). That is the same refusal shape
   `experiments/append-condition` uses for `synchronous = OFF`. Every row prints
   the mask it verified: `affinity_mask=0x55 reported_cores=4`.
2. **What the machine does.** A separate launch runs a parallel-speedup probe —
   fixed CPU work on one thread, then on 24 — and reports the implied core count.
   It is not authoritative, but it is *independent*: it would catch a mask that
   was reported and not enforced, which route 1 cannot.

`std::thread::available_parallelism` is deliberately **not** used as either
route. On Linux it consults `sched_getaffinity` and would be right; on Windows it
reports the machine's processors and ignores the affinity mask, so it answers
"20" under every mask below.

**The masks select distinct physical cores, and that is not cosmetic.** A naive
`0x1, 0x3, 0xf, 0xff` sequence would not: this host is hybrid — six P-cores with
SMT on logical 0-11 (0/1 are one core), eight E-cores without SMT on 12-19. The
claim comes from this machine's own probe, not a datasheet: mask `0x3` measured
**0.9** cores of throughput where `0x5` measured **1.7**.

| label | mask | logical processors | reported | measured |
| --- | --- | --- | --- | --- |
| 1 core | `0x1` | 0 | 1 | 1.0 |
| 2 cores | `0x5` | 0, 2 | 2 | 1.7 |
| 4 cores | `0x55` | 0, 2, 4, 6 | 4 | 3.7 |
| 8 cores | `0x3555` | 0, 2, 4, 6, 8, 10, 12, 13 | 8 | 5.2 |
| 20 cores | `0xfffff` | all | 20 | 5.8 |

The measured column tracks the reported one at the low end and **saturates above
four**, which is where this experiment's answer does not live. It is a
cross-check, not a core count, and on a host running twenty other agents it reads
low. Every run's readings are in `results/raw/cores-*.txt`.

## What is measured, and with what

`experiments/append-condition/tests/contention_at_64.rs`'s race, unchanged in
shape: *n* `rusqlite::Connection`s on one file opened before the timed region;
one head snapshot; one append condition anchored there so exactly one contender
may win; a `std::sync::Barrier` rather than a sleep; every contender's result
collapsed to committed / rejected / busy / failed. `src/candidate.rs`,
`src/strategy.rs`, `src/tags.rs` and `src/durability.rs` are copied from that
crate and **one function differs** — `configure`, which chooses the busy handler.

**Two instruments are added.**

**A counting busy handler** (`src/busy.rs`). `sqliteDefaultBusyCallback`'s delay
table and control flow, transcribed, with accounting: per contender, per lock
event, the milliseconds spent asleep. It reports both the **budget** wait (the
schedule's prefix sum, which is what SQLite's cap is applied against) and the
**measured** wait, because Windows' 15.625 ms timer granularity means a
`sleep(1)` is not one millisecond and the two diverge by up to 5x at low core
counts. `rusqlite::Connection::busy_handler` takes a bare `fn(i32) -> bool` — a
function pointer, not a closure — so the accounting is thread-local, which is
exact here rather than a workaround: one contender is one OS thread with one
connection for its whole life.

**A verified core constraint** (`src/cores.rs`), described above.

**Three racing tests, not five.** `--test-threads` is left at its default so
libtest overlaps them as it overlaps the real family's rules in one binary. The
count is **three**, and the finding this answers said five:
`event_store_concurrency_conformance!` generates five `#[test]` fns, but only
three open `CONTENDERS` handles (`concurrency.rs:394`, `:611`, `:691`).
`k_disjoint_boundaries_admit_exactly_k_commits` opens `BOUNDARIES *
PER_BOUNDARY` = 12 (`:508-509`) and `a_concurrent_reader_never_sees_a_partial_batch`
opens `WRITERS + 1` = 5 (`:775`, `:795`). Reproducing five would over-provision
the gate's load by two rules and produce a number the gate cannot reach.

## Conformance first, measurement second — twice

**A wrong arm is always the fastest**, and this crate does something the sibling
experiment does not: it *replaces the store's busy handler*, on the write path,
in the exact place contention is resolved. A handler that gave up one entry early
would turn a contended-but-fine append into an error; one that never gave up
would convert a bounded race into a hang. Both look like a fast race in a timing
table.

So `tests/arms_are_conformant.rs` points
`happenstance_testkit::event_store_conformance!` at both raced arms and `run.sh`
runs it **under both handlers** before it runs a clock: **178 tests, 89 rules ×
2 arms, all passing on each** (`results/raw/conformance-counting.txt`,
`results/raw/conformance-default.txt`). Two invocations rather than two modules,
because the handler is chosen once per process.

Two unit tests in `src/busy.rs` check the transcription itself: that `TOTALS` is
the prefix sum of `DELAYS`, and that the schedule reaches the cap in a bounded
59 entries — which is what lets `run.sh` promise to terminate unattended.

## Findings

Full tables under [`results/`](results/README.md) — the control, the core sweep,
the distribution and the contender sweep in
[`busy-timeout-margin.md`](results/busy-timeout-margin.md), the nested
`block_on` in [`lost-wakeup.md`](results/lost-wakeup.md). Every figure is a row
in `results/raw/` from one `./run.sh`. In one paragraph each:

**The margin at 64 is 1.38x measured and gone in practice.** In the gate's own
configuration the worst per-contender **budget** wait is **3,628 ms of 5,000**,
and the counting handler is demonstrably optimistic — at the identical
configuration it races 2.6x faster on the median than SQLite's own handler does.
Under the handler that actually ships, the same configuration produces races up
to **5,522,382 µs**, and in the wait-dominated regime a race's wall time and the
unluckiest contender's wait agree to within 1%.

**ADR-0022's falsifier fires at the contender count that ships.** Seven launches
of the gate's configuration at `CONTENDERS = 64`: **one produced `busy > 0`.**
That is `Attempt::Failed` at `concurrency.rs:262`, and
`exactly_one_of_n_contenders_commits` panics on it at `:405-414` with *"a store
failure under contention rather than the concurrency signal"* — a message that
tells the adapter author their store is wrong when nothing about their store is.
At 96 contenders it is 2.2% of attempts; at 128, 13.6%; at 160, 17.0%.

**Cores run the other way, and this refutes the mechanism the finding proposed.**
The worst wait is **monotone increasing** in core count and plateaus around
eight: 8 ms at one core, 628 at two, 2,628 at four, 3,828 at eight, 3,628 at
twenty. A two-core CI container is the *safest* place to run this suite, not the
most dangerous. SQLite's busy handler is a back-off **poll**, not a queue: the
pathology needs 64 contenders simultaneously runnable and retrying in lockstep,
which is what many cores buy and one core forbids.

**The build profile barely enters.** Debug at 20 cores (3,628 ms) and release at
20 cores (3,228 ms alone, 3,328 concurrent) are within this host's noise of each
other. The time is spent *asleep*, not computing, so `--release` cannot recover
it. Half of the finding's premise — "debug build overhead" — does not survive.

**The shipped strategy is not cheaper here.** `monotonic-guard`, which ADR-0022
§4 chose and `happenstance-sqlite` ships, reached 3,728 ms against
`begin-immediate-probe`'s 3,628 ms in the same configuration. The arms are
indistinguishable under contention, which is what `experiments/append-condition/results/contention-64.md`
already said; the margin is a property of the herd, not of the guard.

**M-1's lost wakeup is real and deterministic.** A `block_on` nested inside a
`block_on` on one thread hangs when the outer wake lands during the inner park.
Reproduced on every run, with a baseline and a no-collision control that both
complete. It is measured here because from outside a hang and a timeout
exhaustion are the same event, and CF-33 forbids the suite the watchdog that
could tell them apart.

## Layout

```
src/lib.rs          what this is, what it copies, and what it deliberately is not
src/busy.rs         the counting handler, and the switch to SQLite's own
src/cores.rs        the affinity constraint, verified two ways, refused if wrong
src/candidate.rs    copied; `configure` is the one function that differs
src/strategy.rs     copied verbatim
src/tags.rs         copied verbatim
src/durability.rs   copied verbatim — read the pragmas back, refuse a wrong one
tests/support/      CandidateFixture, copied; BUSY_TIMEOUT_MS re-exported not restated
tests/arms_are_conformant.rs  178 conformance tests, both arms, run under both handlers
tests/busy_margin.rs          the race, three concurrent rules, the counting handler
tests/lost_wakeup.rs          M-1's probe, with the watchdog an experiment may have
affinity-run.ps1    one launch under one affinity mask, by inheritance
run.sh              re-derives all of it; NEVER a gate step (CF-34)
results/            the tables, and the raw rows they came from
```

## What this directory does **not** show

Stated here rather than at the end of the results file, because these are the
sentences most likely to be dropped when a figure is quoted.

* **It does not measure `happenstance-sqlite`.** The store under test is
  `experiments/append-condition`'s measurement candidate, which shares ADR-0022's
  schema, pragmas, `BEGIN IMMEDIATE` and guard SQL but is not the adapter. The
  adapter holds a `Mutex` across its transaction and defers `spawn_blocking` on
  the read path; neither is reproduced here.
* **It is one machine, and a loaded one.** Twenty-odd agents shared this host
  throughout. Absolute times are inflated; the release control lands 1.33-1.36x
  above ADR-0022's recorded row for that reason. Every comparison here is *within* one
  run, which is the only kind this host supports.
* **It says nothing about a 2-vCPU CI container**, despite constraining cores.
  An affinity mask restricts *this process*; a small container also has less
  memory bandwidth, a different filesystem, and no twenty other agents. The
  direction of the core effect is established; its magnitude elsewhere is not.
* **The counting handler is optimistic and the amount is not calibrated.** It
  raced 2.6x faster than SQLite's own on the median at one configuration, on one
  run. That is enough to say every `wait_ms` figure here is a lower bound; it is
  not enough to say by how much.
* **`race_us_max` is used as a proxy for the worst wait on the `sqlite-default`
  rows, because those rows have no per-contender accounting by construction.**
  The proxy is validated on the counting rows to within 1% in the wait-dominated
  regime (≥4 cores) and **breaks at one core**, where the race is CPU-bound
  rather than wait-bound and the ratio is 30x. Do not read a `sqlite-default`
  race time as a wait at low core counts.
* **It does not propose a value.** Whether `BUSY_TIMEOUT_MS` moves, whether
  `CONTENDERS` moves, and whether the testkit grows a third `Attempt`
  classification are ADR-0022's and
  `concurrency-family-and-contender-count`'s. This directory supplies the number
  each of them would inherit.
