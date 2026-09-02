---
item: "HS-S0139"
stage: report
created: "2026-08-18T00:20:00.000Z"
updated: "2026-08-18T00:20:00.000Z"
---

# Report — No fence opts out of the check unnoticed

## Findings Ledger

Seven ACs, all satisfied by reachable behaviour: sixteen unit tests observed red before the
walk existed, plus one recorded run of the real step over a page carrying three of the
defects. Nothing stubbed, nothing skipped, nothing deferred out of this story's set.

| Finding | Evidence | Follow-up |
| ------- | -------- | --------- |
| **AC-001 — an untagged fence is rejected by page and line.** rustdoc compiles it as Rust regardless, so silence means either prose is compiled by accident or Rust is compiled that nobody decided to check. | `xtask/src/lint_narrative.rs:463-490`; `::tests::an_untagged_fence_is_rejected` (red: `got: []`), `a_text_tagged_fence_is_neither_compiled_nor_flagged`. Observed: `docs/three-defects.md:7 — an untagged fence is compiled as Rust; tag it \`rust\` or \`text\`` | None. |
| **AC-002 — the info string is matched exhaustively, with no accepting wildcard.** A novel opt-out spelling is a hard error rather than a silent pass. | `:492-517` (the `_ => recognised = false` arm at `:510` is the mechanism), `:583-587`; `::tests::an_unrecognised_info_string_part_is_a_problem` over `ignore_me` / `norun` / `edition2027`, `the_enumerated_info_string_parts_are_accepted`, `an_error_code_without_compile_fail_is_a_problem`, `a_compile_fail_claiming_a_code_the_prose_never_names_is_a_problem` | The last of those was **green against the copied implementation and is now genuinely failing**: `page.text.contains(code)` was satisfied by the fence's own info string. `names_outside_a_fence_marker` (`:576-581`) fixes it in the copy. `lint_constitution::check_fences` still carries the original; out of scope here by decision. |
| **AC-003 — an `ignore` fence is permitted only by an enumerated entry, never by a comment.** | `:519-556`, `:596-612`; `::tests::an_unlisted_ignore_fence_is_rejected`, `a_comment_above_an_ignore_fence_does_not_permit_it` (the named wrong implementation — the copied parser carries no `preceding` field at all), `a_listed_ignore_fence_passes`, `an_anchored_allowance_survives_an_insertion_above_the_fence`, `an_allowance_with_an_empty_reason_is_a_problem`. Observed at `docs/three-defects.md:13` | Both halves of "line-or-anchor" work. The `const`'s own comment recommends the anchor, because a line-keyed entry drifts the moment a paragraph is inserted above its fence. |
| **AC-004 — the list is swept in reverse, so it cannot accumulate permission for code nobody has.** Four failure shapes, each with its own message: stale, no-longer-opting-out, malformed, duplicated. | `:396-402` and `:534-566` (the `Usage` bookkeeping recorded during the forward pass), `:620-665` (the sweep); `::tests::a_stale_allowance_is_a_problem`, `an_allowance_for_a_fence_that_no_longer_opts_out_is_a_problem`, `a_malformed_allowance_is_a_problem`, `a_duplicate_allowance_is_a_problem` | EC-005 is reported *specifically* rather than as bare staleness, because it is the drift most likely to happen and the one that otherwise reads as the checker being wrong. |
| **AC-005 — no false positive, and the location is the page.** A four-backtick block is stepped over whole; every problem names the page and the fence's line, never the harness. | `:407-460`; `::tests::four_backtick_fences_are_not_examples` (asserted through the whole walk, not only the parser), `a_problem_names_the_page_and_the_fence_line`, `an_unterminated_fence_is_a_problem` | The unpaired-opener case (EC-002) is a hole the precedent parser has: it drops the opener, so its info string is never examined. Fixed in the copy only. |
| **AC-006 — every problem, in source order, never truncated, append-only plain text.** | `:667-682` (per-page line sort before composing), `:688-703` (no early return); `::tests::problems_are_reported_in_source_order`, `every_problem_is_reported_not_the_first` (40 → 40 lines). Observed: five problems then `xtask failed: 5 problem(s) in docs`, exit 1, matching `_design.md` `## Composition` line for line | The walk contains no ANSI escape, no `fs::write` and no early `return`, so piping to a file loses nothing and re-running after a fix is the whole undo. |
| **AC-007 — the green surface is unchanged and the record is in the module's own docs.** | `:28-34` (the `text` back door, inside `# What this does not verify`, before any check) and `:81-100` (the divergence from `lint_constitution`'s comment form, with its argument); `::tests::the_module_docs_state_the_text_limit_and_the_divergence` (reads the module with `include_str!` and asserts all three sentences precede `fn check_fences`), `a_clean_tree_still_reports_one_line_after_the_fence_walk`. `cargo xtask narrative` over the real tree prints one line | Nothing this story adds claims a checked fence means the page teaches. The six-item limits list stays `documented-blind-spots-and-their-proofs`'. |

**Mount point.** No new mount. This story adds checks *inside* the pass
`narrative-checker-mounted-with-pinned-path` created: `check_fences` is called from
`check_page` (`xtask/src/lint_narrative.rs:667-682`), which `problems` (`:688-703`) folds over
the pages, reported by that module's single `bail!` under the step already in `REQUIRED`
(`xtask/src/main.rs:526-553`). `REQUIRED` gains no entry and the dispatch gains no arm in this
diff, which is AC-006's own review clause. Surface rendered: `gate-narrative-checker-step`,
states `pass`, `fail-one` and `fail-many`.

**Deferred, and named rather than implied.** `HIDDEN_MARKERS` and DT-7's enforcement land in
this same walk as `hidden-content-resolution`. The `text` back door is *narrowed and
documented*, not closed — walking a `text`-tagged broken fixture through the gate to prove the
limit is real belongs to `documented-blind-spots-and-their-proofs`. The observed
`cargo xtask ci` failure-and-recovery that project AC-003 requires is
`observed-failure-falsification`'s; the three-defect transcript recorded here is the *step's*
output on a temporary page, and is not offered as that evidence.

**Two defects found in `xtask/src/lint_constitution.rs` and deliberately not fixed there.**
Its `compile_fail` code check is satisfied by the fence's own info string, and its `fences()`
drops an unpaired opener so that opener's info string is never examined. Both are fixed in
this module's copy. RS-81-3 scopes a scanner to the directory whose behaviour it constrains,
and `_storymap.md` forbids editing that file here; a reviewer who wants them fixed there is
looking at a separate story.
