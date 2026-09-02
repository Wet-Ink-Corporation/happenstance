---
item: HS-P0023
stage: implementation
created: 2026-08-20T03:07:10.455Z
updated: 2026-08-20T03:07:10.455Z
template_sig: 4c5f37d6
rendered_sig: 18d02a6c
---

# Slice ledger — Reach and the Adapter Path

The review verdict of every vertical slice of this project, sealed as it closed.

A slice's stories commit BEFORE its adversarial review runs, so "committed" is not "approved".
This ledger is the second axis: it records which slices actually cleared review, so a re-launched
implement run re-reviews a rejected slice instead of walking past it as finished. Each row is sealed
by a commit carrying a `Slice-Verdict: <project-slug>/<slice> <verdict>` trailer — the ledger is the
human-readable record, the trailer is what resume greps.

## Verdicts

| Slice | Verdict | Story checkpoints | Sealed by |
| ----- | ------- | ----------------- | --------- |
| pointer-policy | approved | pointer-policy-and-inventory ef2eb6b | (this commit) |
| adapter-error-site | approved | adapter-reasoning-account af9a241, store-error-site-rewrite f2c7dbe | (this commit) |
| adapter-error-walk | changes-requested | error-site-walk-record 2ac7163 | (this commit) |

## Surviving findings

For each slice whose verdict is `changes-requested`, the findings that survived the in-slice fix
pass, with the `file:line` evidence the reviewer cited. These are the prescription a resumed run —
or a human — starts from. They are hypotheses for the next reviewer to verify, not facts to trust.

### adapter-error-walk

- HS-S0157 (error-site-walk-record): story gate red — boundary, provenance — boundary: changed
  outside declared boundary: CHANGELOG.md, CLAUDE.md, Cargo.lock, Cargo.toml, RUNBOOK.md,
  crates/happenstance-core/Cargo.toml, crates/happenstance-core/src/lib.rs,
  crates/happenstance-core/src/projection.rs, crates/happenstance-core/src/projection_memory.rs,
  crates/happenstance-core/src/store.rs, crates/happenstance-core/tests/projection_memory.rs,
  crates/happenstance-core/tests/projection_probe_round_trip.rs,
  crates/happenstance-ladybug/Cargo.toml, crates/happenstance-ladybug/src/lib.rs,
  crates/happenstance-ladybug/src/projection_store.rs, crates/happenstance-ladybug/src/stand_in.rs,
  crates/happenstance-ladybug/tests/port_shape.rs, crates/happenstance-neon/Cargo.toml,
  crates/happenstance-neon/src/projection_store.rs, crates/happenstance-postgres/Cargo.toml,
  crates/happenstance-postgres/src/projection_store.rs, crates/happenstance-sqlite/Cargo.toml,
  crates/happenstance-sqlite/LICENSE-APACHE, crates/happenstance-sqlite/LICENSE-MIT,
  crates/happenstance-sqlite/README.md, crates/happenstance-sqlite/src/connection.rs,
  crates/happenstance-sqlite/src/event_store.rs, crates/happenstance-sqlite/src/lib.rs,
  crates/happenstance-sqlite/src/projection_store.rs, crates/happenstance-sqlite/src/query_sql.rs,
  crates/happenstance-sqlite/src/row.rs, crates/happenstance-sqlite/tests/append.rs,
  crates/happenstance-sqlite/tests/concurrency.rs, crates/happenstance-sqlite/tests/conformance.rs,
  crates/happenstance-sqlite/tests/front_page.rs, crates/happenstance-sqlite/tests/migration.rs,
  crates/happenstance-sqlite/tests/projection.rs, crates/happenstance-sqlite/tests/read.rs,
  crates/happenstance-sqlite/tests/shapes.rs, crates/happenstance-sqlite/tests/support/mod.rs,
  crates/happenstance-sqlite/tests/wide_query.rs, crates/happenstance-sync/Cargo.toml,
  crates/happenstance-testkit/Cargo.toml, crates/happenstance-testkit/README.md,
  crates/happenstance-testkit/src/bench.rs, crates/happenstance-testkit/src/concurrency.rs,
  crates/happenstance-testkit/src/contract.rs, crates/happenstance-testkit/src/faulty.rs,
  crates/happenstance-testkit/src/fixtures.rs, crates/happenstance-testkit/src/gappy.rs,
  crates/happenstance-testkit/src/lib.rs, crates/happenstance-testkit/src/projection.rs,
  crates/happenstance-testkit/tests/faulty_store_conformance.rs,
  crates/happenstance-testkit/tests/faulty_store_instruments.rs,
  crates/happenstance-testkit/tests/faulty_store_send_guard.rs,
  crates/happenstance-testkit/tests/gappy_memory_conformance.rs,
  crates/happenstance-testkit/tests/gappy_store_instruments.rs,
  crates/happenstance-testkit/tests/memory_benchmarks.rs,
  crates/happenstance-testkit/tests/mutation_coverage.rs,
  crates/happenstance-testkit/tests/mutation_coverage/correct.rs,
  crates/happenstance-testkit/tests/mutation_coverage/harness.rs,
  crates/happenstance-testkit/tests/mutation_coverage/mutants.rs,
  crates/happenstance-testkit/tests/mutation_coverage/racers.rs,
  crates/happenstance-testkit/tests/mutation_coverage/variants.rs,
  crates/happenstance-testkit/tests/projection_conformance.rs,
  crates/happenstance-testkit/tests/projection_conformance_blocking.rs,
  crates/happenstance-testkit/tests/projection_conformance_buffering.rs,
  crates/happenstance-testkit/tests/projection_conformance_wasm.rs,
  crates/happenstance-testkit/tests/projection_harness_parity.rs,
  crates/happenstance-testkit/tests/projection_mutation_coverage.rs,
  crates/happenstance-testkit/tests/projection_mutation_coverage/buffering.rs,
  crates/happenstance-testkit/tests/projection_mutation_coverage/correct.rs,
  crates/happenstance-testkit/tests/projection_mutation_coverage/harness.rs,
  crates/happenstance-testkit/tests/projection_mutation_coverage/mutants.rs,
  crates/happenstance-testkit/tests/projection_mutation_coverage/variants.rs,
  crates/happenstance/Cargo.toml, crates/happenstance/README.md,
  crates/happenstance/src/boundary.rs, crates/happenstance/src/codec.rs,
  crates/happenstance/src/command.rs, crates/happenstance/src/composition.rs,
  crates/happenstance/src/domain.rs, crates/happenstance/src/lib.rs,
  crates/happenstance/src/runner.rs, crates/happenstance/src/sealed.rs,
  crates/happenstance/src/testing/mod.rs, crates/happenstance/src/testing/render.rs,
  crates/happenstance/src/testing/tests.rs, crates/happenstance/src/tests.rs,
  crates/happenstance/tests/boundary_refusal.rs, crates/happenstance/tests/codec_round_trip.rs,
  crates/happenstance/tests/codec_tag.rs, crates/happenstance/tests/command_loop.rs,
  crates/happenstance/tests/composition.rs, crates/happenstance/tests/doc_budget.rs,
  crates/happenstance/tests/doc_surface.rs, crates/happenstance/tests/docs_composition.rs,
  crates/happenstance/tests/dsl_failure_message.rs, crates/happenstance/tests/flavours.rs,
  crates/happenstance/tests/manifest_contract.rs,
  crates/happenstance/tests/mounted_at_the_crate_root.rs,
  crates/happenstance/tests/projection_clauses.rs, crates/happenstance/tests/projection_runner.rs,
  crates/happenstance/tests/retry_without_a_database.rs, deny.toml, docs/README.md,
  docs/adapter-reading-order.md, docs/append-conditions.md, docs/carry-your-invariant.md,
  docs/first-encounter.md, docs/read-the-worked-example.md, docs/text-fences.md,
  examples/course-subscriptions/Cargo.toml, examples/course-subscriptions/src/main.rs,
  examples/course-subscriptions/src/overview.md, examples/course-subscriptions/tests/reach.rs,
  examples/course-subscriptions/tests/runs.rs, examples/course-subscriptions/tests/ui.rs,
  examples/course-subscriptions/tests/ui/handled_variant.rs,
  examples/course-subscriptions/tests/ui/unhandled_variant.rs,
  examples/course-subscriptions/tests/ui/unhandled_variant.stderr,
  examples/outside-projection-adapter/Cargo.toml, examples/outside-projection-adapter/src/lib.rs,
  examples/outside-projection-adapter/tests/outside_projection_capability_skip.rs,
  examples/outside-projection-adapter/tests/outside_projection_conformance.rs,
  examples/outside-projection-adapter/tests/outside_projection_discrimination.rs,
  examples/outside-projection-adapter/tests/support/mod.rs,
  experiments/append-condition/Cargo.lock, experiments/append-condition/Cargo.toml,
  experiments/append-condition/README.md, experiments/append-condition/results/append-condition.md,
  experiments/append-condition/results/contention-64.md,
  experiments/append-condition/results/raw/conformance.txt,
  experiments/append-condition/results/raw/contention.txt,
  experiments/append-condition/results/raw/durability.txt,
  experiments/append-condition/results/raw/harness.txt,
  experiments/append-condition/results/raw/tag-storage.txt,
  experiments/append-condition/results/tag-storage.md, experiments/append-condition/run.sh,
  experiments/append-condition/src/candidate.rs, experiments/append-condition/src/durability.rs,
  experiments/append-condition/src/lib.rs, experiments/append-condition/src/strategy.rs,
  experiments/append-condition/src/tags.rs,
  experiments/append-condition/tests/candidates_are_conformant.rs,
  experiments/append-condition/tests/contention_at_64.rs,
  experiments/append-condition/tests/durability_settings_are_enforced.rs,
  experiments/append-condition/tests/measure.rs, experiments/append-condition/tests/support/mod.rs,
  experiments/append-condition/tests/tag_storage_probe.rs,
  experiments/live-handle-projection-batch/README.md,
  experiments/live-handle-projection-batch/live_handle.rs, experiments/polling-cost/Cargo.lock,
  experiments/polling-cost/Cargo.toml, experiments/polling-cost/README.md,
  experiments/polling-cost/results/pass-001/records.ndjson, experiments/polling-cost/run.sh,
  experiments/polling-cost/schema/manifest.schema.json,
  experiments/polling-cost/schema/result-record.schema.json,
  experiments/polling-cost/src/counting.rs, experiments/polling-cost/src/lib.rs,
  experiments/polling-cost/src/main.rs, experiments/polling-cost/src/observer.rs,
  experiments/polling-cost/src/progress.rs, experiments/polling-cost/src/sources.rs,
  experiments/polling-cost/src/validate.rs, experiments/polling-cost/tests/amplification.rs,
  experiments/polling-cost/tests/gate_inertness.rs, experiments/polling-cost/tests/no_verdict.rs,
  experiments/polling-cost/tests/one_cell.rs, experiments/polling-cost/tests/output_format.rs,
  experiments/polling-cost/tests/readme.rs, experiments/polling-cost/tests/record_conditions.rs,
  experiments/polling-cost/tests/rerun_is_additive.rs, experiments/polling-cost/tests/schema.rs,
  experiments/polling-cost/tests/staleness.rs, experiments/polling-cost/tests/sweep_shape.rs,
  experiments/wire-format/Cargo.lock, references/adapter-shapes.md,
  references/adr/0007-projection-runner-decodes.md,
  references/adr/0017-what-a-projection-batch-owns.md,
  references/adr/0018-returning-a-projection-to-never-run.md,
  references/adr/0019-what-happens-when-apply-fails.md,
  references/adr/0020-fold-query-agreement.md,
  references/adr/0021-payload-evolution-and-codec-tag.md,
  references/adr/0022-append-condition-strategy.md,
  references/adr/0030-the-checkpoint-reports-the-commits-that-happened.md,
  references/evaluation/README.md, references/evaluation/phase-6-projection-proof-at-closeout.md,
  references/evaluation/phase-6-projection-proof.md, references/evaluation/phase-7-contract-defects.md,
  references/evaluation/phase-7-macros-verdict.md,
  references/evaluation/projection-batch-shape-evidence.md,
  references/evaluation/ps-clause-pairing-sweep.md, references/seeds/user-documentation.md,
  spec/E2E-CASES.md, spec/SPECIFICATION.md, standards/pages/00-one-need.md,
  standards/pages/10-the-need-set.md, standards/pages/20-the-fold-line.md,
  standards/pages/30-citing-the-specification.md, standards/pages/40-reviewing-a-page.md,
  standards/pages/README.md, standards/pages/examples/two-needs.md,
  standards/rust/01-standard-of-evidence.md, standards/rust/11-const-construction-and-panics.md,
  standards/rust/13-sealing-and-exhaustiveness.md, standards/rust/20-two-flavour-ports.md,
  standards/rust/21-send-is-not-inherited.md, standards/rust/22-rpitit-and-lifetime-capture.md,
  standards/rust/23-streams-and-state-machines.md, standards/rust/24-the-blocking-bridge.md,
  standards/rust/25-what-removes-send-and-sync.md, standards/rust/30-error-taxonomy.md,
  standards/rust/40-public-surface-and-evolution.md, standards/rust/41-declarative-macros.md,
  standards/rust/51-features-and-no-std.md, standards/rust/52-wasm32-and-target-cfg.md,
  standards/rust/60-what-a-test-must-prove.md, standards/rust/61-compile-time-assertions.md,
  standards/rust/62-doctests-and-harnesses.md, standards/rust/70-rustdoc-obligations.md,
  standards/rust/80-the-gate.md, standards/rust/81-checks-that-cannot-be-types.md,
  standards/rust/90-skeletons-and-todo.md, standards/rust/91-adapter-authoring-recipe.md,
  standards/rust/92-toolchain-limits-and-dead-ends.md, standards/rust/README.md,
  xtask/src/affected.rs, xtask/src/lib.rs, xtask/src/lint_narrative.rs, xtask/src/lint_pages.rs,
  xtask/src/lints.rs, xtask/src/main.rs, xtask/src/narrative.rs, xtask/src/narrative_doctests.rs,
  xtask/src/pointers.rs, xtask/src/proof.rs, xtask/src/reserve.rs, xtask/src/spec_trace.rs,
  xtask/tests/falsification_drill.rs, xtask/tests/first_encounter.rs; declared but matched no
  changed file: .bklg/docs-that-teach/reach-and-adapter-path/error-site-walk-record/**,
  .bklg/docs-that-teach/reach-and-adapter-path/project.md. provenance: 9 file(s) changed inside
  this story's declared boundary and links.commits is empty. Record the checkpoint commit:
  redkiln record-links <id> --sha <sha>.
