---
item: "HS-S0142"
stage: implement
created: "2026-08-17T13:16:04.504Z"
updated: "2026-08-17T13:16:04.504Z"
---

# Implementation Report — A clause id a page cites either resolves or fails the gate

## TDD Evidence

The tests were written first, against a production half that was the **named wrong
implementation** rather than an absent one, so the red run is an assertion failure per row
instead of one compile error:

* `clause_citations` returned `Vec::new()` — the checker that silently non-matches, which is
  the posture `spec_trace::citations` takes (`:2215-2260`) and the one this module must not.
* `check_citations` was a no-op — a page citing `ES-99` passed, which is the falsifier
  `_decomposition.md:485` names and this story exists to close.

The wiring was real from the first run: the `Clauses` struct, the single `clause_ids` call in
`check`, the `read_page` seam, and the deletion of the foundation's
`#[cfg_attr(not(test), expect(dead_code, …))]` all landed with the stub, because a red run
against an unwired module would have been a compile error rather than evidence.

Red run: `cargo test --locked -p xtask --bin xtask lint_narrative::` → **87 passed; 7 failed**.
Green run, after the scanner and the classifier: **94 passed; 0 failed**.

| AC | Test | Red → Green |
| -- | ---- | ----------- |
| AC-001 | `the_recognised_families_are_derived_from_the_resolved_set`, `the_specification_is_resolved_once_per_run`, `the_resolvers_dead_code_marker_is_gone` | RED on the first: `got: []`, left 0 right 1. GREEN once the families were derived by `split_once('-')` over the resolved set — the fictional `ZZ-1` makes `ZZ-9` a *dangling id*, which a literal six-prefix list cannot produce. The other two were green from the wiring and are named below. |
| AC-002 | `a_citation_naming_a_declared_clause_is_not_a_problem` | Green from the start — the negative half of a directional pair. It exists to fail an over-broad implementation (one that reports every citation, or one whose resolution never matches), and it does: the same page with `ES-99` reports, which `the_boundaries_are_both_load_bearing` asserts. |
| AC-003 | `a_dangling_id_names_the_page_the_line_and_the_id` | RED: `got: []`, left 0 right 1. GREEN as the design's own drawn line, asserted character for character: ``docs/append-conditions.md:12 — cites `ES-99`, which SPECIFICATION.md does not define``, plus the ≤ 48-character location-prefix budget. |
| AC-004 | `a_citation_shaped_token_in_an_undeclared_family_is_a_problem`, `a_near_miss_inside_a_declared_family_is_a_problem` | RED: `got: []` on both, the second reporting `` `the ES- family` got: [] ``. GREEN as two wordings that share no phrase: the undeclared-family form lists the six families the specification declares (derived, so it cannot go stale), and the near-miss form says `looks like a clause citation and does not parse`. The second test asserts the *absence* of the `does not define` wording, so the two can never collapse into one message. |
| AC-005 | `a_constitution_rule_id_is_not_a_clause_citation`, `an_adr_reference_is_not_a_clause_citation`, `a_backlog_item_id_is_not_a_clause_citation`, `the_boundaries_are_both_load_bearing` | The first three were green from the start — they are the tests that fail a checker written *without* the boundaries, and the stub had no scanner to violate them. `the_boundaries_are_both_load_bearing` is what made them mean something: RED `the carve-outs must not have emptied the check`, left 0 right 1. It puts `RS-81-1`, `ADR-0001` and `HS-S0142` on one line beside a resolving `ES-40`, then beside a dangling `ES-99`. |
| AC-006 | `five_dangling_ids_on_one_page_report_five_problems_in_source_order`, `a_clause_id_inside_a_fence_is_still_checked`, `the_citation_check_consumes_the_page_text_already_read` | RED: left 0 right 5, and `got: []`. GREEN accumulating onto the module's existing `Vec<String>`, ascending by line, nothing containing `more`; and the fence case reports at line 4, inside a fenced `rust` block. The third test asserts exactly two `read_page(` occurrences in the production source — one definition, one call site. |
| AC-007 | `an_unreadable_specification_is_propagated_not_swallowed`, `an_unreadable_page_is_a_hard_error_naming_the_page` | Green from the wiring, and both were written before it. The first is the reason `clause_ids` is resolved *before* the tree walk: it asserts the chain contains `reading spec/SPECIFICATION.md` **and contains no `docs` at all**, so a page can never be blamed for the specification's condition. |
| AC-008 | `the_citation_check_documents_its_limits_before_its_guarantee` | Green from the doc edit, which landed with the wiring rather than after the checks — recorded rather than hidden. The test asserts `# What this does not verify` precedes `fn check_citations` in the production source *and* that all three new limit sentences are present; the second half is what a doc edit alone would not satisfy. |

Five tests were green from the start and are named above rather than left to be discovered.
Each exists to fail a *future* over-broad implementation — a check that reports resolving
citations, one written without the boundaries, one that blames a page for the specification —
and none is evidence on its own.

## Commits

| SHA | Subject |
| --- | ------- |
| `<checkpoint>` | feat(checked-documentation-surface): Narrative citation resolution |

The row records the checkpoint **as first written**. A commit cannot contain its own SHA, so
the reachable commit is reported in the slice digest and recorded on the item by
`redkiln record-links --sha` at the advance seam.

## Changes

| File | Shape of the change |
| ---- | ------------------- |
| `xtask/src/lint_narrative.rs:58-80` | Four bullets into the module's **existing** `# What this does not verify` section, replacing the placeholder milestone 2 left there (`An unresolvable clause id is still invisible here`). No second heading. |
| `xtask/src/lint_narrative.rs:404-420` | `read_page`, the page-read seam extracted from `collect` so RS-81-2's posture is assertable: a page that cannot be read ends the run naming the page, and the call site `?`-propagates it. |
| `xtask/src/lint_narrative.rs:968-1010` | `struct Clauses` — the resolved id set plus the families **derived** from it. The derivation's doc comment carries the reason a literal list loses, quoting `SECTIONS`' own. `declared()` renders the families into the message so an author who meant a constitution rule sees what the document actually declares. |
| `xtask/src/lint_narrative.rs:1012-1140` | `struct Cited` and `fn clause_citations(line: &str) -> Vec<Cited<'_>>` — the scan, split from the decision, with both boundaries written as explicit predicates and the token each protects (`ADR-0001`, `RS-81-1`) named in a comment beside it. No regex crate, no markdown parser. |
| `xtask/src/lint_narrative.rs:1142-1195` | `fn check_citations(page, clauses, found)` — the decision. Three problem forms, worded apart; a resolving citation is silent; the whole page is scanned, fences included. |
| `xtask/src/lint_narrative.rs:1196-1232` | `check_page` and `problems` thread the resolved set through by reference. |
| `xtask/src/lint_narrative.rs:1256-1310` | `run` becomes a two-liner over a new `fn check(root: &Path)`, so the hard-error postures are assertable against a root the caller chooses. `Clauses::new(clause_ids(root)?)` is the one call, first and before the tree walk. |
| `xtask/src/spec_trace.rs:1770-1806` | **Deletion only**: the foundation's `#[cfg_attr(not(test), expect(dead_code, reason = …))]`. This PR is `clause_ids`' first non-test caller, which is what makes the expectation unfulfilled. No other line in that file changed. |
| `.bklg/…/narrative-citation-resolution/_ledger.md` | Eight rows flipped to `satisfied: true` with `file:line` and test-id evidence. |

**Mount point.** `xtask/src/lint_narrative.rs` — the checker module's own composition root.
`xtask/src/main.rs` is untouched: the `Step`, the dispatch arm, the `print_help()` line, the
`lint_steps()` membership and the `affected::run` call were all mounted by
`narrative-checker-mounted-with-pinned-path`, and the citation problems appear under the one
banner that step already prints.

## Gates

| Command | Result |
| ------- | ------ |
| `cargo test --locked -p xtask --bin xtask lint_narrative::` (red) | `87 passed; 7 failed` |
| `cargo test --locked -p xtask --bin xtask lint_narrative::` (green) | `94 passed; 0 failed` |
| `cargo xtask narrative` (clean tree) | `  1 pages, all consistent` — exactly one line, one banner, no `skipped:` |
| `cargo xtask narrative` (fixture page temporarily carrying `ES-99`, `XX-7`, `ES-4O`; reverted before the commit) | three problem lines in source order, the count last, then `xtask failed: 3 problem(s) in docs` — read against `_design.md`'s worked failure report and matching it |
| `cargo clippy --locked -p xtask --all-targets --all-features` | clean under `-D warnings`, which is only possible once the foundation's expectation is gone |
| `cargo fmt -p xtask -- --check` | clean |
| `cargo xtask lint-constitution` | `27 atoms, all consistent` — copying `lint_constitution.rs`'s shapes did not edit it |
| `cargo xtask spec-trace` | `traceability: no problems found; §7.1–§7.2 matches the checker`, byte-identical to the pre-commit run (NF-005) |
| `cargo xtask affected --base main` | `affected gate passed` — `231 passed; 0 failed; 3 ignored` |
| `cargo xtask ci --fast` | `all required checks passed (--fast: 4 optional step(s) not run)` |

The recorded failing run, verbatim:

```text
  docs/append-conditions.md:7 — cites `ES-99`, which SPECIFICATION.md does not define
  docs/append-conditions.md:7 — cites `XX-7`, and SPECIFICATION.md has no `XX-` family; the families it declares are CF-, ES-, PS-, SY-, VT-, WF-
  docs/append-conditions.md:7 — `ES-4O` looks like a clause citation and does not parse; a citation the checker cannot read is one nothing checks

xtask failed: 3 problem(s) in docs
```

## Notes

**Three decisions the spec left to the implementer, and what each cost.**

*The spaced form.* EC-004 lists `ES -40` among the near-misses. The scanner accepts one space
between the two capitals and the hyphen, and **only** when digits follow — `ES --` is
punctuation, not a mis-typed citation, and reporting it would be the false positive AC-005
calls worse than a miss. `ES -40` is covered; a doubly-spaced or otherwise mangled form is not,
and that is the narrowness the predicate buys.

*The undeclared-family near-miss is silent, and is stated as a limit.* `ES-4O` inside the
declared `ES-` family is reported as unreadable; `QQ-4O` is not, because nothing distinguishes
it from prose that happens to carry two capitals and a hyphen. Reporting it would fire on
correct sentences, which is the direction this story is least allowed to fail in. It is written
into the module's limits (`:72-77`) rather than absorbed.

*Where `clause_ids` sits in `check`.* Before the tree walk, not after. AC-007 requires that an
unreadable specification is never reported with a page's name attached, and resolving first is
what makes the assertion `!chain.contains(TREE)` true rather than merely likely.

**One deviation from the spec's letter, in the test harness rather than in the check.** The
spec's AC-007 asks the unreadable-page test to assert "the problem list is *not* silently short
by one". A *present* page that cannot be read needs a temp directory, and `xtask` deliberately
carries no `tempfile` dev-dependency (DR-12, `xtask/Cargo.toml:16-21`), so the assertion is
made in two halves instead: the read seam `read_page` is driven against a page that is not
there and must name it, and the production source is asserted to call `read_page(root, &rel)?`
— the `let Ok(text) = … else { continue }` being the shape that would shorten the list. The
text-matching half is the in-house `check_harness` pattern (`lint_constitution.rs:424-425`),
not an invention.

**One test-helper change worth a reviewer's eye.** `walk`, the existing helper the fence and
marker tests use, now resolves a small hand-built six-family set rather than nothing. An
*empty* resolution is not neutral: with no declared families every citation classifies as an
undeclared family, which turned the fixture-page tests red for the right reason and showed the
helper had to carry a realistic set. `ES-40` is in it, which is what makes
`the_wrapped_fixture_page_fails_by_file_and_line` and
`the_same_fixture_page_without_its_wrapper_is_clean` continue to mean what they meant. No
assertion in any pre-existing test was weakened; two call sites gained an argument.

**What was not touched.** No page content, no `xtask/src/main.rs`, no new step or banner, no
`spec/SPECIFICATION.md` edit, no `xtask/Cargo.toml` entry, and no refactor of
`lint_constitution.rs`, `constitution.rs` or `spec_trace`'s `run`. The one edit to
`spec_trace.rs` is a deletion. No conformance rule is added and no port changes, so CF-29's
`CHANGELOG.md` obligation is not triggered; no `[FROZEN]` clause is edited, so no ADR is owed.

**What this story does not claim.** The mechanical half of DoD scenario 12 — every clause id a
page cites is one the specification declares — is now checked. Whether a page *defers* to a
clause rather than restating it is BR-09's reviewer half and HS-P0021's, it is stated as the
first of this check's limits, and nothing here is evidence of it.
