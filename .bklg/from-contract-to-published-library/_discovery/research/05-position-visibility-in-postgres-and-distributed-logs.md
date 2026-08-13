---
title: "position-visibility-in-postgres-and-distributed-logs — research for From Contract to Published Library"
kind: research
initiative: from-contract-to-published-library
summary: "Position/offset visibility gaps are a well-documented, cross-ecosystem problem — not a happenstance-specific edge case — and the DCB specification itself is silent on the guarantee, which is exactly the open question ES-10 already names."
sources:
  - "https://blog.sequinstream.com/postgres-sequences-can-commit-out-of-order/"
  - "https://event-driven.io/en/ordering_in_postgres_outbox/"
  - "https://www.cybertec-postgresql.com/en/gaps-in-sequences-postgresql/"
  - "https://www.cybertec-postgresql.com/en/sequences-transactional-behavior/"
  - "https://dev.to/kspeakman/event-storage-in-postgres-4dk2"
  - "https://martendb.io/events/projections/async-daemon.html"
  - "https://martendb.io/events/appending.html"
  - "https://dcb.events/specification/"
  - "https://www.postgresql.org/docs/current/functions-sequence.html"
  - "https://jnidzwetzki.github.io/2024/04/03/postgres-and-snapshots.html"
  - "https://blog.2minutestreaming.com/p/kafka-high-watermark-offset"
  - "https://www.waitingforcode.com/apache-kafka/isolation-level-apache-kafka-consumers/read"
---

# position-visibility-in-postgres-and-distributed-logs

## Findings

- **Postgres sequences assign a number at statement execution time, not at
  commit time, and the database is free to commit transactions in a different
  order than it handed out their numbers.** A slower transaction can be handed
  a lower sequence value and still commit after a faster transaction that got
  a higher one. A reader polling "give me everything after N" can walk past a
  row that will exist a moment later at a *lower* number than rows it has
  already returned. (sequinstream.com, event-driven.io)
- **The gaps this produces are not distinguishable, from the outside, from a
  row that simply has not committed yet.** `nextval()` is never rolled back —
  a value consumed by an aborted transaction is gone for good — so a consumer
  cannot tell, by looking at the numbers alone, whether a missing position is
  "never coming" (rollback) or "coming very soon" (in-flight commit). Both a
  Postgres-specific write-up and the Cybertec Postgres consultancy independently
  describe this as a structural property of sequences, not a misconfiguration.
  (cybertec-postgresql.com — two separate articles, one on causes and remedies,
  one on transactional behaviour; event-driven.io)
- **This is a recognized, named, cross-project pain point in the Postgres
  event-sourcing community, with at least three independent public write-ups
  converging on the same description and the same two candidate fixes** (serialize
  writers through a locked sequence table, or read through a transaction-id
  watermark). One author frames it as the reason "so many APIs are eventually
  consistent" — citing Stripe, Salesforce and HubSpot by name as systems whose
  public API semantics were shaped by this exact problem. (sequinstream.com)
- **A shipping, mature Postgres-backed event store has already built and
  operates the "watermark that lags behind the highest known number" pattern in
  production.** Marten's async projection daemon computes a "high watermark" —
  the furthest sequence number *known safe* to process — which is explicitly
  allowed to sit behind the highest sequence number actually written, and it
  inserts synthetic "tombstone" event rows at detected gap positions so the
  daemon can distinguish an aborted transaction's hole from an in-flight one.
  This is not a proposal; it is an operating design with its own documentation
  page. (martendb.io — Appending Events, Async Projections Daemon)
- **The DCB specification that happenstance implements defines the property
  set for `SequencePosition` narrowly and leaves visibility unaddressed.** Per
  the published spec, a position "MUST be unique," "MUST be monotonic
  increasing," and "MAY contain gaps" — three constraints that are all
  satisfiable by a Postgres sequence exhibiting exactly the out-of-order-commit
  behaviour described above. The spec does not state what a reader is entitled
  to assume about whether a position it has observed implies all lower
  positions are also observable. (dcb.events/specification/)
- **The same shape of problem — "the highest number issued" versus "the
  highest number safe to read" — is a load-bearing, named concept in at least
  one other widely-deployed distributed log**, under different vocabulary.
  Kafka distinguishes the Log End Offset (LEO, the last offset physically
  appended) from the High Watermark (the offset up to which all in-sync
  replicas have caught up); everything between HWM and LEO is, by definition,
  invisible to a consumer under the default read semantics, and Kafka additionally
  offers a `read_committed` isolation level specifically so consumers can opt
  out of seeing writes from transactions that have not yet resolved.
  (blog.2minutestreaming.com; waitingforcode.com)
- **Postgres's own MVCC machinery provides a primitive purpose-built for this
  gap** — `pg_current_snapshot()` / `pg_snapshot_xmin()`, backed by the
  strictly-monotonic, cluster-lifetime-unique `xid8` transaction-id type — and
  independent write-ups (a Postgres internals blog and the event-driven.io
  outbox article) both converge on the same construction: order and filter
  reads by transaction id first, sequence position second, and treat "before
  the snapshot's xmin" as the visibility line. This is presented in every
  source as one known option with a real cost, not a free or obvious default.
  (jnidzwetzki.github.io; event-driven.io)

## Evidence & citations

| Claim | Source |
| --- | --- |
| Sequence values are assigned at statement time; commit order can diverge from assignment order | sequinstream.com — "Postgres Sequences Can Commit Out-of-Order" (https://blog.sequinstream.com/postgres-sequences-can-commit-out-of-order/) |
| `nextval()` is never rolled back; aborted transactions leave permanent holes | PostgreSQL official docs, Sequence Manipulation Functions (https://www.postgresql.org/docs/current/functions-sequence.html); corroborated by Cybertec (https://www.cybertec-postgresql.com/en/gaps-in-sequences-postgresql/) |
| A reader cannot tell a rollback-gap from an in-flight-commit-gap by number alone | event-driven.io, "How Postgres sequences issues can impact your messaging guarantees" (https://event-driven.io/en/ordering_in_postgres_outbox/) |
| Framed as cause of eventual-consistency in production APIs (Stripe, Salesforce, HubSpot named) | sequinstream.com (https://blog.sequinstream.com/postgres-sequences-can-commit-out-of-order/) |
| Marten's async daemon computes a high watermark behind the highest known sequence, using tombstone rows to resolve gaps | martendb.io, "Async Projections Daemon" and "Appending Events" (https://martendb.io/events/projections/async-daemon.html, https://martendb.io/events/appending.html) |
| DCB spec: SequencePosition MUST be unique, MUST be monotonic increasing, MAY contain gaps; silent on visibility semantics | dcb.events/specification/ (https://dcb.events/specification/) |
| Kafka LEO vs. High Watermark; `read_committed` isolation level | blog.2minutestreaming.com (https://blog.2minutestreaming.com/p/kafka-high-watermark-offset); waitingforcode.com (https://www.waitingforcode.com/apache-kafka/isolation-level-apache-kafka-consumers/read) |
| `xid8` / `pg_current_snapshot()` / `pg_snapshot_xmin()` as the watermark primitive, and its use for ordering reads | jnidzwetzki.github.io, "Introduction to Snapshots and Tuple Visibility in PostgreSQL" (https://jnidzwetzki.github.io/2024/04/03/postgres-and-snapshots.html); event-driven.io (https://event-driven.io/en/ordering_in_postgres_outbox/) |
| Sequence gaps described as a structural property, not misconfiguration, across two independent Cybertec write-ups | cybertec-postgresql.com, "Gaps in sequences in PostgreSQL" and "Sequences – transactional behavior" (https://www.cybertec-postgresql.com/en/gaps-in-sequences-postgresql/, https://www.cybertec-postgresql.com/en/sequences-transactional-behavior/) |
| Community write-up describing sequence-gap pain discovered specifically when extracting a subsystem into a separate service (external reader) | dev.to/kspeakman, "Event Storage in Postgres" (https://dev.to/kspeakman/event-storage-in-postgres-4dk2) |

## Implications for the idea

- **The open question this initiative already carries — "does the ES-10
  visibility invariant hold globally or only per-boundary, and how does a
  Postgres-backed store buy visibility at all" — is not a hypothetical the
  project invented; it is a documented, named, recurring failure mode in the
  exact ecosystem the flagship adapter targets.** That strengthens the case
  for treating it as a first-class open question with a written verdict,
  rather than something a Postgres adapter can discover informally during
  implementation and patch around.
- **The specification the library implements is itself silent on the
  guarantee.** `SequencePosition` in the DCB spec is defined permissively
  enough (unique, monotonic, gaps allowed) that a store satisfying the letter
  of the spec can still hand a reader a wrong answer during the window before
  a gap resolves. That is evidence for the intake brief's framing that this is
  an *interpretation* question the project must answer explicitly (global vs.
  per-boundary, and what "safe to read" means), not a conformance clause it
  can derive mechanically from the spec text.
- **This is not a Postgres-only concern to scope narrowly.** The same shape of
  problem — a log that has assigned a position further than it is willing to
  let a reader see — is a recognized, named concept (LEO vs. high watermark) in
  at least one mainstream distributed log with a completely different storage
  model. That is useful grounding for treating "what does a reader see, and
  when" as a property of the *port*, something every adapter answers for
  itself, rather than a Postgres-specific wrinkle to special-case.
- **A mature, shipping, DCB-adjacent Postgres event store has already
  operationalized one answer** (watermark-that-lags plus tombstone rows to
  disambiguate gap causes) rather than treating the problem as unsolved. That
  is evidence the problem is tractable in production, not merely diagnosable —
  useful context for how much confidence the project can have that an honest
  written answer is reachable, without this research taking a position on
  which answer happenstance itself should give.
- **The sentiment evidence (three independent public write-ups converging on
  the same description, one invoking Stripe/Salesforce/HubSpot by name) says
  this is a well-known trap that experienced teams still fall into first and
  discover second** — often, per the dev.to account, specifically at the
  moment a system grows a second, external reader. That maps directly onto the
  intake brief's own framing that "nothing has used the contract the way an
  application would" and that a reader outside the writer's own transaction is
  exactly where contract defects surface — reinforcing why this question
  cannot be closed by reasoning against a type checker alone.
