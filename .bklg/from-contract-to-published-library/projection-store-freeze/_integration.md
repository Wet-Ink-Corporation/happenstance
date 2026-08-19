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
published crate or a freeze verdict later projects own, and running them here would
fail by design. What was proved instead is the pair this project is answerable
for — every capability it delivered is **mounted and reachable**, and its
**integration gate is green**.

**This is a re-audit, and it supersedes the one recorded at `b4c7972`.** Five
commits landed after that record, and one of them changed the answer to its loudest
open item: `dc363f4` landed `fresh_projection_has_no_checkpoint`, the seventeenth
rule the previous audit had to record as a *disclosed absence*. The hold ended on
its own stated escape condition — ADR-0030 reached `status: accepted` through a
human-invoked `/redkiln:kb-ingest` wave (`9efda45`), minting PS-38 rather than
widening `[FROZEN]` PS-19. **There is no longer any missing dependency at this
project's bar.** Everything below was re-executed against the current tree; nothing
is carried forward from the earlier record.

Two things are worth stating out loud rather than leaving to be inferred from a
green tick:

- **The one whole-initiative DoD item this project owns outright — DoD 7 — was
  executed and passed, both halves.** It is listed under the project bar below, not
  among the deferrals.
- **A green gate is a precondition here, never a substitute.** `cargo xtask ci
  --fast` exiting 0 is consistent with a `CheckpointOnlyStore` quietly dropped from
  the registry and a second batch shape that is the oracle wearing a hat. So every
  row below names the **test** and the **subject**, and the two batch shapes and the
  wrong store are named individually.

## Project integration bar

Every scenario this project owns, with the run that produced its result. Gate
command: `cargo xtask ci --fast` (`.redkiln/config.yaml:55`, `integration_scoped`)
— exit 0, closing line *"all required checks passed (--fast: 4 optional steps not
run)"*. Nothing below is `test.fixme`, `#[ignore]`d or flag-gated off: an
`#[ignore` sweep over `crates`, `examples` and `xtask` returns only prose mentions
inside `xtask/src/proof.rs` and `projection_harness_parity.rs`, both of which are
*about* the hazard rather than instances of it.

| # | Scenario (what it proves) | Ran? | Result |
| --- | --- | --- | --- |
| IB-1 | **Initiative DoD 7, first half — the suite discriminates.** `CheckpointOnlyStore` fails by name. | executed | **PASS** — `projection_mutation_coverage::projection_mutants_fail_exactly_their_declared_rules`, which holds the mutant to failing *exactly* `commit_is_atomic_with_the_read_model`, `dropped_batch_leaves_store_usable` and `distinct_projections_advance_independently` (`crates/happenstance-testkit/tests/projection_mutation_coverage.rs:302-314`) |
| IB-2 | The same discrimination reached **from outside the workspace**, through the published rule functions. | executed | **PASS** — `outside_projection_discrimination` 3/3, including `commit_is_atomic_with_the_read_model_fails_the_checkpoint_only_store` and its positive control `..._passes_the_conformant_store` (`examples/outside-projection-adapter/tests/outside_projection_discrimination.rs`) |
| IB-3 | **Initiative DoD 7, second half — two structurally unlike batch shapes pass.** Shape A: a materialised delta. | executed | **PASS** — `crates/happenstance-testkit/tests/projection_conformance.rs` against `MemoryProjectionFixture`: **17/17**, of which 15 pass and 2 are *reported skips* carrying the fixture's stated reason (`COMMIT_FAULT`, `RESET_REFUSAL`) |
| IB-4 | Shape B: a replayable op journal, holding no handle, transaction or lock between `begin` and `commit`. | executed | **PASS** — `crates/happenstance-testkit/tests/projection_conformance_buffering.rs`: **18/18** — all 17 rules `Passed`, **no skip**, plus `the_read_model_changes_only_at_commit`, the assertion a green suite cannot make about itself |
| IB-5 | Shape B is evidence about the **port**, not a second name for the oracle. | executed | **PASS** — `projection_mutation_coverage::the_second_batch_shape_answers_every_rule_with_a_pass`, which requires the outcome set to equal the enumeration, in order, every outcome `Passed` and `declines` empty (`projection_mutation_coverage.rs:1503-1541`), plus `projection_conformant_variants_pass_everything`, whose second assertion requires every rule to have executed against a conformant variant |
| IB-6 | **The seventeenth rule exists and is mounted in the one enumeration** — the previous audit's disclosed absence, now closed. | executed | **PASS** — `fresh_projection_has_no_checkpoint` is defined at `crates/happenstance-testkit/src/projection.rs:1467` and listed at `:1910` inside `for_each_projection_store_rule!` (`:1884-1919`). It ran and passed on **all four fixtures and all three emitters**: `projection_conformance`, `projection_conformance_blocking`, `projection_conformance_buffering`, `projection_conformance_wasm` and `outside_projection_conformance` |
| IB-7 | **AC-007 / DR-08 — the bar is held for an author this repository did not write.** A fixture written from the documentation alone clears the whole suite. | executed | **PASS** — `examples/outside-projection-adapter`: `outside_projection_conformance` **17/17** (16 pass, one skip carrying *this* fixture's own `RESET_REFUSAL` reason), `outside_projection_capability_skip` 3/3, `outside_projection_discrimination` 3/3 |
| IB-8 | **AC-005 / DR-03 — a declined guarantee is reported, never vanished.** | executed | **PASS** — `a_batch_with_no_read_path_is_reported_as_a_skip` asserts both fields of `RuleOutcome::Skipped` against the exported constants; `a_declined_capability_returns_a_skip_carrying_this_fixtures_own_reason` does the same from outside; `mutation_coverage::projection_capability_skips_are_reported` and `::projection_capability_reasons_are_authored_once` hold the reference fixture. Asserted on values, never on stdout |
| IB-9 | **AC-016 / DR-09 — every projection rule runs and passes on the constrained target.** | executed | **PASS** — `CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_RUNNER=wasm-bindgen-test-runner cargo test --locked -p happenstance-testkit --target wasm32-unknown-unknown --test projection_conformance_wasm` gave **17 passed, 0 failed, 0 ignored**. That is CI's `conformance on wasm32` job (`.github/workflows/ci.yml:204-237`) executed here rather than asserted. **Disclosed:** the local `--fast` gate only *type-checks* the wasm harnesses (`xtask/src/main.rs:231-241`), and on the host `projection_conformance_wasm` reports `running 0 tests`. Only the run above discharges AC-016 |
| IB-10 | **AC-001 / DR-02 — one enumeration, three harnesses, no rule silently absent.** | executed | **PASS** — `projection_harness_parity::no_harness_lists_a_rule_by_hand` and `::each_harness_invokes_the_suite_exactly_once` (2/2); tokio 17/17, blocking 17/17, wasm 17/17, all resolving `for_each_projection_store_rule!` |
| IB-11 | **AC-003 / DR-04 — no rule without a named wrong implementation.** | executed | **PASS** — `every_projection_rule_has_a_mutant`, `projection_mutant_registry_is_exhaustive` and `every_projection_mutant_states_its_provenance` (7/7 in the projection registry target). The gate printed *"happenstance-testkit/projection_mutation_coverage: 7 named tests present, 21 registry rows"* — a denominator reported, never a pass rate. Two rows are new since the previous audit: `PresumedLiveCheckpointStore`, which PS-38 made a mutant, and `AbsentAfterResetStore`, the legal occupant of the missing-checkpoint seam |
| IB-12 | **AC-014 — the port's disposition is enforced, not asserted.** | executed | **PASS** — `cargo xtask spec-trace` green (*"201 clauses (139 FROZEN, 50 PROVISIONAL, 10 DEFERRED, 2 NON-NORMATIVE), 112 conformance rules, 58 e2e cases, 379 citations checked"*, then *"traceability: no problems found; §7.1–§7.2 matches the checker"*), `no retired rule is still live` green in the same run, and the four `xtask` guards `the_projection_port_is_behind_an_off_by_default_feature`, `the_gate_is_mounted_on_the_module_and_its_re_exports`, `every_crate_that_names_a_projection_item_opts_in` and `the_typed_layer_makes_no_promise_it_does_not_keep` all pass (`xtask/src/main.rs:1003-1063`). PS-19's and PS-38's dagger is gone from §7.2 because the rule they named now exists |
| IB-13 | **AC-015 — the PS-3 evidence is a written finding, cited by the machine path.** | executed | **PASS** — `references/evaluation/projection-batch-shape-evidence.md` exists, is indexed in `references/evaluation/README.md`, and is cited from PS-3's clause body in `spec/SPECIFICATION.md`, so `spec-trace` resolves it on every gate run. It decides nothing; the exposure call stays `publication-and-positioning`'s |
| IB-14 | **The proof artefact is held by the gate, not merely written.** | executed | **PASS, with finding F-1** — the `each phase's proof artefacts` step ran all five `ARTEFACTS` rows, asserting every named test out of `--list` before running it (`xtask/src/proof.rs:195-226`). The document itself is now pinned to a superseded tree; that is F-1, not a red step |
| IB-15 | **Project DoD 3 — the decision atoms are accepted, and the questions they answer resolved rather than deleted.** | executed | **PASS** — `.kb/decisions/0017-what-a-projection-batch-owns.md`, `0018-returning-a-projection-to-never-run.md`, `0019-what-happens-when-apply-fails.md` and `0030-the-checkpoint-reports-the-commits-that-happened.md` are all `status: accepted`; `ps-1-states-no-progress-obligation`, `ps-19-scope-narrower-than-its-rule` and `projection-store-batch-has-no-apply-seam` are `superseded`, not deleted; `.kb/_intake/` holds only its `README.md`; `redkiln validate --kb` reported *"validate passed"* |
| IB-16 | **AC-013 — the skeletons still implement the port, and the whole workspace still compiles.** | executed | **PASS** — an `impl ... ProjectionStore for` survives in all five adapter crates (`happenstance-sqlite`, `happenstance-postgres`, `happenstance-ladybug`, `happenstance-neon`, plus the testkit's own fixture handle), and `cargo xtask ci --fast` is exit 0: fmt, clippy `-D warnings` over all targets and all features, `cargo test --locked --workspace --all-features`, the proof-artefact step, all four mandatory `wasm32` steps, docs under `RUSTDOCFLAGS=-D warnings`, `spec-trace`, the five file-reading lints, the `--no-default-features` doc build, and `cargo package --list` over the three publishable crates |
| IB-17 | **AC-012 — the port is documented by a doctest, not a mock.** | executed | **PASS** — six projection doctests compiled and ran in the gate: `projection.rs - projection (line 102)`, `projection::ProjectionStore (line 320)`, `projection::SendProjectionStore (line 320)`, `projection_memory::MemoryProjectionStore (line 56)`, `fixtures::MemoryProjectionFixture (line 415)`, `projection::for_each_projection_store_rule (line 1875)` |

**Initiative DoD 7 is therefore executed and green in both halves** (IB-1 / IB-2 and
IB-3 / IB-4 / IB-5). It is the one whole-initiative item this project owns outright,
and it is deliberately *not* in the deferral list below.

## Deferred to terminal project

Whole-initiative Definition-of-Done journeys this project does **not** own. Each
depends on substrate a later project builds; running it here would fail by design,
and none counts against this project's bar.

| Initiative DoD | Why it cannot pass here | Owner |
| --- | --- | --- |
| 1 — @smoke the worked example runs end to end | needs the typed layer and a real store behind `course-subscriptions` | `typed-layer-and-alpha-release` (HS-P0011) |
| 2 — @smoke the compile-fail case for an unhandled event variant | the typed layer's derive surface does not exist yet | `typed-layer-and-alpha-release` (HS-P0011) |
| 3 — the durable store passes the suite for real | every store crate is still a `todo!()` skeleton | `sqlite-durable-store` (HS-P0012) |
| 4 — the constrained-runtime store passes the suite on `wasm32` | `happenstance-cloudflare` is a skeleton. IB-9 proves the *projection* family on that target, which is a different claim | `cloudflare-durable-object-store` (HS-P0013) |
| 5 — a store that does not serialise its writers passes the suite | `happenstance-postgres` is a skeleton | `postgres-and-neon-stores` (HS-P0014) |
| 6 — a store with no connection, transaction or cursor passes the suite | `happenstance-neon` is a skeleton | `postgres-and-neon-stores` (HS-P0014) |
| 8 — the freeze verdict is written | needs the graph-shaped third batch shape; explicitly out of scope per `project.md`, *Out of scope* | `ladybug-projection-store` (HS-P0015) |
| 9 — @smoke a stranger can install it from the registry | nothing is published | `publication-and-positioning` (HS-P0016) |
| 10 — the published crate looks finished | there is no registry page to look at | `publication-and-positioning` (HS-P0016) |
| 11 — the release is diffed against the prior published baseline | there is no prior published baseline | `publication-and-positioning` (HS-P0016) |
| 12 — the clause ledger is audited at publish | a publish-time run over the whole specification | `publication-and-positioning` (HS-P0016) |
| 13 — `cargo xtask ci` green on the exact tree that was published | there is no published tree | `publication-and-positioning` (HS-P0016) |
| 14 — replication has an answer on disk | the ingest decision belongs to a different port | `replication-identity-and-ingest` (HS-P0017) |
| 15 — incomplete logs have an answer on disk | needs a store holding only a suffix of its own log | `retention-and-incomplete-logs` (HS-P0018) |
| 16 — the audience is durable | persona and journey atoms are the closeout's | `closeout-and-durable-audience` (HS-P0019) |

## Reachability

Every capability this project delivered, traced to the place it is actually reached
from — not to a passing unit test. "Constructed" is not "integrated": for a library
the composition root is the **published item graph plus the gate that compiles and
runs it**, so a capability counts as mounted when a `cargo xtask ci` step reaches it
through a real consumer.

| Capability | Mount point | Reachable? |
| --- | --- | --- |
| `ProjectionStore`, its value types and error types — `owned-batch-port-shape` | `crates/happenstance-core/src/lib.rs:128-130,160-165` — `pub mod projection;` and its re-exports, gated on `unstable-projection` | **Yes.** `type Batch;` carries no lifetime (`crates/happenstance-core/src/projection.rs:436`). Consumed by `happenstance-testkit`, which names the feature unconditionally (`crates/happenstance-testkit/Cargo.toml:47`), by the five adapter skeletons, and by `examples/outside-projection-adapter` |
| `ProjectionProbe`, the write seam — `projection-probe-conformance-feature` | `crates/happenstance-core/src/lib.rs:167-169`, behind `conformance`, which implies `unstable-projection` (`crates/happenstance-core/Cargo.toml:97`) | **Yes.** `ProjectionFixture::Store` is bound on it, so every projection rule drives a read model through it; `examples/outside-projection-adapter/src/lib.rs:337,481` implements it in `src/`, which is the orphan-rule reason it lives in the contract crate |
| `MemoryProjectionStore` — the oracle and doctest target — `memory-projection-store` | `crates/happenstance-core/src/lib.rs:139-144,175-182`, behind `memory` **and** `unstable-projection`; impls at `crates/happenstance-core/src/projection_memory.rs:270,364` | **Yes.** Backs `fixtures::MemoryProjectionFixture`, three of the four projection harnesses, and two rendered doctests that ran green |
| `projection_store_conformance!` — the entry point — `projection-suite-entry-point` | `crates/happenstance-testkit/src/lib.rs:296` (`pub mod projection;`, unconditional) and `:522` (the macro), spelling `$crate::__private::ProjectionFixture` | **Yes.** Four in-tree invocations plus one from outside the workspace; the outside invocation is the only thing exercising the `__private` re-export for real |
| The three emitters — tokio, blocking, wasm | `crates/happenstance-testkit/tests/projection_conformance.rs`, `projection_conformance_blocking.rs`, `projection_conformance_wasm.rs` | **Yes.** All three executed: 17/17 each, the wasm one under `wasm-bindgen-test-runner` |
| The seventeen rules — `commit-rollback-and-drop-rules`, `reset-rules`, `read-through-and-rebuild-rules` | `crates/happenstance-testkit/src/projection.rs:1884-1919` — the single enumeration | **Yes.** Each rule is emitted once per harness and none is named by hand anywhere, held by `projection_harness_parity::no_harness_lists_a_rule_by_hand` |
| The second batch shape, `BufferingProjectionStore` — `buffering-conformant-variant` | `crates/happenstance-testkit/tests/projection_mutation_coverage/buffering.rs:282,420`, mounted twice — a `REGISTRY` row and a `for_each_projection_mutant!` entry — and its own conformance target `projection_conformance_buffering.rs` | **Yes.** One definition, two consumers, via `#[path]` — no second copy the registry does not describe |
| The projection mutant registry, 21 rows including `CheckpointOnlyStore` — `projection-mutant-registry` | `crates/happenstance-testkit/tests/projection_mutation_coverage.rs:302-314` and its `mutants.rs` sibling module | **Yes.** Held by the gate's proof-artefact step, which asserts all seven meta-test names out of `--list` before running them (`xtask/src/proof.rs:172-178`) |
| The declension surface — `projection-capability-skips` | `crates/happenstance-testkit/src/contract.rs` — `Capability`, `RuleOutcome`, and `ProjectionFixture`'s three required constants | **Yes.** The reference fixture declines two and the outside fixture declines one; both reasons appeared in the run's captured stdout and both are asserted on `RuleOutcome` values |
| The documented extension surface, six steps — `documented-extension-surface` | `crates/happenstance-testkit/src/lib.rs:175-238` | **Yes.** `examples/outside-projection-adapter` is a workspace member (`Cargo.toml:3`, `members = ["crates/*", "examples/*", "xtask"]`), `publish = false`, and its four targets are built and run by the gate's `--workspace --all-features` test step |
| The `unstable-projection` gate itself — `unstable-projection-gate-and-clause-disposition` | `crates/happenstance-core/Cargo.toml:68,97`, opted into by name in six workspace manifests, and via `conformance` in the outside example | **Yes.** Four `xtask` unit tests read the manifests and `lib.rs` and fail if the gate stops gating (`xtask/src/main.rs:1003-1063`) |
| `cargo xtask proof-artefact` and its two projection rows — `whole-gate-run-and-proof-artefact` | `xtask/src/proof.rs:195-213`, reached from the gate step in `xtask/src/main.rs` | **Yes.** Ran in this audit's gate invocation; `proof::tests::the_phase_six_targets_are_held` (`xtask/src/proof.rs:471`) fails if either row is removed |
| The PS-3 batch-shape finding — `ps3-batch-shape-finding` | `references/evaluation/projection-batch-shape-evidence.md`, indexed in `references/evaluation/README.md` and cited from PS-3's clause body in `spec/SPECIFICATION.md` | **Yes.** `spec-trace` resolves the citation on every gate run, so the document cannot be deleted in silence |
| The PS-clause pairing sweep — `ps-clause-pairing-sweep` | `references/evaluation/ps-clause-pairing-sweep.md`, cited from the ADR-0030 atom and from PS-19's clause body | **Yes.** Same machine path — a citation `spec-trace` resolves |
| The decision atoms — `projection-decision-atoms` | `.kb/decisions/0017-what-a-projection-batch-owns.md`, `0018-returning-a-projection-to-never-run.md`, `0019-what-happens-when-apply-fails.md`, plus `0030-the-checkpoint-reports-the-commits-that-happened.md` minted mid-project | **Yes.** All `status: accepted`; `redkiln validate --kb` passes and checks each accepted atom against `HEAD` |
| The public-API design record — `projection-api-design-record` | `.bklg/from-contract-to-published-library/projection-store-freeze/_design.md` — DT-3 at `:234`, DT-8 at `:366` | **Yes**, with a defect — see F-4. Both tensions carry recorded resolutions; one worked example in the record is contradicted by the shipped manifest |

**Nothing is constructed-but-unmounted, exported-but-unconsumed, or reachable only
through a test that nothing holds.** The one thing that could have been —
`projection_mutation_coverage` and `projection_harness_parity`, whose deletion or
emptying `cargo test --workspace` would not have noticed — is exactly what the two
`ARTEFACTS` rows close.

## Missing dependencies

**None.** The previous audit recorded one: the seventeenth rule
`fresh_projection_has_no_checkpoint`, held because writing it would have widened
`[FROZEN]` PS-19 by test, with the repair staged in `.kb/_intake/`. That hold is
**over, and its own stated escape condition is what ended it** — ADR-0030 reached
`status: accepted` through a human-invoked ingest wave (`9efda45`), minting PS-38
whose second sentence carries the obligation verbatim, and PS-19 is byte-identical
across the whole episode. The rule landed at `dc363f4`, is mounted in the one
enumeration, and passes on four fixtures and three emitters (IB-6).

Nothing else this project should provide is absent. There is no `#[ignore]`, no
`test.fixme`, and no flag-gated-off scenario anywhere in this project's surface.

## Findings

Recorded and routed. None makes a delivered capability unreachable and none reds the
declared integration gate, so none blocks this project's bar — but F-1 and F-2 are
live documentation that now contradicts shipped code, and F-5 is a backlog-state
discrepancy the next run will trip over.

**F-1 — the project's own proof artefact is pinned to a tree that is no longer the
tree.** `references/evaluation/phase-6-projection-proof.md` is pinned to `7620481`
and says, at `:114-118`: *"Sixteen rules are registered in it; a seventeenth,
`fresh_projection_has_no_checkpoint`, is specified and not yet written, and §7.2
daggers it for that reason."* Both halves stopped being true at `dc363f4`: seventeen
rules are registered, and §7.2's dagger was removed in that same commit. The
document is **honestly dated and pinned**, so this is not a false claim — but
`references/evaluation/README.md`'s own lifecycle rule is *superseded rather than
edited*, which means the repair is a **new dated document naming this one**, and
that document does not exist. Until it does, the artefact that the runbook's *"a
phase is done when its proof artefact exists"* points at describes a superseded
tree, and the delta is precisely the item that was this project's disclosed hold.
**Owed by this project before closeout.** The current-tree evidence is in this
record's IB table meanwhile. `references/evaluation/projection-batch-shape-evidence.md`
is in the same position and is *correctly* left alone: it is pinned to `cfd9231`,
reports one named run, and says in terms that a changed rule set is a new document.

**F-2 — `spec/SPECIFICATION.md` contradicts itself about the seventeenth rule, in
three authored places.** §7.2 is generated and correct; the authored prose is not:

- `:5316-5318` — *"The rule is still unwritten, which is why it is marked new above
  and daggered in §7.2"*
- `:5457-5458` — *"and the new `fresh_projection_has_no_checkpoint` for the second
  sentence, which nothing checks today"*
- `:5838-5841` — *"Sixteen of the seventeen now exist ... `fresh_projection_has_no_checkpoint`
  is the one still to be written; it is marked new wherever it is named and daggered
  in §7.2"*

`dc363f4` touched `spec/SPECIFICATION.md` on **two lines only**, both inside the
generated region, so the generated table and the prose around it now disagree inside
one document. `cargo xtask spec-trace` cannot see this — it checks markers, rule
names, case numbers and citations, and the region it regenerates is the one that is
already right. This is the specification a consumer reads, and the three sentences
tell them that a rule which exists and is green on four fixtures does not. **Owed by
this project** — it is the same clause-disposition surface AC-014 owns — and cheap
now.

**F-3 — `crates/happenstance-testkit/README.md` is stale in the direction that
matters, and it is the file crates.io renders.** `:16-20` still say *"The
`ProjectionStore` suite exists but is two rules of seventeen, and neither has been
shown to reject a wrong store yet — the hostile stores that will are named in the
specification and not yet written"*, and `:127` repeats *"Two rules of the seventeen
the specification names, today."* Both are false by a wide margin: seventeen rules
drive the port, twenty-one registry rows drive the suite, and the hostile stores are
in `crates/happenstance-testkit/tests/projection_mutation_coverage/mutants.rs`. This
is a **repeat finding** — raised in the audit at `b4c7972` and unfixed five commits
later. The gate cannot catch it: `cargo package --list` asserts the README is
*present in the package*, not that it is *true*, and this is the third instance of
that class in this repository's own history (`CHANGELOG.md:1430-1437`, the MSRV
promise; D10, the README that never compiled). Routed to
`publication-and-positioning` (HS-P0016), which owns initiative DoD 10 — but it is
the single most consequential false sentence in the tree relative to what this
project built, because it tells the adapter author and the evaluator that this
project's whole deliverable does not exist.

**F-4 — `_design.md:386-389` shows the outside author taking `happenstance-core`
under `[dev-dependencies]`.** The landed manifest correctly puts it under
`[dependencies]` (`examples/outside-projection-adapter/Cargo.toml:20-23`), which is
the only spelling that works, because the `ProjectionProbe` impl lives in `src/` —
the argument the *same design record* makes at `:375-382`. The shipped documentation
is right (`crates/happenstance-testkit/src/lib.rs:184-191`); the design record is
the outlier, superseded by the shipped surface. Also a repeat finding from
`b4c7972`. Record-level only.

**F-5 — two of this project's seventeen stories are recorded as not started.**
`reset-rules` (HS-S0011) and `read-through-and-rebuild-rules` (HS-S0012) sit at
`stage: plan` / `status: ready` with `updated: 2026-08-13`, while the other fifteen
are `stage: report` / `status: in-review`. Their work is fully merged — `10ace94`,
`064687a`, `5f2cf02` and `dc363f4` all touch them, every `_ledger.md` row is
`satisfied: true` with cited evidence, `report.md` and `implementation-report.md`
exist, and `4b39add` sealed their slice `reset-and-rebuild-rules` **approved**. No
`redkiln advance` commit for either id exists anywhere on the branch. The capability
is live either way — both stories' rules run and pass in IB-3, IB-4, IB-7 and IB-9 —
so this is bookkeeping rather than a gap, but it makes `redkiln status` under-report
this project by two. The CLI is the only writer of those fields, so **this audit
cannot and does not fix it.** Surfaced for the orchestrator.

**F-6 — `redkiln doctor` exits 1 with nine `unconsumed-foundation` problems**, one of
which is this project's (`projection-decision-atoms`, HS-S0002) and eight of which
belong to five projects that have not started. All nine story files were added by the
planning commit `ae77ac4`, so this is a decomposition-shape condition predating every
line of implementation here — but CI's `backlog` job asserts the problem list is
empty (`.github/workflows/ci.yml:177`), so that job is red on this branch and stays
red until the decomposition is re-shaped. Initiative-level; not this project's to
settle in passing. Also a repeat finding from `b4c7972`.

## Integration verdict

**Green at this project's bar.** Every capability delivered is mounted into a real
consumer and reached by the gate; the declared integration gate `cargo xtask ci
--fast` is exit 0; the whole-initiative journeys this project does not own are
deferred to the projects that do; and the one it does own — DoD 7 — was executed and
passed in both halves, with the wrong store convicted **by name**
(`commit_is_atomic_with_the_read_model`) and the two batch shapes named individually
(`MemoryProjectionFixture`, `BufferingProjectionFixture`). The previous audit's one
missing dependency is closed by a decision atom rather than by a widened frozen
clause, which is the outcome the hold was held for.

Six findings are recorded above. Four are documentation that has fallen behind the
code it describes, and the fact that three of them are *repeats* of the previous
audit is itself the signal: this repository's gate is unusually good at holding code
to documents and has almost no instrument for holding documents to code. F-1 and F-2
are this project's to repair before closeout.

The freeze itself is **not called earned here**, and nothing in this record implies
it: both shapes that clear the suite are instruments this workspace wrote, PS-2 asks
for two *adapters* at opposite ends of the batch-shape axis, and the verdict is
`ladybug-projection-store`'s to write.
