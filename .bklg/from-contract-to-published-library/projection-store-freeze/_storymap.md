---
item: HS-P0010
stage: storymap
created: 2026-08-12T03:30:08.910Z
updated: 2026-08-12T03:30:08.910Z
template_sig: 1c63534a
rendered_sig: 4105133b
---

# Story Map — Freeze ProjectionStore behind a suite that can fail

Seventeen stories in eight slices, covering `project.md` AC-001 – AC-016. The
briefs are taken as given, not re-decided: **AC-A02** puts `ProjectionProbe` in
`happenstance-core` behind a `conformance` feature
(`spec/SPECIFICATION.md:4998-5031`), **AC-A03** builds the second batch shape as a
testkit-internal CF-5 conformant variant rather than reaching downstream to
`happenstance-sqlite` (`_decomposition.md` Architecture brief, Note 3), and
**AC-A04** takes AC-014's second arm — `unstable-projection` with a stated reason
— because PS-2's bar (`spec/SPECIFICATION.md:4760-4775`) is not clearable by
anything this project can build alone.

## Backbone

The activities, left to right, in the order an adapter author meets them. The
"user" here is an adapter author and the four sibling projects that build against
this port; a library's user-observable surface is its public API, its feature
table and what its suite prints.

| # | Activity | Outcome the user gets | Slices under it |
|---|---|---|---|
| B1 | **Decide the port's shape, in writing** | Every answer this project settles is an accepted atom naming the alternatives that lost, and no `[FROZEN]` clause is line-edited | `decisions-and-design-record` |
| B2 | **Implement against the port** | An owned batch, a generic write seam, and a reference store to copy from — the port an adapter can actually compile against | `projection-port-and-probe` |
| B3 | **Invoke the suite** | One entry point, one test per rule, a named failure — and a *reported* skip, with a reason, where a guarantee does not apply | `projection-conformance-suite` |
| B4 | **Be caught when wrong** | A store that commits the checkpoint and drops the read-model write fails **by name**; every rule has a store that fails it | `commit-atomicity-and-mutants`, `reset-and-rebuild-rules` |
| B5 | **Prove the port against an unlike shape** | Two structurally unlike batch shapes pass the whole suite, and what they disagreed about is written down | `second-batch-shape-and-evidence`, `outside-author-extension-surface` |
| B6 | **Freeze honestly, or say why not** | The module's maturity marker tells the truth, the PS-3 evidence is handed on, and the whole gate is green | `port-disposition-and-freeze-record` |

## Slices

The thin vertical slices (stories) under each activity, grouped by the milestone/slice they are delivered
with. Stories sharing a Milestone are implemented together in a single context and mounted as ONE integrated
surface (the implement stage runs one milestone at a time). Cross-milestone `depends_on` edges must be acyclic.
Type each story `capability` (a user-observable slice) or `foundation` (real in-tree substrate a capability
slice in this initiative consumes — never owned outside it, never a double/fixme).

For a library, "mounted" means the two places a new item is either reachable or
invisible: the crate's `lib.rs` export block and the feature table that gates it
(`_decomposition.md` Architecture brief, Note 1). No story lands an item at one
without the other.

| Milestone | Story | Archetype | One-line slice | depends_on | traces_to |
| --------- | ----- | --------- | -------------- | ---------- | --------- |
| `decisions-and-design-record` | `ps-clause-pairing-sweep` | foundation | Sweep PS-1 – PS-37 for the coupling-versus-progress pairing defect PS-1 and PS-19 both carry, and record the finding — systematic or isolated — as the input that scopes the frozen-clause repair rather than a side effect of writing rules. | — | AC-008 |
| `decisions-and-design-record` | `projection-decision-atoms` | foundation | Write and accept ADR-0017 (batch ownership and write vocabulary, PS-4 – PS-15), ADR-0018 (reset and checkpoint scope, PS-16 – PS-20) and ADR-0019 (apply-failure policy, PS-26 – PS-30) as `.kb/decisions/` atoms, each quoting a compiler transcript from `references/adapter-shapes.md`, each naming `LiveHandleProjectionStore`'s disposition, with every open-question atom they answer marked resolved rather than deleted and `redkiln validate --kb` green. | `ps-clause-pairing-sweep` | AC-008 |
| `decisions-and-design-record` | `projection-api-design-record` | foundation | Resolve DT-3 (one authoritative source for "this guarantee does not apply", citing `.kb/open-questions/cf-40-fixture-limits-ownership.md` rather than minting a second declension policy) and DT-8 (whose adapter-author bar the suite holds) in `_design.md`, alongside the public projection API surface review the repurposed `.redkiln/templates/_design.md` asks for — signatures, visibility, what the shape costs a caller, and a doctest in place of a mock. | `projection-decision-atoms` | AC-006, AC-007 |
| `projection-port-and-probe` | `owned-batch-port-shape` | foundation | Land §4.0's trait verbatim in `crates/happenstance-core/src/projection.rs` — `type Batch;` with no lifetime, non-async infallible `begin`, `Checkpoint`/`Authority`/`CommitError`/`ResetError`, `reset` — re-export the four new types from `lib.rs:98-124`, and restate the Postgres, Ladybug and SQLite skeleton signatures with the same `Batch` type, same `Error` type and same `todo!()` bodies, recorded as a reviewed diff. | `projection-decision-atoms`, `projection-api-design-record` | AC-009, AC-013 |
| `projection-port-and-probe` | `projection-probe-conformance-feature` | foundation | Add `ProjectionProbe` (`READS_THROUGH_BATCH`, `probe_write`, `probe_delete_all`, `probe_read`, `probe_read_through`) to `happenstance-core` behind a new `conformance` feature — mounted in the export block with `#[cfg_attr(docsrs, doc(cfg(…)))]` and in `Cargo.toml`'s feature table — with a round-trip test that writes and reads through nothing but the trait, and the host and wasm32 feature-powersets green over the widened combination set. | `owned-batch-port-shape` | AC-009 |
| `projection-port-and-probe` | `memory-projection-store` | foundation | Ship `MemoryProjectionStore` behind the `memory` feature as the oracle, the doctest target and the cold-start fix, implementing `ProjectionProbe` under `conformance`, with the `error[E0195]` spelling trap documented where an implementer meets it and the module named defensively so a `memory`-gated intra-doc link cannot break `cargo doc`. | `owned-batch-port-shape`, `projection-probe-conformance-feature` | AC-012 |
| `projection-conformance-suite` | `projection-suite-entry-point` | capability | An adapter author writes one line and gets one test per rule: `ProjectionFixture` in the testkit's `contract.rs` (mirroring `Fixture`, reusing `Capability`/`RuleOutcome`, no borrowing GAT), the single enumeration `for_each_projection_store_rule!` beside `for_each_event_store_rule!`, whatever emitter change Note 4 settles, `projection_store_conformance!` plus its `__private` export, an orphan meta-test over the projection rules module, the crate-doc rule-family table, and three harness files — tokio, blocking and `#![cfg(target_arch = "wasm32")]` — running the baseline pair `commit_advances_the_checkpoint` and `commit_is_atomic_with_the_read_model` against `MemoryProjectionStore`. | `owned-batch-port-shape`, `projection-probe-conformance-feature`, `memory-projection-store` | AC-001, AC-016 |
| `projection-conformance-suite` | `projection-capability-skips` | capability | A rule whose projection capability the fixture declines is still emitted as a test, returns `RuleOutcome::Skipped` carrying the fixture's stated reason, and is distinguishable from a pass in the harness output — asserted on `RuleOutcome` values rather than on stdout, with the projection capability set fixed by DT-3's recorded resolution rather than invented here. | `projection-suite-entry-point`, `projection-api-design-record` | AC-005 |
| `commit-atomicity-and-mutants` | `projection-mutant-registry` | capability | `CheckpointOnlyStore` fails the suite by name: a projection `REGISTRY: &[Declared]` with the same shape as the event-store one, the three exactness meta-tests (every rule has a mutant, the registry is exhaustive, each mutant fails **exactly** its declared rules), non-empty provenance per entry, ADR-0010's no-pass-rate warning in the module doc, and the stale "neither a suite nor a mutant yet" scope note replaced. | `projection-suite-entry-point` | AC-002, AC-003 |
| `commit-atomicity-and-mutants` | `commit-rollback-and-drop-rules` | capability | The commit-side rules and the store that fails each: `failed_commit_leaves_both_unchanged`, `rollback_leaves_both_unchanged`, `dropped_batch_leaves_store_usable` (drop bare, then open and commit a second batch — a pooled connection whose `Drop` returns to nothing answers `Busy` forever), `commit_rejects_a_foreign_batch` via the per-instance stamp, `commit_accepts_a_position_the_batch_did_not_write`, `commit_rejects_a_regressing_position` and `distinct_projections_advance_independently`, each registered with its mutant and never asserting a literal position. | `projection-mutant-registry` | AC-003, AC-010 |
| `reset-and-rebuild-rules` | `reset-rules` | capability | `reset` is provably one unit of work, scoped and refusable: `reset_clears_rows_and_checkpoint_together`, `reset_is_scoped_to_one_projection`, `refused_reset_changes_nothing`, `fresh_projection_has_no_checkpoint`, and `reset_is_not_commit_at_first` rejecting `commit(empty, id, FIRST)` as a substitute — with `TruncatingResetStore` and the commit-at-first substitute registered as the stores that fail them. | `projection-mutant-registry`, `projection-capability-skips` | AC-003, AC-011 |
| `reset-and-rebuild-rules` | `read-through-and-rebuild-rules` | capability | `batch_reads_reflect_pending_writes` gated on `ProjectionProbe::READS_THROUGH_BATCH` and emitted as a **reported skip** when an adapter declares `false` (CF-18), `rebuild_is_chunk_size_invariant` replaying a fixed probe sequence at chunk sizes 1, 3 and whole-log, and `rebuilding_is_distinguishable_from_live`, each with the store that fails it — including the batch `get` that answers from committed state. | `projection-mutant-registry`, `projection-capability-skips` | AC-003, AC-005 |
| `second-batch-shape-and-evidence` | `buffering-conformant-variant` | capability | The CF-5 conformant variant — a buffering, replay-at-commit store legally unlike apply-on-write `MemoryProjectionStore`, built in the testkit's own `tests/` per AC-A03 — passes the whole suite, so two structurally unlike batch shapes are green inside one `cargo xtask ci` run with both fixtures named. | `commit-rollback-and-drop-rules`, `reset-rules`, `read-through-and-rebuild-rules` | AC-004 |
| `second-batch-shape-and-evidence` | `ps3-batch-shape-finding` | capability | The PS-3 evidence written as a finding — *did the two batch shapes disagree, and where?* — including the "they agreed everywhere" outcome, which is itself the finding and not a silent success, handed to `publication-and-positioning` (HS-P0016) with no verdict on the `unstable-projection` exposure made here. | `buffering-conformant-variant` | AC-015 |
| `outside-author-extension-surface` | `documented-extension-surface` | capability | DT-8's outside-author arm, discharged as chosen: if the bar is held for an author nobody here supervises, the documented pair `projection_store_conformance!` + `ProjectionProbe` is written up as an extension surface and a fixture built from that documentation alone — not copied from `fixtures.rs` — clears the mutant-registry exactness check and the capability-skip rule; if the narrower arm was taken, the recorded scope is shown to match what the suite actually holds implementers to. | `projection-api-design-record`, `buffering-conformant-variant` | AC-007 |
| `port-disposition-and-freeze-record` | `unstable-projection-gate-and-clause-disposition` | capability | The module stops lying about its own maturity: `projection.rs`'s provisional block is replaced by an `unstable-projection` gate with a stated reason (AC-014's second arm, per AC-A04), every PS-1 – PS-37 clause carries an accurate maturity marker and rule citation, the sweep's frozen-clause repair lands as a new decision atom rather than a line edit, and `cargo xtask spec-trace` is green. | `ps-clause-pairing-sweep`, `ps3-batch-shape-finding` | AC-014 |
| `port-disposition-and-freeze-record` | `whole-gate-run-and-proof-artefact` | capability | One full `cargo xtask ci` on a clean checkout — every projection rule passing under the wasm32 emitter in the same run as the host emitters via the existing conformance-harness check step, the skeletons compiling, `cargo hack` over the widened feature powersets — with the proof artefact recorded: the name of the test `CheckpointOnlyStore` fails, and both passing batch shapes. | `unstable-projection-gate-and-clause-disposition`, `documented-extension-surface` | AC-013, AC-016 |

## Coverage

Every project AC-### is claimed by at least one story, and no two stories own the
same responsibility for one AC — where an AC appears twice, the second story
proves a different half of it (the row says which).

| Project AC | Story / stories | Note |
|---|---|---|
| **AC-001** one enumeration + orphan meta-test | `projection-suite-entry-point` | — |
| **AC-002** `CheckpointOnlyStore` fails by name, exactly | `projection-mutant-registry` | the exactness meta-tests re-assert as later stories add rules |
| **AC-003** every rule has a mutant, provenance stated, no pass rate | `projection-mutant-registry`, `commit-rollback-and-drop-rules`, `reset-rules`, `read-through-and-rebuild-rules` | registry + meta-tests in the first; each rule story carries its own mutants, which is what the exhaustiveness meta-test forces |
| **AC-004** two unlike batch shapes pass | `buffering-conformant-variant` | `MemoryProjectionStore` is the first shape, landed at `memory-projection-store` |
| **AC-005** declined capability emitted as a reasoned skip | `projection-capability-skips`, `read-through-and-rebuild-rules` | machinery and `RuleOutcome` assertion in the first; the real `READS_THROUGH_BATCH = false` instance in the second |
| **AC-006** DT-3 resolved in `_design.md` | `projection-api-design-record` | not test-provable; the design-stage review is the check |
| **AC-007** DT-8 resolved, extension surface demonstrated | `projection-api-design-record`, `documented-extension-surface` | the arm is chosen in the first, discharged in the second |
| **AC-008** ADRs accepted before the port change; open questions resolved | `ps-clause-pairing-sweep`, `projection-decision-atoms` | sweep scopes the ADRs; the ordering against `owned-batch-port-shape` *is* the check |
| **AC-009** `type Batch` has no lifetime; generic write/read-back | `owned-batch-port-shape`, `projection-probe-conformance-feature` | signature half, then seam half |
| **AC-010** dropped batch rolls back and leaves the store usable | `commit-rollback-and-drop-rules` | — |
| **AC-011** `reset` is one unit, scoped, refusable, not `commit(empty, FIRST)` | `reset-rules` | — |
| **AC-012** `MemoryProjectionStore`, doctest target, `E0195` trap | `memory-projection-store` | — |
| **AC-013** skeletons restated only; `cargo xtask ci` green | `owned-batch-port-shape`, `whole-gate-run-and-proof-artefact` | reviewed diff in the first, whole-workspace gate in the second |
| **AC-014** maturity markers accurate; `spec-trace` green | `unstable-projection-gate-and-clause-disposition` | — |
| **AC-015** PS-3 evidence recorded as a finding, no verdict | `ps3-batch-shape-finding` | — |
| **AC-016** every rule under the wasm32 emitter, same run | `projection-suite-entry-point`, `whole-gate-run-and-proof-artefact` | harness file in the first, whole-rule-set verification in the second |

No AC is orphaned, and nothing in the story set reaches outside this project's
scope: no runner, no `Projection` trait, no shipped SQL adapter, no freeze
verdict, and no `[FROZEN]` clause edited in place.

## Merge order

Slice by slice. Every foundation story lands before the capability slices that
consume it, and no story depends on substrate owned outside this initiative — the
registry, `Fixture`/`Capability`/`RuleOutcome` and the mutant registry are all
already in the tree (`crates/happenstance-testkit/src/`).

1. **`decisions-and-design-record`** — `ps-clause-pairing-sweep` → `projection-decision-atoms` → `projection-api-design-record`. The sweep first, because a systematic pairing defect and two isolated ones want different ADR scope; if it returns "systematic", report it here rather than widening ten frozen clauses later under cover of a rule-writing story.
2. **`projection-port-and-probe`** — `owned-batch-port-shape` → `projection-probe-conformance-feature` → `memory-projection-store`. Lands strictly after slice 1: AC-008's ordering is itself the check.
3. **`projection-conformance-suite`** — `projection-suite-entry-point` → `projection-capability-skips`. First green run of a projection rule against a real store.
4. **`commit-atomicity-and-mutants`** — `projection-mutant-registry` → `commit-rollback-and-drop-rules`. DoD 1 is observable at the end of this slice.
5. **`reset-and-rebuild-rules`** — `reset-rules` and `read-through-and-rebuild-rules` (independent of each other; either order within the slice).
6. **`second-batch-shape-and-evidence`** — `buffering-conformant-variant` → `ps3-batch-shape-finding`. DoD 2 is observable at the end of this slice.
7. **`outside-author-extension-surface`** — `documented-extension-surface`. Scope depends on which DT-8 arm slice 1 recorded.
8. **`port-disposition-and-freeze-record`** — `unstable-projection-gate-and-clause-disposition` → `whole-gate-run-and-proof-artefact`. The gate run is a *precondition* for reading DoD 1 and DoD 2, never a substitute for them.

Grain during implementation is `cargo xtask affected --base main` plus
`cargo xtask ci --fast`; the project boundary takes the full `cargo xtask ci`,
because AC-014 and AC-016 are proven by exactly the two steps `--fast` omits
(`spec-trace` and the mandatory wasm32 conformance-harness check).

## What would reshape this map

Named so the implement stage reports rather than absorbs it
(`_decomposition.md` Architecture brief, Note 10):

- **The sweep returns "systematic".** Then the frozen-clause repair is larger than one ADR, slice 1 grows, and this project's scope is wrong — a re-plan, not a wider ADR written quietly.
- **Note 4's emitter parameterisation cannot keep `local_conformance.rs`, `memory_conformance*.rs` and `fixture_instruments.rs` compiling.** Then `projection-suite-entry-point` takes the three-more-emitters option and says why; the story boundary is unchanged.
- **A projection rule that observes the read model without the probe.** Then PS-11 is over-built and `projection-probe-conformance-feature`'s seam should shrink — an ADR paragraph, not a quiet deletion.
- **The buffering variant passing every rule trivially.** That is `ps3-batch-shape-finding`'s content, not a defect in the map.
