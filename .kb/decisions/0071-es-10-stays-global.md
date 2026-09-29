---
id: kb-decision-0071
title: ES-10 stays global, because the frozen checkpoint is one position
kind: decision
status: accepted
authority_tier: decision
adr_id: ADR-0071
reversibility: low
phase: 16
supersedes: null
superseded_by: null
summary: >-
  Settles kb-open-question-global-vs-boundary-visibility-001, which ADR-0013 left "closed by decision,
  not by evidence" and ADR-0024 inherited rather than owned. ES-10's visibility invariant stays
  global and [FROZEN]. The premise ADR-0013 rested it on — a runner resumes from one position that
  covers boundaries it never reads — is now frozen at every seam: ProjectionStore::commit takes one
  SequencePosition, Checkpoint carries one through, PS-17 fixes a checkpoint per (store,
  ProjectionId) and PS-20 resumes strictly after it, all under ADR-0063. ADR-0063 never mentions
  ES-10, so this record answers the circularity worry (sub-question 1) on the checkpoint's own
  merits rather than by citing ES-10: ES-30 keeps head() unscoped because a narrow projection must
  checkpoint past events it examined and did not match, which is a polling-cost argument that does
  not mention visibility; a per-boundary checkpoint would be a position per boundary a projection's
  query touches, an open-ended set for any projection that reads a type across all its instances;
  and SequencePosition carries no boundary, so the replication clauses citing ES-10 — inside 1.0 now
  that sync is — would need the boundary to travel with it. Sub-question 5 is answered on the record:
  if the premise ever falls, ADR-0024 reopens with ADR-0013, and the trigger is the cluster-wide
  staleness coupling (4799.3 ms behind an unrelated five-second write), not throughput, since arm C
  has no measurable steady-state cost. The falsifier is restated as a checkpoint-shape change —
  a boundary-scoped checkpoint — however it arrives: a changed commit or through is breaking and
  reopens ADR-0063, and an additive #[non_exhaustive] variant or defaulted method fires this
  falsifier just the same.
  Sub-questions 2, 3 and 4 are non-breaking to answer and are carried past 1.0 in this record's
  falsifier section rather than dismissed; phase 17 does not take them.
depends_on:
  - kb-decision-0013
  - kb-decision-0024
  - kb-decision-0063
related:
  - kb-decision-0066
  - kb-decision-0018
  - kb-open-question-global-vs-boundary-visibility-001
  - kb-reference-position-visibility-experiment-001
  - kb-reference-position-visibility-adapter-remeasurement-001
source_paths:
  - references/adr/0013-position-assignment-and-visibility.md
  - references/adr/0024-position-visibility-mechanism.md
  - references/adr/0063-the-projection-port-is-frozen.md
  - crates/happenstance-core/src/projection.rs
  - spec/SPECIFICATION.md
last_reviewed: 2026-09-29
---

# ES-10 stays global, because the frozen checkpoint is one position

## The question, and why it was still open

ADR-0013 froze ES-10 as a **global** invariant and said, in its own words, that it had closed the
choice "*by decision* ... not by evidence" (`references/adr/0013-position-assignment-and-visibility.md:600-605`).
Its argument (`:181-204`) was that a per-boundary invariant, arm B-tag, 0.935x at 64 clients,
would make `AppendCondition` sound and the projection checkpoint unsound, because a runner resumes
from one global position. It named its own falsifier: "a projection checkpoint that is
boundary-scoped rather than global", owned by the `ProjectionStore` freeze. ADR-0024 then took
xid8 + `pg_snapshot_xmin` on that inherited premise (`references/adr/0024-position-visibility-mechanism.md:173-183`).

ADR-0063 froze the port, and the falsifier did not fire. But ADR-0063 never mentions ES-10,
visibility or boundaries. The phase-15 re-read therefore declined to close the question on the
falsifier alone, because the premise had still not been argued independently of ES-10. This record
supplies that argument.

## Decision

**ES-10 stays global and `[FROZEN]`.** The premise is now frozen at every seam it touches:

- `ProjectionStore::commit` takes exactly one `position: SequencePosition`
  (`crates/happenstance-core/src/projection.rs:498-504`).
- `Checkpoint::Live` and `Checkpoint::Rebuilding` each carry one `through: SequencePosition`
  (`projection.rs:193-206`).
- PS-17 fixes a checkpoint per `(store, ProjectionId)` (`spec/SPECIFICATION.md:5624-5626`), and
  PS-20 resumes strictly after that one position (`:5702-5705`). Both are `[FROZEN]`.
- ADR-0063 put the whole port under semver, `Checkpoint` included
  (`references/adr/0063-the-projection-port-is-frozen.md:25-37`).

## Sub-question 1: the checkpoint is single-position on its own merits

The worry was circularity: the checkpoint might be global only because ES-10 was already global.
Three reasons stand without citing ES-10.

1. **ES-30's reason is about cost, not visibility.** `head()` is unscoped because "a narrow
   projection's problem is that it cannot advance past events it examined and did not match; the
   global head is what lets it checkpoint past them" (`spec/SPECIFICATION.md:4213-4216`). That
   sentence is about polling cost. It would be just as true under a per-boundary invariant, and it
   is what makes one global resume point the natural shape.
2. **A per-boundary checkpoint is an open-ended set.** A projection reading one event type across
   every instance of a thing, such as every course or every account, touches one boundary per
   instance. A checkpoint per boundary is then a position per instance, and it grows with the
   domain rather than with the projection. This is a reasoned argument, not a measured one.
3. **Replication.** `SequencePosition` carries no boundary, and the `SY` clauses cite ES-10. A
   boundary-scoped invariant would need the boundary to travel with the position (ADR-0013
   `:202-204`). With sync inside 1.0, this leg is no longer hypothetical.

## Sub-question 5: what reopens ADR-0024, and on what trigger

If the premise ever falls, ADR-0024 reopens **with** ADR-0013, because it rejected B-tag "on
principle rather than cost" (`.kb/decisions/0024-position-visibility-mechanism.md:91-98`). The
trigger is **not throughput**. Arm C has no measurable steady-state cost: a 0.593 ms median against
the unguarded 0.521. What it costs is the **cluster-wide staleness coupling**: 4799.3 ms behind an
unrelated five-second held write (`references/adr/0024-position-visibility-mechanism.md:126-137`).
That coupling is the cost B-tag would avoid, so it is the figure a reopening weighs.

## Falsifier

**Restated.** This record reopens on a **checkpoint-shape change**: a projection checkpoint that is
boundary-scoped, or that carries more than one position a runner resumes from. As a changed
`commit`, or as a set of positions in place of `through`, the change is breaking for the frozen
port, and it would arrive through ADR-0063's own reopening condition
(`references/adr/0063-the-projection-port-is-frozen.md:146-153`) and not as a quiet redesign.
There is one semver subtlety. `Checkpoint` is `#[non_exhaustive]` (`projection.rs:183`), so a new
variant, or a new method with a default body, could arrive without a major version. If a
boundary-scoped checkpoint arrived that way, it would fire this falsifier just as surely. What
decides whether this record reopens is the checkpoint's shape, not the semver class of the change.

**Carried past 1.0, not dismissed.** Answering these does not break a published crate, and phase 17
does not take them.

- **Sub-question 2**: whether a boundary-scoped checkpoint design would be priced against B-tag's
  figure before or after its shape is settled. It is moot until one is proposed. Whoever proposes
  one answers it first.
- **Sub-question 3**: whether reopening ES-10 reopens `AppendCondition`'s boundary semantics.
  ADR-0013's own argument says a per-boundary invariant keeps the condition sound. ES-25 and ES-26
  are built on ES-10 (`spec/SPECIFICATION.md:2948-2951`), though, so a reopening has to re-read
  them rather than assume they are unaffected.
- **Sub-question 4**: a hybrid, with a per-boundary checkpoint plus a global watermark. Evaluating
  it on paper is free. Building it is the checkpoint-shape change above.
