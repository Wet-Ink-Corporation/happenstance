# Is a `happenstance-sqlite` read page budgeted in rows, in bytes, or by the caller?

Record: **R-1+J-5-read-page-budget**. Source:
`references/evaluation/review-pre-publication-2026-09-03.md:1116-1160` (the merged finding)
and `:1249-1262` (the measurement it routed for, which refuted half of `J-5`). Repository
read in the `lane/sqlite-ceilings` worktree at `bd11598`.

**This brief did not get the author → two-critic → revision pass the original thirteen
had.** It was written by the lane implementing R-1+J-5, in the same session as the change
it describes. Read it with that discount applied. Like the query-ceilings lane's brief, it
records a question the lane had to *partly* settle rather than defer, and says which part.

---

## Why this is owed

The audit routed this explicitly, and the measurement it asked for came back and said the
question could not be split:

> J-5 and R-1 pull in opposite directions on one knob. The lock-hold table says raise
> `PAGE_SIZE`; the residency arm says raising it is unsafe, because the quantity that
> scales linearly with it is the one with no ceiling — at 2,048 rows the same page would
> be about 2 GiB. There is one knob, in the wrong unit, and its two consequences are not
> co-optimisable by any value of it. That is a decision — *whether a read page is budgeted
> in rows, in bytes, or by the caller* — and it belongs where the ceilings section routed
> it: phase 8's owner, with a row in `RUNBOOK.md:262`'s ADR queue, because it adds public
> surface to a crate about to be published.

## What the lane settled, and why it could not defer it

Two of the three parts had no decision in them and were landed.

1. **The connection is taken per statement rather than per page.** Internal, no public
   surface, and the audit itself called it *"free at any time"*. It makes `PAGE_SIZE`'s own
   doc comment true — it said *"one `spawn_blocking` hop"* while the code held the mutex
   across the selectivity lookup, every statement of the plan, and the merge. Sound
   because of ADR-0011's ceiling: every statement carries `position <= H`, so what commits
   between two of them is invisible to all of them.
2. **A byte budget exists at all.** Without one, *nothing* bounds a page's residency — the
   row budget bounds cardinality and SQLite's near-gigabyte blob limit bounds the rest.
   Leaving that in place was not an option the fix could carry, for the reason CLAUDE.md
   states about ceilings: a limit nothing measures is a number, not a bound.

The third part is the decision, and it is **not** settled:

3. **What the budget should be, and whether it is the adapter's to state.**
   `MAX_PAGE_BYTES_PER_STATEMENT` is `8 * MAX_EVENT_DATA_LEN` = 8 MiB, stated in the same
   register `MAX_EVENT_DATA_LEN` is — *a fact about this adapter, not a trade*. Nothing
   measured that it is the right number, and this brief does not claim it is.

## What is true today

- `PAGE_SIZE` is **unchanged at 512**, and it is private. Its doc no longer says *"a
  placeholder until it is measured"*; it names the three measured consequences and says
  the measurement declined to settle it.
- `SqliteEventStore::MAX_PAGE_BYTES_PER_STATEMENT` is **public**, `8 * MAX_EVENT_DATA_LEN`.
  It is public for the reason `MAX_QUERY_ARMS_PER_STATEMENT` is: a test that has to guess
  a boundary is a test that stops crossing it.
- The stated ceiling on one page's payload residency is
  `MAX_PAGE_BYTES_PER_STATEMENT × planned_statement_count(query)`, both of which a caller
  can compute. It was previously unstated.
- Neither budget is configurable. There is no `ReadOptions`-adjacent knob and no builder.

The measured facts, from `experiments/one-connection-latency`, none of them re-derived
here:

| | at `PAGE_SIZE` 64 | 512 (shipped) | 2,048 |
|---|---|---|---|
| per-page lock hold, width 1 | 187.5 ms | 146.6 ms | 202.6 ms |
| aggregate held mutex over 10⁶ events, width 1 | 2,929.2 s | 286.4 s | 99.1 s |
| one page at the data ceiling | ~64 MiB | **537,036,800 B (512.2 MiB)** | ~2 GiB |

And the caller sharing the handle pays at the tail only: quiet p50 0.118 ms / p99 7.727
ms, against p50 0.148 ms and p99 **799.041 ms** under a concurrent replay at width 1,200.

## Options

### Option A — two constants stated by the adapter (what landed)

**Cost to a caller.** Nothing to change; two numbers to read if they care. A page of
maximal events is eight rows rather than 512, which is a throughput reduction for exactly
the workload that was allocating half a gigabyte per page.

**Cost to an adapter author.** None. Both are this adapter's own.

**Semver.** `MAX_PAGE_BYTES_PER_STATEMENT` is additive and public, therefore permanent
after `0.2.0`. Changing its *value* later is not a semver break but is a behaviour change
a test could be pinned to.

### Option B — a caller-stated budget, on or beside `ReadOptions`

The caller knows its container's memory limit and this crate does not. A
`ReadOptions`-adjacent knob puts the number where the knowledge is.

**Cost.** `ReadOptions` is `happenstance-core`'s, so an adapter-specific budget cannot
live on it without becoming part of the *contract* — which would oblige every adapter to
answer a question only a buffering one has. The alternative, a builder on
`SqliteEventStore`, is adapter-local and makes the store's construction fallible or
stateful in a way it currently is not. Either is real public surface, and the audit's own
routing puts that with phase 8's owner and an ADR.

### Option C — derive the row budget from the byte budget and delete `PAGE_SIZE`

One knob, in the right unit: take rows until the bytes are spent, with no row count at
all.

**Cost.** The row count is not decorative — it bounds `Vec::with_capacity`, it is what
`LIMIT ?` binds, and a page of a million tiny events would otherwise be one statement
returning a million rows to fill 8 MiB. A cardinality bound and a size bound are both
needed; that is the finding's own conclusion, arrived at from the other side.

### Option D — leave `PAGE_SIZE` alone and add nothing

Rejected, and recorded so the option is visible: it is the state the audit found, in which
a page's residency is bounded by SQLite's blob limit and stated nowhere.

## Recommendation

**Keep Option A for `0.2.0`; open Option B as the question for whoever owns phase 8.**

A is the shape that is bounded rather than unbounded, and it costs a caller nothing to
adopt. What it does not do is put the number where the knowledge is, and that is B's
argument, which is good — but B is a public-surface decision with an ADR's worth of
consequences (does the budget belong to the read, the store, or the contract?), and
spending it on the strength of one machine's run would be the premise-audit failure this
directory exists to avoid.

**The strongest argument against, stated in its own words.** *Option A hard-codes 8 MiB on
the evidence of nothing. The measurement that justified having a byte budget at all
measured `PAGE_SIZE`, not this constant; nobody has run a replay at 4 MiB or 32 MiB and
compared. A number chosen for how it reads — "eight maximal payloads" — is exactly the
placeholder the old `PAGE_SIZE` doc apologised for, and the lane has replaced one
unmeasured constant with two.* That is fair and it is why this brief exists. What answers
it partly: the two constants are not the same kind of claim. `PAGE_SIZE`'s value changed
the outcome — 64 against 2,048 moves the aggregate lock hold 30x — while
`MAX_PAGE_BYTES_PER_STATEMENT`'s value moves nothing for any workload below it, and
**every workload in this repository's tests and examples is below it**. It binds only on
payloads near the data ceiling, where the previous behaviour was to allocate without
limit. A wrong bound is strictly better than no bound, and it is the value rather than the
mechanism that stays open.

## Cost of delay

**Dated, and the date is `0.2.0`.** `MAX_PAGE_BYTES_PER_STATEMENT` is public from the
moment the crate is published, and B would either duplicate it or deprecate it. If B is
the answer, it is cheaper now than after.

## What this does not settle

- Whether `PAGE_SIZE` should become public. It is private and unchanged; a caller cannot
  compute the page's row bound, only its byte bound, which is an asymmetry nobody has
  argued for.
- Whether the peak — `MAX_PAGE_BYTES_PER_STATEMENT × planned_statement_count` — is the
  right shape, or whether a wide query should share one budget across its statements. The
  shared form was rejected here on a mechanism (the first statement would spend it and
  every later one would be cut to a single row), not on a measurement.
- **AE-4's hole, which is upstream of all of this.** `check_ceilings` does not bound
  `metadata` at all, so the quantity that can make one row exceed the whole page budget is
  the one this adapter never refuses at `append`. The byte budget makes the *read* survive
  it; nothing makes the *write* refuse it. That is `event-metadata-floor.md`'s question,
  and the two should be read together.
- Whether `RUNBOOK.md`'s ADR queue gains a row for this. It is not this lane's file.
