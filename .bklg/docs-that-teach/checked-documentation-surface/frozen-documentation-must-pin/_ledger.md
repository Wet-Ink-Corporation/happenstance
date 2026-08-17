---
item: HS-S0143
stage: implement
created: "2026-08-17T13:17:03.641Z"
updated: "2026-08-17T13:17:03.641Z"
---

# Acceptance ledger — The frozen documentation MUSTs are enumerated by clause id in one place

The machine-checkable Definition-of-Done ledger for this story (RFC §6.5). One row per spec
`AC-###`. Planning authors every row with `satisfied: false`; the implementer may only flip a row to
`satisfied: true` and MUST cite real evidence (a `file:line` and/or the verifying test id) — never
edit, remove, or re-word a criterion, and never flip a satisfied row back. `redkiln verify --grain
story` reads the fenced block below and blocks `implement → report` unless every spec AC is present,
`satisfied: true`, and carries non-placeholder evidence (and no AC that was satisfied on the base
branch has regressed). Scope changes are a human decision recorded through `redkiln advance`, not a
quiet ledger edit.

Four notes for the implementer, all drawn from `spec.md` rather than added here:

- **`mount_point` is identical on every row and that is the point.** This story adds *no* mount
  site: the pin is a check inside the `run` that `narrative-checker-mounted-with-pinned-path`
  already mounted as the `REQUIRED`, `probe: None` step *every narrative page is checked*
  (`xtask/src/main.rs`, five sites). A pin that compiles and is called by nothing but a test is the
  decorative-check defect this project exists to refuse, so evidence for every row must be
  reachable from `cargo xtask narrative` and from `cargo xtask ci --fast`, not only from
  `cargo test -p xtask`.
- **The `verifying_test` values are paths those tests must land at, not paths that already
  resolve.** They live in `xtask/src/lint_narrative.rs`'s `#[cfg(test)] mod tests`, authored in the
  house shape at `xtask/src/lint_constitution.rs:827-841` including the scoped
  `#![allow(clippy::unwrap_used, reason = "test code, per the house style")]`.
- **AC-001 is the only row whose evidence is partly non-test.** The written re-derivation that
  closes the nine-versus-eight arithmetic is evidence-gathering, deliberately not `#[test]` code
  (`_decomposition.md:617-643`); cite the record's path in this story's own directory *alongside*
  the two tests, not instead of them.
- **AC-006, AC-007 and AC-008 are the composition criteria** taken from the signed-off `_design.md`.
  A pin that satisfies AC-001 through AC-005 while emitting `Err(anyhow!("pin failed"))`, or
  truncating its problem list, or putting its limits section last, satisfies every functional
  assertion and fails this story.

```yaml
- id: AC-001
  criterion: "**GIVEN** a reviewer who inherits two disagreeing statements of the frozen documentation MUST set — `RUNBOOK.md:3830-3845` records \"nine\", the evidence table behind it (`references/evaluation/phase-4-5-reconciliation.md:119-142`) lists eight rows of which two are comment repairs rather than clause discharges, and ES-19 was recorded by the pass rather than re-verified — **WHEN** they open this PR to decide whether the set is now trustworthy, **THEN** they meet **exactly one** enumerated `const` in `xtask/src/lint_narrative.rs` in which *every* candidate the derivation rule produces is disposed of: **pinned** with its clause id, its discharge-site path and its verbatim anchor, or **excluded** with a one-line reason. The derivation rule — (a) the clause is `[FROZEN]` and (b) its obligation falls on **the contract's own documentation**, not on an adapter's or a fixture's — is stated in the comment where the next reader meets it, not in a paragraph elsewhere. **AND** a written re-derivation lives in this story's own artefacts and closes the nine-versus-eight arithmetic by name, including the answer that the count *was* nine and is now eight if that is the answer. No count is written into any comment."
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/lint_narrative.rs — the pin `const` and its checks inside the checker's existing `run`, reached through the `REQUIRED` `probe: None` step `every narrative page is checked` mounted at `xtask/src/main.rs:105`"
  verifying_test: "xtask/src/lint_narrative.rs::tests::every_pin_entry_is_disposed_with_a_site_and_an_anchor and ::the_pin_holds_no_written_count (cargo test -p xtask), plus the re-derivation record in this story's directory"
- id: AC-002
  criterion: "**GIVEN** a contributor who renumbers, deletes or typos a clause id the pin names, **WHEN** they run the gate, **THEN** the run fails with a problem naming **the id and the pin** — never a narrative page — because every pinned id is resolved through `crate::spec_trace::clause_ids(root)`, the resolver `spec-trace-clause-id-accessor` landed beside `all_rules` (`xtask/src/spec_trace.rs:1746`). The checker calls it **once per run** and passes the resulting set to both the citation check and the pin check: no second parser, no regex, no literal prefix list — a fourth list of the six clause families is the one that can half-land, which is what `SECTIONS`' own doc comment says (`:107-120`). Existence, not eligibility: a `[DEFERRED]` id resolves, because the pin's filter is the human classification in AC-001 and not a maturity test inside the resolver."
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/lint_narrative.rs — the pin `const` and its checks inside the checker's existing `run`, reached through the `REQUIRED` `probe: None` step `every narrative page is checked` mounted at `xtask/src/main.rs:105`"
  verifying_test: "xtask/src/lint_narrative.rs::tests::a_pinned_id_absent_from_the_specification_is_a_problem and ::the_specification_is_resolved_once_per_run (cargo test -p xtask)"
- id: AC-003
  criterion: "**GIVEN** an adapter author's discharge site that has been moved, renamed or rewritten out from under the pin — `crates/happenstance-core/src/store.rs`'s `# Cancellation` section deleted, or `tag.rs` split in two — **WHEN** the gate runs, **THEN** two *distinct* problems are possible and the message says which happened: **the path moved** (the discharge-site file could not be read, reported with `read`'s `reading {rel}` context, `xtask/src/spec_trace.rs:2307-2309`) or **the discharging text moved** (the file is there and the verbatim anchor is not). A contributor who reads \"the anchor is gone\" and goes looking for a missing file has been sent to the wrong place, and the anchor is the load-bearing sentence of the discharge rather than a nearby convenience — `# Cancellation` (`store.rs:146`), `at-most-once under verbatim reissue` (`store.rs:169`), `# Equality is byte equality, and nothing is normalised` (`tag.rs:29`)."
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/lint_narrative.rs — the pin `const` and its checks inside the checker's existing `run`, reached through the `REQUIRED` `probe: None` step `every narrative page is checked` mounted at `xtask/src/main.rs:105`"
  verifying_test: "xtask/src/lint_narrative.rs::tests::a_missing_anchor_and_a_missing_site_are_two_different_problems and ::the_whole_pin_holds_against_the_real_tree (cargo test -p xtask)"
- id: AC-004
  criterion: "**GIVEN** a contributor who re-wraps a paragraph in `crates/happenstance-core/src/store.rs` — a formatting edit that changes no meaning — **WHEN** they run the gate, **THEN** it stays green, because the anchor match strips doc-comment prefixes and collapses whitespace before comparing. This is the criterion that protects the pin from its own users: the remedy a contributor reaches for when a check fires on an innocuous edit is to edit the check, and the check *is* the pin, so a jumpy anchor teaches people to delete entries. **AND** every place this story matches a clause id matches a **whole identifier**: `ES-1` is a substring of `ES-17`, `VT-3` of `VT-33`, `ES-4` of `ES-40`, and the shorter-name-swallowed-by-the-longer defect shipped once already in this repository for conformance rule names (`standards/rust/81-checks-that-cannot-be-types.md:209-217`)."
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/lint_narrative.rs — the pin `const` and its checks inside the checker's existing `run`, reached through the `REQUIRED` `probe: None` step `every narrative page is checked` mounted at `xtask/src/main.rs:105`"
  verifying_test: "xtask/src/lint_narrative.rs::tests::the_anchor_match_survives_a_reflowed_doc_comment and ::a_short_clause_id_does_not_match_a_longer_one (cargo test -p xtask)"
- id: AC-005
  criterion: "**GIVEN** the unmerged sibling `initiative/from-contract-to-published-library`, which diverges from `spec/SPECIFICATION.md` by 521 lines and may **add** a documentation MUST, **WHEN** that branch is merged forward and the gate runs on the merge commit, **THEN** the new clause surfaces as **an unclassified candidate** — a gate failure a human disposes of in the pin — rather than waiting for the HS-P0025 closeout re-check `project.md:274-281` had to settle for. The comparison is derived-versus-hand-written: the derived side is a candidate scan of `spec/SPECIFICATION.md` over an enumerated phrase `const`, the hand side is AC-001's classified enumeration, and the failure **names which side moved** — *a candidate nobody classified* (the document grew) is a different sentence from *a pinned or excluded entry that is no longer a candidate* (the enumeration is stale), and both name the ids. Never \"the counts disagree\"."
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/lint_narrative.rs — the pin `const` and its checks inside the checker's existing `run`, reached through the `REQUIRED` `probe: None` step `every narrative page is checked` mounted at `xtask/src/main.rs:105`"
  verifying_test: "xtask/src/lint_narrative.rs::tests::an_unclassified_candidate_says_the_document_grew and ::an_entry_that_is_no_longer_a_candidate_says_the_enumeration_is_stale (cargo test -p xtask)"
- id: AC-006
  criterion: "**GIVEN** a contributor who has broken three pinned entries in one edit, **WHEN** they run `cargo xtask narrative` or the same step inside `cargo xtask ci`, **THEN** they see **all three** problems in one run, in source order, with the count last — never the first one and a stop, because \"a check that stops at the first problem turns one review cycle into six\" (`_decomposition.md:218-219`) — and never a truncated list: no \"… and N more\" anywhere, which is `_design.md` anti-pattern 8. The pin reports **in place**, through the existing step's own `Vec<String>`, its two-space-indented stderr lines and its `bail!(\"{} problem(s) in {TREE}\", problems.len())` — the shape at `xtask/src/lint_constitution.rs:190-199`. **No new step, banner, subcommand, `print_help` line, `REQUIRED` entry or `lint_steps` member is added**: a second banner would split one surface in two and send the reader to a second place for the same fact. On success the checker still prints **exactly one line**, unchanged, because a green check that says ten lines trains people to skip it (`_design.md`, `## Transience policy`)."
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/lint_narrative.rs — the pin `const` and its checks inside the checker's existing `run`, reached through the `REQUIRED` `probe: None` step `every narrative page is checked` mounted at `xtask/src/main.rs:105`"
  verifying_test: "xtask/src/lint_narrative.rs::tests::every_pin_problem_is_reported_not_just_the_first and ::the_pin_never_truncates_its_problem_list (cargo test -p xtask), plus recorded gate output from `cargo xtask narrative`, `cargo xtask ci --fast` and `cargo xtask affected --base main`"
- id: AC-007
  criterion: "**GIVEN** a reviewer reading a failure in an 80-column CI log, **WHEN** a pin problem soft-wraps, **THEN** the artifact is still on the first visual row, because every problem line the pin emits is **location-first** — `{path}:{line} — {message}` where a line is meaningful and `{path} — {message}` where it is not, the form the checker already carries — and the location prefix stays inside the **≤ 48-character** budget the design derives from the 80-column line (`_design.md:419-425`). Anti-pattern 7 is *\"a gate failure whose first visual row does not begin with `path:line`\"* (`_design.md:571-597`). Hierarchy is carried by position and adjacency alone: no colour, no weight, no size, no box-drawing, no second indent level — the terminal primitive inventory `_design.md:31-40` verified is the whole vocabulary, and a primitive outside it is an invented one."
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/lint_narrative.rs — the pin `const` and its checks inside the checker's existing `run`, reached through the `REQUIRED` `probe: None` step `every narrative page is checked` mounted at `xtask/src/main.rs:105`"
  verifying_test: "xtask/src/lint_narrative.rs::tests::every_pin_problem_line_begins_with_its_artifact (cargo test -p xtask)"
- id: AC-008
  criterion: "**GIVEN** a contributor about to trust the pin — the failure mode being that they read a green gate as proof the documentation is *correct* — **WHEN** they open `xtask/src/lint_narrative.rs`, **THEN** the **first** thing they meet is what it does not verify, before the checks and not after, because \"a check whose limits are undocumented is read as a guarantee\" (`xtask/src/lint_constitution.rs:9-28`; `_design.md`, `## What a user meets first`). The pin contributes three limits: an anchor can survive while the *reasoning* around it is rewritten, which is a re-discharge this check cannot detect and which `.kb/governance/rewrite-the-referent-never-the-reasoning.md` covers as a human rule; a clause wording its documentation obligation outside the enumerated candidate phrases is invisible to the derived scan; and the pin proves a discharge is **present**, never that it is **adequate**. **AND** nothing in the module, its messages or its output carries a badge, tick, shield or \"verified\" wording, and nothing anywhere claims the surface proves a page teaches — `_design.md` anti-pattern 9 and project DoD item 8 (`project.md:260`)."
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/lint_narrative.rs — the pin `const` and its checks inside the checker's existing `run`, reached through the `REQUIRED` `probe: None` step `every narrative page is checked` mounted at `xtask/src/main.rs:105`"
  verifying_test: "xtask/src/lint_narrative.rs::tests::the_module_states_the_pins_limits_before_the_pin and ::no_pin_message_claims_a_discharge_is_correct (cargo test -p xtask)"
```
