---
id: kb-open-question-es-17-two-adapter-measurement-001
title: The two-build measurement ES-17's falsifier requires is scheduled by nobody
kind: open_question
status: accepted
authority_tier: note
summary: >-
  ADR-0012 kept append's events &[Event] and declined to lift the clause to frozen, naming phase 8
  as the phase that would produce the evidence; its falsifier item 1 requires two builds of the
  same SQLite adapter differing only in append's ownership, measured on the same harness. Phase 8
  has now produced its append-condition measurement — three candidate stores in an experiment
  crate, recorded as kb-reference-append-condition-experiment-001 — and that is not it: the
  candidates differ in strategy, not in ownership. What is true today is that ADR-0022 records the
  marker as not lifted and quotes ADR-0012's falsifier verbatim rather than discharging it, and
  that the four implementation stories after ADR-0022 in this project's map build one adapter
  rather than two builds of one. What is not decided is who takes the measurement, or whether the
  obligation is deferred to a later phase with a named owner instead; either is a decision, and the
  silence is neither. Forced by whoever proposes to lift ES-17 to frozen, or by phase 12, where
  first publish turns append's signature into a promise. The KB half is this record; the queue row
  it also needs belongs to .bklg/ and is not a KB atom.
depends_on: []
related:
  - kb-decision-0012
  - kb-decision-0022
  - kb-reference-append-condition-experiment-001
  - kb-reference-event-clone-allocations-001
  - kb-decision-0037
  - kb-open-question-adr-0022-falsifiers-fired-001
source_paths:
  - .kb/_intake/0033-adr-0022-append-condition-strategy.md
  - references/adr/0012-append-shape-and-preconditions.md
  - references/adr/0022-append-condition-strategy.md
  - RUNBOOK.md
last_reviewed: 2026-08-17
---

# The two-build measurement ES-17's falsifier requires is scheduled by nobody

## What is true today

`spec/SPECIFICATION.md`'s ES-17 stays `[PROVISIONAL]`, carrying `events: &[Event]` as the shape
`EventStore::append` takes but not the frozen guarantee that shape is correct. ADR-0012
(`kb-decision-0012`) declined to lift it, because the evidence its own marker asks for — an adapter
with a real write path and a benchmark harness, under a realistic batch and rejection mix — did not
exist at phase 4, and named phase 8 as the phase that would produce it. ADR-0012's falsifier item 1
is specific about the shape of that evidence: *two builds of the same SQLite adapter, differing only
in `append`'s ownership of its batch, measured on the same harness.* A `Vec<Event>` build and a
`&[Event]` build of one real adapter, timed the same way.

Phase 8 has now run, and it produced `kb-reference-append-condition-experiment-001` — three
candidate `EventStore` implementations, measured against real SQLite, that settle which
append-condition SQL strategy wins. That is a real measurement, cleared through conformance first,
and ADR-0022 (`kb-decision-0022`) rests on it. But it answers a different question: the three
candidates differ in how they evaluate an append condition, not in whether `append` borrows or owns
its batch. Every one of them was built against the same `&[Event]` signature. ADR-0022 says so
itself, recording the marker as not lifted and quoting ADR-0012's falsifier item 1 verbatim rather
than treating its own measurement as a substitute for it.

The four implementation stories that follow ADR-0022 in this project's story map build one SQLite
adapter — the one the experiment's winning arm becomes — not two differently-owned builds of one.
Nothing currently scheduled produces the falsifier's evidence.

## What is not decided

Who takes the two-build measurement, and when. Two live options, and neither has been chosen: a
story could be added to take it as a deliberate side experiment before ES-17 is next revisited, or
the obligation could be explicitly deferred to a later phase with a named owner — most plausibly
phase 12, where first publish turns `append`'s signature into a promise a downstream crate can pin
against, which is the point ADR-0004's provisional marker on the MSRV floor uses for the same kind of
deadline. Either of those is a decision. What exists right now is neither: the marker sits
`[PROVISIONAL]`, the falsifier's evidence is unproduced, and no queue row or story names an owner.

## What forces it

Two triggers, either sufficient. First, whoever next proposes lifting ES-17 to `[FROZEN]` — they
would either have to produce the two-build measurement first or explain why the append-condition
experiment substitutes for it, and this record is what stops that substitution from happening
silently. Second, phase 12: first publish is the point at which an unresolved signature stops being
free to change, so the measurement (or an explicit, owned deferral past phase 12) is owed by then
regardless of whether ES-17 is ever formally revisited before it. The KB half of this obligation is
this open-question atom; the corresponding backlog row belongs in `.bklg/` and is not authored here.
