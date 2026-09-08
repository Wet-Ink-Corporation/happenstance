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
  matter on a machine small enough for the residency to. Partly answered 2026-09-07 by ADR-0053
  (kb-decision-0053), which takes the first fork and a half: the row budget stays private and
  unchanged at 512, a public MAX_PAGE_BYTES_PER_STATEMENT at 8 x MAX_EVENT_DATA_LEN bounds one
  page's payload residency beside it, and the connection is taken per statement rather than held
  per page - so a page's residency is bounded and stated for the first time, at
  MAX_PAGE_BYTES_PER_STATEMENT x planned_statement_count(query). The third fork, a caller-stated
  budget, is not taken and stays open: whether the number belongs to the read, the store or the
  contract, dated to 0.2.0 because a public constant already shipped is one that answer would have
  to duplicate or deprecate. Three residues travel with it - whether 8 MiB is the right value,
  since nothing measured it and every workload in this repository is below it; whether PAGE_SIZE
  should become public, since a caller can compute the byte bound and not the row bound; and
  whether a wide query should share one budget across its statements rather than multiply it.
depends_on: []
related:
  - kb-decision-0011
  - kb-decision-0053
  - kb-reference-wf-11-memory-ceiling-verdict-001
  - kb-reference-projection-fan-out-cost-001
  - kb-reference-one-connection-latency-001
  - kb-open-question-event-metadata-no-floor-001
  - kb-playbook-anchoring-citations-001
source_paths:
  - .kb/_intake/2026-09-03-pre-publication-review.md
  - .kb/_intake/remediation-2026-09-04-briefs/read-page-budget-rows-bytes-or-caller.md
  - .kb/_intake/remediation-2026-09-04-briefs/sqlite-lane-spec-citation-repoints.md
  - references/evaluation/review-pre-publication-2026-09-03.md
  - crates/happenstance-sqlite/src/event_store.rs
  - spec/SPECIFICATION.md
last_reviewed: 2026-09-07
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
concept at all: `crates/happenstance-sqlite/src/event_store.rs:222` declares `const PAGE_SIZE:
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

## Partly answered 2026-09-07 — ADR-0053 takes the row-and-byte shape; the caller keeps nothing

`kb-decision-0053` settles the parts of the question that had no trade in them and leaves the third
fork exactly where this atom put it. **A byte budget now exists at all**: the public
`SqliteEventStore::MAX_PAGE_BYTES_PER_STATEMENT` (`event_store.rs:380`), set to `8 *
MAX_EVENT_DATA_LEN`, bounds one page's payload residency, and the connection is taken per statement
rather than held across a page — sound only because of ADR-0011's ceiling, since every statement
carries `position <= H` and what commits between two of them is invisible to all of them. `PAGE_SIZE`
is **unchanged at 512 and still private**; the stated peak is now
`MAX_PAGE_BYTES_PER_STATEMENT × planned_statement_count(query)`, both terms computable by a caller,
where before it was bounded only by SQLite's near-gigabyte blob limit and stated nowhere.

That answers the row-versus-byte half of the fork by refusing it: **both**, because the two bound
different quantities. Collapsing to a byte budget alone — deleting `PAGE_SIZE` and taking rows until
the bytes are spent — was rejected on a mechanism, not a preference: the row count binds
`Vec::with_capacity` and is what `LIMIT ?` binds, and a page of a million tiny events would otherwise
be one statement returning a million rows to fill 8 MiB. A cardinality bound and a size bound are
both needed.

**What stays open is the caller-stated budget**, and it narrowed rather than moved. `ReadOptions`
belongs to `happenstance-core`, so an adapter-specific knob cannot live there without obliging every
adapter to answer a question only a buffering one has; the adapter-local alternative, a builder on
`SqliteEventStore`, makes construction stateful or fallible in a way it is not today. Either is real
public surface, which is why it is an ADR rather than a patch — and it is **dated to `0.2.0`**,
because `MAX_PAGE_BYTES_PER_STATEMENT` is public from publication and a caller-stated budget would
have to duplicate or deprecate it.

Three residues travel with it, each named by the lane that landed A rather than discovered later.
**The 8 MiB value is unmeasured** — the experiment that justified having a byte budget measured
`PAGE_SIZE`, not this constant, and nobody has run a replay at 4 MiB or 32 MiB. What answers that
partly is asymmetric evidence rather than more of it: `PAGE_SIZE`'s value moves the aggregate lock
hold 30× between 64 and 2,048, while `MAX_PAGE_BYTES_PER_STATEMENT` binds only on payloads near the
data ceiling, above every workload in this repository's tests and examples. **`PAGE_SIZE`'s
visibility** is an asymmetry nobody has argued for: a caller can compute the page's byte bound and
not its row bound. **The peak's shape** — per-statement rather than one budget shared across a wide
query's statements — was chosen because the shared form starves every statement after the first, a
mechanism argument with no measurement behind it either way.

**Upstream of all three sits AE-4's hole, and the byte budget does not close it.**
`check_ceilings` does not bound `metadata`, so the quantity that can make one row exceed the whole
page budget is the one this adapter never refuses at `append`: the budget makes the *read* survive
it and nothing makes the *write* refuse it. ADR-0043 gave the contract a refusal channel
(`StoreLimit::MetadataLen`, `guaranteed_minimum() == 0`); whether this adapter states one is still
`kb-open-question-event-metadata-no-floor-001`'s residue, and the two are meant to be read together.

**Two citation corrections, recorded rather than applied silently.** The `PAGE_SIZE` declaration
cited in *What is true today* was repointed by anchor from `:141` to `:222` (`const PAGE_SIZE:
usize = 512;`, unique in the file) per `kb-playbook-anchoring-citations-001` — it had already drifted
before this atom was written. And `PAGE_SIZE`'s doc comment no longer calls itself *"a placeholder
until it is measured"*: the measurement came back and the sentence was deleted, so a sibling brief's
quotation of it now names text that is not in the tree. The doc names the three measured
consequences instead, and says the measurement declined to settle the knob.
