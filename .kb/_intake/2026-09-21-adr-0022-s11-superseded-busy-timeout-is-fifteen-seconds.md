# ADR-0022 §11 is superseded in part: the busy timeout is fifteen seconds

**Staged for `/redkiln:kb-ingest`.** This is raw material for a decision atom,
not an atom. It records a decision taken on 2026-09-21 by the repository owner,
with the measurement that decided it.

## The shape of the move

**Supersede ADR-0022 §11 only.** ADR-0022 is `accepted` and immutable under KB
authority rule 1, so a fired falsifier is evidence and not licence to edit the
decision's body. The new atom should carry `supersedes: [kb-decision-0022]`
**scoped to §11**, and say in as many words that §4, §6, §7, §9, §10 and §15
stand unchanged.

Three things this deliberately does **not** do:

- It does not settle **§9**'s firing (the captured tokio `Handle` making
  `SqliteEventStoreError::NoRuntime` unreachable). That firing is still owed the
  twenty-line reproduction its own falsifier's wording demands — *"a deployment
  shows"* — and no deployment in this tree shows it.
- It does not settle **§8 / §16** (the append-condition SQL shape, whose
  falsifier cannot fire as written).
- It does not touch **`CONTENDERS`**. §12 states plainly that the contender
  count is *"not this record's to re-open"* — it supplies a number and
  `concurrency-family-and-contender-count` owns the decision. Lowering it to 8
  was therefore never on this table.

So `.kb/open-questions/adr-0022-falsifiers-have-fired.md` is **amended, not
closed**: one of its three findings is now decided, two are not.

## What was falsified

§11's falsifier reads: *"Re-open the busy timeout if any run ever reports
`busy > 0`, which would mean five seconds stopped being generous"*
(`references/adr/0022-append-condition-strategy.md:614-616`).

The claim it protected is in §11's own body: *"Five seconds was enough:
`busy = 0` in every row of the 64-contender table, so the timeout did real work
and never ran out — which is what makes that zero mean something rather than
being the zero an unbounded handler would also have produced."*

**That premise is false.** `experiments/busy-timeout-margin` recorded `busy > 0`
at the shipped `CONTENDERS = 64` in one launch of seven — three attempts in
6,720 — the first nonzero busy count anywhere in this tree. The margin on the
plateau is 1.31x–1.38x, and every `wait_ms` figure in that experiment is a
**lower** bound, by 2.6x on one measured configuration.

Note the falsifier had been **unfalsifiable for as long as it existed**, because
nothing in the tree reported a busy count at all. That is a defect in the
falsifier, not only in the value, and the replacement below is written to not
repeat it.

## What is decided

`crates/happenstance-sqlite/src/connection.rs`'s `BUSY_TIMEOUT_MS` moves from
`5_000` to `15_000`.

**Measured, not reasoned**, and measured on *this adapter* rather than on the
experiment's candidate — which closes caveat 1 of
`experiments/busy-timeout-margin/README.md`. Full method, conditions and caveats
are in `experiments/busy-timeout-margin/results/adapter-cap-sweep.md`.
Worst-case configuration (`--test-threads=1`, so all 64 contenders get the whole
machine):

| `BUSY_TIMEOUT_MS` | launches | red |
| ---: | ---: | ---: |
| 5,000 | 8 | 7 |
| **15,000** | 16 | **0** |
| 30,000 | 8 | 0 |

**The argument that decided it over leaving the cap alone is that raising it is
free in the healthy path.** The busy handler returns the instant the lock is
acquired, so the cap bounds only the pathological tail rather than delaying
every append. Passing runs at the gate's own parallelism took 4.91–5.65 s at
five seconds and 4.99–5.40 s at fifteen — indistinguishable on a shared host.

What it *is* paid for by: a genuinely stuck writer now takes 15 s rather than
5 s to report. That path is rare, and it ends in a red rule rather than a hang,
because the cap stays **finite**. An unbounded handler remains rejected for
exactly ADR-0022's original reason, which is unchanged and should be restated in
the new atom rather than assumed: CF-33 is `[FROZEN]` and forbids the
conformance suite a watchdog, so this constant is the only liveness bound in the
system, and an unbounded wait converts a livelock into a hung job naming no
rule.

## Alternatives that lost

- **Ratify §11 as still correct, recording the firing as an accepted cost.**
  Defensible on the narrow reading, and it was argued: the *semantic* property
  never broke — `committed` is correct in every row — so what fired was
  liveness, and `exhausted` was structurally uninformative on the rows in
  question. **Rejected** because the cost is paid by the wrong person. The suite
  exists to tell an adapter author whether their store is sound, and a
  ~1-in-7 red at the shipped contender count tells them it is not when it is.
- **30,000 ms.** Equally clean on this host, no measured gain over 15,000, and
  it triples the pathological wait. **Rejected** on the core sweep: fewer cores
  measured *better*, by about 450x from twenty cores to one, so a smaller CI
  runner sits in the safer regime and the extra margin is insurance against a
  direction the evidence does not point in.
- **Lowering `CONTENDERS` to 8.** Out of scope by §12, as above.

## The replacement falsifier, and its instrument

The old falsifier failed because nothing could fire it. The new one must name
the instrument that can, and both now exist:

> Re-open this cap if the adapter's own concurrency target goes red on a
> `SQLITE_BUSY` at `BUSY_TIMEOUT_MS = 15_000` on any host, in any
> configuration — reproduce with
> `experiments/busy-timeout-margin/results/adapter-cap-sweep.md`'s loop. Re-open
> it *downward* if the pathological 15 s wait is ever what makes a real failure
> undiagnosable.

## What this explicitly does not fix, and where it goes instead

A store that is momentarily contended and a store that is wrong still arrive as
the same failed `Attempt`. **Raising the cap lowers the rate at which that
conflation bites; it does not remove it**, and no value of this constant can —
CF-33 denies a rule the clock that would tell them apart.

That question is the testkit's, and it is already owned by
`.kb/open-questions/no-fixture-tolerance-for-transient-contention.md`. It should
be updated to record that **the instrument it was blocked on now exists**:
`happenstance_testkit::FaultyStore::contend_next` refuses the next *n* appends
as `AppendError::Store(FaultyStoreError::Contended)`, landed 2026-09-21, with
`crates/happenstance-testkit/tests/contended_store_instruments.rs` demonstrating
that both affected rules reject a merely-contended store — each on a different
assertion, which is why a tolerance would be a change to what those rules assert
rather than one arm on a private enum.

## Source paths

- `references/adr/0022-append-condition-strategy.md` (§11 at :589-616)
- `experiments/busy-timeout-margin/README.md`
- `experiments/busy-timeout-margin/results/busy-timeout-margin.md`
- `experiments/busy-timeout-margin/results/adapter-cap-sweep.md`
- `crates/happenstance-sqlite/src/connection.rs`
- `crates/happenstance-testkit/src/faulty.rs`
- `crates/happenstance-testkit/tests/contended_store_instruments.rs`
- `.kb/open-questions/adr-0022-falsifiers-have-fired.md`
- `.kb/open-questions/no-fixture-tolerance-for-transient-contention.md`

## Related atoms

`kb-decision-0022`, `kb-reference-busy-timeout-margin-001`,
`kb-open-question-adr-0022-falsifiers-fired-001`,
`kb-open-question-testkit-contention-tolerance-001`,
`kb-decision-0064` (the measurement host's declared conditions),
`kb-governance-what-may-refute-a-finding-001`.
