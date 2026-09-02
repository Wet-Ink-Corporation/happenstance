---
item: "HS-S0143"
stage: implement
created: "2026-08-17T13:16:05.099Z"
updated: "2026-08-17T13:16:05.099Z"
---

# Implementation Report — The frozen documentation MUSTs are enumerated by clause id in one place

## TDD Evidence

The re-derivation came first, before a line of check code, exactly as the spec's implementation
notes ask — it is evidence-gathering rather than `#[test]` code, and it lives at
`.bklg/…/frozen-documentation-must-pin/_rederivation.md`. The one piece of *code* written ahead
of the tests is the derivation instrument itself (`declared_clause`, `documentation_candidates`,
`normalised`, `names_clause`), because the derivation could not be done without it; a temporary
`panic!`-ing test dumped the candidate set, the set was classified by hand against the document,
the `const` was transcribed from that record, and the temporary test was deleted.

Then the tests, against production halves that were the **named wrong implementations**:

* `check_pin_resolution`, `check_anchor` and `check_pin_census` were each a no-op — a pin that
  compiles, is called from `run`, and reports nothing, which is the decorative-check defect this
  project exists to refuse.
* `guard_pin` was written for real, because a vacuity guard that starts as a stub has nothing to
  go red about; its three refusals are asserted directly.

Red run: `cargo test --locked -p xtask --bin xtask lint_narrative::` → **102 passed; 9 failed**.
Green run, after the three check bodies: **111 passed; 0 failed**.

| AC | Test | Red → Green |
| -- | ---- | ----------- |
| AC-001 | `every_pin_entry_is_disposed_with_a_site_and_an_anchor`, `the_pin_holds_no_written_count` | The first is a property of the `const` and was green once the `const` was transcribed — it exists to fail a *future* edit that adds an entry with no site, an anchor equal to its id, or a duplicate. The second went RED twice and both times on real prose: first on `` `xtask/src/spec_trace.rs:1975-1978` `` (the naive digit check saw the `8` inside `1978`), then on the comment's own quotation of the defect it describes — the draft literally said `"nine, per RUNBOOK" above an array holding eight`. The test was **strengthened** rather than weakened: it now asserts no standalone numeral of the array's length or of either half of its disposition, plus their number words. The comment lost the quotation. |
| AC-002 | `a_pinned_id_absent_from_the_specification_is_a_problem`, `the_specification_is_resolved_once_per_run` | RED: `got: []`, left 0 right 1. GREEN on membership in the set `clause_ids` returned, with the problem naming the id and the checker's own path and asserting it names no page. The second test is milestone 3's and was extended to hold `check_pin` to `&Clauses` rather than a `&Path`. |
| AC-003 | `a_missing_anchor_and_a_missing_site_are_two_different_problems`, `the_whole_pin_holds_against_the_real_tree` | RED: `got: []`, left 0 right 2. GREEN as two arms with two sentences — *the path moved* and *the discharging text moved* — and the test asserts neither message is reachable from the other's condition. The real-tree test was green from the stub and is named here rather than hidden: with no-op checks it could only pass, and it becomes the test that fails on the day a `happenstance-core` doc comment un-discharges a `[FROZEN]` clause. |
| AC-004 | `the_anchor_match_survives_a_reflowed_doc_comment`, `a_short_clause_id_does_not_match_a_longer_one` | The reflow test was green from the stub for the same structural reason and is the one this story is least allowed to lose: a false positive here teaches contributors to delete pin entries. `a_short_clause_id_does_not_match_a_longer_one` went RED (left 0 right 2) and is green through set membership plus `names_clause`, copied from `xtask/src/lints.rs:492-500`. |
| AC-005 | `an_unclassified_candidate_says_the_document_grew`, `an_entry_that_is_no_longer_a_candidate_says_the_enumeration_is_stale` | RED: `got: []` on both. GREEN as two sentences that share no phrase, the second test asserting the *absence* of the first's wording (RS-81-5). |
| AC-006 | `every_pin_problem_is_reported_not_just_the_first`, `the_pin_never_truncates_its_problem_list` | RED: left 0 right 3, and left 0 right 20. GREEN accumulating onto the module's existing `Vec<String>`, in pin order then census order, nothing containing `more`. |
| AC-007 | `every_pin_problem_line_begins_with_its_artifact` | RED twice, and the second time usefully: first `got: []`, then `left: 4 right: 5` — only four of the five conditions were reachable from the pin I had written, because an unresolvable id can no longer be a candidate either. The fix was to drive the fifth condition properly rather than to lower the number. |
| AC-008 | `the_module_states_the_pins_limits_before_the_pin`, `no_pin_message_claims_a_discharge_is_correct` | Both green from the doc edit, which landed with the wiring; the ordering half is a real assertion against the production source and the second drives every message the pin can emit through seven claiming tokens. `nothing_in_the_module_claims_a_page_teaches` — milestone 2's — caught two drafting slips in this story's own prose (`verified on main`, twice) and is why they are not in the diff. |

Six tests were green from the start. Each rejects a *future* over-broad or lost implementation
— a jumpy anchor match, a pin that stops holding over the real tree, an entry with no reason —
and none is evidence on its own.

## Commits

| SHA | Subject |
| --- | ------- |
| `<checkpoint>` | feat(checked-documentation-surface): Frozen documentation MUST pin |

The row records the checkpoint **as first written**. A commit cannot contain its own SHA, so
the reachable commit is reported in the slice digest and recorded on the item by
`redkiln record-links --sha` at the advance seam.

## Changes

| File | Shape of the change |
| ---- | ------------------- |
| `xtask/src/lint_narrative.rs:81-99` | Three limits into the module's **existing** `# What this does not verify` section, which stays the first heading. |
| `xtask/src/lint_narrative.rs:250-257` | `SPECIFICATION`, the path the candidate scan reads. A second spelling of a constant `spec_trace` holds privately, and the doc comment says why it is a second spelling rather than a widened visibility. |
| `xtask/src/lint_narrative.rs:1246-1266` | `DOCUMENTATION_OBLIGATIONS` — six phrasings, with its own limit stated beside it (EC-010). |
| `xtask/src/lint_narrative.rs:1268-1483` | `Disposition`, `DocumentationMust`, and `FROZEN_DOC_MUSTS`: twenty-one entries, the decision travelling with the row in the shape `RULE_FILES` and `UNCLAIMED_PENDING_ADR` already use. The derivation rule is in the comment where the next reader meets it; no count is written anywhere. |
| `xtask/src/lint_narrative.rs:1486-1600` | `normalised` (reflow-insensitive matching, applied to both sides), `names_clause` (whole-identifier, copied from `lints.rs`), `declared_clause` and `documentation_candidates` (the derived scan). |
| `xtask/src/lint_narrative.rs:1603-1735` | `guard_pin` (three refusals, before any check runs) and the three assertions, split so each decision is assertable over a `&str` and only `check_pin` does I/O. |
| `xtask/src/lint_narrative.rs:1795-1805` | The wiring: `guard_pin` beside the tree's own vacuity guard, one specification read, and `check_pin` appending to the checker's existing problem list. |
| `.bklg/…/frozen-documentation-must-pin/_rederivation.md` | **New.** The written re-derivation AC-001 requires, closing the arithmetic row by row. |
| `.bklg/…/frozen-documentation-must-pin/_ledger.md` | Eight rows flipped to `satisfied: true` with `file:line` and test-id evidence. |

**Mount point.** `xtask/src/lint_narrative.rs` — the checker's own `run`, already mounted as the
`REQUIRED`, `probe: None` step *every narrative page is checked*. **No mount site was added**:
`xtask/src/main.rs` is unchanged in the diff, and `xtask/src/spec_trace.rs` needed no edit
either, because `narrative-citation-resolution` had already deleted the foundation's
`#[expect(dead_code, …)]` (EC-009 satisfied by the slice-mate, one commit earlier).

## Gates

| Command | Result |
| ------- | ------ |
| `cargo test --locked -p xtask --bin xtask lint_narrative::` (red) | `102 passed; 9 failed` |
| `cargo test --locked -p xtask --bin xtask lint_narrative::` (green) | `111 passed; 0 failed` |
| `cargo test --locked -p xtask --bin xtask` | `162 passed; 0 failed` |
| `cargo test --locked -p xtask --doc` | `63 passed; 0 failed` |
| `cargo xtask narrative` (clean) | `  1 pages, all consistent` — one line, one banner |
| `cargo xtask narrative` (one anchor and one id deliberately broken in the `const`, reverted before the commit) | both problems, location-first, then `xtask failed: 2 problem(s) in docs` |
| `cargo clippy --locked -p xtask --all-targets --all-features` | clean under `-D warnings` |
| `cargo fmt -p xtask -- --check` | clean |
| `cargo xtask lint-constitution` | `27 atoms, all consistent` — `names_rule` was copied, not refactored into a shared helper |
| `cargo xtask spec-trace` | `traceability: no problems found; §7.1–§7.2 matches the checker`, unchanged from before the commit (NF-008, project AC-008) |
| `cargo xtask affected --base main` | `affected gate passed` |
| `cargo xtask ci --fast` | `all required checks passed (--fast: 4 optional step(s) not run)` |

The recorded failing run, verbatim:

```text
  xtask/src/lint_narrative.rs — the pin names `ES-99`, which SPECIFICATION.md does not declare
  crates/happenstance-core/src/store.rs — the discharge of `ES-23` no longer carries `# Cancellation TEMPORARY`; the discharging text moved

xtask failed: 2 problem(s) in docs
```

`git status` was clean after that failing run: the step reads five artifacts and writes none.

## Notes

**The re-derivation disagreed with the spec's expected set, and the re-derivation won.** The
spec states, and explicitly holds open, an expected pinned set of ES-19, ES-23, ES-24, VT-3,
VT-15, VT-17, ES-17 and ES-40. What the document actually supports is a *different* eight:

* Kept: ES-19, ES-23, ES-24, VT-15, VT-17.
* Dropped: **VT-3** (its correction is stated under a heading reading "Prose, not a clause"; it
  words no documentation MUST and is therefore not a candidate at all), **ES-17**
  (`[PROVISIONAL]`, and likewise not a candidate), **ES-40** (`[PROVISIONAL]` — its discharge at
  `append.rs:29` is present and would anchor cleanly, so it becomes a pinned entry the day the
  clause freezes).
* Added: **VT-13**, **VT-32**, **VT-33** — each `[FROZEN]`, each on the contract's own
  documentation, each discharged today, and none of them written down anywhere before this.

**Three `[FROZEN]` documentation MUSTs on the contract's own documentation are not discharged at
all: ES-26, PS-31 and PS-36.** ES-26 requires the inclusive/exclusive asymmetry to be documented
*as deliberate*; both halves are documented separately and nothing says the asymmetry is
deliberate. PS-31 and PS-36 require `projection.rs` to say two things it does not say. They are
**excluded with that reason** rather than pinned, because pinning them would land the gate red
and the only repair is a `happenstance-core` doc-comment edit — HS-P0023's boundary, governed by
`.kb/governance/rewrite-the-referent-never-the-reasoning.md`, and the spec's own implementation
note says exactly this. This is the concrete follow-up the story hands forward.

**Two deviations, both recorded rather than absorbed.**

*The specification's bytes are read twice per gate run.* NF-001 asks for one read and one parse.
The **parse** is one — `clause_ids` is called exactly once and the set is shared. The bytes are
read twice: once inside `clause_ids`, and once by `check` for the candidate scan, which needs
the text and not the ids. Narrowing that to one read would need a second narrow accessor in
`xtask/src/spec_trace.rs`, which is outside this story's one permitted edit to that file
(EC-009). The cost is one 567 KB `read_to_string`, inside the noise floor of the step, and the
follow-up is a `spec_trace` accessor whenever a third consumer wants the text.

*The derived scan needs to attribute a candidate phrase to a clause, and `parse_clauses` is
private.* `declared_clause` decides declaration *position* — first citation-shaped token, at the
start of the line once heading and bold markers are stripped, not closed immediately by `**` —
while **resolution stays `clause_ids`' set membership**, so no fourth family list exists. The
shape is `spec_trace::clause_id`'s, copied rather than shared (Note 8), and it is held honest by
measurement rather than by argument: over the real document it attributes to exactly the 200
clauses the parser resolves (`declared=200 ids=200 missing=[] extra=[]`).

That measurement is a **standing assertion**, not a reading taken once.
`the_declaration_scan_attributes_every_clause_the_parser_declares` re-derives it against the real
checkout on every `cargo test -p xtask`, and the failure prints `missing`/`extra` rather than two
200-id sets. It is what `the_whole_pin_holds_against_the_real_tree` cannot see: that test fails
only when a divergence moves a **candidate**, so an attribution that slides an obligation onto a
neighbour while leaving the candidate set unchanged passes it. Falsified by perturbing
`declared_clause` to skip the `VT` family — the assertion fails naming all thirty-four ids, and
the perturbation was reverted.

**One behaviour decided at implementation and worth a reviewer's eye.** An entry whose clause id
the document no longer declares is reported **once**, by assertion 1, and is skipped by the
census. Reporting it twice would give one defect two messages, the second of which blames the
enumeration for a clause that was renumbered underneath it — the opposite of what RS-81-5 asks
the two census sentences to distinguish.

**What was not touched.** No `spec/SPECIFICATION.md` edit, no `happenstance-core` doc comment,
no page, no `xtask/src/main.rs`, no `xtask/Cargo.toml` entry, and no refactor of
`lint_constitution.rs`, `constitution.rs` or `lints.rs` to share code. No conformance rule and
no port change, so CF-29's `CHANGELOG.md` obligation is not triggered; no `[FROZEN]` clause is
edited, amended or restated, so no ADR is owed.
