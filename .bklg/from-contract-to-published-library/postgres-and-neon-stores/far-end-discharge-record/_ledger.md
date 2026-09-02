---
item: "HS-S0073"
stage: implement
created: "2026-08-12"
updated: "2026-08-12"
---

# Acceptance ledger — The far-end discharge recorded for the publication audit

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
  criterion: "GIVEN the maintainer at publish time is drafting the compliance claim and knows only that this project happened, WHEN they look for its far-end result, THEN they find one document at `references/far-end-discharge.md` carrying exactly nine rows — ES-10, ES-11, ES-12, ES-41, ES-42, VT-21, VT-22, VT-23, VT-24 — each a clause-times-store-times-outcome triple, and they reach it without prior knowledge because both entry points an auditor already uses link to it: `RUNBOOK.md` phase 10's session log and `references/adapter-shapes.md` §5. A clause with no row, a row with no clause, and a row missing any of the three terms are each a failure."
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/main.rs — the `REQUIRED` step array at :105 (Root C), new step `the far-end discharge record is consistent`, listed in `lint_steps()` at :792-808"
  verifying_test: "xtask/src/lints.rs::tests::every_owned_clause_has_exactly_one_row; xtask/src/lints.rs::tests::a_row_missing_a_term_is_rejected"

- id: AC-002
  criterion: "GIVEN the maintainer must name the implementation each clause was checked against and cannot re-open a six-month-old fixture to find out what was behind it, WHEN they read any row, THEN the store term names the fixture AND its real backing — a Neon branch reached over the `/sql` endpoint, or a pinned Postgres image under `testcontainers` — plus the CI job name and run identifier the evidence came from; and the three results this project already decided elsewhere (ADR-0024's mechanism and cost, the Neon `conflicting_position` verdict including whether ES-25's permission to return `None` was exercised, and each fixture's declared capabilities and ceilings) are reproduced by citation to the accepted atom or the story ledger, never restated in this record's own words."
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/main.rs — the `REQUIRED` step array at :105 (Root C), new step `the far-end discharge record is consistent`, implemented in xtask/src/lints.rs"
  verifying_test: "xtask/src/lints.rs::tests::a_row_whose_store_term_names_only_a_type_is_rejected; xtask/src/lints.rs::tests::a_row_without_a_job_and_run_is_rejected"

- id: AC-003
  criterion: "GIVEN a clause whose rule was skipped because a fixture declined a capability has NOT been discharged however green the run was, WHEN the maintainer reads an outcome, THEN it is one of exactly five values — `passed`, `failed`, `skipped — <reason>`, `amended by <record>`, `not exercised` — and a `skipped` row carries the fixture's own `Capability::declined` reason verbatim as printed by the live job's `--show-output`, so that a decline, a pass and a clause nobody ran are three distinguishable facts rather than one."
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/main.rs — the `REQUIRED` step array at :105 (Root C), new step `the far-end discharge record is consistent`, implemented in xtask/src/lints.rs"
  verifying_test: "xtask/src/lints.rs::tests::an_outcome_outside_the_five_is_rejected; xtask/src/lints.rs::tests::a_skip_with_an_empty_reason_is_rejected"

- id: AC-004
  criterion: "GIVEN promoting ES-11 or ES-12 out of `[PROVISIONAL]` is one word per row that `spec-trace` would happily stay green through, and GIVEN that decision belongs to the maintainer at publish time and not to this record, WHEN this story merges, THEN no maturity marker has moved and no clause text has changed — ES-10 is still `[FROZEN]`, the other eight still `[PROVISIONAL]` — each row states the marker it read out of the tree, and the record is henceforth red if any row's stated marker disagrees with the clause's marker in `spec/SPECIFICATION.md`. Any diff to that file is either a `cargo xtask spec-trace --write` regeneration of the §7.1–§7.2 region or a `file:line` citation repair, both recorded; a hand edit inside the generated markers is a failure."
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/main.rs — the `REQUIRED` step array at :105 (Root C): the new discharge-record step beside the existing `cargo xtask spec-trace` step"
  verifying_test: "xtask/src/lints.rs::tests::a_row_whose_marker_disagrees_with_the_clause_is_rejected; cargo xtask spec-trace (xtask/src/spec_trace.rs:200-201 generated-region equality)"

- id: AC-005
  criterion: "GIVEN nothing in this tree can currently fail because a written record is wrong or absent, and GIVEN `references/` reaches no package, WHEN the record rots — a row deleted, an outcome vocabulary drifted, a clause renamed out from under it — THEN the merge gate goes red rather than the audit discovering it months later; and the question of whether phase 10 also earns an `xtask/src/proof.rs` `ARTEFACTS` row is answered in the record itself, taken if and only if the live-suite gating leaves the target listable and runnable-to-a-pass with no Docker and no network, and declined in writing with the gating mechanism named if not."
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/main.rs — the `REQUIRED` step array at :105 (Root C), the subcommand arm at :682-687, and `lint_steps()` at :799-808 so `cargo xtask lints` (half of .redkiln/config.yaml's `reachability_static`) runs it at story grain"
  verifying_test: "xtask/src/lints.rs::tests::a_rotted_record_is_rejected; xtask/src/lints.rs::tests::the_artefacts_verdict_is_recorded"

- id: AC-006
  criterion: "GIVEN `references/adapter-shapes.md` §5 is the table of what a skeleton does not prove and still writes `Phase 10` in the still-empty column for position allocation and transport, WHEN the maintainer reads it after this story, THEN both cells state what the two real adapters told the type checker AND the server — including the two prior assumptions this project contradicted, the Neon CTE that keeps `conflicting_position` against the ledger's standing assumption and the Postgres frontier's structural cost the throughput experiment could not price — and they were edited **in place**, with `references/adapter-shapes.md:297` still at `:297` because `spec/SPECIFICATION.md:8095` cites it and `check_citations` requires the cited subject within twelve lines."
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/main.rs — the `REQUIRED` step array at :105 (Root C): the new discharge-record step plus the existing `cargo xtask spec-trace` citation check"
  verifying_test: "xtask/src/lints.rs::tests::a_phase_10_cell_left_deferring_is_rejected; cargo xtask spec-trace (xtask/src/spec_trace.rs:291-372, :374-377)"

- id: AC-007
  criterion: "GIVEN `RUNBOOK.md` phase 10's **Session log** is empty and its exit criteria are unticked, WHEN the maintainer opens the runbook at the phase this project executed, THEN the log is written against phase 10's own proof artefact — the concurrency family green under a multi-thread runtime on a store that does not serialise its writers, a committed number for the visibility strategy including the held-transaction behaviour, and Neon's capability skip list — it links `references/far-end-discharge.md` rather than duplicating it, and an exit-criteria box that was not met says so rather than being ticked."
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/main.rs — the `REQUIRED` step array at :105 (Root C), new step `the far-end discharge record is consistent`, implemented in xtask/src/lints.rs"
  verifying_test: "xtask/src/lints.rs::tests::an_empty_phase_10_session_log_is_rejected"

- id: AC-008
  criterion: "GIVEN a record reporting three discharges invites `far ends filled` to be read as `portfolio complete`, WHEN the maintainer reads it, THEN it names its own absences in the same document: ES-35 (durability) and ES-40 (completeness) as far ends this project did not touch with their owning stories named; `RUNBOOK.md:606`'s residual-exposure groups as knowingly short by ES-41, ES-42, CF-39 and CF-40 and still carrying ES-10, cited by line as HS-P0016's five edits and not performed here; and the marker promotions this evidence would support as HS-P0016's decision rather than this record's claim."
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/main.rs — the `REQUIRED` step array at :105 (Root C), new step `the far-end discharge record is consistent`, implemented in xtask/src/lints.rs"
  verifying_test: "xtask/src/lints.rs::tests::a_record_omitting_the_untouched_far_ends_is_rejected"

- id: AC-009
  criterion: "GIVEN two of this project's gates need a live server and DR-9 forbids either from entering the default path, WHEN any contributor runs the gate on a clean checkout with no Docker, no network and no credentials, THEN `cargo xtask ci --fast` is green **including** the new discharge-record step — which is a file read and nothing more — no CI job is added, no existing step's `name` string changes, the wasm32 steps still resolve by name, and `lint_steps()`'s doc comment is corrected to match the number of checks it now selects."
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/main.rs — the `REQUIRED` step array at :105 (Root C) and `lint_steps()`/`wasm_steps()` name selection at :771-816"
  verifying_test: "cargo xtask ci --fast (offline, clean checkout — .redkiln/config.yaml `integration_scoped`); cargo xtask affected --base main; cargo test -p xtask; cargo xtask wasm"
```
