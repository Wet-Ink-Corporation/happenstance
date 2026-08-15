---
item: HS-P0010
title: "Integration — Freeze ProjectionStore behind a suite that can fail"
initiative_slug: from-contract-to-published-library
project_slug: projection-store-freeze
terminal: false
stage: integration
created: 2026-08-15
updated: 2026-08-15
dod_green: true
reachability_ok: true
deferred_scenarios:
  - "DoD 1 — @smoke the worked example runs end to end (cargo run -p course-subscriptions)"
  - "DoD 2 — @smoke the compile-fail case for an unhandled event variant"
  - "DoD 3 — the durable store passes event_store_conformance! for real"
  - "DoD 4 — the constrained-runtime store passes the suite on wasm32"
  - "DoD 5 — a store that does not serialise its writers passes the suite"
  - "DoD 6 — a store with no connection, no interactive transaction and no cursor passes the suite"
  - "DoD 8 — the ProjectionStore freeze verdict is written"
  - "DoD 9 — @smoke a stranger can install it from the registry"
  - "DoD 10 — the published crate looks finished (rendered docs and registry page)"
  - "DoD 11 — the release is diffed against the prior published baseline"
  - "DoD 12 — the clause ledger is audited at publish"
  - "DoD 13 — cargo xtask ci green on the assembled whole that was published"
  - "DoD 14 — replication has an answer on disk"
  - "DoD 15 — incomplete logs have an answer on disk"
  - "DoD 16 — the audience is durable (persona and journey atoms)"
---

# Integration — Freeze ProjectionStore behind a suite that can fail

## Verdict

**dod-green**, at this project's own integration bar, and *only* at that bar.

This is a **non-terminal (feature) project**, so the whole-initiative Definition of
Done was deliberately not run: fifteen of its sixteen items depend on adapters, a
published crate or a freeze verdict that later projects own, and running them here
would fail by design. What was proved instead is the pair this project is
answerable for — every capability it delivered is **mounted and reachable**, and
its **affected / integration gate is green**.

Two things are worth stating out loud rather than leaving to be inferred from a
green tick:

- **The one whole-initiative DoD item this project owns outright — DoD 7 — was
  executed and passed, both halves.** It is listed under the project bar below,
  not among the deferrals.
- **A green gate is a precondition here, never a substitute.** `cargo xtask ci
  --fast` exiting 0 is consistent with a `CheckpointOnlyStore` quietly dropped from
  the registry and a second batch shape that is the oracle wearing a hat. So every
  row below names the **test** and the **subject**, and the two batch shapes and
  the wrong store are named individually.

## Project integration bar

Every scenario this project owns, with the run that produced its result. Gate
command: `cargo xtask ci --fast` (`.redkiln/config.yaml:55`, `integration_scoped`)
— exit 0, *"all required checks passed (--fast: 4 optional step(s) not run)"*.
Nothing below is `test.fixme`, `#[ignore]`d or flag-gated off; an `#[ignore` sweep
over `crates`, `examples` and `xtask` returns only prose mentions.

| # | Scenario (what it proves) | Ran? | Result |
| --- | --- | --- | --- |
| IB-1 | **Initiative DoD 7, first half — the suite discriminates.** `CheckpointOnlyStore` fails by name. | executed | **PASS** — `projection_mutation_coverage::projection_mutants_fail_exactly_their_declared_rules`, which holds the mutant to failing *exactly* `commit_is_atomic_with_the_read_model`, `dropped_batch_leaves_store_usable` and `distinct_projections_advance_independently` (`crates/happenstance-testkit/tests/projection_mutation_coverage.rs:294-330`) |
| IB-2 | The same discrimination reached **from outside the workspace**, through the published rule functions. | executed | **PASS** — `outside_projection_discrimination::commit_is_atomic_with_the_read_model_fails_the_checkpoint_only_store` (3/3 in `examples/outside-projection-adapter/tests/outside_projection_discrimination.rs`), with the rule's own panic text captured in the run: *"the read-model write and the checkpoint write must become durable together or not at all … row None, checkpoint Live { through: SequencePosition(1) }"* |
| IB-3 | **Initiative DoD 7, second half — two structurally unlike batch shapes pass.** Shape A: a materialised delta. | executed | **PASS** — `crates/happenstance-testkit/tests/projection_conformance.rs` against `MemoryProjectionFixture`: 16/16, of which 14 pass and 2 are *reported skips* carrying the fixture's stated reason (`RESET_REFUSAL`, `COMMIT_FAULT`) |
| IB-4 | Shape B: a replayable op journal, holding no handle, transaction or lock between `begin` and `commit`. | executed | **PASS** — `crates/happenstance-testkit/tests/projection_conformance_buffering.rs`: 17/17 — all 16 rules `Passed`, **no skip**, plus `the_read_model_changes_only_at_commit`, the assertion a green suite cannot make about itself |
| IB-5 | Shape B is evidence about the **port**, not a second name for the oracle. | executed | **PASS** — `projection_mutation_coverage::the_second_batch_shape_answers_every_rule_with_a_pass` (outcome set equal to the enumeration, in order, every outcome `Passed`, `declines` empty) and `projection_conformant_variants_pass_everything`, whose second assertion requires every rule to have executed against a conformant variant |
| IB-6 | **AC-007 / DR-08 — the bar is held for an author this repository did not write.** A fixture written from the documentation alone clears the whole suite. | executed | **PASS** — `examples/outside-projection-adapter`: `outside_projection_conformance` 16/16 (one skip, carrying *this* fixture's own reason), `outside_projection_capability_skip` 3/3, `outside_projection_discrimination` 3/3 |
| IB-7 | **AC-005 / DR-03 — a declined guarantee is reported, never vanished.** | executed | **PASS** — `a_batch_with_no_read_path_is_reported_as_a_skip` asserts both fields of `RuleOutcome::Skipped` against the exported constants; `a_declined_capability_returns_a_skip_carrying_this_fixtures_own_reason` does the same from outside. Asserted on values, never on stdout |
| IB-8 | **AC-016 — every projection rule runs and passes on the constrained target.** | executed | **PASS** — `CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_RUNNER=wasm-bindgen-test-runner cargo test --locked -p happenstance-testkit --target wasm32-unknown-unknown --test projection_conformance_wasm` gave **16 passed, 0 failed, 0 ignored**. That is the CI job `conformance on wasm32` (`.github/workflows/ci.yml:204-237`) executed here rather than asserted; the local gate only *type-checks* the wasm harnesses, which `references/evaluation/phase-6-projection-proof.md` discloses |
| IB-9 | **AC-001 / DR-02 — one enumeration, three harnesses, no rule silently absent.** | executed | **PASS** — `projection_harness_parity::no_harness_lists_a_rule_by_hand` and `::each_harness_invokes_the_suite_exactly_once` (2/2); tokio 16/16, blocking 16/16, wasm 16/16, all resolving `for_each_projection_store_rule!` |
| IB-10 | **AC-003 / DR-04 — no rule without a named wrong implementation.** | executed | **PASS** — `every_projection_rule_has_a_mutant`, `projection_mutant_registry_is_exhaustive` and `every_projection_mutant_states_its_provenance` (7/7 in the projection registry target). The gate printed *"happenstance-testkit/projection_mutation_coverage: 7 named tests present, 19 registry rows"* — a denominator reported, never a pass rate |
| IB-11 | **AC-014 — the port's disposition is enforced, not asserted.** | executed | **PASS** — `cargo xtask spec-trace` green and `no retired rule is still live` green in the same run; the four `xtask` guards `the_projection_port_is_behind_an_off_by_default_feature`, `the_gate_is_mounted_on_the_module_and_its_re_exports`, `every_crate_that_names_a_projection_item_opts_in` and `the_typed_layer_makes_no_promise_it_does_not_keep` all pass (`xtask/src/main.rs:958-1063`) |
| IB-12 | **AC-015 — the PS-3 evidence is a written finding, cited by the machine path.** | executed | **PASS** — `references/evaluation/projection-batch-shape-evidence.md` exists, is indexed at `references/evaluation/README.md:91-112`, and is cited from PS-3's clause body at `spec/SPECIFICATION.md:4806`, so `spec-trace` resolves it on every gate run. It decides nothing; the exposure call stays `publication-and-positioning`'s |
| IB-13 | **The proof artefact is held by the gate, not merely written.** | executed | **PASS** — the `each phase's proof artefacts` step ran all five `ARTEFACTS` rows, asserting every named test out of `--list` before running it (`xtask/src/proof.rs:195-226`); `references/evaluation/phase-6-projection-proof.md` names the rule, the meta-test, both fixtures, and what the run does **not** cover |
| IB-14 | **Project DoD 3 — the decision atoms are accepted, and the questions they answer resolved rather than deleted.** | executed | **PASS** — `.kb/decisions/0017-what-a-projection-batch-owns.md`, `.kb/decisions/0018-returning-a-projection-to-never-run.md` and `.kb/decisions/0019-what-happens-when-apply-fails.md` are accepted atoms; `redkiln validate --kb` reported *"validate passed"* |
| IB-15 | **AC-013 / DR-09 — the whole workspace still compiles and the constrained target still builds.** | executed | **PASS** — `cargo xtask ci --fast` exit 0: fmt, clippy `-D warnings` (all targets, all features), `cargo test --locked --workspace --all-features`, the proof-artefact step, all four mandatory `wasm32` steps, docs under `RUSTDOCFLAGS=-D warnings`, `spec-trace`, the five file-reading lints, the `--no-default-features` doc build, and `cargo package --list` over the three publishable crates |

**Initiative DoD 7 is therefore executed and green in both halves** (IB-1 / IB-2
and IB-3 / IB-4 / IB-5). It is the one whole-initiative item this project owns
outright, and it is deliberately *not* in the deferral list below.

## Deferred to terminal project

Whole-initiative Definition-of-Done journeys this project does **not** own. Each
depends on substrate a later project builds; running it here would fail by design,
and none counts against this project's bar.

| Initiative DoD | Why it cannot pass here | Owner |
| --- | --- | --- |
| 1 — @smoke the worked example runs end to end | needs the typed layer and a real store behind `course-subscriptions` | `typed-layer-and-alpha-release` (HS-P0011) |
| 2 — @smoke the compile-fail case for an unhandled event variant | the typed layer's derive surface does not exist yet | `typed-layer-and-alpha-release` (HS-P0011) |
| 3 — the durable store passes the suite for real | every store crate is still a `todo!()` skeleton | `sqlite-durable-store` (HS-P0012) |
| 4 — the constrained-runtime store passes the suite on `wasm32` | `happenstance-cloudflare` is a skeleton. IB-8 proves the *projection* family on that target, which is a different claim | `cloudflare-durable-object-store` (HS-P0013) |
| 5 — a store that does not serialise its writers passes the suite | `happenstance-postgres` is a skeleton | `postgres-and-neon-stores` (HS-P0014) |
| 6 — a store with no connection, transaction or cursor passes the suite | `happenstance-neon` is a skeleton | `postgres-and-neon-stores` (HS-P0014) |
| 8 — the freeze verdict is written | needs the graph-shaped third batch shape; explicitly out of scope (`project.md:120-122`) | `ladybug-projection-store` (HS-P0015) |
| 9 — @smoke a stranger can install it from the registry | nothing is published | `publication-and-positioning` (HS-P0016) |
| 10 — the published crate looks finished | there is no registry page to look at | `publication-and-positioning` (HS-P0016) |
| 11 — the release is diffed against the prior published baseline | there is no prior published baseline | `publication-and-positioning` (HS-P0016) |
| 12 — the clause ledger is audited at publish | a publish-time run over the whole specification | `publication-and-positioning` (HS-P0016) |
| 13 — `cargo xtask ci` green on the exact tree that was published | there is no published tree | `publication-and-positioning` (HS-P0016) |
| 14 — replication has an answer on disk | the ingest decision belongs to a different port | `replication-identity-and-ingest` (HS-P0017) |
| 15 — incomplete logs have an answer on disk | needs a store holding only a suffix of its own log | `retention-and-incomplete-logs` (HS-P0018) |
| 16 — the audience is durable | persona and journey atoms are the closeout's | `closeout-and-durable-audience` (HS-P0019) |

## Reachability

Every capability this project delivered, traced to the place it is actually
reached from — not to a passing unit test. "Constructed" is not "integrated": for
a library the composition root is the **published item graph plus the gate that
compiles and runs it**, so a capability counts as mounted when a `cargo xtask ci`
step reaches it through a real consumer.

| Capability | Mount point | Reachable? |
| --- | --- | --- |
| `ProjectionStore`, its value types and error types | `crates/happenstance-core/src/lib.rs:128-130,160-165` — `pub mod projection;` and its re-exports, gated on `unstable-projection` | **Yes.** Consumed by `happenstance-testkit`, which names the feature unconditionally (`crates/happenstance-testkit/Cargo.toml:43-48`), by the five adapter skeletons, and by `examples/outside-projection-adapter` |
| `ProjectionProbe`, the write seam | `crates/happenstance-core/src/lib.rs:167-169`, behind `conformance`, which implies `unstable-projection` (`crates/happenstance-core/Cargo.toml:97`) | **Yes.** `ProjectionFixture::Store` is bound on it, so every projection rule drives a read model through it; the outside adapter implements it in `src/`, which is the orphan-rule reason it lives in the contract crate |
| `MemoryProjectionStore` — the oracle and doctest target | `crates/happenstance-core/src/lib.rs:139-144,175-182`, behind `memory` and `unstable-projection` | **Yes.** Backs `fixtures::MemoryProjectionFixture`, three of the four projection harnesses, and a rendered doctest (`fixtures::MemoryProjectionFixture (line 415)` ran green) |
| `projection_store_conformance!` — the entry point | `crates/happenstance-testkit/src/lib.rs:296` (`pub mod projection;`, unconditional) and the macro's `$crate::__private::ProjectionFixture` spelling | **Yes.** Four in-tree invocations plus one from outside the workspace; the outside invocation is the only thing exercising the `__private` re-export for real |
| The three emitters (tokio, blocking, wasm) | `crates/happenstance-testkit/tests/projection_conformance.rs:49`, `projection_conformance_blocking.rs:21-23`, `projection_conformance_wasm.rs:23-27` | **Yes.** All three executed: 16/16 each, the wasm one under `wasm-bindgen-test-runner` |
| The second batch shape, `BufferingProjectionStore` | `crates/happenstance-testkit/tests/projection_mutation_coverage/buffering.rs`, mounted twice — a `REGISTRY` row and a `for_each_projection_mutant!` entry (`projection_mutation_coverage.rs:802-831`, `:863`), and its own conformance target (`projection_conformance_buffering.rs:52`) | **Yes.** One definition, two consumers, via `#[path]` (`projection_mutation_coverage.rs:84-85`, `projection_conformance_buffering.rs:47-48`) — no second copy the registry does not describe |
| `impl ProjectionSubject for BufferingProjectionFixture` | `crates/happenstance-testkit/tests/projection_mutation_coverage/variants.rs:103` | **Yes.** Drives `run_buffering_store()` (`projection_mutation_coverage.rs:919-921`), which feeds both CF-5's second assertion (`:1351-1362`) and the new meta-test (`:1446`) |
| The projection mutant registry, 19 rows including `CheckpointOnlyStore` | `crates/happenstance-testkit/tests/projection_mutation_coverage.rs:293-832` and `projection_mutation_coverage/mutants.rs` | **Yes.** Held by the gate's proof-artefact step, which asserts all seven meta-test names out of `--list` before running them (`xtask/src/proof.rs:171-179,202-207`) |
| The documented extension surface, six steps | `crates/happenstance-testkit/src/lib.rs:175-238` | **Yes.** `examples/outside-projection-adapter` is a workspace member (`Cargo.toml:3`, `members = ["examples/*"]`), is in `Cargo.lock`, and its four targets are built and run by the gate's `--workspace --all-features` test step |
| The `unstable-projection` gate itself | `crates/happenstance-core/Cargo.toml:68,97`, opted into by name in all six dependent manifests | **Yes.** Four `xtask` unit tests read the manifests and `lib.rs` and fail if the gate stops gating (`xtask/src/main.rs:1003-1063`); the feature powersets compile the `no_std` and `conformance`-without-`memory` combinations |
| `cargo xtask proof-artefact`, and its two new projection rows | `xtask/src/proof.rs:202-213`, reached from the gate step at `xtask/src/main.rs:178-191` | **Yes.** Ran in this audit's gate invocation; `the_phase_six_targets_are_held` fails if either row is removed |
| The PS-3 batch-shape finding | `references/evaluation/README.md:91-112` (the human path) and `spec/SPECIFICATION.md:4806` (the machine path) | **Yes.** `spec-trace` resolves the citation on every gate run, so the document cannot be deleted in silence |
| The phase-6 proof artefact | `references/evaluation/phase-6-projection-proof.md`, indexed at `references/evaluation/README.md:114-129`, asserted by five `#[cfg(test)]` tests in `xtask/src/proof.rs:405-631` | **Yes.** Those tests run in the gate's workspace test step and read the document off disk |

**Nothing is constructed-but-unmounted, exported-but-unconsumed, or reachable only
through a test that nothing holds.** The one thing that could have been —
`projection_mutation_coverage` and `projection_harness_parity`, whose deletion or
emptying `cargo test --workspace` would not have noticed — is exactly what the two
new `ARTEFACTS` rows close.

## Missing dependencies

**None that block a scenario this project owns.**

One hand-off is genuinely outstanding, and it is recorded rather than absorbed:
the seventeenth rule `fresh_projection_has_no_checkpoint` **does not exist**,
because writing it would widen `[FROZEN]` PS-19 by test. The human was offered
path (b) — proceed on the unmet precondition with a named authorisation — and
**refused it** (`_slices.md:355-379`). The repair is staged as
`.kb/_intake/2026-08-15-adr-0030-checkpoint-progress.md`, with its long form at
`references/adr/0030-the-checkpoint-reports-the-commits-that-happened.md`, and must
reach `status: accepted` through one human-invoked `/redkiln:kb-ingest` wave;
authoring the atom by hand is forbidden (`CLAUDE.md`, *Where the work lives*).

The hold is stated at every point it could be missed: `CHANGELOG.md` carries the
non-delivery, §7.2's PS-19 and PS-38 rows carry `†`, and
`crates/happenstance-core/src/projection.rs:13-14` and
`crates/happenstance-testkit/src/projection.rs:1423,1869` each say it where the
rule would sit. That is a disclosed absence, not a skipped test — there is no
`#[ignore]`, no `test.fixme` and no flag-gated-off scenario anywhere in this
project's surface.

## Findings

Not blockers at this bar, and each is routed rather than left.

1. **`crates/happenstance-testkit/README.md` is stale in the direction that
   matters, and it is the file crates.io renders.** Lines 16-20 say *"The
   `ProjectionStore` suite exists but is two rules of seventeen, and neither has
   been shown to reject a wrong store yet — the hostile stores that will are named
   in the specification and not yet written"*, and line 127 repeats *"Two rules of
   the seventeen the specification names, today."* Both are now false: sixteen
   rules drive the port, nineteen registry rows drive the suite, and the hostile
   stores are in
   `crates/happenstance-testkit/tests/projection_mutation_coverage/mutants.rs`.
   `crates/happenstance-core/src/projection.rs:9-10` says outright that this is
   *"what this header used to say"* — the module doc moved and the README did not.
   It is the **same defect class this workspace has already paid for twice**
   (`CHANGELOG.md:1430-1437`, the MSRV promise; D10, the README that never
   compiled), and the reason it survives is stated there: *"the gate asserts the
   file is present in the package, not that it is true."* Routed to
   `publication-and-positioning` (HS-P0016), which owns initiative DoD 10 — but it
   is cheaper to fix now than to rediscover at publish, and a gate step reading a
   README's counts against the enumeration would retire the class.
2. **`redkiln doctor` exits 1 with nine `unconsumed-foundation` problems**, one of
   which is this project's (`projection-decision-atoms`, HS-S0002) and eight of
   which belong to five projects that have not started. All nine story files were
   added by the planning commit `ae77ac4`, so this is a decomposition-shape
   condition predating every line of implementation here — but CI's `backlog` job
   asserts that the problem list is empty (`.github/workflows/ci.yml:177`), so it
   is red on this branch and stays red until the decomposition is re-shaped.
   Initiative-level; not this project's to settle in passing.
3. **`_design.md:379-383` shows the outside author taking `happenstance-core`
   under `[dev-dependencies]`.** The landed manifest correctly puts it under
   `[dependencies]` (`examples/outside-projection-adapter/Cargo.toml:20-23`), which
   is the only spelling that works, because the `ProjectionProbe` impl lives in
   `src/`. `crates/happenstance-testkit/src/lib.rs:184-191` documents it right; the
   design record is the outlier, superseded by the shipped surface.

## Integration verdict

**Green at this project's bar.** Every capability delivered is mounted into a real
consumer and reached by the gate; the affected / integration gate `cargo xtask ci
--fast` is green; the whole-initiative journeys this project does not own are
deferred to the projects that do; and the one it does own — DoD 7 — was executed
and passed in both halves, with the wrong store convicted **by name**
(`commit_is_atomic_with_the_read_model`) and the two batch shapes named
individually (`MemoryProjectionFixture`, `BufferingProjectionFixture`).

The freeze itself is **not called earned here**, and nothing in this record implies
it: both shapes that clear the suite are instruments this workspace wrote, PS-2
asks for two *adapters* at opposite ends of the batch-shape axis, and the verdict
is `ladybug-projection-store`'s to write.
