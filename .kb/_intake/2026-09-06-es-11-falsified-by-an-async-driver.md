# Intake — ES-11's falsifier arrived, from the driver axis

Staged for `/redkiln:kb-ingest`. This is an **open question / finding**, not a
decision: it records what was measured and what it costs, and leaves the clause
question to the story that owns it. Do not hand-author an atom from this file.

## What was found

`happenstance-postgres` passes **93 of 95** conformance rules against a live
PostgreSQL 17.10. The two that fail, on every run, are
`read_result_is_stable_under_concurrent_append` and
`query_items_share_one_snapshot`. They fail for one reason and it is not a defect
in the adapter.

**ES-11 requires a read's state to be fixed no later than the first poll.** An
asynchronous driver cannot do that. The first poll can only *start* the round
trip that takes the snapshot; the snapshot lands when that round trip completes,
which is necessarily after the poll returned `Pending`. Both rules exploit the
gap deliberately — poll once, append, then drain — so the appended event is
inside the snapshot and the read returns four events where three were seeded.

`happenstance-sqlite` passes because `rusqlite` is **synchronous**: it samples its
position ceiling on the polling thread, inside `poll_next`, before handing
anything to a worker. That is a property of the driver, not of the design.

**What was tried.** Handing the cursor's opening to the runtime at the first poll
(`Handle::spawn` rather than an inline future) narrows the window and does not
close it — five failures in five runs either way. The change was kept anyway,
because starting the work at the first poll is closer to what the clause asks for
than starting it whenever the caller next polls.

**What was rejected.** Opening the transaction inside `read` would fix the
snapshot early enough and would break ADR-0011's read laziness and story AC-012's
requirement that an unpolled stream take no pool checkout — trading a
`[PROVISIONAL]` clause for an accepted decision record, which is the worse trade.
Blocking inside `poll_next` on async I/O is not available at all.

## Why this matters more than the clause it names

ES-11 is `[PROVISIONAL]` and names its own falsifier as an adapter on a different
axis. The axis it anticipated was **transport** — one-shot HTTP, no cursor, which
is `happenstance-neon`. The axis that arrived first is the driver being
**asynchronous at all**, which is every adapter in the portfolio except SQLite.
That is a much larger set than the clause's falsifier contemplated, and it is
worth saying plainly: on this reading SQLite is the exception, not the rule.

## The second, weaker exposure — measured, and separate

Arm C admits only rows beneath `pg_snapshot_xmin(pg_current_snapshot())`, and
that frontier is held back by the oldest transaction open **anywhere on the
server**. Two consequences were measured rather than reasoned about:

1. **A parallel suite is its own contention.** 79 of 95 rules pass with the
   default test threading; 93 pass with `--test-threads=1`. Every one of the 14
   differences is a read-after-write rule. The live CI job therefore runs
   serially, and that is a finding about the mechanism rather than a convenience.
2. **An aborted transaction holds the frontier back exactly as a long-running one
   does.** One serial run in five came in at 90/95, and the three extra failures
   were all read-after-write rules that executed *after*
   `append_is_atomic_under_a_mid_batch_fault` and
   `arming_a_mid_batch_fault_makes_the_append_fail` — the two rules that abort a
   transaction on purpose. Runs: 93, 93, 90, 93, 93.

The phase-2 experiment measured staleness as a **latency** (0.688 ms unloaded,
4010.719 ms behind an unrelated five-second write). What this adds is that the
same property shows up as **non-determinism in correctness rules**, which is a
different kind of cost and a harder one to price.

## What the CI gate does with it, and why

The two ES-11 rules are not skipped, not `#[cfg]`-ed out and not filtered from
the run — a rule absent from a run is indistinguishable from a rule that passed
(CF-18). They execute, they fail, and the job asserts:

- both recorded ES-11 rules **must** fail — one of them passing means the clause
  or the adapter moved and the recorded exposure is stale;
- any additional failure must come from the recorded frontier-sensitive set, and
  is printed as a warning naming the rules;
- anything else fails the job.

## What this does not decide

- **Which way ES-11 moves.** Amending a clause is an ADR. This records the
  evidence; the amendment belongs to whoever owns the clause.
- **Whether arm C is still the right mechanism.** ADR-0024 owns that, and this is
  input to it: the mechanism's structural bill is larger than the phase-2
  throughput number suggested.
- **ES-12**, which fails for the same reason and is listed separately only
  because it is a separate clause.

## Links the atoms should carry

- `[[0011-read-laziness-and-isolation]]` — the laziness this refuses to trade
- `[[0013-position-assignment-and-visibility]]` — the mechanism's prior authority
- `[[postgres-arm-c-structural-cost]]` — the open question this feeds
- `spec/SPECIFICATION.md` ES-11, ES-12, ES-30
- `crates/happenstance-postgres/src/read_stream.rs` — the argument at the code
