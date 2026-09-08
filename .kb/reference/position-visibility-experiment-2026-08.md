---
id: kb-reference-position-visibility-experiment-001
title: The position-visibility experiment — four arms against real PostgreSQL
kind: reference
status: accepted
authority_tier: note
summary: >-
  The phase-2 measurement ADR-0013 rests on, run against real PostgreSQL with fsync=on and kept
  in experiments/position-visibility/. Four strategies for buying position visibility when
  nextval() allocates outside the transaction were measured against an inversion detector. Arm C
  (xid8 + pg_snapshot_xmin) is the only one passing on both writer pairs while leaving writers
  unserialised, at throughput ratios to baseline of 0.987, 0.993, 1.015 and 1.026 at 1, 8, 32 and
  64 clients — the one-client figure from a separate 90-second pass, because the 30-second
  baseline spread was 3.70x. Two positive controls fired: the baseline reproduces the inversion,
  and arm A collapses to 0.062x at 64 clients with p99 60x worse. Arm B-tag, the per-boundary
  lock, measured 0.935 at 64 clients and was rejected on invariant grounds rather than cost.
  Read-your-own-writes staleness under arm C was 0.688 ms unloaded and 4010.719 ms behind an
  unrelated five-second write in an unrelated database.
depends_on: []
related:
  - kb-decision-0013
  - kb-playbook-cold-future-hand-polling-001
  - kb-open-question-global-vs-boundary-visibility-001
  - kb-open-question-postgres-arm-c-cost-001
  - kb-open-question-poll-count-rule-strength-001
  - kb-reference-append-condition-experiment-001
  - kb-reference-position-visibility-adapter-remeasurement-001
source_paths:
  - .kb/_intake/0013-position-assignment-and-visibility.md
  - references/adr/0013-position-assignment-and-visibility.md
  - experiments/position-visibility/
  - RUNBOOK.md
last_reviewed: 2026-08-10
---

# The position-visibility experiment — four arms against real PostgreSQL

## What this is a pointer to

The full instrument, its schema per arm, and every raw result file live in
`experiments/position-visibility/`, outside the workspace and the gate. This atom is the citable
summary of what it measured; the conclusion drawn from the measurement — that ES-10 lifts from
`[PROVISIONAL]` to `[FROZEN]`, and what a conformant adapter must then document about
read-your-own-writes — is ADR-0013's, not this atom's. What decided *which* invariant to freeze
(global vs. per-boundary) is a judgement made against this evidence, recorded in the decision
atom that cites it.

## The question the experiment answers

A store whose position assignment (`nextval()`) happens outside the committing transaction can
let a transaction that started later become visible earlier than one that started first and holds
a lower position. That inversion is what makes `AppendCondition::after: Some(P)` unsound: a caller
can condition on a boundary that a slower, lower-positioned writer has not yet crossed, and the
condition evaluates as satisfied because the store has no way to say "not yet — something is still
in flight below that position." The experiment asks which of several affordable mechanisms
restores the invariant "once any reader has observed an event at position P, no subsequent read
may yield an event at a position at-or-below P that was not already visible" — without paying for
full write serialisation.

## The four arms and the inversion detector

Baseline (`nextval()`, no correction), arm A (fully serialised writers), arm B-const (a single
constant advisory lock), arm B-tag (a transaction-scoped advisory lock keyed by tag —
per-boundary rather than global), and arm C (`xid8` + `pg_snapshot_xmin`, reading only rows whose
writing transaction has already passed the current snapshot's xmin frontier). Each arm was run
against two writer-pair shapes, shared-tag and disjoint-tags, with an inversion detector that
counts violations of the invariant above.

**Only arm C passes on both writer pairs while leaving writers unserialised.** Arm B-tag passes on
disjoint tags but reproduces the baseline inversion byte-for-byte on the shared tag, because its
invariant is per-boundary rather than global — the experiment measured that the two properties are
not the same and deliberately left the choice between them to the ADR that would consume the
result.

## Throughput

Ratio to a bracketing baseline, arm C: **0.987 / 0.993 / 1.015 / 1.026** at 1 / 8 / 32 / 64
clients. The one-client figure is from a separate 90-second pass (`results/ratios-c1long.csv`),
not the 30-second series (`results/ratios.csv`): at 30 seconds the one-client baseline spread was
3.70x, larger than every effect in the row, and `ratios.csv` alone records arm C at 0.628 there —
a figure that reads as 37% slower and is an artefact of baseline noise, not of arm C. Citing only
one of the two files produces the wrong conclusion; `README.md:248-256` explains the substitution.

Arm B-tag measured **0.935** at 64 clients — cheaper than arm C — and was rejected on invariant
grounds (it buys the wrong property for a global projection checkpoint), not on cost. Arm A and
arm B-const, the two serialising strategies, collapse to **0.062x** and **0.033x** of baseline at
64 clients, flat from 8 clients upward — the signature of a total serialisation point.

## Positive controls

Two fired, which is what makes a passing result on arm C worth anything: the baseline reproduces
the inversion (the detector can find a real bug), and arm A's collapse at 64 clients carries p99
latency 60x worse than baseline (the throughput number is not measuring something else).

## Read-your-own-writes under arm C

Because arm C's visibility is a predicate (`xact_id < pg_snapshot_xmin(...)`) rather than an
identity, a reader's own just-committed write is not guaranteed visible at the next read. Staleness
is bounded by the longest open write transaction *anywhere in the cluster*, measured at **0.688
ms** with no holder and **4010.719 ms** behind an unrelated five-second write in an unrelated
database (`results/staleness_pinned.txt`). This is a capability limit of the mechanism, not a
tuning parameter, and is the reason `head()` under an arm-C adapter must be read as reporting a
frontier rather than `max(position)`.

## Scope and limitation, stated by the experiment itself

`README.md:364-366`: "Nothing here is an adapter … This measures four SQL strategies, not four
implementations of `SendEventStore`." The experiment establishes that one affordable mechanism
exists; it does not establish that `happenstance-postgres` can express that mechanism cleanly
through `sqlx`, or that its structural cost is acceptable under a real workload. Both are
open, and are `happenstance-postgres`'s (phase 10, ADR-0024's).
