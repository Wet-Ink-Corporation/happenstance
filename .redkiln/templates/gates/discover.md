# Gate: Discover

REQUIRED checklist to leave the discover stage. Every box must be ticked in the
stage's produced artifact (`discover.md`) before `redkiln advance` will pass.

- [ ] The problem is framed in one paragraph.
- [ ] Prior art and constraints are recorded in the signal ledger.
- [ ] Open questions are either answered or explicitly deferred.
- [ ] The next stage (spec) has a clear starting point.

## This repository's additions

- [ ] **The wrong implementation this work rejects is named.** For a conformance
      rule that is literal: a rule with no mutant that fails it is decorative, and
      the mutant belongs in the testkit's own `tests/`. For everything else it is
      the same question one level up — what could someone build that satisfies
      every existing check and is still wrong?
- [ ] **No conformance rule added here asserts a literal position value.** The
      specification permits gaps (VT-11), so an assertion on `[1, 2, 3]` converts
      a `MAY` into a `MUST` with no ADR behind it (CF-6). Compare against the
      positions the store actually assigned.
- [ ] **Any `[FROZEN]` clause this touches is changed by a new ADR**, and the ADR
      is written before the code it constrains — not alongside it, and not after.
- [ ] **If a rule here seems wrong, it is fixed and the reason is given in the
      same change.** Skipping it is not available.
