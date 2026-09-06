# `experiments/correlated-exists-guard/results/`

Everything here comes from one `./run.sh` on **2026-09-05**. Nothing is carried
across runs.

| Path | What it carries |
| --- | --- |
| [`guard-cost.md`](guard-cost.md) | six shapes x four scenarios x three log sizes, round-robin |
| [`query-plans.md`](query-plans.md) | `EXPLAIN QUERY PLAN` per shape — the mechanism, not the timing |
| [`seed-ordering.md`](seed-ordering.md) | the ordering policy on both shapes, alternating |
| [`unselective-pair.md`](unselective-pair.md) | **the adversarial corpus** — no tag selective |
| [`read-path.md`](read-path.md) | **the read path**, and the three candidate repairs for I-3's outer wrapper |
| [`all-query-wrapper.md`](all-query-wrapper.md) | the one of those three that shipped, and the mechanism it was wrong about |
| [`windowed-arms.md`](windowed-arms.md) | **the fourth candidate**, which wins both corpora at one arm |
| [`wide-arms.md`](wide-arms.md) | the same three shapes at VT-23's 128-item floor, where that stops holding |
| [`merge-join.md`](merge-join.md) | **the shape that wins both floors** — SQLite's own co-routine merge |
| `raw/*.txt` | command output, tee'd verbatim by `run.sh` |

The tables are written **by hand** from `raw/`, because a table nobody read is a
table nobody checked.

## The order in `raw/` is the argument

`conformance.txt` and `emitted-sql.txt` come first in `run.sh` and first here for
the same reason: **a shape that is fast and wrong wins every benchmark.**

* `conformance.txt` — 534 tests, six shapes x 89 rules, all passing. The two new
  shapes are conformant stores, not just fast SQL. A correlated `EXISTS` whose
  alias resolved to the inner table would be the tautology `position = position`,
  a two-tag guard silently behaving as a one-tag one, and the winner of every
  cell in `guard-cost.md`.
* `emitted-sql.txt` — this crate's transcription of the shipped chain, checked
  against what `happenstance-sqlite` actually emits off a `sqlite3_trace_v2`
  callback. Every ratio in these tables is taken against that baseline, so if the
  transcription drifted the ratios would be against a shape nothing runs.
* `conditions.txt` — the durability control, forced to fire.

`unselective-pair.md` was written to be able to overturn the others. It did not,
but it produced the hypothesis `read-path.md` then confirmed: most of the guard's
margin comes from `max()` stopping early. **Read those two before acting on the
recommendation** — between them they say how much of the headline transfers to
the path an application spends most of its time on, and what else is in the way.

## Conditions

[`../README.md#conditions`](../README.md#conditions) — machine, OS, filesystem,
toolchain, and the SQLite pragmas **read back off the live connection** rather
than trusted from the `PRAGMA` that issued them. Every printed row in `raw/`
carries the settings string beside it, so a figure and its conditions cannot be
separated by copying one of them.

## What none of it shows

[`../README.md#what-this-does-not-show`](../README.md#what-this-does-not-show),
in full. The one that bites hardest: **no repair of I-3's outer
`position IN (<matched>)` wrapper is built for a tagged query.** `read-path.md` measures three
candidates and shows the choice between them is cardinality-conditional — 1,089x
one way, 2.3x the other — with two selectivity points and nothing between them.
