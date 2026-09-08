---
id: kb-open-question-es-17-two-adapter-measurement-001
title: The two-build measurement ES-17's falsifier requires is scheduled by nobody
kind: open_question
status: accepted
authority_tier: note
summary: >-
  ADR-0012 kept append's events &[Event] and declined to lift the clause to frozen, naming phase 8
  as the phase that would produce the evidence; its falsifier item 1 requires two builds of the
  same adapter differing only in append's ownership, measured on the same harness. Phase 8
  produced an append-condition measurement instead — three candidate stores in an experiment
  crate, recorded as kb-reference-append-condition-experiment-001 — and the candidates differ in
  strategy, not in ownership. The pre-publication brief then settled the measurement's subject and
  kb-decision-0055 recorded it: happenstance-cloudflare, not MemoryEventStore, is falsifier item
  4's benefiting shape, and has been since phase 9, so the falsifier as written — which names the
  SQLite multi-row insert benchmark — points at the one adapter shape item 4 excludes. Two further
  obstacles are now named rather than guessed at: the shipped event_store_benchmarks! harness
  hard-codes one tag against VT-22's 64-tag floor with no BenchmarkParams field that could express
  otherwise, and its contention scenario shares one batch by reference, so the caller-side cost
  by-value would create is structurally invisible. What is still not decided is who takes the
  measurement and when, or whether the obligation is deferred to a later phase with a named owner.
  The deadline is earlier than this atom first recorded: happenstance-core has been published at
  0.2.0-alpha.1 since 2026-08-16, and what phase 12 removes is the pre-release exemption, not the
  first exposure. The KB half is this record; the queue row it also needs belongs to .bklg/.
depends_on: []
related:
  - kb-decision-0012
  - kb-decision-0022
  - kb-decision-0055
  - kb-reference-append-condition-experiment-001
  - kb-reference-event-clone-allocations-001
  - kb-decision-0037
  - kb-open-question-adr-0022-falsifiers-fired-001
  - kb-open-question-references-adr-correction-policy-001
source_paths:
  - .kb/_intake/0033-adr-0022-append-condition-strategy.md
  - .kb/_intake/remediation-2026-09-04-briefs/append-batch-ownership.md
  - references/adr/0012-append-shape-and-preconditions.md
  - references/adr/0022-append-condition-strategy.md
  - RUNBOOK.md
last_reviewed: 2026-09-07
---

# The two-build measurement ES-17's falsifier requires is scheduled by nobody

## What is true today

`spec/SPECIFICATION.md`'s ES-17 stays `[PROVISIONAL]`, carrying `events: &[Event]` as the shape
`EventStore::append` takes but not the frozen guarantee that shape is correct. ADR-0012
(`kb-decision-0012`) declined to lift it, because the evidence its own marker asks for — an adapter
with a real write path and a benchmark harness, under a realistic batch and rejection mix — did not
exist at phase 4, and named phase 8 as the phase that would produce it. ADR-0012's falsifier item 1
is specific about the shape of that evidence: *two builds of the same adapter, differing only in
`append`'s ownership of its batch, measured on the same harness.* A `Vec<Event>` build and a
`&[Event]` build of one real adapter, timed the same way.

Phase 8 has now run, and it produced `kb-reference-append-condition-experiment-001` — three
candidate `EventStore` implementations, measured against real SQLite, that settle which
append-condition SQL strategy wins. That is a real measurement, cleared through conformance first,
and ADR-0022 (`kb-decision-0022`) rests on it. But it answers a different question: the three
candidates differ in how they evaluate an append condition, not in whether `append` borrows or owns
its batch. Every one of them was built against the same `&[Event]` signature. ADR-0022 says so
itself, recording the marker as not lifted and quoting ADR-0012's falsifier item 1 verbatim rather
than treating its own measurement as a substitute for it.

The implementation stories that follow ADR-0022 in this project's story map build one SQLite
adapter — the one the experiment's winning arm becomes — not two differently-owned builds of one.
Nothing currently scheduled produces the falsifier's evidence.

## What the pre-publication brief settled, and what it did not

`kb-decision-0055` keeps `&[Event]` for `0.2.0` and, in doing so, fixes the measurement's *subject*
without naming its owner. Three corrections land on this question.

**The falsifier as written names the one shape that cannot fail it.** ES-17's marker names *the
SQLite adapter's multi-row insert benchmark* as the measurement that would lift it, and ADR-0012's
own item 4 says only a store that moves the payload into an owned row it keeps can benefit.
`happenstance-sqlite`'s `write_batch` binds every column from the borrow, so scheduling the
measurement against SQLite would produce a null by construction. ADR-0055 restates the falsifier so
it names its adapter shape, its tag regime and its tag count; it does not fire it.

**The census ADR-0012 wrote is stale, and that accepted atom cannot say so itself.** ADR-0012 names
`MemoryEventStore` as the only store that clones and disqualifies it as unrepresentative. That was
true at phase 4. Since phase 9 `happenstance-cloudflare` has had full bodies, has run
`event_store_conformance!` and has lost its `publish = false`, and its `write_rows` copies event
type, payload and metadata into owned `SqlValue`s per event because the Workers SQL binding cannot
take a borrow. It is item 4's benefiting shape in an adapter that is neither the reference store nor
SQLite, so the measurement's subject is `happenstance-cloudflare` — and the saving it would show
scales with payload size, not tag count, which is a different axis from the 66x clone finding in
`kb-reference-event-clone-allocations-001` rather than a larger version of it.

**Two obstacles are now named rather than guessed at.** The shipped `event_store_benchmarks!`
harness already runs in the expensive `Cow::Owned` tag regime, but hard-codes **one** tag against
VT-22's 64-tag floor, and `BenchmarkParams` carries no field that could express otherwise — so as
shipped it would answer a 66-allocation question with a 3-allocation number. Its contention
scenario shares one batch by reference, so the caller-side copy a by-value `append` would force on
every retrying contender is structurally invisible to it. Against that, one earlier objection is
gone: `experiments/append-condition/` drives `event_store_benchmarks!` verbatim over five variant
stores from outside the workspace, so the second build needs a local port in an experiment crate,
not a forked testkit.

## What is not decided

Who takes the two-build measurement, and when. Two live options, and neither has been chosen: a
story could be added to take it as a deliberate side experiment before ES-17 is next revisited, or
the obligation could be explicitly deferred to a later phase with a named owner. Either of those is
a decision. What exists right now is neither: the marker sits `[PROVISIONAL]`, the falsifier's
evidence is unproduced, and no queue row or story names an owner. Whether the testkit's harness
should gain a tag-count parameter is a related but separable question — it is testkit surface work
with its own review, and the measurement can be taken from an experiment crate without it.

## What forces it

Two triggers, either sufficient. First, whoever next proposes lifting ES-17 to `[FROZEN]` — they
would either have to produce the two-build measurement first or explain why the append-condition
experiment substitutes for it, and this record is what stops that substitution from happening
silently. Second, phase 12 — but the clock is earlier than this atom first recorded it.
`happenstance-core` has carried `&[Event]` in a published crate since `0.2.0-alpha.1` on
2026-08-16; what keeps the window cheap is not that nothing is published but that no `^0.2`
requirement resolves to a pre-release, so only a dependent who named the alpha explicitly is
pinned. Phase 12 removes that exemption rather than creating the first exposure. The KB half of
this obligation is this open-question atom; the corresponding backlog row belongs in `.bklg/` and
is not authored here.
