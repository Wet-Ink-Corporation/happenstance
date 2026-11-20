# #487 — SY-34: a wrong runner that rolls back on a refused push

Branch `claude/project-thread-qkv7ws-487` off `origin/phase-21-prep` at `e314918`. Open point C
of [ADR-0108](../../.kb/decisions/0108-phase-14s-clauses-are-frozen.md).

## Goal

SY-34's rule assertion — the local log after a refused push equals the log before it — is shown
to reject a runner that rolls its log back on refusal. Today `spec/SPECIFICATION.md:8557` says
so by argument only.

## Constraints and non-goals

- Test-only. No change under `src/`, no public API, no new dependency, no H-16 decision.
- The clause's meaning does not change; the spec edit repairs its evidence line.
- Non-goal: running the watermark, pending or class assertions against `MiniEdge` — those read
  `Edge::status()`, which a runner over the public API alone does not have.

## Design

- `tests/runner/main.rs`: factor the log check of
  `a_refused_push_keeps_the_log_and_the_watermark_and_records_the_class` into
  `fn the_log_is_kept(before: &[SequencedEvent], after: &[SequencedEvent]) -> Result<(), String>`,
  which the test asserts `Ok(())` on (same comparison, same message). No duplication.
- `tests/runner/wrong_runners.rs`:
  - `Defect::RollsBackOnRefusal`: when the far side refuses the push, the runner removes the
    own-origin events it offered (`MemoryIngestStore::remove`, the public removal), rolling the
    spoke back to the last acknowledged position — the tidy wrong implementation SY-34's
    *Rejects* paragraph names — then reports the refusal.
  - `far_refusing()`: a far `StoreSyncPeer::with_limits` with `max_batch_bytes = 0`, the same
    refusal the main.rs test provokes from `MemorySyncPeer`.
  - rule `a_refused_push_keeps_the_log(runner)`: two local events, the push is refused (a step
    that succeeds fails the rule), then `the_log_is_kept`.
  - added to `failing` and to `the_shipped_edge_passes_every_rule`.

## Test list

- [x] `the_shipped_edge_passes_every_rule` also passes `a_refused_push_keeps_the_log`.
- [x] `a_correct_mini_edge_passes_every_rule` still empty with the new rule in `failing`.
- [x] each existing defect still fails only its own rule.
- [x] red: the new rule run against `MiniEdge(RollsBackOnRefusal)` fails.
- [x] `a_refused_push_keeps_the_log_rejects_a_runner_that_rolls_back_on_refusal`.
- [x] spec line 8557 names the wrong runner and the test; `spec-trace` green.
- [x] CHANGELOG `Unreleased` line (test-only lines are kept there, cf. #500).

## Risks

- The shipped edge splits a refused batch before parking; it must still err and keep the log.
  Checked by the shipped-edge test.

## Progress

- Factored `the_log_is_kept` out of the main.rs SY-34 test; that test still asserts the same
  equality, now through `assert_eq!(the_log_is_kept(..), Ok(()))`.
- **Red run** (temporary test `red_487`, asserting the new rule `Ok(())` against
  `MiniEdge(Defect::RollsBackOnRefusal)`, then replaced by the reject-test):

  ```text
  thread 'wrong_runners::red_487' panicked at crates/happenstance-sync/tests/runner/wrong_runners.rs:437:5:
  assertion `left == right` failed
    left: Err("the local log changed on a refused push, from [EventId { store: StoreId([108, ..]),
          position: SequencePosition(1) }, EventId { store: StoreId([76, ..]), position:
          SequencePosition(2) }] to [EventId { store: StoreId([108, ..]), position: SequencePosition(1) }]")
   right: Ok(())
  ```

  The own event (HUB, position 2) was rolled back; the foreign claim it did not offer stayed.
- Green: `cargo test -p happenstance-sync --all-features --test runner` — 79 passed, each
  existing defect still failing only its own rule, the correct `MiniEdge` and the shipped `Edge`
  passing all five.
- spec-trace flagged the new backticked test name on SY-34's Rule line as unresolvable (it is
  outside the sync families); declared it `Elsewhere(".../wrong_runners.rs")` in
  `xtask/src/spec_trace/sync_family.rs` beside the main.rs entry (table 20 -> 21), and
  regenerated §7.1 with `spec-trace --write`. The helper's name was dropped from the sentence
  rather than declared, since it is not a rule.
