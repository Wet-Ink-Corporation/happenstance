---
item: "HS-S0041"
stage: report
created: "2026-08-17"
updated: "2026-08-17"
---

# Report — The concurrency family green, and the 8-versus-64 discrepancy closed

## Findings Ledger

**Seven of seven ACs satisfied. Nothing blocked, nothing deferred.** The racing
family runs against a real SQLite file, all five rules report `Ran` and green, and
the number they run at is now the same number the repository's two stated proof
artefacts have claimed for three phases.

**Mount point:** `crates/happenstance-sqlite/tests/concurrency.rs` — a new
integration target carrying
`happenstance_testkit::event_store_concurrency_conformance!(SqliteFixture::new())`
under `#![cfg(all(feature = "event-store", not(target_arch = "wasm32")))]`.
Auto-discovered by `cargo test -p happenstance-sqlite`, so
`cargo xtask affected --base main` reaches it with no script edit.

**Second wiring point:** `crates/happenstance-sqlite/tests/support/mod.rs` — the
fixture relocated out of `tests/conformance.rs` so two targets can share it without
minting a third test binary. `tests/conformance.rs` is now a 24-line mount.

| AC | Result | What proves it |
| --- | --- | --- |
| **AC-001** — five rules, all `Ran` | **Met** | `9 passed; 0 failed; 0 ignored` on the new target: the five rules of `for_each_concurrency_rule!` by name plus four adapter-local tests. None absent, none `Skipped`; `SECOND_HANDLE` is `SUPPORTED` (`tests/support/mod.rs:135`), so nothing is aborted by the panic a declined MUST raises |
| **AC-002** — no contender ever sees `NoRuntime` | **Met, with the negative control run** | `concurrency.rs:96 store_serves_a_bare_thread_with_no_ambient_runtime` — append *and* a fully drained read from a `std::thread::scope` thread with `Handle::try_current().is_err()` asserted first. Its negative control was run and failed at `concurrency.rs:128`; see *The finding* |
| **AC-003** — a lost race is `Rejected`, not a contended driver | **Met** | `concurrency.rs:202 every_connection_carries_the_declared_busy_timeout` reads `PRAGMA busy_timeout` back off the **first, second and third** `connect()` of one fixture and asserts 5,000 ms on each; `declared > 0` asserted separately, so an unbounded handler is rejected as hard as an absent one. No `Attempt::Failed` appears anywhere in the run |
| **AC-004** — exactly one winner, k admits k, positions distinct, own last position | **Met** | The four racing rules green at 64. No literal position is asserted anywhere this story adds. Falsifiability from `mutation_coverage::the_concurrency_rules_reject_exactly_what_they_claim`, re-run green **at the settled `CONTENDERS`** |
| **AC-005** — the reader never sees a partial batch | **Met** | `a_concurrent_reader_never_sees_a_partial_batch` green, read together with the seam test green in the same binary — so every sighting is a genuine observation. The negative control is the standing proof that this rule *can* report the wrong thing |
| **AC-006** — the contender count becomes a decision | **Met, branch A** | `CONTENDERS` 8 → **64** at `crates/happenstance-testkit/src/concurrency.rs:238`, doc comment rewritten with the reason, ADR-0022 §12's measurement and the workspace-wide cost. **Verified rather than assumed**: `cargo test -p happenstance-testkit --all-features` green at 64 across every target |
| **AC-007** — nothing the previous story earned is spent | **Met** | `shapes` 10 passed, `conformance` 89 passed, `cargo xtask wasm` green, no third test binary — asserted out of the filesystem by `concurrency.rs:237`. `MID_BATCH_FAULT` still declined with HS-S0040's own sentence, byte-identical |

## The finding, which is why the seam test is a separate test

The family's first green run is not evidence that the seam works, and this is
demonstrable rather than arguable. With the captured runtime handle removed —
`src/event_store.rs:330`, `Handle::try_current().ok()` to `None`, reverted
immediately — the target reports:

```text
test dcb_concurrency_conformance::exactly_one_of_n_contenders_commits ... ok
test dcb_concurrency_conformance::k_disjoint_boundaries_admit_exactly_k_commits ... ok
test dcb_concurrency_conformance::positions_are_unique_under_concurrent_appends ... ok
test dcb_concurrency_conformance::append_returns_the_callers_own_last_position ... ok
test dcb_concurrency_conformance::a_concurrent_reader_never_sees_a_partial_batch ... FAILED
  a reader observed 128 batch(es) part-written: [ "a concurrent read failed: no
  tokio runtime is available to run the blocking SQLite query", ... x128 ]
```

Four rules green, and the fifth accusing the adapter of a **partial-batch
visibility violation (ES-18)** for a defect that is not about visibility at all.
`append` takes the connection mutex on the calling thread and never needs a
runtime; `read` defers a `spawn_blocking` into `poll_next` and does. A seam applied
to one of them therefore produces exactly this — a red verdict about the wrong
sentence, in a rule whose failure channel is a *sighting string*. An implementer
reading only that output would debug atomicity.

`store_serves_a_bare_thread_with_no_ambient_runtime` exists so the failure arrives
as itself. It is deliberately **outside** the generated module, in the same binary.

## What was decided, and what was declined

- **Branch A on `CONTENDERS`.** Raised, not amended-around. ADR-0022 §12 also made
  a *third* option visible — invoke this adapter's harness at 64 without moving the
  shared constant — and it was declined: the constant's own documentation calls it
  *the number an adapter author has to size a connection pool against*, and a
  per-adapter override makes that sentence false for every reader while leaving the
  discrepancy alive in a new form.
- **The blocking emitter is declined in writing**, in the target's module doc, with
  the mechanism: `__emit_concurrency_blocking` generates a plain `#[test]`, so no
  tokio runtime exists on any thread including the constructing one, ADR-0022 §9's
  captured handle would be `None`, and every rule would fail with `NoRuntime`. CF-23
  is satisfied for this family inside the testkit, by the harness that can honestly
  run both.
- **No clock was added anywhere.** No `#[timeout]`, no sleep, no retry loop around a
  rule. The wall-clock figures in the ledger are a human timing a command, which
  CF-33 does not bind; the adapter's own finite busy handler is not a rule reading a
  clock either, and `every_connection_carries_the_declared_busy_timeout` is what
  checks it stays finite.
- **`RUNBOOK.md`'s phase-3 and phase-7 session logs were left alone** and only the
  live phase-8 proof artefact corrected. Rewriting a log is not reconciliation.

## Deferred

Nothing. Two items in the spec's *Explicitly not in this PR* remain untouched by
design and belong to slice-mates: the model family and the `BEGIN DEFERRED`
provenance row (`model-family-and-mutant-pass-column`), and the reopen negative
control with the ES-35 / CF-17 / CF-14 verdicts
(`reopen-negative-control-and-durability-verdicts`).

**One thing a reviewer should read as an honest deviation rather than a gap:** the
runtime seam this story was written to *build* was already in the tree when it
started. What this story added is the mount, the decision, and the tests that make
the seam a checked claim instead of a field. `implementation-report.md` says so in
full under *Notes*.
