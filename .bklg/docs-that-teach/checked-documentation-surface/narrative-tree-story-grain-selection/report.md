---
item: "HS-S0137"
stage: report
created: "2026-08-17T22:45:00.000Z"
updated: "2026-08-17T22:45:00.000Z"
---

# Report — A prose-only change selects the package that compiles it

## Findings Ledger

The story's outcome as the review gate reads it. Five ACs, all satisfied by reachable
behaviour with a citation and a test or a recorded observation; nothing stubbed,
skipped or deferred.

| Finding | Evidence | Follow-up |
| ------- | -------- | --------- |
| **A change confined to the narrative tree now selects `xtask`.** The arm's third disjunct fires inside the existing outside-every-member branch, so `WORKSPACE_WIDE` still short-circuits above it. | `xtask/src/affected.rs:217-220`; `::tests::a_narrative_page_selects_xtask` (red: `left: {}` / `right: {"xtask"}`) | None. The structural fix for a *moved* tree — an error by name — is the project's AC-001 and belongs to `narrative-checker-mounted-with-pinned-path`. |
| **The arm is a rule about one directory, and the widening posture is untouched.** Neighbouring prose still reaches no package; an unrecognised path still returns every member. | `::tests::the_narrative_arm_does_not_widen_to_all_prose`; `the_relocated_trees_stay_inert`, `an_unrecognised_path_widens_rather_than_narrows`, `the_readme_selects_xtask`, `a_constitution_atom_selects_xtask`, `the_lockfile_selects_everything` — all green and unedited | None. EC-003 and EC-004 name the two narrowing refinements that will look tempting later (an extension filter, a tighter prefix match) and forbid both. |
| **`"docs/"` left `INERT` rather than being shadowed**, and the predicate is asserted directly so a future reordering of the `else if` chain fails a test instead of silently un-compiling the tree. | `xtask/src/affected.rs:276-293` (removed) and `:267-274` (two deliberately-absent trees, with the reason); `::tests::the_narrative_tree_is_no_longer_inert` (red: `assertion failed: !is_inert(...)`) | None. |
| **Every comment the change falsified says something true**, and the arm states what selection does not prove. | `xtask/src/affected.rs:212-216`, `:136-142`, `:267-274`, `:229-244`; `rg -n '"docs/"' xtask/src/affected.rs` returns the arm's literal and no `INERT` entry | RS-81-1 splits this obligation into a test plus a statement; the tests are the three rows above and this row is the statement. It is a review row by construction — no `#[test]` can assert a comment is true. |
| **The behaviour is observable on the real story-grain path**, in the composition that already exists. | `_observed-affected-run.md`: `=== affected packages ===`, `1 file(s) changed against HEAD`, `  xtask`, no empty-selection line, three step banners actually running (231 tests), `affected gate passed` last | The transcript is dated evidence of one observation, not a golden file; nothing in the suite compares against it, so it cannot rot into a failing test. Its mechanical successor is `cargo xtask ci --fast` continuing to pass. |
| **Selection is not compilation, and nothing here claims a page teaches.** | `xtask/src/affected.rs:238-242`; project DoD items 7 and 8 | The compiling half is the slice-mate's `cargo xtask narrative-doctests`; comprehension is HS-P0024's friction log and is proven by nothing in this project. |
| **No gate wiring was added, and that is a decision rather than an omission.** | No `Step`, no `REQUIRED` member, no subcommand, no `probe`, no `print_help` edit. The mount at `xtask/src/main.rs:64`, `:683-688`, `:729` was already complete | `cargo xtask ci --fast` green proves `steps_named` has nothing new to panic on. |

**Mount point.** `xtask/src/affected.rs` — the outside-every-member arm of
`affected_packages` (`:212-247`) and `is_inert`'s `INERT` list (`:275-293`). Reached in
production by `affected::run` (`:112`) via the `Some("affected")` dispatch at
`xtask/src/main.rs:683-688`, declared at `xtask/src/main.rs:64`, and invoked by
`.redkiln/config.yaml:40` at every `redkiln advance` seam. Nothing new is mounted; an
existing mount changes behaviour, which is what makes this story structural rather
than cosmetic — every one of the eight downstream stories in this initiative runs that
command at its own advance seam, and until this landed each of them was blind to a
prose-only change.

**Deferred:** nothing from this story's AC set. Deliberately out of scope and tracked
elsewhere: the missing-tree error by name (`narrative-checker-mounted-with-pinned-path`,
project AC-001), and whether the milestone-2 checker joins `affected::run`'s
unconditional list at `:120-125` — a live open decision with a recommendation attached
(`_decomposition.md:337-343`) that belongs to the story creating the checker, and
touching it here would settle another story's question in passing.

## Acceptance

| AC | Status | Verification |
| -- | ------ | ------------ |
| AC-001 | Satisfied | `xtask/src/affected.rs::tests::a_narrative_page_selects_xtask`, plus the observed run naming `xtask` |
| AC-002 | Satisfied | `the_narrative_arm_does_not_widen_to_all_prose`; `a_top_level_prose_file_selects_nothing` (renamed, assertion verbatim); five neighbouring tests green and unedited |
| AC-003 | Satisfied | `the_narrative_tree_is_no_longer_inert` asserting the predicate directly; `rg -n '"docs/"' xtask/src/affected.rs` shows no `INERT` entry |
| AC-004 | Satisfied | The four rewritten sites, reviewed against RS-81-1; `cargo xtask lint-constitution` and `cargo test -p xtask --doc` green |
| AC-005 | Satisfied | `_observed-affected-run.md` — the parsed output, not the exit status (RS-81-4); `cargo xtask ci --fast` green |

## Knowledge Harvest

Candidates for `.kb/` at closeout — recorded, not promoted here.

- **A cross-target duplication cannot be abstracted away inside one package, and the
  honest move is to state it.** `xtask` has a lib target and a bin target; the
  narrative harness is in the first and the selector in the second, so a seven-character
  prefix has to be written twice. The alternative — a `pub` seam across the two targets
  — costs more than the duplication and makes a private implementation detail public.
  The same fact shows up as a hard rustdoc error on any intra-doc link across the two.
  This is the third judgement in `affected.rs` recorded as a comment because no type can
  hold it; the module's whole safeguard model is worth one concept atom.
- **A gate whose grain is "which packages could this diff have broken" is blind to
  every corpus that is source without being code.** `README.md`, `standards/rust/` and
  now `docs/` each needed a hand-written arm, discovered separately, each time after the
  corpus already existed. The pattern — *a prose tree compiled by exactly one package
  needs a selection arm in the same change that makes it compiled* — is a playbook atom,
  and the fourth instance should not have to be discovered a fourth time.
- **A transcript that cannot be produced is worth recording as unobtainable.** The
  pre-story counter-observation for AC-005 is unreachable on this tree, because
  reconstructing it requires editing the very file whose directory drives the selection.
  Saying so, with the unit-test evidence that stands in its place, is the difference
  between a limit and a gap.
