---
id: kb-decision-0118
title: ES-18's rejected append is unchanged as observed through the port, consumed positions excepted
kind: decision
status: accepted
authority_tier: decision
adr_id: ADR-0118
reversibility: low
phase: 20
supersedes: null
superseded_by: null
summary: >-
  Accepted 2026-11-25 by the owner; proposed by the v1 AFK run. ES-18 is FROZEN and says a rejected append
  MUST leave the store byte-identical. No rule checks bytes: every rule compares what reads return
  through the port. Three of the four published adapters can leave residue a read never shows: the
  Cloudflare store documents it, and Postgres and Neon burn sequence values on a Busy refusal; the
  testkit already treats a refused append that burns a gapped position as conformant. Recommended:
  narrow the MUST to "unchanged as observed through the port (reads, head, append conditions);
  positions it consumed are excepted and may appear as gaps". Free now, a break after 1.0. The flip
  is residue that is visible through the port.
depends_on:
  - kb-decision-0066
related:
  - kb-open-question-es-18-byte-identical-conformance-001
  - kb-decision-0077
source_paths:
  - spec/SPECIFICATION.md
  - crates/happenstance-testkit/src/suite.rs
  - crates/happenstance-cloudflare/src/event_store.rs
  - crates/happenstance-postgres/src/event_store.rs
  - crates/happenstance-neon/src/event_store.rs
  - crates/happenstance-sqlite/src/event_store.rs
last_reviewed: 2026-11-25
---

# ES-18's rejected append is unchanged as observed through the port, consumed positions excepted

**Accepted 2026-11-25** by the owner (Weigh-In wi-c978794167); proposed by the unattended v1 run.

## Context

ES-18 (`[FROZEN]`, `spec/SPECIFICATION.md` about `:3710`): "A rejected append MUST leave the store
byte-identical." Its rules are `append_is_atomic`, `condition_rejection_leaves_store_unchanged`,
`append_is_atomic_under_a_mid_batch_fault` and `a_concurrent_reader_never_sees_a_partial_batch`.

`condition_rejection_leaves_store_unchanged` (`crates/happenstance-testkit/src/suite.rs`, about
`:5925`) compares `read(Query::all())` before and after a refused conditional append, on one handle
and on a second. Its message says "byte-identical"; it checks what reads return, not `head`, not
the next position and not storage. `.kb/open-questions/es-18-byte-identical-versus-conformance-reading.md`
asks the same question and has no issue. (Two of its citations are stale: the clause line, and a
SQLite test, `autoincrement_high_water_is_restored_by_a_rollback`, that is no longer in the tree.)

## Evidence, per store

| Store | A rejected append (`ConditionViolated`) | Busy, or a failure after rows landed |
| --- | --- | --- |
| `MemoryEventStore` | checked under the write lock before anything is stored; positions come from the length, nothing consumed | no Busy path |
| `happenstance-sqlite` | evaluated inside `BEGIN IMMEDIATE`, transaction dropped; `sqlite_sequence` is transactional and rolls back | Busy comes from `BEGIN IMMEDIATE`, before any write |
| `happenstance-postgres` | the condition is evaluated before positions are allocated (`nextval`); nothing consumed | a `40001` that outlasts the retry loop can arise after `nextval`; sequences do not roll back, so positions are burned |
| `happenstance-neon` | a probe, then a conditional `INSERT` whose positions come from a `nextval` subquery; whether a rejected one draws is **not verified** in the tree | a serialisable abort after the draw burns positions, as on Postgres |
| `happenstance-cloudflare` | probe first, no await, writes nothing | never Busy; a write that fails after rows land is compensated by `discard_from`, which leaves the `AUTOINCREMENT` high-water mark advanced; the adapter documents that this is not byte-identical (`event_store.rs`, about `:1604-1613`) |

And the testkit already accepts consumed positions: `GappedTrailingFrontierStore` is a conformant
control "with a position burned by each refused conditional append, as nextval() burns one", and
`a_refused_append_burns_a_gapped_position` asserts the next batch starts past the burned position
(`crates/happenstance-testkit/tests/mutation_coverage/trailing.rs`).

## Options

- **A. Narrow the MUST (recommended):** *"A rejected append MUST leave the store unchanged as
  observed through the port — reads, `head` and append conditions. Positions it consumed are
  excepted, and may appear as gaps (ES-9)."* That is what every rule checks and what every
  adapter does. Free before 1.0, a major after.
- B. Keep "byte-identical" and fix the adapters: Postgres and Neon cannot un-draw a sequence value,
  so this means a different allocation scheme, re-opening ADR-0024.
- C. Keep the text and record a reading. A frozen MUST that three of four adapters do not meet,
  explained elsewhere, is the drift this specification's markers exist to prevent.

## Decision

A. On acceptance: ES-18's MUST is reworded by this record; its rules, cases and `Rejects:` stand;
the open question is superseded. No code changes; the rule's message may say "unchanged through the
port" instead of "byte-identical".

## What would flip it

A refused append whose residue is visible *through the port*: Cloudflare's `head`, or a read that
surfaces an unswept row or a burned position. Then the narrowed sentence is also unmet, and the fix
is the adapter, not the clause.

## Door

One-way: a `[FROZEN]` clause's MUST changes, and it narrows what a consumer may rely on.
