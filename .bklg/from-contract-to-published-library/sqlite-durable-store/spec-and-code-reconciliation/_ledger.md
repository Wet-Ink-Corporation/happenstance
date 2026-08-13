---
item: HS-S0047
stage: implement
created: 2026-08-12
updated: 2026-08-12
---

# Acceptance ledger — The standing reconciliation criterion, discharged

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
  criterion: "GIVEN the maintainer of phase 9 opens any clause in phase 8's computed range, WHEN they read its MUST and its supporting prose against the merged tree, THEN every one of those clauses carries an explicit verdict — unchanged, repaired, or gap — in _reconciliation.md, and a clause whose MUST is untouched but whose prose describes a superseded implementation is recorded as a defect and not as a pass. No clause in the range is absent from the table, and none carries a verdict of \"looks fine\"."
  satisfied: false
  evidence: ""
  mount_point: "spec/SPECIFICATION.md"
  verifying_test: ".bklg/from-contract-to-published-library/sqlite-durable-store/spec-and-code-reconciliation/_reconciliation.md (per-clause verdict table, one row per ID in the AC-004 range) + `cargo xtask spec-trace` green"

- id: AC-002
  criterion: "GIVEN a reader following a clause's own file:line pointer into crates/happenstance-sqlite, WHEN they land, THEN they find the subject the clause said they would: the four schema-sketch citations no longer point at the sketch the runbook itself calls wrong, §5's projection-store census sentence states the count that is now true, the [PROVISIONAL] marker naming this crate as its instrument no longer waits for an event that has already happened, and D7's read-path paragraph no longer explains conformance by the behaviour of a todo!() body. Every drifted pointer is re-anchored or repaired — none is deleted."
  satisfied: false
  evidence: ""
  mount_point: "spec/SPECIFICATION.md"
  verifying_test: "`cargo xtask spec-trace` — windowed subject match at xtask/src/spec_trace.rs:374-390 passing for every repaired citation, with each of spec/SPECIFICATION.md:372, 2552, 2642, 2945, 3326, 3841, 3958, 4393, 4586, 4604, 7218, 7285 dispositioned in _reconciliation.md"

- id: AC-003
  criterion: "GIVEN a reviewer who must be able to trust that this pass changed no promise, WHEN they diff spec/SPECIFICATION.md, THEN every edit is a repair — the set of implementations the clause admits is unchanged — and any finding that would change what a [FROZEN] clause admits appears as a recorded, escalated gap inside the clause it is about, with nothing normative altered and no ADR written here. A met obligation keeps its MUST verbatim, is named as a discharge, and cites the code and the test that assert it."
  satisfied: false
  evidence: ""
  mount_point: "spec/SPECIFICATION.md"
  verifying_test: "Diff review against .kb/playbooks/repairing-a-frozen-clause-without-amending-it.md's discriminator: no [FROZEN] clause MUST text changes in `git diff`, and every gap row in _reconciliation.md names its runbook ADR-queue escalation"

- id: AC-004
  criterion: "GIVEN the phase-4 precedent, where queue rows named 35 clauses and the body discharged 64 and the 29 missing were invisible in exactly the way a finished clause is invisible (RUNBOOK.md:315-322), WHEN phase 8 closes, THEN two enumerated ID sets exist side by side — the phase's stated clause range and the union of its ADRs' ranges — and every element of the symmetric difference carries a disposition: already discharged elsewhere, owed by ADR-0022, or escalated. The three disagreeing statements of the phase's range (RUNBOOK.md:4175-4176, RUNBOOK.md:605-606, _intake-brief.md:76-92) are reconciled into one set rather than averaged, and ADR-0022's queue row carrying no parenthesised range at all (RUNBOOK.md:301) is itself recorded as a finding."
  satisfied: false
  evidence: ""
  mount_point: "spec/SPECIFICATION.md"
  verifying_test: ".bklg/from-contract-to-published-library/sqlite-durable-store/spec-and-code-reconciliation/_reconciliation.md §Clause ranges — two literal ID sets, their symmetric difference computed, one disposition row per element"

- id: AC-005
  criterion: "GIVEN a consumer who needs to know the specification's coverage did not shrink to buy its greenness, WHEN the slice merges, THEN `cargo xtask spec-trace` is green and both figures in its summary line — N citations checked and M anchored to their subject — are at or above the figures recorded from the pre-slice merge-base. Deleting a citation to silence a failure is not a remedy (it lowers checked); re-pointing one at a line whose subject no longer matches is not a remedy either (it lowers anchored). §1.3's hand census is edited by hand in the same commit if any maturity marker moved; §7.1/§7.2 are regenerated with `cargo xtask spec-trace --write` and never hand-edited."
  satisfied: false
  evidence: ""
  mount_point: "spec/SPECIFICATION.md"
  verifying_test: "`cargo xtask spec-trace` (summary line emitted at xtask/src/spec_trace.rs:995-1007; census check at :447-471) run at the recorded merge-base sha and on the merged tree, both figures compared in _reconciliation.md; gated by `cargo xtask affected --base main` (.redkiln/config.yaml:40)"

- id: AC-006
  criterion: "GIVEN publication-and-positioning's clause-ledger audit must read a decision rather than a silence (project DR-07), WHEN it opens CF-14, CF-17 and ES-35, THEN each carries the verdict reopen-negative-control-and-durability-verdicts produced, transcribed here without being re-opened, and every surviving [PROVISIONAL] / [DEFERRED] marker names a falsifier that is still live and long enough to satisfy CF-38's build failure. The reconciliation open question is left open: no committed citation baseline, no new gate check, no machine-readable clause-range format — findings that argue for mechanisation are staged under .kb/_intake/ for /redkiln:kb-ingest to adjudicate, never hand-written into .kb/."
  satisfied: false
  evidence: ""
  mount_point: "spec/SPECIFICATION.md"
  verifying_test: "`cargo xtask spec-trace` falsifier-length check (xtask/src/spec_trace.rs:659-662) + `redkiln validate --kb && redkiln doctor` clean, with CF-14/CF-17/ES-35 each traced in _reconciliation.md to the upstream story's ledger row and .kb/open-questions/nothing-owns-the-post-phase-reconciliation.md unedited in the diff"
```
