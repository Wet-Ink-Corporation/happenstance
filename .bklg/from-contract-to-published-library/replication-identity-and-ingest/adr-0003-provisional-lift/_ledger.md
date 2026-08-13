---
item: HS-S0108
stage: implement
created: "2026-08-12"
updated: "2026-08-12"
---

# Acceptance ledger — ADR-0003 loses provisional by a new atom, never by an edit

The machine-checkable Definition-of-Done ledger for this story (RFC §6.5). One row per spec
`AC-###`. Planning authors every row with `satisfied: false`; the implementer may only flip a row to
`satisfied: true` and MUST cite real evidence (a `file:line` and/or the verifying test id) — never
edit, remove, or re-word a criterion, and never flip a satisfied row back. `redkiln verify --grain
story` reads the fenced block below and blocks `implement → report` unless every spec AC is present,
`satisfied: true`, and carries non-placeholder evidence (and no AC that was satisfied on the base
branch has regressed). Scope changes are a human decision recorded through `redkiln advance`, not a
quiet ledger edit.

This story compiles nothing, so every `verifying_test` is a file-reading gate command or a `git
diff` assertion rather than a `#[test]`. That is the whole instrument set named in
[`../_decomposition.md`](../_decomposition.md) *The test mix, tier by tier*, Static, and in
[`spec.md`](spec.md) *Tests and CI*.

```yaml
- id: AC-001
  criterion: "GIVEN an evaluator who follows CLAUDE.md binding constraint 2 to .kb/decisions/0003-opaque-payloads.md to judge whether this project's signed decisions are stable, WHEN this PR has merged, THEN the atom they read is byte-identical to the one that was there before — body and frontmatter, status: accepted and superseded_by: null included — so no signed decision was rewritten under them to make a later claim true."
  satisfied: false
  evidence: "" # file:line and/or verifying test id — required once satisfied
  mount_point: ".kb/maps/decision-map.md"
  verifying_test: "git diff --exit-code HEAD~..HEAD -- .kb/decisions/0003-opaque-payloads.md (no change) AND redkiln validate --kb (accepted-decision immutability against HEAD, .kb/decisions/README.md:7-13)"

- id: AC-002
  criterion: "GIVEN the same evaluator asking whether this knowledge base is a process output or a hand-maintained pile of markdown, WHEN they read the diff, THEN every atom it adds under .kb/ was written by /redkiln:kb-ingest from documents staged in .kb/_intake/, the wave records itself under .kb/_governance/integration-waves/, .kb/_intake/ is left cleared, and redkiln validate --kb && redkiln doctor are green."
  satisfied: false
  evidence: ""
  mount_point: ".kb/maps/decision-map.md"
  verifying_test: "redkiln validate --kb && redkiln doctor; a new wave directory exists under .kb/_governance/integration-waves/; .kb/_intake/ holds only its README.md"

- id: AC-003
  criterion: "GIVEN the next contributor who wants serde in happenstance-core and follows binding constraint 2 to see what stands against it, WHEN they land on ADR-0003 after this PR, THEN it is still in force and not retired: the new atom declares supersedes: null and depends_on: [kb-decision-0003], ADR-0003 keeps superseded_by: null, and the new atom's summary says in its own words that ADR-0003's body stays verbatim and its constraints stay in force."
  satisfied: false
  evidence: ""
  mount_point: ".kb/maps/decision-map.md"
  verifying_test: "rg -n '^supersedes:|^depends_on:' .kb/decisions/0030-*.md AND rg -n '^superseded_by:' .kb/decisions/0003-opaque-payloads.md (still null) AND redkiln validate --kb link resolution; shape precedent .kb/decisions/0029-msrv-raised-to-1-97-1.md:12-14, :25-26"

- id: AC-004
  criterion: "GIVEN a reader scanning the decision corpus for what each atom decides, WHEN they reach the lift, THEN it is a standalone atom numbered ADR-0030 whose title states the lift and nothing else — it does not also settle what a peer is, what ingest promises, or what serde may be used for — and RUNBOOK.md's ADR queue carries a row for ADR-0030 in the same 'unscheduled — the queue had no number for it' form RUNBOOK.md:287 already uses for ADR-0029, so an ADR number invisible to the queue cannot happen twice."
  satisfied: false
  evidence: ""
  mount_point: ".kb/maps/decision-map.md"
  verifying_test: "rg -n 'ADR-0030' RUNBOOK.md returns a queue row inside RUNBOOK.md:285-308 AND rg -n '0030' .kb/decisions/0026-*.md .kb/decisions/0027-*.md returns nothing AND the atom's title names one subject (.kb/playbooks/one-decision-per-adr-title.md)"

- id: AC-005
  criterion: "GIVEN an evaluator who cannot run the suite and has to trust what the project published, WHEN they open the evidence the lift rests on, THEN a kind: reference atom records what was run, on which commit sha, on what date, and what it returned, and the ADR-0030 decision atom cites it by id rather than restating how the number was obtained — so a measurement that later stops being true is detectable rather than reading as current."
  satisfied: false
  evidence: ""
  mount_point: ".kb/maps/decision-map.md"
  verifying_test: "rg -n '^kind: reference' .kb/reference/<new-atom>.md; the atom body carries a commit sha and a date (.kb/reference/README.md dating rule; precedent .kb/reference/wire-format-encoding-measurements.md); ADR-0030's related: names it and redkiln validate --kb resolves the link"

- id: AC-006
  criterion: "GIVEN ADR-0003's own lift condition — 'round-trips an event between two stores without deserialising its payload' (.kb/decisions/0003-opaque-payloads.md:91-93) — quoted verbatim rather than paraphrased, WHEN ADR-0030 asserts it discharged, THEN it names the assertion and the negative control by path: the payload Bytes compared for equality at the receiver after commit, the replay witness, and the conformant undecodable-payload variant (PayloadTouchingSuite, fails: &[]) passing — never 'AC-006 green', which is equally true of a comparison on decoded values. OR, if either conjunct failed, ADR-0030 lifts nothing and instead records which assertion failed and what would have to be true to lift."
  satisfied: false
  evidence: ""
  mount_point: ".kb/maps/decision-map.md"
  verifying_test: "rg -F the quoted lift sentence against .kb/decisions/0003-opaque-payloads.md (exact match); every evidence path cited in .kb/decisions/0030-*.md exists under crates/happenstance-sync-testkit/ at the cited commit; rg -n 'AC-006' .kb/decisions/0030-*.md finds no AC id standing in for an assertion"

- id: AC-007
  criterion: "GIVEN an adapter author who reads a 'provisional lifted' atom and asks what it loosened, WHEN they read ADR-0030, THEN it states that nothing was retired: Event::data stays bytes::Bytes, happenstance-core carries no serde in its default features, and the serde feature covers envelope types only — and that happenstance, the typed layer, is unaffected and may still depend on serde."
  satisfied: false
  evidence: ""
  mount_point: ".kb/maps/decision-map.md"
  verifying_test: "rg -n 'happenstance-core|bytes::Bytes|envelope types only|typed layer' .kb/decisions/0030-*.md finds all four claims restated as in force, and no line relaxing them; cross-read against CLAUDE.md binding constraint 2"

- id: AC-008
  criterion: "GIVEN a reader who starts at the index rather than at an ADR number, WHEN they open .kb/maps/decision-map.md after this PR, THEN ADR-0003's row no longer reads 'accepted (provisional)' and carries the amendment edge in the shape :65 and :76-79 already use for ADR-0029 → ADR-0004, ADR-0030 and the reference atom have their own rows, and the long-form record lands beside as references/adr/0030-*.md while references/adr/0003-opaque-payloads.md does not move — so SY-12's line citation still resolves."
  satisfied: false
  evidence: ""
  mount_point: ".kb/maps/decision-map.md"
  verifying_test: "rg -n 'ADR-0003|ADR-0030' .kb/maps/decision-map.md shows the updated status, the amendment edge and a new row; git diff --exit-code HEAD~..HEAD -- references/adr/0003-opaque-payloads.md reports no change; cargo xtask spec-trace green (xtask/src/spec_trace.rs)"
```
