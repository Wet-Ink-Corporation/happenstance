---
item: "HS-S0136"
stage: implement
created: "2026-08-17T13:16:01.147Z"
updated: "2026-08-17T13:16:01.147Z"
---

# Acceptance ledger — The narrative tree exists and every fence in it compiles, mandatorily

The machine-checkable Definition-of-Done ledger for this story (RFC §6.5). One row per spec
`AC-###`. Planning authors every row with `satisfied: false`; the implementer may only flip a row to
`satisfied: true` and MUST cite real evidence (a `file:line` and/or the verifying test id) — never
edit, remove, or re-word a criterion, and never flip a satisfied row back. `redkiln verify --grain
story` reads the fenced block below and blocks `implement → report` unless every spec AC is present,
`satisfied: true`, and carries non-placeholder evidence (and no AC that was satisfied on the base
branch has regressed). Scope changes are a human decision recorded through `redkiln advance`, not a
quiet ledger edit.

```yaml
- id: AC-001
  criterion: "**GIVEN** an application author in *the first fifteen minutes*, who has run `cargo add happenstance` and opened `docs/` looking for how a consistency boundary is actually expressed, **WHEN** they open the page the index routes them to, **THEN** they meet one page composed exactly as the signed-off design writes it — a ≤ 40-character H1, the reserved answered-need line present and empty, the `ES-40` citation inside the sentence that depends on it rather than in a footer, each `rust` fence immediately after the sentence it demonstrates, the two-scope band last and visible at level 3 in alphabetical order, ≤ 250 source lines, fence lines ≤ 80 columns, prose source wrapped ≤ 90 — and **nothing they must click, expand or unfold to read**."
  satisfied: true
  evidence: "docs/append-conditions.md:1-28 — H1 `Appending under a condition` (26 chars), the reserved answered-need slot at :3 present and empty, `(ES-40)` inline at :7 in the sentence it qualifies, the `rust` fence at :9-14 immediately after that sentence, the two-scope level-3 band last at :19-28 in alphabetical order (postgres, sqlite), 28 source lines, longest line 86 columns, fence lines ≤ 47. Machine half: xtask/src/narrative_doctests.rs::tests::the_fixture_page_carries_no_hidden_marker_and_fits_the_path_budget (passing) asserts none of the seven HIDDEN_MARKERS tokens is present and that the 25-character repo-relative path fits the 32-character budget. Editorial half reviewed against `_design.md` `## The doctest`; the two deviations are stated in the implementation report's Notes."
  mount_point: "docs/append-conditions.md — the `narrative-page` surface, reached from `docs/README.md`'s narrative table"
  verifying_test: "xtask/src/narrative_doctests.rs::tests::the_fixture_page_carries_no_hidden_marker_and_fits_the_path_budget (plus review against `_design.md` `## The doctest` for the editorial budgets the design leaves as review rules)"

- id: AC-002
  criterion: "**GIVEN** an application author who copies the append-condition example out of a narrative page into their own program, **WHEN** a maintainer later removes, renames or re-signatures the public item that example calls, **THEN** the gate fails on that page before the change can merge, so the reader never meets an example that no longer compiles against the crate they installed."
  satisfied: true
  evidence: "xtask/src/narrative.rs:68-71 — `#[cfg(doctest)] mod append_conditions { #![doc = include_str!(\"../../docs/append-conditions.md\")] }`. Observed: `cargo test --locked -p xtask --doc -- --list` names `xtask\\src\\../../docs/append-conditions.md - narrative::append_conditions (line 9)`, and the filtered run compiles it (1 passed). Observed-fail: renaming `store.len()` to `store.length()` in the fence made the step exit non-zero with `no method named `length` found` at `xtask\\src\\../../docs/append-conditions.md:13:18`, proving the fence resolves against `happenstance_core::MemoryEventStore` (crates/happenstance-core/src/memory.rs:171) rather than a copy; reverted."
  mount_point: "xtask/src/narrative.rs — one `#[cfg(doctest)] mod` per page, declared from xtask/src/lib.rs:28"
  verifying_test: "cargo test --locked -p xtask --doc -- narrative:: (names `xtask::narrative::append_conditions (line N)`)"

- id: AC-003
  criterion: "**GIVEN** a contributor wiring the harness for the first time, **WHEN** they declare `mod narrative;` from the bin crate instead of the lib crate, **THEN** the mistake is rejected loudly, rather than producing a repository that compiles clean while nothing on any page is ever compiled."
  satisfied: true
  evidence: "xtask/src/lib.rs:36 — `mod narrative;` beside `mod constitution;` (:28), in the doctest root. Observed rejection of the CR-1 mis-mount: commenting that line out makes `cargo run -p xtask -- narrative-doctests` fail with `no `narrative::` doctest was listed …`, naming `xtask/src/narrative.rs`, `xtask/src/lib.rs` and the `xtask/src/main.rs` bin-crate mis-mount; restored. Unit-level statement: xtask/src/narrative_doctests.rs::tests::a_listing_with_no_narrative_doctest_is_a_problem (passing)."
  mount_point: "xtask/src/lib.rs:28 — `mod narrative;` beside `mod constitution;`, the doctest root (CR-1)"
  verifying_test: "xtask/src/narrative_doctests.rs::tests::a_listing_with_no_narrative_doctest_is_a_problem (plus `cargo test --locked -p xtask --doc -- --list` listing a `narrative::` doctest)"

- id: AC-004
  criterion: "**GIVEN** a reviewer reading a red CI log who needs to know which half of the documentation machine broke, **WHEN** a Rust fence on a narrative page stops compiling, **THEN** the first banner they meet is `=== the narrative tree's examples compile ===` — never the constitution's — the step is reached on every runner with no tool to install, and its `RUSTDOCFLAGS` are visible in the step rather than smuggled into the environment of every other step."
  satisfied: true
  evidence: "xtask/src/main.rs:481-509 — the `Step` named `narrative_doctests::STEP` (xtask/src/narrative_doctests.rs:67), `probe: None`, `env: &[(\"RUSTDOCFLAGS\", \"-D warnings\")]`, `--locked` in its argv, inserted immediately before `the constitution's examples compile` (:510-523). Tests, all passing: xtask/src/narrative_doctests.rs::tests::{the_narrative_step_is_required_and_unprobed, the_narrative_step_precedes_the_constitution_step, the_narrative_step_carries_rustdocflags_in_its_own_env, the_narrative_step_passes_locked}. Observed `cargo xtask ci --fast`: `=== the narrative tree's examples compile ===` prints immediately above `=== the constitution's examples compile ===`, with no `skipped` line."
  mount_point: "xtask/src/main.rs:105 — the `REQUIRED` array, entry immediately before `the constitution's examples compile` (:488-493)"
  verifying_test: "xtask/src/narrative_doctests.rs::tests::{the_narrative_step_is_required_and_unprobed, the_narrative_step_precedes_the_constitution_step, the_narrative_step_carries_rustdocflags_in_its_own_env, the_narrative_step_passes_locked}"

- id: AC-005
  criterion: "**GIVEN** a contributor who mis-mounts the harness, deletes the last page, or adds a page nobody registered, **WHEN** the gate runs, **THEN** the step **fails** naming the expected harness path and the CR-1 mis-mount, and prints how many pages it enumerated when it passes — so a green banner can never mean \"there was nothing to compile\"."
  satisfied: true
  evidence: "xtask/src/narrative_doctests.rs:156-172 — `enumerated_pages` bails when the listing names no `narrative::` doctest, naming `xtask/src/narrative.rs`, `xtask/src/lib.rs` and the `xtask/src/main.rs` mis-mount; :92-113 prints `  {n} page(s)' examples enumerated` before running the filtered tests; :120-138 (`list`) hard-errors with its own message when the enumeration itself fails, so EC-002 and EC-004 can never share a diagnosis. Tests, all passing: ::tests::{an_empty_listing_is_a_problem, a_listing_with_no_narrative_doctest_is_a_problem, a_listing_with_one_narrative_doctest_is_accepted_and_counted}. Observed: the step prints `  1 page(s)' examples enumerated` inside `cargo xtask ci --fast`."
  mount_point: "xtask/src/narrative_doctests.rs — invoked by the `REQUIRED` step's argv as `cargo run -p xtask -- narrative-doctests`"
  verifying_test: "xtask/src/narrative_doctests.rs::tests::{an_empty_listing_is_a_problem, a_listing_with_no_narrative_doctest_is_a_problem, a_listing_with_one_narrative_doctest_is_accepted_and_counted}"

- id: AC-006
  criterion: "**GIVEN** a contributor iterating on a page who does not want to pay for the whole gate, **WHEN** they run `cargo xtask narrative-doctests` or read `cargo xtask` with no arguments, **THEN** the subcommand runs the same check the gate runs and is listed in the help beside the others — and a half-mounted step (named in the gate, unreachable by hand, or vice versa) is a test or build failure rather than a silent omission."
  satisfied: true
  evidence: "xtask/src/main.rs:67 (`mod narrative_doctests;`), :712 (`Some(\"narrative-doctests\") => narrative_doctests::run()`, beside `proof-artefact` at :711) and :795-799 (the `print_help()` entry, naming the vacuity argument). Deliberately absent from `lint_steps()` (:835-844): that family reads files, this one compiles. Tests, both passing: xtask/src/narrative_doctests.rs::tests::{the_gate_can_select_the_narrative_step_by_name, the_narrative_step_is_not_a_lint_step}. Observed: `cargo run -p xtask -- narrative-doctests` prints `  1 page(s)' examples enumerated` and exits 0; `cargo xtask` with no arguments lists the subcommand."
  mount_point: "xtask/src/main.rs:642-707 — the subcommand dispatch arm — and :718+ — the `print_help()` register"
  verifying_test: "xtask/src/narrative_doctests.rs::tests::{the_gate_can_select_the_narrative_step_by_name, the_narrative_step_is_not_a_lint_step}"

- id: AC-007
  criterion: "**GIVEN** the evaluator in *survive the second question*, who lands on `docs/README.md` after one answer and needs somewhere to go next, and **GIVEN** the contributor who reads the same file to learn which trees the gate pins by path, **WHEN** either opens it, **THEN** the narrative table is the **first** table on the page and routes to the page by the name in its H1, the paragraph naming the gate-read trees names the narrative tree as one of them, and no breadcrumb, hand-rolled TOC or prev/next footer was added."
  satisfied: true
  evidence: "docs/README.md:13-15 — the two-column narrative table (`| Page | Read it at |`, one row routing to `append-conditions.md` by the name in its H1), placed above the existing pointer-out table at :17-28, whose ten rows are unchanged in the diff. :30-37 — the pin-by-path paragraph rewritten from `Two of those are read by the gate` to `Three trees are read by the gate`, naming the narrative tree, `xtask/src/narrative.rs` and `cargo test -p xtask --doc`. No breadcrumb, hand-rolled TOC or prev/next footer was added. Tests, both passing: xtask/src/narrative_doctests.rs::tests::{the_index_names_the_narrative_tree_as_gate_read, the_narrative_table_precedes_the_pointer_out_table}."
  mount_point: "docs/README.md:12-29 — the `narrative-tree-index` surface: the narrative table above the existing pointer-out table, and the pin-by-path paragraph"
  verifying_test: "xtask/src/narrative_doctests.rs::tests::{the_index_names_the_narrative_tree_as_gate_read, the_narrative_table_precedes_the_pointer_out_table}"

- id: AC-008
  criterion: "**GIVEN** any contributor or downstream project reading the new machinery for the first time, **WHEN** they open `xtask/src/narrative.rs` or `xtask/src/narrative_doctests.rs`, **THEN** the **first** thing in the module docs is what the check does not verify — six limits, ending in one unhedged sentence saying this step is silent about whether the page teaches — so the green step is never inherited as evidence of teachability."
  satisfied: true
  evidence: "xtask/src/narrative.rs:3-35 and xtask/src/narrative_doctests.rs:3-33 — `# What this does not verify` is the first heading in each module's docs, carrying six limits and ending in the unhedged sentence (`narrative.rs:34`, `narrative_doctests.rs:32`) that the step is silent about whether the page teaches. Limit 5 records the `RUSTDOCFLAGS`-through-an-extra-`cargo run`-hop behaviour as unmeasured here and names `documented-blind-spots-and-their-proofs` as its owner, rather than inheriting `xtask/src/constitution.rs`'s finding. Test: xtask/src/narrative_doctests.rs::tests::the_new_modules_state_their_limits_first (passing) reads both files as text and asserts the heading precedes every other heading in each. `cargo xtask lint-constitution` prints `27 atoms, all consistent`; `cargo test -p xtask --doc` is green."
  mount_point: "xtask/src/narrative.rs and xtask/src/narrative_doctests.rs — the modules' own opening docs, in the shape of xtask/src/lint_constitution.rs:9-13"
  verifying_test: "xtask/src/narrative_doctests.rs::tests::the_new_modules_state_their_limits_first"

- id: AC-009
  criterion: "**GIVEN** a contributor on a fresh clone with nothing installed beyond the pinned toolchain — no `mdbook`, no site generator, no local state — **WHEN** they run `cargo xtask ci --fast` (and, before the project closes, `cargo xtask ci`), **THEN** it is green with the new banner in its output and **no `skipped` line for it**, the narrative material is built and read as an ordinary step rather than a manual one, nothing already in the gate regresses, and no dependency was added to pay for it."
  satisfied: true
  evidence: "`cargo xtask ci --fast` observed green on this tree: `=== the narrative tree's examples compile ===` followed by `  1 page(s)' examples enumerated`, immediately above `=== the constitution's examples compile ===`, with no `skipped` line anywhere and `all required checks passed (--fast: 4 optional step(s) not run)` last. Regression tier: `cargo test -p xtask --all-features` 231 passed / 0 failed, including xtask/src/affected.rs::tests::a_docs_only_change_selects_nothing; `cargo xtask lint-constitution` prints `27 atoms, all consistent`; `cargo xtask affected --base main` selects `xtask` and prints `affected gate passed`. Zero-dependency tier: `git diff --stat -- xtask/Cargo.toml Cargo.lock` is empty, and no `book.toml`, `book/`, `site/` or `.css` path appears in the diff."
  mount_point: "xtask/src/main.rs:105 — the `REQUIRED` array, reached by `run_fast` (:853-860) and by the full run (:829)"
  verifying_test: "cargo xtask ci --fast and cargo xtask ci (observed, recorded in the implementation report); cargo test -p xtask (xtask/src/affected.rs::tests::a_docs_only_change_selects_nothing still passing); cargo xtask lint-constitution; empty `git diff --stat -- xtask/Cargo.toml Cargo.lock` for the dependency tables"
```
