---
item: HS-P0010
stage: review
title: "Review — Freeze ProjectionStore behind a suite that can fail"
initiative_slug: from-contract-to-published-library
project_slug: projection-store-freeze
terminal: false
created: 2026-08-15
updated: 2026-08-15
overall: 2
dod_green: true
rubric:
  ac-coverage: 2
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
- [x] The gate is genuinely green, formatter included — `cargo xtask ci` **whole**, not `--fast`, re-run at HEAD `b83ce3c`
- [x] This project's applicable Definition-of-Done bar is green (`_integration.md`, `dod_green: true`)
- [x] **The repairs `_integration.md` declares owed by this project before closeout have been made** — all four, one checkpoint each
- [ ] Presentation reviewed — **DOES NOT APPLY and was NEVER OBSERVED**; see the Rubric note

## Verdict

**approved**, at HEAD `b83ce3c`, after the four required changes below were made
as four checkpoints and the whole gate was re-run.

The original verdict was `changes-requested` and it is left standing below,
because the reason it was given is the reason the repairs exist. It was never
about a capability being missing, faked or unmounted: everything this project
built is real, reachable and held by tests that a wrong store fails by name —
the discriminator, both batch shapes, the outside-workspace fixture and the
`wasm32` emitter were all re-run and all green at review time and again now. The
bounce was mechanical: **this project's own integration audit named two repairs
"owed by this project before closeout" (`_integration.md:188`, `:210-212`), this
review is that closeout, and neither had been made.** Four documents inside the
project's boundary contradicted the code it had committed, and the gate could not
see any of them.

**What closed it, in order:**

| # | Checkpoint | What it repaired |
| --- | --- | --- |
| 1 | `a930e12` | FR-1 — the three authored sentences in `spec/SPECIFICATION.md` that said `fresh_projection_has_no_checkpoint` does not exist, now agreeing with the generated §7.2 that lists it against PS-19 and PS-38 with no dagger. Nothing normative moved: both MUSTs, both maturity markers, every **Cases:** and **Rejects:** field and the whole generated region are byte-identical |
| 2 | `d0d3db2` | The thirteen `file:line` citations that moved because checkpoint 1 added six lines — repointed by content, not by arithmetic, in the testkit and the runbook. `.kb/` atoms and `references/` documents are deliberately untouched, being dated records under a supersede-rather-than-edit rule |
| 3 | `263b7e6` | FR-3 — the two false README sentences, **and the class**: a new mandatory gate step, `cargo xtask lint-rule-counts`, written red first against the exact sentences the review named, which then found the same stale claim in three more shipped documents in three file formats |
| 4 | `04b1f4d` | FR-4 — `_design.md`'s DT-8 now spells the extension surface the way it ships, with `happenstance-core` under `[dependencies]`, matching the landed manifest and the rendered rustdoc |
| 5 | `b83ce3c` | FR-2 — `references/evaluation/phase-6-projection-proof-at-closeout.md`, pinned to `04b1f4d`, superseding the artefact pinned to `7620481` by name rather than editing it, with `xtask/src/proof.rs`'s guards repointed at it |

The one thing worth calling out as more than paperwork is checkpoint 3. The
review's Required Change 3 offered a durable fix as a suggestion; taking it is
what turned four correction sites into a check. `cargo xtask lint-rule-counts`
reads the four documents a consumer meets — the crates.io README, the crate
page, the feature list and the manifest comment beside the feature — and fails
when a cardinal qualifying `rule` or `rules` in a paragraph about the suite is
not a count the rule files actually have. It is derived from `collect_rules`, the
same parse CF-29 and `spec-trace` use, so landing a rule moves the bar with no
edit to the check. Its first run reproduced the audit's finding at `:16` and
`:127` and then convicted three documents nobody had looked at.

## Rubric

Each dimension scored 0 (absent) to 3 (excellent). `approved` requires every
dimension to score at least 2, with `gate-greenness` = 3,
`integration-reachability` = 3, `intent-fidelity` at least 2, this project's
applicable Definition-of-Done bar green, **and** no scope drift or carried
quality-bar regression. Every condition is now met; the last one was what the
original verdict turned on, and the five checkpoints above are what discharged it.

**The scores below are the post-repair scores; the frontmatter is not.** This
pass was permitted to write the body of this artifact and forbidden to touch its
YAML, so `overall: 2` and the `rubric:` block at the top of the file still carry
the original review's numbers. They are stale by design, not by oversight. The
values a re-review would carry are `overall: 3` with `ac-coverage: 3`,
`brief-fidelity: 3` and `intent-fidelity: 3`; `integration-reachability`,
`test-integrity` and `gate-greenness` are unchanged at 3 and
`presentation-fidelity` unchanged at 0-and-exempt.

`presentation-fidelity` is the single exception to "every dimension at least 2",
and here the exemption applies in its **does not apply** form, on two independent
grounds. `_design.md` declares an empty surface manifest — *"No user-facing
surface. This project ships nothing a person looks at or clicks"*
(`_design.md:11-12`) — and `.redkiln/config.yaml` carries **no `design:` block at
all** (`.redkiln/config.yaml:75-84`, where its absence is deliberate and
explained), so `design.capture` is undeclared and the perceptual review is a
declared skip rather than a silent pass. No `_design-review.md` exists and none
was owed. The dimension therefore scores **0 and is exempt from the bar** —
exempt from the BAR, never from the RECORD. **Presentation was NEVER OBSERVED.**
That 0 records the absence of perceptual evidence; it is not a finding that the
presentation is sound, and it must not be read as a passed perceptual check.

| Dimension | Score | Rationale |
| --------- | ----- | --------- |
| ac-coverage | 2 → **3** | All sixteen project ACs are met by executed, reachable behaviour — the coverage map below cites a run or a `file:line` for each, and I executed four of them myself rather than reading the report. It was held off 3 by one AC's *surface*: AC-014 owns the PS-1 to PS-37 clause disposition (`project.md:225-228`, scope item 9 at `:104-110`), and the specification stated in three authored places that `fresh_projection_has_no_checkpoint` is unwritten while its own generated §7.2 listed it against PS-19 and PS-38 with no dagger. `a930e12` repaired all three — PS-19's **Rule:** line and its recorded phase-6 answer, and PS-38's **Rule:** line — and rewrote §4.11's opener to point at `for_each_projection_store_rule` (`crates/happenstance-testkit/src/projection.rs:1884-1919`) instead of restating a count nothing in the gate reads. The new citation is one `spec-trace` checks: **380 citations** where the review's own run checked 379. Nothing normative moved, which is the part that matters for a `[FROZEN]` clause: PS-19's MUST, its marker, its **Cases:** and **Rejects:** fields and the whole generated region are byte-identical. |
| integration-reachability | 3 | Verified independently, not read off the reachability table. For a library the composition root is the export block plus the feature gate plus the gate that compiles and runs it, and all three are held by tests. `pub mod projection;` and its re-exports are cfg-gated on `unstable-projection` (`crates/happenstance-core/src/lib.rs:128-129,160-161,175-178`), and four `xtask` unit tests read the manifests and `lib.rs` and fail if the gate stops gating (`xtask/src/main.rs:1003-1063`). Every deliverable has a real consumer: `ProjectionProbe` is the bound on `ProjectionFixture::Store`, so all seventeen rules drive a read model through it; `MemoryProjectionStore` backs three of four harnesses and two rendered doctests; `BufferingProjectionStore` has one definition and two consumers via `#[path]`; and the extension surface is exercised **from outside the workspace** by `examples/outside-projection-adapter`, whose targets I built and ran (`outside_projection_conformance` 17/17, `outside_projection_discrimination` 3/3). Nothing is constructed-but-unmounted or reachable only through a test nothing holds — the one thing that could have been, the meta-test binaries, is closed by three `ARTEFACTS` rows asserting every named test out of `--list` before running it (`xtask/src/proof.rs:195-213`). |
| test-integrity | 3 | No `#[ignore]`, no `test.fixme`, no flag-gated-off scenario anywhere in `crates`, `examples` or `xtask` — I swept independently and the only five matches are prose *warning against* the practice (`crates/happenstance-testkit/tests/projection_harness_parity.rs:102-103`, `xtask/src/proof.rs:19,185,478`). No double stands in for a real contract: every mutant is a real `ProjectionStore` built as a single overridden step over a shared correct-steps module, and `CheckpointOnlyStore`'s entire defect is one missing `apply` call whose own comment names the one-line deletion that turns the rule green (`crates/happenstance-testkit/tests/projection_mutation_coverage/mutants.rs:44-64`). No assertion is fixture-pinned: `fresh_projection_has_no_checkpoint` asserts the `Checkpoint::NeverRun` **variant** with no position compared anywhere (`crates/happenstance-testkit/src/projection.rs:1467-1489`), and the exactness meta-test holds each of the eighteen mutants to failing *exactly* its declared rules. `git diff --diff-filter=D` over the whole project range is **empty** — nothing was deleted; `live_handle.rs` is a rename into `experiments/live-handle-projection-batch/` with a zero-line diff, which is ADR-0017's decision and preserves the only compiled evidence against the port's own shape. |
| gate-greenness | 3 | Re-run at HEAD `01a99e9` at review time and again at HEAD `b83ce3c` after the repairs, and the second run is the **whole** gate rather than the project's `--fast` bar: `cargo xtask ci` — **exit 0**, closing line *"all checks passed"*, with **four of four `OPTIONAL` steps ran and none skipped** (`cargo hack` feature powerset, the `wasm32` powerset, `cargo deny`, and the nightly `--cfg docsrs` build). That covers fmt, clippy `-D warnings`, `cargo test --locked --workspace --all-features`, the proof-artefact step, all four mandatory `wasm32` steps, docs under `RUSTDOCFLAGS=-D warnings`, `spec-trace`, the now-**six** file-reading lints, both `--no-default-features` and default-feature doc builds, and `cargo package --list` over the three publishable crates. The formatter is proven separately and explicitly, as non-negotiable: **`cargo fmt --all --check` exits clean**. `redkiln validate --kb` reports *"validate passed"*. Working tree clean. No Playwright specs exist in this repository, so the collection-only `--list` pass does not apply. |
| brief-fidelity | 2 → **3** | The architecture, UX and testing briefs and the signed-off `_design.md` are honoured on every expensive point: the second batch shape was built inside the testkit rather than borrowed from `sqlite-durable-store`; `ProjectionProbe` sits in the contract crate behind `conformance` so an outside author pays one flag and no new graph edge (AC-U02, proved by `examples/outside-projection-adapter/Cargo.toml:20-23`); `Checkpoint` ships as the three-variant enum rather than a tuple (AC-U03); `CommitError` and `ResetError` stay two (AC-U04); one skip vocabulary and one line shape, routed to `console_log!` on `wasm32` (AC-U08, AC-U11). No `#[async_trait]` (ADR-0001), no `serde` in `happenstance-core` (ADR-0003), no borrowing GAT on the fixture, and nothing `[FROZEN]` line-edited — PS-19 is byte-identical across the whole ADR-0030 episode. All four decision atoms are `status: accepted` and `redkiln validate --kb` passes. It was held off 3 because the signed-off design record contradicted the shipped surface: `_design.md`'s DT-8 showed the outside author taking `happenstance-core` under `[dev-dependencies]`, which cannot work, because the `ProjectionProbe` impl lives in `src/` — the argument the same record makes two paragraphs earlier and the shipped rustdoc gets right (`crates/happenstance-testkit/src/lib.rs:184-191`). `04b1f4d` repaired it in the record's own body: `happenstance-core` under `[dependencies]` with the feature, the testkit under `[dev-dependencies]`, the `src/` placement stated, and the correction dated in place rather than swapped in silently. The resolution, its cost bound and the obligation on `documented-extension-surface` are unchanged — this was the wrong spelling of a decision, never a different one. |
| intent-fidelity | 2 → **3** | Design intent is honoured well past the letter, and in three places the project took the harder answer when a cheaper one was available and would have looked identical in a gate log. It refused to widen `[FROZEN]` PS-19 by test to land a seventeenth rule, halted instead, and resumed only when ADR-0030 reached `status: accepted` through a human-invoked ingest wave and minted PS-38. It states in the port's own header that PS-2's freeze bar is **not** cleared by two instruments this workspace wrote. It wrote the PS-3 batch-shape evidence as a finding and left the exposure verdict to `publication-and-positioning`. Interaction intent, read in this project's actual medium — the CI log an adapter author reads — is likewise honoured: a declined guarantee is reported **in place**, inside the run the author already started; it does **not occlude** the rule, which is still emitted as a test and is distinguishable from a pass by `RuleOutcome` value rather than by absence; it names **the constant the author can change** (`RESET_REFUSAL`, `COMMIT_FAULT`, `ProjectionProbe::READS_THROUGH_BATCH`), so the next action is reachable rather than guessed; and the reason survives the constrained target. It was held at 2, not 3, by the document that same author meets *before* the run: `crates/happenstance-testkit/README.md:16-20` and `:127` — the file `cargo package` ships to crates.io — told them the projection suite *"is two rules of seventeen, and neither has been shown to reject a wrong store yet"*, a sentence authored **by this project** at `79df6b7` and falsified **by this project** fifteen rules later. `263b7e6` corrects both, and then does the thing that moves this to 3: it makes the class a build failure instead of an audit finding. The same intent that put a declined capability's reason *in the CI log the author already started* now applies to the page the author reads before starting — and the check was written **red first**, against the shipped sentence quoted verbatim across its own line wrap, before either sentence was touched. |
| presentation-fidelity | 0 | **NEVER OBSERVED — the review does not apply.** `_design.md:11-12` declares an empty surface manifest; `.redkiln/config.yaml` carries no `design:` block, so `design.capture` is undeclared. Nothing was captured, no `_design-review.md` was written and none was owed. This 0 is the absence of perceptual evidence, never a finding that presentation is sound, and it is **exempt from the at-least-2 bar** — exempt from the bar, never from the record. |

`intent-fidelity` scores BEHAVIOUR from the diff; `presentation-fidelity` scores
FORM from perceptual evidence. There is none here, and saying so plainly rather
than scoring the silence as a pass is the point of the dimension.

## Evidence

### Project AC coverage map

| AC id | Met by reachable behaviour? | Evidence |
| --- | --- | --- |
| AC-001 | Yes | One enumeration — `for_each_projection_store_rule!` at `crates/happenstance-testkit/src/projection.rs:1884-1919`, seventeen rules. `no_orphan_projection_rules` checks both directions over the module's own `include_str!`-baked source; `projection_harness_parity::no_harness_lists_a_rule_by_hand` and `::each_harness_invokes_the_suite_exactly_once` (2/2, re-run by me) stop a harness maintaining a hand list. |
| AC-002 | Yes | `CheckpointOnlyStore` (`crates/happenstance-testkit/tests/projection_mutation_coverage/mutants.rs:44-64`) held to failing **exactly** `commit_is_atomic_with_the_read_model`, `dropped_batch_leaves_store_usable` and `distinct_projections_advance_independently` by `projection_mutants_fail_exactly_their_declared_rules` (`tests/projection_mutation_coverage.rs:300-330`). Reproduced from outside the workspace: `outside_projection_discrimination` 3/3, re-run by me, including the positive control `..._passes_the_conformant_store`. |
| AC-003 | Yes | `every_projection_rule_has_a_mutant`, `projection_mutant_registry_is_exhaustive`, `every_projection_mutant_states_its_provenance` — 7/7 in `projection_mutation_coverage`, re-run by me. Eighteen mutant stores plus three conformant variants make 21 registry rows over seventeen rules; the gate prints a **denominator**, never a pass rate, per ADR-0010. |
| AC-004 | Yes | Shape A `MemoryProjectionFixture` (materialised delta): `projection_conformance` 17/17 with two reported skips. Shape B `BufferingProjectionFixture` (replayable op journal, holding no handle, transaction or lock between `begin` and `commit`): `projection_conformance_buffering` **18/18, no skip**, including `the_read_model_changes_only_at_commit`. Both re-run by me. Shape B's provenance is the architecture brief's decision to build it inside the testkit, not an assumption; `the_second_batch_shape_answers_every_rule_with_a_pass` requires the outcome set to equal the enumeration, in order. |
| AC-005 | Yes | `RuleOutcome::Skipped` carrying the fixture's own `const` reason, asserted on **values** and never on stdout: `a_batch_with_no_read_path_is_reported_as_a_skip`, `projection_capability_skips_are_reported`, `projection_capability_reasons_are_authored_once`, and from outside the workspace `outside_projection_capability_skip` 3/3. |
| AC-006 | Yes | `_design.md` DT-3 — the suite's own output is authoritative, all three reason-writers disposed, CF-40's atom cited as authoritative rather than re-litigated, and the projection fixture declares no numeric-limit constants so no second declension policy is minted. |
| AC-007 | Yes | The outside-author arm is taken in `_design.md` DT-8 and falsified for real. `examples/outside-projection-adapter`, written from the rendered documentation, clears `outside_projection_conformance` **17/17** (16 pass, one skip carrying its *own* `RESET_REFUSAL` reason), `outside_projection_capability_skip` 3/3 and `outside_projection_discrimination` 3/3 — all re-run by me at HEAD. Both trait impls live in `src/`, which is the orphan-rule argument that put `ProjectionProbe` in the contract crate, now measured rather than claimed. |
| AC-008 | Yes | `.kb/decisions/0017-what-a-projection-batch-owns.md`, `0018-returning-a-projection-to-never-run.md` and `0019-what-happens-when-apply-fails.md` are all `status: accepted` at `493a194`, which precedes the port change at `2eade38`. `redkiln validate --kb` reports *"validate passed"*. `git diff --diff-filter=D` over the range is empty: the open questions are `superseded`, never deleted. |
| AC-009 | Yes | `type Batch;` with no lifetime (`crates/happenstance-core/src/projection.rs:436`); generic rules write through the probe and read back out of band through a fresh handle, never through an inherent method on a concrete store. `crates/happenstance-core/tests/projection_probe_round_trip.rs` is the standalone round-trip. |
| AC-010 | Yes | `dropped_batch_leaves_store_usable`, backed by `PooledConnectionStore`, whose declared failure is exactly that rule — a store that answers `Busy` after a bare drop. The rule opens a **second** batch on the same handle and commits it. |
| AC-011 | Yes | Five reset rules in the one enumeration — `reset_clears_rows_and_checkpoint_together`, `reset_is_scoped_to_one_projection`, `refused_reset_changes_nothing`, `fresh_projection_has_no_checkpoint`, `reset_is_not_commit_at_first` — each with a registered wrong store, including `TruncatingResetStore` and `CommitAtFirstResetStore`, the `commit(empty, id, FIRST)` substitute. |
| AC-012 | Yes | `MemoryProjectionStore` behind `memory` **and** `unstable-projection` (`crates/happenstance-core/src/lib.rs:139-144,175-182`; `crates/happenstance-core/src/projection_memory.rs`), the doctest target; six projection doctests compiled and ran green in the gate. The `E0195` trap is disposed of by a compiling walkthrough on the port's own page. |
| AC-013 | Yes, on the AC's stated bar | `happenstance-ladybug` keeps `GraphWriteSet` and `LadybugProjectionStoreError`; `happenstance-postgres` keeps `Transaction` over `Postgres`; every affected body is still `todo!()`. Beyond the lifetime's removal the signatures were restated for the write seam this project's scope item 2 required (`begin` non-`async`, `Checkpoint`, `Authority`, `CommitError`, `reset`) — compelled by the port's growth, disclosed at `project.md:308-311`, and not a change of `Batch` type, error type or body. `cargo xtask ci --fast` exit 0 on the workspace in this review's own run. |
| AC-014 | **Yes, after `a930e12`** | The module is behind `unstable-projection` and says why; four `xtask` guards fail if the gate stops gating. `cargo xtask spec-trace` is green — *"201 clauses (139 FROZEN, 50 PROVISIONAL, 10 DEFERRED, 2 NON-NORMATIVE), 112 conformance rules, 58 e2e cases, 380 citations checked (70 anchored to their subject, 12 external)"* — and §7.2's markers are accurate. The three authored statements that asserted the seventeenth rule was unwritten are repaired, and the word `new` is gone from both **Rule:** lines where it carried the document's own convention for *does not exist yet* — six other clauses spell an unwritten rule that way and every one of them is still daggered in §7.2, so the convention survives the repair rather than being quietly abandoned. The disposition a consumer reads now says what the generated table says. |
| AC-015 | Yes | `references/evaluation/projection-batch-shape-evidence.md` exists, is indexed in `references/evaluation/README.md`, and is cited from PS-3's clause body in `spec/SPECIFICATION.md`, so `spec-trace` resolves it on every gate run and it cannot be deleted in silence. It records the finding and makes no exposure verdict. |
| AC-016 | Yes, with the disclosed split | **Executed by this reviewer**: `CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_RUNNER=wasm-bindgen-test-runner cargo test --locked -p happenstance-testkit --target wasm32-unknown-unknown --test projection_conformance_wasm` gave **17 passed, 0 failed, 0 ignored**, all seventeen rules named individually. The AC's literal wording ("in the same `cargo xtask ci` run") holds only for *type-checking* locally — `xtask/src/main.rs:231-241` is `cargo check --tests --target wasm32` — while **execution** is CI's `conformance on wasm32` job (`.github/workflows/ci.yml:204-237`). The split is disclosed in the proof artefact rather than papered over, and the run above discharges it. |

### Gate and Definition-of-Done results

**Post-repair, at HEAD `b83ce3c`, whole gate rather than the project's `--fast`
bar** — because two of the four repairs touch paths the gate compiles and one of
them adds a step to it:

- `cargo xtask ci` — **exit 0**, closing line *"all checks passed"*. Four of four
  `OPTIONAL` steps **ran**; nothing skipped. The new *every stated rule count
  matches the suite* step reports *"6 stated rule count(s) checked against 1
  (model.rs), 5 (concurrency.rs), 17 (projection.rs), 89 (suite.rs), 112 (all
  four rule files)"*.
- `cargo fmt --all --check` — **clean**. `redkiln validate --kb` — *"validate
  passed"*. Working tree clean.
- `cargo test -p happenstance-testkit --all-features` over the projection targets
  at this HEAD — `projection_conformance` 17/17,
  `projection_conformance_blocking` 17/17, `projection_conformance_buffering`
  18/18, `projection_harness_parity` 2/2, `projection_mutation_coverage` 7/7;
  `outside-projection-adapter` 17/17 + 3/3 + 3/3; and
  `projection_conformance_wasm` under `wasm-bindgen-test-runner` **17 passed, 0
  failed, 0 ignored**.
- `cargo xtask lints` — all six green, including `lint-constitution` at *"27
  atoms, all consistent"* after the twenty citations that moved with the xtask
  edit were repointed by anchor.

The review's own earlier run, kept for the record:

- `cargo xtask ci --fast` (`.redkiln/config.yaml:55`, `integration_scoped`) at HEAD
  `01a99e9` — **exit 0**, *"all required checks passed (--fast: 4 optional step(s) not run)"*.
- `cargo fmt --all --check` — **clean**. Non-negotiable, and proven here rather than
  inferred from the gate's own fmt step.
- `cargo test -p happenstance-testkit --all-features` over the four projection targets —
  `projection_conformance` 17/17, `projection_conformance_buffering` 18/18,
  `projection_harness_parity` 2/2, `projection_mutation_coverage` 7/7.
- `cargo test -p outside-projection-adapter --all-features` — 17/17 plus 3/3 plus 3/3.
- `cargo test -p happenstance-testkit --target wasm32-unknown-unknown --test projection_conformance_wasm`
  under `wasm-bindgen-test-runner` — **17 passed, 0 failed, 0 ignored**.
- `redkiln validate --kb` — *"validate passed"*.
- No Playwright specs exist in this repository; the `--list` collection pass does not apply.

Definition of Done: this project is **non-terminal**, so `dod_green: true` in
`.bklg/from-contract-to-published-library/projection-store-freeze/_integration.md`
is this project's own integration bar, and the fifteen whole-initiative journeys
it does not own are listed under `deferred_scenarios` with a named owner each and
are **not** held against it. Confirmed: `reachability_ok: true`; rows IB-1 to
IB-17 all `executed` / `PASS`; and **no scenario this project owns is
`test.fixme`d, `#[ignore]`d or flag-gated off** — I swept `crates`, `examples`
and `xtask` independently and the only matches are prose warning against the
practice. The one whole-initiative item this project owns outright, **DoD 7, was
executed and passed in both halves**: the wrong store convicted **by name**
(`commit_is_atomic_with_the_read_model`) and the two batch shapes named
individually (`MemoryProjectionFixture`, `BufferingProjectionFixture`).

### Escape-hatch hunt — nothing found

I looked specifically for the patterns that make a green suite meaningless, and
found none.

1. **No double, no-op or injected stub standing in for a real contract.** Every
   mutant and every conformant variant is a real `ProjectionStore` built as a
   single overridden step over a shared correct-steps module
   (`crates/happenstance-testkit/tests/projection_mutation_coverage/correct.rs`),
   never a stub configured to fail on cue.
2. **No fixture-pinned assertion.** Rules read back through fresh handles and
   compare against what the store actually assigned;
   `fresh_projection_has_no_checkpoint` and `reset_is_not_commit_at_first` assert
   `Checkpoint` **variants** rather than positions, which is both the CF-6
   discipline and what lets them tell two mutants of the same family apart.
3. **No skip standing in for a pass, and no rule missing from a binary.** A
   declined capability still emits its test and returns `RuleOutcome::Skipped`;
   the exactness meta-test rejects any `Skipped` the registry does not account
   for.
4. **No unmounted deliverable.** Every capability in the reachability table
   resolves to a consumer the gate compiles and runs, including one outside the
   workspace.

The seventeenth rule is the case worth stating out loud, because it is where a
weaker project would have gamed it: it was **withdrawn** on 2026-08-14 because
landing it would have widened `[FROZEN]` PS-19 by test, and **restored** at
`dc363f4` only after ADR-0030 reached `status: accepted` and minted PS-38. PS-19
is byte-identical across the episode. That is the opposite of an escape hatch and
it is recorded here as credit, not as a caveat.

### Findings

The four blocking findings are recorded below as they were written, and each
carries the checkpoint that closed it. They are not rewritten into the past
tense: the finding is the evidence for the repair, and a finding edited to agree
with the fix stops being able to explain why the fix exists.

| Finding | State | Closed by |
| --- | --- | --- |
| FR-1 — the specification contradicts itself about the seventeenth rule | **resolved** | `a930e12`, with `d0d3db2` repointing the citations the six added lines moved |
| FR-2 — the proof artefact is pinned to a superseded tree | **resolved** | `b83ce3c` — a new dated document naming the old one, never an edit, with the `proof.rs` guards repointed at it |
| FR-3 — the crates.io README describes a suite two thirds smaller than the one beside it | **resolved, and the class retired** | `263b7e6` — both sentences, three further documents the sweep found, and a mandatory gate step that fails on the next one |
| FR-4 — the signed-off design record teaches a `[dev-dependencies]` spelling that cannot compile | **resolved** | `04b1f4d` |
| FR-5 — two stories recorded as not started | **still open, CLI-owned** | not this pass's to write; see below |
| FR-6 — nine `unconsumed-foundation` problems | **still open, initiative-level** | not this project's; see below |

**FR-1 (blocking) — `spec/SPECIFICATION.md` contradicts itself about the
seventeenth rule, in three authored places.** §7.2 is generated and correct
(`:8852`, PS-38 with both falsifiers and no dagger); the prose around it is not:

- `:5314-5315` — *"The rule is still unwritten, which is why it is marked new
  above and daggered in §7.2"*. It is not daggered; the dagger was removed in
  the same commit that landed the rule.
- `:5457-5458` — PS-38's own **Rule:** line: *"and the new
  `fresh_projection_has_no_checkpoint` for the second sentence, which nothing
  checks today"*. It is checked on four fixtures and three emitters.
- `:5836-5841` — *"**Sixteen of the seventeen now exist** …
  `fresh_projection_has_no_checkpoint` is the one still to be written"*.

`dc363f4` touched `spec/SPECIFICATION.md` on four lines, all inside the generated
region; its own commit message records correcting *"three that said the
seventeenth rule was held"* — it corrected three **elsewhere** and left these
three. `cargo xtask spec-trace` structurally cannot see this: it checks markers,
rule names, case numbers and citations, and the region it regenerates is the one
already right. This is the specification a consumer reads, it is the clause
disposition AC-014 and scope item 9 own, and `_integration.md:210-212` already
says **"Owed by this project — and cheap now."**

**FR-2 (blocking) — the project's proof artefact is pinned to a superseded tree,
and no superseding document exists.**
`references/evaluation/phase-6-projection-proof.md:114-118` says *"Sixteen rules
are registered in it; a seventeenth, `fresh_projection_has_no_checkpoint`, is
specified and not yet written, and §7.2 daggers it for that reason."* Both halves
stopped being true at `dc363f4`. The document is honestly dated and pinned to
`7620481`, so it states no falsehood about its own subject — but
`references/evaluation/README.md`'s lifecycle rule is *superseded rather than
edited*, so the repair is a **new dated document naming this one**, and it does
not exist. The runbook's *"a phase is done when its proof artefact exists"*
therefore points at a description of a tree that is not this one, and the delta
is precisely the deliverable that was this project's disclosed hold.
`_integration.md:188` states it: **"Owed by this project before closeout."** This
review is that closeout.

**FR-3 (blocking) — `crates/happenstance-testkit/README.md` tells crates.io that
this project's deliverable does not exist, and this project wrote the sentence.**
`:16-20` — *"The `ProjectionStore` suite exists but is two rules of seventeen,
and neither has been shown to reject a wrong store yet — the hostile stores that
will are named in the specification and not yet written"* — and `:127` — *"Two
rules of the seventeen the specification names, today."* Seventeen rules drive
the port, twenty-one registry rows drive the suite, and eighteen hostile stores
live in `crates/happenstance-testkit/tests/projection_mutation_coverage/mutants.rs`.
This is not inherited: `79df6b7`, this project's `projection-suite-entry-point`
checkpoint, **authored** both sentences, and the fifteen rules that falsified
them landed afterwards inside the same project. The gate confirmed the file ships
— *"happenstance-testkit: 41 files packaged, including … README.md"* — and cannot
see the defect, because `cargo package --list` asserts presence, not truth. This
is the **third audit** at which it has been raised. The UX brief does scope the
*crate landing page* to `publication-and-positioning`
(`_decomposition.md:60-64`), and the docs.rs crate doc is correct and current
(`crates/happenstance-testkit/src/lib.rs:262`, *"All seventeen rules … including
`fresh_projection_has_no_checkpoint`"*), which is why this is a required change
with a low bar rather than a rejection of the routing. But a project does not
discharge a false sentence it wrote by handing it to a project that has no reason
to know the rule count.

**FR-4 (blocking, record-level) — the signed-off `_design.md` contradicts the
shipped extension surface.** `_design.md:386-389` shows the outside author taking
`happenstance-core` under `[dev-dependencies]`. That spelling cannot work,
because the `ProjectionProbe` impl lives in `src/` — the argument the same record
makes at `:375-382`. The landed manifest is right
(`examples/outside-projection-adapter/Cargo.toml:20-23`) and so is the shipped
rustdoc (`crates/happenstance-testkit/src/lib.rs:184-191`). `_design.md` is the
binding design record for the one extension surface DT-8 created, and it is the
outlier. Repeat finding, raised at `b4c7972` and unfixed.

**FR-5 (not blocking, and not mine to fix) — two of seventeen stories are
recorded as not started.** `reset-rules` (HS-S0011) and
`read-through-and-rebuild-rules` (HS-S0012) sit at `stage: plan` / `status:
ready` with `updated: 2026-08-13`, while the other fifteen are `stage: report` /
`status: in-review`. Their work is fully merged (`10ace94`, `5be22ab`, `064687a`,
`dc363f4`), every `_ledger.md` row is `satisfied: true` with cited evidence, and
`4b39add` sealed their slice **approved**. The CLI is the only writer of those
fields, so this review neither can nor does touch them — surfaced for the
orchestrator, because a project cannot honestly be declared done while its own
state record is two stories short.

**FR-6 (not blocking, pre-existing) — `redkiln doctor` exits 1 with nine
`unconsumed-foundation` problems**, one of which is this project's
(`projection-decision-atoms`, HS-S0002) and eight of which belong to five
projects that have not started. All nine story files were added by the planning
commit `ae77ac4`, an ancestor of this project's base `2136ddeb`, so this is a
decomposition-shape condition predating every line of implementation here. CI's
`backlog` job asserts the list is empty (`.github/workflows/ci.yml:177`), so that
job stays red until the decomposition is re-shaped. Initiative-level.

### Scope drift and quality-bar regressions

No functional scope drift: every capability delivered maps to a project AC, and
the two things a reviewer would expect to find creeping in — a SQL projection
adapter, and a freeze verdict — are both correctly absent and correctly
attributed to `sqlite-durable-store` and `ladybug-projection-store`. That is
still true after the repairs: nothing in the five checkpoints touches a store, a
port signature or a rule body, and no freeze verdict is written anywhere in them.

The regression was a class rather than an instance: **four separate documents
inside this project's own boundary contradicted the code this project
committed** (FR-1 through FR-4), three of them repeats across two prior audits,
and every one invisible to a gate that is otherwise unusually good at holding
code to documents. `_integration.md:276-279` named the same pattern in its own
words — *"this repository's gate is unusually good at holding code to documents
and has almost no instrument for holding documents to code."*

**It now has one.** `cargo xtask lint-rule-counts` is mandatory in `cargo xtask
ci` and in the story-grain `affected` gate, and its own rustdoc states what it
does not verify at length, in this module's convention: only a cardinal
qualifying `rule` or `rules`, only in a paragraph about the suite, only in the
four documents a consumer reads. `CHANGELOG.md` is excluded on principle — a
released entry that was true when written must not be rewritten to keep a check
green — and a document set that states no count at all fails the step rather than
passing it vacuously. The sweep that installed it found the same stale sentence
in three more places than the audit had named, which is the argument for a check
over a correction stated as a measurement rather than a preference.

Two edits *caused* drift and repaired it in the same pass rather than leaving it:
adding six lines to the specification moved thirteen `file:line` citations
(`d0d3db2`), and growing `lints.rs` and `main.rs` moved twenty constitution
citations (`263b7e6`). Both sets were repointed by content and both are held by a
gate step — the second by `lint-constitution`, which is what caught them.

### Per-story checkpoint SHAs

Derived from git rather than from the seed.
`git log 2136ddeb..HEAD --grep "Story: projection-store-freeze/"` returns
**twenty-one** commits, oldest first.

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

Reconciliation with the workflow-reported list, resolved in git's favour: git
additionally carries `5d9b4fd`, `2a7ae8a`, `7620481` and `501fb89`, while the
workflow list's `4b39add` is a **slice seal** (`Slice-Verdict:` trailer), not a
story checkpoint. The ten slice seals are `560bb4b`, `0ccf16e`, `f2f7871`,
`42144d2`, `7d54243`, `41ec9d1`, `6ce1cf3`, `56b17b7`, `e76f363` and `4b39add`;
the in-slice fix passes are `cc0f158`, `b7c1600`, `fb4161c`, `064687a`, `d9eb8de`
and `dc363f4`; the integration audits are `b4c7972` (superseded) and `01a99e9`.

**Baseline repairs: none.**
`git log 2136ddeb..HEAD --grep "Baseline-Repair" --grep "Slice-Repair"` returns
nothing, matching this run's declaration that no out-of-band baseline repair
occurred.

## Required Changes — all four made

Recorded as they were written, each with what actually landed against it. Three
were paperwork the capability work invalidated; the third grew a gate step,
because the review invited one and the sweep proved it was needed.

1. **Done — `a930e12`** (+ `d0d3db2` for the citations the edit moved).
2. **Done — `b83ce3c`**: a new dated document,
   `references/evaluation/phase-6-projection-proof-at-closeout.md`, pinned to
   `04b1f4d`, naming the document it supersedes; the old one untouched;
   `xtask/src/proof.rs`'s seven guards repointed at the current artefact.
3. **Done — `263b7e6`**, and the durable fix was taken rather than deferred. The
   README's two sentences are corrected; so are the crate page, the manifest
   comment and the changelog entry the sweep found saying the same thing; and
   `cargo xtask lint-rule-counts` now fails the build on the next one. Two more
   the check cannot see were fixed by hand in the same pass —
   `mutation_coverage.rs`'s `PROJECTION_MUST_REJECT` doc said "thirteen of
   sixteen" and named "the three absentees" when there are four.
4. **Done — `04b1f4d`**.

The original text of the four follows.

1. **Repair `spec/SPECIFICATION.md`'s three false statements about
   `fresh_projection_has_no_checkpoint`** — `:5314-5315`, `:5457-5458` and
   `:5836-5841` — so the authored prose agrees with the generated §7.2 it cites.
   Prefer wording that points at the enumeration
   (`crates/happenstance-testkit/src/projection.rs:1884-1919`) over restating a
   count nothing in the gate reads. This is AC-014's surface, and
   `_integration.md:210-212` already assigns it here.
2. **Supersede `references/evaluation/phase-6-projection-proof.md` with a new
   dated document pinned to the current tree**, per that directory's own
   *superseded rather than edited* rule — not an edit to the existing one. It
   should carry the seventeen-rule enumeration, the twenty-one registry rows, the
   four fixtures and the `wasm32` execution, and name the document it supersedes.
   `_integration.md:188` assigns it here, before closeout.
3. **Correct `crates/happenstance-testkit/README.md:16-20` and `:127`.** This
   project wrote those two sentences and this project falsified them; they ship
   to crates.io and the gate cannot see them. If the landing page's *positioning*
   is genuinely `publication-and-positioning`'s, correct the two factual counts
   here and leave the positioning to them — do not hand a false sentence forward.
   Consider the durable fix `_integration.md` names: a gate step reading a
   README's counts against the enumeration, which retires the class instead of
   correcting its fifth instance.
4. **Correct `_design.md:386-389`** to `[dependencies]`, matching the shipped
   manifest and the shipped rustdoc, so the binding design record stops teaching
   the one spelling of the extension surface that cannot compile.

Two items are recorded for the orchestrator rather than for the implementer, and
neither blocks the verdict. **Both are still open**, and the repair pass could
not close either without exceeding what it is allowed to write:

- **HS-S0011 and HS-S0012 need a `redkiln advance`** so the project's state record
  matches its merged work (FR-5). The CLI is the only writer of those fields;
  neither this review nor the repair pass touched them.
- **The nine `unconsumed-foundation` problems are initiative-level** (FR-6) and
  keep CI's `backlog` job red on this branch until the decomposition is re-shaped.
  Eight of the nine belong to five projects that have not started.

The gate was re-run from scratch anyway, whole rather than `--fast`, because two
of the repairs touch compiled paths and one adds a step to the gate itself:
**exit 0 at `b83ce3c`, four of four optional steps ran, formatter clean.**
