---
id: reference-experiment-position-visibility
title: "Measured: what four Postgres position-visibility mechanisms actually cost"
kind: reference
status: accepted
authority_tier: reference
summary: >-
  A reproducible experiment against a real server. `xid8` + `pg_snapshot_xmin` buys ES-10 as
  written at no measurable throughput cost. Two mechanisms buy it by serialising every
  writer, at 16× and 30× the throughput. Advisory locks keyed by tags are cheap BECAUSE they
  do not buy ES-10 — they reproduce the inversion whenever two writers hold disjoint keys.
depends_on: []
related:
  - question-postgres-position-visibility
  - adr-0013-position-assignment-and-visibility
source_paths:
  - docs/experiments/position-visibility/README.md
last_reviewed: 2026-08-09
---

# The position-visibility measurement

## What was measured

Four candidate mechanisms for making a Postgres event store satisfy ES-10, which
`nextval()` breaks by construction: it allocates outside the transaction, so a writer that
took 99 can commit after one that took 100, and a reader that has already observed 100 later
sees 99 appear beneath it.

## The result

| Mechanism | Buys ES-10? | Cost |
| --- | --- | --- |
| `xid8` + `pg_snapshot_xmin` | yes, as written | no measurable throughput cost |
| serialising mechanism A | yes | 16× throughput |
| serialising mechanism B | yes | 30× throughput |
| advisory locks keyed by tags | **no** | cheap, because it buys a different invariant |

The fourth is the one worth remembering: it looks like the cheap win and is cheap precisely
because it reproduces the inversion whenever two writers hold disjoint keys. What it buys is
a per-boundary invariant that ES-10 does not state.

## Standing

Reproducible, and deliberately **not** in the gate: `docs/experiments/` is measurements, not
workspace members and not `cargo xtask ci` steps. Every qualitative claim in the adapter's
own prior analysis survived contact with a server; the experiment does not revise it.
