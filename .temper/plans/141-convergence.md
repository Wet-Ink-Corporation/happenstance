# 141 · SY-20 and SY-22: convergence and cost-layers rules

## Goal
`convergent_projection_is_interleaving_independent` (SY-20) and
`non_convergent_projections_are_excluded_not_failed` (SY-22) in
`happenstance-sync-testkit`, against `happenstance::ConvergentProjection`.
Markers stay PROVISIONAL; the freeze edits are reported, not made.

## Design
- New dep: `happenstance = { workspace = true, features = ["postcard"] }` (owner-approved;
  `postcard` is already in the testkit's graph, so the only new node is `happenstance`).
  No cycle: `happenstance` depends on no sync crate.
- `src/convergence_fixture.rs`: `pub trait ConvergenceFixture` — `type Projection: Projection`
  (no marker bound: SY-22's exclusion lives on the rule, not the trait), `type Codec`,
  `projection()`, `codec()`, `authored() -> [Vec<Event>; 2]`, `store()`, `read_model() -> Vec<u8>`.
  Reference fixtures over one stock domain: `StockOnHandFixture` (sums, declared convergent),
  `CostLayersFixture` (FIFO, undeclared) — the honest SY-22 case.
- `src/convergence_suite.rs`: `pub mod convergence_rules`. A private core, unbounded on the
  marker, replays both origins one-first and two-first into `MemoryEventStore::restore` logs
  (origin identities kept, one `RecordedAt` for all) and runs the fixture's projection over
  each; the SY-20 rule bounds `F::Projection: ConvergentProjection` and compares bytes. The
  SY-22 rule runs both reference fixtures through the core: stock-on-hand must agree,
  cost-layers must differ; a `compile_fail` fence beside a compiling control pins the bound.
- Degenerate cases are violations, not passes: an origin with no events; a run that applied
  fewer events than it was handed.
- spec-trace: add the suite file to `RULE_FILES` (7 -> 8); VT-9's `Scheduled` entry for the
  SY-20 rule goes stale and is removed (69 -> 68).

## Tests
- [x] `tests/convergence_conformance.rs`: both rules green on the reference fixtures.
- [x] `tests/wrong_projections.rs`: unit-ledger (last arrival wins per serial), arrival rank,
      first arrival wins, a one-origin case, an out-of-scope case -> SY-20 violations;
      `LatestByEventId` passes.
- [x] Mutant of the core (one interleaving twice): SY-22 and every wrong projection go red.
- [x] Mutant of the rule's bound removed: the `compile_fail` fence goes red.

## Progress
- Red: `convergence_conformance` did not compile, E0432 unresolved
  `StockOnHandFixture`, `convergence_rules` (API not yet written).
- Core mutant (two_first replays one-first): SY-22 rule red ("cost layers committed
  \"cost_of_goods_issued=24\n\" under both interleavings"); wrong_projections 4 of 7 red
  (unit_ledger, first_arrival_wins, arrival_rank, declared cost layers).
- Guard mutants (empty-origin and applied-count checks disabled): the two refusal tests red.
- Bound mutant (`F::Projection: Sized` for the marker): the compile_fail fence red,
  "Test compiled successfully, but it's marked `compile_fail`".
- CF-24 lint red until `for_each_convergence_rule!` and its ENUMERATIONS row landed.
- Finding: SY-20's "disagree everywhere else" admits exactly one pair (the two block
  orders), so a fold sensitive to the alternation pattern passes. Reported, not legislated.
