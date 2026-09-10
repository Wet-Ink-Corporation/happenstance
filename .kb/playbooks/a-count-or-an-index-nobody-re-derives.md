---
id: kb-playbook-count-or-index-nobody-re-derives-001
title: A count in a document nobody re-reads is a claim, not a fact
kind: playbook
status: accepted
authority_tier: guideline
summary: >-
  Six measured instances of one defect, in three shapes. A cardinal number —
  CLAUDE.md said seventeen decision atoms when there were thirty-two, RUNBOOK.md
  said 48 briefs awaited ratification three days after seventeen were ratified,
  ci.yml's gated-test arithmetic said 101. A curated index read as a listing —
  the remediation README's table classified 41 of 49 briefs. And a status claim
  about an axis: when ES-11's provisional falsifier fired on happenstance-neon,
  four passages of spec/SPECIFICATION.md became false at once and none of them
  carried a number, including ES-11's own Rejects bullet ending "It is
  conformant today", written 2026-08-06 and untouched through the two phases in
  which that adapter was built and shipped. The repair pattern: remove the count
  where a command already answers it, leave an assertion as a floor rather than
  an equality where a number must exist, spell a list by its members once a
  count has drifted, never read a curated index as a completeness proof, and
  repair the prose that names an axis in the same change that fires its
  falsifier — because cargo xtask spec-trace checks that citations resolve, not
  that prose is current.
depends_on: []
related:
  - kb-playbook-verify-referent-report-coverage-001
  - kb-governance-what-may-refute-a-finding-001
  - kb-decision-0040
  - kb-decision-0061
  - kb-playbook-anchoring-citations-001
  - kb-playbook-ratchet-gate-landing-001
  - kb-open-question-provisional-falsifiers-001
source_paths:
  - .kb/_intake/2026-09-07-ratifications-discharged-and-what-execution-changed.md
  - .kb/_intake/ratifications-2026-09-06-pre-publication.md
  - .kb/_intake/2026-09-08-adr-0061-es-11s-sufficiency-condition-assumed-a-queue.md
  - spec/SPECIFICATION.md
last_reviewed: 2026-09-09
---

# A count in a document nobody re-reads is a claim, not a fact

## The claim

A written claim decays the moment the thing it describes changes and nobody is
looking. A cardinal number in a document that loads on every task, a curated
index presented as a listing, and a status claim about an axis are three shapes
of that one root, and all three were measured in this repository within days of
each other.

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

## Instance three: the status claim a fired falsifier made false

The third shape carries no number at all. ES-11's `[PROVISIONAL]` marker named
the adapter that would falsify it, and in 2026-09 `happenstance-neon` did, and
ADR-0061 narrowed the clause's sufficiency condition in response.
Four passages of `spec/SPECIFICATION.md` — the highest document in this
repository's precedence ladder — became false in the same instant: §6.5's
**Transport** row still called `happenstance-neon` a phase-2 skeleton and "not a
far end"; its **Position allocation** row said the same of
`happenstance-postgres`; §3's `EventStore` cell said transport was "empty at both
ends"; and ES-11's own `Rejects:` bullet ended *"It is conformant today"*,
written 2026-08-06 (`12ecb1a`) and untouched through the two phases in which that
adapter was built, shipped and made the sentence wrong. All four were repaired in
the same change as the ADR that fired the falsifier. `cargo xtask spec-trace` is
a standing gate step and it caught none of them, because it checks that a
clause's citations **resolve**, not that its prose is **current** — so a green
gate carried four false sentences in the normative document.

## The repair pattern, in five moves

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
5. **Sweep the prose that names an axis in the same change that fires its
   falsifier.** A falsifier firing invalidates every sentence that describes the
   state it just changed, and nothing mechanical looks for them: grep the axis's
   own words — the adapter name, the clause id, the phrase "at both ends" — and
   repair the hits before the change lands. Where the claim must stay in prose,
   write it so that the event which would invalidate it is the same event that
   reaches the sentence: a marker that *records* a fired falsifier cannot go
   stale the way one that *predicts* an unfired one does.

## Boundary

This does not argue against counts, indices or status claims generally — a floor
assertion is still a number, an index is still worth having as a routing aid,
and a clause must be able to say what is true today. It argues against treating
any of them as authoritative once the thing described can change without the
description being re-derived. The tell is the same in all three instances: the
artifact answers a question a command, a full read, or the change itself could
answer more cheaply and more durably, and the artifact's age is invisible from
inside it.
