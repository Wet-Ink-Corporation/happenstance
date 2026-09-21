# The cap sweep, on the adapter that ships

What red rate `happenstance-sqlite` actually has at four candidate values of
`BUSY_TIMEOUT_MS`, and what raising it costs when nothing is contended.

**The short answer.** At the shipped 5,000 ms the worst configuration this host
can produce goes red **7 launches in 8**. At 15,000 ms it goes red **0 in 16**.
Healthy-path duration is unchanged to within noise. 30,000 ms is also clean and
buys nothing measurable over 15,000 here.

## This is not `run.sh`, and the difference is the point

Every other page in this directory is written from [`raw/`](raw/), produced by
one [`../run.sh`](../run.sh) against `experiments/append-condition`'s
measurement *candidate*. This page is not. It was produced by a shell loop
against the **real adapter's own conformance target**, and it exists to close
the first of the four caveats
[`../README.md`](../README.md#what-none-of-this-shows) raises about everything
else here:

> 1. **It is not `happenstance-sqlite`.** The store is
>    `experiments/append-condition`'s measurement candidate — same schema,
>    pragmas, `BEGIN IMMEDIATE` and guard SQL, one differing function. The
>    adapter holds a `Mutex` across its transaction and does more inside it (tag
>    inserts, cardinality upserts); neither is reproduced.

It is. Both of those are in the loop below, because the loop runs
`crates/happenstance-sqlite`'s own tests. What this page gives up in exchange is
the instrumentation: there is no counting busy handler here, no per-contender
wait vector, no percentiles. **It reports one bit per launch — red or not
red** — which is the bit an adapter author actually experiences, and the bit
the other pages can only infer.

Read the two together. `busy-timeout-margin.md` says *how close to the cap the
waits run*; this says *how often the suite therefore goes red*.

## Method

`BUSY_TIMEOUT_MS` is edited in place, the target is rebuilt, and the whole
concurrency target is launched *n* times; a launch counts as red if libtest
prints `test result: FAILED`. Nothing else is changed, and the constant is
restored afterwards.

```sh
measure() {                     # $1 = value, $2 = launches
  sed -i "s/pub const BUSY_TIMEOUT_MS: u64 = [0-9_]*;/pub const BUSY_TIMEOUT_MS: u64 = ${1};/" \
    crates/happenstance-sqlite/src/connection.rs
  fail=0
  for i in $(seq 1 "$2"); do
    out=$(cargo test --locked -p happenstance-sqlite --test concurrency -- --test-threads=1 2>&1)
    echo "$out" | grep -q "^test result: FAILED" && fail=$((fail+1))
  done
  echo "${1}: ${fail}/${2} red"
}
```

**`--test-threads=1` is the worst case, not the gate's case, and that is
deliberate.** It is counter-intuitive enough to state plainly: serialising the
*rules* gives each 64-contender race the whole machine, so all 64 writers run
genuinely simultaneously and contention is maximal. Letting libtest overlap the
three rules — what `cargo xtask ci` actually does — spreads them and makes the
suite *less* likely to go red, not more. Measured here, at 5,000 ms: 8 of 8 red
serialised against 1 of 8 red at the gate's own parallelism.

That is the same direction as [the core sweep](busy-timeout-margin.md), where
cutting the process to one core reduced the worst wait about 450x. A sensitive
detector is what is wanted for choosing a cap, so the sweep below uses it.

## Conditions

Same host as everything else here — see
[`../README.md`](../README.md#conditions) — on **2026-09-21**, with
`happenstance-sqlite` at `0.3.2` and the tree otherwise clean.

| | |
| --- | --- |
| Subject | `crates/happenstance-sqlite`, `tests/concurrency.rs`, unmodified |
| Build | **debug**, as `cargo xtask ci` runs it |
| `CONTENDERS` | 64, the shipped value |
| Parallelism | `--test-threads=1` for the sweep; libtest default for the duration rows |
| Host load | **shared**, as before. Absolute durations are inflated; only within-run comparisons are used. |

## The sweep

| `BUSY_TIMEOUT_MS` | launches | red | rate |
| ---: | ---: | ---: | ---: |
| **5,000** *(shipped before this)* | 8 | **7** | **88%** |
| **15,000** | 16 | **0** | **0%** |
| **30,000** | 8 | **0** | **0%** |

The failing rules are the two that require *every* contender to commit —
`positions_are_unique_under_concurrent_appends` on the count, and
`append_returns_the_callers_own_last_position` per contender. `committed` is
correct in every row throughout: **nothing here is a semantic failure.** What
fails is liveness, which is the distinction `busy-timeout-margin.md` draws and
this page inherits.

## What the raise costs when nothing is contended

Nothing measurable, and this is the row that decided 15,000 over leaving the
cap alone. The busy handler returns the instant the lock is acquired, so the cap
bounds only the tail — it is not a delay every append pays.

Six launches at the gate's own parallelism, durations of the passing runs:

| `BUSY_TIMEOUT_MS` | target duration, per launch |
| ---: | --- |
| 5,000 | 5.57, 5.47, 4.91, 5.48, 5.65, 4.51 s |
| 15,000 | 5.40, 5.26, 5.03, 4.99, 5.27, 5.23 s |

Indistinguishable on a shared host. What the raise *is* paid for by is a
genuinely stuck writer, which now takes 15 s rather than 5 s to report — a path
that is rare, and that ends in a red rule rather than a hang, because the cap
stays finite. CF-33 forbids the suite a watchdog, so this constant remains the
only liveness bound in the system and an unbounded handler remains rejected.

## Why not 30,000

It is equally clean here and buys nothing measurable over 15,000 on this host,
while tripling the pathological wait. The argument that 15,000 is defensible
*off* this host is the core sweep rather than this page: fewer cores measured
**better**, by about 450x from twenty cores to one, so a smaller CI runner sits
in the safer regime. `../README.md`'s fourth caveat still stands — this says
nothing quantitative about a 2-vCPU container — but the *direction* is
established, and it points away from needing the extra margin.

## What this does not show

1. **One host.** Everything [`../README.md`](../README.md#what-none-of-this-shows)
   says about that applies unchanged.
2. **It is one bit per launch.** A launch that cleared 15,000 ms by 1 ms and one
   that cleared it by 14 s are the same row here. The margin question is
   [`busy-timeout-margin.md`](busy-timeout-margin.md)'s and is not re-measured at
   the new cap.
3. **It does not show the conflation is fixed**, because it is not. A contended
   store and a broken store are still the same `Attempt`; this only lowers how
   often the suite meets one. That question is the testkit's — see
   `.kb/open-questions/no-fixture-tolerance-for-transient-contention.md`, whose
   instrument now exists as
   `happenstance_testkit::FaultyStore::contend_next`.
