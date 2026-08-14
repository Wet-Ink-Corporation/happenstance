---
id: kb-playbook-one-decision-per-adr-title-001
title: An "and" in a decision's title is usually a second, weaker decision
kind: playbook
status: accepted
authority_tier: guideline
summary: >-
  Three times in two days, a title joining two claims with 'and' turned out to be carrying a
  strong decision and a weak one, and the weak one was reversed within days while the strong one
  stood. ADR-0005 bundled a rename with a crate allocation; ADR-0006 bundled the corrected
  allocation with the projection runner's home; ADR-0007 named the pattern outright. The
  discriminator is not the conjunction but what each half rests on: the halves that stood were
  settled by being made, and the halves that fell were claims about code that did not exist yet -
  a decision about where code lives, taken before that code exists, is a guess. So when a title
  needs an 'and', either split the document, or mark the weaker half provisional with the
  observation that would refute it and the phase that would produce it. That is what the corpus
  did each time, and it is why each reversal cost a paragraph rather than an argument. It stops
  being worth the split when both halves are settled by the same evidence, in which case the
  conjunction is describing one decision with two consequences.
depends_on: []
related:
  - kb-decision-0005
  - kb-decision-0006
  - kb-decision-0007
  - kb-decision-0017
  - kb-decision-0018
  - kb-governance-referent-not-reasoning-001
source_paths:
  - .kb/_intake/0005-rename-to-happenstance.md
  - .kb/_intake/0006-bare-name-to-the-typed-layer.md
  - .kb/_intake/0007-projection-runner-decodes.md
  - references/adr/0007-projection-runner-decodes.md
last_reviewed: 2026-08-13
---

# An "and" in a decision's title is usually a second, weaker decision

## The pattern

Three ADRs in two days shared a shape: a title joining two claims with "and", where one half
held and the other reversed within days. ADR-0005 — rename the project to `happenstance`, **and**
make it the contract crate — settled the rename permanently and got the crate allocation wrong.
ADR-0006 corrected the allocation, and bundled in a second claim under the same "and": that the
projection runner "never decodes a payload" and therefore belongs beside the ports in the
contract crate. ADR-0007 named the pattern outright, in its own text, as "the third time an
'and' in an ADR title has concealed a second, weaker decision" — and corrected the runner claim
in turn.

## The discriminator

Not the conjunction itself — a title can join two claims that are genuinely one decision with two
consequences. The discriminator is what each half rests on. The halves that stood in all three
cases were settled by being made: a project rename is true the moment it is chosen, and there is
nothing further to observe that would unmake it. The halves that fell were claims about code
that did not yet exist: ADR-0005's crate allocation was argued from an empirical claim about "the
behaviour of users who do not exist, asserted without evidence"; ADR-0006's runner claim was
asserted before any runner had been built against `ProjectionStore`, and testing it against a
concrete consumer — an application projecting into two different stores — showed the claim
false. A decision about where code lives, taken before that code exists, is a guess wearing the
form of a decision.

## What to do about it

When a title needs an "and", either split the document into two ADRs, or mark the weaker half
**provisional**, stating the falsifier that would refute it and the phase expected to produce the
code that tests it. That is what happened after the fact each time in this corpus: ADR-0006
recorded its runner allocation as resting on the same unfounded kind of claim that made ADR-0005's
crate allocation fail; ADR-0007's own falsification clause — collapse the core pump upward and
supersede the ADR if it has acquired no independent caller by the time the typed layer's phase
exits — is this playbook's recommendation applied to itself, in the same document that names the
pattern. Each reversal in this corpus cost a paragraph and a new ADR because the weaker half had
been named and dated, not silently mixed into the stronger claim.

## When the split is not worth it

It stops being worth splitting when both halves are settled by the same evidence — the
conjunction is then describing one decision with two consequences, not two decisions wearing one
title. The test is not "does this title have an 'and'" but "if I deleted one half, would the
other half's argument still be complete." ADR-0005's rename argument (`happenstance` is free on
crates.io, the GitHub repository already carries the name) would have been complete without the
crate-allocation claim riding along; that incompleteness-if-split test is what would have flagged
the risk before the fact, not just explained it after.

## Why this generalises

The shape recurs anywhere a single sitting produces a title with more than one claim: a naming
decision paired with a layout decision, a scope decision paired with an implementation detail.
What makes it worth writing down rather than trusting judgement fresh each time is that the
corpus got it wrong identically three times running, with the same tell each time — the weaker
half was a claim about code, made before the code existed.
