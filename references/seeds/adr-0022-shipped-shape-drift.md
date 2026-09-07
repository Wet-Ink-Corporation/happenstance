# Seed — ADR-0022 has never described what ships, and the gap just widened

Raw material for the runbook's ADR pass. This states a problem and what is now
known about it. It deliberately does **not** write the decision, choose between
the options, or amend ADR-0022: an accepted decision atom is immutable, and
correcting one means a new atom that supersedes it.

## Where this stands

**ADR-0022 §8 fixes the general multi-tag superset test as
`GROUP BY position HAVING COUNT(DISTINCT tag) = n`, and the adapter has never
emitted it.** `crates/happenstance-sqlite/src/query_sql.rs` emits an
intersection chain. That was true before 2026-09-05 and it is true now.

Three things follow, and the third is new:

1. **The two shipped requirements ADR-0022 §8 derives were measured on a form
   that does not run.** The single-tag fast path (1,093 µs → 556 µs) and
   `tag_cardinality`'s most-selective-first ordering (*"roughly 200x"*) were both
   taken against the aggregate.
   `experiments/shipped-append-condition-sql/` established this and priced the
   consequence: on the shape that shipped, the documented ordering cost
   **36x–43x** rather than earning anything.
2. **§16's falsifier for §8 cannot fire.** It reads *"re-open it if a future
   SQLite pushes predicates through an aggregate, at which point the special case
   stops earning its branch"* — and the aggregate it names is not what runs. A
   decision whose stated re-opening condition is unreachable has stopped being a
   decision and become a sentence.
3. **The shipped shape has now moved again, and the record does not say so.**
   `experiments/correlated-exists-guard/` measured the intersection chain
   rewritten as a **correlated** `EXISTS` with the guard's boundary bound into
   the seed arm, and that is what `query_sql.rs` emits as of this seed. At 10^6
   events a two-tag guard went from 579,883 µs to 33 µs; the documented ordering
   flipped from a 36x–43x penalty to a 2.0x–2.2x win; and the plan lost its
   `LIST SUBQUERY` entirely. Conformance was run first — 89 rules against every
   candidate shape, 534 tests — and the whole gate is green.

   That change is *consistent with* ADR-0022's intent and *undescribed by* its
   text. The adapter's own module documentation was updated to match; the
   decision record was not, because that is not a side effect a code change gets
   to have.

   **And then it moved once more, on the read path.** `fetch_page` no longer
   wraps the matched set at all: it emits a compound of windowed arms with the
   page budget on the compound, joined to `event`, which SQLite runs as merged
   co-routines with early termination. Sixteen cells out of sixteen against the
   three alternatives, at both of VT-23's arm-count floors
   (`results/merge-join.md`). Question 4 below records how that was arrived at.

   So the adapter's shape has changed three times in a week and the record has
   described none of them. That is the whole of why this seed exists, and the
   chronology in §4 is kept rather than collapsed because three of the four
   candidates were refuted by a measurement rather than by an argument, and a
   pass that only sees the winner cannot tell which.

## What the ADR pass is being asked to settle

Four questions, in the order they depend on each other.

1. **Is the intersection chain the decided shape, or the aggregate?** The
   evidence now favours the chain unambiguously: both remediated chain forms beat
   `grouped-adr0022` in all nine cells of
   `experiments/correlated-exists-guard/results/guard-cost.md`, by 2,000x–2,600x
   at 10^6 events. But "the measurements favour it" is not the same as "the
   record says it", and §8 currently says the other thing.
2. **What replaces §16's falsifier?** Whatever the answer to (1), the new record
   needs a re-opening condition that names something that can actually happen.
3. **Does `tag_cardinality` still earn its keep?** It is a migration-1 table and
   an upsert inside every write transaction. Its ordering now buys 2.0x–2.2x
   rather than costing 36x–43x, which is a real change of sign — but finding I-5
   measured `Selectivity::read_for` at **40x** a `BTreeSet` at VT-23's 128-item
   floor, *inside the write lock*, and that has not been repaired. The trade
   moved; nobody has re-taken it.
4. **I-3's outer wrapper.** The shape is settled and shipped; what is open is
   whether the record says so, and what the seam between the two callers should
   be. The rest of this item is the chronology that got there, in the order it
   happened, because three of the four candidates were refuted by a measurement
   and the fourth was found by reading a manual — and a pass shown only the
   winner cannot tell which claims were tested.

   `fetch_page` used to wrap every chunk in a second, uncorrelated
   `WHERE position IN (<matched>)`, and that was the read path's floor: the
   correlated chain was worth 1.7x there against 1,617x–1,681x on the guard.
   `experiments/correlated-exists-guard/results/read-path.md` prices three
   candidates and finds a **crossover** rather than a winner — the best shape for
   a broad query is 1,089x better there and 2.3x worse on a selective one.

   Its §4 proposes an approach in three parts: an unconditional `Query::all`
   special case (a literal tautology, needing no cardinality estimate); a
   cardinality-conditional wrapper for tagged queries; and nothing for the middle
   until the crossover threshold is measured with more than two points.

   **The first has now shipped**, on the ground that it needs no estimate: it is
   true by construction that a chunk matching every event makes the membership
   test vacuous. `results/all-query-wrapper.md` measures it, and corrects the
   mechanism the proposal was reasoned from — the tautology is *not*
   materialised, because `position` is the rowid; SQLite drives the scan from the
   subquery and `resume_from` is then unpushable, so each page re-walks the
   prefix below it and a full replay is quadratic in the log's length. 56,310 µs
   to 182 µs on a page 90% of the way through 500,000 events.

   **Parts (2) and (3) are the ones this pass owns.** They are a rule about when
   the adapter changes its query plan on data it samples, which is a decision and
   not an optimisation.

   **There is a fourth candidate, and it makes the rule unnecessary rather than
   easier.** Every candidate in `read-path.md` argues about which
   side of the join to drive from. None of them asks why the wrapper's own
   `resume_from`, ceiling and page budget stop at the subquery boundary — the
   adapter *builds* that subquery, so it could push the read's window into each
   arm instead of applying it outside:

   ```sql
   SELECT position FROM (
     SELECT seed.position AS position FROM event_tag AS seed
     WHERE seed.tag = ? AND seed.position >= ? AND seed.position <= ?
       AND EXISTS (SELECT 1 FROM event_tag AS m0
                   WHERE m0.tag = ? AND m0.position = seed.position)
     ORDER BY seed.position ASC LIMIT 512)
   ```

   The soundness argument is one the adapter already relies on: the merged top
   *b* of a union is a subset of the union of the per-arm top *b*, which is why
   `fetch_page` already bounds each **chunk** by the page budget. Bounding each
   **arm** the same way is the same claim one level down. The subquery wrapper is
   required rather than stylistic — SQLite rejects a bare `LIMIT` on a compound
   arm.

   The matched set is then at most `budget x arms` whatever the corpus, so the
   `IN` shape stops being bad on a broad query and the crossover the conditional
   rule exists to navigate **does not arise**. The plan gains
   `SEARCH seed USING PRIMARY KEY (tag=? AND position>? AND position<?)`: the
   window becomes a seek into the interior of one contiguous `(tag, position)`
   range instead of a walk from its start.

   **Measured, on this crate's harness rather than sketched.**
   `results/windowed-arms.md`: 500,000 events, two corpora, three replay depths
   and one backwards cell, the returned page compared column-by-column every
   round before any time was recorded. It wins every cell — **4.7x–8.8x** over
   what ships on a selective corpus, **1,129x–1,732x** on an unselective one,
   and **10.2x–12.4x** over `wrapper-exists`, the candidate the crossover was
   about, while staying ahead of it on the unselective corpus too.

   **So question 4 changes shape.** It was *"what threshold decides between two
   plans?"*. It is now *"is there a reason to keep either of them?"* A rule that
   navigates a crossover needs a cardinality estimate, a threshold fitted to the
   points someone happened to measure, and an adapter whose query plan depends on
   data it sampled. A shape that wins both ends needs none of those.

   It does **not** remove the read path's dependence on `tag_cardinality`: the
   windowed arm still seeds on the most selective tag, so question 3 stands
   unchanged. What it removes is a *second*, new dependence that (2) would have
   introduced — sampling to pick a plan, on top of sampling to order a chain.

   What it does cost is real, and is this pass's to weigh: `query_sql::chunks`
   would take the read's window and direction. Only the read path has one — the
   guard takes `max(position)` over the whole matched set and has no window at
   all — so the two callers of the one module they deliberately share would stop
   passing the same shape of argument. That is a decision about `query_sql`'s
   seam, which is exactly the kind of thing ADR-0022 §10 reasoned about when it
   refused to mint `Query::index_arms()`.

   **That gap has now been measured, and it takes half the claim back.**
   `results/wide-arms.md`: the same three shapes at VT-23's 128-item floor, on a
   store where 128 items name 128 distinct tags rather than repeating a few.

   * The windowed arm still beats **what ships** in all eight cells — 2.2x–5.1x
     on a query whose 128 arms partition the log, 1.4x–3.7x on one where 127
     arms match nothing.
   * It no longer beats `wrapper-exists` everywhere. On the broad 128-item query
     it is **1.4x–3.4x worse**, because `budget x arms` is 65,536 positions
     materialised to return 512 — bounded, as the argument said, but still 128x
     more work than the answer needs, where `wrapper-exists` materialises
     nothing.
   * On the selective 128-item query `wrapper-exists` is **1,264x–1,747x worse**
     — 25.9 seconds for one page, the slowest statement in this crate. Width
     makes `read-path.md`'s *"2.3x worse on a selective query"* an
     understatement by three orders of magnitude.

   **And then a fifth candidate settled it, because the first four were all
   answering the wrong question.** `results/merge-join.md`.

   `in-exists`, `wrapper-exists` and the windowed arm all produce the whole
   matched set and then take a page from it — they differ only in whether they
   materialise it, test it per row, or truncate it per arm. That is why each is
   best at one end of some axis, and why every new axis produced a new crossover.
   A paged read does not ask *"which positions match?"*; it asks *"the next
   `budget` matching positions in order"*, and the answer to that is a **merge of
   ordered streams with early termination**.

   SQLite already has it. Its compound-`SELECT` handling runs each arm as a
   co-routine and merges them in sorted order, abandoning the rest when the
   `LIMIT` is met (<https://sqlite.org/lang_select.html>); PostgreSQL's planner
   does the same under the name `MergeAppend` and has pushed `LIMIT` into union
   arms since 2005. **The windowed arm was a hand-rolled, worse version of a
   standard optimisation both engines already have** — worse because a per-arm
   limit bounds work at `budget x arms` where a merge bounds it at `budget`.

   The shape is the arms unioned with the window on each and the **budget on the
   compound**, joined to `event` rather than fed to `position IN (…)`:

   ```sql
   SELECT event.<cols> FROM event JOIN (
       <arm> UNION <arm> … ORDER BY position ASC LIMIT ?
   ) AS m ON m.position = event.position ORDER BY event.position ASC
   ```

   It wins **sixteen cells out of sixteen** across both arm-count floors, both
   corpora at each, both directions and three replay depths: 4.0x–11.6x and
   1,187x–1,993x over what ships at one arm, 82x–108x and 2.7x–4.3x at 128, and
   ahead of `wrapper-exists` everywhere by 11x–15x and 1,625x–1,904x. No cell in
   either table has a different winner, which is the first time that has been
   true here.

   **So question 4 becomes a much smaller question, and it has been acted on.**
   It was *"what threshold decides between two plans?"*, then *"is there a reason
   to keep either of them?"*. It is now *"is there any reason not to adopt the
   shape both engines' planners already implement?"* — and this evidence names
   none. There is no crossover to navigate, so no threshold, no sampling for plan
   choice, and no query plan that depends on data the adapter measured.

   **`query_sql` now emits it.** `Window` and `page_statements` are new; `chunks`
   is the guard's alone; `fetch_page`'s outer wrapper is gone rather than
   replaced. The whole gate is green, the 89 conformance rules included, and
   three assertions came with it — two on the query plan in `event_store.rs`, and
   one in `experiments/correlated-exists-guard/tests/emitted_sql.rs` that traces
   the statement off a running store.

   **Shipping it does not settle the record, and this seed still stands.** What
   the adapter emits and what `.kb/decisions/` says have now diverged for the
   third time in one week; the difference is only that the divergence is
   documented in the adapter and measured in an experiment rather than
   discovered later. Questions 1, 2 and 3 are untouched by any of it.

   Three things the pass owns about the shape itself, and they are about seams
   and risk rather than about which one is faster:

   * **The two callers of one module stopped passing the same argument.** That
     module doc's opening claim — *"one question, asked in two places"* — is now
     false as written: the guard asks *"is there a matching position above the
     boundary?"* and a paged read asks for *"the next `budget` positions in
     order"*, which is not the same question and never was. Whether that seam
     should stay one module is the pass's call.
   * **The shape depends on a planner choice that cannot be requested.** Every
     cell measured reported `MERGE (UNION)`; `USE TEMP B-TREE FOR ORDER BY`
     inside the compound would mean SQLite declined and the statement had become
     a sort of the whole matched set. That is a falsifier an adapter can assert
     on in its own tests, and it is a better §16 re-opening condition than the
     one ADR-0022 has.
   * **Above `MAX_QUERY_ARMS_PER_STATEMENT` the merge is per chunk**, so a query
     of 800 items merges twice and the page budget is spent twice before the
     Rust-side merge. That is the same soundness argument the adapter already
     makes for chunking, but it is argued and not measured.

   Still unmeasured: **arm counts between 1 and 128, and above 400.** Two points,
   both floors, nothing between or beyond them.

## What must remain true

- **An accepted decision atom is immutable** (`CLAUDE.md`, *Where the work
  lives*). Correcting ADR-0022 means a new atom that supersedes it, authored by
  `/redkiln:kb-ingest` from `.kb/_intake/`, not a hand edit.
- **A provisional marker must name an artefact, a measurement or a deployment**
  (`spec/SPECIFICATION.md:203-208`). §16's current falsifier names a code path
  that does not exist, which is the failure mode that rule exists to prevent.
- **A benchmark never gates a merge** (CF-34). Every figure cited above comes
  from a crate outside the workspace that `cargo xtask ci` cannot see.
- **Conformance first.** Every shape whose figure is quoted passed all 89 rules
  before it was timed, and the shipped change passed the whole gate.

## Supporting material

| Path | What it carries |
| --- | --- |
| `experiments/correlated-exists-guard/README.md` | the change that shipped, what did not, and why |
| `.../results/guard-cost.md` | six shapes x four scenarios x three log sizes |
| `.../results/read-path.md` | I-3's wrapper, three candidates, the crossover, and the proposed approach |
| `.../results/all-query-wrapper.md` | the part of that approach which shipped, and the mechanism it corrected |
| `.../results/windowed-arms.md` | the fourth candidate at one arm: one shape winning both ends of the axis |
| `.../results/wide-arms.md` | the same three shapes at VT-23's 128-item floor, where that stops holding |
| `.../results/merge-join.md` | **the shape to decide on** — SQLite's own co-routine merge, sixteen cells out of sixteen |
| `.../results/seed-ordering.md` | the ordering policy's sign flip, both shapes |
| `.../results/unselective-pair.md` | the adversarial corpus, and the early-exit hypothesis it produced |
| `experiments/shipped-append-condition-sql/` | the closed record of the shape that shipped until 2026-09-05 |
| `references/adr/0022-append-condition-strategy.md` | §8, §10, §16 — the text under question |

## Deliberately not decided here

**Everything above.** This seed exists because a code change should not be able
to settle a decision record by being green, and because the alternative —
writing the ADR as a side effect of the measurement that motivated it — is the
one move `CLAUDE.md` and the runbook both forbid.
