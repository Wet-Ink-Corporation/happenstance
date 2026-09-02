---
item: "HS-S0140"
stage: report
created: "2026-08-18T01:00:00.000Z"
updated: "2026-08-18T01:00:00.000Z"
---

# Report — Hidden content is inside the check, or absent

## Findings Ledger

Six ACs, all satisfied by reachable behaviour: twelve unit tests observed red against a
one-token, unwired scan, plus a recorded run of the real step in its `fail-hidden-marker`
state. Nothing stubbed, nothing skipped, nothing deferred out of this story's set.

| Finding | Evidence | Follow-up |
| ------- | -------- | --------- |
| **AC-001 — a disclosure marker under `docs/` fails the gate by file and line, from inside the walk that already exists.** The problem joins the same `Vec<String>` and the same terminal `bail!`, so a contributor's other problems arrive in the same run. | `xtask/src/lint_narrative.rs:727-740`, called at `:750`; `::tests::every_hidden_marker_is_reported_once_per_occurrence`, `two_markers_on_one_line_are_two_problems`, `marker_and_fence_problems_arrive_in_one_list_in_source_order`. Gate: `cargo xtask narrative` and `cargo xtask ci --fast` exit 0 over the real tree | None. Mounted and not vacuous: the same command was observed failing on a temporary `docs/folded.md` and returning to green when it was removed. |
| **AC-002 — the named wrong implementation fails, and its clean twin passes.** The `_design.md` `## The doctest` fixture page wrapped around its scope band yields exactly two problems on their own lines; the identical page minus the wrapper yields none. | `::tests::the_wrapped_fixture_page_fails_by_file_and_line`, `the_same_fixture_page_without_its_wrapper_is_clean`. The wrapped form is derived from the clean constant at runtime rather than hand-copied beside it | The fixture is test material and never a file under `docs/` — a committed wrapper would fail `cargo xtask ci` forever, which is precisely what makes it a wrong implementation the rule can fail. |
| **AC-003 — every spelling a renderer accepts is rejected, and there is no region the scan skips.** Case-insensitive, line-based over the whole page including fenced blocks and including the index. | `:728-729`; `::tests::a_hidden_marker_is_matched_whatever_its_case` (`<Details>`, `<DETAILS open>`, `{{#TABS}}` — all of which a case-sensitive `contains` accepts), `a_marker_inside_a_fence_is_still_reported` (EC-003), `the_real_index_carries_no_hidden_marker` reading `docs/README.md`'s actual bytes | A fence-aware scan is the most plausible refinement and it is refused. The consequence — a page under `docs/` cannot quote `<details` — is stated in the module docs rather than hidden. |
| **AC-004 — the token set is pinned, and a shrink fails a test naming which token moved.** | `:208-216` with the `const`'s own doc at `:180-207` (DT-7, `_design.md`, "shrinking it requires a new design record"); `::tests::the_hidden_marker_set_is_pinned_to_the_design` (RS-81-5, both directions), `every_hidden_marker_is_already_lowercase`. RED against a one-token `HIDDEN_MARKERS` | The lowercase assertion is what makes AC-003's case-folding correct rather than accidentally correct after a future edit. |
| **AC-005 — the composed line is the design's, verbatim; the list is unbounded and ordered; no new chrome exists.** | `:735-738` (message), composed at `:742-757`; `::tests::a_marker_problem_is_the_composed_line_the_design_specifies` (whole-line exact string), `forty_markers_print_as_forty_lines`, `marker_problems_are_ordered_by_page_then_line`, `the_marker_scan_adds_no_step_or_banner_of_its_own`. Module docs lead with this check's three limits (`:35-47`), asserted to precede the first check by `the_module_docs_state_the_marker_scans_limits`. `cargo xtask lint-constitution` and `cargo test --locked -p xtask --doc` still pass | The location form carries no column, deliberately: `_design.md`'s primary element is `{path}:{line}` and a column would change its shape. |
| **AC-006 — no allowance path exists, no hook for one was left, and nothing claims teaching.** | `::tests::no_input_makes_a_hidden_marker_pass` (six shapes, including a comment claiming an exemption and a page naming an allowance) and `the_marker_scan_has_no_allowance_environment_or_cfg_hook` (reads the scan's own body for `IGNORE_ALLOWANCES`, `env::var`, `cfg(`, `feature =`); `nothing_in_the_module_claims_a_page_teaches` over the module's production half | This test caught a real slip during the story: a doc comment used the word `unverified`. The prose was reworded rather than the test relaxed. |

**Mount point.** No new mount, and that is AC-005's own review clause rather than an
omission. `check_hidden_markers` (`xtask/src/lint_narrative.rs:727-740`) is called from
`check_page` (`:750`) — the per-page walk `fence-discipline-and-allowance-list` wrote — which
`problems` folds over the tree and reports through one `bail!`, under the `REQUIRED` step
`narrative-checker-mounted-with-pinned-path` wired at `xtask/src/main.rs:526-553`. `REQUIRED`
gains no entry, the dispatch gains no arm, and `xtask/src/main.rs` does not mention a marker
anywhere. Surface rendered: `gate-narrative-checker-step`, its `fail-hidden-marker` state.

**Project AC-006 resolves to its first arm, and the second is deliberately not built.**
`_design.md` D2 chose *(a) mechanically forbidden* on 2026-08-17 with no conditions, so the
hidden-panel falsification is a branch not taken: observing a claim broken inside a
non-default panel would establish the property for one construct, one extractor and one
toolchain, and nothing would notice it regressing — a rule no implementation can fail. The
rejection is recorded here as well as in the spec because the tempting failure is a later
reader building it.

**Deferred, and named so nothing is quietly claimed.** DT-8's aside/constraint line is
HS-P0021's; a blanket ban makes it moot inside `docs/` only. The scope-band threshold, the
250-line page cap and the 40-character H1 cap stay review rules by decision. The recorded
`cargo xtask ci` failure-and-recovery project AC-003 requires is
`observed-failure-falsification`'s, on a *fence* rather than a panel, and the transcript in
the implementation report is not offered as that evidence. The six-item limits list is
`documented-blind-spots-and-their-proofs`'; this story wrote the three its own check creates.

**No ADR is owed and none was written.** No Accepted decision atom under `.kb/decisions/`
governs gate structure, documentation trees or fence discipline, and no `[FROZEN]` clause is
touched. `_design.md` is the design record DT-7's resolution required, and it already exists
and is signed off.
