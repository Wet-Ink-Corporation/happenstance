# Phase 21 audit debt: four proposed ADRs (#185, #115, #145, #188)

## Goal
Write a proposed decision record for each of four audit-debt issues, so the owner can accept or
redirect each one in a single read. Land now only what is purely additive, and leave the exact
specification, rule and `spec_trace.rs` edits for after acceptance.

## Constraints and non-goals
- No clause is minted or marked `[FROZEN]`, no accepted atom is edited, and no normative text in
  `spec/SPECIFICATION.md` changes until the owner accepts.
- Additive work only on this branch: README lines, dated sections in open-question atoms, and one
  owning-phase box in the runbook.
- No Rust is written by this pass. Each ADR's "What acceptance changes" section is the brief for
  the implementer who comes after acceptance.
- `cargo xtask spec-trace`, `lints`, `lint-kb` (base `1b7c85d`) and `lint-constitution` stay green.

## Invariants
- `UNCLAIMED_PENDING_ADR` still has three entries, and `UNRESOLVABLE_RULE_NAMES` still declares
  ES-6, ES-29 and ES-31 `Scheduled`. Both change only on acceptance.
- Every `file:line` cited in the new atoms resolves at `5d27623`.

## Design (per issue)

### #185 to ADR-0111 (`.kb/decisions/0111-the-three-unclaimed-rules-are-claimed.md`)
- Read fault: mint ES-44 (the store: an `Err` item, never an end of stream) and CF-44 (the
  fixture's `READ_FAULT` promise, CF-39's twin). Both frozen on landing, on the real faults
  Postgres and Neon arm.
- Disjoint boundaries: ES-25 claims the rule. Since ADR-0077 the rule asserts only ES-25's *if and
  only if* under contention, plus the progress anchor ES-25's existing claim on
  `exactly_one_of_n_contenders_commits` already carries. The edit is a Rule line and one paragraph,
  which is a repair.
- Model family: mint CF-45, a suite clause. It obliges the testkit to carry a generative family,
  so it obliges no store and does not depend on `PROPTEST_CASES`.
- `UNCLAIMED_PENDING_ADR` goes to `[(&str, &str); 0]` and is kept.
- Alternatives considered: widening ES-2 (frozen, about signatures); a separate independence
  clause (two sentences for one proposition); a store clause for the model (true or false by
  environment); a `family-claimed` disposition in check 6 (weakens CF-24).

### #115 to ADR-0112 (`.kb/decisions/0112-foreign-rollback-stays-non-normative-for-1-0.md`)
- Declined for 1.0 and renewed. Phase 18's rule would be decorative: under PS-5 a do-nothing
  rollback is the correct one. The record names the rule that would not be decorative (an open
  batch on B, then a foreign rollback, then B's commit lands) and its trigger: an adapter that
  stages writes on the server. Firing that trigger is additive, so ADR-0066 §2 permits the renewal.

### #145 to ADR-0113 (`.kb/decisions/0113-a-declined-capability-is-printed-and-listed.md`)
- Residual 1 is not accepted. `RuleOutcome::report` writes through a locked `io::stderr()` handle.
  That rests on the claim that libtest captures only the `print!` family, which is unverified here:
  the temper hook refused a scratch `.rs` file, and the toolchain has no `rust-src`. Verification
  gates the change, and the fallback is to accept the gap.
- Residual 3: each published adapter's README carries a table of declined capabilities. **Landed**
  for sqlite, postgres, neon and cloudflare. Cloudflare's README had falsely said its fixture
  declines nothing.

### #188 to ADR-0114 (`.kb/decisions/0114-es-6-es-29-and-es-31-get-their-rules.md`)
- ES-6: a `happenstance-core` test bound by a locally declared ADR-0009 marker, which spawns
  `append` and returns the error through the `JoinHandle`. Its negative is a `compile_fail,E0277`
  doctest in `send_shape.rs`.
- ES-31: a test named `checkpoint_lag_is_not_a_position_difference` in
  `crates/happenstance/tests/projection_runner.rs`, over a stride-7 `GappyMemoryStore`.
- ES-29: the shared rule with SY-6, which needs SY-6's refusal built. Nothing refuses a guard that
  carries `after` today. The fork is the owner's.
- Owning phase: 20. **Landed** as a box under "Work carried in from phase 21's audit".

## Test list (for the implementer, after acceptance, in this order)
- [ ] ES-31: `checkpoint_lag_is_not_a_position_difference`. Red: a compile error against the
      missing test name in `spec_trace`'s `Scheduled` entry going stale. Green: the test, then
      `Elsewhere`.
- [ ] ES-6, positive: `store_error_crosses_a_join_handle` compiles and runs, spawning `append`
      under the marker.
- [ ] ES-6, negative: a `compile_fail,E0277` doctest. The marker refuses `SendStoreWithLocalError`.
      Mutation: remove `Error: Send + Sync` from the marker and watch the doctest fail.
- [ ] ADR-0111: `NoopReadFaultFixture` fails only the read-fault rule. Then the clauses land and
      the three entries leave `UNCLAIMED_PENDING_ADR`, and `spec-trace` prints no unclaimed rule.
- [ ] ADR-0113, verification first: a default `cargo test -p happenstance-sqlite --test
      conformance` shows no `SKIP` line (red). After the change it shows them (green). A unit test
      checks `report_to(&mut Vec<u8>)`.
- [ ] ES-29, under the chosen fork: `wire_condition_with_after_is_refused` fails against
      `EvaluatesForeignAfterStore`, and under (i) against `KeepsPositionRelativeGuardStore` too.
      Then the four adapters and `MemoryIngestStore` pass.

## Decisions (H-16)
| Decision | Options | Recommendation | Decided by | Outcome |
|---|---|---|---|---|
| #185: freeze ES-44, CF-44 and CF-45 on landing | frozen on landing / provisional with a ledger row | frozen on landing | owner | pending |
| #185: disjoint rule | ES-25 repair / new clause | ES-25 repair | owner | pending |
| #115: foreign rollback | decline and renew / mint now / never | decline and renew | owner | pending |
| #145: residual 1 | stderr handle / accept gap / narrow CF-18 | stderr handle, falling back to accept | owner | pending |
| #145: residual 3 | README table convention / clause / lint / none | README convention (landed) | owner | pending |
| #188: ES-6 shape | core test plus probe / public testkit marker group / demote | core test plus probe | owner | pending |
| #188: SY-6 refusal | (i) drop the guard and land the events / (ii) `IngestError` arm, `Stop` | (i) | owner | pending |
| #188: owning phase | 20 / 21 | 20 | owner | pending |

## Risks
- The libtest capture claim is unverified (see #145), and Decision 2 of ADR-0113 gates on it.
- Branch (i) of the SY-6 fork narrows SY-30's "same guards" to position-free guards. ADR-0114
  calls that a prose qualifier, and the owner may judge it an amendment.
- The progress anchor in the disjoint and exactly-one rules is unstated. ADR-0111 records it and
  does not settle it.
- The trailing model's monotone-visibility assumption stays unclaimed. CF-5 exposure is unchanged.

## Progress
- 2026-11-18: four atoms written with `status: proposed`; decision-map section added; five
  open-question atoms given dated sections; README tables landed; phase-20 box landed for #188.
- Gate: see the commit message for the four `xtask` checks run.
