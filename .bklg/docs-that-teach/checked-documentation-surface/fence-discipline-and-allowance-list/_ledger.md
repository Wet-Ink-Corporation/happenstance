---
item: HS-S0139
stage: implement
created: "2026-08-17T13:16:02.729Z"
updated: "2026-08-17T13:16:02.729Z"
---

# Acceptance ledger — No fence opts out of the check unnoticed

The machine-checkable Definition-of-Done ledger for this story (RFC §6.5). One row per spec
`AC-###`. Planning authors every row with `satisfied: false`; the implementer may only flip a row to
`satisfied: true` and MUST cite real evidence (a `file:line` and/or the verifying test id) — never
edit, remove, or re-word a criterion, and never flip a satisfied row back. `redkiln verify --grain
story` reads the fenced block below and blocks `implement → report` unless every spec AC is present,
`satisfied: true`, and carries non-placeholder evidence (and no AC that was satisfied on the base
branch has regressed). Scope changes are a human decision recorded through `redkiln advance`, not a
quiet ledger edit.

Two notes for whoever flips these rows. The mount point is the same for all seven — this story adds
checks *inside* the fence walk that `narrative-checker-mounted-with-pinned-path` (HS-S0138) creates,
and adds no module, step or subcommand of its own — so `satisfied: true` on a unit test alone is not
enough for AC-006: the Merge DoD in `spec.md` requires the checks observed in `cargo xtask narrative`'s
real output on a page carrying each defect. And the checker module's file name comes from HS-S0138
(`xtask/src/lint_narrative.rs`); if it landed under another name, correct the `verifying_test` paths
below to the file that actually exists and cite it — that is a path correction, not a scope change.

```yaml
- id: AC-001
  criterion: 'GIVEN an application author reading a fenced example and trusting it still compiles against the crate they installed, WHEN a contributor adds a fence with an empty info string to a page under `TREE`, THEN `cargo xtask narrative` reports `{TREE}/{page}:{line} — an untagged fence is compiled as Rust; tag it \`rust\` or \`text\`` and the gate fails — because rustdoc compiles an untagged fence as Rust regardless, so silence here means either prose is compiled by accident or Rust is compiled that nobody decided to check.'
  satisfied: false
  evidence: ""
  mount_point: "the bin-crate narrative checker module's `run()` fence walk (`xtask/src/lint_narrative.rs`), declared from `xtask/src/main.rs:64-70` beside `mod lint_constitution;` at `:65`, dispatched by `cargo xtask narrative` and run inside `cargo xtask ci` via `REQUIRED` (`xtask/src/main.rs:105`), `lint_steps` (`:799-826`) and `affected`'s unconditional file-reading lints (`xtask/src/affected.rs:116-125`)"
  verifying_test: "`xtask/src/lint_narrative.rs` → `mod tests::an_untagged_fence_is_rejected` (`cargo test -p xtask`), plus the defect observed in `cargo xtask narrative`'s output on a fixture page"
- id: AC-002
  criterion: 'GIVEN a reviewer who cannot be expected to know every spelling rustdoc accepts, WHEN a fence''s info string carries any part the walk does not enumerate — `ignore_me`, `norun`, a future rustdoc attribute, or an error code on a fence that is not `compile_fail` — THEN the walk names the whole info string as unrecognised and the gate fails, because the closed match has no accepting wildcard arm; a novel opt-out spelling is therefore a hard error and never a silent pass.'
  satisfied: false
  evidence: ""
  mount_point: "the bin-crate narrative checker module's `run()` fence walk (`xtask/src/lint_narrative.rs`), declared from `xtask/src/main.rs:64-70` beside `mod lint_constitution;` at `:65`, dispatched by `cargo xtask narrative` and run inside `cargo xtask ci` via `REQUIRED` (`xtask/src/main.rs:105`), `lint_steps` (`:799-826`) and `affected`'s unconditional file-reading lints (`xtask/src/affected.rs:116-125`)"
  verifying_test: "`xtask/src/lint_narrative.rs` → `mod tests::an_unrecognised_info_string_part_is_a_problem` and `mod tests::an_error_code_without_compile_fail_is_a_problem` (`cargo test -p xtask`)"
- id: AC-003
  criterion: 'GIVEN a reviewer who must be able to read, in one place and in full, every block on the narrative tree that no compiler read, WHEN a fence carries an `ignore`-class part and no `IGNORE_ALLOWANCES` entry names it, THEN it is a problem naming file and line and an `<!-- ignore: … -->` comment above it does not rescue it; and WHEN an entry does name that fence and carries a non-empty reason, THEN the fence passes and the walk reports nothing else about the page.'
  satisfied: false
  evidence: ""
  mount_point: "the bin-crate narrative checker module's `run()` fence walk (`xtask/src/lint_narrative.rs`), declared from `xtask/src/main.rs:64-70` beside `mod lint_constitution;` at `:65`, dispatched by `cargo xtask narrative` and run inside `cargo xtask ci` via `REQUIRED` (`xtask/src/main.rs:105`), `lint_steps` (`:799-826`) and `affected`'s unconditional file-reading lints (`xtask/src/affected.rs:116-125`)"
  verifying_test: "`xtask/src/lint_narrative.rs` → `mod tests::an_unlisted_ignore_fence_is_rejected`, `mod tests::a_comment_above_an_ignore_fence_does_not_permit_it`, `mod tests::a_listed_ignore_fence_passes`, `mod tests::an_allowance_with_an_empty_reason_is_a_problem` (`cargo test -p xtask`)"
- id: AC-004
  criterion: 'GIVEN a reviewer six months later auditing what the tree is still allowed to skip, WHEN an `IGNORE_ALLOWANCES` entry names a page or fence that no longer exists, or names a fence that is no longer `ignore`-class, THEN that entry is itself a problem naming the entry, so the list cannot accumulate standing permission for code nobody has.'
  satisfied: false
  evidence: ""
  mount_point: "the bin-crate narrative checker module's `run()` fence walk (`xtask/src/lint_narrative.rs`), declared from `xtask/src/main.rs:64-70` beside `mod lint_constitution;` at `:65`, dispatched by `cargo xtask narrative` and run inside `cargo xtask ci` via `REQUIRED` (`xtask/src/main.rs:105`), `lint_steps` (`:799-826`) and `affected`'s unconditional file-reading lints (`xtask/src/affected.rs:116-125`)"
  verifying_test: "`xtask/src/lint_narrative.rs` → `mod tests::a_stale_allowance_is_a_problem` and `mod tests::an_allowance_for_a_fence_that_no_longer_opts_out_is_a_problem` (`cargo test -p xtask`)"
- id: AC-005
  criterion: 'GIVEN a contributor who will keep reading the gate''s output only while it is right about their tree, WHEN a page displays fenced material inside a four-backtick block — the shape a page teaching how to tag a fence must use — THEN the walk steps over the entire block and reports nothing for the fences inside it; and every problem it does report names the page''s own path and the line of the offending fence, so the reader opens the file the message names.'
  satisfied: false
  evidence: ""
  mount_point: "the bin-crate narrative checker module's `run()` fence walk (`xtask/src/lint_narrative.rs`), declared from `xtask/src/main.rs:64-70` beside `mod lint_constitution;` at `:65`, dispatched by `cargo xtask narrative` and run inside `cargo xtask ci` via `REQUIRED` (`xtask/src/main.rs:105`), `lint_steps` (`:799-826`) and `affected`'s unconditional file-reading lints (`xtask/src/affected.rs:116-125`)"
  verifying_test: "`xtask/src/lint_narrative.rs` → `mod tests::four_backtick_fences_are_not_examples` (carried over from `xtask/src/lint_constitution.rs:862-868`) and `mod tests::a_problem_names_the_page_and_the_fence_line` (`cargo test -p xtask`)"
- id: AC-006
  criterion: 'GIVEN a contributor reading a CI log after the step failed, WHEN the walk finds n problems anywhere in the tree, THEN all n print — never truncated, never `… and N more`, never a bare non-zero exit — under the step''s banner on stderr in source order (path, then line) as `  {path}:{line} — {message}` with the location first and inside the ≤ 48-character prefix budget, and the count last as `bail!("{n} problem(s) in {TREE}")`; the output is append-only plain text with no screen clear, no cursor rewrite, no pager and no colour-only or TTY-only distinction, so the log reads identically to the terminal and the banner and earlier steps stay on screen; and the walk writes no file, so repairing the fence or adding an allowance returns the same command to green with nothing to clear.'
  satisfied: false
  evidence: ""
  mount_point: "the bin-crate narrative checker module's `run()` fence walk (`xtask/src/lint_narrative.rs`), reported through the module's single `bail!` in the shape of `xtask/src/lint_constitution.rs:169-198`, under the step banner `xtask/src/main.rs:864`"
  verifying_test: "`xtask/src/lint_narrative.rs` → `mod tests::problems_are_reported_in_source_order` and `mod tests::every_problem_is_reported_not_the_first` (`cargo test -p xtask`), plus `cargo xtask narrative` run on a fixture page carrying three distinct defects with its output compared against `_design.md` `## Composition`"
- id: AC-007
  criterion: 'GIVEN a contributor who must not read this green step as more than it is, WHEN the walk finds nothing, THEN the step adds no per-page or per-fence output — one summary line, no spinner, no progress ticker — and WHEN that contributor opens the checker module, THEN its `//!` docs state, *before* the checks, that a Rust example deliberately tagged `text` is neither compiled nor flagged and that this list narrows that hole rather than closing it, and that the narrative tree deliberately diverges from `lint_constitution`''s `<!-- ignore: … -->` form together with why; and nothing this story adds claims anywhere that a checked fence means the page teaches.'
  satisfied: false
  evidence: ""
  mount_point: "`xtask/src/lint_narrative.rs`'s `//!` module documentation and the same module's green-path summary line, printed by the `cargo xtask narrative` step the module story mounted (`xtask/src/main.rs:105`)"
  verifying_test: "`xtask/src/lint_narrative.rs` → `mod tests::the_module_docs_state_the_text_limit_and_the_divergence` (reads the module's own source with `include_str!`, the technique `xtask/src/constitution.rs` uses), plus an observed one-line green run of `cargo xtask narrative`"
```
