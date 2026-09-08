---
id: kb-open-question-cf-36-unperformed-cross-reference-001
title: CF-36's cross-reference now exists, and the thirteen breaches it found take three different repairs
kind: open_question
status: accepted
authority_tier: note
summary: >-
  CF-36 is FROZEN and its Rule line claims cargo xtask spec-trace
  cross-references each case's level marker. Until the 2026-09-04 remediation
  pass it did not — grep -c "Level" over xtask/src/spec_trace.rs returned zero
  against 58 markers — and this atom's original question was which of two routes
  the clause took: implement the cross-reference, or supersede the clause so it
  stops claiming an instrument nobody built. The implement route was taken.
  check_case_levels reads every case's marker against a closed level vocabulary,
  treats a missing marker and an unknown word as failures rather than silent
  passes, and takes the first word so that contract (sync) counts as contract.
  Its first run found thirteen clauses in breach, recorded in CF36_UNDISCHARGED
  and reconciled in both directions so the list can only shrink. What is open is
  no longer whether the instrument exists but which of CF-36's three repairs each
  of three groups takes — and the live sub-question under the largest group is
  whether a rule named for a named-but-unbuilt suite is a rule CF-36 forbids at
  all. Every repair is an edit to spec/SPECIFICATION.md, which the implementing
  lane was not permitted to make.
depends_on: []
related:
  - kb-reference-spec-trace-has-suite-001
  - kb-playbook-repair-frozen-clause-001
  - kb-open-question-es-6-unwritable-rule-001
  - kb-open-question-no-ps-rule-name-resolved-001
  - kb-open-question-cf-25-cf-26-portfolio-check-001
  - kb-reference-spec-trace-unresolved-declarations-001
  - kb-open-question-cf-38-case-naming-no-clause-001
  - kb-decision-0045
  - kb-playbook-anchoring-citations-001
  - kb-playbook-ratchet-gate-landing-001
source_paths:
  - .kb/_intake/contract-defect-log-phase-7.md
  - .kb/_intake/remediation-2026-09-04-briefs/cf-36-thirteen-recorded-breaches.md
  - .kb/_intake/remediation-2026-09-04-briefs/stated-only-defects-and-the-reopen-must.md
  - references/evaluation/phase-7-contract-defects.md
  - xtask/src/spec_trace.rs
  - spec/SPECIFICATION.md
last_reviewed: 2026-09-07
---

# CF-36's cross-reference now exists, and the thirteen breaches it found take three different repairs

## What changed

CF-36 (`spec/SPECIFICATION.md:8980-8991`) is `[FROZEN]` and its `Rule:` line
claims `cargo xtask spec-trace` cross-references each case's level marker
(`E2E-CASES.md:19-28`). It did not: at `9b06836`, `grep -c "Level"
xtask/src/spec_trace.rs` returned `0` against 58 `- **Level:**` markers, so a
required, green gate step read as confirmation of a clause whose named mechanism
was absent — nothing failed, because the absent thing's job was to make things
fail.

Of the two routes this atom named — implement, or supersede — **implement was
taken**. `check_case_levels` in `spec_trace.rs` is check 11: it collects each
case's first `**Level:**` bullet, fails on a case with no marker (the
cross-reference would be vacuous rather than wrong), fails on a level word
outside the closed `KNOWN_LEVELS` list rather than letting a typo'd
`intergration` fall through to the contract-level branch, and takes the marker's
**first word**, because eight of the 58 read `contract (sync)` and the qualifier
is not the level. That last decision was reached by trying to refute the check
rather than by reading the document, and it matters here: it is what puts those
eight cases on the contract side.

## The thirteen, in three groups

The first run found thirteen clauses naming a conformance rule with no
contract-level case: VT-21, WF-9, PS-29, PS-30, SY-2, SY-4, SY-5, SY-7, SY-9,
SY-10, SY-24, SY-25, SY-34.

- **Nine are one fact.** The `SY` family names rules that would live in
  `happenstance-sync-testkit` — the crate CF-36's own `Rejects` paragraph names
  as their home, at `crates/happenstance-sync/src/lib.rs:23-24`, and which does
  not exist. That is one question asked nine times.
- **Two cite the wrong case.** VT-21 and WF-9 name *live* rules in `suite.rs`
  (`store_accepts_the_guaranteed_minimum_payload`,
  `append_reports_exceeded_store_limits`) against E2E-42 alone, which is
  transitive convergence across a peer mesh. Single-store rules, a case that is
  not; the mismatch reads as a `Cases:` line rather than a misplaced rule.
- **Two wait on the projection runner.** PS-29 and PS-30 cite E2E-28 and name
  rules that are unwritten and declared `Scheduled`
  (`kb-reference-spec-trace-unresolved-declarations-001`). Whether either can be
  expressed against a single projection-store handle is a question the runner
  answers.

## What is not decided

1. **Is a rule named for a named-but-unbuilt suite a rule CF-36 forbids?** If it
   is not, the nine take one superseding qualification rather than nine edits,
   and §7.2's `†` convention is the precedent. The argument against is that the
   qualification is exactly the escape hatch that lets any clause name any rule
   by promising a crate.
2. **Do the two `Cases:` lines get repaired now?** They are cheaper than the rest
   and should not wait for first publish: a peer-mesh case cited for single-store
   rules is a factual error rendered on docs.rs. It is still an edit inside a
   `[FROZEN]` clause and runs through `kb-playbook-repair-frozen-clause-001`.
3. **Are the eight `contract (sync)` cases contract-level for CF-36's purposes?**
   The check's first-word reading says yes. If they are not, group 1 grows.

## Why the record is a table and not a skip list

`CF36_UNDISCHARGED` is reconciled in both directions on every run: a clause that
stops breaching fails the gate as a stale entry, a recorded clause the document
stops declaring fails the same way, and a fourteenth breach fails because it is
not recorded. The count is printed on every green run beside
`UNCLAIMED_PENDING_ADR`'s, because a cost that only appears when something is
already broken is a cost nobody prices. The alternative — leaving CF-36
unimplemented rather than landing a check with thirteen recorded breaches — is
wrong in one specific way: unimplemented, the fourteenth is as invisible as these
thirteen were.

## What forces it

Unchanged: phase 12, where first publish turns `spec/SPECIFICATION.md` from a
working note into a promise, and a promised clause whose repairs are all still
owed is a liability a pre-publish repository can carry and a published one
cannot. Group 2 is cheaper than that deadline and named above.

## The citation this pass repointed, and the one it refused

This atom cited `SPECIFICATION.md:8611-8622`, correct when written (CF-36 began
at 8611 at `2abb99f`) and stale since. The
`stated-only-defects-and-the-reopen-must` brief offered `8648`. **That repoint
was not applied.** Its anchor sentence — *"is recorded as what is still missing
rather than as what was always meant"* — belongs to §6.5's portfolio-axis prose,
not to CF-36, and at the brief's own measurement commit that sentence sat at
8696 while CF-36 sat at 8903. Taking the offered number would have moved the
citation further off the clause it names. `8980-8991` above is CF-36's block,
verified by grep in the tree today. Both briefs behind this atom were written by
the lane implementing the change and had no two-critic pass; this is what that
discount buys, and it is the case `kb-decision-0045` and
`kb-playbook-anchoring-citations-001` exist for.
