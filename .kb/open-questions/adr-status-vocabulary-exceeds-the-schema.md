---
id: kb-open-question-adr-status-vocabulary-001
title: An accepted-but-provisional ADR has no status the KB can express
kind: open_question
status: accepted
authority_tier: note
summary: >-
  KbFrontmatter's status is draft, proposed, accepted, superseded or withdrawn. The imported ADR
  corpus uses two values it does not have, and both are load-bearing. Nine of the seventeen ADRs
  are 'accepted - provisional', either wholly (ADR-0001 before its lift, ADR-0003, ADR-0004) or
  in named parts (ADR-0011, ADR-0012, ADR-0014, ADR-0015), each with a stated falsifier and a
  lifting phase; ADR-0003 says in terms that work contradicting a provisional decision still needs
  a superseding ADR but 'does not owe deference to a decision the code has not yet voted on',
  which is a weaker authority than accepted and is lost by mapping to it. Two more are 'partly
  superseded' - ADR-0005 and ADR-0006 - where marking the atom superseded would strip status from
  a half that still binds, and marking it accepted hides that a half does not. The 2026-08-10
  import applied a stated convention rather than inventing keys: status accepted, with the
  qualification and its falsifier in the first clause of the summary, and the supersedes pair used
  only for full supersession. What is not decided is whether that is the corpus's answer or a
  stopgap.
depends_on: []
related:
  - kb-open-question-provisional-falsifiers-001
  - kb-decision-0003
  - kb-decision-0005
  - kb-decision-0006
  - kb-decision-0017
  - kb-decision-0018
  - kb-decision-0019
source_paths:
  - .kb/_intake/0003-opaque-payloads.md
  - .kb/_intake/0004-edition-and-msrv.md
  - .kb/_intake/0005-rename-to-happenstance.md
  - .kb/_intake/0006-bare-name-to-the-typed-layer.md
  - .kb/_intake/0014-event-identity-and-recorded-time.md
  - .kb/_governance/integration-waves/2026-08-10-intake-2/02-placement-and-adjudication.md
  - .kb/README.md
  - .kb/decisions/README.md
last_reviewed: 2026-08-13
---

# An accepted-but-provisional ADR has no status the KB can express

## What is true today

`KbFrontmatter`'s `status` field is a closed enum: `draft`, `proposed`, `accepted`,
`superseded`, `withdrawn` (`.kb/README.md`, `src/schema/kb.ts`). The imported ADR corpus of
seventeen decisions carries two header values that map onto none of the five cleanly, and both
carry information the five-value vocabulary cannot hold.

**"accepted — provisional."** Nine of the seventeen ADRs use it, either for the whole document
(ADR-0001 before its provisional marker lifted, ADR-0003, ADR-0004) or for named parts of one
(ADR-0011, ADR-0012, ADR-0014, ADR-0015), and each instance carries a stated falsifier and the
phase expected to lift it. ADR-0003 states outright what the marker is worth: work that
contradicts a provisional decision still needs a superseding ADR to land, but does "not owe
deference to a decision the code has not yet voted on" — a real, intermediate authority level
between "settled" and "merely proposed," and mapping the header straight to `accepted` erases the
distinction between a decision the code has tested and one it has not.

**"partly superseded by."** ADR-0005 and ADR-0006 both carry it. Each bundled two claims under
one title; one claim stands and one was later reversed by a following ADR. Marking either atom
`superseded` would strip status from the half that still governs — ADR-0005's rename to
`happenstance` stands untouched by ADR-0006's correction of the crate allocation it was bundled
with. Marking either `accepted` hides that a stated part of it does not bind any longer.

## What is not decided

**Whether the 2026-08-10 convention is the corpus's real answer or a stopgap.** The convention
applied on this import, stated in the wave's own placement record: status `accepted` for both
cases, with the qualification and its falsifier folded into the first clause of `summary` so a
reader who reads only the summary is not misled, and the `supersedes` / `superseded_by` pair
reserved for full supersession only — a partly-superseded ADR keeps `superseded_by: null` and
carries the correction as prose plus a `related` link instead. That is a working convention
adopted to avoid inventing frontmatter keys `KbFrontmatter` does not own, not a schema change, and
nothing enforces that a future import applies it the same way.

**Whether the schema should grow instead.** The alternative not taken here is extending
`status`'s enum, or adding an optional qualifier field (a `provisional: bool` or similar)
recognised by `redkiln validate --kb`. That would let the state be queried rather than only read
in prose, at the cost of a schema change that binds every future atom of every kind, not just
imported ADRs.

## What forces it

The next ingest that imports a decision corpus with its own status vocabulary — internal or
external — meets the same gap and, absent a decision here, may pick a different convention,
leaving two waves' worth of provisional-but-accepted atoms distinguishable only by reading each
one's summary text. It is also forced the day one of the nine provisional ADRs lifts or is
superseded: the atom's `status` does not change (it was already `accepted`), so the only signal
that anything happened is an edit to `summary` and `last_reviewed` — indistinguishable, without
reading the diff, from an unrelated wording pass.

## Ordered sub-questions

1. Does `status` gain values, or does a separate field carry the qualification? This determines
   whether the fix is a schema change (binds every atom, every kind) or a convention (binds only
   readers who know to look).
2. If a convention: is the first-clause-of-summary placement used here the one to standardise, or
   should it live in a dedicated, greppable location (a `provisional_until` key, passthrough and
   therefore already legal to add on an atom without a schema change)?
3. Who owns the two "partly superseded" ADRs' eventual full resolution — does `related` stay the
   only link between the reversed half and its correction, or does the corpus want a stronger,
   queryable relation than a plain `related` array can express?

## Why this is a question and not a task

Answering it well needs a second data point: whether the next imported corpus's status
vocabulary looks anything like this one's. Settling the schema against a sample of one risks
building the wrong generalisation, and reversing a schema addition after atoms are written
against it is exactly the kind of edit the immutability rule was built to make expensive.

## Owner

Unassigned.
