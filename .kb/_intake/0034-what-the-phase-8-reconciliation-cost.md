# What the phase-8 post-phase reconciliation cost, and what it found

**Staged for `/redkiln:kb-ingest`. Not an atom.** This document records evidence
from one hand-run of the standing criterion at `RUNBOOK.md:3810-3820`. It settles
nothing. In particular it does **not** answer any of the five ordered
sub-questions in `.kb/open-questions/nothing-owns-the-post-phase-reconciliation.md`,
and nothing here should be ingested as a decision — the two arguments below are
inputs to an ADR that does not exist yet.

Source: `.bklg/from-contract-to-published-library/sqlite-durable-store/spec-and-code-reconciliation/_reconciliation.md`,
the phase-8 pass, run against baseline `53a4764`.

## What it cost, in the units the open question asks about

| Quantity | Measured |
| --- | --- |
| Clauses in the phase's computed range | 8 (CF-14, CF-17, CF-34, ES-35, VT-21 – VT-24) |
| Clauses whose *prose* cites the phase's crate and had to be read anyway | 15 further passages |
| Verdicts *unchanged* | 3 of 8 in the range — all three already written by an upstream story in the same phase |
| Verdicts *repaired* | 5 of 8 in the range, and 15 of 15 outside it |
| Verdicts *gap* | 0 normative; 2 escalations, neither of them a clause |
| Stale `file:line` citations found | 6 — every one of them still **resolving**, and every one pointing at the wrong subject |
| Factually false counts found | 4 — "seven real variants" (twelve), "five of them skeletons" (four), "**Two** of the five are `todo!()` throughout" plus "Nothing runs against any of the five" (one now runs a suite), "one adapter instrument at one far end" (two) |
| Markers whose falsifier named an event that had already happened | 1 — ES-41's, which named `happenstance-sqlite` at phase 8 as its instrument |
| Effort shape | one pass, one context, no tooling written |

## Finding 1 — `spec-trace` green is not evidence that a citation is right

This is the strongest argument the pass produced, and it is mechanical rather
than editorial.

Six citations into `crates/happenstance-sqlite` **resolved** — the file existed
and the line was in range — while pointing at an entirely different subject than
the sentence claimed. `event_store.rs:151-193` was cited as an error enum and now
holds a struct definition; `:202` was cited as an impl writing `+ Send` and now
holds a `const MIGRATION_1`; `:195` was cited as a `head` impl and now holds
`SCHEMA_VERSION`; four separate clauses cited `:47-53` as "the tag side table"
when `:47-53` is the `event` table and the side table is `:62-67`.

None of them was reported. The reason is visible in the tool's own numbers: of
401 citations checked, **80** are *anchored*, because `subject_before`
(`xtask/src/spec_trace.rs:2158`) declines unless the code span immediately before
the citation is a lowercase identifier of four characters or more on the citation's
own line or the one above. Every one of the six sits in a sentence whose nearest
span is a type name, a quoted phrase or another citation — so the subject is not
derivable, the citation is counted and not anchored, and drift is invisible.

That is the tool behaving exactly as documented and arguably correctly: it
"declines rather than guesses". The finding is not that the heuristic is wrong.
It is that **`checked` is a coverage number and `anchored` is the only quality
number, and 80 of 401 is the fraction of the corpus under any anchoring
discipline at all.** A reader who takes a green `spec-trace` as "the citations
are right" is reading a claim about 20% of them.

*Candidate mechanisations, none implemented:* an explicit `<!-- anchor: name -->`
comment beside a citation whose subject the heuristic cannot derive, which would
let the anchored fraction be raised deliberately rather than accidentally; or a
per-run report of the *unanchored* citations, so the 318 are a visible backlog
rather than a silence.

## Finding 2 — the arithmetic bullet has nothing to close against

The criterion's second bullet asks for the phase's clause range and the union of
its ADRs' ranges, computed and compared. Phase 8 has one ADR, and
`.kb/decisions/0022-append-condition-strategy.md` **states no clause range in any
form** — no frontmatter field, no clause ID in its `## Decision`, and
`RUNBOOK.md:301`'s queue row for it is the only row in that queue with no
parenthesised range.

So the second set had to be derived from the phase's own three (disagreeing)
statements, which is the arithmetic comparing a number against itself. The
phase-4 precedent is the same defect from the other side: queue rows named 35
clauses, the body discharged 64, and the 29 missing were invisible in exactly the
way a finished clause is invisible (`RUNBOOK.md:315-322`).

*Candidate mechanisation, not implemented:* a `clauses:` list in the decision
atom's frontmatter, which `redkiln validate --kb` could require of any atom whose
`phase` is set. That is sub-question 3 of the open atom — *whether clause ranges
become machine-readable* — and answering it in passing is what this document
refuses to do.

## Finding 3 — the pass's real value was in prose nothing checks

Of 20 repairs, **6** were citation line numbers a machine could plausibly have
caught and **14** were sentences that were simply false: counts, tenses
("*planned* SQLite schema" for a schema that had landed), and one `[PROVISIONAL]`
falsifier naming an event that had already occurred. No conceivable extension of
`spec-trace` catches "seven" when the answer is twelve.

That cuts both ways for the open question. It argues *against* believing a gate
step could replace the pass, and *for* the pass being owned by someone rather than
left standing — because the failure mode is silent, durable, and reads as
authoritative.

## What was deliberately not done

- No committed citation baseline. Writing `389/76` into the tree beside the list
  it counts would answer sub-question 2 against
  `.kb/playbooks/landing-a-stricter-gate-without-a-red-baseline.md`.
- No new gate step, no new `xtask` check, no widening of `ANCHOR_SLACK`.
- No edit to `.kb/open-questions/nothing-owns-the-post-phase-reconciliation.md`.
- No clause amended, no marker moved, no ADR written.
