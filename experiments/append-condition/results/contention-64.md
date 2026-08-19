# Sixty-four connections on one file

The measurement `concurrency-family-and-contender-count` inherits, and the one
this story supplies rather than applies. `crates/happenstance-testkit/**` is not
touched by this diff and `concurrency::CONTENDERS` is still 8.

Every figure is a row in [`raw/contention.txt`](raw/contention.txt), produced by
one `./run.sh` under the conditions [`../README.md`](../README.md) states.

## Why the number was owed

`crates/happenstance-testkit/src/concurrency.rs:200-206` sets `CONTENDERS = 8`
and its own documentation says it is **not a tuning knob** — the rules assert set
properties that hold at any size above one. Both of phase 8's stated proof
artefacts nevertheless read *64 contenders*. Raising the constant is
workspace-wide: every fixture in the workspace, including `MemoryFixture` and
every registered mutant, re-runs at the new number, and so does every adapter's
CI everywhere. That is a change that should inherit a measured claim.

## What was run

Ten races per arm per contender count, round-robin across the three strategies
inside one process. Each race:

1. opens *n* `rusqlite::Connection`s onto one file — one per contender, opened
   **before** the timed region, so the figure is the contention rather than *n*
   file opens;
2. reads the head and builds one append condition anchored there, so every
   contender decides from the same snapshot and exactly one may win;
3. releases all *n* threads through a `std::sync::Barrier` — a barrier and not a
   sleep, for the reason `concurrency::race` records: it synchronises on the
   other threads rather than on a wall clock, so it is exact on a twenty-core
   host and on a loaded single-core runner alike, and spawning N threads in a
   loop does not start them together;
4. joins them and collapses each result to *committed*, *rejected*
   (`ConditionViolated`), *busy* (`SQLITE_BUSY` / `SQLITE_LOCKED`) or *failed*.

Tag storage is held at the join table. 5,000 events are already in the log, so
each contender's probe has something to walk.

### Why this is the caller's measurement and not the harness's

`event_store_benchmarks!`'s contended scenario interleaves *k* append futures on
**one thread**, because the benchmark family binds `Fixture` and imposes no
`Send` bound — the `!Send` flavour the two-trait design exists for has to be able
to run it. That produces a correct rejection mix and **no lock contention at
all**: the first future polled runs its whole synchronous body, commits and
returns before the second is entered, so `SQLITE_BUSY` can never fire.

`bench.rs`'s own module documentation delegates the rest in terms — *"an adapter
that wants thread-level contention supplies it through its own emitter"* — and
this is that emitter. Both figures are in `results/`; they answer different
questions and the record says which.

## Results

Wall time per race, microseconds, median with the observed range.

| strategy | contenders | median | range | committed | rejected | busy | failed |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `BEGIN IMMEDIATE` + probe | 8 | 135,948 | 127,973 – 153,998 | 10 / 10 | 70 | 0 | 0 |
| conditional insert | 8 | 122,702 | 70,228 – 152,042 | 10 / 10 | 70 | 0 | 0 |
| monotonic guard | 8 | 131,636 | 62,586 – 154,251 | 10 / 10 | 70 | 0 | 0 |
| `BEGIN IMMEDIATE` + probe | **64** | 2,724,759 | 1,970,474 – 3,496,577 | 10 / 10 | 630 | 0 | 0 |
| conditional insert | **64** | 1,878,294 | 1,427,292 – 2,452,566 | 10 / 10 | 630 | 0 | 0 |
| monotonic guard | **64** | 1,437,206 | 349,134 – 2,405,583 | 10 / 10 | 630 | 0 | 0 |

## The four things this says

**1. Sixty-four connections on one file work.** Every one of the 64
`rusqlite::Connection`s opened, on every one of thirty races, on Windows 11 with
NTFS and a local NVMe. EC-007's failure mode — a file-descriptor or connection
ceiling — did not occur here, which is a fact about this platform rather than a
guarantee about every one. A platform that does hit it should record the limit
rather than quietly measure at 32.

**2. Every race was correct.** Exactly one contender committed in all thirty
races at both counts; every loser learned it lost as a `ConditionViolated` rather
than as an adapter error; `committed + rejected + busy + failed` accounts for
every contender in every round, asserted in the test rather than eyeballed here.
Whatever else raising the constant costs, it does not cost correctness for any of
the three strategies.

**3. `busy = 0`, and that is the load-bearing result for the pragma.** Not one
contender exhausted the 5,000 ms busy timeout, at either count. The timeout is
doing real work — sixty-four writers on one file cannot all hold
`BEGIN IMMEDIATE` — and it never ran out, so `AppendError::Store` never appeared
where `ConditionViolated` belonged. An *unbounded* handler would have produced
the same zero and told us nothing, while converting any future livelock into a
hung run naming no rule (CF-33: there is no watchdog anywhere in the suite).
Finite and generous is what makes the zero mean something.

**4. The cost is wall time, and it is 11x to 20x.**

| strategy | 8 | 64 | factor |
| --- | --- | --- | --- |
| `BEGIN IMMEDIATE` + probe | 136 ms | 2,725 ms | 20.0x |
| conditional insert | 123 ms | 1,878 ms | 15.3x |
| monotonic guard | 132 ms | 1,437 ms | 10.9x |

A race that takes about 130 ms at 8 takes 1.4 to 2.7 seconds at 64 against a
SQLite adapter on this machine. The concurrency family carries **five** racing
rules, so the arithmetic a raise has to justify is roughly *0.7 s → 7-14 s per
adapter per CI run*, plus whatever a slower or more contended runner adds.

**A caution on reading the arms against each other here.** The 64-contender
column orders the same way the rejection-path control does — monotonic guard
fastest — but its ranges overlap almost completely, one arm's own spread is 7x,
and across earlier runs the ordering changed twice. Treat it as consistent with
the strategy verdict rather than as independent evidence for it; the verdict
rests on [`append-condition.md`](append-condition.md) §1.

## The recommendation this story is allowed to make

**Raising `CONTENDERS` to 64 is supportable**, and the two permitted outcomes
resolve as follows.

* *Correctness*: supported. Nothing about the raise threatens any of the three
  strategies, and none of them needed the busy timeout's ceiling.
* *Cost*: real and one order of magnitude, concentrated in the racing rules of
  one opt-in family. It is not free and it is not prohibitive.

`concurrency-family-and-contender-count` owns the decision and the edit, and it
should weigh a third option this measurement makes visible: the two are
separable, because the *proof artefacts* that read 64 could be satisfied by a
harness invocation at 64 without the shared constant moving for every adapter in
the world.
