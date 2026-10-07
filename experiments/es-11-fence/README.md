# `experiments/es-11-fence`

Whether one ordering primitive `happenstance-neon` can supply itself closes the
ES-11 / ES-12 race on one-shot HTTP. The primitive is **hold an `append` until
every read the same transport dispatched before it has been answered**:
`SqlTransport::reads_settled`, backed by `ReadLedger`, on the spike branch
`lane/p17-es11-fence` (plan: `.temper/plans/p17-l8.md`, design F1).

The record this feeds is ADR-0087 (`proposed`, superseding ADR-0061), whichever
way the result goes.

**Status: pre-registered, no run yet.** Everything under "Decision rule" below
was written on 2026-10-07, before the first trial ran, and has not been edited
since a row existed. That ordering is **self-attested**: nothing but the commit
history vouches for it, as with `wi-95d2b2`. `results/raw/` is empty until the
first attempt.

## The instrument

`es11_fence_sweep`, an `#[ignore]`d test in `crates/happenstance-neon`'s
`neon_conformance` target (body in `tests/support/sweep.rs`). CI's `live-neon`
job already runs that target with `-- --ignored --show-output --test-threads=1`,
so the sweep runs there with no workflow change, and its rows reach the job log
through `--show-output`. It never fails on a measured outcome (CF-34: a
measurement does not gate). It fails only when the harness cannot run: a seed
append or the `before` read failing.

**One trial** reproduces the two racing rules' shape on one handle:

1. seed, scoped by a tag unique to the trial;
2. `before` = a full read of that scope;
3. build a stream over the same scope and poll it **once** on the test task;
4. append a late event the scope matches;
5. drain the stream.

A trial is **`red`** when the drained set differs from `before` (positions and
events, in order) or contains the late event; **`pass`** otherwise; **`error`**
when the first poll, the append or the drain returned an error.

* Shape `es11` — `read_result_is_stable_under_concurrent_append`: one seeded
  `Seeded`, late `Later`.
* Shape `es12` — `query_items_share_one_snapshot`: seeded `Alpha` and `Omega`, a
  two-item query, late `Omega` matching the second item.

**Two arms**, same binary, same client, same endpoint, one migrated schema:

* `baseline` — `HyperTransport::shared_unfenced()`: registers nothing and its
  `reads_settled` is ready at once. The adapter as shipped before the fence.
* `fence` — `HyperTransport::shared()`: every read registered in a fresh
  `ReadLedger` before `round_trip` returns, settled when its spawned task ends.

Arms and shapes are interleaved per iteration, the order of the four cells
rotated by one each iteration, so drift in the endpoint falls on both arms alike.

**One row per trial**, a JSON object on one line prefixed `ES11-SWEEP `:
`run, attempt, iter, arm, shape, outcome, error, before_n, drained_n,
late_in_drained`, and client-side times in microseconds from the trial's start:
`t_read_dispatch_us, t_read_send_us, t_read_answer_us, t_append_call_us,
t_append_dispatch_us, t_append_send_us, t_append_answer_us, fence_wait_us`.
*Dispatch* is `round_trip` being called, *send* the spawned task handing the
request to the client, *answer* the task ending. One `ES11-SWEEP-META` row per
attempt carries the run, the attempt and the commit SHA. No host and no
credential is in any row.

## Decision rule

Written before the first run. Self-attested. Applied mechanically by `run.sh`.

**Size.** 250 trials per arm per shape per job attempt — 1,000 trials per
attempt. At most **3 attempts**, pooled. An attempt whose extracted row count is
not exactly 1,000 is **void** (truncated log or lost rows) and is excluded from
the pool, and the void is reported. `error` rows are reported and are neither
`pass` nor `red`; an attempt with more than 25 `error` rows is void.

**The instrument is live** if the pooled `baseline` arm shows **at least 3**
`red` trials (both shapes together). If it shows fewer after 3 non-void
attempts — 1,500 baseline trials — the verdict is **inconclusive: the race was
not reproduced at this N**. It is reported to the owner and nothing is claimed.

**The fence works** if all three hold:

* the instrument is live;
* the pooled `fence` arm has **0 `red`** trials, of any class;
* V1: the conformance rules `read_result_is_stable_under_concurrent_append` and
  `query_items_share_one_snapshot` are green in every attempt in which they ran.

The record then quotes the counts per arm and shape, the rule-of-three 95% upper
bound for the fence arm (3 / N per shape and pooled: 0.6% per shape at N = 500,
0.3% pooled at N = 1,000), and the baseline count with its Clopper–Pearson 95%
interval.

**The fence fails** if any `fence` row is `red`. Each red is classed by
client-side order:

* **C1, client reorder** — `t_append_send < t_read_send`: the append's task
  reached the client first.
* **C2, proxy or backend reorder** — the read was sent first, and the append was
  dispatched before the read was answered.
* **C3, causal violation** — `t_append_dispatch > t_read_answer`: the endpoint
  served a snapshot that postdates a commit sent after the read was answered.

A C1 or C2 red **under the fence** means the fence was not engaged: a spike
defect, not a finding. Check the trace, fix, re-run, and record the defect here.
A C3 red means the store does not honour response-before-dispatch; ADR-0087's
falsifier 1 fires, and the instruction is to **stop and ask the owner**.

`baseline` reds are reported by class whatever the verdict. A non-zero C1 count
partly answers ADR-0061's falsifier 3 (client-side reordering exists).

Green conformance runs are not evidence on their own (ADR-0061's long form,
`:241-242`). The claim rests on the mechanism and on the offline tests in
`crates/happenstance-neon/tests/es11_fence.rs`. This sweep checks that the
instrument sees the race and bounds the residual rate.

## How to run it

1. Open the spike branch as a **draft PR** from this repository; its `live-neon`
   job has the secret and runs the sweep.
2. Each further attempt: `gh run rerun <run-id> --job <live-neon-job-id>`. Do
   not use `gh workflow run CI --ref …`, which also runs the deployed `workerd`
   leg and deploys to Cloudflare.
3. `./run.sh fetch <run-id> <attempt>` for each attempt, or
   `./run.sh extract <saved-job-log>…` for logs saved by hand.
4. `./run.sh tally` writes `results/tally.md` from `results/raw/` and prints the
   verdict the rule above gives.

`run.sh` needs `bash`, `jq` and `awk`; `fetch` also needs an authenticated `gh`.

## Conditions

Filled in from the rows, not from memory: the commit SHA and the run and attempt
IDs are in each attempt's `ES11-SWEEP-META` row and in each raw file's name. The
endpoint is the `live-neon` job's Neon branch, PostgreSQL behind the pooler, over
HTTP/2 with a one-connection pool (`tests/support/transport.rs`), from a GitHub
`ubuntu-latest` runner.

## What this does not measure

* **Cross-handle order.** The ordering domain is one transport value. Two handles
  over independent transports are not ordered and no rule can observe that
  through the port; the sweep, like the rules, uses one handle per arm.
* **A `wasm32` transport.** No `fetch` transport exists in this tree. That the
  fence compiles for `wasm32-unknown-unknown` without tokio is checked by
  `cargo xtask wasm`. That a `fetch` transport can settle reads from a promise
  callback is reasoning, recorded in the plan, and not measured here.
* **Throughput cost.** `fence_wait_us` is recorded per trial, but the sweep is
  not a benchmark and no performance claim rests on it.
* **The anchor read's frontier lag.** `READ_YOUR_OWN_WRITES` is declined; the
  sweep has no anchor read.

## Results

None yet.
