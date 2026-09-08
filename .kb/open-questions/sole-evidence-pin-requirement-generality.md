---
id: kb-open-question-sole-evidence-pin-generality-001
title: Fifteen more rules share the shotgun mutant's exact hazard, and only the shotgun's own pin was required
kind: open_question
status: accepted
authority_tier: note
summary: >-
  The suite-self-reports lane found a false claim about REGISTRY's shotgun mutant — that no rule in
  the table is covered by that mutant alone — and fixed it narrowly: the paragraph now states what
  holds, and the_shotgun_mutants_sole_coverage_is_pinned requires an expect pin only where that one
  mutant is a rule's sole evidence. A crude regex-based count taken against REGISTRY at the time
  found roughly sixteen rules with exactly one declaring mutant whose fails list is longer than one
  entry, most carrying no pin — the same hazard CF-1 exists to police, generalised: the day any of
  them gains a second assertion, its evidence may silently become an anchor failure instead of the
  rule it was written to catch, and mutants_fail_exactly_their_declared_rules would not notice
  because the substring check only runs where a pin already exists. Whether every singly-covered
  rule should carry a pin, or only the shotgun's, was left open: the counter-argument is real — a
  general pin requirement creates pressure to add a second mutant instead of a pin, satisfying the
  check by accident rather than by the property that actually matters, which is two independent
  evidences per rule rather than one evidence plus a promise about it. Costing the general option
  needs a Rust-side census rather than the regex used here, and its natural home is the same pass
  that decides whether expect pins should be required rather than optional.
depends_on: []
related:
  - kb-decision-0010
  - kb-decision-0045
source_paths:
  - .kb/_intake/remediation-2026-09-04-briefs/sole-evidence-pins-and-moved-file-citations.md
  - crates/happenstance-testkit/tests/
  - spec/SPECIFICATION.md
last_reviewed: 2026-09-07
---

# Fifteen more rules share the shotgun mutant's exact hazard, and only the shotgun's own pin was required

## What is true today

The suite-self-reports lane (`L3-04`) found that a paragraph in the testkit's mutation-coverage
tests claimed "no rule in the table is covered by that mutant alone" about `InnerJoinTagStore`,
`REGISTRY`'s shotgun mutant — false, because `untagged_events_match_query_all` appears in exactly
one `fails` list in the whole event-store registry and it is `InnerJoinTagStore`'s. That sentence is
exactly what a reviewer reads to decide no further audit is needed, aimed at the one case that made
the audit unnecessary to skip.

The fix that landed is narrow by design: the paragraph now states what actually holds, the row
carries an `expect` pin naming the assertion, and a new rule,
`the_shotgun_mutants_sole_coverage_is_pinned`, requires a pin specifically wherever the *shotgun*
mutant — derived from the table as the row with the strictly broadest `fails` list — is a rule's
only registered evidence. The check and the paragraph are constructed so they can never drift apart
about which store is "the" shotgun.

A crude regex-based count taken against `REGISTRY` at the time of writing found roughly sixteen
rules whose only declaring mutant has a `fails` list longer than one entry — meaning that mutant is
shared evidence across several rules, which is exactly the shape that makes a pin meaningful — and
most of those sixteen carry no pin today. The count is explicitly an order of magnitude rather than
an exact figure: the regex mis-reads at least one row the file's own prose says is already pinned.
Named for whoever costs this properly: `LosingFixture` (two rules), `DropsMetadataStore`,
`ReadTimeClockStore`, `SingleGuardFastPathStore`, `PreCommitPositionStore`,
`SharedBatchPositionStore`, `TagsAreOrStore` (two rules), `ExactTagMatchReadStore`,
`UninternedTypeStore`, `NullHeadPagingStore`, `PayloadDedupStore`, `Latin1IdentifierStore`, and
`InnerJoinTagStore` itself.

CF-1 (`spec/SPECIFICATION.md:7656`) requires every conformance rule to be paired with at least one
mutant store that fails it — the obligation these pins exist to keep honest. The hazard the fifteen
share with the shotgun is structural, not particular to `InnerJoinTagStore`: any rule whose only
registered evidence is one mutant is one new assertion away from that mutant's coverage becoming an
anchor failure rather than the rule's own defect, at which point CF-1's obligation is discharged in
name only — the table says the rule is covered, and the coverage no longer proves what it claims to.

## What is not decided

Whether the pin requirement generalises to every singly-covered rule (`the_shotgun_mutants_sole_
coverage_is_pinned`'s shape, applied wherever a rule's sole declaring mutant's `fails` list has more
than one entry) or stays scoped to the one mutant the original finding was actually about. The
argument for leaving it narrow: the finding was about a specific false sentence, and a check that
ranges wider than the sentence it was written to falsify is a different change wearing this one's
clothes. The argument for generalising: the hazard is general and nothing else in the tree currently
says so, for roughly fifteen other rules.

The strongest argument against generalising is not cost but incentive: requiring a pin wherever
coverage is singular creates pressure to add a second mutant instead of writing a pin — which would
be the better outcome, and would satisfy a general check by accident, but is not the outcome anyone
would have chosen deliberately. If two independent evidences per rule is the property that actually
matters, a blanket pin requirement is a proxy for that property rather than a statement of it, and
conflating the two risks optimizing for the proxy.

## What forces it

Whoever next decides whether `expect` pins should be *required* rather than optional across the
registry generally — that pass is this question's natural home, because it needs a Rust-side census
of `REGISTRY`'s actual `fails` lists rather than the regex used here, which is known to mis-read at
least one row. Until that census runs, the fifteen named rules above are usable as a starting list
but not as a final one.
