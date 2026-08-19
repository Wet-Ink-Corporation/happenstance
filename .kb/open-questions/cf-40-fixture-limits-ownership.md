---
id: kb-open-question-cf-40-ownership-001
title: CF-40's ownership is claimed and disclaimed by the same decision
kind: open_question
status: accepted
authority_tier: note
summary: >-
  CF-40 is the clause requiring a fixture to declare its actual numeric limits — maximum event data length, tags per event, events per batch — as optional constants defaulting to no ceiling, reported through the declined-capability path. ADR-0015 mints it and contradicts itself about whether it owns it: the header and introduction say CF-40 was settled at sign-off and 'lands here', while the Consequences section for the same decision says the fixture-constant ownership question 'is not settled' and that decision 8 'sets out both and declines to choose'. The other claimant is ADR-0012, which already owns CF-39 and MID_BATCH_FAULT in the same fixture-contract area, and ADR-0015's own reasoning is that neither decision may resolve this unilaterally. Both decisions are accepted and neither is edited, so the contradiction stands as imported. What is not decided is which document the fixture's capability surface belongs to as a whole — the question is less about CF-40 than about whether the fixture contract has one owner or is amended by whichever decision needs it next. Forced by phase 8, the first adapter with real limits.
depends_on: []
related:
  - kb-decision-0015
  - kb-decision-0012
  - kb-decision-0010
  - kb-decision-0022
source_paths:
  - .kb/_intake/0015-validated-identifiers-and-store-limits.md
  - .kb/_intake/0012-append-shape-and-preconditions.md
  - references/adr/0015-validated-identifiers-and-store-limits.md
  - references/adr/0012-append-shape-and-preconditions.md
  - crates/happenstance-testkit/src/fixtures.rs
  - spec/SPECIFICATION.md
last_reviewed: 2026-08-10
---

# CF-40's ownership is claimed and disclaimed by the same decision

## What is true today

`append_reports_exceeded_store_limits` — the rule that proves an adapter
actually returns `AppendError::ExceedsStoreLimit` at its stated ceiling — is
unwritable against a fixture with no way to declare a ceiling in the first
place. `Fixture` declares exactly three `Capability` constants
(`SECOND_HANDLE`, `REOPEN`, `MID_BATCH_FAULT`), and none of them can carry a
number. ADR-0015 names this gap in its own decision 8 and, at the very end of
that decision, mints **CF-40**: a `Fixture` MUST be able to declare
`MAX_EVENT_DATA_LEN`, `MAX_TAGS_PER_EVENT` and `MAX_EVENTS_PER_BATCH`, each
`Option<usize>` defaulting to `None`, reported through the same
declined-capability path a boolean `Capability` uses.

The document contradicts itself about whether *it* is the one deciding this.
The frontmatter's "Adds" line and the decision-8 prose both say the clause
lands here — "**Settled at sign-off: it lands here, as CF-40**" — with an
explicit argument for why the competing home, ADR-0012, is wrong: "What
`SPECIFICATION.md:6794-6804` assigns is the **`MID_BATCH_FAULT`** clause, it
assigns it to phase 4 by name, and it names no ADR. ADR-0012 has taken that
slot as **CF-39**, and CF-39 is about a *boolean* capability... Nothing...
says a fixture may declare a **number**. Parking this blocker on ADR-0012
would therefore park it on an ADR that has not accepted it."

But the same ADR's own Consequences section, describing the identical
decision, says the opposite: "What is *not* settled, and must be settled
before the rule is written, is which ADR's clause set the fixture constants
land in: this one, because the rule serves VT-25 and VT-19, or ADR-0012's,
because it is already amending the fixture contract at CF-39. **Decision 8
sets out both and declines to choose.**"

ADR-0012 is the other claimant by adjacency rather than by counter-assertion:
it already owns `CF-39` and `MID_BATCH_FAULT` in the same
fixture-contract neighbourhood, in the same specification section, and
nothing in its own text disclaims CF-40 or defers to ADR-0015 explicitly —
the competing claim comes entirely from ADR-0015's own hedge.

## What is not decided

Whether CF-40 is ADR-0015's clause, full stop — which is what its own header
and decision-8 prose assert — or an open placement question its Consequences
section says was never actually settled. Both readings are present, accepted,
and unedited (this wave has no standing to prefer one, per the authority
rules governing accepted decisions). The deeper question the contradiction
points at is less about CF-40 specifically than about whether the fixture's
capability surface — `CF-16` through `CF-40` and whatever comes after — has
one owning document or is legitimately amended piecemeal by whichever ADR
needs the next capability, with the numbering spread across `§6` "by subject"
as ADR-0015 itself puts it.

## What forces it

Phase 8, the first adapter (`happenstance-sqlite`) with real, enforceable
numeric limits to declare and a rule (`append_reports_exceeded_store_limits`)
that needs somewhere authoritative to point when a future ADR amends the
fixture contract again. Until then the contradiction costs nothing — the
clause's text is identical regardless of which document a reader believes
owns it — but the next fixture-contract change (a `POLL_BUDGET`-shaped
capability, an ADR-0013 §8 concern already flagged as deferred to phase 10)
will hit the same unresolved question of where fixture-contract clauses get
minted.

## Ordered sub-questions

1. Does a future ADR (or the phase 4/5 reconciliation pass) explicitly
   confirm CF-40 as ADR-0015's, superseding the hedge in its own Consequences
   section, or does it move the clause to ADR-0012 to sit beside CF-39?
2. Independent of CF-40 specifically: should the fixture contract (`CF-`
   clauses) get a single named owner going forward, the way `EventStore` and
   `ProjectionStore` each have one, rather than being amended by whichever
   ADR happens to need the next capability?
3. When phase 10's `POLL_BUDGET`-shaped question (ADR-0013 §8) or any other
   new fixture capability arrives, does it default to the same
   spread-by-subject pattern this question exposes, or does the answer to
   sub-question 2 change that by then?
