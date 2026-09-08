---
id: kb-open-question-cf-38-case-naming-no-clause-001
title: CF-38's fourth condition reads two ways and they differ by 54 cases
kind: open_question
status: accepted
authority_tier: note
summary: >-
  CF-38 [FROZEN] lists five conditions its checker must fail on; the fourth
  reads "a case naming no clause". cargo xtask spec-trace implements the
  reading "a case no clause claims" (check 10), reports zero orphans, and
  demonstrably catches the hazard it targets (deleting E2E-14 from SY-6's
  Cases: line now fails). The other reading — "a case body that names a
  clause", which is what CF-37's own Rejects: paragraph and MUST sentence
  say — is unmet by 54 of the 58 E2E-nn cases in spec/E2E-CASES.md, none of
  which carry a Clauses: field. The printed spec-trace summary line ("58 e2e
  cases (0 claimed by no clause)") now reads, to a hurried reader, as if
  CF-37 were discharged too, which is worse than the state before the check
  landed. Not decided: whether CF-38's fourth condition means the landed
  reading (leaving CF-37 unrepaired or superseded) or the stricter one
  (requiring 54 cases to acquire a field, or CF-37 to be rewritten). Forced
  by the next reader who cites a green spec-trace as evidence CF-37 is met,
  and by any future edit to E2E-CASES.md that could re-orphan a
  single-claim case without the fourth condition being able to say so.
depends_on: []
related:
  - kb-reference-spec-trace-has-suite-001
  - kb-playbook-repair-frozen-clause-001
  - kb-open-question-cf-36-unperformed-cross-reference-001
source_paths:
  - .kb/_intake/remediation-2026-09-04-briefs/which-reading-of-a-case-naming-no-clause.md
last_reviewed: 2026-09-07
---

# CF-38's fourth condition reads two ways and they differ by 54 cases

## What is true today

CF-38 (`[FROZEN]`) lists five conditions its checker must fail on. The
fourth is: "a case naming no clause". CF-37 (`[FROZEN]`) is the obligation
behind it: "Every E2E case MUST name the clause or clauses it exercises."

Measured against `spec/E2E-CASES.md`: 58 `### E2E-nn` headings, zero
`Clauses:` fields of any kind, and 54 of 58 case bodies containing no
clause identifier anywhere. CF-37's own `Rejects:` paragraph explains why:
`E2E-CASES.md` was written before the specification and therefore names no
clauses, and adding the back-reference is what was meant to make "is every
clause exercised by something concrete?" answerable by a command.

`cargo xtask spec-trace`'s check 10 implements the **inverse** direction:
every case in `E2E-CASES.md` must be claimed by at least one clause's
`Cases:` line. It reports zero orphans on every green run, and the
guarantee is real — demonstrated in a scratch worktree, deleting `E2E-14`
from SY-6's `Cases:` line (SY-6 is its only claimant, and §7.6 names E2E-14
as one of four single-claim cases whose loss would go unnoticed) now fails
the check by name, where before the check existed `spec-trace` reported no
problems and exited 0. §7.6 states this is exactly the hazard the check
exists to guard.

So the checker satisfies a real reading of CF-38's fourth condition, and
that reading is not the one CF-37's own sentence describes.

## The two readings

**Option A — "a case no clause claims."** This is what landed. Zero
failures today. It guards the single-claim-orphan hazard §7.6 names. It
does not discharge CF-37, which is about what a case *body* says about
itself, not about whether some clause elsewhere claims it.

**Option B — "a case body that names no clause."** 54 of 58 cases fail
this reading. Discharging it means either adding a `Clauses:` field to 54
cases — a real edit to a document whose whole point is predating the
specification — or superseding CF-37 so it no longer makes that claim.

**Option C — both**, with B tracked as a declared list of the 54 breaching
cases, the shape already used elsewhere in this corpus for a stated-only
defect. Cost: a 54-entry table whose only content is "this case predates
the specification," written once per case rather than once.

## Where this sits

Option A is landed and green; it is not proposed as a change, only as one
of two candidate readings of a clause whose text supports the other. The
question is which reading CF-38's fourth condition is understood to carry,
and, downstream of that, whether CF-37 needs repair (via
`kb-playbook-repair-frozen-clause-001`) or a superseding clause. Both
routes are decisions with an ADR-shaped record, not a line edit — CF-37 is
`[FROZEN]`.

The cost of leaving this open is not zero, unlike most orphaned-check
findings: the printed spec-trace summary already reads as evidence CF-37
is satisfied, to any reader who has not read the checker's source. That
reading gets slightly more entrenched every green run that goes by
unaddressed.

## What this does not settle

Whether `E2E-CASES.md` acquires a `Clauses:` field on 54 cases, or whether
the four single-claim cases §7.6 names should instead be given a second
claimant so the hazard becomes structural rather than checked. Both are
downstream of picking a reading, not resolved by this atom.
