---
item: HS-P0017
stage: intake
created: 2026-08-12T03:23:25.490Z
updated: 2026-08-12T03:23:25.490Z
template_sig: ab516678
rendered_sig: b3d094ea
---

# Intake Brief — What a position means across a store boundary

## Problem

`SequencePosition` is meaningful only within one store, so positions cannot be
replicated as-is. Whether ingest re-checks a writer's asserted append conditions is
the central unanswered question, and it is unanswered in a field that has reached
it the hard way — via documented duplicate-delivery bugs in shipped systems. Today
the shape of the peer port, and whether hub-and-spoke and peer-to-peer are one
abstraction or two, live as prose in `crates/happenstance-sync/src/lib.rs`. Nine
SY-* clauses are provisional and five more are deferred **on a decision, not on
effort** — which means nothing is waiting on capacity; it is waiting on someone
deciding.

## Desired Outcome

Replication identity has a written answer on file — an accepted decision atom, or
an explicit reasoned refusal — with something behind it that would not exist if the
answer were wrong. We know it worked when `happenstance-sync-testkit` exists and one
suite is green against three peers, two of them genuinely unlike, with a
byte-identical payload surviving a round trip across a boundary; when the
corresponding open-question atom is **resolved rather than deleted**; and when
`redkiln validate --kb` passes (DoD 14, AC-13).

## Constraints

- **Depends on** `publication-and-positioning`; blocks `retention-and-incomplete-logs`
  (which needs SY-32). This is `RUNBOOK.md`'s 12 → 13 → 14 order.
- **Neither sync crate is published by this initiative.** The sync port stays out of
  the contract crate precisely so that publishing never waits on replication —
  which is also why this project lands *after* `0.2.0` at no cost to anything
  published.
- **`happenstance-sync` is a port crate, not an adapter.** Peer adapters depend on
  it the way store adapters depend on `happenstance-core`. No adapter may depend on
  another adapter.
- **The `IngestStore` seam stays out of `EventStore`** — the coupling
  `RUNBOOK.md:440-466` warns about. The residual risk (a real peer needing a
  store-side seam on a **now-published** port) is evaluated explicitly rather than
  assumed away.
- **Non-goals**, each naming its owner: publishing either sync crate → out of the
  release train per the charter, and `publication-and-positioning` owns what ships;
  what a store may forget → `retention-and-incomplete-logs`; any DCB
  wire-interoperability bridge → deferred on the stronger reason that the DCB
  specification publishes no wire format at all.

## Open Questions

- **ADR-0026** — what a sync *peer* is; what the port may assume about a transport
  it cannot see; what ingest promises (SY-8 – SY-18).
- **ADR-0027** — the merge rule, the compensation contract, and whether
  hub-and-spoke and peer-to-peer are one abstraction or two (SY-1 – SY-7,
  SY-19 – SY-31).
- **Does ingest re-check the writer's asserted append conditions?** The central
  question. A refusal to define it is an acceptable answer only if it is written
  down with its reasons.
- **Two specification passages whose words are now wrong** and must be corrected as
  part of this work: the two clauses framing ingest as re-checking conditions, and
  the three clauses whose named wrong implementation was `happenstance-sync`'s own
  superseded proposal — a named wrong implementation that no longer exists cannot
  reject anything.
- Whether **WF-1**'s interoperability half stays deferred on its stronger reason, or
  is answered here.
- **ADR-0003 loses its `provisional` marker** as part of this pass — confirm that is
  still correct once the envelope types are exercised by a real peer.

## Proof artefact

**One conformance suite green against three peers, two of them structurally
unlike, with a byte-identical payload round-tripped across a store boundary.**
This would not exist if the design were wrong: a replication port designed against
one peer shape encodes that peer's assumptions about identity, and only an unlike
second and third peer can reveal that a position was being treated as portable when
it is a statement about one store's log. If the outcome is a reasoned refusal
instead, the artefact is the written decision atom naming the alternatives that
lost — silence is not an outcome.

## Clauses

- **SY-1 – SY-7, SY-19 – SY-31** `[PROVISIONAL]` — settled by **ADR-0027**.
- **SY-8 – SY-18** `[PROVISIONAL]` — settled by **ADR-0026**.
- **SY-14, SY-18, SY-27, SY-28** `[DEFERRED]` — deferred on a decision, not on
  effort; each is answered here or its deferral is restated with a stronger reason.
- **SY-32** `[DEFERRED]` — answered here, because `retention-and-incomplete-logs`
  needs it.
- **VT-*** `[PROVISIONAL]` (≈9) — the wire clauses, against a frozen wire format
  with negative controls asserted by name.
- **WF-1** `[DEFERRED]` — its interoperability half re-affirmed or answered.
- **ADR-0003 loses `provisional`.**
- The two mis-worded ingest clauses and the three clauses with a superseded named
  wrong implementation are corrected — by decision record where they are frozen,
  never by edit.

## Gate: Intake

Tick each box once the sections above satisfy it. `redkiln advance` reads these
against `.redkiln/templates/gates/intake.md` — where each box's rationale is
written — and will not leave `intake` until every one is ticked. Keep each box on
one line; the parser matches line by line.

- [x] The problem and desired outcome are stated.
- [x] Constraints and non-goals are recorded.
- [x] Open questions are captured for distillation.
- [x] The proof artefact is named, and it would not exist if the design were wrong.
- [x] For a port freeze: the axis it is most likely to be wrong about is named, and something in the workspace sits at the other end of it.
- [x] The clauses this work discharges or amends are listed by id.
- [x] Where this brief and `SPECIFICATION.md` disagree, the specification wins.
