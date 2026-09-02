---
id: kb-playbook-ratchet-gate-landing-001
title: Landing a stricter gate check when the corpus cannot pass it yet
kind: playbook
status: accepted
authority_tier: guideline
summary: >-
  What to do when tightening a gate check surfaces violations you are not authorised to fix.
  Waiting for a clean corpus lands the check after the decision it should have pressured; landing
  it report-only removes exactly that pressure. The third option is a ratchet: fatal from day one,
  with a named exemption list that prints every entry on every green run, fails when an entry
  becomes discharged so the list can only shrink, and carries the owed decision as an argument
  rather than a name. Includes why the tool change and the corpus change had to be one commit,
  why a list never carries a count beside it, and the corpus shape where a ratchet is the wrong
  instrument.
depends_on: []
related:
  - kb-reference-phase-4-5-spec-reconciliation-001
  - kb-playbook-repair-frozen-clause-001
  - kb-open-question-disjoint-boundaries-no-clause-001
  - kb-open-question-model-family-rule-no-clause-001
  - kb-open-question-post-phase-reconciliation-001
  - kb-reference-spec-trace-has-suite-001
  - kb-reference-phase-8-spec-reconciliation-001
  - kb-playbook-declared-page-need-001
source_paths:
  - .kb/_intake/lesson-landing-a-stricter-gate-without-a-red-baseline.md
  - xtask/src/spec_trace.rs
  - spec/SPECIFICATION.md
  - RUNBOOK.md
  - references/evaluation/phase-4-5-reconciliation.md
last_reviewed: 2026-09-02
---

# Landing a stricter gate check when the corpus cannot pass it yet

## The situation

You tighten a gate check and the existing corpus fails it in ways you are not authorised to fix.
Here: widening a specification-tracing check to sweep every rule file, not just one, surfaced six
conformance rules that no clause claimed. Four were attribution errors and were repaired in the
same commit. Two were not errors — they were holes in the specification, and closing either would
mean asserting that a `[FROZEN]` clause contains a proposition it does not contain. That is an
ADR's decision, not a checker's, so the check could not go green by repair, and the pass was not
permitted to make it go green by decision.

## The three options

**A — wait until the corpus is clean.** Refused: the clean-up needs an ADR pass that has not been
scheduled, and a check that waits for a decision lands after the decision has already been made
without it.

**B — land it report-only, promote to fatal later.** Refused: a warning inside a green run is
invisible within a week, the promotion never gets scheduled, and worse, report-only removes
exactly the pressure the check exists to create.

**C — a ratchet.** Land the check **fatal from day one**, with a named, evidenced,
self-invalidating exemption list holding exactly the entries that cannot be resolved without a
decision.

## The four properties that make it a ratchet, not an allowlist

1. **Fatal from day one.** The default arm is a hard failure naming three ways out: a clause
   claims the rule, a clause retires it with a stated reason, or it goes in the exemption list
   with the decision it is waiting on. No warning tier exists, so nothing accumulates below the
   failure threshold.
2. **Every entry prints on every green run, not only on failure.** An open question that only
   surfaces when something else is already broken is an open question nobody reads. Printing the
   full argument on every pass is deliberate friction, applied continuously to whoever runs the
   gate rather than episodically to whoever remembers the list exists.
3. **An entry that becomes discharged is itself a failure.** If a rule is both claimed by a clause
   and still listed as pending, the check fails and says so. Without this half, a stale exemption
   is free and the list only grows; with it, the list can move in exactly one direction.
4. **Each entry carries its evidence and its owed decision, not just a name.** What the rule
   enforces, which clause looks like it should claim it, why claiming it there would be false, and
   what an ADR would have to do. A name-only entry is a mute suppression; an argued one is a work
   item whose analysis is already done.

## Why the list carries no count

The exemption array's type fixes its length, but neither the doc comment nor the printed output
hard-codes that number — the count is computed at the call site, following a precedent set
earlier in this repository's history: *never write a count of a list beside the list*, because a
self-referential number in a comment is a citation to the thing it sits on, and it rots the same
way every other citation rots. Either compute it or omit it.

## Why the tool change and the corpus change had to be one commit

Widening the check and repairing the four attribution errors landed in a single commit, which is
normally poor practice — a tool change and a content change have different review needs — and
here was forced: widening without repair leaves the gate red at that commit, so it is not
independently buildable and `git bisect` runs into a wall; repairing without widening makes the
repairs unverifiable at that commit, since nothing yet checks the files they live in. **When a
check and the corpus it checks change together, they must land together if and only if either
half alone would leave the tree unverifiable.** A green gate at every commit is the only evidence
that the widening found exactly the right number of rules. The corollary: the exemption entries
had to land in that same commit too — a ratchet introduced one commit after the check it exempts
from is a ratchet with a red commit in front of it, and the first thing anyone does with a red
commit is weaken the check.

## Where this is the wrong instrument

A ratchet suits a small, bounded, individually-argued set of exceptions — single digits, each
owed to a specific pending decision. It does not suit a legacy corpus with hundreds of
violations: there, the per-entry argument goes unwritten, printing every entry on every run is
noise instead of pressure, and the honest instrument is a decreasing threshold count with a
deadline instead. The test: can you write, for each entry, the sentence "this is not a defect; it
is waiting on `<named decision>`" — and mean it? If not, it is a defect and the list is an
allowlist.

The two exemption entries this mechanism holds today are each an owed decision recorded as its
own open question in this same intake wave; the mechanism lives here, the decisions live there,
and neither absorbs the other.
