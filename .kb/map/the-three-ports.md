---
id: map-the-three-ports
title: "Map: the three ports, and where their clauses live"
kind: map
status: accepted
authority_tier: note
summary: >-
  `EventStore` (§3, ES-*), `ProjectionStore` (§4, PS-*) and `SyncPeer` (§5, SY-*), plus the
  value types and wire format (§2, VT-*/WF-*) and the conformance obligation (§6, CF-*).
  `SPECIFICATION.md` says what is TRUE NOW, clause by clause with a maturity marker; the
  ADRs say why and when. Where they disagree, the specification wins.
depends_on: []
related:
  - map-the-instrument-portfolio
  - map-the-crate-dependency-rule
  - question-projection-store-port-is-provisional
  - question-replication-semantics
source_paths:
  - docs/architecture/SPECIFICATION.md
last_reviewed: 2026-08-09
---

# The three ports

## Where each lives

| Port / area | Section | Clause prefix | Status |
| --- | --- | --- | --- |
| Value types and the wire format | §2 | `VT-`, `WF-` | frozen at phases 4 and 5 |
| `EventStore` | §3 | `ES-` | frozen at phase 4 |
| `ProjectionStore` | §4 | `PS-` | **provisional** — no conformance suite yet |
| `SyncPeer` | §5 | `SY-` | indicative shape only |
| The conformance obligation | §6 | `CF-` | frozen at phase 3 |

## How to read a clause

Every clause carries exactly one maturity marker: `[FROZEN]`, `[PROVISIONAL]`, `[DEFERRED]`,
or `[NON-NORMATIVE]` for prose that was demoted rather than deleted. Each also names the
conformance rule that checks it and the wrong implementation it forbids.

A `[FROZEN]` clause changes by new ADR, never by edit. `cargo xtask spec-trace` is a gate
step precisely so those markers and citations cannot rot into decoration — its first run
found 156 problems, 24 of them real dangling citations.

## The two documents, and their tenses

An ADR records *why* a decision was taken and *when*. The specification records *what is
true now*. They drift, because a decision is recorded when it is taken and executed later or
not at all — the worked example being the crate rename, accepted in ADR-0006 and untrue on
disk for a whole phase. The rule: **where an ADR and the specification disagree, the ADR is
history and the specification is current.**
