---
id: question-postgres-position-visibility
title: How a Postgres adapter buys position visibility — measured, not preferred
kind: open_question
status: draft
authority_tier: note
summary: >-
  `nextval()` allocates outside the transaction, so a Postgres store violates ES-10 by
  construction unless it does something about it. The measurement now exists and names a
  winner — `xid8` + `pg_snapshot_xmin`, at no measurable throughput cost — but the ADR that
  binds the adapter to it is ADR-0024 and has not been written.
depends_on:
  - adr-0013-position-assignment-and-visibility
related:
  - reference-experiment-position-visibility
  - map-the-instrument-portfolio
source_paths:
  - crates/happenstance-postgres/src/event_store.rs
  - docs/experiments/position-visibility/README.md
last_reviewed: 2026-08-09
---

# Postgres position visibility

**What is true today.** ES-10 requires positions to become visible in assignment order.
`nextval()` allocates outside the transaction, so a writer taking 99 can commit after a
writer taking 100 — and a reader that has already observed 100 later sees 99 appear beneath
it. The measurement in `docs/experiments/position-visibility/` tested four mechanisms and
found that `xid8` + `pg_snapshot_xmin` buys ES-10 as written at no measurable throughput
cost; two others buy it by serialising every writer, at 16× and 30× the throughput; and
advisory locks keyed by tags are cheap **because they do not buy ES-10 at all**.

**What is not decided.** Which mechanism the adapter takes, in a binding form. The
measurement answers the empirical question; it does not by itself constitute the decision,
and ADR-0024 is the instrument for that.

**What forces it.** Phase 10. The adapter cannot pass the conformance suite's concurrency
family without a choice here, and the choice is owed a measurement rather than a preference
— which is now paid.
