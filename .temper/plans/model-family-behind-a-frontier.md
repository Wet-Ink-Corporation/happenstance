# The model family behind a visibility frontier (#79, IMPL-7)

## Goal

`ops_agree_with_the_model` **runs** instead of skipping on a fixture that declines
`READ_YOUR_OWN_WRITES` (Postgres, Neon). It checks a weaker property that holds
whatever the server-wide `pg_snapshot_xmin` lag is. It reads no clock and never
waits (CF-33), and named wrong implementations in the testkit's `tests/` fail it.

## What declining RYOW actually permits (the oracle the property is built on)

The sources are the `Fixture::READ_YOUR_OWN_WRITES` rustdoc (`crates/happenstance-testkit/src/contract.rs:244-301`),
ES-10 (`spec/SPECIFICATION.md:3040`, FROZEN), ES-11 (`:3150`, FROZEN), ES-19
(`:3787`, FROZEN), ES-25 (`:4049`), ADR-0024 and ADR-0095, and the two adapters'
`evaluate` docs (`crates/happenstance-postgres/src/event_store.rs:941-951`,
`crates/happenstance-neon/src/event_store.rs:458-461`).

Under the model's own driving (one handle, strictly sequential, each op awaited):

- **Held = accepted.** `Ok(P)` is a durability promise. Every accepted batch is held
  from the moment `append` returns.
- **Visible is a position prefix of held.** ES-10 says position order is visibility
  order. The model has one sequential writer, so `xact_id` and position both rise with
  append order, and `xact_id < xmin` admits a prefix of the accepted log.
- **The frontier lags by an unbounded time and never moves backward.** It never moves
  backward because `pg_snapshot_xmin` of a fresh snapshot on one primary does not
  decrease. **No clause states this** (see D1).
- **Nothing is reused.** Positions are unique and strictly increasing (VT-11). The
  batch-last position equals `Ok(P)` (ES-19). An event keeps the position it was first
  seen at.
- **Conditions are judged over held rows, not visible ones** (ES-25, ADR-0095
  reading; both adapters' `evaluate` carry no frontier predicate). So the verdict
  depends on rows the model may not have seen yet. The model knows *that* they exist,
  and their content and order, from `expected`. It does not always know their
  *positions*.
- **A read is one snapshot** (ES-11). Its frontier lies between the frontier of the
  full read before it and the frontier of the full read after it.

Contract.rs:286-298 currently argues that *"no weakening of the comparison supplies"*
an oracle for the condition verdict. That is true of a prefix-only weakening. It is
false once the model predicts over `expected` (held) with **interval positions**
and **defers** the verdicts it cannot decide yet. Correcting that paragraph is part of
this change.

## The weaker property ("trailing agreement"), precisely

The fixture declines RYOW, so the rule runs `TrailingModel` instead of `Model`.
Every check is decided from data already in hand.

- **T1 (log).** Every full `read(Query::all())`:
  - (a) is a prefix, by content and order, of `expected`;
  - (b) is no shorter than the previous full read (monotone, see D1);
  - (c) has strictly increasing positions;
  - (d) agrees on position with every event already seen;
  - (e) carries, at every visible batch-last index, the position `append` returned for
    that batch;
  - (f) has its highest visible position below every returned position of a batch that
    is still invisible.
- **T2 (reads).** Let `k_before` and `k_after` be the visible lengths before and after
  the read (a full read is issued after each `Op::Read`). The read result `R` must
  equal `select(known_after[..k], options)` for **some** `k` in
  `k_before..=k_after`. Anchors are resolved once against `known_before` and handed to
  both sides, as today.
- **T3 (verdicts).** Each held event gets an exact position (visible, or a batch-last
  returned by `append`) or an open interval `(above, below)` taken from its nearest
  exact neighbours. A violation is monotone non-decreasing in position. The model
  probes `is_violated_by` at the interval's low end (`above.next()`, or `FIRST`) and
  at its high end (`below`, or `CEILING`), which gives a three-valued prediction:
  - `Rejected`: the condition is definitely violated. The store must refuse with
    `ConditionViolated`.
  - `Accepted`: no held event can violate it. The store must accept.
  - `Undecided`: either verdict is legal now. It is recorded as a `Deferred` and
    re-judged at every later reconcile, once more positions are known. A mismatch then
    fails, naming the original op.
  - Verdicts still unresolved when the sequence ends are left unjudged. That residue
    is the only thing the weakening gives up on conditions.
- **Zero lag collapses to strict.** With no lag, `k_before == k_after == len` and
  nothing is `Undecided`. T1 to T3 then reduce to today's checks, except that a
  **lost tail write** (a write that never appears) reads as lag. Without waiting it
  cannot be told apart from lag, and test 15 pins exactly what that costs.

`CEILING` is `match SequencePosition::new(u64::MAX) { Some(p) => p, None => SequencePosition::FIRST }`
in a `const`. The dead arm is total and never taken, so this needs no panic and no
core API change.

## Constraints and non-goals

- No change to `happenstance-core`, no new dependency, and no new `pub` item. `Op`,
  `Anchor`, `RuleOutcome`, the macros and emitters stay as they are, so semver is
  untouched.
- No clock, sleep, retry or op-count assertion (CF-33). Nothing holds the Postgres
  frontier back on purpose, for the reason given at `crates/happenstance-postgres/tests/support/mod.rs:619-632`.
- **Non-goal:** gating the *named* event-store rules on RYOW. They are not gated
  today (`suite.rs` has no `require!(F: READ_YOUR_OWN_WRITES)`), and Postgres and Neon
  pass them only because the server is quiet under `--test-threads=1`. This is a real
  sibling of IMPL-7. File it as an intake note or issue. It is not in scope here.
- **Non-goal:** liveness ("an accepted write eventually appears"). It needs waiting,
  which CF-33 forbids.

## Invariants

- On a fixture that supports RYOW the rule's behaviour is byte-for-byte today's.
  `MODEL_COVERAGE` and its heading counts do not move.
- On a declining fixture the rule returns `RuleOutcome::Ran` or panics. It never
  returns `Skipped`.
- The trailing model never fails a store that meets the facts above, at any lag
  schedule. `TrailingFrontierStore<Settling>` is the control.
- At zero lag the trailing model rejects a subset of what the strict model rejects.

## Design

- **Placement.** Everything goes in `happenstance-testkit`, which is the conformance
  crate, so dependencies still point inward (ARC-02).
  - New private module `crates/happenstance-testkit/src/model/trailing.rs`, in the
    `foo.rs` + `foo/bar.rs` layout (ARC-06). `model.rs` is already 924 lines.
  - `model.rs` keeps `Model` and gains a `mod trailing;`.
  - `resolve` and `select` move to free functions over `&[SequencedEvent]`, so both
    models share one copy (CRF-05).
- **Dispatch** (API-09, a closed set gives an enum). This is private:
  ```rust
  enum Checker { Strict(Model), Trailing(TrailingModel) }
  impl Checker {
      fn for_fixture<F: Fixture>() -> Self;   // F::READ_YOUR_OWN_WRITES.is_supported()
      async fn apply<S: EventStore>(&mut self, store: &S, index: usize, op: &Op) -> Result<(), String>;
  }
  ```
  - `replay` builds `Checker::for_fixture::<F>()` instead of `Model::default()`.
  - The early `Skipped` return at `model.rs:693-698` is deleted.
  - `pub async fn ops_agree_with_the_model` keeps its signature and stays at line 686
    if possible. Spec §6.4 cites `model.rs:686` and `:800`, so if they move, repoint
    them; a repoint is not a normative edit.
- **Types in `trailing.rs`.** All are private, and `Event`/`SequencedEvent` are owned
  because the model stores them (OWN-02):
  ```rust
  pub(super) struct TrailingModel {
      expected: Vec<Event>,                       // held: every accepted event, append order
      known: Vec<SequencedEvent>,                 // visible prefix, as last read; len <= expected.len()
      returned: Vec<(usize, SequencePosition)>,   // (index of batch-last in `expected`, Ok(P)); both strictly ascending
      deferred: Vec<Deferred>,
  }
  struct Deferred { op: usize, condition: AppendCondition, held: usize, accepted: bool }
  enum Prediction { Rejected, Accepted, Undecided }        // not bool + Option (API-10)
  enum Placed { At(SequencePosition), Between { above: Option<SequencePosition>, below: Option<SequencePosition> } }
  fn placed(&self, index: usize) -> Placed;               // exact, or bounds from nearest exact neighbours
  fn predict(&self, condition: &AppendCondition, held: usize) -> Prediction;
  async fn reconcile<S: EventStore>(&mut self, store: &S) -> Result<(), String>;   // T1, then re-judge `deferred`
  pub(super) async fn apply<S: EventStore>(&mut self, store: &S, index: usize, op: &Op) -> Result<(), String>;
  ```
  - `Op::AppendConditional` resolves the anchor against `known`, then calls
    `predict(.., expected.len())`, then calls the store. It matches
    `(Prediction, Result)`:
    - `Undecided` with `Ok`: extend `expected`, record `returned`, push a `Deferred`.
    - `Undecided` with `ConditionViolated`: push a `Deferred`.
    - `Rejected` with `Ok`, or `Accepted` with `ConditionViolated`: fail now, as
      today.
    - Any other `Err`: fail.
    - The `conflicting_position` value is ignored, for ES-25's reason at
      `model.rs:408-432`.
  - Bounds: `above` is `None` means there is no lower bound, so the low probe is
    `FIRST`. `above.next()` being `None` would mean the store returned `u64::MAX` and
    then held more. Treat that as `Undecided`, which is always sound, rather than as a
    panic.
  - Slicing goes through `.get(..k)` (H-04).
- **Errors.** Errors stay `Result<(), String>`, which matches the file. They are
  private, never in a `pub` signature, and become the rule's panic message, so H-09
  does not apply. Every message names the op, the visible length and the held length.
- **Concurrency and resources.** None new. The model is sequential, with one handle
  per replay. Each `Op::Read` costs one extra full read. `known` and `deferred` are
  bounded by `MAX_OPS * 3` events.
- **Alternatives considered.**
  - **Prefix-only weakening.** It leaves no oracle for verdicts, which contract.rs is
    right about.
  - **Compare only verdict-free ops.** That drops the conditional half, which is where
    the frontier-filtered-condition bug lives.
  - **Wait for the frontier, or poll `head`.** CF-33 forbids it.
  - **A second rule.** See D2.

### Testkit instruments (tests/, the falsifiers)

These go in `tests/mutation_coverage/variants.rs` (the control) and `mutants.rs` (the
mutants), after the `FrontierStore`/`Membership` pattern: the stores differ in one
function.

- **`TrailingLog`** holds `events` (held) and `reads: Vec<usize>` (the held count at
  each read call).
- **The schedule.** Read call *n* sees the prefix of length `reads[n - n % 3]`. That
  gives a lag of zero, one or two reads, cycling. It is deterministic and monotone,
  and it produces every case: invisible held violators, `Undecided` verdicts at
  `BeyondHead`, and reads bracketed across a frontier move.
- **`trait Trailing`** has `const NAME`, `fn shown(log, n) -> Vec<SequencedEvent>`
  (default: the scheduled prefix) and
  `fn conflict(log, &AppendCondition) -> Option<SequencePosition>` (default: the first
  held violator).
- **`TrailingFrontierFixture<T>`**:
  - `READ_YOUR_OWN_WRITES` is declined, in its own words.
  - `HELD_ABOVE_FRONTIER` is declined, in its own words. The text must not equal
    `UNSTATED_*`.
  - `SECOND_HANDLE` is supported. `REOPEN` is declined.
- **Named wrong implementations.** Provenance is given for each, as REGISTRY rows
  carry it.

| Store | One-function defect | Plausible because | Caught by |
|---|---|---|---|
| `TrailingFrontierStore` (`Settling`) | none (the control) | the shape of Postgres and Neon | must be **Agreed** |
| `FrontierFilteredConditionStore` | `conflict` scans `shown` rather than held | the condition SQL reuses the read path's `xact_id < pg_snapshot_xmin(...)` predicate. This is the defect `evaluate`'s "No frontier predicate here" note warns of, and a frontier store can do it silently | T3 `Rejected` with `Ok` |
| `HoleyFrontierStore` | `shown` hides the oldest invisible row while showing newer ones | the `happenstance_naive_arm` (`Visibility::Naive`): `nextval()` outside the transaction, a later writer commits first | T1(a) non-prefix |
| `RetreatingFrontierStore` | the schedule index is `n - 2 * (n % 2)`, which goes backward | a frontier read off a replica or a pooler that hops between servers | T1(b). **Only if D1 = A** |
| `TrailingAfterIsInclusiveStore` | `conflict` treats `after` as inclusive | `AfterIsInclusiveStore`, behind lag | T3, including via a `Deferred` |

- These are **not** in `REGISTRY` or `for_each_mutant!`. A lagging store fails the
  named rules that read their own writes, which is the non-goal above, and
  `mutant_registry_is_exhaustive` would demand rows for them. Instead they get:
  - a new `for_each_trailing_subject!` macro;
  - a `TRAILING_MODEL_COVERAGE: &[(&str, ModelOutcome, &str /* provenance */)]` table;
  - a meta-test that every enumerated subject has exactly one row and the reverse;
  - a test that each row's claimed outcome is the observed one, through the existing
    `harness::model_probes` and `run_probe`.
- **The weakening, measured.** `Declined<S>` is a wrapper fixture over every
  `for_each_mutant!` subject. It delegates `Store`, `connect`, `SECOND_HANDLE` and
  `REOPEN`, declines `READ_YOUR_OWN_WRITES` and states `HELD_ABOVE_FRONTIER`. Two
  checks run over it:
  - weak-`Rejected` is a subset of strict-`Rejected` (soundness, because strict
    implies weak);
  - strict-`Rejected` minus weak-`Rejected` equals a pinned `WEAKENING_COSTS: &[&str]`,
    with one line of reason each. The expected members are the lost-tail-write shapes.
    The list is to be measured, not guessed.

## Mounting

- **Postgres.** It is already mounted (`crates/happenstance-postgres/tests/postgres_conformance.rs:144`,
  ignored emitter). CI `live-postgres` already runs `--test postgres_conformance --
  --ignored --show-output` (`ci.yml:616`).
  - **Add one step:** `grep` that `run.txt` has
    `dcb_model_conformance::ops_agree_with_the_model ... ok` and **no**
    `SKIP ops_agree_with_the_model` line. Without it, the step would pass a reverted
    skip silently, because a skip is a pass.
  - **Local run:** `cargo test -p happenstance-postgres --all-features --test postgres_conformance -- --ignored dcb_model_conformance --show-output`.
    The fixture uses testcontainers with an image pinned to `17.10`, so the local PG16
    binaries cannot serve it without a fixture change, which is out of scope. Run it
    where Docker is up. Otherwise CI is the record.
- **Neon.** It is already mounted (`crates/happenstance-neon/tests/neon_conformance.rs:154`).
  It runs **only** in CI `live-neon` (`ci.yml:1166`). Add the same grep step on
  `run-events.txt`.
  - **Runtime risk:** each op is 80-100 ms over HTTP and each replay opens a fresh
    fixture. Measure it on the PR's run against the job's 30-minute timeout (see D4).
- **Docs to correct in the same change** (PROC-11):
  - `contract.rs:244-301` (the "no weakening supplies an oracle" paragraph, rewritten
    to name the trailing mode);
  - `crates/happenstance-postgres/src/event_store.rs:8-15` and `README.md:30`;
  - the Postgres fixture doc `tests/support/mod.rs:439-452`;
  - `crates/happenstance-neon/src/lib.rs:26` and `README.md:165`;
  - `crates/happenstance-testkit/README.md:39`;
  - `docs/pass-conformance-suite.md:104`;
  - the stale "No model rule is capability-gated" comment at
    `tests/mutation_coverage.rs:2766`;
  - the CI comments "1 model" (unchanged count, now a run rather than a skip);
  - a `CHANGELOG.md` `[Unreleased]` entry (additive);
  - an update to `.kb/open-questions/model-family-rule-has-no-clause.md`. It is not a
    decision atom, so it is editable. The composed set grows by ES-10 and ES-19, and
    it gains the trailing mode's one unstated assumption.

## Clause question

**No clause change and no census change.** `ops_agree_with_the_model` is
already unclaimed. It is listed in `UNCLAIMED_PENDING_ADR` (`xtask/src/spec_trace.rs:2945`),
and that list "can only shrink". T1 to T3 compose ES-10, ES-11, ES-19 and ES-25,
which are all existing clauses.

The single proposition no clause states is **monotone visibility** (T1(b), and the
lower end of the T2 bracket). Writing it into ES-10 is an edit to a **FROZEN**
clause. That makes it **one-way** and needs a new ADR, so it is **not** proposed here
(see D1).

## Test list (drive in this order)

- [x] 1. Refactor `resolve` and `select` to free functions. `memory_model_conformance`
      and `MODEL_COVERAGE` stay green and unchanged (strict mode, no behaviour change).
- [x] 2. `predict` returns `Rejected` when a visible event violates the condition
      (unit, `#[cfg(test)]` in `trailing.rs`).
- [x] 3. `predict` returns `Rejected` when an invisible held event matches and the
      anchor is at or below the visible head.
- [x] 4. `predict` returns `Undecided` when the only candidate is invisible with an
      unknown position and the anchor is `BeyondHead`.
- [x] 5. `predict` returns `Accepted` when no held event matches the query.
- [x] 6. `predict` decides case 4 once the candidate is a batch-last with a returned
      position.
- [x] 7. `reconcile`:
  - accepts a strict prefix;
  - rejects a non-prefix;
  - rejects shrinkage (D1 = A);
  - rejects a position that moved;
  - rejects a visible batch-last whose position is not `Ok(P)`;
  - rejects a visible position at or above a still-invisible returned position.
- [x] 8. The read bracket accepts a result explained by an intermediate `k` and
      rejects one explained by no `k`.
- [x] 9. A `Deferred` verdict is re-judged when positions arrive, and a mismatch
      names the original op.
- [x] 10. The rule on a declining fixture returns `Ran`, not `Skipped`, against
      `TrailingFrontierStore<Settling>` (the false-failure guard, `Agreed`).
- [x] 11. `FrontierFilteredConditionStore` is `Rejected`.
- [x] 12. `HoleyFrontierStore` is `Rejected`.
- [x] 13. `RetreatingFrontierStore` is `Rejected` (D1 = A only).
- [x] 14. `TrailingAfterIsInclusiveStore` is `Rejected`.
- [x] 15. `Declined<S>` sweep: soundness (weak is a subset of strict), and pin
      `WEAKENING_COSTS`.
- [x] 16. `frontier_fixture_declension`, `capability_skips_are_reported` and the
      `MODEL_COVERAGE` heading counts are still green.
- [ ] 17. Live run against Postgres (local where Docker is up, plus CI `live-postgres`).
      The model rule is `ok` with no SKIP line. Record the case count and wall time.
- [ ] 18. Live run against Neon in CI `live-neon`, with the same check. Record the wall
      time.
- [x] 19. Run `cargo xtask spec-trace`, `lint-kb` and the full gate.

## Decisions (H-16)

| # | Decision | Options | Recommendation | Decided by | Outcome |
|---|---|---|---|---|---|
| D1 | Monotone visibility: no clause states it, and the trailing model relies on it | **A** Assert it in the trailing model as a stated testkit assumption, with `RetreatingFrontierStore` as its mutant. Both frontier adapters meet it by construction, and strict mode already implies it. **B** Rely on it without asserting it: no T1(b) and no mutant. A retreating store then fails with a misleading read-mismatch message instead. **C** Make it normative. Either edit ES-10, which is FROZEN, **one-way** and needs an ADR, or mint a new PROVISIONAL clause, which is census +1 and its own ADR | **A** now, plus an `.kb/_intake/` note proposing C for an ADR pass | delegating agent, 2026-10-19 (two-way default, not a human approval) | **A**. T1(b) asserted as a stated testkit assumption; `RetreatingFrontierStore` is its mutant; `.kb/_intake/` note proposes C for an ADR pass |
| D2 | One rule with two modes, or a second rule | **A** `ops_agree_with_the_model` picks the strict or trailing model from the capability. The name, enumeration and harness counts are unchanged, and the issue's acceptance reads "the rule runs". **B** A new `ops_agree_with_the_model_behind_a_frontier` that runs on every store, while the old rule keeps skipping. This adds an unclaimed rule, but `UNCLAIMED_PENDING_ADR` can only shrink, so B forces a new clause and a census change | **A** | delegating agent, 2026-10-19 (two-way default) | **A**. One rule; mode picked from `READ_YOUR_OWN_WRITES` |
| D3 | Should the run say it ran weakened? | **A** No API change. The weakening is stated in rustdoc, the adapters' READMEs and CHANGELOG. **B** A new `RuleOutcome` variant. `RuleOutcome` is deliberately exhaustive, so this is a breaking change to a published crate. **C** A `println!` note from the rule, which is the SKIP line's precedent. It is invisible without `--show-output` | **A** | delegating agent, 2026-10-19 (two-way default) | **A**. No API change; disclosed in rustdoc, adapter READMEs and CHANGELOG |
| D4 | The Neon model run's cost in CI | **A** Keep the default `PROPTEST_CASES` (256) and measure on the PR's run. **B** Set a lower `PROPTEST_CASES` for `live-neon` only. The case cliff at 192 is a measured loss of power (`kb-reference-model-family-case-cliff-001`) | **A**, and revisit only if the run exceeds about 15 of the 30 minutes | delegating agent, 2026-10-19 (two-way default) | **A**. 256 cases kept |
| D5 | Rewriting `contract.rs`'s shipped rustdoc argument, which says the opposite of this change | Rewrite it in the same PR. No ADR says it, so it is doc, not decision. **Or** first record an atom saying why it was wrong | Rewrite it in the same PR, and cite this plan in the CHANGELOG | delegating agent, 2026-10-19 (two-way default) | Rewrite the `contract.rs` paragraph in the same PR |

## Risks

- **The monotonicity premise is false somewhere.** For example, a Neon pooler hops
  between computes, or a read replica sits behind the endpoint. It would then show as a
  T1(b) red on live CI, which is correct behaviour but an unexpected red. Mitigation:
  the message names "frontier went backward" explicitly, and D1 says what then reopens.
- **The trailing model is wrong in a way that false-fails.** The guard is the
  `Settling` control under a schedule with lag 0, 1 and 2, plus the zero-lag soundness
  sweep (test 15). A live Postgres or Neon red would be the expensive version, and
  tests 10 and 15 come first for that reason.
- **The power loss hides a regression.** `WEAKENING_COSTS` is pinned, so a store
  moving from caught to missed changes the list, and the list is reviewed.
- **`mutation_coverage` runtime roughly doubles under `proptest`** because of the
  `Declined<S>` sweep. Record it. If it hurts, restrict the sweep to the strict-Rejected
  rows. That is a test-binary cost, not a user-facing one.
- **The spec §6.4 line citations into `model.rs` go stale.** `spec-trace` checks only
  that a cited file is long enough. Re-point the citations by hand.
- **Named rules on frontier stores pass only on a quiet server.** That is the non-goal.
  File it so it is not mistaken for something this change covers.

## Size (lower bound)

About 12 files and about 1,000 lines:

| Change | Lines |
|---|---|
| `src/model/trailing.rs` | about 400, including unit tests |
| `src/model.rs` | +40 / -15 |
| `tests/mutation_coverage/variants.rs` | +130 |
| `tests/mutation_coverage/mutants.rs` | +120 |
| `tests/mutation_coverage.rs` | +170 |
| `ci.yml` | +20 (two steps) |
| Docs (8 files) | about 80 |
| CHANGELOG, `.kb` atom, intake note | about 60 |

## Progress

- 2026-10-19: plan written (architect). Nothing implemented.
- 2026-10-19: decisions D1–D5 recorded as the delegating agent's two-way
  defaults (A, A, A, A, rewrite in this PR). Not human approvals.
- Test 1 (refactor): `memory_model_conformance` 2/2 and `mutation_coverage -- model`
  4/4 green before and after; no expectation touched.
- Tests 2–6, red: `cargo test -p happenstance-testkit --lib trailing` failed with
  `error[E0599]: no method named predict found for struct TrailingModel` (6×).
  Green after `predict` and `range`.
- Test 7, red: `error[E0599]: no method named check_log` (9×). Green after
  `check_log`/`check_returned`.
- Tests 8–9 were written in the same edit as `check_read`/`rejudge`; their red
  was the compile error for the missing methods, not separately run. 16/16 unit
  tests green.
- Test 10, red: `the_trailing_model_rejects_exactly_what_it_claims` panicked with
  "ran `TrailingFrontierStore` into something that is not its own assertion.
  Saw: skipped (`READ_YOUR_OWN_WRITES` declined: ...)"; test 15 the same for
  `GappedPositionStore`. Green after deleting the skip in the rule.
- Tests 11–14: each wrong store rejected by the model's own assertion. Shrunk
  failures: FrontierFiltered — "accepted at 4, but a held event violates" (4 ops);
  Holey — "3 is visible while the batch `append` returned 3 for is not" (T1(f),
  2 ops; the plan predicted T1(a)); Retreating — "the frontier went backward"
  (3 ops); TrailingAfterIsInclusive — "rejected ... but no held event can violate"
  (2 ops).
- Added beyond the plan: `StalledFrontierStore`, a measured `Agreed` row pinning
  the liveness non-goal.
- Test 15: `WEAKENING_COSTS` measured **empty**: at zero lag the trailing model
  rejects every store the strict one rejects, and nothing the strict one agrees
  with. `the_model_rule_rejects_exactly_what_it_claims` alone took 3.1 s before;
  the three new trailing tests together take about 3 s more, and the whole
  `mutation_coverage` binary runs in about 6 s.
- Placement deviation: the instruments live in `tests/mutation_coverage/trailing.rs`,
  declared from `variants.rs` behind `proptest` (the `metadata_ceilings` precedent),
  not split across `variants.rs`/`mutants.rs`. A new `#[path]` is refused by the
  hook, and items driven only under `proptest` are dead code without it.
- Docs: the no-longer-caught list lives on `ops_agree_with_the_model`, not on
  `Fixture::READ_YOUR_OWN_WRITES`, so `contract.rs` keeps its line count
  (standards/rust cites it by line). Neon `lib.rs:26` left as it was: it never
  claimed a skip, and adding lines would break an accepted decision's citation.
- Tests 17–18: not run. Docker is unavailable on this machine
  (`/var/run/docker.sock` absent), and Neon runs only in CI. CI is the record.

### Review round 1 (F1–F9)

- F1: T1(b) is `TrailingModel::check_never_retreats`, called once from
  `check_log`. "Assume only" is deleting that call and that function, plus
  `RetreatingFrontierStore`'s row and the unit test that names it.
- F6, F4 (unit half): tests moved to `src/model/trailing/tests.rs`. The new
  ones (`accept`'s floor, `rejudge`'s contradicted acceptance and refusal, an
  empty `range`, an unplaceable event) characterise code already written and
  passed on first run; their red is not separately on record.
- F2, red: `DrainingFrontierStore` and `GappedTrailingFrontierStore` first
  failed to compile as dead code until enumerated. The first `Draining`
  schedule (one batch per read) never built lag, because every op ends in a full
  read: with `check_read` narrowed to the bracket's two endpoints, the whole
  trailing table still passed. The schedule now stands still on a read that
  follows a write and drains one batch per quiet read; with the same narrowing,
  `the_trailing_model_rejects_exactly_what_it_claims` failed on
  `DrainingFrontierStore` alone ("claims Agreed and the trailing model answered
  Rejected"). Narrowing reverted. `pinned::a_draining_frontier_puts_a_read_strictly_inside_its_bracket`
  and `pinned::a_refused_append_burns_a_gapped_position` hold the schedule and
  the burn directly.
- F4: `TRAILING_MODEL_COVERAGE` carries the substring each verdict must
  contain, asserted in `the_trailing_model_rejects_exactly_what_it_claims`.
- F8: not done. `spec_trace::RULE_FILES` lists files that define a rule (a
  `pub async fn`), and `lints::true_rule_counts` bails on a member that parses
  no rule, so adding `model/trailing.rs` fails `cargo xtask lints`. CF-6's
  position-literal scan therefore does not cover it; a follow-up could give that
  scan its own file list.
- F3/F7/F9 done; rebased onto `origin/main` at `d2a01f95`. The rebase conflicted
  only on the two `standards/rust` ci.yml citations, repointed to `:1890` and
  `:1945`.
