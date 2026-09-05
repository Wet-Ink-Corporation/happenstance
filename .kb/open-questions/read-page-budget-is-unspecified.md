---
id: kb-open-question-read-page-budget-001
title: A read page is budgeted in rows, and the two costs of that knob pull opposite ways
kind: open_question
status: accepted
authority_tier: note
summary: >-
  ADR-0011 settled what read promises - one state sampled no later than the first poll, a position
  ceiling bounding every later statement, an inclusive to and an Option<usize> limit - and settled
  nothing about what a page costs. PAGE_SIZE appears nowhere in the knowledge base before this
  atom. The 2026-09-03 pre-publication review measured both of its consequences and they are not
  co-optimisable on one knob: one page of 512 ceiling-sized events is 512.2 MiB resident, which
  says the page is already too large, while raising PAGE_SIZE is the measured win for aggregate
  lock time, 2,929 seconds down to 99 over a one-million-event replay, which says it is too small.
  What is not decided is whether a page is budgeted in rows, in bytes, or by the caller. A row
  budget is what ships and is what the residency measurement indicts; a byte budget bounds
  residency and makes the number depend on payload size, which is exactly the quantity a store
  cannot know before reading; a caller-supplied budget moves the choice to the only party that
  knows both the payload distribution and the memory it has, at the cost of a port surface change
  that phase 12 closes. Forced by phase 12, after which the read surface is a promise, and
  independently by the first deployment that replays a log large enough for the lock time to
  matter on a machine small enough for the residency to.
depends_on: []
related:
  - kb-decision-0011
  - kb-reference-wf-11-memory-ceiling-verdict-001
  - kb-reference-projection-fan-out-cost-001
source_paths:
  - .kb/_intake/2026-09-03-pre-publication-review.md
  - references/evaluation/review-pre-publication-2026-09-03.md
  - crates/happenstance-sqlite/src/event_store.rs
  - spec/SPECIFICATION.md
last_reviewed: 2026-09-04
---

# A read page is budgeted in rows, and the two costs of that knob pull opposite ways

## What is true today

ADR-0011 settled seven dispositions on the read side: `read` keeps `query: &Query`, the port's
promise is one state sampled no later than the first poll rather than fully lazy, an adapter
issuing more than one statement per read must capture a position ceiling no later than the first
poll and bound every later statement by it, and `ReadOptions` gains an inclusive `to` and an
`Option<usize>` limit. None of those seven is about what one page of that read costs to hold in
memory or in lock time. `PAGE_SIZE` — the row count an adapter fetches per underlying statement
when a read spans more than one — appears nowhere in `.kb/` before this atom, and it is not a port
concept at all: `crates/happenstance-sqlite/src/event_store.rs:141` declares `const PAGE_SIZE:
usize = 512` as a private implementation constant, one `happenstance-sqlite` chose and the port
never asked for.

The 2026-09-03 pre-publication review measured what that private choice costs on both of its axes,
and the two measurements pull in opposite directions on the same knob. `kb-reference-wf-11-memory-ceiling-verdict-001`
found that one page of 512 events, each at the `MIN_SUPPORTED_EVENT_DATA_LEN` ceiling, is 512.2 MiB
resident at once — an argument that 512 is already too large for a store that wants to bound memory
per read. `kb-reference-projection-fan-out-cost-001`'s replay measurement found the opposite
pressure: raising the page size is the measured lever that took a one-million-event replay's
aggregate lock time from 2,929 seconds down to 99 — an argument that 512 is too small, and that
shrinking it to bound memory would make every replay slower by making SQLite reacquire its read
lock more often.

## What is not decided

Whether a page is budgeted in rows, in bytes, or by the caller — three different port surfaces, not
three tuning values for one. **A row budget** is what ships today (`PAGE_SIZE = 512`, a compile-time
constant) and is exactly what the residency measurement indicts: worst-case memory scales with
payload size the port never bounds, so a row count alone cannot bound memory unless it also caps
payload size far below what `StoreLimit::EventDataLen` already permits. **A byte budget** bounds
residency directly but makes the row count vary with payload size, which is precisely the quantity
a store cannot know before it has already read the rows — an adapter would need to fetch
speculatively, discard, and re-fetch smaller, or accept a soft rather than a hard ceiling. **A
caller-supplied budget** moves the choice to the party that actually knows both its payload
distribution and the memory it has to spend, at the cost of a new parameter on the read surface —
a change `ADR-0011` did not make and phase 12 closes off from being made without a semver break.

## What forces it

Phase 12, after which `read`'s surface — and therefore whatever `PAGE_SIZE` becomes or does not
become part of it — is a promise `cargo-semver-checks` polices. Independently, the first deployment
that replays a log long enough for the lock-time measurement to matter on a machine small enough
for the residency measurement to matter at the same time, which is exactly the scenario neither
number was measured against in combination.
