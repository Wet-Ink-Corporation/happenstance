---
id: kb-playbook-verify-referent-report-coverage-001
title: A cross-reference checker verifies the referent, and reports its own coverage
kind: playbook
status: accepted
authority_tier: guideline
summary: >-
  Two independent obligations for any checker over cross-references — citations, links, ids,
  schema $refs, test-to-requirement traceability. Verify the referent and not merely the address:
  that a file:line resolves says nothing about whether the attributed content is there. And report
  coverage: a check that does not state what fraction of the corpus it parsed is indistinguishable
  from one that sees all of it. Grounded in a parser that checked 84 of 338 citations while
  printing "no problems found", and in the one-line fix that made the claim falsifiable.
depends_on: []
related:
  - kb-reference-phase-4-5-spec-reconciliation-001
  - kb-playbook-anchoring-citations-001
  - kb-playbook-declared-page-need-001
  - kb-playbook-assert-execution-not-discovery-001
  - kb-governance-what-may-refute-a-finding-001
source_paths:
  - .kb/_intake/lesson-a-check-that-verifies-the-address-not-the-referent.md
  - xtask/src/spec_trace.rs
  - xtask/src/lint_constitution.rs
  - spec/SPECIFICATION.md
  - references/evaluation/phase-4-5-reconciliation.md
last_reviewed: 2026-09-02
---

# A cross-reference checker verifies the referent, and reports its own coverage

## The claim

A checker over cross-references has two independent obligations. Satisfying the first while
silently failing the second produces a green step that means nothing:

1. **Verify the referent, not the address.** That a `file:line` resolves — the file exists, the
   line is in bounds — says nothing about whether the thing the sentence attributes to that
   location is actually there.
2. **Report your own coverage.** A check that does not state what fraction of the corpus it
   parsed is **indistinguishable from one that sees all of it**, and a reader has no way to tell
   the two apart from outside the code.

The second is the sharper failure, because a checker with a narrow filter degrades in exactly the
way that produces false confidence: it never reports a false positive, it passes on every run,
and nobody has cause to look at it again.

## The measurement

`xtask/src/spec_trace.rs`'s citation parser filtered on two conditions before this pass: a
citation had to be path-qualified, and it had to name a `.rs` or `.toml` file. Against
`spec/SPECIFICATION.md` as it then stood — 338 citations total — only 84 satisfied both
conditions and were checked; 200 were bare file names and 56 pointed into Markdown, and neither
form was ever parsed. `check_citations` was not a weak check over the corpus; it was a correct
check over a quarter of it, and it printed "no problems found" the entire time. This was not
hypothetical: an earlier commit had already found eight citations "green and wrong" and repaired
them by hand, inside the quarter anyone could see.

## The fix, and why the smaller half is the durable one

Widening the parser to resolve bare names through a workspace index took coverage from 84 parsed
to 358 checked — a real fix, but not the transferable one. The transferable change is one line in
the summary output: the coverage figure itself, `358 citations checked (69 anchored to their
subject, 12 external)`. The code states why: a check that does not state its own coverage is
indistinguishable from one that sees everything, which is exactly how this step reported success
while parsing a quarter of the corpus. **A reader who is told a number can falsify it. A reader
who is told "no problems found" cannot.**

## Generalisation

For any checker over cross-references — citations, links, ids, symbol references, schema
`$ref`s, test-to-requirement traceability:

- **Print the denominator.** Candidates found, candidates parsed, candidates actually verified —
  three numbers, because "checked" and "verified against its referent" are different
  populations, and collapsing them restores the original defect at a higher coverage level.
- **State the population you declined.** An exemption that is counted and named is a decision; an
  exemption that is a silent filter is a blind spot.
- **A narrowing filter is a silent scope reduction.** Every `continue` in a parser is an
  unverified population. If you cannot name what it skips, you do not know what the check covers.
- **Suspect a check that has never failed.** Absence of findings over a long period is evidence
  either the corpus is clean or the check is not looking; only the coverage number distinguishes
  them.

## The counter-consideration, stated honestly

Coverage reporting does not make a check correct — it makes its scope legible. All 358 citations
are now checked for addressing, but only 69 are checked for referent, because a derivable subject
exists for only that many. **81% of the corpus is still verified for addressing only.** The
summary line says so on purpose: a summary that reported "358 citations verified" and stopped
would have re-created the original problem one level up. See
`kb-reference-phase-4-5-spec-reconciliation-001` for the dated figures this playbook is grounded
in.
