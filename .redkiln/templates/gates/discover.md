# Gate: Discover

REQUIRED checklist to leave the discover stage. Every box must be ticked in the
stage's produced artifact (`discover.md`) before `redkiln advance` will pass.

**Each box below is one line, deliberately** — see `gates/intake.md` for why a
wrapped box can never be matched.

- [ ] The problem is framed in one paragraph.
- [ ] Prior art and constraints are recorded in the signal ledger.
- [ ] Open questions are either answered or explicitly deferred.
- [ ] The next stage (spec) has a clear starting point.
- [ ] The wrong implementation this work rejects is named.
- [ ] No conformance rule added here asserts a literal position value.
- [ ] Any `[FROZEN]` clause this touches is changed by a new ADR, written first.
- [ ] If a rule here seems wrong, it is fixed and the reason given in the same change.

## Why the last four exist

**The wrong implementation.** For a conformance rule this is literal: a rule with
no mutant that fails it is decorative, and the mutant belongs in the testkit's own
`tests/`. For everything else it is the same question one level up — what could
someone build that satisfies every existing check and is still wrong?

**Literal position values.** The specification permits gaps (VT-11), so an
assertion on `[1, 2, 3]` converts a `MAY` into a `MUST` with no ADR behind it
(CF-6). Compare against the positions the store actually assigned.

**A new ADR, written first.** Before the code it constrains, not alongside it: a
rule written after the signature it protects is written by someone who already
believes the signature is right.

**Fix the rule, do not skip it.** Skipping is not available. If a rule seems
wrong, fix it and give the reason in the same change.
