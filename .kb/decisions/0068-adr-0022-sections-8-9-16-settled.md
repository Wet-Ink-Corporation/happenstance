---
id: kb-decision-0068
title: ADR-0022 §8 and §16 are superseded in part, §9 is not decided here, and the rest is ratified
kind: decision
status: accepted
authority_tier: decision
adr_id: ADR-0068
reversibility: medium
phase: 16
supersedes: null
superseded_by: null
summary: >-
  Takes up the three questions kb-open-question-adr-0022-falsifiers-fired-001 left open after
  kb-decision-0065 settled §11, and answers them one section at a time. As with 0065, the record
  supersedes only part of ADR-0022, so supersedes stays null and kb-decision-0022 stays accepted.
  §8 is superseded at items 1 and 2, and §16's falsifier for §8 goes with them. The open question
  and the phase-16 survey both reasoned from experiments/shipped-append-condition-sql/, whose
  subject is the uncorrelated `position IN (…)` chain. That chain stopped shipping at 8c8b215 on
  2026-09-05, before 0.2.0. What ships now is a correlated `EXISTS` intersection chain seeded by
  the most selective tag, with the guard's boundary bound into the seed. It was measured in
  experiments/correlated-exists-guard/ after all 89 conformance rules ran against every shape.
  At 10^6 events a two-tag guard costs 33–65 µs against the uncorrelated chain's 549,047–580,703
  µs. The unbounded correlated chain (chain-exists) beats the GROUP BY aggregate that §8 names by
  2,000x–2,600x, and the bounded-seed chain that ships by roughly 5,000x–10,000x. On chain-exists
  most-selective-first earns 2.0x–2.2x, where on the old shape it cost 35.6x–43.0x; the shipped
  bounded-seed shape was not ordering-measured.
  So the requirement stands and its reason changes: tag_cardinality is a requirement because the
  chain is correlated, and not for the reason §8 gave, which was an aggregate the adapter never
  emitted. The replacement falsifier names an instrument that can fire it: a `LIST SUBQUERY` in
  the multi-tag guard's plan, or least-selective-first measuring cheaper on the correlated chain.
  ES-27's `Rejects:` prose quoted the aggregate's "roughly 200x" as if it described what ships.
  Repairing it edits a [FROZEN] clause, so this record authorises the replacement text, and the
  line-neutral edit lands in the same change. §9 is not
  decided. Its falsifier asks that "a deployment shows" the cost, and the reproduction that would
  show it (a store built on one runtime and read after that runtime is gone) exists nowhere in the
  tree. The reproduction is assigned to phase 17, because it is what decides whether the remedy is
  additive or a change of behaviour on the constructors of two published adapters, and the
  breaking window is the last place the second answer is cheap. Sections 1–7, 10 and 12–15 are
  ratified, with the fired and unfired falsifiers recorded against each.
depends_on:
  - kb-decision-0022
  - kb-decision-0065
related:
  - kb-open-question-adr-0022-falsifiers-fired-001
  - kb-reference-shipped-append-condition-sql-001
  - kb-reference-append-condition-experiment-001
  - kb-decision-0058
  - kb-open-question-es-17-two-adapter-measurement-001
  - kb-open-question-cf-40-ownership-001
  - kb-open-question-query-plan-parameter-chunking-001
  - kb-open-question-testkit-contention-tolerance-001
source_paths:
  - references/adr/0068-adr-0022-sections-8-9-16-settled.md
  - references/adr/0022-append-condition-strategy.md
  - references/seeds/adr-0022-shipped-shape-drift.md
  - experiments/correlated-exists-guard/
  - experiments/shipped-append-condition-sql/
  - crates/happenstance-sqlite/src/query_sql.rs
  - crates/happenstance-sqlite/src/event_store.rs
  - crates/happenstance-sqlite/src/projection_store.rs
  - crates/happenstance-postgres/src/event_store.rs
  - crates/happenstance-postgres/src/read_stream.rs
  - spec/SPECIFICATION.md
last_reviewed: 2026-09-29
---

# ADR-0022 §8 and §16 are superseded in part, §9 is not decided here, and the rest is ratified

## Decision

This record supersedes part of `kb-decision-0022` (ADR-0022), in the same way `kb-decision-0065`
did. It names the sections it corrects, carries `depends_on: [kb-decision-0022, kb-decision-0065]`,
and leaves the old atom's `status` and `superseded_by` alone. It does not flip ADR-0022 to
`superseded`, because that would be false: most of ADR-0022 is still the record
`happenstance-sqlite` is built on. Section by section:

| § | verdict here |
| --- | --- |
| 1–3 | ratified: the question, the stale driver half (`rusqlite`, no pool), the instrument |
| 4, 5 | ratified: the `max(position)` guard inside `BEGIN IMMEDIATE`, and what lost |
| 6, 7 | ratified: `event_tag(tag, position)` `WITHOUT ROWID`, `event_type` covering |
| **8** | **superseded at items 1 and 2**; the single-tag fast path and the `ANALYZE` rejection stand |
| **9** | **not decided here**; the reproduction is owed at phase 17 |
| 10 | ratified: `index_arms()` rejected, decomposition adapter-private |
| 11 | already superseded in part by `kb-decision-0065`; untouched here |
| 12–15 | ratified as the record of what that change supplied and declined |
| **16** | **superseded for §8 only**; every other falsifier in it stands |

## §8: the multi-tag shape is a correlated chain, and the ordering requirement survives on new ground

§8 made `GROUP BY position HAVING COUNT(DISTINCT tag) = n` the general superset test. It priced
the multi-tag path at "roughly 200x" a single-tag one (`references/adr/0022-append-condition-strategy.md:370-372`)
and derived two requirements from that figure (`:375-382`). The adapter has never emitted the
aggregate. Until 2026-09-05 it emitted an uncorrelated `position IN (SELECT …)` chain, on which
`experiments/shipped-append-condition-sql/` measured the aggregate winning 9 cells of 9 at
1.54x–1.86x. On that same chain most-selective-first measured 38.1x–44.0x *backwards*
(`.kb/reference/shipped-append-condition-sql-experiment-2026-09.md:62-95`).

**That is not the shape that ships, and it had stopped shipping before `0.2.0`.** Commit `8c8b215`
(2026-09-05) rewrote the chain as a correlated `EXISTS (… WHERE tag = ? AND position = seed.position)`
and bound the guard's boundary into the seed (`crates/happenstance-sqlite/src/query_sql.rs:769-815`).
`experiments/correlated-exists-guard/` measured the new chain in one run, after 534 conformance tests
(six shapes × 89 rules) had passed:

- At 10^6 events a two-tag guard costs **33 / 37 / 65 µs** (at head / mid-log / no boundary),
  against 579,883 / 580,703 / 549,047 µs for the uncorrelated chain (`results/guard-cost.md:31-33`;
  `README.md:17-21`).
- Both remediated chains beat `grouped-adr0022` in all nine cells. At 10^6 the unbounded
  correlated chain (`chain-exists`) wins by **2,000x–2,600x** (`results/guard-cost.md:86-92`), and
  the bounded-seed chain that ships by roughly **5,000x–10,000x** (`results/guard-cost.md:31-33`).
- Most-selective-first is correct on the correlated chain. On `chain-exists`, the correlated
  chain without the boundary on the seed, it runs at **0.46x–0.50x** the least-selective ordering,
  a 2.0x–2.2x win, where the uncorrelated chain ran it at 35.6x–43.0x
  (`results/seed-ordering.md:15-22`). The bounded-seed shape that ships was not ordering-measured
  (`tests/seed_ordering.rs:159` runs `Chain` and `ChainExists` only); that it behaves the same is
  inferred from the plan, not measured.
- The plan has no `LIST SUBQUERY` (`results/query-plans.md:44-61`; the shipped shape at `:56-61`), and the single-tag control is
  8–10 µs on every shape (`results/guard-cost.md:14`).

What this settles:

1. **Item 1 is superseded.** The multi-tag guard is the correlated intersection chain seeded by
   the most selective tag, with the boundary on the seed. `tag_cardinality` and
   most-selective-first stay requirements, but for a different reason: the chain is correlated,
   so the seed is the outer loop. The reason §8 gave was an aggregate the adapter never emitted.
   The public statement of the requirement already says this
   (`crates/happenstance-sqlite/src/event_store.rs:91-105`).
2. **Item 2 is superseded.** The 200x belongs to the aggregate. On the chain that ships, a
   two-tag guard at 50,000 events is 32–33 µs against the 8–10 µs single-tag control in the same
   run (`results/guard-cost.md:14,25-27`). The multi-tag path is no longer a different order of
   magnitude.
3. **The single-tag fast path stands**, and so does the rejection of `ANALYZE`
   (`0022:383-385`). The fast path keeps a falsifier, restated in §16 below.

What stays unmeasured, stated so that nobody reads it as measured: three or more tags, a
moderately selective pair, cold cache as a first-class column, older SQLite versions
(`experiments/correlated-exists-guard/README.md:204-246`), and the write-side cost of the
`tag_cardinality` upsert. The seed's question about the quadratic in `Selectivity::read_for` has
been answered: it accumulates into a `BTreeSet` (`query_sql.rs:223`, `177dfa0`).

**ES-27.** Its `Rejects:` prose (`spec/SPECIFICATION.md:4272-4277`) quoted the aggregate's "roughly
200x" as the reason the adapter ships these requirements. The clause is `[FROZEN]`, so this record
authorises the repair, and the replacement text is in the long form's §5. The edit landed in the
change that lands this record, replacing three lines with three so that no later
`SPECIFICATION.md:N` citation moves. The repair does not pin another multiple measured warm, on one
host.

## §16: the replacement falsifier for §8

Re-open §8 if either of the following happens:

- the `EXPLAIN QUERY PLAN` of the adapter's multi-tag guard statement, on the SQLite it bundles,
  shows a `LIST SUBQUERY` (the planner has decorrelated or hoisted the chain);
- least-selective-first measures cheaper than most-selective-first on the correlated chain
  (reproduce with `experiments/correlated-exists-guard/tests/seed_ordering.rs`; it runs
  `chain-exists`, so reproducing it on the shape that ships means adding
  `Shape::ChainExistsBoundedSeed` to the loop at `seed_ordering.rs:159`).

Either would mean the requirement has changed sign again. The first can be asserted in the
adapter's own tests, but it is not asserted there today: the read path's plan is
(`event_store.rs:2939`), and the guard's plan is not. That assertion is additive and owed, and
phase 17 owns it. Until it exists, this falsifier fires by hand.

The single-tag fast path keeps its own falsifier. ADR-0022's was headed *"§8, the fast path"*
(`0022:606-607`); the mechanism it named goes, and the condition it guarded is restated: re-open the
fast path if the single-tag guard stops being cheaper than the multi-tag chain in the same run
(today 8–10 µs against 32–65 µs, `results/guard-cost.md:14,25-33`).

## §9 is not decided here, and phase 17 owns it

§9's falsifier asks that *"a deployment shows"* the captured `Handle` costing something
(`0022:609-612`). The capture is unconditional in the three places it is written:
`crates/happenstance-sqlite/src/event_store.rs:513`, `projection_store.rs:234` and
`crates/happenstance-postgres/src/event_store.rs:314`. What the open question describes is a store
that outlives its runtime and whose reads hang, or yield a `Worker(cancelled)` item, but never
`NoRuntime`. That outcome was reasoned from tokio's semantics. Nothing in the tree reproduces it:
`tests/read.rs:157-180` and `tests/concurrency.rs:146-176` cover a store with *no* runtime, and
`examples/transfers-on-sqlite/tests/contention.rs:198-212` constructs and drives the store inside
one runtime. **The reproduction does not exist, so §9 is neither superseded nor ratified here.**

**The owner is phase 17, not a later phase**, and the reason is classification rather than
urgency. If the reproduction confirms the hazard, the remedy is one of two things:

- a new constructor, which is additive and has no deadline (`kb-decision-0058`);
- a change to what the existing `open` / `new` capture, or to which variant a stranded read
  reports. That changes the behaviour of two published adapters.

Only the reproduction says which. After `1.0.0` the second becomes a major version, so the twenty
lines belong before the breaking window closes. Phase 17 writes them, against both adapters that
capture (`happenstance-sqlite` and `happenstance-postgres`). If the
remedy is additive, it may land after 1.0.

§9's second limb, *"if `postgres-and-neon-stores` finds the seam generalising"*, is recorded
without a verdict. Postgres adopted the same capture, and it produced a handle-sampling defect of
its own (`crates/happenstance-postgres/src/read_stream.rs:458-480`).

## The rest is ratified, with each falsifier's state recorded

- **§4's falsifier has not fired.** The tag storage has not changed, and no measurement over a
  large contiguous range has put the probe arm ahead.
- **§6's falsifier has not fired.** No deployment's dominant read is broad.
- **§10 has no falsifier in §16**; its own re-open trigger (`0022:452-455`), Postgres or Neon
  independently needing the same per-item decomposition, has not fired. `happenstance-neon`'s
  probe unions per guard, not per query item (`crates/happenstance-neon/src/event_store.rs:441-471`),
  and stays adapter-private. The merged read path (`b3c8d84`) is the decomposition staying
  adapter-private in SQLite too, which is §10's verdict.
- **§12** supplied a number. `CONTENDERS` is now 64 (`crates/happenstance-testkit/src/concurrency.rs:271`),
  set by the owner §12 named.
- **§13 and §14** are non-verdicts whose owners are unchanged:
  `kb-open-question-es-17-two-adapter-measurement-001` and `kb-open-question-cf-40-ownership-001`.
- **§15** is ratified as a statement about the change that landed ADR-0022, not as a description
  of the crate today.

## What follows

`kb-open-question-adr-0022-falsifiers-fired-001` can close for §8 and §16. It stays open for §9
alone, owned by phase 17. The ES-27 edit landed in the same change as this record. The guard-plan
assertion is owed, and phase 17 owns it together with §9's reproduction.
