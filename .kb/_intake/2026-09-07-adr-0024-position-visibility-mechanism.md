# Intake — ADR-0024, the position-visibility mechanism

Staged for `/redkiln:kb-ingest`. **Do not hand-author the atom** — that is what
commit `0269720` reverted. The long form is
`references/adr/0024-position-visibility-mechanism.md`, is retained rather than
folded in, and should be cited by line.

## The shape the atom should carry

- **kind:** decision · **authority_tier:** decision · **status:** accepted
- **adr_id:** ADR-0024 · **phase:** 10 · **reversibility:** medium
- **settles:** RUNBOOK ADR queue row 0024, and
  `kb-open-question-postgres-arm-c-cost-001`
- **rests on:** `kb-decision-0013`, which lifted ES-10 and explicitly did **not**
  choose the adapter's mechanism
- **amends nothing.** `spec/SPECIFICATION.md` is unedited; ES-10 is `[FROZEN]` and
  untouched.
- Needs a row in `.kb/maps/decision-map.md`.

## The decision

`xid8` + `pg_snapshot_xmin`. Each row carries `pg_current_xact_id()`; every read
and `head()` admit only rows beneath `pg_snapshot_xmin(pg_current_snapshot())`.
`position` keeps no column default — the sequence is read explicitly at the one
`INSERT` that allocates, so "not `bigserial`" stays checkable in
`information_schema`.

## The two numbers, and the honest weight of each

Measured against the **built** adapter — pool, transaction tied to `append`'s
async boundary, cursor, error mapping — with the naive arm (same store, predicate
removed) as the paired baseline, inside the container over the unix socket, under
`fsync=on`. Raw and method:
`experiments/position-visibility/results/adapter-remeasurement.{txt,md}`.

**Steady state — no measurable cost, and the precision is part of the finding.**
Medians 0.985 / 1.131 / 1.026 at 1 / 8 / 32 clients, spread about ±20%, against an
effect phase 2 measured at 0.99–1.03. The measurement *cannot resolve* the cost
and the record says so instead of quoting a point estimate. Baseline drift within
a triple ran 1.01–1.88 on Docker Desktop's virtual disk. It establishes only that
no cost large enough to matter hides inside ±20% — which is enough, because the
rivals cost 16× and 30×.

**Staleness — the real bill, and this one is tight.** Nine samples per cell:

| arm | control median | behind a 5 s hold |
|---|---|---|
| baseline (unguarded) | 0.521 ms | 0.595 ms |
| arm C | 0.593 ms | **4799.3 ms** |

The unguarded row is the control **phase 2 did not have**: it compared arm C
against itself, which shows the frontier moves but not that the frontier is what
moved. Under an identical hold the unguarded arm is unaffected. Same server, same
hold, same load; only the predicate differs.

## The four sub-questions, answered in order

1. **Does sqlx's pooling and transaction model force a compromise the bare harness
   never had?** The trait signature is untouched, and three real things were
   forced: a captured runtime `Handle` for every operation (`Handle::enter` is
   `!Send` and would break `SendEventStore`); the cursor carrying that handle so
   `PoolConnection::drop` can return its connection; and `SERIALIZABLE` plus a
   bounded retry on the conditional path, because `READ COMMITTED` lets two
   writers racing one boundary both win and a write lock is the move this crate
   exists not to make.
2. **Does CF-13 still pass?** Yes — **and the pass is not evidence.** The *naive*
   arm passes it too (HS-S0064), because this adapter advances off-poll. The
   mechanism is evidenced by direct observation instead: the probe and the
   staleness table.
3. **What bound should the docs promise?** Sub-millisecond unloaded; the duration
   of the longest open write transaction anywhere on the cluster otherwise. It is
   the *remainder* of the holder, not a fraction: 4799 ms against a 5000 ms hold.
   That is the planning figure the open question asked for, and HS-S0066 has put
   it on `happenstance-postgres`'s crate root where a consumer meets it before
   any method — so sub-question 3 is closed in the docs and not only in a record.
4. **Reconsider B-tag?** No. Its branch was "if arm C proves structurally
   expensive"; it did not. B-tag stays rejected on the invariant — per-boundary
   where ES-10 is global — not on cost.

## The losing arms, priced by what they buy before what they cost

A (serialised sequence table) 0.062 and B-const (constant advisory lock) 0.033 at
64 writers: correct, and they buy ES-10 by deleting the reason to reach for
Postgres. B-tag 0.935 — nearly free, and it reproduces the baseline inversion on
disjoint keys. **B-tag is cheap because it buys something weaker**, so ranking by
throughput would have chosen it.

## Where the bill is stated normatively — no new clause, decided not defaulted

`postgres-structural-bill` (HS-S0066) owed a *decision* on whether the three
consumer-facing consequences — frontier `head`, no read-your-own-writes,
cluster-wide staleness — are owed a clause of their own or stay prose inside
ES-10. **Verdict: no new clause.** Long form §9.

Two reasons, the first decisive:

- **No conformance rule can fail an adapter over them.** They are *permissions*
  granted to an adapter, not requirements binding one, and a clause that names no
  rule and no wrong implementation is decorative (CF-4). The single testable
  sentence — a head is a **bound**, not an equality — is already
  `head_is_the_highest_visible_position` under ES-30.
- **A second copy of the visibility invariant is a drift this document has
  already suffered.** ES-10 declares itself the sole statement because VT-12's
  copy diverged within one editing pass. The drift is demonstrable now, not
  merely predicted: ES-10's frozen prose still quotes phase 2's 0.688 ms /
  4010.719 ms, which the adapter re-measurement above supersedes, and the clause
  is `[FROZEN]` and not edited.

The auditor's reading order is recorded in the long form: ES-10, then ES-30's
rule, then §3–§4 here, then the crate's own rustdoc. **`spec/SPECIFICATION.md` is
unedited by phase 10.**

## Inherited, not owned

ES-10's **global** framing rests on the projection checkpoint being global, and
the checkpoint is phase 6's. A boundary-scoped checkpoint could make B-tag's
weaker invariant sufficient and would reopen ADR-0013 and this measurement with
it. The atom must link
`.kb/open-questions/global-versus-per-boundary-visibility-invariant.md` and state
this as inherited rather than settled. **No edit to `.kb/decisions/0013-*`** —
accepted atoms are immutable.

## What it does not decide

- Whether the suite should be able to catch an **off-poll** adapter's visibility
  defect at all, and with what instrument. The port exposes no suspension point
  between allocation and commit; the thing that catches it is adapter-specific.
- The global-versus-per-boundary question.
- `contains_event_id`'s frontier disagreement. ES-41 stays `[PROVISIONAL]`.
- The steady-state ratio, which is owed a machine that can resolve it.

## Links the atom should carry

- `[[0013-position-assignment-and-visibility]]` — the premise, discharged and inherited
- `[[postgres-arm-c-structural-cost]]` — resolved by this
- `[[global-versus-per-boundary-visibility-invariant]]` — the successor premise
- `[[0001-async-port-flavours]]` — why `Handle::enter` was not available
- `references/adr/0024-position-visibility-mechanism.md` — the long form
- `experiments/position-visibility/results/adapter-remeasurement.md` — the numbers
