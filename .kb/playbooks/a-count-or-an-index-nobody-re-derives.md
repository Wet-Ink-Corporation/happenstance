---
id: kb-playbook-count-or-index-nobody-re-derives-001
title: A count in a document nobody re-reads is a claim, not a fact
kind: playbook
status: accepted
authority_tier: guideline
summary: >-
  Four measured instances of the same defect — CLAUDE.md said seventeen decision
  atoms when there were thirty-two, RUNBOOK.md said 48 briefs awaited
  ratification three days after seventeen were ratified, ci.yml's gated-test
  arithmetic said 101, and the remediation README's table classified 41 of 49
  briefs. The repair pattern: remove the count where a command already answers
  it, leave an assertion as a floor rather than an equality where a number must
  exist, spell a list by its members once a count has drifted, and never read a
  curated index as a completeness proof.
depends_on: []
related:
  - kb-playbook-verify-referent-report-coverage-001
  - kb-governance-what-may-refute-a-finding-001
  - kb-decision-0040
  - kb-playbook-anchoring-citations-001
  - kb-playbook-ratchet-gate-landing-001
source_paths:
  - .kb/_intake/2026-09-07-ratifications-discharged-and-what-execution-changed.md
  - .kb/_intake/ratifications-2026-09-06-pre-publication.md
last_reviewed: 2026-09-07
---

# A count in a document nobody re-reads is a claim, not a fact

## The claim

A cardinal number written into a document that loads on every task, or a curated
index presented as a listing, decays the moment the thing it counts changes and
nobody is looking. Two failure shapes share this root and were both measured in
this repository within days of each other.

## Instance one: the stale count

`CLAUDE.md` — a file that loads on *every* task — stated `.kb/decisions/` held
"the seventeen atoms" until 2026-09-07, by which point there were thirty-two.
`RUNBOOK.md` said 48 briefs awaited ratification three days after seventeen of
them had already been ratified. `ci.yml`'s gated-test arithmetic hard-coded 101.
None of these numbers was wrong when written; each went stale the moment the
corpus it counted grew, and nothing forced a re-read, because a number that
merely sits in prose has no test that fails when it drifts.

## Instance two: the curated index read as complete

`.kb/_intake/remediation-2026-09-04-briefs/`'s README classifies briefs into
tables by semver impact and ratification deadline — 41 of 49 briefs, as it turned
out. Trusting that index as the full population missed `codec-foreign-tag-
resolution.md`, which carried an option "breaking, and therefore free only before
`0.2.0`" — a publish gate sitting outside the very index whose job was to surface
publish gates. Finding the gap required reading every brief's own `## Cost of
delay` section rather than trusting the table that summarized them. A curated
index is a claim about coverage, not a proof of it, and the two are
indistinguishable from inside the index itself.

## The repair pattern, in four moves

1. **Remove the count where a command already answers it.** `CLAUDE.md`'s fix
   was not a corrected number — it was deleting the number and pointing at `ls
   .kb/decisions/`, which cannot go stale because it is not written down.
2. **Leave an assertion as a floor, not an equality, where a number must exist
   in code.** An equality fails on the very commit that adds a rule, which
   trains whoever adds the rule to edit the number rather than read what the
   assertion is for. A floor (`>= N`) only fails when the count regresses, which
   is the failure worth catching.
3. **Spell a list by its members once a bare count has already drifted.**
   `CLAUDE.md`'s publishable-crate count is written as five names, not "five,"
   precisely because the bare count drifted once already (staying "three"
   through a promotion that should have moved it) and nobody caught it. Naming
   the members makes an omission visible on sight; a member missing from a
   named list is legible in a way a wrong integer is not.
4. **Never read a curated index as a completeness proof.** Where an index exists
   to route or classify a population, verify coverage independently — by reading
   every item's own record of the property the index claims to summarize —
   before relying on the index's silence as evidence of absence.

## Boundary

This does not argue against counts or indices generally — a floor assertion is
still a number, and an index is still worth having as a routing aid. It argues
against treating either as authoritative once the thing it describes can change
without the description being re-derived. The tell is the same in both
instances: the artifact answers a question a command or a full read could answer
more cheaply and more durably, and the artifact's age is invisible from inside
it.
