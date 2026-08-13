---
item: HS-S0001
stage: implement
created: 2026-08-12T13:45:56.341Z
updated: 2026-08-12T13:45:56.341Z
---

# Acceptance ledger — Sweep PS-1 – PS-37 for the pairing defect before any repair is scoped

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
  criterion: "**GIVEN** an adapter author about to implement PS-16, who cannot tell from `spec/SPECIFICATION.md` alone whether any clause other than PS-1 and PS-19 asks for more than it says, **WHEN** they open `references/evaluation/ps-clause-pairing-sweep.md`, **THEN** they find a row for every clause PS-1 through PS-37 with no omissions and no \"not applicable\" escapes — including the seven whose §7.2 rule cell reads `*(none — see clause)*` (PS-9, PS-31, PS-32, PS-33, PS-35, PS-36, PS-37), for which \"no rule assigned\" is itself the pairing under verdict — and each row carries exactly one verdict from the closed vocabulary `sound` / `defective` / `undetermined`."
  satisfied: false
  evidence: ""
  mount_point: "references/evaluation/README.md — the \"Later additions, which are neither\" section (:40-55), where the census document is registered"
  verifying_test: "Static census check over references/evaluation/ps-clause-pairing-sweep.md — rg over the census table yields 37 unique PS-row ids (PS-1 … PS-37, none missing, none duplicated), and every verdict cell is one of sound / defective / undetermined; recorded in _review.md"
- id: AC-002
  criterion: "**GIVEN** that same author asking \"is this rule the right rule for this clause?\" — the exact question `cargo xtask spec-trace` states it cannot answer (`xtask/src/spec_trace.rs:17-18`) — **WHEN** they read the document's method section, **THEN** they find **one** stated classifier, applied identically to every `(clause, rule)` pair drawn from all three attribution sources (the clause body's own `**Rule:**` field, §4.11's rule → clause table, §7.2's clause → rule index): *does an implementation exist that satisfies the clause's `MUST` verbatim and fails that rule?* — yes → `defective` (a gap, an ADR's), no → `sound`, cannot tell → `undetermined`, and **THEN** they can re-apply it to a clause themselves and reach the same verdict."
  satisfied: false
  evidence: ""
  mount_point: "references/evaluation/README.md — the \"Later additions, which are neither\" section (:40-55), where the census document is registered"
  verifying_test: "Adversarial slice review recorded in .bklg/from-contract-to-published-library/projection-store-freeze/ps-clause-pairing-sweep/_review.md — the method section states the classifier against .kb/decisions/README.md:20-22 and .kb/playbooks/repairing-a-frozen-clause-without-amending-it.md:56-72, and three sampled rows are independently re-derived to the same verdict"
- id: AC-003
  criterion: "**GIVEN** the corollary that a finding no implementation can exhibit is decorative in exactly the way a rule no adapter can fail is (`CLAUDE.md`, \"The rule that matters\"), **WHEN** the author reads any row verdicted `defective`, **THEN** it names the shape of store that satisfies the clause's `MUST` and fails the assigned rule — concretely enough to be built — and says whether that store is a plausible first cut or a contrivance; **AND** a row for which no such store can be named is recorded `undetermined`, never rounded to `sound` and never left as an unsupported `defective`."
  satisfied: false
  evidence: ""
  mount_point: "references/evaluation/README.md — the \"Later additions, which are neither\" section (:40-55), where the census document is registered"
  verifying_test: "Static field check over references/evaluation/ps-clause-pairing-sweep.md — no row verdicted defective has an empty or TBD exposing-implementation cell — plus the _review.md check against the bar at .kb/open-questions/ps-19-scope-narrower-than-its-rule.md:41-48"
- id: AC-004
  criterion: "**GIVEN** two open-question atoms that already assert a defect and could simply be believed, **WHEN** the author reads the PS-1 and PS-19 rows, **THEN** each verdict is derived independently from today's clause text and cited to it (`spec/SPECIFICATION.md:4744-4759`, `:5218-5249`), not copied from the atom; **AND** if either fails to reproduce, that non-reproduction is stated as the document's headline finding — the atoms are reported wrong rather than quietly agreed with."
  satisfied: false
  evidence: ""
  mount_point: "references/evaluation/README.md — the \"Later additions, which are neither\" section (:40-55), where the census document is registered"
  verifying_test: "_review.md derivation check — the PS-1 and PS-19 rows cite spec/SPECIFICATION.md:4744-4759 and :5218-5249 as their evidence, with the open-question atoms cited only as the priors under test and not paraphrased"
- id: AC-005
  criterion: "**GIVEN** that \"systematic\" and \"isolated\" send this project down different roads, **WHEN** the author reaches the verdict section, **THEN** the threshold separating the two — and the reasoning behind it — appears **before** the tally, the count is presented against it, and same-shape defects (evidence of a systematic §4.11 table-population assumption) are distinguished from different-shape ones (evidence of isolated incidents), with the similarity-40 PS-1/PS-19 pairing named as the hypothesis under test; **AND** if the verdict is systematic, the document states the re-plan trigger explicitly — which clauses, what a wider repair would have to decide — rather than widening ADR scope in place."
  satisfied: false
  evidence: ""
  mount_point: "references/evaluation/README.md — the \"Later additions, which are neither\" section (:40-55), where the census document is registered"
  verifying_test: "_review.md document-order check over references/evaluation/ps-clause-pairing-sweep.md — the threshold section literally precedes the tally section and carries the same-shape vs different-shape reasoning; a systematic verdict additionally requires the re-plan note in implementation-report.md"
- id: AC-006
  criterion: "**GIVEN** that `projection-decision-atoms` must scope three ADRs without re-reading 37 clauses, **WHEN** its implementer opens the sweep, **THEN** every `defective` row names which of ADR-0017 (batch ownership and write vocabulary, PS-4 – PS-15), ADR-0018 (reset and checkpoint scope, PS-16 – PS-20), ADR-0019 (apply-failure policy, PS-26 – PS-30) or a new decision would own it; **AND** any row whose repair would touch `ProjectionId` validation or a boundary-scoped checkpoint is **routed** to its existing open question with a statement that the sweep stopped there and why, never settled in passing."
  satisfied: false
  evidence: ""
  mount_point: "references/evaluation/README.md — the \"Later additions, which are neither\" section (:40-55), where the census document is registered"
  verifying_test: "Static owner-cell check over references/evaluation/ps-clause-pairing-sweep.md against .bklg/from-contract-to-published-library/projection-store-freeze/project.md:65-71 — every defective row carries an owner from ADR-0017 / ADR-0018 / ADR-0019 / new decision / routed, and every routed cell cites .kb/open-questions/projection-id-is-unvalidated.md or .kb/open-questions/global-versus-per-boundary-visibility-invariant.md; recorded in _review.md"
- id: AC-007
  criterion: "**GIVEN** that a document dropped into `references/evaluation/` without a README row is reachable only by `ls`, and that the README claims to enumerate the directory, **WHEN** anyone opens `references/evaluation/README.md`, **THEN** the sweep appears in \"Later additions, which are neither\" (`:40-55`) with its date, the commit it is pinned to, and the immutable *supersede-rather-than-edit* lifecycle stated — the same lifecycle that section already assigns to `review-citation-drift.md` — **AND** the document itself carries that date and pin in its own header."
  satisfied: false
  evidence: ""
  mount_point: "references/evaluation/README.md — the \"Later additions, which are neither\" section (:40-55), where the census document is registered"
  verifying_test: "Static registration check — rg for ps-clause-pairing-sweep in references/evaluation/README.md returns a hit inside the :40-55 section, the entry carries a date and a commit sha that git cat-file -e resolves, and the census document's own header repeats both"
- id: AC-008
  criterion: "**GIVEN** that `CLAUDE.md` forbids editing a `[FROZEN]` clause and forbids hand-writing `.kb/` atoms, and that this story's whole value is being a baseline the next two stories are judged against, **WHEN** a reviewer runs `git diff` over the merge, **THEN** not one normative byte has moved — zero changes under `spec/`, `crates/`, `xtask/`, `examples/`, and under `.kb/` outside `_intake/` — the KB-side amendments to the two open-question atoms and `.kb/maps/open-questions-index.md` exist only as a staged `.kb/_intake/` document for the next `/redkiln:kb-ingest` wave, and `cargo xtask spec-trace`, `redkiln validate --kb` and `redkiln doctor` are green with the same clause census and the same six `template-drift` advisories as on `main`."
  satisfied: false
  evidence: ""
  mount_point: "references/evaluation/README.md — the \"Later additions, which are neither\" section (:40-55), where the census document is registered"
  verifying_test: "Static diff and gate checks — git diff --stat main...HEAD over spec/, crates/, xtask/, examples/ is empty; git diff --stat over .kb/ excluding .kb/_intake/ is empty; git diff --diff-filter=D over .kb/open-questions/ is empty; then cargo xtask spec-trace (xtask/src/spec_trace.rs), redkiln validate --kb, redkiln doctor and cargo xtask ci all green"
```
