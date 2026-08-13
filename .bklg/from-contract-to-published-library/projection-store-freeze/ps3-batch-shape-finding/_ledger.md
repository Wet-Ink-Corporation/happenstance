---
item: HS-S0014
stage: implement
created: 2026-08-12T13:46:08.816Z
updated: 2026-08-12T13:46:08.816Z
---

# Acceptance ledger — The PS-3 evidence written as a finding, not a verdict

The machine-checkable Definition-of-Done ledger for this story (RFC §6.5). One row per spec
`AC-###`. Planning authors every row with `satisfied: false`; the implementer may only flip a row to
`satisfied: true` and MUST cite real evidence (a `file:line` and/or the verifying test id) — never
edit, remove, or re-word a criterion, and never flip a satisfied row back. `redkiln verify --grain
story` reads the fenced block below and blocks `implement → report` unless every spec AC is present,
`satisfied: true`, and carries non-placeholder evidence (and no AC that was satisfied on the base
branch has regressed). Scope changes are a human decision recorded through `redkiln advance`, not a
quiet ledger edit.

Two notes specific to this story, both from `spec.md`. First, AC-015 is typed **E2E (process,
derived from AC-004)** (`../_decomposition.md:785`), so several `verifying_test` values name a gate
step or a recorded mechanical comparison rather than a `#[test]` — that is the tier, not a gap.
Second, three rows (AC-003, AC-006, AC-007) are discharged by the story review, because no tool can
check whether a document reads as a verdict; their evidence must be the review's own `file:line`
findings against `references/evaluation/projection-batch-shape-evidence.md`, not a self-assessment.

```yaml
- id: AC-001
  criterion: "GIVEN a planner who has never read this project's implementation history, WHEN they follow a single path from the specification or the evidence index, THEN they land on `references/evaluation/projection-batch-shape-evidence.md`, whose first block states the date, the commit sha of the run it reports, and that it is superseded rather than edited — AND no `.kb/decisions/` atom, `.kb/reference/` atom or `.kb/_intake/` file was created by this story, so the planner is reading evidence they may still decide against, not a decision they would have to supersede."
  satisfied: false
  evidence: ""
  mount_point: "references/evaluation/README.md"
  verifying_test: "Static: the file exists at references/evaluation/projection-batch-shape-evidence.md with its lifecycle header (date, pinned sha, supersede-not-edit) per references/evaluation/README.md:9-13; `git status --porcelain .kb/` empty for this PR; `redkiln validate --kb` green"
- id: AC-002
  criterion: "GIVEN a planner who cannot re-run the suite themselves, WHEN they ask \"what was this actually observed on?\", THEN the finding names both fixtures by their real identifiers (the CF-5 buffering replay-at-commit variant in `crates/happenstance-testkit/tests/`, and apply-on-write `MemoryProjectionStore`), the harness invocation that drove them, and the single `cargo xtask ci` invocation with its sha — one run, not two `cargo test` runs reconciled by hand."
  satisfied: false
  evidence: ""
  mount_point: "references/evaluation/README.md"
  verifying_test: "E2E (process, derived from AC-004): the slice-mate's single `cargo xtask ci` run (buffering-conformant-variant, HS-S0013) per ../_decomposition.md:774; `git cat-file -e <sha>` resolves the pinned sha on this branch; review of the fixture identifiers against crates/happenstance-testkit/tests/"
- id: AC-003
  criterion: "GIVEN two readers of the same evidence — the HS-P0016 planner and the HS-P0015 verdict author — WHEN each classifies the same observation, THEN they reach the same label, because the finding states the vocabulary operationally before the ledger: D1 per-shape handling · D2 asymmetric declension · D3 assertion loosened · D4 divergent observable behaviour, plus agreed and not comparable as distinct verdicts — and no fifth category is minted inside the ledger to accommodate a row."
  satisfied: false
  evidence: ""
  mount_point: "references/evaluation/README.md"
  verifying_test: "Story review against spec.md Context pack §4 and discover.md:40-45, plus the mechanical check that the set of verdict labels used in the ledger column is a subset of the six defined labels"
- id: AC-004
  criterion: "GIVEN a planner who must know the evidence is complete and not curated, WHEN they compare the finding's ledger against the suite's own rule enumeration, THEN every rule named in `for_each_projection_store_rule!` appears exactly once, and no row names a rule that is not in it — so a rule cannot be silently absent from the evidence the way a rule must not be silently absent from a run."
  satisfied: false
  evidence: ""
  mount_point: "references/evaluation/README.md"
  verifying_test: "Static (mechanical, both directions): ledger rule-name set == arm set of `for_each_projection_store_rule!` in crates/happenstance-testkit/src/registry.rs at the pinned sha, mirroring `no_orphan_rules` (crates/happenstance-testkit/src/registry.rs:412); the comparison is recorded in the implementation report"
- id: AC-005
  criterion: "GIVEN a planner deciding how much a \"skip\" weakens the evidence, WHEN they read any D2 row or any not-comparable row, THEN it cites the `RuleOutcome::Skipped` value and the capability constant that produced it — a fixture-declared `Capability`, or `ProjectionProbe::READS_THROUGH_BATCH` — never a remembered or quoted stdout line, because libtest suppresses that line for a passing test unless `--show-output` is passed."
  satisfied: false
  evidence: ""
  mount_point: "references/evaluation/README.md"
  verifying_test: "Review: every D2 / not-comparable row carries a citation resolving into crates/happenstance-testkit/src/contract.rs:473-537 (`RuleOutcome`, `skip_line`, `report`), per ../project.md:194-197"
- id: AC-006
  criterion: "GIVEN a planner reading a ledger with no disagreements in it, WHEN they look for what that means, THEN the finding says so explicitly as a result and weighs Architecture brief Note 10 item 2's two candidate readings — the rules are shape-blind in a way that hides the axis, or the axis is not where §4.2 says it is — choosing one with its reasoning, or recording that this evidence cannot distinguish them. \"No disagreements observed\" as the whole of it does not discharge this, and neither does omitting the section because there was nothing to report."
  satisfied: false
  evidence: ""
  mount_point: "references/evaluation/README.md"
  verifying_test: "Story review against ../_decomposition.md:716-719 and discover.md:89-98: a named section addressing both readings exists whenever the ledger is all-agreed, and its conclusion is one of {reading A, reading B, this evidence cannot distinguish them}"
- id: AC-007
  criterion: "GIVEN a planner who could otherwise inherit a conclusion as though it were evidence, WHEN they reach the end of the finding, THEN they meet two mandatory statements and no third: that PS-2's bar — two adapters at opposite ends of the batch-shape axis — is not met by anything this project built alone, citing PS-2's Rejects clause naming the two-instrument monoculture verbatim; and that no exposure verdict is made here, naming HS-P0016 as owner of the `unstable-projection` call and HS-P0015 as owner of the freeze verdict. The document contains no recommendation, preference, leaning or prediction about when PS-2's bar will be met."
  satisfied: false
  evidence: ""
  mount_point: "references/evaluation/README.md"
  verifying_test: "Story review reading for discover.md:75-87's named wrong implementation, with both statements checked against spec/SPECIFICATION.md:4760-4775, ../_decomposition.md:307-311,436-456 and ../project.md:129-131,301-307"
- id: AC-008
  criterion: "GIVEN a planner who does not already know this file's name, WHEN they arrive from either direction a reader actually travels — browsing `references/evaluation/README.md`, or following PS-3's clause body — THEN they find it: one index entry in the README's \"Later additions, which are neither\" section, and one additive sentence in PS-3 citing it by `file:line`. AND `cargo xtask ci` is green including `spec-trace`; PS-2's body is byte-identical; PS-3's `[PROVISIONAL]` marker, `Rule:`, `Cases:` and `Rejects:` fields are unchanged; no other clause is touched."
  satisfied: false
  evidence: ""
  mount_point: "references/evaluation/README.md"
  verifying_test: "Static: `cargo xtask spec-trace` (mandatory step of `cargo xtask ci`) resolves the PS-3 citation via `check_citations` (xtask/src/spec_trace.rs:297-370, ANCHOR_SLACK=12 at :391); `git diff spec/SPECIFICATION.md` shows exactly one added sentence inside PS-3 and nothing inside PS-2; review of the README entry against references/evaluation/README.md:40-48"
```
