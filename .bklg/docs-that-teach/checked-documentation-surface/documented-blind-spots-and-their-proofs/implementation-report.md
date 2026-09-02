---
item: "HS-S0145"
stage: implement
created: "2026-08-17T13:16:06.300Z"
updated: "2026-08-17T13:16:06.300Z"
---

# Implementation Report — What the check does not verify is stated first, and executed where it can be

## TDD Evidence

The tests came first, and they came first *twice*: once against the section that did not exist
yet, and once against the two claims the slice-mate's run had just proved false.

**Red.** The whole obligation was written as `#[cfg(test)]` code in the checker module before
a word of the sections changed — the `NOTE_TEN` table (six limits, each with its owning module,
its `claim` half and its `instrument` half), the `limits_problems` pass that reads a module's
docs as text, and eleven tests over it. `cargo test --locked -p xtask --bin xtask lint_narrative::`
→ **115 passed; 9 failed**, every failure an assertion about missing behaviour and not one of
them a compile or import error:

| Test | Red message |
| ---- | ----------- |
| `both_modules_state_every_limit_they_own_and_none_of_the_others` | limits 1 and 3 missing from the harness, limit 5 restated there, limit 4 restated in the checker |
| `limit_one_says_no_mechanical_test_can_close_it` | `the harness must state limit 1` |
| `limit_three_states_the_measurement_and_cites_the_record` | ``limit 3 must carry `_limits-evidence.md`, got: **What `RUSTDOCFLAGS=-D warnings` actually enforces … is unmeasured here.**`` |
| `the_cross_target_reference_is_a_plain_path_and_not_an_intra_doc_link` | `the harness must point at xtask/src/lint_narrative.rs by path` |
| `a_hedged_teaching_sentence_is_rejected` | limit 6's canonical sentence absent, so the mutation had nothing to hedge |
| `the_text_fixture_is_not_flagged_by_the_real_fence_walk` | `reading docs/text-fences.md: The system cannot find the file specified` |
| `the_text_fixture_is_registered_in_both_directions_and_inside_the_budget` | `xtask/src/narrative.rs must include text-fences.md` |
| `the_text_fixture_has_an_index_row_and_reorders_nothing` | `docs/README.md must route to text-fences.md` |
| `neither_module_carries_a_mark_claiming_the_documentation_is_checked` | the scan read the whole file, tests included |

**Two tests passed in the red run for the wrong reason and were tightened rather than kept.**
`a_module_missing_one_of_its_limits_is_rejected` mutated a bullet that was not there, so
`without_bullet` removed nothing and the test passed on the limit's genuine absence — it now
asserts the bullet exists *before* the mutation and is gone *after* it, so the mutation cannot
be a no-op. `neither_module_carries_a_mark_claiming_the_documentation_is_checked` failed because
it scanned the whole file, including this module's own test code, which quotes the tokens the
scan forbids; it now reads the shipping half through a `shipping_source` helper — the split
`production_source` already used, generalised to a path — which is the honest fix rather than
deleting the token from the assertion list.

**Green.** The sections were written, the fixture page created and registered, the index row
appended, and the two false sentences corrected. `cargo test --locked -p xtask --bin xtask` →
**175 passed; 0 failed**. No test was weakened to get there; two were strengthened.

| AC | Test / evidence | Red → Green |
| -- | --------------- | ----------- |
| AC-001 | `both_modules_state_every_limit_they_own_and_none_of_the_others` (harness arm) + `documentation` step | RED on missing limits → GREEN with the section first and `-D warnings` clean |
| AC-002 | the same test's checker arm + `the_cross_target_reference_is_a_plain_path_and_not_an_intra_doc_link` | RED: no plain-path pointer either way → GREEN with both |
| AC-003 | the per-limit arms of `limits_problems`, both directions | RED: two missing, two restated → GREEN, six limits, none restated |
| AC-004 | `limit_one_says_no_mechanical_test_can_close_it` | RED: `the harness must state limit 1` → GREEN; **no test is named for limit 1** anywhere in this PR, and that absence is the point |
| AC-005 | `limit_three_states_the_measurement_and_cites_the_record` + four probe transcripts | RED on the "unmeasured here" wording → GREEN on the measurement |
| AC-006 | `cargo xtask ci --fast` and `cargo xtask ci` over the tree holding the fixture | RED: the page did not exist → GREEN: `2 pages, all consistent`, no problem line |
| AC-007 | `the_text_fixture_is_not_flagged_by_the_real_fence_walk` | RED: file not found → GREEN through the real `check_page`, with an instruction in its failure message |
| AC-008 | limit 4's wording, traced to `_falsification.md` | The forecast was falsified by the record, so limit 4 was written from the record and the old sentence deleted from two modules |
| AC-009 | four negative tests, one per named wrong implementation | Each rejects a mutation of the **real** module text with a distinguishable message |
| AC-010 | the eleven-row reconciliation in `_limits-evidence.md` | Written **before** the sections, so it could tell whether Note 10's six were still six |
| AC-011 | `neither_module_carries_a_mark_claiming_the_documentation_is_checked` + the hedge arm + `git ls-files` | RED on the whole-file scan → GREEN on the shipping half; 0 rows of `.css`/`book.toml`/`book/`/`site/` |
| AC-012 | `the_text_fixture_is_registered_in_both_directions_and_inside_the_budget`, `the_text_fixture_has_an_index_row_and_reorders_nothing`, plus the checker's own checks over the page | RED on both → GREEN; measured 19-character path, 30-character H1, 79-column widest line, 27 source lines |

## Commits

| SHA | Subject |
| --- | ------- |
| `<checkpoint>` | feat(checked-documentation-surface): Documented blind spots and their proofs |

The row records the checkpoint **as first written**. A commit cannot contain its own SHA, so the
reachable commit is reported in the slice digest and recorded on the item by
`redkiln record-links --sha` at the advance seam. Single checkpoint on
`initiative/docs-that-teach`, not pushed.

## Changes

| File | Shape of the change |
| ---- | ------------------- |
| `xtask/src/narrative.rs` | The limits section rewritten to carry limits 1, 2, 3, 4 and 6 in the two-part bullet shape, with limit 5 moved out and replaced by a plain-path pointer to the checker. Limit 3 states what the re-run probe measured on 1.97.1; limit 4 is written from `observed-failure-falsification`'s record. Plus the `text` fixture's `#[cfg(doctest)] mod text_fences` registration. |
| `xtask/src/lint_narrative.rs` | Limit 5 rewritten in the two-part shape and pointed at its retained fixture; the harness's four limits replaced by a plain-path pointer; limit 6 given the canonical unhedged sentence. In `#[cfg(test)]`: the `Limit`/`NOTE_TEN` enumeration, `shipping_source`, `module_docs`, `limits_bullets`, `limits_problems`, `without_bullet`, and eleven tests. One legacy assertion updated with its reason. |
| `xtask/src/narrative_doctests.rs` | Two sentences the runs measured to be false, corrected: the limit-4 claim that the report names the harness, and the limit-3 claim that the measurement was blocked by the extra `cargo run -p xtask` hop. Plus the count's real meaning, stated beside it. |
| `docs/text-fences.md` | **New.** The retained fixture: a conforming `narrative-page` whose only fence is tagged `text` and is deliberately false about the library, with prose saying so. |
| `docs/README.md` | One appended row in the two-column narrative table. Nothing reordered. |
| `.bklg/…/documented-blind-spots-and-their-proofs/_limits-evidence.md` | **New.** Provenance, the probe's stimulus control, four probe transcripts, the `text`-fence walk, the limit-4 forecast-vs-record table, the eleven-row completeness reconciliation with the `compile_fail` disposition, two findings, and the record's own limits. |
| `.bklg/…/documented-blind-spots-and-their-proofs/_ledger.md` | Twelve rows flipped `false` → `true` with real citations. No criterion re-worded. |
| `.bklg/…/documented-blind-spots-and-their-proofs/implementation-report.md`, `report.md` | **New.** This file and the findings ledger. |

No new `REQUIRED` step, subcommand, `print_help` line or `lint_steps` member; no change to
`TREE`, `HARNESS`, `HIDDEN_MARKERS`, `IGNORE_ALLOWANCES`, the resolver, the pin, the fence
walk's rules, or any step's name, args or `env`; no new dependency in `xtask/Cargo.toml`; no
`book.toml`, `book/`, `site/` or `.css`.

## Gates

| Command | Result |
| ------- | ------ |
| `cargo test --locked -p xtask --bin xtask lint_narrative::` (red) | **115 passed; 9 failed** |
| `cargo test --locked -p xtask --bin xtask` (green) | **175 passed; 0 failed** |
| `cargo xtask narrative` | `  2 pages, all consistent` |
| `RUSTDOCFLAGS=-D warnings cargo run --locked --quiet -p xtask -- narrative-doctests` | green, `  1 page(s)' examples enumerated` |
| `cargo xtask lint-constitution` | `  27 atoms, all consistent` — the checker whose shape this copies is not regressed |
| `cargo xtask affected --base main` | `affected gate passed` |
| `cargo xtask ci --fast` | exit **0** — `all required checks passed (--fast: 4 optional step(s) not run)`, including `documentation` under `-D warnings` |
| `cargo xtask ci` (full) | exit **0** — `all checks passed`, 0 `skipped:` lines. This is the project's terminal story, so the full gate is run here (`project.md` DoD item 6) |
| `cargo fmt --all --check` | clean, run last after the clippy fixes |

Three clippy findings surfaced on the first `--fast` run and were fixed rather than allowed:
`collapsible_if` (folded into a let-chain, which the 1.97.1 MSRV permits — ADR-0029),
`nonminimal_bool` (`is_none_or`), and `redundant_closure_for_method_calls` (`str::is_empty`).
The formatter ran last.

## Notes

**Four deviations, three of them forced by measurements rather than chosen.**

1. **Limit 4 was rewritten in two modules because the slice-mate's run falsified it.**
   Architecture brief Note 3 forecast that a fence failure is reported against the harness file.
   It is not: the report names ``xtask\src\../../docs/append-conditions.md -
   narrative::append_conditions (line 9)`` — the page's own path, reached through the harness's
   *directory*. EC-004 settles the tie-break in advance ("the record wins and limit 4 is
   reworded to match it"), so the sentence was corrected where it holds and deleted where it was
   a second copy. That deletion required editing one assertion in
   `narrative-checker-mounted-with-pinned-path`'s test — it required the literal phrase
   `names the harness, not the markdown`, which is now known to be false. The test was kept and
   its claim replaced by the plain-path pointer, with a doc comment recording why. Weakening a
   sibling's test to fit new prose would be gaming; correcting an assertion the evidence
   disproved is the opposite, and the reason is written where the next reader will find it.

2. **`xtask/src/narrative_doctests.rs` was edited, and the spec names only two modules.** It is
   a third module this project added, and it carried both false claims — limit 4's, and limit
   3's "unmeasured here … the extra `cargo run -p xtask` hop", which observation (b′) exonerates.
   Leaving two sentences a run has just disproved inside the story whose entire subject is
   honest limits was not defensible. The edit is inside the PR boundary (`xtask/src/*.rs`),
   changes no behaviour, and is recorded as finding **L1**.

3. **The `RUSTDOCFLAGS` probe returned a third answer, and EC-001 governs.** It agrees with
   neither `xtask/src/constitution.rs:31-36` nor the upstream reports as stated: `-D warnings`
   reaches *nothing* inside a narrative fence on 1.97.1 — not clippy, not the workspace `[lints]`
   table, and not rustc's own default-on lints, which is the half the earlier probe recorded as
   recovered. It is written down as what happened. It is **not** reconciled against either prior
   claim, limit 3 is not softened into "may not", and the merged-doctest mechanism — active in
   every transcript — is explicitly *not* claimed as the cause, because naming a cause the run
   did not isolate would be the copying Note 10 forbids. `constitution.rs`'s own sentence is
   routed to that module's owner rather than edited here: it is about a different step and is
   owed its own re-run.

4. **A fourth negative test was added beyond AC-009's three.** AC-003's negative direction — a
   module restating the other's limit — had no wrong implementation behind it, and a rule no
   input can fail is decorative. `a_module_restating_the_other_modules_limit_is_rejected`
   splices limit 4's claim into the checker's real source and requires `restates limit 4`.

**Nothing pretends to cover limit 1.** No test, fixture, metric or count in this PR is named for
it, deliberately: the precedent is `xtask/src/constitution.rs:20-25`, and a decorative
instrument here is what gets HS-P0024's friction log deleted as redundant.

**One new finding fell out of the reconciliation** and is stated where it can be read: the
compile step's `1 page(s)' examples enumerated` counts pages that *produce a doctest*, not pages
the harness registers, which is why it prints `1` beside the checker's `2 pages, all consistent`.
Both numbers are correct and they answer different questions; the sentence saying so now sits
next to the count.

---

## Addendum — 2026-08-17, the review fix (limit 7 / finding L6)

Appended rather than rewritten: the counts above are what the implement stage measured and stay
as they were. What changed afterwards, under the slice review:

- **A seventh limit landed** in `xtask/src/narrative.rs` — a broken fence in this tree does not
  fail under this step's banner; it fails under `=== tests ===` at index 2, because that step
  compiles the lib target's doctests and `run_steps` bails first. It is `_falsification.md`
  **F2**, which HS-S0144 called the most valuable output of its run and which had landed
  nowhere. `NOTE_TEN` carries it as a seventh entry, and two tests hold it there.
- **A third measured-false sentence** in `xtask/src/narrative_doctests.rs` — the ordering claim
  on `the_narrative_step_precedes_the_constitution_step`, which L1's sweep missed — was reworded
  to L1's standard. No assertion weakened; no step name, argument or `env` entry touched.
- **`narrative_doctests::tests::the_steps_that_compile_this_tree_are_pinned_in_gate_order`** now
  pins the three `REQUIRED` steps that hand these pages to rustdoc, in gate order, so the prose
  and the array cannot drift apart again.
- **The ordering defect is routed with an addressee**: `FU-1` of `HS-P0020`, in the project
  charter's `## Follow-ups routed out of this project`.

Gate after the fix: `cargo xtask ci` exit 0, `all checks passed`, 0 `skipped:` lines;
`cargo test --locked -p xtask --bin xtask` **178 passed, 0 failed**; `cargo xtask affected
--base main` → `affected gate passed`; `cargo fmt --all --check` clean.
