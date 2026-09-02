---
item: "HS-S0136"
stage: report
created: "2026-08-17T22:30:00.000Z"
updated: "2026-08-17T22:30:00.000Z"
---

# Report — The narrative tree exists and every fence in it compiles, mandatorily

## Findings Ledger

The story's outcome as the review gate reads it. Every AC is satisfied by reachable
behaviour with a citation and a test; nothing is deferred, stubbed or fixture-pinned.

| Finding | Evidence | Follow-up |
| ------- | -------- | --------- |
| **The tree exists and its one page is composed as the signed-off design writes it.** H1 26 chars, reserved answered-need slot present and empty, `ES-40` inline, fence adjacent to its claim, two-scope level-3 band last, 28 source lines, no hidden marker, 25-character path. | `docs/append-conditions.md:1-28`; `xtask/src/narrative_doctests.rs::tests::the_fixture_page_carries_no_hidden_marker_and_fits_the_path_budget` | The editorial budgets (H1 cap, 250-line cap, band order) stay **review** rules by `_design.md` `## Open questions` item 2. Two deviations from the design's fenced block are stated in the implementation report's Notes and are the owner's to accept or amend. |
| **Every Rust fence in the tree is compiled against the real workspace crates.** One `#[cfg(doctest)] mod` per page; `include_str!` resolves at compile time, so moving the page without editing the harness is a compile error. | `xtask/src/narrative.rs:68-71`; `cargo test --locked -p xtask --doc -- narrative::` names `narrative::append_conditions (line 9)` | Observed-fail recorded here (`store.len()` to `store.length()` fails the step at `docs/append-conditions.md:13:18`); the **recorded** observed-fail/observed-recover procedure over the whole gate is `observed-failure-falsification`'s. |
| **The harness is declared from the lib target, and the mis-mount is rejected loudly.** | `xtask/src/lib.rs:36`; `xtask/src/narrative_doctests.rs:156-172`; test `a_listing_with_no_narrative_doctest_is_a_problem`. Confirmed end to end by commenting the `mod` out and watching the step fail. | None. This guard is the only mechanism in the project that can see CR-1. |
| **The compile is a mandatory gate step named as a claim, ordered so its banner answers one question.** `probe: None`, `--locked`, step-scoped `RUSTDOCFLAGS`, immediately before the constitution's step. | `xtask/src/main.rs:481-509`; four passing step-shape tests; observed `cargo xtask ci --fast` output | None. The constitution step is deliberately **not** filtered in compensation — that would drop the repository README's doctest out of the gate. |
| **Zero matching doctests is a hard failure, and a green step prints what it enumerated.** | `xtask/src/narrative_doctests.rs:92-113, 120-138, 156-172`; three passing listing tests; observed `  1 page(s)' examples enumerated` | The page-on-disk to module-in-harness comparison is deliberately **not** here; it is `narrative-checker-mounted-with-pinned-path`'s AC-005 and belongs in one place. |
| **The mount is complete in all four places**, and half of it is a panic rather than a silent omission. | `xtask/src/main.rs:67, 481-509, 712, 795-799`; tests `the_gate_can_select_the_narrative_step_by_name`, `the_narrative_step_is_not_a_lint_step` | None. |
| **The index routes to the page, and the pin-by-path paragraph stopped being false.** | `docs/README.md:13-15` (narrative table, first on the page) and `:30-37` (three gate-read trees); two passing tests | The opening sentence was corrected too — a third edit, stated in the implementation report's Notes. |
| **Both new modules state their limits before anything else**, ending in one unhedged sentence that the step is silent about teaching. | `xtask/src/narrative.rs:3-35`, `xtask/src/narrative_doctests.rs:3-33`; test `the_new_modules_state_their_limits_first` | Limit 5 records the `RUSTDOCFLAGS`-through-an-extra-hop behaviour as **unmeasured here** rather than inheriting `constitution.rs`'s finding. Measuring it is `documented-blind-spots-and-their-proofs`'. |
| **Nothing in the gate regressed and no dependency was added.** | `cargo xtask ci --fast` green; `cargo test -p xtask --all-features` 231/0; `cargo xtask lint-constitution` `27 atoms, all consistent`; empty `git diff --stat -- xtask/Cargo.toml Cargo.lock` | Nine constitution citations into `xtask/src/main.rs` had to be repointed by line number — the pin-by-path mechanism firing, recorded in Notes. |
| **A green banner here is evidence of nothing about teaching**, and every artifact this story wrote says so. | `xtask/src/narrative.rs:34`, `xtask/src/narrative_doctests.rs:32`, the executive summary of `spec.md`, this row | HS-P0024's friction log is the instrument for comprehension and is not substitutable. |

**Mount point.** `xtask/src/main.rs:481-509` (the `REQUIRED` array) with
`xtask/src/lib.rs:36` as the co-required doctest root, plus the subcommand dispatch at
`xtask/src/main.rs:712` and its help entry at `:795-799`. Neither root is sufficient
alone: a step in `REQUIRED` with the harness in the bin crate compiles nothing, and a
harness in the lib crate with no step is reached only under the constitution's banner.

**Deferred:** nothing from this story's AC set. Out of scope by design and tracked
elsewhere: the story-grain selection arm for `docs/` (slice-mate
`narrative-tree-story-grain-selection`, landing immediately after), the pinned tree
constants and the bidirectional registration check
(`narrative-checker-mounted-with-pinned-path`), the fence walk and allowance list
(`fence-discipline-and-allowance-list`), the `HIDDEN_MARKERS` tree sweep
(`hidden-content-resolution`), the recorded observed-fail procedure
(`observed-failure-falsification`), and the full limits list with its re-run
`RUSTDOCFLAGS` probe (`documented-blind-spots-and-their-proofs`).

## Acceptance

| AC | Status | Verification |
| -- | ------ | ------------ |
| AC-001 | Satisfied | `the_fixture_page_carries_no_hidden_marker_and_fits_the_path_budget` + review against `_design.md` `## The doctest` |
| AC-002 | Satisfied | `cargo test --locked -p xtask --doc -- narrative::`, plus the recorded observed-fail on `MemoryEventStore::len` |
| AC-003 | Satisfied | `a_listing_with_no_narrative_doctest_is_a_problem`, plus the mis-mount reproduced and observed |
| AC-004 | Satisfied | `the_narrative_step_is_required_and_unprobed`, `..._precedes_the_constitution_step`, `..._carries_rustdocflags_in_its_own_env`, `..._passes_locked`, plus observed `ci --fast` output |
| AC-005 | Satisfied | `an_empty_listing_is_a_problem`, `a_listing_with_no_narrative_doctest_is_a_problem`, `a_listing_with_one_narrative_doctest_is_accepted_and_counted` |
| AC-006 | Satisfied | `the_gate_can_select_the_narrative_step_by_name`, `the_narrative_step_is_not_a_lint_step`, plus observed `cargo xtask narrative-doctests` and help output |
| AC-007 | Satisfied | `the_index_names_the_narrative_tree_as_gate_read`, `the_narrative_table_precedes_the_pointer_out_table` |
| AC-008 | Satisfied | `the_new_modules_state_their_limits_first` |
| AC-009 | Satisfied | `cargo xtask ci --fast`, `cargo xtask affected --base main`, `cargo test -p xtask --all-features`, `cargo xtask lint-constitution`, empty dependency diff |

## Knowledge Harvest

Candidates for `.kb/` at closeout — recorded, not promoted here.

- **A `file:line` citation corpus makes every insertion into the cited file a
  cross-tree edit.** Adding one `Step` to `xtask/src/main.rs` broke nine constitution
  citations. That is the pin working, but it is a cost nothing had measured before, and
  it will recur on every future gate step. Worth an atom, and worth asking whether the
  citation checker should offer a `--write` that repoints line numbers the way
  `spec-trace --write` regenerates its table.
- **The vacuity guard generalises.** This is the third place in the repository
  (`package.rs`, `proof.rs`, now `narrative_doctests.rs`) where the pattern is *assert
  the names out of `--list`, then run them*, each time because a cargo subcommand exits
  0 over an empty set. Three instances is where a playbook atom earns its keep.
- **Two targets in one package do not share modules, and intra-doc links prove it.**
  An intra-doc link to `crate::lint_constitution` written from `xtask/src/narrative.rs`
  is a hard rustdoc error, because the harness is in the lib target and the lint is in
  the bin. The same fact is the whole of CR-1 and the whole of why the slice-mate
  cannot import a shared `TREE` constant. Worth one concept atom stating it once.
