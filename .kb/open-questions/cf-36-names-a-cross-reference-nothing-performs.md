---
id: kb-open-question-cf-36-unperformed-cross-reference-001
title: CF-36 names a cross-reference cargo xtask spec-trace does not perform
kind: open_question
status: accepted
authority_tier: note
summary: >-
  CF-36 is FROZEN and its Rule line claims cargo xtask spec-trace
  cross-references each case's level marker. It does not: grep -c "Level"
  over xtask/src/spec_trace.rs returns zero, and no other check performs the
  comparison under another name. The consequence is worse than an unwritten
  check, because the gate is green and reads as evidence: a passing
  spec-trace is currently taken as confirmation of a clause whose stated
  mechanism does not exist, which is the same defect shape a stale exemption
  carries — nothing fails when the check is absent, because the absent
  thing's job is to make things fail. What is not decided is which of two
  routes CF-36 takes: implementing the cross-reference in spec_trace.rs so
  the clause becomes true, or superseding CF-36 so it stops claiming an
  instrument nobody built. Both are decisions with alternatives and a
  record; neither is a patch, and the repository's own method for the
  second is kb-playbook-repair-frozen-clause-001. Forced by the next reader
  who cites a green spec-trace as evidence for a level marker, and by phase
  12, where first publish makes the specification a promise rather than a
  working note.
depends_on: []
related:
  - kb-reference-spec-trace-has-suite-001
  - kb-playbook-repair-frozen-clause-001
  - kb-open-question-es-6-unwritable-rule-001
  - kb-open-question-no-ps-rule-name-resolved-001
source_paths:
  - .kb/_intake/contract-defect-log-phase-7.md
  - references/evaluation/phase-7-contract-defects.md
  - xtask/src/spec_trace.rs
  - spec/SPECIFICATION.md
last_reviewed: 2026-08-17
---

# CF-36 names a cross-reference cargo xtask spec-trace does not perform

## What is true today

CF-36 (`spec/SPECIFICATION.md:8611-8622`) is `[FROZEN]`. Its `Rule:` line
claims that `cargo xtask spec-trace` cross-references each case's level
marker — the per-case maturity annotation (frozen, provisional, deferred,
non-normative) that the rest of the specification relies on `spec-trace`
being a gate step precisely to keep honest. It does not perform that
cross-reference. `grep -c "Level" xtask/src/spec_trace.rs` returns `0`:
nothing in the file's source contains the string at all, let alone a
comparison against it. Nor does any other check in `spec_trace.rs` perform
the same comparison under a different name — this is not a naming mismatch,
it is an absent mechanism.

The consequence is not merely an unwritten check; it is actively
misleading. `cargo xtask spec-trace` is a required CI step, and it passes.
A passing `spec-trace` therefore currently reads, to anyone who has not read
its source, as confirmation that level markers are cross-referenced and
correct — which is exactly what CF-36's own text promises. Nothing fails
when the comparison is absent, because the absent thing's entire job would
have been to make something fail. This is the same defect shape a stale
exemption carries: silence is read as a clean bill of health rather than as
the absence of a test.

## What is not decided

Which of two routes CF-36 takes. **Implement**: add the cross-reference to
`spec_trace.rs` so the clause's `Rule:` line becomes literally true, at the
cost of writing and maintaining a new check against 200 numbered clauses'
level markers. **Supersede**: write a new clause (or demote CF-36's Rule
line to non-normative prose) so the specification stops claiming an
instrument that was never built, at the cost of admitting the intended
cross-check does not exist and may not be worth building.

Both are decisions with alternatives, a cost, and a record — not a line
edit to either the clause or the source file. The repository's own method
for touching a `[FROZEN]` clause without silently rewriting it is
`kb-playbook-repair-frozen-clause-001`, and either route runs through it:
implementing the check is a repair that keeps the clause's text and closes
the gap under it; superseding is a repair that changes the clause's claim
outright.

## What forces it

Two events. The first is any reader who cites a green `spec-trace` run as
evidence that level markers are correct — the citation is currently false,
and nothing in the repository currently prevents it. The second is phase
12, where first publish turns `spec/SPECIFICATION.md` from a working note
into a promise; a promised clause whose named mechanism does not exist is a
liability a pre-publish repository can carry and a published one cannot.

## Ordered sub-questions

1. Is a level-marker cross-reference worth building at all, given
   `spec-trace` already performs other cross-reference checks (rule-name
   resolution via `has_suite`, among them) — or would implementing it
   duplicate a guarantee the maturity markers already provide by convention?
2. If implemented, does it belong in `spec_trace.rs`'s existing check
   sequence (a new numbered check) or as a separate gate step, given
   `cargo xtask ci`'s existing step list already runs `spec-trace` once?
3. If superseded instead, does CF-36's `Rule:` line get rewritten to
   describe what `spec-trace` actually does, or does the clause move to
   non-normative prose entirely — the two outcomes differ in whether a
   *future* implementer is still invited to build the cross-reference.
