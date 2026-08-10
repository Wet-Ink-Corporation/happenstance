---
id: question-append-condition-sql-strategy
title: The append-condition SQL strategy for the SQLite adapter
kind: open_question
status: draft
authority_tier: note
summary: >-
  The driver question is settled — `happenstance-sqlite` is rusqlite and `happenstance-postgres`
  is sqlx, and the two are in the tree for different reasons rather than as candidates. How an
  append condition becomes SQL is not; it is ADR-0022's, with notes in the crate.
depends_on:
  - adr-0012-append-shape-and-preconditions
related:
  - question-postgres-position-visibility
source_paths:
  - crates/happenstance-sqlite/src
last_reviewed: 2026-08-09
---

# The append-condition SQL strategy

**What is true today.** The driver question closed at phase 2 by building both: rusqlite for
SQLite, sqlx for Postgres. `AppendCondition` is frozen at the contract level by ADR-0012 —
its shape, its preconditions, and what a dropped future may have done.

**What is not decided.** How the condition is evaluated against a SQL store: as a predicate
in the insert, as a separate read inside the transaction, or as a uniqueness constraint that
turns a violation into a conflict error. Each has a different cost under contention and a
different failure mode when the store leaves position gaps.

**What forces it.** Phase 8, ADR-0022 — together with the schema and tag-storage questions
in the same ADR.
