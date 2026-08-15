---
item: HS-P0010
stage: review
title: "Review — Freeze ProjectionStore behind a suite that can fail"
initiative_slug: from-contract-to-published-library
project_slug: projection-store-freeze
terminal: false
created: 2026-08-15
updated: 2026-08-15
overall: 3
dod_green: true
rubric:
  ac-coverage: 3
  integration-reachability: 3
  test-integrity: 3
  gate-greenness: 3
  brief-fidelity: 2
  intent-fidelity: 2
  presentation-fidelity: 0
---

# Review — Freeze ProjectionStore behind a suite that can fail

- [x] Every project acceptance criterion (AC-001 – AC-016) is met by real, reachable, committed behaviour
- [x] Every delivered capability is mounted into a real consumer the gate reaches — nothing constructed-but-unmounted
- [x] No test gutted, skipped, `#[ignore]`d, flag-gated off, or replaced by a double
- [x] The affected-package gate is genuinely green, formatter included, re-run by this reviewer at HEAD
- [x] The work honours the briefs, `_design.md` and every Accepted KB decision
- [x] This project's applicable Definition-of-Done bar is green (`_integration.md`, `dod_green: true`)
- [ ] Presentation reviewed — **DOES NOT APPLY and was NEVER OBSERVED**; see the Rubric note

## Verdict

**approved**

## Rubric

Each dimension scored 0 (absent) → 3 (excellent). `approved` requires every dimension >= 2, with
`gate-greenness` = 3, `integration-reachability` = 3, `intent-fidelity` >= 2, and this project's
applicable Definition-of-Done bar green.

`presentation-fidelity` is the single exception to "every dimension >= 2", and here the exemption
applies in its **does not apply** form, on two independent grounds rather than one: `_design.md`
declares `surfaces: []` — this project ships a Rust type surface and one line of CI stdout, and no
screen — and `.redkiln/config.yaml` carries **no `design:` block at all**, so `design.capture` is
undeclared and `design.require_design_review` unset (blocking=false). No `_design-review.md` exists
and none was owed. The dimension therefore scores **0 and is exempt from the bar** —
**presentation was NEVER OBSERVED**. That 0 records absent evidence; it is not a finding that the
presentation is sound, and it must not be read as a passed perceptual check.

| Dimension | Score | Rationale |
| --------- | ----- | --------- |
| ac-coverage | 3 | All sixteen project ACs are met by executed behaviour with cited evidence — see the coverage map below. Each is backed by a test that fails if the behaviour regresses, and the two that are records rather than code (AC-006, AC-015) are machine-anchored: the PS-3 finding is cited from `spec/SPECIFICATION.md:4806`, so `spec-trace` resolves it on every gate run and it cannot be deleted in silence. AC-016's literal wording ("in the same `cargo xtask ci` run") holds for type-checking locally, while **execution** is the CI job `conformance on wasm32` (`.github/workflows/ci.yml:204-237`); `_integration.md` IB-8 reproduces that run at 16 passed / 0 failed / 0 ignored, and `references/evaluation/phase-6-projection-proof.md` discloses the split rather than hiding it. |
| integration-reachability | 3 | For a library the composition root is the export block plus the feature table that gates it, and both are held by tests rather than asserted. `pub mod projection;` and its re-exports are gated on `unstable-projection`, and four `xtask` unit tests fail if the gate stops gating (`xtask/src/main.rs:1003-1063`). Every deliverable has a real consumer: `ProjectionProbe` is the bound on `ProjectionFixture::Store`, so every rule drives a read model through it; `MemoryProjectionStore` backs three of four harnesses and a rendered doctest; `BufferingProjectionStore` has one definition and two consumers via `#[path]`; and the extension surface is exercised from **outside the workspace** by `examples/outside-projection-adapter`, a workspace member whose four targets the gate's `--workspace --all-features` test step builds and runs. The one thing that could have been unmounted — meta-tests whose deletion `cargo test --workspace` would not notice — is closed by two `ARTEFACTS` rows asserting every named test out of `-- --list` **before** running it (`xtask/src/proof.rs:195-226`). |
| test-integrity | 3 | No `#[ignore]`, no `test.fixme`, no flag-gated-off scenario anywhere in `crates`, `examples` or `xtask` — swept independently; the only matches are prose warning against the practice. No double stands in for a real contract: `CheckpointOnlyStore` is a real `ProjectionStore` whose entire defect is one missing `apply` call (`tests/projection_mutation_coverage/mutants.rs:49-63`), and its own doc names the one-line deletion that turns the rule green. Assertions are not fixture-pinned: `commit_is_atomic_with_the_read_model` reads both halves through **fresh handles** and compares durability of row against durability of checkpoint (`src/projection.rs:530-547`) rather than against a literal. `assert_undeclared_outcome` accepts `Skipped` only for a capability the fixture actually declines and calls anything else "a hole in the map" (`tests/projection_mutation_coverage.rs:1138-1163`). One test was removed, `both_batch_shapes_satisfy_the_same_generic_code`, and only because its second subject moved to `experiments/live-handle-projection-batch/` under ADR-0017 (`crates/happenstance-ladybug/tests/port_shape.rs:8-20`). The seventeenth rule is an **absence, not a skip**: no test exists to be ignored, and `spec/SPECIFICATION.md:8833` daggers it inside a region `spec-trace` regenerates and stale-checks. |
| gate-greenness | 3 | Re-run by this reviewer at HEAD `b4c7972`, not carried from a report. `cargo xtask ci --fast` exit 0. Then the **whole** gate, `cargo xtask ci`, exit 0 with `all checks passed` and no "optional step(s) not run" — so fmt, clippy `-D warnings` over all targets and all features, `cargo test --locked --workspace --all-features`, the proof-artefact step, all four mandatory `wasm32` steps, docs under `RUSTDOCFLAGS=-D warnings`, `spec-trace`, the file-reading lints, the `--no-default-features` doc build and `cargo package --list` all ran, **and** so did the four optional steps: both feature powersets, `cargo deny` (`advisories ok, bans ok, licenses ok, sources ok`) and the nightly `--cfg docsrs` build. The formatter is proven separately and explicitly: `cargo fmt --all --check` exits clean. Working tree clean apart from redkiln telemetry. `redkiln validate --kb` reports `validate passed`. |
| brief-fidelity | 2 | The architecture, UX and testing briefs and the signed-off `_design.md` are honoured point for point, including the expensive ones: the second batch shape was built inside the testkit rather than borrowed from `sqlite-durable-store` (AC-A03); `ProjectionProbe` sits in the contract crate behind `conformance` with no `dep:` entry (AC-A02), and the feature table states outright that the placement is void the day it acquires one; `ProjectionId::new` stays infallible (AC-A09); no borrowing GAT anywhere on the fixture; no `#[async_trait]`; `serde` untouched in `happenstance-core`; and `LiveHandleProjectionStore` was **moved to `experiments/live-handle-projection-batch/`, not deleted**, exactly as ADR-0017 decides. Nothing `[FROZEN]` was line-edited. Held off 3 by six unfixed documentation-accuracy defects, itemised under Evidence — four contradicted by an assertion this project itself committed, and two in the README `cargo package` ships to crates.io. One set was raised by the `reset-and-rebuild-rules` slice review as "the fourth occurrence in three slices" and survived its fix pass; the other is `_integration.md` Finding 1. |
| intent-fidelity | 2 | Design intent is honoured beyond the letter. The project refused three separate chances to make a number look better: it declined to widen `[FROZEN]` PS-19 by test in order to land a seventeenth rule; it states in the port's own header that PS-2's bar is **not** cleared by two instruments this workspace wrote (`crates/happenstance-core/src/projection.rs:16-24`); and it wrote the PS-3 batch-shape evidence as a finding while leaving the exposure verdict to `publication-and-positioning`. Interaction intent, read in this project's actual medium — the CI log an adapter author reads: a declined guarantee is reported **in place**, inside the run the author already started, never as a separate mode or an opt-in; it does **not occlude** the rule, which is still emitted as a test and is distinguishable from a pass by `RuleOutcome` value rather than by absence; it names **the constant the author can change** (`RESET_REFUSAL`, `COMMIT_FAULT`, `ProjectionProbe::READS_THROUGH_BATCH`), so the next action is reachable rather than guessed; and the reason **survives the constrained target**, because the wasm emitter routes `skip_line` to `console_log!` instead of a `println!` that writes nowhere (`src/projection.rs:1937-1949`). One skip vocabulary, one line shape, no projection-local skip type. Held at 2 by the surface a stranger meets first: `crates/happenstance-testkit/README.md:18` still tells the adapter-author persona this project exists to serve that the projection suite "is two rules of seventeen, and neither has been shown to reject a wrong store yet" — wrong in the *understating* direction, disclosed and routed, but it is the crates.io page of a publishable crate. |
| presentation-fidelity | 0 | **NEVER OBSERVED — the review does not apply.** `_design.md` declares `surfaces: []`; `.redkiln/config.yaml` carries no `design:` block, so `design.capture` is undeclared and `design.require_design_review` unset. Nothing was captured and no `_design-review.md` was written or owed. This 0 is the absence of perceptual evidence, never a finding that presentation is sound, and it is **exempt from the >= 2 bar** — exempt from the bar, never from the record. |

`intent-fidelity` scores BEHAVIOR from the diff; `presentation-fidelity` scores FORM from perceptual
evidence. There is none here, and saying so plainly is the point.

## Evidence

### Project AC coverage map

| AC id | Met by reachable behaviour? | Evidence |
| --- | --- | --- |
| AC-001 | Yes | One enumeration at `crates/happenstance-testkit/src/projection.rs:1843-1881` (sixteen rules); `no_orphan_projection_rules` checks **both** directions by scanning the module's own `include_str!`-baked source (`:1998-2024`); `projection_harness_parity::no_harness_lists_a_rule_by_hand` and `::each_harness_invokes_the_suite_exactly_once` stop a harness maintaining a hand list. Three negative controls recorded in that story's ledger. |
| AC-002 | Yes | `CheckpointOnlyStore` (`tests/projection_mutation_coverage/mutants.rs:44-64`) held to failing **exactly** `commit_is_atomic_with_the_read_model`, `dropped_batch_leaves_store_usable` and `distinct_projections_advance_independently` by `projection_mutants_fail_exactly_their_declared_rules` (`tests/projection_mutation_coverage.rs:1117-1136`); reproduced from outside the workspace by `outside_projection_discrimination` (3/3). |
| AC-003 | Yes | `every_projection_rule_has_a_mutant`, `projection_mutant_registry_is_exhaustive`, `every_projection_mutant_states_its_provenance` (`tests/projection_mutation_coverage.rs:972-1003`, `:1495`). Nineteen registry rows over sixteen rules; the gate prints a **denominator** (`19 registry rows`) and never a pass rate, per ADR-0010. |
| AC-004 | Yes | `MemoryProjectionFixture` (materialised delta) 16/16 and `BufferingProjectionFixture` (replayable op journal, holding no handle, transaction or lock between `begin` and `commit`) 17/17 — `tests/projection_conformance.rs`, `tests/projection_conformance_buffering.rs`. Both named in `references/evaluation/phase-6-projection-proof.md:108-131`; the second shape's provenance is the Architecture brief's Note 3 decision, not an assumption. |
| AC-005 | Yes | `require!` returns `RuleOutcome::Skipped` carrying the fixture's own `const` reason (`src/projection.rs:182-192`); asserted on **values**, never stdout, by `a_declined_reset_refusal_is_reported_with_the_fixtures_reason` (`:2048-2073`), `assert_reference_projection_declensions` (`tests/mutation_coverage.rs:3553-3576`, an equality rather than a may-skip) and `a_batch_with_no_read_path_is_reported_as_a_skip`. Repeated from outside by `outside_projection_capability_skip` (3/3). |
| AC-006 | Yes | `_design.md`, DT-3 — the suite's own output is authoritative; all three reason-writers disposed; CF-40's atom cited as authoritative rather than re-litigated, with the projection fixture declaring **no** numeric-limit constants so no second instance of the contradiction is minted. |
| AC-007 | Yes | The outside-author arm is taken in `_design.md`, DT-8, and falsified for real: `examples/outside-projection-adapter`, written from rendered rustdoc with the reference fixture on a written denylist, clears `outside_projection_conformance` 16/16, `outside_projection_capability_skip` 3/3 and `outside_projection_discrimination` 3/3. Store and both trait impls live in `src/` because `tests/` is a different crate — the orphan-rule argument that put `ProjectionProbe` in the contract crate, now measured rather than claimed. |
| AC-008 | Yes | `.kb/decisions/0017-*.md`, `0018-*.md`, `0019-*.md` accepted at `493a194`, which precedes the port change at `2eade38`. `redkiln validate --kb` reports `validate passed`. `git diff --diff-filter=D` over `.kb` is **empty**: the apply-seam question is `status: superseded`, and `ps-1-*` / `ps-19-*` stay `accepted` with their dispositions written into the body — resolved in place, never deleted. |
| AC-009 | Yes | `type Batch;` with no lifetime (`crates/happenstance-core/src/projection.rs:437`); generic rules write through `probe_write` and read back out of band through `probe_read` and a fresh handle, never an inherent method on a concrete store. `crates/happenstance-core/tests/projection_probe_round_trip.rs` is the standalone round-trip. |
| AC-010 | Yes | `dropped_batch_leaves_store_usable`, backed by a registered mutant whose declared failure is exactly that rule (`tests/projection_mutation_coverage.rs:451-459`) — a store that answers `Busy` after a bare drop. The rule opens a **second** batch on the same handle and commits it. |
| AC-011 | Yes | Four rules land — `reset_clears_rows_and_checkpoint_together`, `reset_is_scoped_to_one_projection`, `refused_reset_changes_nothing`, `reset_is_not_commit_at_first` (`src/projection.rs:1866-1873`) — each with its own registered wrong store, including `TruncatingResetStore` and the `commit(empty, id, FIRST)` substitute. |
| AC-012 | Yes | `MemoryProjectionStore` behind `memory` (`crates/happenstance-core/src/projection_memory.rs`), the doctest target, rendered green in the gate's doc step. The `E0195` trap is disposed of *by omission* in a compiling walkthrough on the port's own page (`src/projection.rs:305-417`): there is no `Self::Batch<'_>` to spell, and the doc says that is the point. |
| AC-013 | Yes | Reviewed diff: `happenstance-postgres` keeps `Transaction<'static, Postgres>`, `happenstance-ladybug` keeps `GraphWriteSet`, both keep their error types, and every affected body is still `todo!()`; the only change is the compelled signature restatement plus deletion of `where Self: 'a`. `LiveHandleProjectionStore` moved to `experiments/`, per ADR-0017. `cargo xtask ci` green on the workspace in this review's own run. |
| AC-014 | Yes | The module is behind `unstable-projection` and says why in the terms PS-2 sets (`crates/happenstance-core/src/projection.rs:3-42`). Four `xtask` guards fail if the gate stops gating (`xtask/src/main.rs:1003-1063`). `cargo xtask spec-trace` green in this run; §7.2's PS rows carry accurate markers with `†` regenerated against actual rule existence — `spec/SPECIFICATION.md:8833` daggers the held rule rather than pretending it exists. |
| AC-015 | Yes | `references/evaluation/projection-batch-shape-evidence.md`, indexed at `references/evaluation/README.md:91-112` and cited from PS-3's clause body at `spec/SPECIFICATION.md:4806`, so `spec-trace` resolves it every run. It records the finding and makes no exposure verdict. |
| AC-016 | Yes | One enumeration, three emitters, no per-harness list (AC-001's parity tests). The mandatory `wasm32` conformance-harness step type-checks `projection_conformance_wasm.rs` in every gate run; execution is the CI job `conformance on wasm32` (`.github/workflows/ci.yml:204-237`), reproduced in `_integration.md` IB-8 as **16 passed, 0 failed, 0 ignored** under `wasm-bindgen-test-runner`. The local gate's type-check-only limit is disclosed in the proof artefact, not papered over. |

### Gate and Definition-of-Done results

Re-run by this reviewer at HEAD `b4c7972`, from the worktree root:

- `cargo xtask ci --fast` — **exit 0**, `all required checks passed (--fast: 4 optional step(s) not run)`.
- `cargo xtask ci` (whole) — **exit 0**, `all checks passed`; `cargo deny` reported
  `advisories ok, bans ok, licenses ok, sources ok`; the nightly `--cfg docsrs` build rendered
  `happenstance-core` and `happenstance-testkit`.
- `cargo fmt --all --check` — **clean**. The formatter is non-negotiable and is proven here rather
  than inferred from the gate's own fmt step.
- `redkiln validate --kb` — `validate passed`.
- Working tree clean apart from `.redkiln/telemetry/**`.

Definition of Done: this project is **non-terminal**, so `dod_green: true` in
`.bklg/from-contract-to-published-library/projection-store-freeze/_integration.md` is this project's
own integration bar, and the fifteen whole-initiative journeys it does not own are listed under
`deferred_scenarios` with a named owner each. Confirmed: `reachability_ok: true`; scenario rows
IB-1 to IB-15 all `executed` / `PASS`; and **no scenario this project owns is `test.fixme`d,
`#[ignore]`d or flag-gated off** — I swept `crates`, `examples` and `xtask` independently, and the
only `#[ignore]` matches are prose warning against it. The one whole-initiative item this project
owns outright, **DoD 7, was executed and passed in both halves**: the wrong store convicted **by
name** (`commit_is_atomic_with_the_read_model`) and the two batch shapes named individually.

### Escape-hatch hunt — nothing found, and three near-misses that went the right way

I looked specifically for the patterns that make a green suite meaningless, and found none:

1. **No double or no-op standing in for a real contract.** Every mutant and every conformant variant
   is a real `ProjectionStore` implementation built as a single overridden step over a shared
   correct-steps module (`tests/projection_mutation_coverage/correct.rs`), never a stub configured
   to fail on cue.
2. **No fixture-pinned assertion.** Rules read back through fresh handles and compare against what
   the store actually assigned; no literal position values anywhere.
3. **No skip standing in for a pass.** `must!` **panics** for `SECOND_HANDLE` rather than skipping,
   and the exactness meta-test rejects any `Skipped` the registry does not account for.

Three moments where a weaker project would have gamed it and this one did not, recorded because a
reviewer should say when the answer taken was the harder one:

- The seventeenth rule, `fresh_projection_has_no_checkpoint`, was **withdrawn because it convicted a
  conformant store** — a missing row resolved as `Live { through: FIRST }` breaks no frozen `MUST` —
  not because it was red. Restoring it would have widened `[FROZEN]` PS-19 *by test*, which
  `CLAUDE.md` and `.kb/playbooks/repairing-a-frozen-clause-without-amending-it.md` both forbid.
- PS-2's freeze bar is stated as **not met**, in the port's own header, against the temptation to
  read "two shapes passed" as "the port is frozen".
- The PS-3 evidence is written as a finding, with the verdict explicitly left to
  `publication-and-positioning`.

### Findings — none blocking, all cited, all routed

1. **Six documentation-accuracy defects, four of them contradicted by an assertion this project
   committed.** `assert_reference_projection_declensions`
   (`crates/happenstance-testkit/tests/mutation_coverage.rs:3561-3566`) pins the reference run's
   skip set to **exactly two** rules, `failed_commit_leaves_both_unchanged` and
   `refused_reset_changes_nothing`. Four doc sites still describe one:
   `crates/happenstance-testkit/tests/projection_conformance.rs:24` ("One rule in this run is
   answered by a `SKIP`"); `crates/happenstance-testkit/src/fixtures.rs:461-462` ("prints one
   `SKIP` line for `failed_commit_leaves_both_unchanged`");
   `crates/happenstance-testkit/src/fixtures.rs:442-443`, still future tense — "the day that rule
   lands" — for a rule that landed at `10ace94`; and
   `crates/happenstance-testkit/src/projection.rs:2030-2032`
   ("`refused_reset_changes_nothing` is the only thing it can answer with a skip"). The
   `reset-and-rebuild-rules` slice review raised this as its finding 2, called it "the fourth
   occurrence in three slices", and asked that these docs point at the assertion instead of
   restating a count; it survived that slice's fix pass. Two more sit in the file `cargo package`
   ships to crates.io: `crates/happenstance-testkit/README.md:18` and `:127` still say the
   projection suite "is two rules of seventeen, and neither has been shown to reject a wrong store
   yet" and that the hostile stores are "not yet written" — false on all three counts now, with
   nineteen of them in `tests/projection_mutation_coverage/mutants.rs`. `_integration.md` Finding 1
   already routes it to `publication-and-positioning` (HS-P0016), which owns initiative DoD 10.
   **Not blocking**: no project AC covers a prose count, every error is in the *understating*
   direction, and the machine-checked half is right. It is nevertheless a quality-bar regression
   carried across the project boundary, and the standing lesson — `CHANGELOG.md:1430-1437`, *"the
   gate asserts the file is present in the package, not that it is true"* — is now owed a gate step
   rather than a fifth hand correction.
2. **One slice is sealed `changes-requested` and cannot be cleared inside this project.**
   `_slices.md` records `reset-and-rebuild-rules` as `changes-requested`, its blocking finding being
   `reset-rules` AC-005 — the held seventeenth rule. This is a **carried, human-adjudicated hold**,
   not a dropped responsibility: the human was offered path (b), proceeding on the unmet
   precondition with a named authorisation, and **refused it** (`_slices.md:355-379`). The repair is
   a new accepted decision atom that only a human-invoked `/redkiln:kb-ingest` wave can produce; it
   is staged at `.kb/_intake/2026-08-15-adr-0030-checkpoint-progress.md`, with its long form at
   `references/adr/0030-the-checkpoint-reports-the-commits-that-happened.md`. Bouncing this project
   back cannot resolve it — the story would re-halt on the same finding — and the only alternative
   is the frozen-clause widening the whole halt exists to prevent. The non-delivery is disclosed at
   every point it could be missed: `CHANGELOG.md`, §7.2's `†` on PS-19 and PS-38,
   `crates/happenstance-core/src/projection.rs:13-14`, `crates/happenstance-testkit/src/lib.rs:257`,
   and `crates/happenstance-testkit/src/projection.rs:1423` and `:1869` where the rule would sit.
   Recorded as project-grain scope drift and handed forward, not held against this gate.
3. **`redkiln doctor` exits with nine `unconsumed-foundation` problems, and CI's `backlog` job
   asserts that list is empty** (`.github/workflows/ci.yml:176-187`). One is this project's
   (`projection-decision-atoms`, HS-S0002); eight belong to five projects that have not started. All
   nine story files were created by the planning commit `ae77ac4`, which I confirmed is an
   **ancestor of this project's base** `2136ddeb` — so this is a pre-existing decomposition-shape
   condition, not a regression this project introduced, and no out-of-band baseline repair was made
   this run. Initiative-level; not this project's to settle in passing.
4. **`_design.md` shows the outside author taking `happenstance-core` under `[dev-dependencies]`.**
   The landed manifest correctly puts it under `[dependencies]`
   (`examples/outside-projection-adapter/Cargo.toml`), which is the only spelling that works,
   because the `ProjectionProbe` impl lives in `src/`. The shipped surface documents it correctly
   (`crates/happenstance-testkit/src/lib.rs:184-191`); the design record is the outlier, superseded
   in fact. Already recorded as `_integration.md` Finding 3.

### Per-story checkpoint SHAs

Derived from git rather than from the seed.
`git log 2136ddeb..HEAD --grep "Story: projection-store-freeze/"` returns **twenty-one** commits,
oldest first:

| # | SHA | Story checkpoint |
| --- | --- | --- |
| 1 | `f77f183` | Sweep the PS clause range for the pairing defect |
| 2 | `9520b28` | The three projection decision atoms |
| 3 | `0df2c1c` | The projection API design record |
| 4 | `2eade38` | The owned-batch port shape |
| 5 | `cb495ee` | ProjectionProbe behind a conformance feature |
| 6 | `5fd62c6` | MemoryProjectionStore, the oracle |
| 7 | `79df6b7` | The projection suite entry point |
| 8 | `7fcb378` | A declined capability is not a pass |
| 9 | `c385e40` | The mutant registry |
| 10 | `5d9b4fd` | Commit, rollback and drop rules |
| 11 | `d9eb8de` | The slice review's three findings, and PS-1's second conjunct |
| 12 | `10ace94` | Reset is one unit of work |
| 13 | `5be22ab` | Read-through and rebuild rules |
| 14 | `cfd9231` | The second, unlike batch shape |
| 15 | `d9cfb0e` | The PS-3 batch-shape finding |
| 16 | `d6496cd` | The outside-author extension surface |
| 17 | `984e7fd` | The unstable-projection gate and clause disposition |
| 18 | `2a7ae8a` | Record the story checkpoint's own SHA |
| 19 | `7620481` | The whole gate, and the proof artefact |
| 20 | `674c459` | The proof artefact, recorded and registered |
| 21 | `501fb89` | Record the evidence checkpoint's own SHA |

Reconciliation with the workflow-reported list, resolved in git's favour: git additionally carries
`5d9b4fd`, `2a7ae8a`, `7620481` and `501fb89`, while the workflow list's `6ce1cf3`, `56b17b7` and
`e76f363` are **slice-seal** commits (`Slice-Verdict:` trailers), not story checkpoints. The nine
slice seals are `560bb4b`, `0ccf16e`, `f2f7871`, `42144d2`, `7d54243`, `41ec9d1`, `6ce1cf3`,
`56b17b7` and `e76f363`; the integration audit is `b4c7972`.

**Baseline repairs: none.** `git log --grep "Baseline-Repair|Slice-Repair"` over the project range
returns nothing, matching this run's declaration that no out-of-band baseline repair occurred. The
in-slice fix passes — `cc0f158`, `b7c1600`, `fb4161c`, `064687a`, `d9eb8de` — are review-driven work
inside a slice's own declared boundary, and are expected rather than drift.

## Required Changes

None block this gate; the verdict is **approved**. Three follow-ups are recorded so they are carried
rather than rediscovered, and each already has an owner.

1. **Correct the six stale doc sites, and stop counting in prose where a test already counts.**
   `crates/happenstance-testkit/tests/projection_conformance.rs:24`,
   `crates/happenstance-testkit/src/fixtures.rs:442-443` and `:461-462`, and
   `crates/happenstance-testkit/src/projection.rs:2030-2032` should name **both** declined
   capabilities and point at `assert_reference_projection_declensions` as the authority for the set;
   `crates/happenstance-testkit/README.md:18` and `:127` should stop telling crates.io that the
   projection suite is two rules of seventeen that have never rejected a wrong store. Cheaper here
   than at publish. The durable fix is the one `_integration.md` names: a gate step reading a
   README's counts against the enumeration, which retires the class instead of correcting its fifth
   instance. Owner: `publication-and-positioning` (HS-P0016), which owns initiative DoD 10.
2. **Land ADR-0030 through one human-invoked `/redkiln:kb-ingest` wave**, then restore
   `fresh_projection_has_no_checkpoint` and `PresumedLiveCheckpointStore` and re-seal
   `reset-and-rebuild-rules`. The intake document is already staged at
   `.kb/_intake/2026-08-15-adr-0030-checkpoint-progress.md`. Do **not** restore the rule before that
   atom is `status: accepted`; and when it lands, add the conformant variant modelling a missing row
   resolved as `Live { through: FIRST }`, so CF-5's positive control can catch the next rule that
   convicts a legal adapter.
3. **Re-shape the decomposition so `redkiln doctor`'s problem list empties**, or the `backlog` CI job
   stays red on this branch. Initiative-level: nine foundation stories across six projects.
