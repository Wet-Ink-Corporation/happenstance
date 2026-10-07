# `experiments/es-11-fence`

Whether one ordering primitive `happenstance-neon` can supply itself closes the
ES-11 / ES-12 race on one-shot HTTP. The primitive is **hold an `append` until
every read the same transport dispatched before it has been answered**:
`SqlTransport::reads_settled`, backed by `ReadLedger`, on the spike branch
`lane/p17-es11-fence` (plan: `.temper/plans/p17-l8.md`, design F1).

The record this feeds is ADR-0087 (`proposed`, superseding ADR-0061), whichever
way the result goes.

**Status: pre-registered, amended once, no counted run yet.** The decision
rule below was first written on 2026-10-07, before any trial ran. It was
amended the same day, after a review and before the first **counted** run; the
amendment and its reason are recorded under "Amendment, before the first
counted run", and the rule below is the amended one. One attempt ran before the
amendment, under the old rule. It is a **pilot**, excluded from the tally
whatever it shows. The ordering is **self-attested**: nothing but the commit
history vouches for it, as with `wi-95d2b2`. `results/raw/` holds no counted
attempt yet.

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

Each trial gets one outcome, decided in this order:

* **`anchor`**: `before` did not hold exactly the seeded events at the positions
  their appends returned (1 event for `es11`, 2 for `es12`). This is the rule's
  own anchor assertion. Every read carries the visibility frontier, which CI has
  seen hide committed rows, so a lagged `before` can differ from the drain with
  no race in the window. Nothing the window shows is judged against it.
* **`error`**: the first poll, the append or the drain returned an error.
* **`red`**: the drain contains the late event (`reason: late_in_drained`), or
  it differs from `before`, compared by positions and events in order
  (`reason: drained_ne_before`). These are the rule's two assertions, in that
  order.
* **`pass`** otherwise.

* Shape `es11` follows `read_result_is_stable_under_concurrent_append`: one
  seeded `Seeded` and a late `Later`.
* Shape `es12` follows `query_items_share_one_snapshot`: seeded `Alpha` and
  `Omega`, a two-item query, and a late `Omega` matching the second item.

**The shapes follow the rules. They are not copies of them.** The differences:

* The ES-11 rule seeds **three** events with distinct tags on a **fresh**
  fixture and reads `Query::all()`. The `es11` shape seeds **one** event and
  reads with a query scoped by the trial's tag, on **one schema shared** by every
  trial of the attempt. So the rule's snapshot covers the whole log, and the
  sweep's covers one tag's rows in a log that grows by about 2,500 events per
  attempt.
* The ES-12 rule's two items are **type-only** (`Alpha`, `Omega`, no tags). The
  `es12` shape's items carry the trial's tag as well, for the same scoping
  reason.
* Neither rule anchors on `before` the way this sweep scores it. The rules
  **assert** the anchor and fail the test. The sweep records an `anchor`
  outcome and goes on.

So a sweep result speaks to the race on this shape, and it carries over to the
rules only by argument. The argument: the fence orders an append against a
read, and neither the query's breadth nor the schema's age enters that.

**Two arms**, same binary, same client, same endpoint, one migrated schema:

* `baseline` — `HyperTransport::shared_unfenced()`: registers nothing and its
  `reads_settled` is ready at once. The adapter as shipped before the fence.
* `fence` — `HyperTransport::shared()`: every read registered in a fresh
  `ReadLedger` before `round_trip` returns, settled when its spawned task ends.

Arms and shapes are interleaved per iteration, the order of the four cells
rotated by one each iteration, so drift in the endpoint falls on both arms alike.

**One row per trial**, a JSON object on one line prefixed `ES11-SWEEP `:
`schema` (2 since the amendment), `run, attempt, iter, arm, shape, outcome,
reason, error, seeded_n, before_complete, before_n, drained_n,
late_in_drained`, and client-side times in microseconds from the trial's start:
`t_read_dispatch_us, t_read_send_us, t_read_answer_us, t_append_call_us,
t_append_dispatch_us, t_append_send_us, t_append_answer_us, fence_wait_us`.
*Dispatch* is `round_trip` being called, *send* the spawned task about to hand
the request to `hyper`, and *answer* the task ending. *Send* is taken **before**
`hyper` and its HTTP/2 connection, so the order in which two requests' frames
reached the wire is not recorded. Any reordering inside `hyper` or `h2` shows up
as C2 below, not C1. Every round trip is bounded at 30 s by the test transport
(`ROUND_TRIP_TIMEOUT`). One that hits the bound is an `error`, and its read is
settled, so a hung read cannot hold the next append back indefinitely.

One `ES11-SWEEP-META` row per attempt carries the run, the attempt and the
commit SHA. No host and no credential is in any row.

## Decision rule

Written before the first run, and amended before the first counted run (see
"Amendment, before the first counted run"). Self-attested. Applied mechanically
by `run.sh`.

**Size.** 250 trials per arm per shape per job attempt, so 1,000 trials per
attempt. Only rows of `schema` 2 are judged. An attempt whose rows lack it is
the pilot's and is excluded. So is the pilot attempt by name (run
`37591126575`, attempt 1).

**Void attempts.** An attempt is **void**, excluded from the pool and reported
as void, when any of these holds:

* its extracted row count is not exactly 1,000 (a truncated log or lost rows);
* it has more than 25 `error` rows;
* it has more than **50 `anchor` rows** (5% of the attempt);
* its log does not carry exactly **one** result line (`... ok` or
  `... FAILED`) for each of the two racing rules (V1 below).

**Pooling.** Valid attempts are pooled in attempt order (run ID, then attempt
number, both numeric), and only the **first 3** are pooled. A valid attempt
after the third is reported and not pooled, whatever it shows.

**What counts.** A trial counts as `red` only when its `before` was complete.
`anchor` and `error` rows are reported, per arm and shape, and are neither
`pass` nor `red`. They are left out of every N below. A judged trial is a
`pass` or a `red`.

**The instrument is live** if the pooled `baseline` arm shows **at least 3**
`red` trials (both shapes together). If it shows fewer after 3 pooled
attempts, about 1,500 judged baseline trials, the verdict is **inconclusive:
the race was not reproduced at this N**. It is reported to the owner and
nothing is claimed.

**The fence works** if all three hold:

* the instrument is live;
* the pooled `fence` arm has **0 `red`** trials, of any class;
* V1: the conformance rules `read_result_is_stable_under_concurrent_append` and
  `query_items_share_one_snapshot` are green in every pooled attempt. Each
  pooled attempt carries one result line per rule, so V1 cannot pass on zero
  lines.

The record then quotes the counts per arm and shape, the rule-of-three 95% upper
bound for the fence arm (3 / N, with N the judged fence trials, per shape and
pooled: about 0.4% per shape at N = 750 and 0.2% pooled at N = 1,500 when no
attempt is void and nothing is an anchor), and the baseline count with its
Clopper–Pearson 95% interval.

**The fence fails** if any `fence` row is `red`. Each red is classed by
client-side order:

* **C3, causal violation**: `t_append_dispatch_us >= t_read_answer_us`. The
  endpoint served a snapshot that postdates a commit sent no earlier than the
  read was answered. It is tested first. The comparison is `>=` on the
  microsecond times: a tie at that resolution is classed C3. A true C3 is never
  filed as a spike defect. At worst a tie that was really C2 is filed as C3,
  and that error stops the work and asks the owner.
* **C1, client reorder**: `t_append_send_us < t_read_send_us`. The append's
  task reached `hyper` first.
* **C2, reorder after send**: the read reached `hyper` first, and the append was
  dispatched before the read was answered. This takes in reordering anywhere
  after *send*: inside `hyper` and `h2` on the client, at the proxy, or at the
  backend. The sweep cannot tell these apart.

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

## Amendment, before the first counted run

**2026-10-07.** A review of the spike at `81eab3b` found that the rule as first
written could score a frontier lag as a falsifier (finding W1, with W2–W4 beside
it, and W5 on this file). The rule was amended before any counted run, and the pilot is set
aside.

**Who saw the pilot, and when.** The amendment's content (W1–W5, the anchor outcome,
the pooling and V1 rules) was fixed by the review and handed to the agent that wrote
it before the pilot's log was fetched, and that agent never read the pilot's rows.
The session that ran the spike did read them afterwards: baseline 27 red of 250 per
shape, fence 0 of 250 per shape, under the old schema. They informed no part of this
rule, and they are not evidence for the verdict: the pilot is excluded by name.

**W1, the anchor.** `before` was never checked against the seeds. Every read
carries the xmin frontier, which CI has seen hide committed rows. A lagged
`before` then gave `drained != before`, which was scored `red`. Under the fence
its timings class it C3, so it read as **a false falsifier**. Under the baseline
it inflated the reds that make the instrument live. Now:

* a trial whose `before` does not hold exactly the seeded events is `anchor`,
  neither `red` nor `error`;
* each `red` records its `reason`, `late_in_drained` or `drained_ne_before`;
* only trials with a complete `before` count as red;
* anchors are reported separately, and an attempt with more than 50 (5%) is
  void.

**W2, pooling.** The first version pooled every valid attempt. Now only the
first 3 valid attempts in attempt order are pooled, so a fourth attempt cannot
be run to change the verdict.

**W3, V1.** "V1 green" held with zero result lines. Now every pooled attempt must
carry exactly one result line per racing rule, or it is void.

**W4, ties.** C3 was `>` on microsecond times, which can file a true C3 as C2.
Now it is `>=`, as stated under the classes.

**W5, the claim.** The trial description overstated how closely it follows the
rules. The differences are now listed under "One trial", and C2 now says that it
takes in client-side reordering inside `hyper` and `h2`.

**The pilot.** The first CI attempt ran on `81eab3b` (CI run `37591126575`,
attempt 1, started 2026-10-07 at about 08:02 UTC; `live-neon` job
`112692596645`). It ran under the old rule. It is a **pilot** and is excluded
from the tally **whatever it shows**: by name in `run.sh`, and by its rows, which
lack `schema` 2. The amendment rests on the review's reading of the code and the
rule, not on anything the pilot printed.

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
* **The anchor read's frontier lag, as a rate.** `before` is an anchor read.
  When the frontier hides a seed, the trial is an `anchor`, counted and
  reported. It is not a measurement of the lag: `READ_YOUR_OWN_WRITES` is
  declined, and the anchor threshold only bounds how much of an attempt it may
  consume.
* **Client-side reordering inside `hyper`.** See C2 above.

## Results

None yet.

## Result

Added after the counted runs; the text above, "None yet" included, is as it
stood before them. Counted attempts: CI run `37594816236`, attempts 1, 2 and 3,
`live-neon` job on spike PR #50 at `1177cfc` (merge ref `19b1271`). Pilot,
excluded: run `37591126575`, attempt 1. The counts, the classes and the verdict
are in [`results/tally.md`](results/tally.md), written by `./run.sh tally` from
`results/raw/`: **the fence works** under the rule above. The record is
ADR-0087 (`proposed`).
