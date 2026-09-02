---
item: HS-B0001
stage: discover
created: 2026-08-19T14:51:00.896Z
updated: 2026-08-19T14:51:00.896Z
template_sig: 86ce4036
rendered_sig: 6dac95ce
---

# Discover — The crate-root fence and headings exceed the documentation density budget

## Signal Ledger

Evidence gathered: prior art, constraints, and the problem framing.

| Signal | Source | Implication |
| ------ | ------ | ----------- |
|        |        |             |

## Questions

Open questions to resolve before specifying.

## Decision

What we now believe is true and what we will spec next.

## The wrong implementation

What could someone build that satisfies every existing check and is still wrong?
For a conformance rule this is literal — name the mutant, and put it in the
testkit's own `tests/` if it is not already there.

## Gate: Discover

Tick each box once the sections above satisfy it. `redkiln advance` reads these
against `.redkiln/templates/gates/discover.md` — where each box's rationale is
written — and will not leave `discover` until every one is ticked. Keep each box on
one line; the parser matches line by line.

- [ ] The problem is framed in one paragraph.
- [ ] Prior art and constraints are recorded in the signal ledger.
- [ ] Open questions are either answered or explicitly deferred.
- [ ] The next stage (spec) has a clear starting point.
- [ ] The wrong implementation this work rejects is named.
- [ ] No conformance rule added here asserts a literal position value.
- [ ] Any `[FROZEN]` clause this touches is changed by a new ADR, written first.
- [ ] If a rule here seems wrong, it is fixed and the reason given in the same change.
