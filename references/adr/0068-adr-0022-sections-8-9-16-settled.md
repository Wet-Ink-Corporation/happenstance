# ADR-0068 — ADR-0022 §8 and §16 are superseded in part, §9 is not decided here, and the rest is ratified

- **Status:** accepted.
- **Date:** 2026-09-29
- **Phase:** 16 (define 1.0)
- **Settles:** the §8/§16 and §9 findings of
  [`.kb/open-questions/adr-0022-falsifiers-have-fired.md`](../../.kb/open-questions/adr-0022-falsifiers-have-fired.md),
  and the ledger row that carries it. It uses the partial-supersession shape of
  [ADR-0065](../../.kb/decisions/0065-adr-0022-s11-busy-timeout-is-fifteen-seconds.md),
  which settled §11 on 2026-09-21.
- **Supersedes, in part:** [ADR-0022](0022-append-condition-strategy.md) §8 items 1 and 2
  (`:375-382`), and §16's falsifier for §8 (`:606-607`). `supersedes: null`;
  `kb-decision-0022` stays `accepted`.
- **Does not decide:** §9. Its falsifier asks for a deployment, and none exists in this tree. §7.
- **Ratifies:** §§1–7, 10 and 12–15, each with its falsifier's state recorded. §8.
- **Authorises one `[FROZEN]` clause edit, made in the same change:** ES-27's `Rejects:` prose
  (`spec/SPECIFICATION.md:4061-4066`). §5.
- **Evidence:** `experiments/correlated-exists-guard/` (one `./run.sh`, 2026-09-05) and
  `experiments/shipped-append-condition-sql/` (one `./run.sh`, 2026-09-03). Both sit outside the
  workspace and outside the gate (CF-34).

---

## 1. The question, and the premise it arrived with that is no longer true

The open question recorded three findings against ADR-0022's falsifiers. §11 fired, and ADR-0065
answered it. §9 fired by a route its wording did not anticipate. And §16's falsifier for §8
*"cannot fire as written"*, because the aggregate it names is not what the adapter emits
(`.kb/open-questions/adr-0022-falsifiers-have-fired.md:125-136`).

The §8 finding was written against `experiments/shipped-append-condition-sql/`. It says that *"the
chain that does ship loses to that aggregate in nine of nine two-tag cells"*
(`adr-0022-falsifiers-have-fired.md:23-25`), and the phase-16 survey carried that sentence forward.
The measurement it rests on is sound. **Its subject is not what ships.** That experiment's
`results/README.md:3-5` dates its run to 2026-09-03. On 2026-09-05, commit `8c8b215`
(*"perf(sqlite): correlate the guard chain, bind its boundary, drop a tautology"*) replaced the
chain it measured, and `0.2.0` shipped five days later with the replacement. The open question was
amended on 2026-09-21 and still says the falsifier *"still cannot fire as written"*
(`:209-214`). That remains true. What changed is what the falsifier should have been guarding, and
`references/seeds/adr-0022-shipped-shape-drift.md:29-41` is the only document that noticed.

So this record does not choose a SQL shape. One was chosen, measured and shipped, and the record
has not caught up with it. What this record does is make the record say what ships.

## 2. What §8 said, and what it was measured on

§8 fixes the general superset test as `GROUP BY position HAVING COUNT(DISTINCT tag) = n`
(`0022:355-360`). It measures the single-tag fast path against its own negative control at
1,093 µs → 556 µs (`:363-367`). It prices a two-tag boundary at *"42 ms to 66 ms per guard
evaluation against 0.2 ms to 0.3 ms for a single-tag one — roughly 200x"* (`:369-372`). From that
it derives two requirements:

1. `tag_cardinality`, and probing most-selective-tag-first, are requirements (`:375-379`);
2. the 200x is a floor on the multi-tag path (`:380-381`).

Every one of those figures was taken on the aggregate. The adapter has emitted three shapes, and
none of them is the aggregate:

| period | shape | where it is measured |
| --- | --- | --- |
| to 2026-09-05 | uncorrelated chain: `position IN (SELECT position FROM event_tag WHERE tag = ?)` per extra tag | `experiments/shipped-append-condition-sql/` |
| 2026-09-05 → now | **correlated** chain: `EXISTS (SELECT 1 FROM event_tag AS m{i} WHERE m{i}.tag = ? AND m{i}.position = seed.position)`, boundary on the seed | `experiments/correlated-exists-guard/` |
| never | `GROUP BY … HAVING COUNT(DISTINCT tag) = n` | both, as the `grouped-adr0022` arm |

The shipped text is `crates/happenstance-sqlite/src/query_sql.rs:769-815`. The seed is aliased and
carries `seed.position > ?` (`:776-792`), and each additional tag is a correlated `EXISTS` (`:810-815`).

## 3. The measurements, read from the tree

Every figure below comes from one run on one host, and is a ratio between shapes within that run
(`experiments/correlated-exists-guard/README.md:162-181`, SQLite 3.53.2 bundled, `rustc 1.97.1`,
Windows 11, NVMe). All six shapes passed 89 conformance rules each, 534 tests, before anything was
timed (`results/README.md:27-31`).

**The guard, two tags, µs, median** (`results/guard-cost.md:23-33`):

| size | scenario | uncorrelated chain (shipped to 09-05) | `grouped-adr0022` | correlated + bounded seed (**ships**) |
| ---: | --- | ---: | ---: | ---: |
| 50,000 | accepted, at head | 17,477 | 9,953 | 32 |
| 50,000 | rejected, unbounded | 17,592 | 10,092 | 33 |
| 10^6 | accepted, at head | 579,883 | 331,397 | 33 |
| 10^6 | rejected, mid-log | 580,703 | 353,897 | 37 |
| 10^6 | rejected, unbounded | 549,047 | 332,810 | 65 |

Both correlated chains beat the aggregate in all nine cells. At 10^6 the unbounded correlated
chain (`chain-exists`, not tabled here) beats it by 2,000x–2,600x (`guard-cost.md:86-92`), and the
bounded-seed chain that ships by roughly 5,000x–10,000x (331,397 / 33, 353,897 / 37 and
332,810 / 65, `guard-cost.md:31-33`). The single-tag control is 8–10 µs on every shape
(`guard-cost.md:12-19`). A table where the shapes differed on that control would be measuring the
harness.

**The ordering** (`results/seed-ordering.md:15-22`), 500,000 events, a 97:1 cardinality ratio,
alternating which ordering runs first:

| shape | most-selective ÷ least-selective |
| --- | --- |
| uncorrelated chain | 37.9x, 35.6x, 43.0x |
| correlated chain, unbounded seed (`chain-exists`) | **0.47x, 0.46x, 0.50x** |

The ordering was measured on `chain-exists`, the correlated chain **without** the boundary on the
seed. The bounded-seed chain that ships was not ordering-measured:
`experiments/correlated-exists-guard/tests/seed_ordering.rs:159` loops over `Shape::Chain` and
`Shape::ChainExists` only. The mechanism below — correlation makes the seed the outer loop — is
the same on both, and that is an inference, not a measurement.

The sign flips. On the uncorrelated chain SQLite materialises the chained arm as a
`LIST SUBQUERY`, so the most selective tag lands on the cheap side of the plan and buys nothing.
The correlated chain cannot be hoisted, so the seed is the outer loop. The plan shows
`SEARCH seed USING PRIMARY KEY (tag=? AND position>?)` and
`SEARCH m0 EXISTS USING PRIMARY KEY (tag=? AND position=?)`, with no `LIST SUBQUERY`
(`results/query-plans.md:44-61`; the shipped shape's plan is `:56-61`).

**The adversarial corpus**, where no tag is selective: the correlated chain does not lose there. It
runs at 122 / 128 µs against 401,116 / 423,501 µs for the aggregate at 500,000 events
(`results/unselective-pair.md:23-26`).

**The sibling experiment's figures still stand for the shape they measured.** On the uncorrelated
chain it found the aggregate winning 9 of 9 at 1.54x–1.86x, and most-selective-first 38.1x–44.0x
backwards across two runs (`.kb/reference/shipped-append-condition-sql-experiment-2026-09.md:62-95`).
`correlated-exists-guard` reproduces the first finding (`guard-cost.md:88-91`, *"the sibling
experiment's finding reproduced"*). They are the history of a shape that no longer ships.

## 4. §8 — superseded at items 1 and 2

**Item 1, superseded.** The multi-tag superset test in `happenstance-sqlite` is the correlated
intersection chain: the seed is the most selective tag, the type constraint and the guard's
boundary sit on the seed, and each further tag is an `EXISTS` correlated to `seed.position`.
`tag_cardinality`, and most-selective-first probing, **remain requirements**. The reason changes.
§8 justified them by an aggregate that makes tag order irrelevant (`tag IN (?,?)` is
order-independent, `seed-ordering.md:55-59`). They are justified instead by the correlation, which
is what makes the seed's selectivity decide the cost (measured on `chain-exists`; §3 says why the
shipped shape is inferred rather than measured here). The crate's public documentation already
states it this way, including the instruction that anyone who returns the chain to `IN (…)` must
delete the requirement in the same change (`crates/happenstance-sqlite/src/event_store.rs:91-105`).

**Item 2, superseded.** *"The 200x is … a floor on the multi-tag path"* describes the aggregate.
On the chain that ships, a two-tag guard at 50,000 events is 32–33 µs against the 8–10 µs
single-tag control in the same run. The multi-tag path is no longer a different order of magnitude.

**What stands in §8:** the single-tag fast path, which emits no chained subquery at all; and the
rejection of `ANALYZE` (`0022:383-385`), because a most-selective-first seed still needs a per-value
count that SQLite does not keep.

**What this does not establish,** carried from the experiment rather than from optimism
(`experiments/correlated-exists-guard/README.md:204-246`). It was measured on:

- two tags only;
- two corpora, 97:1 and 1:1, with no moderately selective pair;
- one SQLite version;
- warm cache.

The write-side cost of the `tag_cardinality` upsert is bounded by arithmetic and has not been
measured (`shipped-append-condition-sql-experiment-2026-09.md:116-119`). The seed's third question,
the quadratic in `Selectivity::read_for` that finding I-5 measured, has been repaired. It now
accumulates into a `BTreeSet` (`query_sql.rs:223`, commit `177dfa0`), so the trade the seed says
*"moved; nobody has re-taken it"* is re-taken here on the ordering's side, and remains unmeasured on
the upsert's.

## 5. ES-27's `Rejects:` prose, a `[FROZEN]` clause, repaired by this record

`spec/SPECIFICATION.md:4061-4066` read, until the change that lands this record:

> … the landed SQLite schema puts tags in a separate table
> (`crates/happenstance-sqlite/src/event_store.rs:62-67`) — measured at roughly
> 200x a single-tag boundary's cost at 50,000 events, which is why ADR-0022
> ships `tag_cardinality` and most-selective-tag-first probing as requirements.

The figure is the aggregate's, and the reason it gives is §8 item 1's, which §4 above supersedes.
The clause's normative content is its MUST and its rules, and neither moves. This record authorises
replacing that sentence's tail (`:4064-4066`) with:

> (`crates/happenstance-sqlite/src/event_store.rs:62-67`) — two orders of
> magnitude on ADR-0022's `GROUP BY` form, which is why the guard ships as a
> correlated chain whose seed `tag_cardinality` orders (ADR-0068).

**The replacement must not change the clause's line count.** It replaces the three lines
`:4064-4066` with three lines, so that no `SPECIFICATION.md:N` citation past the clause moves;
spec-trace and the ledgers depend on those offsets.

The replacement states an order of magnitude on the shape that was measured, and no multiple for
the shape that ships. That follows the open question's own advice (`:138-146`): a new warm-cache
multiple from one host would rot the same way the old one did. The edit landed in the change that
lands this record, line-neutral as above. `event_store.rs:62-67` in the same sentence is a
citation, and repointing it is the repair `kb-governance-referent-not-reasoning-001` allows. It is
not a decision.

## 6. §16 — the replacement falsifier for §8

The old falsifier was *"Re-open it if a future SQLite pushes predicates through an aggregate"*
(`0022:606-607`). It named a mechanism the adapter never used, so it could not fire even while the
condition it existed to catch was happening. It guarded the single-tag fast path, which this record
keeps; the fast path's own trigger is restated at the end of this section.

**Replacement.** Re-open §8 as amended here if either of these happens:

1. On the SQLite the adapter bundles, `EXPLAIN QUERY PLAN` for a multi-tag guard statement shows
   a `LIST SUBQUERY`. That would mean the planner has decorrelated or hoisted the chain, and the
   seed is no longer the outer loop.
2. On the correlated chain, least-selective-first measures cheaper than most-selective-first.
   Reproduce with `experiments/correlated-exists-guard/tests/seed_ordering.rs`, which asserts that
   both orderings return the same position before either one is timed. That instrument runs
   `chain-exists`, not the bounded-seed shape that ships: reproducing it on the shipped shape
   means adding `Shape::ChainExistsBoundedSeed` to the loop at `seed_ordering.rs:159`.

Either one means the requirement has changed sign again. **The first can be asserted inside the
adapter, and today it is not.** The read path's plan is asserted
(`crates/happenstance-sqlite/src/event_store.rs:2418`, `the_page_plan_is_a_merge_and_not_a_sort`),
and the guard's plan is not. Adding that assertion is additive and owed, and **phase 17 owns
it**, alongside §9's reproduction. Until it lands, the falsifier fires by hand, which is better
than a falsifier that cannot fire at all, and not good enough to leave as it is.

**The single-tag fast path keeps a falsifier of its own.** ADR-0022's §8 falsifier was headed
*"§8, the fast path"*, and its trigger was that *"the special case stops earning its branch"*
(`0022:606-607`). The mechanism it named goes; the condition it guarded stays, restated against the
shape that ships: re-open the fast path if the single-tag guard stops being cheaper than the
multi-tag chain in the same run — today 8–10 µs against 32–65 µs (`guard-cost.md:14`, `:25-33`).

Every other falsifier in §16 stands as written. §11's was replaced by ADR-0065. §12's is *"not this
record's to re-open"*.

## 7. §9 — not decided here, and why phase 17 is its owner

**What §9 decided.** Capture a `tokio::runtime::Handle` at construction, prefer it, and fall back
to `Handle::try_current()`, so that `SqliteEventStoreError::NoRuntime` keeps a real meaning
(`0022:389-412`).

**What fired.** The capture is unconditional. It happens in three places:

- `crates/happenstance-sqlite/src/event_store.rs:512`;
- `crates/happenstance-sqlite/src/projection_store.rs:234`;
- `crates/happenstance-postgres/src/event_store.rs:314`.

No constructor clears the captured handle or replaces it. Take a store built inside one runtime and
served after that runtime is gone. Its `spawn_blocking` goes to a runtime that has shut down, and
the read should hang or end in `Worker(JoinError)` (`event_store.rs:1218-1220`). It never ends in
`NoRuntime`.

**What does not exist.** Neither outcome has been observed in this tree. The two tests that keep
`NoRuntime` honest build a store with **no** runtime at all:

- `crates/happenstance-sqlite/tests/read.rs:157-180`;
- `crates/happenstance-sqlite/tests/concurrency.rs:146-176`.

The one place a store is built inside an explicit runtime builds it and drives it on that same
runtime (`examples/transfers-on-sqlite/tests/contention.rs:198-212`). The falsifier's wording,
*"a deployment shows"* (`0022:609`), asks for exactly the twenty lines nobody has written. Adopting
a remedy for an outcome nobody has reproduced would break this repository's own rule: write the
failing case first, then fix it.

**So §9 is neither superseded nor ratified.** It is assigned. **The owner is phase 17**, for this
reason. The remedy is one of two things, and only the reproduction can say which:

- **Additive.** A constructor that takes an explicit `Handle`, or one that declines to capture.
  `kb-decision-0058` already calls this *"additive either way"* and gives it no deadline.
- **Behavioural.** A change to what `SqliteEventStore::open` / `::new` and `PostgresEventStore::new`
  capture, or to which variant a stranded read reports. That changes the behaviour of two adapters
  that were published at `0.2.0`, and after `1.0.0` it would need a major version.

Phase 17 is the breaking window. It is therefore the last point where the reproduction can come
out either way at no extra cost. Phase 17 writes the reproduction, runs it against both adapters (`happenstance-sqlite` and
`happenstance-postgres`, the two that capture),
and records which remedy it calls for. If that remedy is additive, it can land after 1.0 without a
further decision. If it is behavioural, it lands in phase 17.

**§9's second limb, recorded without a verdict.** *"… or if `postgres-and-neon-stores` finds the
seam generalising — in which case it becomes a testkit question"* (`0022:609-612`). Postgres did
adopt the seam. It also met a handle-sampling defect of its own: a cursor recorded `None`, its
`Drop` found no runtime on a bare thread, and the result leaked a pooled connection with an open
`REPEATABLE READ` transaction. The fix prefers the executing runtime over the captured one
(`crates/happenstance-postgres/src/read_stream.rs:458-480`). That is evidence the seam's behaviour
is not specific to one adapter. It is not the reproduction §9 asks for. Phase 17 reads it alongside
the reproduction.

## 8. The rest is ratified, with each falsifier's state

| § | verdict | falsifier state, read at HEAD |
| --- | --- | --- |
| 1–3 | ratified | context, the stale driver half and the instrument; no falsifier of their own |
| 4 | ratified | not fired: `event_tag` is still keyed `(tag, position)`, and no measurement over a large contiguous range has put `EXISTS` ahead |
| 5 | ratified | the figures that lost; a record, not a verdict |
| 6 | ratified | not fired: no deployment's dominant read is broad |
| 7 | ratified | the migration-1 correction; `event_type` is a covering column (`event_store.rs:80-90`) |
| 10 | ratified | §16 names no falsifier for §10; its own re-open trigger (`0022:452-455`) — Postgres or Neon independently needing the same per-item decomposition — has not fired. `happenstance-neon`'s probe unions per guard, not per query item (`crates/happenstance-neon/src/event_store.rs:427-457`). The merged read path (`b3c8d84`, `query_sql.rs:516-520`) keeps the decomposition adapter-private, which is §10's verdict applied to a second caller |
| 12 | ratified | it supplied a number and declined to apply it. `CONTENDERS` is 64 (`crates/happenstance-testkit/src/concurrency.rs:247`), set by the owner §12 named |
| 13 | ratified as a non-verdict | ES-17 stays with `kb-open-question-es-17-two-adapter-measurement-001` |
| 14 | ratified as a non-verdict | CF-40's clause home stays with `kb-open-question-cf-40-ownership-001` |
| 15 | ratified | a statement about the change that landed ADR-0022 (the `todo!()` bodies, the untouched specification), and not a description of the crate today |

§10 needs one qualification. The seed notes that `query_sql`'s two callers stopped passing the
same argument when the read path became a merge (`references/seeds/adr-0022-shipped-shape-drift.md:238-243`).
That is a question about a private module's seam, and §10's verdict (adapter-private) covers it. It
does not reopen §10.

## 9. Alternatives rejected

**Supersede ADR-0022 whole.** Rejected because it would misdescribe at least ten sections that are
still the basis of a published adapter. ADR-0065 §"Why `supersedes` stays `null`" makes the same
argument for §11.

**Ratify §8 as still correct, with the firings recorded as accepted costs.** Rejected because
§8's reason is false of the code that ships, and ES-27 quotes that reason inside a `[FROZEN]`
clause. Ratifying it would leave the adapter's public documentation and its decision record
contradicting each other, with the record wrong.

**Adopt the aggregate, or the uncorrelated chain bounded in every arm.** The survey offered these
two as the choice. Both are measured and both lose. The aggregate loses to the unbounded correlated
chain (`chain-exists`) by 2,000x–2,600x at 10^6, and to the bounded-seed chain that ships by roughly
5,000x–10,000x (`guard-cost.md:31-33`, `:86-92`). The bounded uncorrelated chain collapses without an anchor, to 561,838 µs
(`guard-cost.md:62-66`). Neither is what ships, and the survey's framing predates `8c8b215`.

**Decide §9 on reasoning.** Rejected, for the reason §7 gives: reproduce first, then fix.

## 10. Consequences

- `kb-open-question-adr-0022-falsifiers-fired-001` is answered for §8 and §16. For §9 alone it is
  owned by phase 17.
- `references/seeds/adr-0022-shipped-shape-drift.md` has an answer to its questions 1, 2 and 4:
  the chain is the decided shape; the falsifier is replaced; the read-path merge stays
  adapter-private. Its question 3, whether `tag_cardinality` earns its keep, is answered on the
  ordering's side and left unmeasured on the upsert's.
- The ES-27 edit (§5) landed in the change that lands this record, line-neutral.
- One item is owed, and it is not breaking: a guard-plan assertion in `crates/happenstance-sqlite`
  (§6). Phase 17 owns it.
- No public signature moves, and no partition constant moves. Nothing here belongs on phase 17's
  breaking list except §9's reproduction, which is on it because it decides whether the remedy
  would be breaking. Phase 17 carries both as work items.

## 11. Falsifiers of this record

- **§8 as amended:** §6's two conditions.
- **§9's assignment:** if the reproduction shows the stranded read reporting `NoRuntime` after all,
  the hazard is not real, §9 is ratified as written, and the assignment closes with nothing to do.
- **The ratifications in §8:** the falsifiers ADR-0022 §16 names for §4 and §6, and §10's own
  re-open trigger at `0022:452-455`, which all still stand.
- **The fast path:** §6's restated trigger.
