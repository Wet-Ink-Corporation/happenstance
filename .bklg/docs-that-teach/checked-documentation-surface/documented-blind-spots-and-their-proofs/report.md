---
item: "HS-S0145"
stage: report
created: "2026-08-17T13:16:06.300Z"
updated: "2026-08-17T13:16:06.300Z"
---

# Report — What the check does not verify is stated first, and executed where it can be

## Findings Ledger

Twelve ACs, all satisfied by reachable behaviour with a `file:line` citation and either a test
or a recorded measurement behind each. Nothing stubbed, nothing skipped, nothing deferred out of
this story's own set. Two of the ACs were satisfied by **correcting claims a measurement
disproved**, which is the more interesting half of this report and is called out per row.

This is the project's terminal story, so the full `cargo xtask ci` was run here rather than only
`--fast`: exit **0**, `all checks passed`, zero `skipped:` lines.

| AC | Result | Evidence | Follow-up |
| -- | ------ | -------- | --------- |
| **AC-001 — the limits section is first in the harness, composed, and survives `-D warnings`.** | met | `xtask/src/narrative.rs:1-6`; `::tests::both_modules_state_every_limit_they_own_and_none_of_the_others`; `::tests::a_limits_section_moved_below_another_heading_is_rejected`; `documentation` step green in `cargo xtask ci` | None. The section earns `-D warnings` by carrying no intra-doc link across the target boundary, asserted directly. |
| **AC-002 — first in the checker too, with a plain-path cross-reference.** | met | `xtask/src/lint_narrative.rs:10-22`; `::tests::the_cross_target_reference_is_a_plain_path_and_not_an_intra_doc_link` | None. Both directions are asserted, and `[crate::narrative]` / `[crate::lint_narrative]` are rejected by name. |
| **AC-003 — all six limits, each where it holds, in the two-part shape, and neither module restating the other's.** | met | harness `:17-71`, checker `:31-45` and `:114-118`; the `NOTE_TEN` table and `limits_problems`; negative arms for a dropped limit and a restated one | None. `claim` and `instrument` are required in the **same bullet**, so a limit whose instrument is named three paragraphs away fails. |
| **AC-004 — limit 1 says no mechanical test can close it, says why, names the friction log, and nothing pretends otherwise.** | met | `xtask/src/narrative.rs:17-27`; `::tests::limit_one_says_no_mechanical_test_can_close_it`; the absence of any test named for limit 1 in the diff | Limit 1 is worded **narrower** than Note 10 invites, because F1 measured that the machine *can* catch a false-but-compiling assertion. What it cannot see is a claim the prose makes that the fence never asserts. |
| **AC-005 — limit 3 states the measurement, cites the record, names the upstream reports as context, and no probe fence survives.** | met | `xtask/src/narrative.rs:35-49`; `_limits-evidence.md` § *Limit 3*; `::tests::limit_three_states_the_measurement_and_cites_the_record`; `git status --porcelain` carries no `docs/` entry | **EC-001 fired.** See finding **L3**. |
| **AC-006 — limit 5 walked through the whole gate and observed to pass.** | met | `docs/text-fences.md`; `_limits-evidence.md` § *Limit 5*; `cargo xtask ci --fast` and `cargo xtask ci` green with `2 pages, all consistent` | EC-003 did **not** fire: the fixture is not flagged, so limit 5 keeps its subject. |
| **AC-007 — a positive test pins the silence, and its message is an instruction.** | met | `::tests::the_text_fixture_is_not_flagged_by_the_real_fence_walk`, run by the `tests` step | None. It asserts the fixture still carries a `text` fence before asserting the walk is silent, so it cannot pass vacuously. |
| **AC-008 — limit 4 is quoted from the run, and the record beat the forecast.** | met (forecast falsified) | `xtask/src/narrative.rs:50-66`; `_limits-evidence.md` § *Limit 4*, a four-row forecast-vs-record table | **EC-004 fired.** See finding **L4**. |
| **AC-009 — the section rots loudly: four named wrong implementations, four distinguishable messages.** | met | `::tests::a_limits_section_moved_below_another_heading_is_rejected`, `::a_module_missing_one_of_its_limits_is_rejected`, `::a_hedged_teaching_sentence_is_rejected`, `::a_module_restating_the_other_modules_limit_is_rejected` | The fourth is beyond the criterion's three, added because AC-003's negative direction otherwise had no input that could fail it. |
| **AC-010 — every delivered check has its limit on the record, and the `compile_fail` candidate has a written disposition.** | met | `_limits-evidence.md` § *The completeness reconciliation*, eleven rows | EC-009 did not fire. `compile_fail`: **not applicable, not a seventh bullet** — the delivered walk permits it *and* requires a `compile_fail` fence to name its error code in prose, which is the mitigation the research's limit asks for; no page in `docs/` uses it today. |
| **AC-011 — nothing anywhere claims the surface proves a page teaches.** | met | limit 6 unhedged at harness `:67-71` and checker `:114-118`; `::tests::neither_module_carries_a_mark_claiming_the_documentation_is_checked`; the hedge arm; `git ls-files` returning **0** rows for `.css`/`book.toml`/`book/`/`site/` | None. The fixture page says the opposite in as many words: nothing in the gate reports the false fence it carries. |
| **AC-012 — the fixture is a conforming `narrative-page`.** | met | `docs/text-fences.md`; `::tests::the_text_fixture_is_registered_in_both_directions_and_inside_the_budget`, `::the_text_fixture_has_an_index_row_and_reorders_nothing`; the checker's own checks over the page in `cargo xtask ci` | Measured, not eyeballed: path **19**/32, H1 **30**/40, widest line **79**/80, page **27**/250. |

### Findings

| # | Finding | Evidence | Disposition |
| - | ------- | -------- | ----------- |
| **L1** | **`xtask/src/narrative_doctests.rs` carried two sentences the runs measured to be false** — that a failure names the harness rather than the page, and that the `RUSTDOCFLAGS` question was unmeasurable because of the extra `cargo run -p xtask` hop. | `_falsification.md` F5; `_limits-evidence.md` observations (a), (b), (b′) | **Corrected in this change.** A third module, outside the spec's named two, but leaving a disproved limit inside the story about honest limits was not defensible. Behaviour unchanged. |
| **L2** | **The compile step's coverage number counts doctests, not registrations.** With two pages registered, the checker prints `2 pages, all consistent` and the compile step prints `1 page(s)' examples enumerated`. A `text`-only page is registered, walked, and absent from that number. | the two transcripts in `_limits-evidence.md` § *Limit 5* | Not a defect — the number is a fact about what rustdoc was given. Now **stated beside the count** in `xtask/src/narrative_doctests.rs`, because a reader who reads it as "pages in the tree" under-counts. |
| **L3** | **The `RUSTDOCFLAGS` probe returned a third answer.** On 1.97.1, against the step as `REQUIRED` declares it, `-D warnings` reaches **nothing** inside a narrative fence — not clippy, not the workspace `[lints]` table, and not rustc's own default-on lints, which is the half `xtask/src/constitution.rs:31-36` recorded as recovered. A `non_snake_case` violation compiled and ran with exit 0 and no diagnostic, with the variable, without it, and without the extra hop. A plain-`rustc` control confirms the stimulus fires. | `_limits-evidence.md` § *Limit 3*, four transcripts + the control | **EC-001**: recorded, not reconciled. Limit 3 is not softened into "may not"; the merged-doctest mechanism is explicitly *not* claimed as the cause. `constitution.rs`'s sentence is **routed to that module's owner** — it is about a different step and is owed its own re-run, not an edit from here. |
| **L4** | **Architecture brief Note 3's forecast for limit 4 is wrong about its subject.** The report names the page's own path reached through the harness's *directory*, never the harness's filename; `(line N)` is the fence's opening line, not the failing statement; and the panic's own `file:line` is unusable in both of its two forms. | `_falsification.md` §§ *What the failure actually identified*, F4, F5; `_limits-evidence.md` § *Limit 4* | **EC-004**: the record won and limit 4 says what the record says. Consequence recorded for the design's owner — `_design.md` anti-pattern 7 cannot be satisfied on the compile surface, because that body is rustdoc's. |
| **L5** | **One sibling assertion was corrected, not weakened.** `narrative-checker-mounted-with-pinned-path`'s `the_module_states_its_own_limits_and_its_one_divergence` required the literal `names the harness, not the markdown` — a claim L4 disproves. | `xtask/src/lint_narrative.rs`, that test's doc comment | The test is kept and its claim replaced by the plain-path pointer, with the reason written in the test's own doc comment so the next reader is not left guessing. |

### Mount points

- **`xtask/src/narrative.rs`** — the lib-target harness declared from `xtask/src/lib.rs:28`. Its
  module docs are the one surface in this story that a gate step actually *renders*, under the
  `documentation` step's `RUSTDOCFLAGS=-D warnings`. It also carries the fixture's
  `#[cfg(doctest)] mod text_fences` registration.
- **`xtask/src/lint_narrative.rs`** — the bin-crate checker declared from `xtask/src/main.rs:64-70`.
  Its limits section, and in its `#[cfg(test)]` block the eleven tests that fail when this
  story's obligations stop being true, run by the `tests` step (`xtask/src/main.rs:145-155`).
- **`xtask/src/narrative_doctests.rs`** — the compile step's driver; two corrected sentences.
- **`docs/text-fences.md` and `docs/README.md`** — under `TREE`, walked by both narrative steps
  on every gate run.

Every obligation here is reachable from `cargo xtask ci` on a clean checkout with no manual
step and no tool to install. Nothing is behind a `#[cfg]`, a feature, or a probe.

### Surfaces

It changes none of the five. It adds one instance of `narrative-page` (the fixture, state
`default`) and observes `gate-narrative-compile-step` and `gate-narrative-checker-step` in their
`pass` states — the proof of limit 5 being precisely that both stay green.

### Deferred

Nothing from this story's own set. One item is routed rather than fixed: the sentence at
`xtask/src/constitution.rs:31-36`, which is about the constitution's own step and is owed its
own re-run by that module's owner. Fixing it from here would be this story editing a claim it
did not measure.

### Gate

`cargo xtask ci` exit 0, `all checks passed`, 0 `skipped:` lines ·
`cargo xtask ci --fast` exit 0 · `cargo xtask affected --base main` → `affected gate passed` ·
`cargo test --locked -p xtask --bin xtask` → 175 passed, 0 failed ·
`cargo xtask lint-constitution` → `27 atoms, all consistent` · `cargo fmt --all --check` clean.
