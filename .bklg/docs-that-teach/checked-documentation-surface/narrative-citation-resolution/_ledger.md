---
item: HS-S0142
stage: implement
created: "2026-08-17T13:16:04.504Z"
updated: "2026-08-17T13:16:04.504Z"
---

# Acceptance ledger — A clause id a page cites either resolves or fails the gate

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
  criterion: "GIVEN a maintainer who later adds a seventh clause family to `SECTIONS` — the one place the six families are enumerated, whose own doc comment says three lists must agree \"so that adding a family cannot half-land\" (`xtask/src/spec_trace.rs:107-120`) — WHEN they add it there and run the gate, THEN the narrative checker recognises citations in the new family with no second edit, because it resolves through `crate::spec_trace::clause_ids(&root)` and derives its recognised prefixes from that set's own ids rather than from a literal list that would be the fourth. `clause_ids` is called once in `run()`, before the page loop, and the `BTreeSet<String>` is passed by reference to the check. AND because this PR is `clause_ids`' first caller, the foundation's `#[expect(dead_code, reason = …)]` becomes unfulfilled — itself a warning, which `-D warnings` makes a failure — so it is deleted in the same change and `#[allow(dead_code)]` appears nowhere in the diff."
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/lint_narrative.rs — the single `crate::spec_trace::clause_ids(&root)?` call in `run()`, before the page loop, with the family derivation beside it; paired with the deletion-only edit to xtask/src/spec_trace.rs (the `#[expect(dead_code, reason = …)]` on `clause_ids`, beside `all_rules` at :1746). xtask/src/main.rs is deliberately untouched."
  verifying_test: "xtask/src/lint_narrative.rs::tests::the_recognised_families_are_derived_from_the_resolved_set (cargo test -p xtask), plus the clippy `-D warnings` step of cargo xtask ci --fast for the expect-deletion half"

- id: AC-002
  criterion: "GIVEN an application author who has written a teaching page and cited the clause the page's claim rests on — the fixture page's own `(ES-40)`, which is a claim about `spec/SPECIFICATION.md` that nothing verifies today — WHEN they run `cargo xtask narrative` or the gate, THEN the page passes with no output about the citation at all, and the step still prints exactly one summary line. Provenance is checked, not narrated: a check that reports each resolving citation makes the one line that matters harder to find (`_design.md`, `## Transience policy`)."
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/lint_narrative.rs — the `check_citations(page, &ids, &mut problems)` call site in `run()`'s per-page check list, mirroring xtask/src/lint_constitution.rs:180-190; observable through the already-mounted `REQUIRED` step `every narrative page is checked`"
  verifying_test: "xtask/src/lint_narrative.rs::tests::a_citation_naming_a_declared_clause_is_not_a_problem (cargo test -p xtask), plus `cargo xtask narrative` standalone over the tree as milestone 2 left it (one banner, one summary line)"

- id: AC-003
  criterion: "GIVEN an evaluator who follows a clause citation on a teaching page and lands on nothing — the provenance failure AC-007 exists because of, and one the reviewer half of BR-09 cannot catch at scale — WHEN the gate runs over a page citing an id `spec/SPECIFICATION.md` does not declare, THEN the run fails, naming the page, the line and the id, in the composed form the signed-off design already drew: ``docs/append-conditions.md:12 — cites `ES-99`, which SPECIFICATION.md does not define`` (`_design.md:327-339`). A message naming only the id, or only the page, does not satisfy this row: the location must be first so soft-wrap in an 80-column log cannot push it off the first visual row."
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/lint_narrative.rs — the dangling-id problem composed onto the module's existing accumulating `Vec<String>`, in the surface `gate-narrative-checker-step`'s `fail-one`/`fail-many` states (_design.md:327-352, :555-569)"
  verifying_test: "xtask/src/lint_narrative.rs::tests::a_dangling_id_names_the_page_the_line_and_the_id (cargo test -p xtask), plus `cargo xtask narrative` over a deliberately dangling page read against _design.md:327-352"

- id: AC-004
  criterion: "GIVEN a page author who wrote something that looks like a clause citation and is not one the checker can resolve — `ES-` with no digits, `ES-4O` with a letter for a zero, or a family (`XX-7`) the specification declares nowhere — WHEN the gate runs, THEN each is reported as a problem, never skipped, and the undeclared-family form names the families the specification does declare so the author can spell what they meant. `spec_trace::citations` silently skips a span it does not recognise, which is safe there (`xtask/src/spec_trace.rs:2215-2260`); the opposite posture is mandatory here, for `lint_constitution`'s stated reason — \"the span *is* the check, so a citation this parser declines to read is a citation nothing verifies\" (`xtask/src/lint_constitution.rs:30-44`, and the problem it pushes at `:688-693`). A checker that silently non-matches is the named wrong implementation this row rejects."
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/lint_narrative.rs — the candidate-token classifier inside the citation check: the `let Some(x) = … else { push; continue }` posture copied from xtask/src/lint_constitution.rs:674-693, pushing onto the module's existing problem list"
  verifying_test: "xtask/src/lint_narrative.rs::tests::a_citation_shaped_token_in_an_undeclared_family_is_a_problem and ::tests::a_near_miss_inside_a_declared_family_is_a_problem (cargo test -p xtask)"

- id: AC-005
  criterion: "GIVEN a page author who legitimately cites this repository's own constitution rule ids and ADR numbers in prose — `RS-81-1`, `ADR-0001`, and a backlog item id such as `HS-S0142` — WHEN the gate runs, THEN none of them is reported, because the token shape carries both boundaries: the preceding character is not ASCII-alphanumeric (so `ADR-0001` is never read as `DR-0001`) and the following character is neither a digit, a hyphen, nor alphanumeric (so `RS-81-1` is not read as a dangling `RS-81`). `HS-S0142` fails the shape outright — no digit follows the hyphen. A checker missing either boundary fails on prose that is correct, which is worse than missing a defect: it teaches contributors to remove true sentences."
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/lint_narrative.rs — the `fn clause_citations(line: &str) -> Vec<&str>`-shaped scanner and its two `char` boundary predicates (no regex crate, per NF-002/DR-12)"
  verifying_test: "xtask/src/lint_narrative.rs::tests::a_constitution_rule_id_is_not_a_clause_citation, ::tests::an_adr_reference_is_not_a_clause_citation, ::tests::a_backlog_item_id_is_not_a_clause_citation and ::tests::the_boundaries_are_both_load_bearing (cargo test -p xtask)"

- id: AC-006
  criterion: "GIVEN a reviewer reading a failing CI log they cannot re-run, on a branch where five pages each cite a dangling id, WHEN the checker fails, THEN they get every problem in one run — pushed onto the module's existing `Vec<String>`, two-space indent, `{path}:{line} — {message}`, pages in enumeration order and lines ascending, count and directory last, never truncated and never `… and N more` — because \"a check that stops at the first problem turns one review cycle into six\" (`_decomposition.md:218-219`). AND the scan covers the whole page including fenced blocks — a clause id in a comment above an example is still a claim, and excluding fences would put a hiding place inside the checked artifact — consuming the page text the module already read once, not re-reading the file."
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/lint_narrative.rs — the check function's signature takes the already-read page text plus the page, and accumulates onto the module's existing `Vec<String>` ahead of its `bail!(\"{n} problem(s) in {TREE}\")`; no second `read_to_string` of a page in the diff"
  verifying_test: "xtask/src/lint_narrative.rs::tests::five_dangling_ids_on_one_page_report_five_problems_in_source_order and ::tests::a_clause_id_inside_a_fence_is_still_checked (cargo test -p xtask)"

- id: AC-007
  criterion: "GIVEN a contributor whose checkout has a truncated, moved or unreadable `spec/SPECIFICATION.md`, or a page that is not valid UTF-8, WHEN the gate runs, THEN the run fails with a message naming the artifact that actually broke — the specification, with the checker blamed rather than the pages; or the page, by path — and never reports every real citation on every page as dangling, and never skips the file it could not inspect. `clause_ids`' error is propagated unchanged (`../spec-trace-clause-id-accessor/spec.md` AC-004, EC-001/EC-002); an unreadable page is a hard error because \"a scanner that silently skips what it cannot read reports green over exactly the file it failed to inspect\" (`standards/rust/81-checks-that-cannot-be-types.md:95`, RS-81-2). A page blamed for the specification's condition does not satisfy this row."
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/lint_narrative.rs — the `clause_ids(&root)?` propagation point in `run()` (no bespoke message, no `unwrap_or_default()`), and the module's existing page-read error context"
  verifying_test: "xtask/src/lint_narrative.rs::tests::an_unreadable_specification_is_propagated_not_swallowed and ::tests::an_unreadable_page_is_a_hard_error_naming_the_page (cargo test -p xtask)"

- id: AC-008
  criterion: "GIVEN a contributor or a downstream project (HS-P0021/22/23) opening this check for the first time to decide how far to trust it, WHEN they read the module, THEN the first thing they meet is `# What this does not verify`, now carrying this check's own limits — that a resolving citation says nothing about whether the sentence above it is true, nor whether the page defers to the clause rather than restating it (BR-09's reviewer half, HS-P0021's); that a clause-shaped token inside a fence is not distinguished from prose; and that a doubly-declared id collapses upstream in `clause_ids` — AND nowhere in the module, its output, `docs/README.md` or this story's own artefacts does anything claim the surface proves a page teaches. A check whose limits are undocumented is read as a guarantee (`xtask/src/lint_constitution.rs:9-13`, RS-81-1), and a green step read as evidence of teachability is the initiative's top-ranked risk (`project.md:286`)."
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/lint_narrative.rs — the module's existing `# What this does not verify` section (created by narrative-checker-mounted-with-pinned-path), appended to rather than duplicated; rendered by cargo xtask ci's docs step"
  verifying_test: "xtask/src/lint_narrative.rs::tests::the_citation_check_documents_its_limits_before_its_guarantee (cargo test -p xtask), plus cargo xtask lint-constitution and cargo test --locked -p xtask --doc"
```
