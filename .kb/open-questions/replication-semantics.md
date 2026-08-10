---
id: question-replication-semantics
title: Whether ingest re-checks append conditions, and what a position means across stores
kind: open_question
status: draft
authority_tier: note
summary: >-
  `SequencePosition` is meaningful only within one store, so positions cannot be replicated
  as-is. Whether ingest re-checks append conditions is the central unanswered question —
  §5.1 records a bound decision that ingest never rejects, and the shape of the peer port
  and the hub-and-spoke / peer-to-peer split are open with it.
depends_on: []
related:
  - map-the-three-ports
  - adr-0016-the-wire-format
source_paths:
  - crates/happenstance-sync/src/lib.rs
  - docs/architecture/SPECIFICATION.md
last_reviewed: 2026-08-09
---

# Replication semantics

**What is true today.** `happenstance-sync` is a skeleton port crate with `todo!()` bodies.
§5 of the specification carries an indicative shape for `SyncPeer` and a bound decision at
§5.1 — ingest never rejects — along with clauses on identity and idempotent ingest, the
transport floor, convergence, transitivity, causality, scope and retention. The wire format
is frozen (ADR-0016) and is **private to happenstance**, which is what makes reversals in it
free rather than breaking.

**What is not decided.** Whether ingest re-checks append conditions. A position is a
statement about one store's log, so it cannot cross a peer boundary unchanged; what replaces
it, and whether the receiving store may reject an event that would have violated a condition
locally, decides whether replication is a merge or a replay.

**What forces it.** Phase 13, which needs three real stores — proving a port takes two unlike
implementations and an oracle — and therefore waits on phases 8, 9, 10 and 12.

**Ordered sub-questions.**

1. Does ingest re-check append conditions? (§5.1's bound decision says no; the falsifier is
   still owed.)
2. What identifies an event across stores, such that ingest is idempotent?
3. Are hub-and-spoke and peer-to-peer one abstraction or two?
4. Where does a per-peer watermark live?
5. What does retention across a peer set do to a store that has been deleted from?
