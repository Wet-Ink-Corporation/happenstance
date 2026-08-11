---
id: kb-reference-phase-4-5-spec-reconciliation-001
title: The phase 4/5 specification reconciliation — census and pointer
kind: reference
status: accepted
authority_tier: note
summary: >-
  What the unscheduled phase 4/5 specification reconciliation found, counted by class of defect:
  six commits (3c704d3 through 84dcc67 on redkiln-adoption, each green under cargo xtask ci)
  against spec/SPECIFICATION.md, and what cargo xtask spec-trace reports for the document on
  2026-08-10 — 200 clauses (139 FROZEN), 95 conformance rules, 358 citations checked of which 69
  are anchored to their subject. The evidence stays in references/evaluation/phase-4-5-reconciliation.md;
  this atom is the citable pointer, the counts, and the residual defects the pass left recorded.
depends_on: []
related:
  - kb-playbook-verify-referent-report-coverage-001
  - kb-playbook-anchoring-citations-001
  - kb-playbook-ratchet-gate-landing-001
  - kb-playbook-repair-frozen-clause-001
  - kb-open-question-disjoint-boundaries-no-clause-001
  - kb-open-question-model-family-rule-no-clause-001
  - kb-open-question-ps-1-no-progress-obligation-001
  - kb-open-question-ps-19-scope-narrower-001
  - kb-open-question-es-6-unwritable-rule-001
  - kb-open-question-provisional-falsifiers-001
  - kb-open-question-post-phase-reconciliation-001
source_paths:
  - .kb/_intake/2026-08-10-phase-4-5-pressure-test.md
  - .kb/_intake/lesson-repairing-a-frozen-clause-without-amending-it.md
  - .kb/_intake/lesson-anchoring-citations-in-a-long-lived-document.md
  - references/evaluation/phase-4-5-reconciliation.md
  - references/evaluation/review-citation-drift.md
  - RUNBOOK.md
  - spec/SPECIFICATION.md
  - xtask/src/spec_trace.rs
  - xtask/src/lint_constitution.rs
last_reviewed: 2026-08-10
---

# The phase 4/5 specification reconciliation — census and pointer

## What this is a pointer to

The evidence lives in `references/evaluation/phase-4-5-reconciliation.md`, written for the pass
and committed alongside it; `RUNBOOK.md`'s section "Between 5 and 6 — the reconciliation nothing
owned" carries the plan's own account. This atom is the census and the citable summary, not a
second copy — read it for counts and dates, read the evidence document for the finding-by-finding
detail.

## What the pass was

Six commits on branch `redkiln-adoption`, `3c704d3` through `84dcc67`, stacked on `3712c9b`,
each landing with `cargo xtask ci` green. It reconciled `spec/SPECIFICATION.md` with the tree
that phases 4 and 5 actually produced. No phase owned it; the runbook records it as "Not a
phase. A pass that had to happen and that this plan had not scheduled."

## Census, by class of defect

1. **Documentation obligations `[FROZEN]` clauses imposed and the code never met — nine.**
   `3c704d3`. Doc-comment content owed by ES-23, ES-24, ES-19, VT-15, VT-17, VT-3/ES-17 and
   ES-40. The same commit removed two false code comments (`memory.rs`, `ingest.rs`).
2. **Tests clauses name that the tree did not contain — five.** `e551cdf`. Each observed failing
   against a named wrong implementation before being written, per CLAUDE.md's rule that a rule
   no adapter can fail is decorative.
3. **Citations the checker never parsed — 254 of 338.** `a843b99`. The old filter required a
   citation to be path-qualified and name a `.rs` or `.toml` file; 84 of 338 satisfied both, 200
   were bare file names, 56 pointed into Markdown. The step had reported "no problems found"
   over a quarter of the corpus.
4. **Conformance rules claimed by no clause — six.** `52105d2`. Widening the sweep from
   `suite.rs` alone to all three `RULE_FILES` entries found them; four were attribution errors
   repaired in the same commit, two are genuine holes held in `UNCLAIMED_PENDING_ADR`.
5. **Clauses stating false things about the code — sixteen.** `89bb966`. Includes
   `AppendCondition`'s fields after VT-30, `ReadOptions`'s upper bound, and
   `SequencePosition::next`'s arithmetic after it became `checked_add`.
6. **Citations that resolve but point at the wrong thing — the content anchor.** `84dcc67`. Added
   the check that a citation is *about* the identifier the sentence attributes to it; it found
   two more defects immediately, both introduced by the sweep in `89bb966`.

## What the tool reports today

`cargo run -p xtask -- spec-trace`, against the working tree on 2026-08-10:

> 200 clauses (139 FROZEN, 49 PROVISIONAL, 10 DEFERRED, 2 NON-NORMATIVE), 95 conformance rules,
> 58 e2e cases, 358 citations checked (69 anchored to their subject, 12 external)
>
> 2 rule(s) claimed by no clause and owing a decision: …
>
> traceability: no problems found; §7.1–§7.2 matches the checker

358 citations are checked for addressing; only 69 carry a derivable subject and are anchored to
it. 81% of the corpus is verified for addressing only, and that ceiling is stated rather than
hidden.

## Independent corroboration

`references/evaluation/review-citation-drift.md` (dated 2026-08-10, pinned to `3712c9b`) was
written independently, as a byproduct of building `standards/rust/`. It found six stale
citations, diagnosed the identical root cause, and recommended porting `parse_citation` /
`check_citations` from `xtask/src/lint_constitution.rs`. Five of its six `SPECIFICATION.md`
citations, and its ADR-0009 citation, are discharged by this pass. **Two independent passes
converging on the same root cause and the same remedy is itself evidence the defect is
structural, not incidental.**

## What was not done, and the residual defect

The pass wrote no ADR. Seven findings needed one and were recorded rather than decided — six in
`gaps-owed-a-decision.md`, one in `open-question-nothing-owns-the-post-phase-reconciliation.md`
— each becoming its own open-question atom in this wave; two are additionally held in
`UNCLAIMED_PENDING_ADR` so the gate prints them on every green run.

One defect survived inside the repair itself: `xtask/src/spec_trace.rs`'s doc comment describes
its `ANCHOR_SLACK` as "the same twelve `standards/rust`'s own citation lint uses." Verified by
grep on 2026-08-10: `xtask/src/spec_trace.rs:391` sets `ANCHOR_SLACK = 12`;
`xtask/src/lint_constitution.rs:111` sets it to `10`. (The intake draft that fed this atom cited
line `:381`; the constant is at `:391`.) The values disagree and the comment says they do not —
a one-line repair, not an ADR.
