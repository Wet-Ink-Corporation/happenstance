---
item: HS-S0089
stage: implement
created: 2026-08-12
updated: 2026-08-12
---

# Acceptance ledger — A publish-time clause audit that reconciles and cannot be talked out of a frozen clause

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
  criterion: "GIVEN a maintainer who must not have to *remember* to audit the specification, WHEN they run `cargo xtask ci --fast` — or `cargo xtask affected --base main` on a commit that touches only `RUNBOOK.md` and no Rust package — THEN the clause audit runs anyway, prints how many clauses it read, and exits non-zero on any problem: it is in the **Mandatory** list with `probe: None`, it is in `affected.rs`'s unconditional file-reading block, it is named in `print_help`, and `main.rs`'s \"what the gate proves\" module doc says it is there."
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/main.rs (REQUIRED, the `main` dispatch, `print_help`, the module doc at :8-24); second mount xtask/src/affected.rs:116-125"
  verifying_test: "xtask/src/main.rs::tests::clause_audit_is_mandatory_and_probes_for_nothing; plus `cargo xtask affected --base HEAD~1` observed on a RUNBOOK.md-only commit"
- id: AC-002
  criterion: "GIVEN §1.3's census is the one count in the document a human produced by reading it, and the whole point of it is that the human's count and the machine's count are *independent* (`xtask/src/spec_trace.rs`:39-57), WHEN the audit runs against a document whose §1.3 sentence has been seeded to disagree with its own clauses, THEN the run fails naming both figures and `spec/SPECIFICATION.md`'s line — and it does so by **calling** `spec_trace::check_stated_census`, not by re-counting, so there is exactly one implementation of the reconciliation and two callers."
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/main.rs (REQUIRED entry, `clause-audit` dispatch arm); wires into xtask/src/spec_trace.rs::check_stated_census"
  verifying_test: "xtask/src/clause_audit.rs::tests::seeded_census_disagreement_fails; xtask/src/clause_audit.rs::tests::census_check_is_the_spec_trace_one"
- id: AC-003
  criterion: "GIVEN Persona 4 must learn what *provisional* means in the same sentence that uses it, and DT-5's resolution fixes that sentence's four numbers as \"taken from the AC-003 clause audit at the publish commit, not typed from memory\" (`_design.md`, `### DT-5`), WHEN the maintainer runs `cargo xtask clause-audit --write --date <YYYY-MM-DD>`, THEN `spec/audits/clause-maturity-<date>.md` is written carrying the date, the commit, the baseline's captured-at rev, one row per clause, and a reconciliation block in which 139 / 49 / 10 / 2 are four readable numbers — not a total a writer must derive by counting 200 rows — and the file is committed."
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/main.rs (`clause-audit --write --date` dispatch arm); artefact at spec/audits/clause-maturity-<date>.md"
  verifying_test: "xtask/src/clause_audit.rs::tests::report_states_all_four_counts_verbatim; xtask/src/clause_audit.rs::tests::report_lists_provisional_alongside_frozen; xtask/src/clause_audit.rs::tests::write_is_deterministic_for_a_given_date"
- id: AC-004
  criterion: "GIVEN the falsifier ledger was reconciled by hand once and agreed only \"at that commit\" (`RUNBOOK.md`:618-620) and has been wrong ever since by its own admission (`RUNBOOK.md`:622-635), WHEN the audit reads the repaired ledger, THEN it asserts a three-way equality — the number in the `### The 49 [PROVISIONAL] clauses` heading, the union of the `Clauses` column, and `spec-trace`'s parsed `[PROVISIONAL]` set — and a ledger short by one clause, a ledger carrying a clause that is no longer provisional, or a heading whose number disagrees with its own rows each fail, naming the direction (*in the ledger, not provisional* vs *provisional, not in the ledger*) and the IDs, because the remedy differs and only a reader can choose it."
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/main.rs (REQUIRED entry); reads RUNBOOK.md:580-635 and xtask/src/spec_trace.rs::parse_clauses"
  verifying_test: "xtask/src/clause_audit.rs::tests::ledger_short_by_one_fails_naming_the_missing_id; xtask/src/clause_audit.rs::tests::ledger_carrying_a_demoted_clause_fails_naming_the_direction; xtask/src/clause_audit.rs::tests::heading_number_disagreeing_with_its_rows_fails"
- id: AC-005
  criterion: "GIVEN a locator that sweeps a whole ledger row rather than its `Clauses` cell reports a set that is wrong in both directions, and the real table contains both en-dash ranges (`PS-4 – PS-6`) and `Falsified by` prose that mentions clause IDs which are *not* membership, WHEN the audit parses the ledger, THEN ranges are expanded to their members and IDs appearing outside the `Clauses` cell are not counted — so the check cannot pass by accident on a table it is misreading."
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/clause_audit.rs (the ledger parser), invoked from the REQUIRED step in xtask/src/main.rs"
  verifying_test: "xtask/src/clause_audit.rs::tests::expands_en_dash_ranges; xtask/src/clause_audit.rs::tests::falsified_by_prose_is_not_membership"
- id: AC-006
  criterion: "GIVEN this project promised to amend nothing `[FROZEN]` (DR-15) and nothing in the tree can currently observe that a frozen clause changed — an edited frozen clause makes the document *more* internally consistent, so `spec-trace` will not see it — WHEN the audit runs against `spec/audits/frozen-baseline.tsv`, captured from the merge-base with `main` and stamped with that rev and date, THEN a seeded word change inside a frozen clause fails naming the clause ID and its line, a purely cosmetic re-wrap of the same clause does not fail, and the fingerprint is value-stable across toolchains because the hash is a documented FNV-1a/64 written in this module with a pinned test vector rather than `DefaultHasher`."
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/main.rs (REQUIRED entry and the `--rebaseline --from <rev>` dispatch arm); baseline artefact at spec/audits/frozen-baseline.tsv"
  verifying_test: "xtask/src/clause_audit.rs::tests::seeded_frozen_word_change_fails; xtask/src/clause_audit.rs::tests::rewrapping_a_frozen_clause_does_not_fire; xtask/src/clause_audit.rs::tests::fnv1a64_pinned_vector; xtask/src/clause_audit.rs::tests::rebaseline_refuses_when_the_rev_and_the_tree_disagree"
- id: AC-007
  criterion: "GIVEN AC-015 is about the frozen *set*, not only about frozen text, WHEN a clause the baseline lists has left the frozen set (demoted to `[PROVISIONAL]`), or a clause that is `[FROZEN]` in the tree is absent from the baseline, THEN each fails by name — a new frozen clause is a specification change belonging to another project, and a demotion is exactly the laundering DR-15 forbids."
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/main.rs (REQUIRED entry); compares spec/audits/frozen-baseline.tsv against spec/SPECIFICATION.md"
  verifying_test: "xtask/src/clause_audit.rs::tests::a_demoted_frozen_clause_fails; xtask/src/clause_audit.rs::tests::a_new_frozen_clause_absent_from_the_baseline_fails"
- id: AC-008
  criterion: "GIVEN ADR-0010's discipline that a skip is reported and never silent (`.kb/decisions/0010-the-suite-must-prove-itself.md`) and that the single most likely wrong implementation of this story is a ledger locator that finds zero rows and reports two equal empty sets, WHEN the ledger heading is renamed or absent, the ledger table is empty, the baseline is missing / unparseable / has an empty header, a clause carries no maturity marker this table knows, or the report target cannot be written, THEN the run fails — never passes — with the path, the line and what the reader is expected to do, and never with a comparison of one empty set against another."
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/main.rs (REQUIRED entry, `probe: None` — the step has no tool to probe for, so it can never degrade to a skip)"
  verifying_test: "xtask/src/clause_audit.rs::tests::renamed_ledger_heading_fails_not_passes; xtask/src/clause_audit.rs::tests::empty_ledger_table_is_never_an_equal_empty_set; xtask/src/clause_audit.rs::tests::absent_baseline_fails_with_the_path; xtask/src/clause_audit.rs::tests::a_clause_with_no_marker_fails"
- id: AC-009
  criterion: "GIVEN a second parser would double the surface on which the document could be misread and would destroy the independence §1.3 exists to prove, WHEN this story lands, THEN `xtask/src/spec_trace.rs`'s diff is visibility and doc comments only — `parse_clauses`, `Clause`, `Census`, `check_stated_census`, `SPEC` and `workspace_root` widened to `pub(crate)` — `cargo xtask spec-trace` and `cargo xtask spec-trace --write` produce identical output before and after, and §7.1/§7.2's generated regions are byte-identical."
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/spec_trace.rs (visibility widening only); guarded by the `specification traceability` step at xtask/src/main.rs:303-328"
  verifying_test: "`cargo xtask spec-trace` output diffed across the change and `cargo xtask spec-trace --write` producing no change; `cargo xtask ci --fast` green"
```
