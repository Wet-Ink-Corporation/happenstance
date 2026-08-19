---
item: HS-S0113
stage: implement
created: "2026-08-12"
updated: "2026-08-12"
---

# Acceptance ledger — The deferrals name experiments and the clause arithmetic comes out

The machine-checkable Definition-of-Done ledger for this story (RFC §6.5). One row per spec
`AC-###`. Planning authors every row with `satisfied: false`; the implementer may only flip a row to
`satisfied: true` and MUST cite real evidence (a `file:line` and/or the verifying test id) — never
edit, remove, or re-word a criterion, and never flip a satisfied row back. `redkiln verify --grain
story` reads the fenced block below and blocks `implement → report` unless every spec AC is present,
`satisfied: true`, and carries non-placeholder evidence (and no AC that was satisfied on the base
branch has regressed). Scope changes are a human decision recorded through `redkiln advance`, not a
quiet ledger edit.

**This story compiles nothing**, so no row's `verifying_test` is a `#[test]` name. The verifying
instruments are the gate's own file-reading checks (`cargo xtask spec-trace`, whose checks live at
`xtask/src/spec_trace.rs`) and the written computation at
`.bklg/from-contract-to-published-library/replication-identity-and-ingest/clause-arithmetic-and-deferral-renewals/_clause-arithmetic.md`.
A green `spec-trace` is the floor for several rows and sufficient for none of them on its own —
CF-38 checks `falsifier.trim().len() < 12` (`xtask/src/spec_trace.rs:659`), which a stale experiment
passes forever. Evidence that cites only the command, with no `_clause-arithmetic.md` section and no
`spec/SPECIFICATION.md` line, is placeholder evidence.

```yaml
- id: AC-001
  criterion: "GIVEN an evaluator who has been handed the intake brief's clause ledger, WHEN they open spec/SPECIFICATION.md §5 at this story's merge commit, THEN the SY population and maturity split written into _clause-arithmetic.md — 21 [FROZEN], 9 [PROVISIONAL], 5 [DEFERRED] (SY-14, SY-18, SY-27, SY-28, SY-32) — is the split the document itself yields, and every place a brief disagrees is resolved for the document and the disagreement recorded rather than quietly dropped."
  satisfied: false
  evidence: ""
  mount_point: "spec/SPECIFICATION.md — §5's SY clause headings and their maturity markers, plus §1.3's stated census sentence (:219-222)"
  verifying_test: "cargo xtask spec-trace (check_stated_census, xtask/src/spec_trace.rs:459-508) green, plus the recorded derivation in .bklg/.../clause-arithmetic-and-deferral-renewals/_clause-arithmetic.md §Re-derived ledger re-run by the reviewer"

- id: AC-002
  criterion: "GIVEN an evaluator reading a [DEFERRED] SY clause to decide whether the promise is being worked on, WHEN they read any of the five, THEN each carries exactly one written disposition — renew (marker stays, interior rewritten in place at the clause, never as a sidecar note or a companion file) or settle (marker moves) — and every settlement names, beside it in _clause-arithmetic.md, the merged ADR-0026 or ADR-0027 statement that authorises it by clause id; where a settlement makes a scheduled rule writable, the rule is recorded as a named handoff and keeps its (new, …) marker until the rule exists."
  satisfied: false
  evidence: ""
  mount_point: "spec/SPECIFICATION.md — the five SY maturity markers (:6234-6260, :6362-6392, :6629-6650, :6653-6670, :6775-6800)"
  verifying_test: "cargo xtask spec-trace check 4 (rule resolution, xtask/src/spec_trace.rs:684-720, :1626-1634) green, plus .bklg/.../clause-arithmetic-and-deferral-renewals/_clause-arithmetic.md §Dispositions (five rows with an authorising-ADR cell) read against git diff spec/SPECIFICATION.md"

- id: AC-003
  criterion: "GIVEN an evaluator who wants to know what would end a deferral, WHEN they read the marker's experiment, THEN it names something that could still falsify the clause today — verified by hand against its referent, not merely by being non-empty — and in particular SY-18's experiment no longer cites building an API that crates/happenstance-sync/src/peer.rs:150-158 already provides while PeerLimits::admits stays advisory (:319-324)."
  satisfied: false
  evidence: ""
  mount_point: "spec/SPECIFICATION.md — the interior of each [DEFERRED] SY marker, in the three-part form §1.3 requires (:204-208)"
  verifying_test: "cargo xtask spec-trace check 2 (CF-38, xtask/src/spec_trace.rs:651-670) green as the floor, plus the per-clause 'what would falsify this' rows in .bklg/.../clause-arithmetic-and-deferral-renewals/_clause-arithmetic.md §Dispositions"

- id: AC-004
  criterion: "GIVEN the next phase's implementer picking up an open clause, WHEN they read its owning phase, THEN no [DEFERRED] SY marker names a phase that has already run: each of the five names a phase still ahead (phase 14 retention-and-incomplete-logs / ADR-0028 being the last one left in the plan, and the owner of the suffix-store instrument), and where no unrun phase can honestly own an experiment a blocker is raised rather than a phase number invented."
  satisfied: false
  evidence: ""
  mount_point: "spec/SPECIFICATION.md — the owning-phase clause of each [DEFERRED] SY marker (:6240-6241, :6368-6369, :6635-6636, :6787-6789)"
  verifying_test: "rg -n 'the phase that builds the two peer adapters' spec/SPECIFICATION.md returns zero hits inside any SY marker, cross-checked against RUNBOOK.md:4626-4670 and recorded in .bklg/.../clause-arithmetic-and-deferral-renewals/_clause-arithmetic.md §Dispositions"

- id: AC-005
  criterion: "GIVEN an evaluator who follows a citation out of a marker to see the evidence, WHEN they open it, THEN it lands on the referent rather than on whatever moved into that address: spec/E2E-CASES.md's completeness instrument is cited at :1682 and not the stale :1595-1600, and references/evaluation/PRESSURE-TEST.md:685-693 is confirmed to still carry the whole-log-versus-scoped experiment."
  satisfied: false
  evidence: ""
  mount_point: "spec/SPECIFICATION.md — every citation inside a rewritten SY marker, notably SY-27 (:6635-6636) and SY-32 (:6783-6784)"
  verifying_test: ".bklg/.../clause-arithmetic-and-deferral-renewals/_clause-arithmetic.md §Citations re-anchored — one row per citation with the matched subject string quoted, per .kb/playbooks/anchoring-citations-in-a-long-lived-document.md"

- id: AC-006
  criterion: "GIVEN an adapter author who would write the round-trip rule SY-28 warns about, WHEN they read SY-27 and SY-28 after this story, THEN the two carry one disposition and neither has moved without the other; and any settlement of SY-27 on whole-log replication justified by a green suite is refused in writing, because no scoped peer exists in the tree to refute it and green is the absence of the instrument rather than evidence."
  satisfied: false
  evidence: ""
  mount_point: "spec/SPECIFICATION.md — SY-27 (:6629-6650) and SY-28 (:6653-6670), whose markers bind them to one experiment"
  verifying_test: ".bklg/.../clause-arithmetic-and-deferral-renewals/_clause-arithmetic.md §Dispositions shows SY-27 and SY-28 with identical disposition and identical experiment, with the written refusal citing spec/SPECIFICATION.md:6665-6670 and CF-1's worked failure (:7161-7207)"

- id: AC-007
  criterion: "GIVEN an evaluator who reads §5.13 and then SY-32 and finds the document disagreeing with itself, WHEN they read them after this story, THEN SY-32's marker names phase 14 / retention-and-incomplete-logs / ADR-0028 as its owner — agreeing with 'SY-32 depends on ES-39 and cannot be settled ahead of it' — and neither clause's normative sentence has been touched."
  satisfied: false
  evidence: ""
  mount_point: "spec/SPECIFICATION.md — SY-32's marker (:6775-6800), read against §5.13 (:7002-7003)"
  verifying_test: "git diff spec/SPECIFICATION.md confined to SY-32's marker interior, matched against the handoff ADR-0026 records for project AC-011 and recorded in .bklg/.../clause-arithmetic-and-deferral-renewals/_clause-arithmetic.md §Dispositions"

- id: AC-008
  criterion: "GIVEN a reviewer asking whether this project covered the clause range it claimed, WHEN they open _clause-arithmetic.md, THEN they find a set computation, not an assertion: (ADR-0026 claims ∪ ADR-0027 claims ∪ {SY-32 → ADR-0028}) against {SY-1 … SY-35}, differenced in both directions (claimed by nothing; claimed twice), with claims counted and citations excluded — ADR-0026 cites SY-1, SY-2 and SY-6 without claiming them — and the two ADRs read as merged, never as planned."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/from-contract-to-published-library/replication-identity-and-ingest/clause-arithmetic-and-deferral-renewals/_clause-arithmetic.md §The arithmetic — the artifact project AC-014's 'computed and written down at exit' names"
  verifying_test: "The two explicit id lists and two difference lists in _clause-arithmetic.md §The arithmetic, each per-ADR claim list cited by file:line into the merged atom under .kb/decisions/ (long form under references/adr/)"

- id: AC-009
  criterion: "GIVEN SY-33, SY-34 and SY-35 were assigned to no ADR by the intake split, WHEN the arithmetic runs, THEN the merged records are checked for those three ids specifically and the result stated either way — claimed, or a shortfall of exactly three recorded with a named owner and raised — and in neither case is RUNBOOK.md:4535's range edited to make the totals agree."
  satisfied: false
  evidence: ""
  mount_point: "spec/SPECIFICATION.md — SY-33 (:6808), SY-34 (:6833), SY-35 (:6868); the assignment recorded in _clause-arithmetic.md §The arithmetic"
  verifying_test: "git diff --name-only contains no RUNBOOK.md, plus the three ids named individually with their claiming record or shortfall row in .bklg/.../clause-arithmetic-and-deferral-renewals/_clause-arithmetic.md §The arithmetic"

- id: AC-010
  criterion: "GIVEN the repository owner reviewing this project's exit against DoD 7, WHEN they read git diff spec/SPECIFICATION.md for this story, THEN it shows marker interiors, at most §1.3's hand-written census sentence, and the spec-trace-regenerated §7.1–§7.2 — no normative MUST, no Rejects: field, no Rule: line, and no maturity marker moved without a named authorising ADR — with the order honoured (markers → §1.3 by hand → cargo xtask spec-trace --write → Mode::Check) and cargo xtask spec-trace green."
  satisfied: false
  evidence: ""
  mount_point: "spec/SPECIFICATION.md — reached by the gate through the REQUIRED 'specification traceability' step (xtask/src/main.rs:303-328) and per story through reachability_static (.redkiln/config.yaml:48)"
  verifying_test: "cargo xtask lints && cargo xtask spec-trace green in Mode::Check (region equality at xtask/src/spec_trace.rs:735-744), a clean re-check after cargo xtask spec-trace --write, and cargo xtask ci --fast green (.redkiln/config.yaml:55)"
```
